pub mod redis {
    use std::fmt;
    use axum::{http::StatusCode, response::{IntoResponse, Response}};

    #[derive(Debug)]
    pub enum Error {
        RedisError(redis::RedisError),
        JsonError(serde_json::Error),
        NotFound(String),
        BadRequest(String),
        PoolError(String)
    }
    impl std::error::Error for Error {}
    
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::PoolError(e) => write!(f, "Redis Pool Error: {}", e),
                _ => write!(f, "{:?}", self)
            }
        }
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
}


pub mod postgres {
    use std::fmt;
    use axum::{http::StatusCode, response::{IntoResponse, Response}};
    
    #[derive(Debug)]
    pub enum Error {
        PostgresError(sqlx::Error),
        JsonError(serde_json::Error),
        NotFound(String),
        BadRequest(String),
        PoolError(String)
    }
    impl std::error::Error for Error {}
    
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::PoolError(e) => write!(f, "Postgres Pool Error: {}", e),
                _ => write!(f, "{:?}", self)
            }
        }
    }

    impl From<sqlx::Error> for Error {
        fn from(err: sqlx::Error) -> Self {
            Error::PostgresError(err)
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
                Error::PostgresError(e) => {
                    tracing::error!("Postgres error: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal Server Error: Database operation failed with error: {}", e))
                }
                Error::JsonError(e) => {
                    tracing::error!("JSON error: {:?}", e);
                    (StatusCode::BAD_REQUEST, format!("JSON processing error: {}", e))
                }
                Error::NotFound(resource) => (StatusCode::NOT_FOUND, format!("Resource not found: {}", resource)),
                Error::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
                Error::PoolError(e) => {
                    tracing::error!("Postgres Pool error: {:?}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error: Failed to get connection".to_string())
                }
            };

            (status, error_message).into_response()
        }
    }
}
