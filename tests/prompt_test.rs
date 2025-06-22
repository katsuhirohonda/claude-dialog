use claude_dialog::prompt::{load_system_prompt, SystemPromptConfig};
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_no_system_prompt() {
    let config = SystemPromptConfig {
        system_prompt_files: vec![],
        append_prompt_file: None,
    };
    let result = load_system_prompt(config).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_single_system_prompt_file() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "This is a test prompt").unwrap();
    
    let config = SystemPromptConfig {
        system_prompt_files: vec![temp_file.path().to_str().unwrap().to_string()],
        append_prompt_file: None,
    };
    
    let result = load_system_prompt(config).unwrap();
    assert_eq!(result, "This is a test prompt\n");
}

#[test]
fn test_multiple_system_prompt_files() {
    let mut temp_file1 = NamedTempFile::new().unwrap();
    writeln!(temp_file1, "First prompt").unwrap();
    
    let mut temp_file2 = NamedTempFile::new().unwrap();
    writeln!(temp_file2, "Second prompt").unwrap();
    
    let config = SystemPromptConfig {
        system_prompt_files: vec![
            temp_file1.path().to_str().unwrap().to_string(),
            temp_file2.path().to_str().unwrap().to_string(),
        ],
        append_prompt_file: None,
    };
    
    let result = load_system_prompt(config).unwrap();
    assert_eq!(result, "First prompt\n\n\nSecond prompt\n");
}

#[test]
fn test_append_prompt_file() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Additional prompt").unwrap();
    
    let config = SystemPromptConfig {
        system_prompt_files: vec![],
        append_prompt_file: Some(temp_file.path().to_str().unwrap().to_string()),
    };
    
    let result = load_system_prompt(config).unwrap();
    assert_eq!(result, "Additional prompt\n");
}

#[test]
fn test_missing_file() {
    let config = SystemPromptConfig {
        system_prompt_files: vec!["nonexistent.md".to_string()],
        append_prompt_file: None,
    };
    
    let result = load_system_prompt(config);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("nonexistent.md"));
}

#[test]
fn test_empty_file() {
    let temp_file = NamedTempFile::new().unwrap();
    // Don't write anything to the file
    
    let config = SystemPromptConfig {
        system_prompt_files: vec![temp_file.path().to_str().unwrap().to_string()],
        append_prompt_file: None,
    };
    
    let result = load_system_prompt(config).unwrap();
    assert_eq!(result, "");
}