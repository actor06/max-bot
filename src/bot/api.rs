use reqwest::Client;
use serde_json::json;
use crate::error::BotError;

const MAX_API_TIMEOUT: u64 = 10;

pub struct ApiClient {
    client: Client,
    base_url: String,
    token: String,
}

impl ApiClient {
    pub fn new(base_url: String, token: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            token,
        }
    }

    pub async fn send_response(&self, chat_id: i64, text: &str) -> Result<(), BotError> {
        let response = self.client
        .post(&format!("{}/sendMessage", self.base_url))
        .timeout(std::time::Duration::from_secs(MAX_API_TIMEOUT))
        .header("Authorization", &self.token)
        .json(&json!({
            "chat_id": chat_id,
            "text": text,
            "parse_mode": "Markdown"
        }))
        .send()
        .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            return Err(BotError::ApiFailure(format!(
                "Status: {}, Body: {}",
                status, body
            )));
        }

        Ok(())
    }
}
