use axum::{http::StatusCode, response::IntoResponse};
use pasu_profile_backend::error::AppError;
use sqlx::Error as SqlxError;
use std::env::VarError;
use thiserror::Error;

#[test]
fn test_app_error_not_found() {
    let error = AppError::NotFound;
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_app_error_auth_error() {
    let error = AppError::AuthError;
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_app_error_validation_error() {
    let error = AppError::ValidationError("Invalid input".to_string());
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn test_app_error_upload_error() {
    let error = AppError::UploadError("Upload failed".to_string());
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_app_error_display() {
    let not_found = AppError::NotFound;
    assert_eq!(format!("{}", not_found), "Not found");

    let auth_error = AppError::AuthError;
    assert_eq!(format!("{}", auth_error), "Authentication failed");

    let validation_error = AppError::ValidationError("Invalid email".to_string());
    assert_eq!(format!("{}", validation_error), "Validation error: Invalid email");

    let upload_error = AppError::UploadError("Cloudinary error".to_string());
    assert_eq!(format!("{}", upload_error), "Upload error: Cloudinary error");
}

#[test]
fn test_app_error_debug() {
    let error = AppError::NotFound;
    let debug_output = format!("{:?}", error);
    assert!(debug_output.contains("NotFound"));
}

#[test]
fn test_app_error_from_sqlx_error() {
    let sqlx_error = SqlxError::RowNotFound;
    let app_error: AppError = sqlx_error.into();
    let response = app_error.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_app_error_from_var_error() {
    let var_error = VarError::NotPresent;
    let app_error: AppError = var_error.into();
    let response = app_error.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_app_error_internal_error() {
    let error = AppError::InternalError(anyhow::anyhow!("Something went wrong"));
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_app_error_validation_error_message() {
    let error = AppError::ValidationError("Email is required".to_string());
    let response = error.into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn test_multiple_validation_errors() {
    let errors = vec![
        AppError::ValidationError("Invalid name".to_string()),
        AppError::ValidationError("Invalid email".to_string()),
        AppError::ValidationError("Invalid phone".to_string()),
    ];

    for error in errors {
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

#[test]
fn test_error_chain() {
    let error = AppError::ValidationError("Invalid input".to_string());
    let error_message = format!("{}", error);
    assert!(error_message.contains("Invalid input"));
    assert!(error_message.contains("Validation error"));
}

#[test]
fn test_error_types_are_distinct() {
    let not_found = AppError::NotFound;
    let auth_error = AppError::AuthError;
    let validation_error = AppError::ValidationError("test".to_string());

    let nf_response = not_found.into_response();
    let auth_response = auth_error.into_response();
    let val_response = validation_error.into_response();

    assert_eq!(nf_response.status(), StatusCode::NOT_FOUND);
    assert_eq!(auth_response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(val_response.status(), StatusCode::BAD_REQUEST);
}
