//! CLI argument definitions

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rephraser")]
#[command(author = "Your Name")]
#[command(version = "0.1.0")]
#[command(about = "macOS text transformation tool with LLM integration", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Transform text using an action
    Rephrase {
        /// Action name (e.g., "polite", "organize", "summarize")
        #[arg(value_name = "ACTION")]
        action: String,

        /// Text to transform
        #[arg(value_name = "TEXT")]
        text: String,
    },

    /// Configuration management
    Config {
        #[command(subcommand)]
        subcommand: ConfigCommands,
    },

    /// List available actions
    ListActions,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Initialize configuration file with defaults
    Init,

    /// Show current configuration
    Show,

    /// Set a configuration value
    Set {
        /// Configuration key (e.g., "llm.model", "output.method")
        #[arg(value_name = "KEY")]
        key: String,

        /// Configuration value
        #[arg(value_name = "VALUE")]
        value: String,
    },

    /// Show configuration file path
    Path,
}
