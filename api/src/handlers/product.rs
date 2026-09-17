use axum::{extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

use crate::{AppState, error::AppError, models::product::Product};

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
    Json(product): Json<AddProductRequest>,
) -> Result<StatusCode, AppError> {
    sqlx::query!(
        "INSERT INTO products (name, price) VALUES ($1, $2)",
        product.name,
        product.price
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::OK)
}

async fn delete_product(
    State(state): State<AppState>,
    Json(id): Json<i32>,
) -> Result<StatusCode, AppError> {
    sqlx::query_as!(Product, "DELETE FROM products WHERE id = $1", id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::OK)
}

async fn get_product(State(state): State<AppState>) {}
