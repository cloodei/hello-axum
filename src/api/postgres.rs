use crate::prelude::postgres::{AppState, Datas, DatasPayload, Result};
use axum::{extract::{Path, State}, response::IntoResponse, http::StatusCode, Json};
use sqlx::{query_as, query};

pub async fn get_datas(State(app): State<AppState>) -> Result<Json<Vec<Datas>>> {
    let x = query_as!(Datas, "SELECT * FROM items.datas").fetch_all(&app.pg_pool).await?;

    Ok(Json(x))
}

pub async fn get_data(State(app): State<AppState>, Path(id): Path<i32>) -> Result<Json<Datas>> {
    let x = query_as!(Datas, "SELECT * FROM items.datas WHERE id = $1", id).fetch_one(&app.pg_pool).await?;

    Ok(Json(x))
}

pub async fn create_datas(State(app): State<AppState>, Json(x): Json<DatasPayload>) -> Result<impl IntoResponse> {
    query!(
        "INSERT INTO items.datas (name, flags, sys) VALUES ($1, $2, $3)",
        x.name,
        x.flags,
        x.sys,
    ).execute(&app.pg_pool).await?;

    Ok(StatusCode::CREATED)
}
