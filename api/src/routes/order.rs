use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::handlers::order::{clear_orders, create_order, get_user_orders};

async fn order_router() {
    let router = Router::new()
        .route("/", post(create_order))
        .route("/{id}", get(get_user_orders()))
        .route("/clear", delete(clear_orders()));
}
