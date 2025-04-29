mod bot;
mod db;
mod error;

use crate::{bot::MaxBot, db::Database, error::BotError};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), BotError> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    // Подключение к БД
    let database_url = std::env::var("DATABASE_URL")
    .map_err(|_| BotError::ConfigError("DATABASE_URL not set".into()))?;

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;

    let db = Database::new(pool);
    let bot = MaxBot::new(db);

    let cycle_id = bot.handle_start(123456789).await?;

    bot.db.save_response(cycle_id, 1, "Фаза 1: Начало диалога")
    .await?;

    sleep(Duration::from_secs(1)).await;

    bot.db.save_response(cycle_id, 2, "Фаза 2: Деконструкция")
    .await?;

    Ok(())
}
