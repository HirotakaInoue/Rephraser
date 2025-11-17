//! Action resolution

use crate::actions::template::TemplateEngine;
use crate::config::{ActionConfig, Config};
use crate::error::{RephraserError, Result};

/// Action resolver
///
/// Resolves action names to prompt templates and performs variable substitution
pub struct ActionResolver {
    actions: Vec<ActionConfig>,
}

impl ActionResolver {
    /// Create a new action resolver from config
    pub fn new(config: &Config) -> Self {
        Self {
            actions: config.actions.clone(),
        }
    }

    /// Get all available actions
    pub fn list_actions(&self) -> &[ActionConfig] {
        &self.actions
    }

    /// Find an action by name
    pub fn find_action(&self, name: &str) -> Option<&ActionConfig> {
        self.actions.iter().find(|a| a.name == name)
    }

    /// Resolve an action and render its prompt with the given text
    ///
    /// # Arguments
    /// * `action_name` - Name of the action to resolve
    /// * `text` - Text to process
    ///
    /// # Returns
    /// * `Result<String>` - Rendered prompt ready to send to LLM
    ///
    /// # Errors
    /// * If the action is not found
    /// * If template rendering fails
    pub fn resolve(&self, action_name: &str, text: &str) -> Result<String> {
        let action = self
            .find_action(action_name)
            .ok_or_else(|| RephraserError::ActionNotFound(action_name.to_string()))?;

        let mut engine = TemplateEngine::new();
        engine.set("text", text);

        engine.render(&action.prompt_template)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_resolve_action() {
        let config = Config::default();
        let resolver = ActionResolver::new(&config);

        let prompt = resolver.resolve("polite", "Hello").unwrap();
        assert!(prompt.contains("Hello"));
        assert!(prompt.contains("丁寧な表現"));
    }

    #[test]
    fn test_action_not_found() {
        let config = Config::default();
        let resolver = ActionResolver::new(&config);

        let result = resolver.resolve("nonexistent", "test");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_actions() {
        let config = Config::default();
        let resolver = ActionResolver::new(&config);

        let actions = resolver.list_actions();
        assert_eq!(actions.len(), 3);
        assert!(actions.iter().any(|a| a.name == "polite"));
        assert!(actions.iter().any(|a| a.name == "organize"));
        assert!(actions.iter().any(|a| a.name == "summarize"));
    }
}
