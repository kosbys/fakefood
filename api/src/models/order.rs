use chrono::{DateTime, Utc};

pub struct Order {
    pub id: i32,
    pub status: String,
    pub total: i32,
    pub created_at: DateTime<Utc>,
}
