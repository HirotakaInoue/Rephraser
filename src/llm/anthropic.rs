//! Anthropic API client

use crate::error::{RephraserError, Result};
use crate::llm::client::LlmClient;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";

/// Anthropic message in the conversation
#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

/// Anthropic messages API request
#[derive(Debug, Serialize)]
struct MessagesRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: usize,
    temperature: f32,
}

/// Response content block
#[derive(Debug, Deserialize)]
struct ResponseContent {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    content_type: String,
    text: String,
}

/// Anthropic messages API response
#[derive(Debug, Deserialize)]
struct MessagesResponse {
    content: Vec<ResponseContent>,
}

/// Anthropic API error response
#[derive(Debug, Deserialize)]
struct AnthropicErrorResponse {
    error: AnthropicError,
}

#[derive(Debug, Deserialize)]
struct AnthropicError {
    message: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    error_type: String,
}

/// Anthropic API client
pub struct AnthropicClient {
    client: Client,
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
            client: Client::new(),
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
        // Construct request
        let request = MessagesRequest {
            model: self.model.clone(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: self.max_tokens,
            temperature: self.temperature,
        };

        // Send request
        let response = self
            .client
            .post(ANTHROPIC_API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        // Check status code
        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());

            // Try to parse structured error
            let error_msg = if let Ok(err_resp) = serde_json::from_str::<AnthropicErrorResponse>(&error_text) {
                err_resp.error.message
            } else {
                error_text
            };

            return Err(match status.as_u16() {
                401 | 403 => RephraserError::LlmAuth(format!("Anthropic authentication failed: {}", error_msg)),
                429 => RephraserError::LlmRateLimit(format!("Anthropic rate limit exceeded: {}", error_msg)),
                400 => RephraserError::LlmBadRequest(format!("Anthropic bad request: {}", error_msg)),
                _ => RephraserError::LlmServiceError(format!("Anthropic API error ({}): {}", status, error_msg)),
            });
        }

        // Parse successful response
        let messages_response: MessagesResponse = response.json().await?;

        // Extract text from first content block
        messages_response
            .content
            .first()
            .map(|content| content.text.clone())
            .ok_or_else(|| RephraserError::LlmApi("Anthropic returned no content".to_string()))
    }

    fn provider_name(&self) -> &str {
        "anthropic"
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
        let request = MessagesRequest {
            model: "claude-3-sonnet-20240229".to_string(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
            max_tokens: 500,
            temperature: 0.7,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"model\":\"claude-3-sonnet-20240229\""));
        assert!(json.contains("\"temperature\":0.7"));
        assert!(json.contains("\"role\":\"user\""));
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{
            "content": [{
                "type": "text",
                "text": "Hello! How can I assist you?"
            }]
        }"#;

        let response: MessagesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.content[0].text, "Hello! How can I assist you?");
    }

    #[test]
    fn test_error_response_parsing() {
        let json = r#"{
            "error": {
                "message": "Invalid API key",
                "type": "authentication_error"
            }
        }"#;

        let err_resp: AnthropicErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(err_resp.error.message, "Invalid API key");
    }
}
