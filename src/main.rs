mod bot;
mod db;

use crate::{bot::MaxBot, db::Database};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::time::{sleep, Duration};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация логгера
    tracing_subscriber::fmt::init();

    dotenv().ok();
    info!("Starting bot initialization...");

    // Инициализация подключения к БД
    let pool = match PgPoolOptions::new()
    .max_connections(5)
    .connect(&env::var("DATABASE_URL")?)
    .await
    {
        Ok(pool) => {
            info!("Database connection established");
            pool
        }
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            return Err(e.into());
        }
    };

    let db = Database::new(pool);
    let bot = MaxBot::new(db);

    // Тестовый пользователь
    let user_id = 123456789;
    info!("Processing test user: {}", user_id);

    // Инициализация цикла
    let cycle_id = match bot.handle_start(user_id).await {
        Ok(id) => {
            info!("Cycle started successfully: {}", id);
            id
        }
        Err(e) => {
            error!("Failed to start cycle: {}", e);
            return Err(e);
        }
    };

    // Фаза 1: Первый ответ
    if let Err(e) = bot.save_response(
        cycle_id,
        1,
        "Фаза 1: Начало диалога с абстракцией"
    ).await {
        error!("Failed to save phase 1 response: {}", e);
        return Err(e);
    }

    // Имитация 24-часового ожидания
    info!("Waiting for phase 2...");
    sleep(Duration::from_secs(1)).await;  // В реальном коде используйте 86400 сек

    // Фаза 2: Финальный ответ
    if let Err(e) = bot.save_response(
        cycle_id,
        2,
        "Фаза 2: Деконструкция первоначального тезиса"
    ).await {
        error!("Failed to save phase 2 response: {}", e);
        return Err(e);
    }

    info!("Bot completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_db_connection() -> Result<(), sqlx::Error> {
        dotenv().ok();
        let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await?;

        let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(42_i64)
        .fetch_one(&pool)
        .await?;

        assert_eq!(row.0, 42);
        Ok(())
    }
}
