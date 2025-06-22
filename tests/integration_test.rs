use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("claude-dialog").unwrap();
    cmd.arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Interactive CLI for Claude conversations"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("claude-dialog").unwrap();
    cmd.arg("--version");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("claude-dialog"));
}

#[test]
fn test_exit_command() {
    let mut cmd = Command::cargo_bin("claude-dialog").unwrap();
    cmd.write_stdin("exit\n");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Claude Dialog Shell"))
        .stdout(predicate::str::contains("You>"))
        .stdout(predicate::str::contains("Exiting conversation"));
}

#[test]
fn test_quit_command() {
    let mut cmd = Command::cargo_bin("claude-dialog").unwrap();
    cmd.write_stdin("quit\n");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Claude Dialog Shell"))
        .stdout(predicate::str::contains("You>"))
        .stdout(predicate::str::contains("Exiting conversation"));
}

#[test]
fn test_with_system_prompt_file() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "You are a helpful assistant").unwrap();
    
    let mut cmd = Command::cargo_bin("claude-dialog").unwrap();
    cmd.arg("--system-prompt")
        .arg(temp_file.path())
        .write_stdin("exit\n");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("System Prompt:"))
        .stdout(predicate::str::contains(temp_file.path().to_str().unwrap()));
}

#[test]
fn test_with_model() {
    let mut cmd = Command::cargo_bin("claude-dialog").unwrap();
    cmd.arg("--model")
        .arg("claude-3-opus")
        .write_stdin("exit\n");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Model: claude-3-opus"));
}