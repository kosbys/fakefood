use axum::{Router, routing::post};

use crate::{AppState, handlers::admin::admin_login};

async fn admin_router(state: AppState) -> Router {
    Router::new().route("/admin", post(admin_login))
}
