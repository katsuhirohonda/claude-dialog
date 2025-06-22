use anyhow::Result;
use clap::Parser;
use claude_dialog::{
    cli::Args,
    prompt::{load_system_prompt, SystemPromptConfig},
    dialog::{DialogLoop, DialogConfig},
    ui::UI,
};

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