use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: i64,
    pub current_cycle: i32,
    pub last_interaction: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cycle {
    pub cycle_id: i32,
    pub user_id: i64,
    pub phase1_response: Option<String>,
    pub phase2_response: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
