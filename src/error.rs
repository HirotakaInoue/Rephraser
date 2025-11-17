//! Error types for Rephraser

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RephraserError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Action '{0}' not found")]
    ActionNotFound(String),

    #[error("LLM API error: {0}")]
    LlmApi(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Input too long (max {max} characters, got {actual})")]
    InputTooLong { max: usize, actual: usize },

    #[error("Invalid template: {0}")]
    InvalidTemplate(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, RephraserError>;
