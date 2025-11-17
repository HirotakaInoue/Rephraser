//! Configuration module

pub mod manager;
pub mod models;

pub use manager::ConfigManager;
pub use models::{ActionConfig, Config, LlmConfig, OutputConfig, OutputMethod};
