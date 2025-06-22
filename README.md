# claude-dialog

Interactive CLI for engaging in continuous conversations with Claude Code. This tool provides a shell-like interface for chatting with Claude AI, with support for custom system prompts and model selection.

## Overview

`claude-dialog` creates an interactive chat environment with Claude Code, allowing you to:
- Have continuous back-and-forth conversations with Claude
- Use custom system prompts to guide Claude's behavior
- Maintain conversation context throughout the session
- Switch between different Claude models

## Prerequisites

Before installing `claude-dialog`, ensure you have the following:

### Required
- **Claude Code CLI**: The `claude` command must be installed and available in your PATH
  - Install from: https://github.com/anthropics/claude-code
  - Verify installation: `claude --version`
- **Rust and Cargo**: Required to build and install from source
  - Install from: https://rustup.rs/
  - Minimum Rust version: 1.70.0

### System Requirements
- Operating System: macOS, Linux, or Windows
- Terminal with color support for best experience

## Installation

```bash
cargo install claude-dialog
```

## Usage

### Basic usage

```bash
claude-dialog
```

### With custom system prompt

```bash
claude-dialog --system-prompt custom_prompt.md
```

### With multiple system prompts (concatenated)

```bash
claude-dialog --system-prompt prompt1.md --system-prompt prompt2.md
```

### Append to default system prompt

```bash
claude-dialog --append-system-prompt additional_prompt.md
```

### Specify model

```bash
claude-dialog --model claude-3-opus-20240229
```

## Interactive Chat Experience

Once started, `claude-dialog` provides an interactive shell where you can:

1. Type your questions or prompts
2. Claude will respond directly in the terminal
3. Continue the conversation with follow-up questions
4. Claude maintains context from previous messages in the session

Example session:
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Claude Dialog Shell
System Prompt: Default
Model: claude-3-opus-20240229
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Type 'exit' or 'quit' to end the conversation

You> Hello Claude, can you help me with Python?
Claude>
I'd be happy to help you with Python! What specific topic or problem would you like assistance with?

You> How do I read a JSON file?
Claude>
Here's how to read a JSON file in Python...
```

## Commands

- `exit` or `quit` - Exit the conversation

## License

MIT License - see LICENSE file for details