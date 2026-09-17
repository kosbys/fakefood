use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

use crate::{AppState, models::product::Product};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddProductRequest {
    pub name: String,
    pub price: i32,
}

async fn get_all_products(State(state): State<AppState>) -> Result<Json<Vec<Product>>, String> {
    let products = sqlx::query_as!(Product, "SELECT id, name, price FROM products")
        .fetch_all(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(products))
}

async fn create_product(
    State(state): State<AppState>,
    Json(order): Json<AddProductRequest>,
) -> Result<_, _> {
}

async fn delete_product(State(state): State<AppState>) {
    todo!()
}

async fn get_product(State(state): State<AppState>) {}
