use sqlx::PgPool;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn start_cycle(&self, user_id: i64) -> Result<i32, sqlx::Error> {
        let cycle = sqlx::query!(
            r#"
            INSERT INTO cycles (user_id)
        VALUES ($1)
        RETURNING cycle_id
        "#,
        user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(cycle.cycle_id)
    }

    pub async fn save_response(&self, cycle_id: i32, phase: i32, response: &str) -> Result<(), sqlx::Error> {
        match phase {
            1 => sqlx::query!(
                "UPDATE cycles SET phase1_response = $1 WHERE cycle_id = $2",
                response,
                cycle_id
            ),
            2 => sqlx::query!(
                "UPDATE cycles SET phase2_response = $1 WHERE cycle_id = $2",
                response,
                cycle_id
            ),
            _ => return Err(sqlx::Error::Protocol("Invalid phase number".into()))
        }
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
