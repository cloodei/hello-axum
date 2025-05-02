use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::api::postgres::{create_datas, get_data, get_datas};

pub async fn redis() -> anyhow::Result<()> {
    use bb8_redis::{bb8, RedisConnectionManager};
    use axum::{Router, routing::{get, post}};
    use crate::prelude::redis::*;
    use crate::api::*;
    
    tracing_subscriber::fmt()
        .without_time()
        .with_thread_names(false)
        .with_line_number(true)
        .pretty()
        .with_target(false)
        .with_level(true)
        .init();

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = RedisConnectionManager::new(redis_url)?;
    let redis_pool = bb8::Pool::builder().build(manager).await?;
    let app_state = AppState { redis_pool };

    let app = Router::new()
        .route(
            "/api/items",
            post(redis::create_item).get(redis::get_items)
        )
        .route(
            "/api/items/{id}",
            get(redis::get_item).put(redis::update_item).delete(redis::delete_item)
        )
        .with_state(app_state);

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("🚀 Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}


pub async fn postgres() -> anyhow::Result<()> {
    use axum::{Router, routing::get};
    use crate::prelude::postgres::*;

    tracing_subscriber::fmt()
        .without_time()
        .with_thread_names(false)
        .with_line_number(true)
        .pretty()
        .with_target(false)
        .with_level(true)
        .init();

    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_pool = PgPool::connect(&database_url).await?;

    let app_state = AppState { pg_pool };

    let app = Router::new()
        .route("/datas", get(get_datas).post(create_datas))
        .route("/datas/{id}", get(get_data))
        .with_state(app_state);

    let lstn = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(lstn, app).await?;

    Ok(())
}
