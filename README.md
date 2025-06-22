# claude-dialog

Interactive CLI for Claude conversations with custom system prompts.

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

## Commands

- `exit` or `quit` - Exit the conversation

## Requirements

- `claude` CLI must be installed and available in PATH

## License

MIT License - see LICENSE file for details