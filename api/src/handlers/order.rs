use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, error::AppError};

#[derive(Debug, Deserialize)]
pub struct AddOrderRequest {
    pub items: Vec<OrderItemRequest>,
    pub customer_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct OrderItemRequest {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderItemResponse {
    pub product_id: i32,
    pub quantity: i32,
    pub price: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    total: i32,
    status: String,
    id: i32,
    items: Vec<OrderItemResponse>,
}

async fn create_order(
    State(state): State<AppState>,
    Json(order): Json<AddOrderRequest>,
) -> Result<Json<OrderResponse>, AppError> {
    let products_id: Vec<i32> = order.items.iter().map(|item| item.product_id).collect();

    let products = sqlx::query!(
        "SELECT id, price FROM products WHERE id = ANY($1)",
        &products_id
    )
    .fetch_all(&state.db)
    .await?;

    let mut total = 0;

    for item in &order.items {
        let product = products
            .iter()
            .find(|product| product.id == item.product_id)
            .ok_or(AppError::NotFound)?;

        total += product.price * item.quantity;
    }

    let mut transaction = state.db.begin().await?;

    let order_id = sqlx::query!(
        r#"
        INSERT INTO orders (status, total) VALUES ('processing', $1) RETURNING id as "id!"
        "#,
        total
    )
    .fetch_one(&mut *transaction)
    .await?;

    let mut response_items = Vec::new();

    for item in &order.items {
        let product = products
            .iter()
            .find(|product| product.id == item.product_id)
            .ok_or(AppError::NotFound)?;

        sqlx::query!(
            "INSERT INTO order_items (order_id, product_id, quantity, price) VALUES ($1, $2, $3, $4)",
            order_id.id,
            item.product_id,
            item.quantity,
            product.price,
        ).execute(&mut *transaction).await?;

        response_items.push(OrderItemResponse {
            product_id: item.product_id,
            quantity: item.quantity,
            price: product.price,
        });
    }

    transaction.commit().await?;

    Ok(Json(OrderResponse {
        total,
        status: "processing".to_string(),
        id: order_id.id,
        items: response_items,
    }))
}

async fn get_user_orders() {}

async fn clear_orders() {}
