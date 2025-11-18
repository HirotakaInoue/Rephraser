//! Output formatting and display

use crate::config::OutputMethod;
use crate::error::Result;
use std::process::Command;

/// Maximum length for notification text
const MAX_NOTIFICATION_LENGTH: usize = 200;

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
    ///
    /// # Errors
    /// Returns an error if:
    /// - The platform is not macOS
    /// - pbcopy command is not available
    /// - The command execution fails
    fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        use crate::error::RephraserError;
        check_macos_platform()?;

        let mut child = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| RephraserError::Output(
                format!("Failed to spawn pbcopy: {}", e)
            ))?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(text.as_bytes())
                .map_err(|e| RephraserError::Output(
                    format!("Failed to write to pbcopy stdin: {}", e)
                ))?;
        }

        let status = child.wait()
            .map_err(|e| RephraserError::Output(
                format!("Failed to wait for pbcopy: {}", e)
            ))?;

        if !status.success() {
            return Err(RephraserError::Output(
                format!("pbcopy exited with status: {}", status)
            ));
        }

        Ok(())
    }

    /// Show macOS notification
    ///
    /// Displays a system notification with title "Rephraser".
    /// Text longer than 200 characters will be truncated with ellipsis.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The platform is not macOS
    /// - osascript command is not available
    /// - The AppleScript execution fails
    fn show_notification(&self, text: &str) -> Result<()> {
        use crate::error::RephraserError;
        check_macos_platform()?;

        // Truncate and escape the text
        let truncated = truncate_notification_text(text, MAX_NOTIFICATION_LENGTH);
        // Remove newlines (AppleScript notifications don't support them)
        let single_line = truncated.replace('\n', " ").replace('\r', " ");
        let escaped = escape_applescript_string(&single_line);

        // Build AppleScript command
        let script = format!(
            r#"display notification "{}" with title "Rephraser""#,
            escaped
        );

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| RephraserError::Output(
                format!("Failed to execute osascript: {}", e)
            ))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(RephraserError::Output(
                format!("osascript failed: {}", stderr)
            ));
        }

        Ok(())
    }

    /// Show macOS dialog
    ///
    /// Displays a blocking dialog box with the text and an OK button.
    /// Long text will be scrollable within the dialog.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The platform is not macOS
    /// - osascript command is not available
    /// - The AppleScript execution fails
    fn show_dialog(&self, text: &str) -> Result<()> {
        use crate::error::RephraserError;
        check_macos_platform()?;

        // Escape the text for AppleScript
        let escaped = escape_applescript_string(text);

        // Build AppleScript command with scrollable text
        // Note: For long text, AppleScript automatically makes dialogs scrollable
        let script = format!(
            r#"display dialog "{}" with title "Rephraser" buttons {{"OK"}} default button "OK""#,
            escaped
        );

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| RephraserError::Output(
                format!("Failed to execute osascript: {}", e)
            ))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(RephraserError::Output(
                format!("osascript dialog failed: {}", stderr)
            ));
        }

        Ok(())
    }
}

/// Escape a string for safe use in AppleScript
///
/// AppleScript string literals require:
/// - Backslashes escaped as \\
/// - Double quotes escaped as \"
fn escape_applescript_string(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('"', "\\\"")
}

/// Truncate text for notification display
///
/// If text exceeds max length, truncate and add ellipsis
fn truncate_notification_text(text: &str, max: usize) -> String {
    if text.len() <= max {
        text.to_string()
    } else {
        // Find a safe truncation point (avoid cutting multi-byte chars)
        let mut truncate_at = max.saturating_sub(3); // Reserve space for "..."
        while truncate_at > 0 && !text.is_char_boundary(truncate_at) {
            truncate_at -= 1;
        }
        format!("{}...", &text[..truncate_at])
    }
}

/// Check if the current platform is macOS
///
/// Returns an error if not on macOS
fn check_macos_platform() -> Result<()> {
    #[cfg(not(target_os = "macos"))]
    {
        use crate::error::RephraserError;
        return Err(RephraserError::Output(
            "Output methods are only supported on macOS".to_string()
        ));
    }
    #[cfg(target_os = "macos")]
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn test_clipboard_handler() {
        let handler = OutputHandler::new(OutputMethod::Clipboard);
        let result = handler.handle("test clipboard content");
        assert!(result.is_ok());

        // Verify by reading back from clipboard
        let output = std::process::Command::new("pbpaste")
            .output()
            .expect("Failed to run pbpaste");
        let clipboard_content = String::from_utf8_lossy(&output.stdout);
        assert_eq!(clipboard_content, "test clipboard content");
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore] // This displays actual notifications - run manually
    fn test_notification_handler() {
        let handler = OutputHandler::new(OutputMethod::Notification);

        // Test with simple text
        let result = handler.handle("test notification");
        assert!(result.is_ok());

        // Test with special characters
        let result = handler.handle("test with \"quotes\" and \\backslash");
        assert!(result.is_ok());

        // Test with very long text (should truncate)
        let long_text = "a".repeat(500);
        let result = handler.handle(&long_text);
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(target_os = "macos")]
    #[ignore] // Requires manual interaction (user must click OK)
    fn test_dialog_handler() {
        let handler = OutputHandler::new(OutputMethod::Dialog);

        // Test with simple text
        let result = handler.handle("test dialog");
        assert!(result.is_ok());

        // Test with special characters
        let result = handler.handle("test with \"quotes\" and \\backslash\nand newlines");
        assert!(result.is_ok());
    }

    #[test]
    fn test_escape_applescript_string() {
        assert_eq!(
            escape_applescript_string("simple text"),
            "simple text"
        );
        assert_eq!(
            escape_applescript_string("text with \"quotes\""),
            "text with \\\"quotes\\\""
        );
        assert_eq!(
            escape_applescript_string("path\\to\\file"),
            "path\\\\to\\\\file"
        );
        assert_eq!(
            escape_applescript_string("mixed: \"path\\file\""),
            "mixed: \\\"path\\\\file\\\""
        );
    }

    #[test]
    fn test_truncate_notification_text() {
        assert_eq!(
            truncate_notification_text("short", 100),
            "short"
        );

        let long_text = "a".repeat(250);
        let truncated = truncate_notification_text(&long_text, 200);
        assert_eq!(truncated.len(), 200);
        assert!(truncated.ends_with("..."));

        // Test with multi-byte characters (Japanese)
        let japanese = "こんにちは".repeat(50); // ~150 chars (3 bytes each)
        let truncated = truncate_notification_text(&japanese, 200);
        assert!(truncated.len() <= 200);
        assert!(truncated.ends_with("..."));
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn test_platform_check_fails_on_non_macos() {
        let result = check_macos_platform();
        assert!(result.is_err());
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_platform_check_succeeds_on_macos() {
        let result = check_macos_platform();
        assert!(result.is_ok());
    }
}
