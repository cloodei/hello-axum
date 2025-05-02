pub mod redis {
    use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
    use serde::{Deserialize, Serialize};
    use crate::error::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Item {
        pub id: usize,
        pub name: String,
        pub description: String,
        pub count: usize,
        pub height: usize,
        pub weight: usize
    }
    
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct CreateItemPayload {
        pub name: String,
        pub description: String,
        pub count: usize,
        pub height: usize,
        pub weight: usize
    }
    
    type RedisPool = bb8::Pool<bb8_redis::RedisConnectionManager>;
    
    #[derive(Clone)]
    pub struct AppState {
        pub redis_pool: RedisPool
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
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal Server Error: Database operation failed with error: {}", e))
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
    
            (status, error_message).into_response()
        }
    }
    
    pub fn map_pool_error<E: std::error::Error + 'static>(e: bb8::RunError<E>) -> Error {
        Error::PoolError(e.to_string())
    }
    
    pub type Result<T> = std::result::Result<T, Error>;    
}
