use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState, error::AppError, extractors::current_user::CurrentUser, models::product::Product,
};

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

#[derive(Debug, Serialize, Deserialize)]
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

pub async fn create_order(
    CurrentUser { id }: CurrentUser,
    State(state): State<AppState>,
    Json(order): Json<AddOrderRequest>,
) -> Result<Json<OrderResponse>, AppError> {
    let products_id: Vec<i32> = order.items.iter().map(|item| item.product_id).collect();

    let products = sqlx::query_as!(
        Product,
        "SELECT id, name, price FROM products WHERE id = ANY($1)",
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
        INSERT INTO orders (status, total, user_id) VALUES ('processing', $1, $2) RETURNING id as "id!"
        "#,
        total, id
    )
    .fetch_one(&mut *transaction)
    .await?;

    let mut response_items = Vec::new();

    for item in &order.items {
        let product = products
            .iter()
            .find(|product| product.id == item.product_id)
            .ok_or(AppError::NotFound)?;

        sqlx::query_as!(OrderItem,
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

// find all orders from user, then get all info about them neatly
pub async fn get_user_orders(
    CurrentUser { id }: CurrentUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<OrderResponse>>, AppError> {
    // each order gets an array containing items in json shape
    let orders = sqlx::query!(
        r#"
        SELECT
            o.id,
            o.status,
            o.total,
            o.created_at,
            json_agg(
                json_build_object(
                    'product_id', oi.product_id,
                    'quantity', oi.quantity,
                    'price', oi.price
                )
            ) AS items
        FROM orders AS o
        JOIN order_items AS oi
            ON oi.order_id = o.id
        WHERE o.user_id = $1
          AND o.status = 'completed'
        GROUP BY o.id, o.status, o.total, o.created_at
        ORDER BY o.created_at DESC
        "#,
        &id
    )
    .fetch_all(&state.db)
    .await?;

    // deserializing to orderresponse
    let order_responses: Vec<OrderResponse> = orders
        .into_iter()
        .map(|o| {
            let items: Vec<OrderItemResponse> = serde_json::from_value(o.items.unwrap())
                .map_err(|e| AppError::BadRequest(e.to_string()))?;

            Ok(OrderResponse {
                id: o.id,
                status: o.status.to_string(),
                total: o.total,
                items,
            })
        })
        .collect::<Result<Vec<OrderResponse>, AppError>>()?;

    Ok(Json(order_responses))
}

pub async fn clear_orders(
    CurrentUser { id }: CurrentUser,
    State(state): State<AppState>,
) -> Result<StatusCode, AppError> {
    // cascades and deletes order items
    sqlx::query_as!(Order, "DELETE FROM orders WHERE user_id = $1", id)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::OK)
}
