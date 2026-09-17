use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
}
