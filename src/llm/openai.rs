//! OpenAI API client

use crate::error::Result;
use crate::llm::client::LlmClient;
use async_trait::async_trait;

/// OpenAI API client
pub struct OpenAiClient {
    api_key: String,
    model: String,
    temperature: f32,
    max_tokens: usize,
}

impl OpenAiClient {
    /// Create a new OpenAI client
    ///
    /// # Arguments
    /// * `api_key` - OpenAI API key
    /// * `model` - Model name (e.g., "gpt-4", "gpt-3.5-turbo")
    /// * `temperature` - Temperature parameter (0.0-2.0)
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
impl LlmClient for OpenAiClient {
    async fn complete(&self, prompt: &str) -> Result<String> {
        // TODO: Implement actual OpenAI API call
        // For now, return a placeholder
        Ok(format!(
            "[OpenAI Mock] Would call {} with prompt: {}",
            self.model,
            &prompt[..prompt.len().min(50)]
        ))
    }

    fn provider_name(&self) -> &str {
        "openai"
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}
