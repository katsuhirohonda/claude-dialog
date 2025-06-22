//! Dialog loop and conversation management module
//!
//! This module implements the main conversation loop, handling user input,
//! command execution, and interaction flow with Claude.
//!
//! # Examples
//!
//! ```no_run
//! use claude_dialog::dialog::{DialogConfig, DialogLoop};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let config = DialogConfig {
//!     system_prompt: Some("You are a helpful assistant.".to_string()),
//!     append_prompt: None,
//!     model: Some("claude-3-opus".to_string()),
//! };
//!
//! let dialog = DialogLoop::new(config);
//! dialog.run().await?;
//! # Ok(())
//! # }
//! ```

use anyhow::Result;
use std::io::{self, BufRead, BufReader, Read, Write};
use crate::claude_executor::{ClaudeCommand, execute_claude};
use crate::ui::UI;

/// Configuration for the dialog loop
///
/// Contains all settings that affect how the conversation with Claude
/// is configured, including system prompts and model selection.
///
/// # Examples
///
/// ```
/// use claude_dialog::dialog::DialogConfig;
///
/// // Basic configuration with no customization
/// let config = DialogConfig {
///     system_prompt: None,
///     append_prompt: None,
///     model: None,
/// };
///
/// // Configuration with custom system prompt
/// let config = DialogConfig {
///     system_prompt: Some("You are an expert programmer.".to_string()),
///     append_prompt: None,
///     model: Some("claude-3-opus".to_string()),
/// };
///
/// // Configuration with append prompt
/// let config = DialogConfig {
///     system_prompt: None,
///     append_prompt: Some("Always provide code examples.".to_string()),
///     model: None,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct DialogConfig {
    /// Optional system prompt to replace the default
    pub system_prompt: Option<String>,
    
    /// Optional prompt to append to the default system prompt
    pub append_prompt: Option<String>,
    
    /// Optional model specification
    pub model: Option<String>,
}

/// Main dialog loop for interactive conversations with Claude
///
/// This struct manages the conversation flow, handling user input,
/// executing Claude commands, and managing the session lifecycle.
///
/// # Examples
///
/// ```
/// use claude_dialog::dialog::{DialogConfig, DialogLoop};
///
/// let config = DialogConfig {
///     system_prompt: None,
///     append_prompt: None,
///     model: None,
/// };
///
/// let dialog = DialogLoop::new(config);
/// // dialog.run().await?; // Start the conversation loop
/// ```
pub struct DialogLoop {
    config: DialogConfig,
}

impl DialogLoop {
    /// Create a new dialog loop with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the dialog session
    ///
    /// # Returns
    ///
    /// A new `DialogLoop` instance
    ///
    /// # Examples
    ///
    /// ```
    /// use claude_dialog::dialog::{DialogConfig, DialogLoop};
    ///
    /// let config = DialogConfig {
    ///     system_prompt: Some("Be concise.".to_string()),
    ///     append_prompt: None,
    ///     model: None,
    /// };
    ///
    /// let dialog = DialogLoop::new(config);
    /// ```
    pub fn new(config: DialogConfig) -> Self {
        Self { config }
    }
    
    /// Check if the given input is an exit command
    ///
    /// Recognizes "exit" and "quit" commands (case-insensitive).
    ///
    /// # Arguments
    ///
    /// * `input` - The user input to check
    ///
    /// # Returns
    ///
    /// `true` if the input is an exit command, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use claude_dialog::dialog::{DialogConfig, DialogLoop};
    ///
    /// let dialog = DialogLoop::new(DialogConfig {
    ///     system_prompt: None,
    ///     append_prompt: None,
    ///     model: None,
    /// });
    ///
    /// assert!(dialog.is_exit_command("exit"));
    /// assert!(dialog.is_exit_command("QUIT"));
    /// assert!(dialog.is_exit_command("Exit"));
    /// assert!(!dialog.is_exit_command("hello"));
    /// ```
    pub fn is_exit_command(&self, input: &str) -> bool {
        let lower = input.to_lowercase();
        lower == "exit" || lower == "quit"
    }
    
    /// Read a line of input from the given reader
    ///
    /// This method is primarily used for testing purposes, allowing
    /// input to be read from any `Read` implementation.
    ///
    /// # Arguments
    ///
    /// * `reader` - The reader to read input from
    ///
    /// # Returns
    ///
    /// * `Ok(Some(String))` - Non-empty input line (trimmed)
    /// * `Ok(None)` - Empty input line
    /// * `Err(_)` - Read error
    ///
    /// # Examples
    ///
    /// ```
    /// use claude_dialog::dialog::{DialogConfig, DialogLoop};
    /// use std::io::Cursor;
    ///
    /// let dialog = DialogLoop::new(DialogConfig {
    ///     system_prompt: None,
    ///     append_prompt: None,
    ///     model: None,
    /// });
    ///
    /// let mut input = Cursor::new("Hello, Claude!\n");
    /// let result = dialog.read_input(&mut input).unwrap();
    /// assert_eq!(result, Some("Hello, Claude!".to_string()));
    ///
    /// let mut empty_input = Cursor::new("\n");
    /// let result = dialog.read_input(&mut empty_input).unwrap();
    /// assert_eq!(result, None);
    /// ```
    pub fn read_input<R: Read>(&self, reader: &mut R) -> Result<Option<String>> {
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();
        buf_reader.read_line(&mut line)?;
        
        let trimmed = line.trim();
        if trimmed.is_empty() {
            Ok(None)
        } else {
            Ok(Some(trimmed.to_string()))
        }
    }
    
