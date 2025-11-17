//! Anthropic API client

use crate::error::Result;
use crate::llm::client::LlmClient;
use async_trait::async_trait;

/// Anthropic API client
pub struct AnthropicClient {
    api_key: String,
    model: String,
    temperature: f32,
    max_tokens: usize,
}

impl AnthropicClient {
    /// Create a new Anthropic client
    ///
    /// # Arguments
    /// * `api_key` - Anthropic API key
    /// * `model` - Model name (e.g., "claude-3-sonnet-20240229")
    /// * `temperature` - Temperature parameter (0.0-1.0)
    /// * `max_tokens` - Maximum tokens in response
    pub fn new(api_key: String, model: String, temperature: f32, max_tokens: usize) -> Self {
        Self {
            api_key,
            model,
            temperature,
            max_tokens,
        }
    }
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn complete(&self, prompt: &str) -> Result<String> {
        // TODO: Implement actual Anthropic API call
        // For now, return a placeholder
        Ok(format!(
            "[Anthropic Mock] Would call {} with prompt: {}",
            self.model,
            &prompt[..prompt.len().min(50)]
        ))
    }

    fn provider_name(&self) -> &str {
        "anthropic"
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}
