mod api;
mod events;

use crate::{db::Database, error::BotError};
use api::ApiClient;
use std::sync::Arc;

pub struct BotCore {
    db: Arc<Database>,
    api: ApiClient,
}

impl BotCore {
    pub fn new(db: Database, api_url: String, api_token: String) -> Self {
        Self {
            db: Arc::new(db),
            api: ApiClient::new(api_url, api_token),
        }
    }

    pub async fn process_message(&self, user_id: i64, text: &str) -> Result<(), BotError> {
        let response = events::handle_event(&self.db, user_id, text).await?;
        self.api.send_response(user_id, &response).await
    }
}
