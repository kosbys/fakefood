use axum::{Router, routing::get};

use crate::{
    AppState,
    handlers::product::{get_all_products, get_product},
};

pub fn product_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_products))
        .route("/{id}", get(get_product))
        .with_state(state)
}
