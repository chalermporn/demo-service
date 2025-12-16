use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::application::RepositoryError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, error: String, message: String) -> Self {
        Self {
            status: status.as_u16(),
            error,
            message,
            details: None,
        }
    }

    pub fn with_details(mut self, details: Vec<String>) -> Self {
        self.details = Some(details);
        self
    }
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    ValidationError(Vec<String>),
    InternalError(String),
    DatabaseError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::ValidationError(errors) => {
                write!(f, "Validation error: {}", errors.join(", "))
            }
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message, details) = match self {
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND".to_string(),
                msg,
                None,
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "BAD_REQUEST".to_string(),
                msg,
                None,
            ),
            AppError::ValidationError(errors) => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR".to_string(),
                "Input validation failed".to_string(),
                Some(errors),
            ),
            AppError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR".to_string(),
                msg,
                None,
            ),
            AppError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR".to_string(),
                format!("Database operation failed: {}", msg),
                None,
            ),
        };

        let mut error_response = ErrorResponse::new(status, error_type, message);
        if let Some(details) = details {
            error_response = error_response.with_details(details);
        }

        (status, Json(error_response)).into_response()
    }
}

impl From<RepositoryError> for AppError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::NotFound => {
                AppError::NotFound("Resource not found".to_string())
            }
            RepositoryError::DatabaseError(msg) => AppError::DatabaseError(msg),
            RepositoryError::InvalidInput(msg) => AppError::BadRequest(msg),
        }
    }
}
