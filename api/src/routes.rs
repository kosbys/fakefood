use axum::Router;

pub mod admin;
pub mod order;
pub mod product;

pub fn router() -> Router {
    Router::new()
}
