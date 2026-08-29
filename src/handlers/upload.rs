use axum::{extract::Multipart, Json};
use serde::Serialize;
use std::env;
use reqwest::multipart;
use crate::error::AppError;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct UploadResponse {
    success: bool,
    url: String,
    #[serde(rename = "publicId")]
    public_id: String,
}

#[derive(serde::Deserialize)]
struct CloudinaryResponse {
    secure_url: String,
    public_id: String,
}

/// Sniff the leading bytes and return the real image type, or `None` when the
/// payload is not one of the formats we accept (JPEG, PNG, GIF, WebP).
fn detect_image_mime(bytes: &[u8]) -> Option<&'static str> {
    if bytes.len() < 12 {
        return None;
    }
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some("image/jpeg");
    }
    if bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
        return Some("image/png");
    }
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return Some("image/gif");
    }
    if bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        return Some("image/webp");
    }
    None
}

#[utoipa::path(
    post,
    path = "/api/upload",
    request_body(content = String, description = "Multipart form data with 'file' field", content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "File uploaded successfully", body = UploadResponse),
        (status = 400, description = "Bad request")
    )
)]
pub async fn upload_image(mut multipart: Multipart) -> Result<Json<UploadResponse>, AppError> {
    let mut file_bytes = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::UploadError(e.to_string()))? {
        if field.name() == Some("file") {
            file_bytes = field.bytes().await.map_err(|e| AppError::UploadError(e.to_string()))?.to_vec();
            break;
        }
    }

    if file_bytes.is_empty() {
        return Err(AppError::ValidationError("No file uploaded".to_string()));
    }

    if file_bytes.len() > 5 * 1024 * 1024 {
        return Err(AppError::ValidationError("File size too large. Max 5MB.".to_string()));
    }

    // The declared Content-Type is both attacker-controlled and unreliable —
    // browsers send `application/octet-stream` for a file whose type the OS
    // cannot infer — so the bytes decide. The sniffed type is what we forward.
    let Some(detected_mime) = detect_image_mime(&file_bytes) else {
        return Err(AppError::ValidationError(
            "Invalid file type. Only images are allowed.".to_string(),
        ));
    };

    let cloudinary_url = env::var("CLOUDINARY_URL").map_err(|_| AppError::InternalError(anyhow::anyhow!("CLOUDINARY_URL not set")))?;
    
    // Parse cloudinary://API_KEY:API_SECRET@CLOUD_NAME
    let stripped = cloudinary_url.strip_prefix("cloudinary://").ok_or_else(|| AppError::InternalError(anyhow::anyhow!("Invalid CLOUDINARY_URL")))?;
    let parts: Vec<&str> = stripped.split('@').collect();
    if parts.len() != 2 {
        return Err(AppError::InternalError(anyhow::anyhow!("Invalid CLOUDINARY_URL format")));
    }
    
    let auth_parts: Vec<&str> = parts[0].split(':').collect();
    if auth_parts.len() != 2 {
        return Err(AppError::InternalError(anyhow::anyhow!("Invalid CLOUDINARY_URL auth format")));
    }

    let api_key = auth_parts[0];
    let api_secret = auth_parts[1];
    let cloud_name = parts[1];

    let timestamp = chrono::Utc::now().timestamp().to_string();
    let folder = "portfolio";
    
    // Cloudinary signature generation (SHA-1)
    let string_to_sign = format!("folder={}&timestamp={}{}", folder, timestamp, api_secret);
    let signature = {
        use sha1::{Sha1, Digest};
        let mut hasher = Sha1::new();
        hasher.update(string_to_sign.as_bytes());
        format!("{:x}", hasher.finalize())
    };

    let client = reqwest::Client::new();
    let file_part = multipart::Part::bytes(file_bytes)
        .file_name("upload.img")
        .mime_str(detected_mime)
        .map_err(|e| AppError::InternalError(e.into()))?;

    let form = multipart::Form::new()
        .text("folder", folder.to_string())
        .text("timestamp", timestamp)
        .text("api_key", api_key.to_string())
        .text("signature", signature)
        .part("file", file_part);

    let res = client
        .post(format!("https://api.cloudinary.com/v1_1/{}/image/upload", cloud_name))
        .multipart(form)
        .send()
        .await
        .map_err(|e| AppError::InternalError(e.into()))?;

    let res_status = res.status();
    let res_text = res.text().await.unwrap_or_default();

    if !res_status.is_success() {
        tracing::error!("Cloudinary upload failed ({}): {}", res_status, res_text);
        return Err(AppError::UploadError(
            "Image upload failed. Please try again.".to_string(),
        ));
    }

    let parsed: CloudinaryResponse = serde_json::from_str(&res_text)
        .map_err(|e| AppError::InternalError(e.into()))?;

    Ok(Json(UploadResponse {
        success: true,
        url: parsed.secure_url,
        public_id: parsed.public_id,
    }))
}

#[cfg(test)]
mod tests {
    use super::detect_image_mime;

    #[test]
    fn detects_each_supported_format() {
        let mut png = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        png.extend_from_slice(&[0u8; 8]);
        assert_eq!(detect_image_mime(&png), Some("image/png"));

        let mut jpeg = vec![0xFF, 0xD8, 0xFF, 0xE0];
        jpeg.extend_from_slice(&[0u8; 8]);
        assert_eq!(detect_image_mime(&jpeg), Some("image/jpeg"));

        let mut webp = b"RIFF\0\0\0\0WEBP".to_vec();
        webp.extend_from_slice(&[0u8; 4]);
        assert_eq!(detect_image_mime(&webp), Some("image/webp"));

        let mut gif = b"GIF89a".to_vec();
        gif.extend_from_slice(&[0u8; 8]);
        assert_eq!(detect_image_mime(&gif), Some("image/gif"));
    }

    #[test]
    fn rejects_non_image_payloads() {
        // A script renamed to .png with a spoofed Content-Type.
        assert_eq!(detect_image_mime(b"<?php system($_GET[0]); ?>   "), None);
        assert_eq!(detect_image_mime(b"too short"), None);
        assert_eq!(detect_image_mime(&[]), None);
    }
}
