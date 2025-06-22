//! Claude Dialog - An interactive CLI for conversations with Claude AI
//!
//! This library provides a command-line interface for having continuous conversations
//! with Claude AI. It handles argument parsing, command execution, dialog management,
//! and terminal UI rendering.
//!
//! # Architecture
//!
//! The library is organized into the following modules:
//!
//! - [`cli`]: Command-line argument parsing and validation
//! - [`prompt`]: System prompt configuration and loading
//! - [`claude_executor`]: Claude command building and execution
//! - [`dialog`]: Main dialog loop and conversation flow management
//! - [`ui`]: Terminal user interface with colored output
//!
//! # Example Usage
//!
//! ```no_run
//! use claude_dialog::{cli, dialog, prompt, ui};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Parse command-line arguments
//! let args = cli::parse_args(vec!["claude-dialog", "--model", "claude-3-opus"])?;
//!
//! // Load system prompt
//! let prompt_config = prompt::SystemPromptConfig {
//!     system_prompt_files: args.system_prompt_files,
//!     append_prompt_file: args.append_prompt_file,
//! };
//! let system_prompt = prompt::load_system_prompt(prompt_config)?;
//!
//! // Create dialog configuration
//! let config = dialog::DialogConfig { 
//!     system_prompt: if !system_prompt.is_empty() { Some(system_prompt) } else { None },
//!     append_prompt: None,
//!     model: args.model,
//! };
//!
//! // Display welcome message
//! ui::UI::print_welcome("Using default prompt", config.model.as_ref());
//!
//! // Start the dialog loop
//! let dialog_loop = dialog::DialogLoop::new(config);
//! dialog_loop.run().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! - Interactive conversation mode with Claude AI
//! - Custom system prompts from file or command line
//! - Colored terminal output for better readability
//! - Session management with proper command building
//! - Support for various Claude models and parameters

/// Command-line interface module for parsing arguments
pub mod cli;

/// System prompt configuration and loading module
pub mod prompt;

/// Claude command execution module
pub mod claude_executor;

/// Dialog loop and conversation management module
pub mod dialog;

/// Terminal user interface module with colored output
pub mod ui;