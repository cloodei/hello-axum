pub mod redis {
    use std::fmt;

    #[derive(Debug)]
    pub enum Error {
        RedisError(redis::RedisError),
        JsonError(serde_json::Error),
        NotFound(String),
        BadRequest(String),
        PoolError(String)
    }
    
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::PoolError(e) => write!(f, "Redis Pool Error: {}", e),
                _ => write!(f, "{:?}", self)
            }
        }
    }
    impl std::error::Error for Error {}
}
