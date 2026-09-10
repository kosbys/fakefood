use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Order {
    pub id: i32,
    pub user_id: Uuid,
    pub status: String,
    pub total: i32,
    pub created_at: DateTime<Utc>,
}
