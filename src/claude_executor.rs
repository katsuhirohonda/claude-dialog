use anyhow::{Result, Context};
use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct ClaudeCommand {
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub append_prompt: Option<String>,
    pub model: Option<String>,
}

impl ClaudeCommand {
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
}