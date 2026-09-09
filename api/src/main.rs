mod db;
mod routes;

use crate::db::create_pool;
use axum::{Router, routing::get};
use dotenv::dotenv;
use std::env;

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
