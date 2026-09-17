use crate::{AppState, error::AppError, models::product::Product};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddProductRequest {
    pub name: String,
    pub price: i32,
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Product>, AppError> {
    let product = sqlx::query_as!(Product, "SELECT * FROM products WHERE id = $1", id)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(product))
}

pub async fn get_all_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<Product>>, AppError> {
    let products = sqlx::query_as!(Product, "SELECT id, name, price FROM products")
        .fetch_all(&state.db)
        .await?;

    Ok(Json(products))
}

// TODO: VERIFICATION FOR REAL PRODUCT
pub async fn create_product(
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

// need this?
pub async fn update_product() {
    todo!()
}

pub async fn delete_product(
    State(state): State<AppState>,
    Json(id): Json<i32>,
) -> Result<StatusCode, AppError> {
    sqlx::query_as!(Product, "DELETE FROM products WHERE id = $1", id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::OK)
}
