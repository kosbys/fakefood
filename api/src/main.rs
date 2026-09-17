mod cookie;
mod db;
mod error;
mod extractors;
mod handlers;
mod middleware;
mod models;
mod routes;

use db::create_pool;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

use crate::routes::create_router;

#[derive(Clone)]
struct AppState {
    pub db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();

    let db = create_pool(&db_url).await;

    let state = AppState { db };

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Working");
    axum::serve(listener, app).await.unwrap();
}
