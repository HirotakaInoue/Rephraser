//! Prompt template engine

use crate::error::{RephraserError, Result};
use std::collections::HashMap;

/// Simple template engine for prompt templates
///
/// Supports variable substitution like {text}, {language}, etc.
pub struct TemplateEngine {
    variables: HashMap<String, String>,
}

impl TemplateEngine {
    /// Create a new template engine
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Set a variable value
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.variables.insert(key.into(), value.into());
        self
    }

    /// Render a template with the current variables
    ///
    /// # Arguments
    /// * `template` - Template string with variables like {text}
    ///
    /// # Returns
    /// * `Result<String>` - Rendered template
    ///
    /// # Errors
    /// * If a variable in the template is not set
    pub fn render(&self, template: &str) -> Result<String> {
        let mut result = template.to_string();

        // Find all variables in the template
        let mut missing_vars = Vec::new();

        for (key, value) in &self.variables {
            let placeholder = format!("{{{}}}", key);
            if result.contains(&placeholder) {
                result = result.replace(&placeholder, value);
            }
        }

        // Check for unresolved variables
        let mut chars = result.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '{' {
                let mut var_name = String::new();
                while let Some(&next_char) = chars.peek() {
                    if next_char == '}' {
                        chars.next();
                        if !var_name.is_empty() && !self.variables.contains_key(&var_name) {
                            missing_vars.push(var_name.clone());
                        }
                        break;
                    }
                    var_name.push(chars.next().unwrap());
                }
            }
        }

        if !missing_vars.is_empty() {
            return Err(RephraserError::InvalidTemplate(format!(
                "Missing variables: {}",
                missing_vars.join(", ")
            )));
        }

        Ok(result)
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_substitution() {
        let mut engine = TemplateEngine::new();
        engine.set("text", "Hello");

        let result = engine.render("Process this: {text}").unwrap();
        assert_eq!(result, "Process this: Hello");
    }

    #[test]
    fn test_multiple_variables() {
        let mut engine = TemplateEngine::new();
        engine.set("text", "Hello").set("language", "English");

        let result = engine.render("Translate '{text}' to {language}").unwrap();
        assert_eq!(result, "Translate 'Hello' to English");
    }

    #[test]
    fn test_missing_variable() {
        let engine = TemplateEngine::new();
        let result = engine.render("Process this: {text}");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_variables() {
        let engine = TemplateEngine::new();
        let result = engine.render("No variables here").unwrap();
        assert_eq!(result, "No variables here");
    }
}
