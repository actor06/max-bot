use crate::{db::Database, error::BotError};

pub struct MaxBot {
    pub db: Database, // Делаем поле публичным
}

impl MaxBot {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn handle_start(&self, user_id: i64) -> Result<i64, BotError> {
        Ok(1) // Заглушка для примера
    }
}
