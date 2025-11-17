//! Rephraser - macOS text transformation tool with LLM integration
//!
//! This library provides the core functionality for transforming text using
//! Large Language Models (LLMs) through customizable actions.

pub mod actions;
pub mod cli;
pub mod config;
pub mod error;
pub mod llm;
pub mod output;

pub use error::{RephraserError, Result};
