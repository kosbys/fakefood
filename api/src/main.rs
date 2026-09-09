mod db;
mod models;
mod routes;

use axum::{Router, routing::get};
use db::create_pool;
use dotenv::dotenv;
use std::env;

struct AppState {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("database_string").unwrap();

    let database = create_pool(&db_url).await;

    let app = Router::new().route("/", get(|| async { "Hello" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Working");
    axum::serve(listener, app).await.unwrap();
}
