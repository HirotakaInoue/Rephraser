//! LLM Client trait definition

use crate::error::Result;
use async_trait::async_trait;

/// Core trait for LLM clients
///
/// This trait abstracts different LLM providers (OpenAI, Anthropic, etc.)
/// allowing them to be used interchangeably.
#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Send a prompt to the LLM and receive a completion
    ///
    /// # Arguments
    /// * `prompt` - The text prompt to send to the LLM
    ///
    /// # Returns
    /// * `Result<String>` - The LLM's response text
    ///
    /// # Errors
    /// * Network errors
    /// * API errors (rate limits, invalid API key, etc.)
    /// * Response parsing errors
    async fn complete(&self, prompt: &str) -> Result<String>;

    /// Get the name of this LLM provider (e.g., "openai", "anthropic", "mock")
    fn provider_name(&self) -> &str;

    /// Get the model name being used (e.g., "gpt-4", "claude-3-sonnet")
    fn model_name(&self) -> &str;
}

/// Parameters for LLM API calls
#[derive(Debug, Clone)]
pub struct LlmParameters {
    pub temperature: f32,
    pub max_tokens: usize,
}

impl Default for LlmParameters {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: 500,
        }
    }
}
