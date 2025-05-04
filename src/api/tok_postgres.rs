use axum::{extract::{Path, State}, Json, http::StatusCode};
use crate::{error::tok_postgres::{map_pool_error, Error}, prelude::tok_postgres::{AppState, Datas, DatasPayload, Result}};


pub async fn get_datas(State(state): State<AppState>) -> Result<Json<Vec<Datas>>> {
    let conn = state.pg_pool.get().await.map_err(map_pool_error)?;

    let res = conn
        .query(&state.get_datas, &[])
        .await?
        .drain(..)
        .map(|x| {
            Datas {
                id: x.get(0),
                name: x.get(1),
                flags: x.get(2),
                sys: x.get(3),
            }
        })
        .collect();

    Ok(Json(res))
}

pub async fn get_data(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Result<Json<Datas>> {
    let conn = state.pg_pool.get().await.map_err(map_pool_error)?;

    let res = conn.query_opt(&state.get_data, &[&id]).await?;

    match res {
        Some(x) => Ok(Json(Datas {
            id: x.get(0),
            name: x.get(1),
            flags: x.get(2),
            sys: x.get(3),
        })),
        None => Err(Error::NotFound("Not here btw".to_string()))
    }
}

pub async fn create_datas(
    State(state): State<AppState>,
    Json(payload): Json<DatasPayload>
) -> Result<(StatusCode, Json<i32>)> {
    let conn = state.pg_pool.get().await.map_err(map_pool_error)?;
    let id = conn.query_one(&state.create_datas, &[&payload.name, &payload.flags, &payload.sys]).await?;

    Ok((StatusCode::CREATED, Json(id.get(0))))
}

pub async fn edit_datas(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<DatasPayload>,
) -> Result<StatusCode> {
    let conn = state.pg_pool.get().await.map_err(map_pool_error)?;
    conn.execute(&state.edit_datas, &[&payload.name, &payload.flags, &payload.sys, &id]).await?;

    Ok(StatusCode::OK)
}

pub async fn destroy_datas(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Result<()> {
    let conn = state.pg_pool.get().await.map_err(map_pool_error)?;
    conn.execute(&state.destroy_datas, &[&id]).await?;

    Ok(())
}
