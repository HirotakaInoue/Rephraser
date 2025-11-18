//! OpenAI API client

use crate::error::{RephraserError, Result};
use crate::llm::client::LlmClient;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

/// Chat completion request message
#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// OpenAI chat completion request
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: usize,
}

/// OpenAI chat completion response choice
#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

/// OpenAI response message
#[derive(Debug, Deserialize)]
struct ChatResponseMessage {
    content: String,
}

/// OpenAI chat completion response
#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

/// Error response from OpenAI API
#[derive(Debug, Deserialize)]
struct OpenAiErrorResponse {
    error: OpenAiError,
}

#[derive(Debug, Deserialize)]
struct OpenAiError {
    message: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    error_type: String,
}

/// OpenAI API client
pub struct OpenAiClient {
    client: Client,
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
            client: Client::new(),
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
        // Construct request
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };

        // Send request
        let response = self
            .client
            .post(OPENAI_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        // Check status code
        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());

            // Try to parse structured error
            let error_msg = if let Ok(err_resp) = serde_json::from_str::<OpenAiErrorResponse>(&error_text) {
                err_resp.error.message
            } else {
                error_text
            };

            return Err(match status.as_u16() {
                401 | 403 => RephraserError::LlmAuth(format!("OpenAI authentication failed: {}", error_msg)),
                429 => RephraserError::LlmRateLimit(format!("OpenAI rate limit exceeded: {}", error_msg)),
                400 => RephraserError::LlmBadRequest(format!("OpenAI bad request: {}", error_msg)),
                _ => RephraserError::LlmServiceError(format!("OpenAI API error ({}): {}", status, error_msg)),
            });
        }

        // Parse successful response
        let completion_response: ChatCompletionResponse = response.json().await?;

        // Extract text from first choice
        completion_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| RephraserError::LlmApi("OpenAI returned no choices".to_string()))
    }

    fn provider_name(&self) -> &str {
        "openai"
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let request = ChatCompletionRequest {
            model: "gpt-4".to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
            temperature: 0.7,
            max_tokens: 500,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"model\":\"gpt-4\""));
        assert!(json.contains("\"temperature\":0.7"));
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": "Hello! How can I help?"
                }
            }]
        }"#;

        let response: ChatCompletionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.choices[0].message.content, "Hello! How can I help?");
    }

    #[test]
    fn test_error_response_parsing() {
        let json = r#"{
            "error": {
                "message": "Invalid API key",
                "type": "invalid_request_error"
            }
        }"#;

        let err_resp: OpenAiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(err_resp.error.message, "Invalid API key");
    }
}
