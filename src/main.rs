//! Claude Dialog - Interactive CLI for conversations with Claude AI
//!
//! This is the main entry point for the Claude Dialog application.
//! It handles initialization, argument parsing, prompt configuration,
//! and starts the interactive conversation loop.
//!
//! # Usage
//!
//! ```bash
//! # Basic usage with default settings
//! claude-dialog
//!
//! # Use a custom system prompt
//! claude-dialog --system-prompt custom_prompt.md
//!
//! # Use multiple system prompts
//! claude-dialog --system-prompt base.md --system-prompt specific.md
//!
//! # Append to the default prompt
//! claude-dialog --append-system-prompt additions.md
//!
//! # Specify a model
//! claude-dialog --model claude-3-opus
//! ```
//!
//! # Exit
//!
//! Type "exit" or "quit" during the conversation to end the session.

use anyhow::Result;
use clap::Parser;
use claude_dialog::{
    cli::Args,
    prompt::{load_system_prompt, SystemPromptConfig},
    dialog::{DialogLoop, DialogConfig},
    ui::UI,
};

/// Main entry point for the Claude Dialog application
///
/// This function orchestrates the entire application flow:
/// 1. Parses command-line arguments
/// 2. Loads and configures system prompts
/// 3. Displays welcome information
/// 4. Starts the interactive dialog loop
///
/// # Errors
///
/// Returns an error if:
/// - System prompt files cannot be read
/// - The dialog loop encounters an error during execution
#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();
    
    // Load system prompt
    let prompt_config = SystemPromptConfig {
        system_prompt_files: args.system_prompt_files.clone(),
        append_prompt_file: args.append_prompt_file.clone(),
    };
    
    let system_prompt = load_system_prompt(prompt_config)?;
    
    // Determine system prompt info for display
    let system_prompt_info = if !args.system_prompt_files.is_empty() {
        args.system_prompt_files.join(", ")
    } else if let Some(append_file) = &args.append_prompt_file {
        format!("Default + {}", append_file)
    } else {
        "Default".to_string()
    };
    
    // Print welcome message
    UI::print_welcome(&system_prompt_info, args.model.as_ref());
    
    // Create dialog configuration
    let dialog_config = DialogConfig {
        system_prompt: if !system_prompt.is_empty() {
            Some(system_prompt.clone())
        } else {
            None
        },
        append_prompt: if args.append_prompt_file.is_some() {
            Some(system_prompt)
        } else {
            None
        },
        model: args.model,
    };
    
    // Run the dialog loop
    let dialog = DialogLoop::new(dialog_config);
    dialog.run().await?;
    
    Ok(())
}