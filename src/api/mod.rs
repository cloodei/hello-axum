use axum::{extract::{Path, State}, response::IntoResponse, http::StatusCode, Json};
use redis::AsyncCommands;
use serde_json::{from_str, to_string};
use crate::{error::redis::Error, prelude::redis::*};

const NEXT_ID_KEY: &str = "next_item_id";
const ITEM_INDEX_KEY: &str = "items_index";

fn item_key(id: usize) -> String {
    format!("item:{}", id)
}

/// POST /api/items - Create a new item
pub async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItemPayload>,
) -> Result<impl IntoResponse> {
    let mut con = state.redis_pool.get().await.map_err(map_pool_error)?;

    // 1. Get a new unique ID atomically
    let new_id: usize = con.incr(NEXT_ID_KEY, 1).await?;

    // 2. Create the full Item struct
    let new_item = Item {
        id: new_id,
        name: payload.name,
        description: payload.description,
        count: payload.count,
        height: payload.height,
        weight: payload.weight,
    };

    // 3. Serialize the item to JSON
    let item_json = to_string(&new_item)?;
    let key = item_key(new_id);

    // 4. Store the item in Redis
    con.set::<_, _, ()>(&key[..], item_json).await?;
    
    Ok((StatusCode::CREATED, Json(new_item)))
}

/// GET /api/items - List all items
pub async fn get_items(State(state): State<AppState>) -> Result<Json<Vec<Item>>> {
    let mut con = state.redis_pool.get().await.map_err(map_pool_error)?;

    // 1. Get all item keys from the index set
    let item_keys: Vec<String> = con.smembers(ITEM_INDEX_KEY).await?;

    if item_keys.is_empty() {
        return Ok(Json(Vec::new()));
    }

    // 2. Fetch all items using MGET (efficiently gets multiple keys)
    let items_json: Vec<Option<String>> = con.mget(item_keys).await?;

    // 3. Deserialize JSON strings into Item structs, filtering out None values
    //    (in case an item was deleted but MGET ran before SREM finished, though unlikely with atomic DEL)
    let items: Vec<Item> = items_json
        .into_iter()
        .flatten() // Remove None options
        .filter_map(|json_str| from_str(&json_str).ok())
        .collect();

    Ok(Json(items))
}

/// GET /api/items/{id} - Get a specific item by ID
pub async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> Result<Json<Item>> {
    let mut con = state.redis_pool.get().await.map_err(map_pool_error)?;
    let key = item_key(id);

    let item_json: Option<String> = con.get(&key[..]).await?;

    match item_json {
        Some(json_str) => {
            // tracing::info!("✅ Fetched item with ID: {}", id);
            Ok(Json(from_str(&json_str)?))
        },
        None => Err(Error::NotFound(format!("Item ID: {}", id)))
    }
}

/// PUT /api/items/{id} - Update an existing item
pub async fn update_item(
    State(state): State<AppState>,
    Path(id): Path<usize>,
    Json(payload): Json<CreateItemPayload>, // Reuse payload struct for updates
) -> Result<Json<Item>> {
    let mut con = state.redis_pool.get().await.map_err(map_pool_error)?;
    let key = item_key(id);

    // 1. Check if item exists before updating (optional, depends on desired PUT semantics)
    let exists: bool = con.exists(&key).await?;
    if !exists {
        return Err(Error::NotFound(format!("Item ID: {}", id)));
    }

    // 2. Create the updated item struct (ensure ID remains the same)
    let updated_item = Item {
        id,
        name: payload.name,
        description: payload.description,
        count: payload.count,
        height: payload.height,
        weight: payload.weight,
    };

    // 3. Serialize and overwrite the item in Redis
    let item_json = to_string(&updated_item)?;
    let x = &key[..];
    con.set::<_, _, ()>(x, item_json).await?;

    // tracing::info!("✅ Updated item with ID: {}", id);
    Ok(Json(updated_item))
}

/// DELETE /api/items/:id - Delete an item by ID
pub async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> Result<StatusCode> {
    let mut con = state.redis_pool.get().await.map_err(map_pool_error)?;
    let key = item_key(id);

    // Use a transaction to remove the item data and its key from the index atomically
    let (del_count, _): (isize, isize) = redis::pipe()
        .atomic()
        .del(&key) // Delete the item hash/JSON
        .srem(ITEM_INDEX_KEY, &key) // Remove the key from the index set
        .query_async(&mut *con)
        .await?;

    if del_count == 0 {
        // If DEL returned 0, the item key didn't exist
        Err(Error::NotFound(format!("Item ID: {}", id)))
    }
    else {
        // SREM might return 0 if key was already removed, which is fine if DEL succeeded
        // tracing::info!("✅ Deleted item with ID: {}", id);
        Ok(StatusCode::NO_CONTENT)
    }
}
