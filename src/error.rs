use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("API request failed: {0}")]
    ApiFailure(String),

    #[error("Invalid phase transition")]
    PhaseTransition,

    #[error("Authentication error")]
    AuthError,

    #[error("Configuration error")]
    ConfigError,

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

impl From<reqwest::Error> for BotError {
    fn from(e: reqwest::Error) -> Self {
        BotError::ApiFailure(e.to_string())
    }
}
