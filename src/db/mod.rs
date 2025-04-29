mod models;
use models::{User, Cycle};
use sqlx::PgPool;
use crate::error::BotError;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn init_user(&self, user_id: i64) -> Result<User, BotError> {
        let user = sqlx::query_as!(
            User,
            r#"INSERT INTO users (user_id) VALUES ($1)
        ON CONFLICT (user_id) DO NOTHING
        RETURNING *"#,
        user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_active_cycle(&self, user_id: i64) -> Result<Option<i32>, BotError> {
        let cycle = sqlx::query_scalar!(
            "SELECT current_cycle FROM users WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(cycle)
    }
}
