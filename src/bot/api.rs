//! Взаимодействие с MAX API
use reqwest::{Client, Error};
use serde_json::Value;
use tracing::info;

const MAX_API_URL: &str = "https://api.max.ru/bot/v1";

/// Запуск LongPoll-сервера
pub async fn start_longpoll(max_token: &str) -> Result<(), Error> {
    let client = Client::new();

    loop {
        let response = client
        .get(format!("{}/events", MAX_API_URL))
        .header("Authorization", format!("Bearer {}", max_token))
        .send()
        .await?
        .json::<Value>()
        .await?;

        if let Some(events) = response["events"].as_array() {
            for event in events {
                process_event(event).await?;
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

/// Основной обработчик событий
async fn process_event(event: &Value) -> Result<(), Error> {
    let event_type = event["type"].as_str().unwrap_or_default();

    match event_type {
        "message_new" => handle_message(event).await?,
        "button_pressed" => handle_button(event).await?,
        _ => info!("Unknown event type: {}", event_type),
    }

    Ok(())
}
