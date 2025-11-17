//! Configuration management

use crate::config::models::Config;
use crate::error::{RephraserError, Result};
use std::fs;
use std::path::PathBuf;

/// Configuration manager
pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    /// Create a new ConfigManager
    ///
    /// Uses ~/.rephraser/config.toml as the default path
    pub fn new() -> Result<Self> {
        let config_dir = dirs::home_dir()
            .ok_or_else(|| RephraserError::Config("Could not find home directory".to_string()))?
            .join(".rephraser");

        let config_path = config_dir.join("config.toml");

        Ok(Self { config_path })
    }

    /// Create a new ConfigManager with a custom path
    pub fn with_path(path: PathBuf) -> Self {
        Self { config_path: path }
    }

    /// Get the config file path
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }

    /// Load configuration from file
    ///
    /// If the file doesn't exist, returns default configuration
    pub fn load(&self) -> Result<Config> {
        if !self.config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&self.config_path)?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }

    /// Save configuration to file
    ///
    /// Creates the config directory if it doesn't exist
    pub fn save(&self, config: &Config) -> Result<()> {
        // Create config directory if it doesn't exist
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(config)
            .map_err(|e| RephraserError::Config(format!("Failed to serialize config: {}", e)))?;

        fs::write(&self.config_path, content)?;

        Ok(())
    }

    /// Initialize configuration with defaults
    ///
    /// Creates a new config file with default values if it doesn't exist
    /// Returns an error if the file already exists
    pub fn init(&self) -> Result<()> {
        if self.config_path.exists() {
            return Err(RephraserError::Config(format!(
                "Config file already exists at {:?}",
                self.config_path
            )));
        }

        let default_config = Config::default();
        self.save(&default_config)?;

        Ok(())
    }

    /// Check if config file exists
    pub fn exists(&self) -> bool {
        self.config_path.exists()
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default ConfigManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.llm.provider, "openai");
        assert_eq!(config.actions.len(), 3);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.llm.provider, config.llm.provider);
    }
}
