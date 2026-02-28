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
    let mut mime_type = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::UploadError(e.to_string()))? {
        if field.name() == Some("file") {
            if let Some(content_type) = field.content_type() {
                mime_type = content_type.to_string();
            }
            file_bytes = field.bytes().await.map_err(|e| AppError::UploadError(e.to_string()))?.to_vec();
            break;
        }
    }

    if file_bytes.is_empty() {
        return Err(AppError::ValidationError("No file uploaded".to_string()));
    }

    let allowed_types = ["image/jpeg", "image/jpg", "image/png", "image/gif", "image/webp"];
    if !allowed_types.contains(&mime_type.as_str()) {
        return Err(AppError::ValidationError("Invalid file type. Only images are allowed.".to_string()));
    }

    if file_bytes.len() > 5 * 1024 * 1024 {
        return Err(AppError::ValidationError("File size too large. Max 5MB.".to_string()));
    }

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
        .mime_str(&mime_type)
        .map_err(|e| AppError::InternalError(e.into()))?;

    let form = multipart::Form::new()
        .text("folder", folder.to_string())
        .text("timestamp", timestamp)
        .text("api_key", api_key.to_string())
        .text("signature", signature)
        .part("file", file_part);

    let res = client
        .post(&format!("https://api.cloudinary.com/v1_1/{}/image/upload", cloud_name))
        .multipart(form)
        .send()
        .await
        .map_err(|e| AppError::InternalError(e.into()))?;

    let res_status = res.status();
    let res_text = res.text().await.unwrap_or_default();

    if !res_status.is_success() {
        return Err(AppError::UploadError(format!("Cloudinary error: {}", res_text)));
    }

    let parsed: CloudinaryResponse = serde_json::from_str(&res_text)
        .map_err(|e| AppError::InternalError(e.into()))?;

    Ok(Json(UploadResponse {
        success: true,
        url: parsed.secure_url,
        public_id: parsed.public_id,
    }))
}
