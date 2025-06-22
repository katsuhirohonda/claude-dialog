use clap::Parser;
use anyhow::{Result, anyhow};

#[derive(Parser, Debug)]
#[command(
    name = "claude-dialog",
    about = "Interactive CLI for Claude conversations with custom system prompts",
    long_about = None
)]
pub struct Args {
    /// Custom system prompt files (can be specified multiple times)
    #[arg(long = "system-prompt", value_name = "FILE", action = clap::ArgAction::Append)]
    pub system_prompt_files: Vec<String>,

    /// Append to default system prompt
    #[arg(long = "append-system-prompt", value_name = "FILE", conflicts_with = "system_prompt_files")]
    pub append_prompt_file: Option<String>,

    /// Claude model to use
    #[arg(long = "model", value_name = "MODEL")]
    pub model: Option<String>,
}

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