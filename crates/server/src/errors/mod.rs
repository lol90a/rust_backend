use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;
use tracing::warn;

use app_core::domain::errors::DomainError;

/// Unified API error type returned by all handlers.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("internal error")]
    Internal(#[from] anyhow::Error),
}

impl From<DomainError> for ApiError {
    fn from(e: DomainError) -> Self {
        match &e {
            DomainError::UserNotFound(_) => ApiError::NotFound(e.to_string()),
            DomainError::UserAlreadyExists(_) => ApiError::Conflict(e.to_string()),
            DomainError::InvalidEmail(_) | DomainError::InvalidDisplayName => {
                ApiError::BadRequest(e.to_string())
            },
        }
    }
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        if status.is_server_error() {
            warn!(error = %self, "internal server error");
        }

        let body = ErrorBody {
            error: status
                .canonical_reason()
                .unwrap_or("Unknown Error")
                .to_owned(),
            detail: Some(self.to_string()),
        };

        HttpResponse::build(status).json(body)
    }
}
