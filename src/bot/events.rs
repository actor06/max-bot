//! Обработчики событий
use serde_json::Value;
use sqlx::PgPool;
use crate::db;

/// Обработка входящих сообщений
pub async fn handle_message(event: &Value, pool: &PgPool) -> Result<(), String> {
    let user_id = event["user_id"].as_i64().ok_or("Invalid user ID")?;
    let text = event["text"].as_str().unwrap_or_default();

    // Сохранение сообщения в БД
    db::save_message(user_id, text, pool)
    .await
    .map_err(|e| format!("DB error: {}", e))?;

    // Генерация ответа
    let response = generate_response(text).await;

    // Отправка ответа
    send_message(user_id, &response).await
}

/// Генерация ответа через AI
async fn generate_response(input: &str) -> String {
    // Временная заглушка
    format!("Вы сказали: {}. Это было глубокомысленно.", input)
}
