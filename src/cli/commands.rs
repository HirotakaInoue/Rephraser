//! CLI command implementations

use crate::actions::ActionResolver;
use crate::config::ConfigManager;
use crate::error::{RephraserError, Result};
use crate::llm::{AnthropicClient, LlmClient, MockLlmClient, OpenAiClient};
use crate::output::OutputHandler;
use std::sync::Arc;

/// Execute the rephrase command
pub async fn rephrase(action: &str, text: &str) -> Result<()> {
    // Load configuration
    let config_manager = ConfigManager::new()?;
    let config = config_manager.load()?;

    // Resolve action to prompt
    let resolver = ActionResolver::new(&config);
    let prompt = resolver.resolve(action, text)?;

    // Create LLM client based on config
    let client = create_llm_client(&config)?;

    // Call LLM API
    let response = client.complete(&prompt).await?;

    // Handle output
    let output_handler = OutputHandler::new(config.output.method);
    output_handler.handle(&response)?;

    Ok(())
}

/// List all available actions
pub async fn list_actions() -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let config = config_manager.load()?;

    let resolver = ActionResolver::new(&config);
    let actions = resolver.list_actions();

    println!("Available actions:");
    println!();

    for action in actions {
        println!("  {} ({})", action.name, action.display_name);
    }

    Ok(())
}

/// Initialize configuration
pub async fn config_init() -> Result<()> {
    let config_manager = ConfigManager::new()?;

    if config_manager.exists() {
        return Err(RephraserError::Config(format!(
            "Config file already exists at: {:?}",
            config_manager.config_path()
        )));
    }

    config_manager.init()?;

    println!(
        "Configuration initialized at: {:?}",
        config_manager.config_path()
    );
    println!();
    println!("Edit the file to customize your settings.");
    println!("Don't forget to set your API key environment variable!");

    Ok(())
}

/// Show current configuration
pub async fn config_show() -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let config = config_manager.load()?;

    let toml_str = toml::to_string_pretty(&config)
        .map_err(|e| RephraserError::Config(format!("Failed to serialize config: {}", e)))?;

    println!("Current configuration:");
    println!();
    println!("{}", toml_str);

    Ok(())
}

/// Set a configuration value
pub async fn config_set(key: &str, value: &str) -> Result<()> {
    // TODO: Implement config value setting
    // This requires parsing the key path and updating nested values
    println!("[TODO] Set {} = {}", key, value);
    println!("For now, please edit the config file directly.");

    Ok(())
}

/// Show configuration file path
pub async fn config_path() -> Result<()> {
    let config_manager = ConfigManager::new()?;
    println!("{}", config_manager.config_path().display());

    Ok(())
}

/// Create an LLM client based on configuration
fn create_llm_client(config: &crate::config::Config) -> Result<Arc<dyn LlmClient>> {
    match config.llm.provider.as_str() {
        "openai" => {
            let api_key = std::env::var(&config.llm.api_key_env).map_err(|_| {
                RephraserError::Config(format!(
                    "Environment variable '{}' not found",
                    config.llm.api_key_env
                ))
            })?;

            Ok(Arc::new(OpenAiClient::new(
                api_key,
                config.llm.model.clone(),
                config.llm.parameters.temperature,
                config.llm.parameters.max_tokens,
            )))
        }
        "anthropic" => {
            let api_key = std::env::var(&config.llm.api_key_env).map_err(|_| {
                RephraserError::Config(format!(
                    "Environment variable '{}' not found",
                    config.llm.api_key_env
                ))
            })?;

            Ok(Arc::new(AnthropicClient::new(
                api_key,
                config.llm.model.clone(),
                config.llm.parameters.temperature,
                config.llm.parameters.max_tokens,
            )))
        }
        "mock" => Ok(Arc::new(MockLlmClient::new())),
        _ => Err(RephraserError::Config(format!(
            "Unknown provider: {}",
            config.llm.provider
        ))),
    }
}
