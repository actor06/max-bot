use crate::db::Database;
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;

pub struct MaxBot {
    db: Arc<Database>,  // Используем Arc для потокобезопасности
    client: Client,
    api_url: String,
}

impl MaxBot {
    pub fn new(db: Database) -> Self {
        Self {
            client: Client::new(),
            db: Arc::new(db),  // Обернули в Arc
            api_url: "https://api.max.ru/bot".to_string(),
        }
    }

    pub async fn handle_start(&self, user_id: i64) -> Result<i32, Box<dyn std::error::Error>> {
        let cycle_id = self.db.start_cycle(user_id).await?;

        self.send_message(
            user_id,
            &format!("RAB | BOT_ID | {} | Цикл #{}", user_id, cycle_id)
        ).await?;

        Ok(cycle_id)
    }

    pub async fn save_response(&self, cycle_id: i32, phase: i32, response: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.db.save_response(cycle_id, phase, response).await?;
        Ok(())
    }

    async fn send_message(&self, chat_id: i64, text: &str) -> Result<(), reqwest::Error> {
        self.client
        .post(&format!("{}/messages", self.api_url))
        .json(&json!({
            "chat_id": chat_id,
            "text": text
        }))
        .send()
        .await?;
        Ok(())
    }
}
