//! Claude command execution module
//!
//! This module handles building and executing Claude CLI commands with the appropriate
//! arguments for prompts, models, and tool permissions.
//!
//! # Examples
//!
//! ```no_run
//! use claude_dialog::claude_executor::{ClaudeCommand, execute_claude};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a basic Claude command
//! let command = ClaudeCommand {
//!     prompt: "Hello, Claude!".to_string(),
//!     system_prompt: None,
//!     append_prompt: None,
//!     model: Some("claude-3-opus".to_string()),
//! };
//!
//! // Execute the command
//! execute_claude(command).await?;
//! # Ok(())
//! # }
//! ```

use anyhow::{Result, Context};
use tokio::process::Command;

/// Represents a Claude command with all necessary parameters
///
/// This structure encapsulates all the information needed to construct
/// and execute a Claude CLI command, including prompts, model selection,
/// and system prompt configuration.
///
/// # Examples
///
/// ```
/// use claude_dialog::claude_executor::ClaudeCommand;
///
/// // Basic command with just a prompt
/// let cmd = ClaudeCommand {
///     prompt: "What is Rust?".to_string(),
///     system_prompt: None,
///     append_prompt: None,
///     model: None,
/// };
///
/// // Command with custom system prompt and model
/// let cmd = ClaudeCommand {
///     prompt: "Explain memory safety".to_string(),
///     system_prompt: Some("You are a Rust expert.".to_string()),
///     append_prompt: None,
///     model: Some("claude-3-opus".to_string()),
/// };
///
/// // Command with append prompt
/// let cmd = ClaudeCommand {
///     prompt: "Write a function".to_string(),
///     system_prompt: None,
///     append_prompt: Some("Always use idiomatic Rust.".to_string()),
///     model: None,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ClaudeCommand {
    /// The main prompt to send to Claude
    pub prompt: String,
    
    /// Optional system prompt that replaces the default
    pub system_prompt: Option<String>,
    
    /// Optional prompt to append to the default system prompt
    pub append_prompt: Option<String>,
    
    /// Optional model specification (e.g., "claude-3-opus")
    pub model: Option<String>,
}

impl ClaudeCommand {
    /// Build command-line arguments for the Claude CLI
    ///
    /// Constructs a vector of arguments based on the command configuration,
    /// including the prompt, system prompts, model selection, and allowed tools.
    ///
    /// # Returns
    ///
    /// A vector of strings representing the command-line arguments
    ///
    /// # Examples
    ///
    /// ```
    /// use claude_dialog::claude_executor::ClaudeCommand;
    ///
    /// let cmd = ClaudeCommand {
    ///     prompt: "Hello".to_string(),
    ///     system_prompt: Some("Be helpful".to_string()),
    ///     append_prompt: None,
    ///     model: Some("claude-3-opus".to_string()),
    /// };
    ///
    /// let args = cmd.build_args();
    /// assert!(args.contains(&"--continue".to_string()));
    /// assert!(args.contains(&"-p".to_string()));
    /// assert!(args.contains(&"Hello".to_string()));
    /// assert!(args.contains(&"--system-prompt".to_string()));
    /// assert!(args.contains(&"Be helpful".to_string()));
    /// assert!(args.contains(&"--model".to_string()));
    /// assert!(args.contains(&"claude-3-opus".to_string()));
    /// ```
    pub fn build_args(&self) -> Vec<String> {
        let mut args = vec!["--continue".to_string(), "-p".to_string(), self.prompt.clone()];
        
        if let Some(system_prompt) = &self.system_prompt {
            args.push("--system-prompt".to_string());
            args.push(system_prompt.clone());
        }
        
        if let Some(append_prompt) = &self.append_prompt {
            args.push("--append-system-prompt".to_string());
            args.push(append_prompt.clone());
        }
        
        if let Some(model) = &self.model {
            args.push("--model".to_string());
            args.push(model.clone());
        }
        
        // Add allowed tools
        args.push("--allowedTools".to_string());
        args.push("Write".to_string());
        args.push("Edit".to_string());
        
        args
    }
}

