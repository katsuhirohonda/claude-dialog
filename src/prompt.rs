use anyhow::{Result, Context};
use std::fs;

#[derive(Debug)]
pub struct SystemPromptConfig {
    pub system_prompt_files: Vec<String>,
    pub append_prompt_file: Option<String>,
}

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