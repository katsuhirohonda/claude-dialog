use claude_dialog::dialog::{DialogLoop, DialogConfig};
use std::io::Cursor;

#[test]
fn test_dialog_config() {
    let config = DialogConfig {
        system_prompt: Some("Test prompt".to_string()),
        append_prompt: None,
        model: None,
    };
    
    assert_eq!(config.system_prompt, Some("Test prompt".to_string()));
    assert!(config.append_prompt.is_none());
    assert!(config.model.is_none());
}

#[test]
fn test_is_exit_command() {
    let dialog = DialogLoop::new(DialogConfig {
        system_prompt: None,
        append_prompt: None,
        model: None,
    });
    
    assert!(dialog.is_exit_command("exit"));
    assert!(dialog.is_exit_command("quit"));
    assert!(dialog.is_exit_command("EXIT"));
    assert!(dialog.is_exit_command("QUIT"));
    assert!(dialog.is_exit_command("Exit"));
    assert!(dialog.is_exit_command("Quit"));
    assert!(!dialog.is_exit_command("hello"));
    assert!(!dialog.is_exit_command(""));
}

#[test]
fn test_read_input_empty() {
    let input = b"\n";
    let mut cursor = Cursor::new(input);
    let dialog = DialogLoop::new(DialogConfig {
        system_prompt: None,
        append_prompt: None,
        model: None,
    });
    
    let result = dialog.read_input(&mut cursor).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_read_input_valid() {
    let input = b"hello world\n";
    let mut cursor = Cursor::new(input);
    let dialog = DialogLoop::new(DialogConfig {
        system_prompt: None,
        append_prompt: None,
        model: None,
    });
    
    let result = dialog.read_input(&mut cursor).unwrap();
    assert_eq!(result, Some("hello world".to_string()));
}

#[test]
fn test_read_input_exit() {
    let input = b"exit\n";
    let mut cursor = Cursor::new(input);
    let dialog = DialogLoop::new(DialogConfig {
        system_prompt: None,
        append_prompt: None,
        model: None,
    });
    
    let result = dialog.read_input(&mut cursor).unwrap();
    assert_eq!(result, Some("exit".to_string()));
}