/// Execute a Claude command asynchronously
///
/// This function builds the command arguments and executes the Claude CLI
/// with the specified configuration. It waits for the command to complete
/// and returns an error if the command fails.
///
/// # Arguments
///
/// * `command` - The Claude command configuration to execute
///
/// # Returns
///
/// * `Result<()>` - Success or an error if the command fails
///
/// # Errors
///
/// Returns an error if:
/// - The Claude CLI is not found or cannot be executed
/// - The Claude command returns a non-zero exit status
///
/// # Examples
///
/// ```no_run
/// use claude_dialog::claude_executor::{ClaudeCommand, execute_claude};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let command = ClaudeCommand {
///     prompt: "What is 2 + 2?".to_string(),
///     system_prompt: None,
///     append_prompt: None,
///     model: None,
/// };
///
/// execute_claude(command).await?;
/// # Ok(())
/// # }
/// ```
pub async fn execute_claude(command: ClaudeCommand) -> Result<()> {
    let args = command.build_args();
    
    let mut cmd = Command::new("claude");
    cmd.args(&args);
    
    let status = cmd.status()
        .await
        .context("Failed to execute claude command")?;
    
    if !status.success() {
        anyhow::bail!("Claude command failed with status: {}", status);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_command_creation() {
        let cmd = ClaudeCommand {
            prompt: "test".to_string(),
            system_prompt: None,
            append_prompt: None,
            model: None,
        };
        assert_eq!(cmd.prompt, "test");
    }

    #[test]
    fn test_build_args_basic() {
        let cmd = ClaudeCommand {
            prompt: "Hello, Claude!".to_string(),
            system_prompt: None,
            append_prompt: None,
            model: None,
        };
        
        let args = cmd.build_args();
        assert_eq!(args[0], "--continue");
        assert_eq!(args[1], "-p");
        assert_eq!(args[2], "Hello, Claude!");
        assert!(args.contains(&"--allowedTools".to_string()));
        assert!(args.contains(&"Write".to_string()));
        assert!(args.contains(&"Edit".to_string()));
    }

    #[test]
    fn test_build_args_with_system_prompt() {
        let cmd = ClaudeCommand {
            prompt: "Test".to_string(),
            system_prompt: Some("Custom system prompt".to_string()),
            append_prompt: None,
            model: None,
        };
        
        let args = cmd.build_args();
        assert!(args.contains(&"--system-prompt".to_string()));
        assert!(args.contains(&"Custom system prompt".to_string()));
    }

    #[test]
    fn test_build_args_with_append_prompt() {
        let cmd = ClaudeCommand {
            prompt: "Test".to_string(),
            system_prompt: None,
            append_prompt: Some("Additional instructions".to_string()),
            model: None,
        };
        
        let args = cmd.build_args();
        assert!(args.contains(&"--append-system-prompt".to_string()));
        assert!(args.contains(&"Additional instructions".to_string()));
    }

    #[test]
    fn test_build_args_with_model() {
        let cmd = ClaudeCommand {
            prompt: "Test".to_string(),
            system_prompt: None,
            append_prompt: None,
            model: Some("claude-3-opus".to_string()),
        };
        
        let args = cmd.build_args();
        assert!(args.contains(&"--model".to_string()));
        assert!(args.contains(&"claude-3-opus".to_string()));
    }

    #[test]
    fn test_build_args_full() {
        let cmd = ClaudeCommand {
            prompt: "Complex test".to_string(),
            system_prompt: Some("System".to_string()),
            append_prompt: Some("Append".to_string()),
            model: Some("claude-3-sonnet".to_string()),
        };
        
        let args = cmd.build_args();
        assert!(args.contains(&"Complex test".to_string()));
        assert!(args.contains(&"--system-prompt".to_string()));
        assert!(args.contains(&"System".to_string()));
        assert!(args.contains(&"--append-system-prompt".to_string()));
        assert!(args.contains(&"Append".to_string()));
        assert!(args.contains(&"--model".to_string()));
        assert!(args.contains(&"claude-3-sonnet".to_string()));
    }
}