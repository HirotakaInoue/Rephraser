//! Output formatting and display

use crate::config::OutputMethod;
use crate::error::Result;
use std::process::Command;

/// Output handler
///
/// Handles different output methods: clipboard, notification, dialog
pub struct OutputHandler {
    method: OutputMethod,
}

impl OutputHandler {
    /// Create a new output handler
    pub fn new(method: OutputMethod) -> Self {
        Self { method }
    }

    /// Handle output based on the configured method
    ///
    /// # Arguments
    /// * `text` - The text to output
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn handle(&self, text: &str) -> Result<()> {
        match self.method {
            OutputMethod::Clipboard => self.copy_to_clipboard(text),
            OutputMethod::Notification => self.show_notification(text),
            OutputMethod::Dialog => self.show_dialog(text),
        }
    }

    /// Copy text to clipboard using pbcopy
    fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        // TODO: Implement actual clipboard copy using pbcopy
        println!("[MOCK] Copied to clipboard:");
        println!("{}", text);
        Ok(())
    }

    /// Show macOS notification
    fn show_notification(&self, text: &str) -> Result<()> {
        // TODO: Implement actual notification using osascript
        println!("[MOCK] Notification:");
        println!("{}", text);
        Ok(())
    }

    /// Show macOS dialog
    fn show_dialog(&self, text: &str) -> Result<()> {
        // TODO: Implement actual dialog using osascript
        println!("[MOCK] Dialog:");
        println!("{}", text);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_handler() {
        let handler = OutputHandler::new(OutputMethod::Clipboard);
        let result = handler.handle("test text");
        assert!(result.is_ok());
    }

    #[test]
    fn test_notification_handler() {
        let handler = OutputHandler::new(OutputMethod::Notification);
        let result = handler.handle("test text");
        assert!(result.is_ok());
    }

    #[test]
    fn test_dialog_handler() {
        let handler = OutputHandler::new(OutputMethod::Dialog);
        let result = handler.handle("test text");
        assert!(result.is_ok());
    }
}
