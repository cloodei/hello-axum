use axum::{extract::Path, Json, http::StatusCode};
use crate::{error::tok_postgres::Error, prelude::tok_postgres::{Datas, DatasPayload, PgConnection, Result}};


pub async fn get_datas(PgConnection(state): PgConnection) -> Result<Json<Vec<Datas>>> {
    let res = state.client
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
    PgConnection(state): PgConnection,
    Path(id): Path<i32>
) -> Result<Json<Datas>> {
    let res = state.client.query_opt(&state.get_data, &[&id]).await?;

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
    PgConnection(state): PgConnection,
    Json(payload): Json<DatasPayload>
) -> Result<(StatusCode, Json<i32>)> {
    let id = state.client.query_one(&state.create_datas, &[&payload.name, &payload.flags, &payload.sys]).await?;

    Ok((StatusCode::CREATED, Json(id.get(0))))
}

pub async fn edit_datas(
    PgConnection(state): PgConnection,
    Path(id): Path<i32>,
    Json(payload): Json<DatasPayload>,
) -> Result<StatusCode> {
    state.client.execute(&state.edit_datas, &[&payload.name, &payload.flags, &payload.sys, &id]).await?;

    Ok(StatusCode::OK)
}

pub async fn destroy_datas(
    PgConnection(state): PgConnection,
    Path(id): Path<i32>
) -> Result<()> {
    state.client.execute(&state.destroy_datas, &[&id]).await?;

    Ok(())
}
