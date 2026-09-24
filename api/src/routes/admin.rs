use axum::{
    Router, middleware,
    routing::{delete, post},
};

use crate::{
    AppState,
    handlers::{
        admin::admin_login,
        product::{create_product, delete_product},
    },
    middleware::admin::admin,
};

pub fn admin_router(state: AppState) -> Router {
    let admin_only = Router::new()
        .route("/products", post(create_product))
        .route("/products/{id}", delete(delete_product))
        .layer(middleware::from_fn(admin))
        .with_state(state.clone());

    // login route is public
    Router::new()
        .route("/", post(admin_login))
        .merge(admin_only)
}
