use sqlx::postgres::PgPool;

#[derive(Debug)]
pub struct Database {
    pub pool: PgPool, // Делаем поле публичным
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_response(
        &self,
        cycle_id: i64,
        phase: i32,
        response: &str
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO responses (cycle_id, phase, content) VALUES ($1, $2, $3)",
                     cycle_id,
                     phase,
                     response
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