    /// Run the main dialog loop
    ///
    /// This method starts an interactive conversation session with Claude,
    /// continuously reading user input and executing Claude commands until
    /// the user enters an exit command.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Loop exited normally
    /// * `Err(_)` - An error occurred during execution
    ///
    /// # Behavior
    ///
    /// 1. Displays a user prompt
    /// 2. Reads user input
    /// 3. Checks for exit commands ("exit" or "quit")
    /// 4. Executes Claude with the user's input
    /// 5. Repeats until exit
    ///
    /// Empty input lines are ignored and the loop continues.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use claude_dialog::dialog::{DialogConfig, DialogLoop};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = DialogConfig {
    ///     system_prompt: None,
    ///     append_prompt: None,
    ///     model: None,
    /// };
    ///
    /// let dialog = DialogLoop::new(config);
    /// dialog.run().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run(&self) -> Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        
        loop {
            // Show prompt
            UI::print_user_prompt();
            stdout.flush()?;
            
            // Read input
            let mut input = String::new();
            stdin.read_line(&mut input)?;
            let input = input.trim();
            
            // Check for empty input
            if input.is_empty() {
                continue;
            }
            
            // Check for exit command
            if self.is_exit_command(input) {
                UI::print_exit_message();
                break;
            }
            
            // Show Claude prompt
            UI::print_claude_prompt();
            
            // Execute Claude command
            let command = ClaudeCommand {
                prompt: input.to_string(),
                system_prompt: self.config.system_prompt.clone(),
                append_prompt: self.config.append_prompt.clone(),
                model: self.config.model.clone(),
            };
            
            execute_claude(command).await?;
            println!(); // Add newline after Claude response
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_loop_creation() {
        let config = DialogConfig {
            system_prompt: None,
            append_prompt: None,
            model: None,
        };
        let dialog = DialogLoop::new(config);
        assert!(dialog.config.system_prompt.is_none());
    }

    #[test]
    fn test_dialog_config_with_values() {
        let config = DialogConfig {
            system_prompt: Some("System".to_string()),
            append_prompt: Some("Append".to_string()),
            model: Some("claude-3-opus".to_string()),
        };
        
        assert_eq!(config.system_prompt, Some("System".to_string()));
        assert_eq!(config.append_prompt, Some("Append".to_string()));
        assert_eq!(config.model, Some("claude-3-opus".to_string()));
    }

    #[test]
    fn test_is_exit_command() {
        let dialog = DialogLoop::new(DialogConfig {
            system_prompt: None,
            append_prompt: None,
            model: None,
        });
        
        // Test various exit commands
        assert!(dialog.is_exit_command("exit"));
        assert!(dialog.is_exit_command("EXIT"));
        assert!(dialog.is_exit_command("Exit"));
        assert!(dialog.is_exit_command("quit"));
        assert!(dialog.is_exit_command("QUIT"));
        assert!(dialog.is_exit_command("Quit"));
        
        // Test non-exit commands
        assert!(!dialog.is_exit_command("hello"));
        assert!(!dialog.is_exit_command("exit "));
        assert!(!dialog.is_exit_command(" quit"));
        assert!(!dialog.is_exit_command(""));
    }

    #[test]
    fn test_read_input() {
        let dialog = DialogLoop::new(DialogConfig {
            system_prompt: None,
            append_prompt: None,
            model: None,
        });
        
        // Test normal input
        let mut cursor = std::io::Cursor::new("Hello, world!\n");
        let result = dialog.read_input(&mut cursor).unwrap();
        assert_eq!(result, Some("Hello, world!".to_string()));
        
        // Test empty input
        let mut cursor = std::io::Cursor::new("\n");
        let result = dialog.read_input(&mut cursor).unwrap();
        assert_eq!(result, None);
        
        // Test whitespace-only input
        let mut cursor = std::io::Cursor::new("   \n");
        let result = dialog.read_input(&mut cursor).unwrap();
        assert_eq!(result, None);
        
        // Test input with surrounding whitespace
        let mut cursor = std::io::Cursor::new("  test  \n");
        let result = dialog.read_input(&mut cursor).unwrap();
        assert_eq!(result, Some("test".to_string()));
    }
}