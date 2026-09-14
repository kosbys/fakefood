use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{
    AppState,
    handlers::order::{clear_orders, create_order, get_user_orders},
};

async fn order_router(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_order))
        .route("/", get(get_user_orders))
        .route("/clear", delete(clear_orders))
        .with_state(state)
}
