use axum::extract::State;
use sqlx::types::Json;

use crate::{AppState, models::product::Product};

async fn get_all_products(State(state): State<AppState>) -> Result<Json<Vec<Product>>, String> {
    let products = sqlx::query_as!(Product, "SELECT id, name, price FROM products")
        .fetch_all(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(products))
}

async fn get_product() {}
