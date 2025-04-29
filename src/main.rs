mod bot;
mod db;
mod error;

use crate::bot::BotCore;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Инициализация БД
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&env::var("DATABASE_URL")?)
    .await?;

    let db = db::Database::new(pool);

    // Создание экземпляра бота
    let bot = BotCore::new(
        db,
        env::var("API_BASE_URL")?,
                           env::var("BOT_TOKEN")?,
    );

    // Пример обработки сообщения
    bot.process_message(123456789, "Тестовый запрос").await?;

    Ok(())
}
