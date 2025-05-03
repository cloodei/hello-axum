use anyhow::Result;
use bb8_postgres::PostgresConnectionManager;
use tokio::net::TcpListener;
use axum::{Router, routing::get};

pub async fn redis() -> Result<()> {
    use bb8_redis::{bb8, RedisConnectionManager};
    use crate::prelude::redis::*;
    use crate::api::redis::*;
    
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
        .route("/api/items", get(get_items).post(create_item))
        .route(
            "/api/items/{id}",
            get(get_item).put(update_item).delete(delete_item)
        )
        .with_state(app_state);

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("🚀 Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}


pub async fn sqlx() -> Result<()> {
    use sqlx::PgPool;
    use crate::prelude::sqlx::*;
    use crate::api::sqlx::*;

    tracing_subscriber::fmt()
        .without_time()
        .with_thread_names(false)
        .with_line_number(true)
        .pretty()
        .with_target(false)
        .with_level(true)
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_pool = PgPool::connect(&database_url).await?;
    let app_state = AppState { pg_pool };

    let app = Router::new()
        .route("/api/datas", get(get_datas).post(create_datas))
        .route("/api/datas/{id}", get(get_data).put(edit_datas).delete(destroy_datas))
        .with_state(app_state);

    let lstn = TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("🚀 Server listening on http://localhost:3000/api/datas");

    axum::serve(lstn, app).await?;

    Ok(())
}


pub async fn tok_postgres() -> Result<()> {
    use tokio_postgres::NoTls;
    use crate::api::tok_postgres::*;

    tracing_subscriber::fmt()
        .without_time()
        .with_thread_names(false)
        .with_line_number(true)
        .pretty()
        .with_target(false)
        .with_level(true)
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = PostgresConnectionManager::new(database_url.parse()?, NoTls);
    let pool = bb8::Pool::builder().build(manager).await?;

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let app = Router::new()
        // .route("/api/datas", get(get_datas).post(create_datas))
        // .route("/api/datas/{id}", get(get_data).put(edit_datas).delete(destroy_datas))
        .with_state(pool);

    tracing::info!("🚀 Server listening on http://localhost:3000/api/datas");

    axum::serve(listener, app).await?;

    Ok(())
}
