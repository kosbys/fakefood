use axum::{Router, middleware, routing::get};

use crate::{
    AppState,
    middleware::cookie::id_cookie,
    routes::{admin::admin_router, order::order_router, product::product_router},
};

pub mod admin;
pub mod order;
pub mod product;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Health Checked! 100%" }))
        .nest("/products", product_router(state.clone()))
        .nest("/orders", order_router(state.clone()))
        .nest("/admin", admin_router(state))
        .layer(middleware::from_fn(id_cookie))
}
