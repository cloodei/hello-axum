use crate::{error::postgres::Error, prelude::postgres::{AppState, Datas, DatasPayload, Result}};
use axum::{extract::{Path, State}, http::StatusCode, Json};
use sqlx::{query_as, query};

pub async fn get_datas(State(app): State<AppState>) -> Result<Json<Vec<Datas>>> {
    let x = query_as!(Datas, "SELECT * FROM items.datas").fetch_all(&app.pg_pool).await?;

    Ok(Json(x))
}

pub async fn get_data(
    State(app): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Datas>> {
    let x = query_as!(Datas, "SELECT * FROM items.datas WHERE id = $1", id).fetch_optional(&app.pg_pool).await?;

    match x {
        Some(x) => Ok(Json(x)),
        None => Err(Error::NotFound("Invalid ID, didn't find the requested data".to_string()))
    }
}

pub async fn create_datas(
    State(app): State<AppState>,
    Json(payload): Json<DatasPayload>,
) -> Result<StatusCode> {
    query!(
        "INSERT INTO items.datas (name, flags, sys) VALUES ($1, $2, $3)",
        payload.name,
        payload.flags,
        payload.sys,
    ).execute(&app.pg_pool).await?;

    Ok(StatusCode::CREATED)
}

pub async fn edit_datas(
    State(app): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<DatasPayload>,
) -> Result<Json<i32>> {
    let x = query!(
        "UPDATE items.datas SET name = $1, flags = $2, sys = $3 WHERE ID = $4 RETURNING id",
        payload.name,
        payload.flags,
        payload.sys,
        id
    ).fetch_one(&app.pg_pool).await?;

    Ok(Json(x.id))
}

pub async fn destroy_datas(State(app): State<AppState>, Path(id): Path<i32>) -> Result<()> {
    query!("DELETE FROM items.datas WHERE id = $1", id).execute(&app.pg_pool).await?;

    Ok(())
}
