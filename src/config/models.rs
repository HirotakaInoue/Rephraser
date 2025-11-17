//! Configuration data structures

use serde::{Deserialize, Serialize};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub llm: LlmConfig,
    pub output: OutputConfig,
    pub actions: Vec<ActionConfig>,
}

/// LLM provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Provider name: "openai", "anthropic"
    pub provider: String,

    /// Model name (e.g., "gpt-4o-mini", "claude-3-sonnet-20240229")
    pub model: String,

    /// Environment variable name containing the API key
    pub api_key_env: String,

    /// LLM parameters
    #[serde(default)]
    pub parameters: LlmParameters,
}

/// LLM API parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmParameters {
    #[serde(default = "default_temperature")]
    pub temperature: f32,

    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
}

impl Default for LlmParameters {
    fn default() -> Self {
        Self {
            temperature: default_temperature(),
            max_tokens: default_max_tokens(),
        }
    }
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> usize {
    500
}

/// Output method configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Output method: "clipboard", "notification", "dialog"
    pub method: OutputMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputMethod {
    Clipboard,
    Notification,
    Dialog,
}

/// Action configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfig {
    /// Internal name (used in CLI)
    pub name: String,

    /// Display name (shown in UI)
    pub display_name: String,

    /// Prompt template with variables like {text}
    pub prompt_template: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            llm: LlmConfig {
                provider: "openai".to_string(),
                model: "gpt-4o-mini".to_string(),
                api_key_env: "OPENAI_API_KEY".to_string(),
                parameters: LlmParameters::default(),
            },
            output: OutputConfig {
                method: OutputMethod::Notification,
            },
            actions: default_actions(),
        }
    }
}

fn default_actions() -> Vec<ActionConfig> {
    vec![
        ActionConfig {
            name: "polite".to_string(),
            display_name: "丁寧に".to_string(),
            prompt_template: r#"以下のテキストを丁寧な表現に変換してください。元の意味を保ったまま、敬語や丁寧語を適切に使用してください。

テキスト:
{text}

丁寧な表現:"#.to_string(),
        },
        ActionConfig {
            name: "organize".to_string(),
            display_name: "整理する".to_string(),
            prompt_template: r#"以下のテキストを論理的に整理し、読みやすく構造化してください。

テキスト:
{text}

整理されたテキスト:"#.to_string(),
        },
        ActionConfig {
            name: "summarize".to_string(),
            display_name: "要約".to_string(),
            prompt_template: r#"以下のテキストを簡潔に要約してください。

テキスト:
{text}

要約:"#.to_string(),
        },
    ]
}
