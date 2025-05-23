pub mod redis {
    use serde::{Deserialize, Serialize};
    use crate::error::redis::Error;

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
    
    pub type Result<T> = std::result::Result<T, Error>;
}


pub mod sqlx {
    use serde::{Deserialize, Serialize};
    use crate::error::sqlx::Error;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Datas {
        pub id: i32,
        pub name: String,
        pub flags: i64,
        pub sys: i16
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct DatasPayload {
        pub name: String,
        pub flags: i64,
        pub sys: i16
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Niceties {
        pub id: i32,
        pub datas_id: i32,
        pub mem: i64,
        pub stack: i16,
        pub info: String
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct NicetiesPaylod {
        pub datas_id: i32,
        pub mem: i64,
        pub stack: i16,
        pub info: String
    }

    #[derive(Clone)]
    pub struct AppState {
        pub pg_pool: sqlx::Pool<sqlx::Postgres>
    }

    pub type Result<T> = std::result::Result<T, Error>;
}


pub mod tok_postgres {
    use std::{convert::Infallible, sync::Arc};
    use serde::{Deserialize, Serialize};
    use tokio_postgres::{Client, Statement};
    use axum::{extract::FromRequestParts, http::request::Parts};
    use crate::error::tok_postgres::Error;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Datas {
        pub id: i32,
        pub name: String,
        pub flags: i64,
        pub sys: i16
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct DatasPayload {
        pub name: String,
        pub flags: i64,
        pub sys: i16
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Niceties {
        pub id: i32,
        pub datas_id: i32,
        pub mem: i64,
        pub stack: i16,
        pub info: String
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct NicetiesPaylod {
        pub datas_id: i32,
        pub mem: i64,
        pub stack: i16,
        pub info: String
    }

    #[derive(Clone)]
    pub struct AppState {
        pub pg_pool: PgPool,
        pub get_datas: Statement,
        pub get_data: Statement,
        pub create_datas: Statement,
        pub edit_datas: Statement,
        pub destroy_datas: Statement,
    }

    pub struct PgClient {
        pub client: Client,
        pub get_datas: Statement,
        pub get_data: Statement,
        pub create_datas: Statement,
        pub edit_datas: Statement,
        pub destroy_datas: Statement,
    }

    pub struct PgConnection(pub Arc<PgClient>);

    impl FromRequestParts<Arc<PgClient>> for PgConnection {
        type Rejection = Infallible;

        async fn from_request_parts(_: &mut Parts, pg_connection: &Arc<PgClient>) -> std::result::Result<Self, Self::Rejection> {
            Ok(Self(pg_connection.clone()))
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;
    pub type PgPool = bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>>;
}
