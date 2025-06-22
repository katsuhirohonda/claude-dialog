//! Terminal user interface module with colored output
//!
//! This module provides formatted console output with color support
//! for a better user experience during interactive conversations.
//!
//! # Examples
//!
//! ```
//! use claude_dialog::ui::UI;
//!
//! // Print welcome message
//! UI::print_welcome("Using default prompt", Some(&"claude-3-opus".to_string()));
//!
//! // Print prompts
//! UI::print_user_prompt();
//! UI::print_claude_prompt();
//!
//! // Print exit message
//! UI::print_exit_message();
//! ```

use colored::*;

/// User interface handler for terminal output
///
/// Provides static methods for printing formatted and colored messages
/// to the terminal during the conversation flow.
///
/// # Color Scheme
///
/// - Blue: Separators and Claude prompt
/// - Yellow: Title and exit message
/// - Green: System prompt info, model info, and user prompt
///
/// # Examples
///
/// ```
/// use claude_dialog::ui::UI;
///
/// // Display welcome screen
/// UI::print_welcome("Custom system prompt loaded", None);
///
/// // Show user prompt (typically followed by user input)
/// UI::print_user_prompt();
/// ```
pub struct UI;

impl UI {
    /// Print the welcome banner with system information
    ///
    /// Displays a formatted welcome message including the system prompt
    /// status and optionally the model being used.
    ///
    /// # Arguments
    ///
    /// * `system_prompt_info` - Description of the system prompt configuration
    /// * `model` - Optional model name to display
    ///
    /// # Output Format
    ///
    /// ```text
    /// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    /// Claude Dialog Shell
    /// System Prompt: [system_prompt_info]
    /// Model: [model] (if provided)
    /// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    /// 
    /// Type 'exit' or 'quit' to end the conversation
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use claude_dialog::ui::UI;
    ///
    /// // With model specified
    /// UI::print_welcome("Using custom prompt from file", Some(&"claude-3-opus".to_string()));
    ///
    /// // Without model
    /// UI::print_welcome("Using default system prompt", None);
    /// ```
    pub fn print_welcome(system_prompt_info: &str, model: Option<&String>) {
        println!("{}", "━".repeat(60).blue());
        println!("{}", "Claude Dialog Shell".yellow().bold());
        println!("System Prompt: {}", system_prompt_info.green());
        if let Some(model) = model {
            println!("Model: {}", model.green());
        }
        println!("{}", "━".repeat(60).blue());
        println!();
        println!("Type 'exit' or 'quit' to end the conversation");
        println!();
    }
    
    /// Print the user input prompt
    ///
    /// Displays a colored prompt indicator for user input.
    /// This should be called before reading user input.
    ///
    /// # Output
    ///
    /// Prints "You> " in green color without a newline.
    ///
    /// # Examples
    ///
    /// ```
    /// use claude_dialog::ui::UI;
    /// use std::io::{self, Write};
    ///
    /// UI::print_user_prompt();
    /// io::stdout().flush().unwrap(); // Ensure prompt is displayed
    /// // User types their input here
    /// ```
    pub fn print_user_prompt() {
        print!("{} ", "You>".green());
    }
    
    /// Print the Claude response prompt
    ///
    /// Displays a colored prompt indicator before Claude's response.
    /// This should be called before executing the Claude command.
    ///
    /// # Output
    ///
    /// Prints "Claude>" in blue color with a newline.
    ///
    /// # Examples
    ///
    /// ```
    /// use claude_dialog::ui::UI;
    ///
    /// UI::print_claude_prompt();
    /// // Claude's response will follow
    /// ```
    pub fn print_claude_prompt() {
        println!("{}", "Claude>".blue());
    }
    
    /// Print the exit message
    ///
    /// Displays a farewell message when the user exits the conversation.
    ///
    /// # Output
    ///
    /// Prints "Exiting conversation..." in yellow color.
    ///
    /// # Examples
    ///
    /// ```
    /// use claude_dialog::ui::UI;
    ///
    /// // When user types "exit" or "quit"
    /// UI::print_exit_message();
    /// ```
    pub fn print_exit_message() {
        println!("{}", "Exiting conversation...".yellow());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_creation() {
        // This is mainly to ensure UI struct can be created
        let _ui = UI;
    }

    #[test]
    fn test_colored_output() {
        // Test that colored strings are created without panic
        let _user_prompt = "You>".green();
        let _claude_prompt = "Claude>".blue();
        let _title = "Claude Dialog Shell".yellow().bold();
        let _separator = "━".repeat(60).blue();
        let _exit_msg = "Exiting conversation...".yellow();
    }
}