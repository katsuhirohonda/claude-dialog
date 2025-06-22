//! System prompt configuration and loading module
//!
//! This module handles loading and managing system prompts from files.
//! It supports both complete prompt replacement and appending to default prompts.
//!
//! # Examples
//!
//! ```no_run
//! use claude_dialog::prompt::{SystemPromptConfig, load_system_prompt};
//!
//! // Load a single system prompt file
//! let config = SystemPromptConfig {
//!     system_prompt_files: vec!["prompt.md".to_string()],
//!     append_prompt_file: None,
//! };
//! let prompt = load_system_prompt(config).unwrap();
//!
//! // Load multiple system prompt files
//! let config = SystemPromptConfig {
//!     system_prompt_files: vec!["base.md".to_string(), "specific.md".to_string()],
//!     append_prompt_file: None,
//! };
//! let prompt = load_system_prompt(config).unwrap();
//! ```

use anyhow::{Result, Context};
use std::fs;

/// Configuration for system prompt loading
///
/// This structure determines how system prompts are loaded and combined.
/// It supports two modes:
/// 1. Complete replacement with one or more prompt files
/// 2. Appending additional content to the default prompt
///
/// # Examples
///
/// ```
/// use claude_dialog::prompt::SystemPromptConfig;
///
/// // Configuration for multiple system prompts
/// let config = SystemPromptConfig {
///     system_prompt_files: vec!["base.md".to_string(), "custom.md".to_string()],
///     append_prompt_file: None,
/// };
///
/// // Configuration for appending to default prompt
/// let config = SystemPromptConfig {
///     system_prompt_files: vec![],
///     append_prompt_file: Some("additions.md".to_string()),
/// };
/// ```
#[derive(Debug)]
pub struct SystemPromptConfig {
    /// List of system prompt files to load (replaces default prompt)
    ///
    /// When provided, these files completely replace the default system prompt.
    /// Multiple files are concatenated with double newlines between them.
    pub system_prompt_files: Vec<String>,
    
    /// Optional file to append to the default prompt
    ///
    /// When provided (and system_prompt_files is empty), this file's contents
    /// are appended to the default system prompt rather than replacing it.
    pub append_prompt_file: Option<String>,
}

/// Load system prompt based on the provided configuration
///
/// This function handles three scenarios:
/// 1. Multiple system prompt files - loads and concatenates them
/// 2. Single append file - loads it for appending to default prompt
/// 3. No files specified - returns empty string
///
/// # Arguments
///
/// * `config` - Configuration specifying which prompt files to load
///
/// # Returns
///
/// * `Result<String>` - The loaded prompt content or an error
///
/// # Errors
///
/// Returns an error if any specified file cannot be read.
///
/// # Examples
///
/// ```no_run
/// use claude_dialog::prompt::{SystemPromptConfig, load_system_prompt};
/// use std::fs;
///
/// // Create test files
/// fs::write("test1.md", "First prompt").unwrap();
/// fs::write("test2.md", "Second prompt").unwrap();
///
/// // Load multiple prompts
/// let config = SystemPromptConfig {
///     system_prompt_files: vec!["test1.md".to_string(), "test2.md".to_string()],
///     append_prompt_file: None,
/// };
/// let result = load_system_prompt(config).unwrap();
/// assert_eq!(result, "First prompt\n\nSecond prompt");
///
/// // Clean up
/// fs::remove_file("test1.md").unwrap();
/// fs::remove_file("test2.md").unwrap();
/// ```
pub fn load_system_prompt(config: SystemPromptConfig) -> Result<String> {
    if !config.system_prompt_files.is_empty() {
        // Load and concatenate multiple system prompt files
        let mut prompts = Vec::new();
        
        for file_path in &config.system_prompt_files {
            let content = fs::read_to_string(file_path)
                .with_context(|| format!("Failed to read system prompt file: {}", file_path))?;
            prompts.push(content);
        }
        
        // Join with double newlines between files
        Ok(prompts.join("\n\n"))
    } else if let Some(append_file) = config.append_prompt_file {
        // Load append prompt file
        let content = fs::read_to_string(&append_file)
            .with_context(|| format!("Failed to read append prompt file: {}", append_file))?;
        Ok(content)
    } else {
        // No custom prompt
        Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_prompt_config() {
        let config = SystemPromptConfig {
            system_prompt_files: vec!["test.md".to_string()],
            append_prompt_file: None,
        };
        assert_eq!(config.system_prompt_files.len(), 1);
        assert!(config.append_prompt_file.is_none());
    }
}