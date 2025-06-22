//! Command-line argument parsing for the Claude Dialog CLI
//!
//! This module handles parsing and validation of command-line arguments using the `clap` crate.
//! It supports specifying custom system prompts, model selection, and prompt modification options.
//!
//! # Examples
//!
//! ```
//! use claude_dialog::cli::{Args, parse_args};
//!
//! // Parse arguments with a custom system prompt
//! let args = parse_args(vec!["claude-dialog", "--system-prompt", "custom.md"]).unwrap();
//! assert_eq!(args.system_prompt_files, vec!["custom.md"]);
//!
//! // Parse arguments with a model specification
//! let args = parse_args(vec!["claude-dialog", "--model", "claude-3-opus"]).unwrap();
//! assert_eq!(args.model, Some("claude-3-opus".to_string()));
//! ```

use clap::Parser;
use anyhow::{Result, anyhow};

/// Command-line arguments for the Claude Dialog application
///
/// This structure defines all available command-line options for configuring
/// the Claude conversation interface.
///
/// # Examples
///
/// ```
/// use claude_dialog::cli::Args;
///
/// let args = Args {
///     system_prompt_files: vec!["prompt1.md".to_string(), "prompt2.md".to_string()],
///     append_prompt_file: None,
///     model: Some("claude-3-opus".to_string()),
/// };
///
/// assert_eq!(args.system_prompt_files.len(), 2);
/// assert!(args.append_prompt_file.is_none());
/// ```
#[derive(Parser, Debug)]
#[command(
    name = "claude-dialog",
    about = "Interactive CLI for Claude conversations with custom system prompts",
    long_about = None,
    version
)]
pub struct Args {
    /// Custom system prompt files (can be specified multiple times)
    ///
    /// These files will completely replace the default system prompt.
    /// Multiple files can be specified and will be concatenated in order.
    ///
    /// # Example
    ///
    /// ```bash
    /// claude-dialog --system-prompt prompt1.md --system-prompt prompt2.md
    /// ```
    #[arg(long = "system-prompt", value_name = "FILE", action = clap::ArgAction::Append)]
    pub system_prompt_files: Vec<String>,

    /// Append to default system prompt
    ///
    /// This file's contents will be appended to the default system prompt
    /// rather than replacing it. Cannot be used together with `--system-prompt`.
    ///
    /// # Example
    ///
    /// ```bash
    /// claude-dialog --append-system-prompt additional_instructions.md
    /// ```
    #[arg(long = "append-system-prompt", value_name = "FILE", conflicts_with = "system_prompt_files")]
    pub append_prompt_file: Option<String>,

    /// Claude model to use
    ///
    /// Specifies which Claude model to use for the conversation.
    /// Common options include: claude-3-opus, claude-3-sonnet, claude-3-haiku
    ///
    /// # Example
    ///
    /// ```bash
    /// claude-dialog --model claude-3-opus
    /// ```
    #[arg(long = "model", value_name = "MODEL")]
    pub model: Option<String>,
}

/// Parse command-line arguments from a vector of strings
///
/// This function wraps the clap parser and converts parsing errors
/// into anyhow errors for consistent error handling.
///
/// # Arguments
///
/// * `args` - A vector of string slices representing command-line arguments
///
/// # Returns
///
/// * `Result<Args>` - Parsed arguments or an error if parsing fails
///
/// # Examples
///
/// ```
/// use claude_dialog::cli::parse_args;
///
/// // Basic usage
/// let args = parse_args(vec!["claude-dialog"]).unwrap();
/// assert!(args.system_prompt_files.is_empty());
///
/// // With system prompt
/// let args = parse_args(vec!["claude-dialog", "--system-prompt", "custom.md"]).unwrap();
/// assert_eq!(args.system_prompt_files, vec!["custom.md"]);
///
/// // With multiple options
/// let args = parse_args(vec![
///     "claude-dialog",
///     "--system-prompt", "p1.md",
///     "--system-prompt", "p2.md",
///     "--model", "claude-3-opus"
/// ]).unwrap();
/// assert_eq!(args.system_prompt_files.len(), 2);
/// assert_eq!(args.model, Some("claude-3-opus".to_string()));
/// ```
pub fn parse_args(args: Vec<&str>) -> Result<Args> {
    let args = Args::try_parse_from(args)
        .map_err(|e| anyhow!(e.to_string()))?;
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_struct() {
        let args = Args {
            system_prompt_files: vec!["test.md".to_string()],
            append_prompt_file: None,
            model: Some("claude-3".to_string()),
        };
        assert_eq!(args.system_prompt_files.len(), 1);
        assert!(args.append_prompt_file.is_none());
        assert_eq!(args.model, Some("claude-3".to_string()));
    }
}