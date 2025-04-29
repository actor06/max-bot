use thiserror::Error;
use sqlx::Error as SqlxError;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Operation failed: {0}")]
    OperationError(String),
}

impl From<sqlx::Error> for BotError {
    fn from(err: sqlx::Error) -> Self {
        BotError::DatabaseError(err)
    }
}
