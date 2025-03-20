use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HeimdallError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Permission error: {0}")]
    Permission(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl IntoResponse for HeimdallError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            HeimdallError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            HeimdallError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            HeimdallError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            HeimdallError::Permission(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            HeimdallError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            HeimdallError::Json(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            HeimdallError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
        };

        // Log the error when it happens
        tracing::error!(%status, error = %error_message, "Request error");

        // Build the response
        let body = Json(json!({
            "error": {
                "message": error_message,
                "type": format!("{:?}", self).split('(').next().unwrap_or("Unknown"),
            }
        }));

        (status, body).into_response()
    }
}

pub type HeimdallResult<T> = Result<T, HeimdallError>;
