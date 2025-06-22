use claude_dialog::claude_executor::ClaudeCommand;

#[test]
fn test_claude_command_basic() {
    let cmd = ClaudeCommand {
        prompt: "Hello".to_string(),
        system_prompt: None,
        append_prompt: None,
        model: None,
    };
    
    assert_eq!(cmd.prompt, "Hello");
    assert!(cmd.system_prompt.is_none());
    assert!(cmd.append_prompt.is_none());
    assert!(cmd.model.is_none());
}

#[test]
fn test_claude_command_with_system_prompt() {
    let cmd = ClaudeCommand {
        prompt: "Hello".to_string(),
        system_prompt: Some("You are helpful".to_string()),
        append_prompt: None,
        model: None,
    };
    
    assert!(cmd.system_prompt.is_some());
    assert_eq!(cmd.system_prompt.unwrap(), "You are helpful");
}

#[test]
fn test_claude_command_with_append_prompt() {
    let cmd = ClaudeCommand {
        prompt: "Hello".to_string(),
        system_prompt: None,
        append_prompt: Some("Additional context".to_string()),
        model: None,
    };
    
    assert!(cmd.append_prompt.is_some());
    assert_eq!(cmd.append_prompt.unwrap(), "Additional context");
}

#[test]
fn test_claude_command_with_model() {
    let cmd = ClaudeCommand {
        prompt: "Hello".to_string(),
        system_prompt: None,
        append_prompt: None,
        model: Some("claude-3-opus".to_string()),
    };
    
    assert!(cmd.model.is_some());
    assert_eq!(cmd.model.unwrap(), "claude-3-opus");
}

#[test]
fn test_build_command_args() {
    let cmd = ClaudeCommand {
        prompt: "Hello".to_string(),
        system_prompt: Some("System".to_string()),
        append_prompt: None,
        model: Some("claude-3".to_string()),
    };
    
    let args = cmd.build_args();
    assert!(args.contains(&"--continue".to_string()));
    assert!(args.contains(&"-p".to_string()));
    assert!(args.contains(&"Hello".to_string()));
    assert!(args.contains(&"--system-prompt".to_string()));
    assert!(args.contains(&"System".to_string()));
    assert!(args.contains(&"--model".to_string()));
    assert!(args.contains(&"claude-3".to_string()));
    assert!(args.contains(&"--allowedTools".to_string()));
    assert!(args.contains(&"Write".to_string()));
    assert!(args.contains(&"Edit".to_string()));
}

#[test]
fn test_build_command_args_append() {
    let cmd = ClaudeCommand {
        prompt: "Hello".to_string(),
        system_prompt: None,
        append_prompt: Some("Append".to_string()),
        model: None,
    };
    
    let args = cmd.build_args();
    assert!(args.contains(&"--append-system-prompt".to_string()));
    assert!(args.contains(&"Append".to_string()));
    assert!(!args.contains(&"--system-prompt".to_string()));
}