use crate::{db::Database, error::BotError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IncomingEvent {
    pub user_id: i64,
    pub text: String,
    pub timestamp: i64,
}

pub async fn handle_event(
    db: &Database,
    event: IncomingEvent,
) -> Result<String, BotError> {
    let user = db.init_user(event.user_id).await?;

    match db.get_active_cycle(user.user_id).await? {
        Some(cycle_id) => process_existing_cycle(db, cycle_id, &event.text).await,
        None => start_new_cycle(db, user.user_id, &event.text).await,
    }
}

async fn start_new_cycle(db: &Database, user_id: i64, text: &str) -> Result<String, BotError> {
    // Логика инициализации нового цикла
    Ok("Новый диалог начат".into())
}
