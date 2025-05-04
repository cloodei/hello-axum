use axum::{extract::{Path, State}, Json, http::StatusCode};
use crate::{error::tok_postgres::{map_pool_error, Error}, prelude::tok_postgres::{Datas, DatasPayload, PgPool, Result}};


pub async fn get_datas(State(pool): State<PgPool>) -> Result<Json<Vec<Datas>>> {
    let conn = pool.get().await.map_err(map_pool_error)?;

    let res = conn
        .query_typed("SELECT * FROM items.datas", &[])
        .await?
        .drain(..)
        .map(|x| {
            Datas {
                id: x.get("id"),
                name: x.get("name"),
                flags: x.get("flags"),
                sys: x.get("sys")
            }
        })
        .collect();

    Ok(Json(res))
}

pub async fn get_data(
    State(pool): State<PgPool>,
    Path(id): Path<i32>
) -> Result<Json<Datas>> {
    let conn = pool.get().await.map_err(map_pool_error)?;

    let res = conn.query_opt("SELECT * FROM items.datas WHERE id = $1", &[&id]).await?;

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
    State(pool): State<PgPool>,
    Json(payload): Json<DatasPayload>
) -> Result<(StatusCode, Json<i32>)> {
    let conn = pool.get().await.map_err(map_pool_error)?;

    let id = conn
        .query_one(
            "INSERT INTO items.datas (name, flags, sys) VALUES ($1, $2, $3) RETURNING id",
            &[&payload.name, &payload.flags, &payload.sys]
        )
        .await?;

    Ok((StatusCode::CREATED, Json(id.get(0))))
}

pub async fn edit_datas(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<DatasPayload>,
) -> Result<StatusCode> {
    let conn = pool.get().await.map_err(map_pool_error)?;

    conn
        .execute(
            "UPDATE items.datas SET name = $1, flags = $2, sys = $3 WHERE id = $4",
            &[&payload.name, &payload.flags, &payload.sys, &id]
        )
        .await?;

    Ok(StatusCode::OK)
}

pub async fn destroy_datas(
    State(pool): State<PgPool>,
    Path(id): Path<i32>
) -> Result<()> {
    let conn = pool.get().await.map_err(map_pool_error)?;

    conn.execute("DELETE FROM items.datas WHERE id = $1", &[&id]).await?;

    Ok(())
}
