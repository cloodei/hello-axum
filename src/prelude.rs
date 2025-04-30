#![allow(non_snake_case, unused, unused_imports, dead_code)]

use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};
use crate::error::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Item {
    id: usize,
    name: String,
    description: String,
    count: usize,
    height: usize,
    weight: usize
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateItemPayload {
    name: String,
    description: String,
    count: usize,
    height: usize,
    weight: usize
}


impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::RedisError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::RedisError(e) => {
                tracing::error!("Redis error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error: Database operation failed".to_string())
            }
            Error::JsonError(e) => {
                tracing::error!("JSON error: {:?}", e);
                (StatusCode::BAD_REQUEST, format!("JSON processing error: {}", e))
            }
            Error::NotFound(resource) => (StatusCode::NOT_FOUND, format!("Resource not found: {}", resource)),
            Error::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Error::PoolError(e) => {
                tracing::error!("Redis Pool error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error: Failed to get connection".to_string())
            }
        };

        let body = Json(serde_json::json!({ "error": error_message }));
        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
