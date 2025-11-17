//! Mock LLM client for testing

use crate::error::Result;
use crate::llm::client::LlmClient;
use async_trait::async_trait;
use std::collections::HashMap;

/// Mock LLM client that returns predefined responses
///
/// Useful for testing without making actual API calls
pub struct MockLlmClient {
    responses: HashMap<String, String>,
    default_response: String,
}

impl MockLlmClient {
    /// Create a new mock client with predefined responses
    pub fn new() -> Self {
        let mut responses = HashMap::new();

        // Predefined responses for common actions
        responses.insert(
            "polite".to_string(),
            "こんにちは、お元気でしょうか。いつもありがとうございます。".to_string(),
        );

        responses.insert(
            "organize".to_string(),
            r#"整理されたテキスト：

1. 主要ポイント
   - 項目A
   - 項目B

2. 詳細説明
   - 説明1
   - 説明2

3. まとめ
   - 結論"#
                .to_string(),
        );

        responses.insert(
            "summarize".to_string(),
            "要約: このテキストは主要な3つのポイントを含んでいます。".to_string(),
        );

        Self {
            responses,
            default_response: "[Mock LLM Response] Processed successfully.".to_string(),
        }
    }

    /// Add or update a custom response for a specific action
    pub fn add_response(&mut self, action: impl Into<String>, response: impl Into<String>) {
        self.responses.insert(action.into(), response.into());
    }

    /// Set the default response for unknown actions
    pub fn set_default_response(&mut self, response: impl Into<String>) {
        self.default_response = response.into();
    }

    /// Extract action name from prompt (simple heuristic)
    fn extract_action(&self, prompt: &str) -> Option<String> {
        // Try to match known action keywords in the prompt
        for action in self.responses.keys() {
            if prompt.contains(action) {
                return Some(action.clone());
            }
        }

        // Check for Japanese action markers
        if prompt.contains("丁寧") {
            return Some("polite".to_string());
        }
        if prompt.contains("整理") {
            return Some("organize".to_string());
        }
        if prompt.contains("要約") {
            return Some("summarize".to_string());
        }

        None
    }
}

impl Default for MockLlmClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LlmClient for MockLlmClient {
    async fn complete(&self, prompt: &str) -> Result<String> {
        // Simulate slight delay
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Try to find a matching response
        if let Some(action) = self.extract_action(prompt) {
            if let Some(response) = self.responses.get(&action) {
                return Ok(response.clone());
            }
        }

        // Return default response
        Ok(self.default_response.clone())
    }

    fn provider_name(&self) -> &str {
        "mock"
    }

    fn model_name(&self) -> &str {
        "mock-model-v1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_client_default_response() {
        let client = MockLlmClient::new();
        let result = client.complete("some random prompt").await.unwrap();
        assert!(result.contains("Mock LLM Response"));
    }

    #[tokio::test]
    async fn test_mock_client_polite_action() {
        let client = MockLlmClient::new();
        let result = client
            .complete("丁寧な表現に変換してください")
            .await
            .unwrap();
        assert!(result.contains("お元気でしょうか"));
    }

    #[tokio::test]
    async fn test_mock_client_custom_response() {
        let mut client = MockLlmClient::new();
        client.add_response("custom", "Custom response");

        let result = client.complete("custom action").await.unwrap();
        assert_eq!(result, "Custom response");
    }

    #[test]
    fn test_provider_info() {
        let client = MockLlmClient::new();
        assert_eq!(client.provider_name(), "mock");
        assert_eq!(client.model_name(), "mock-model-v1");
    }
}
