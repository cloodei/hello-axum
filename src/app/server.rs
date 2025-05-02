#![allow(non_snake_case, unused, unused_imports, dead_code)]

use bb8_redis::{bb8, RedisConnectionManager};
use axum::{extract::State, Router, routing::{get, post, put, delete}, response::IntoResponse};
use crate::prelude::redis::*;
use crate::api::*;
use crate::error::*;

pub async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
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
        .route("/api/items", post(create_item).get(get_items))
        .route("/api/items/{id}", get(get_item).put(update_item).delete(delete_item))
        .with_state(app_state);

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("🚀 Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
