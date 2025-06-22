use claude_dialog::cli::parse_args;

#[test]
fn test_default_args() {
    let args = vec!["claude-dialog"];
    let result = parse_args(args);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert!(args.system_prompt_files.is_empty());
    assert!(args.append_prompt_file.is_none());
    assert!(args.model.is_none());
}

#[test]
fn test_single_system_prompt() {
    let args = vec!["claude-dialog", "--system-prompt", "prompt.md"];
    let result = parse_args(args);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(args.system_prompt_files, vec!["prompt.md"]);
    assert!(args.append_prompt_file.is_none());
}

#[test]
fn test_multiple_system_prompts() {
    let args = vec![
        "claude-dialog",
        "--system-prompt", "prompt1.md",
        "--system-prompt", "prompt2.md"
    ];
    let result = parse_args(args);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(args.system_prompt_files, vec!["prompt1.md", "prompt2.md"]);
}

#[test]
fn test_append_prompt() {
    let args = vec!["claude-dialog", "--append-system-prompt", "append.md"];
    let result = parse_args(args);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert!(args.system_prompt_files.is_empty());
    assert_eq!(args.append_prompt_file, Some("append.md".to_string()));
}

#[test]
fn test_model_option() {
    let args = vec!["claude-dialog", "--model", "claude-3-opus-20240229"];
    let result = parse_args(args);
    assert!(result.is_ok());
    let args = result.unwrap();
    assert_eq!(args.model, Some("claude-3-opus-20240229".to_string()));
}

#[test]
fn test_system_and_append_conflict() {
    let args = vec![
        "claude-dialog",
        "--system-prompt", "prompt.md",
        "--append-system-prompt", "append.md"
    ];
    let result = parse_args(args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("cannot be used with"));
}

#[test]
fn test_help_option() {
    let args = vec!["claude-dialog", "--help"];
    let result = parse_args(args);
    // Help should cause an early exit, which we handle as an error in tests
    assert!(result.is_err());
}