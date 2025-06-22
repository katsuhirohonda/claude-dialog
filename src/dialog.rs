use anyhow::Result;
use std::io::{self, BufRead, BufReader, Read, Write};
use crate::claude_executor::{ClaudeCommand, execute_claude};

#[derive(Debug, Clone)]
pub struct DialogConfig {
    pub system_prompt: Option<String>,
    pub append_prompt: Option<String>,
    pub model: Option<String>,
}

pub struct DialogLoop {
    config: DialogConfig,
}

impl DialogLoop {
    pub fn new(config: DialogConfig) -> Self {
        Self { config }
    }
    
    pub fn is_exit_command(&self, input: &str) -> bool {
        let lower = input.to_lowercase();
        lower == "exit" || lower == "quit"
    }
    
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
    
    pub async fn run(&self) -> Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        
        loop {
            // Show prompt
            print!("> ");
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
                println!("Exiting...");
                break;
            }
            
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
}