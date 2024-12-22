# Scripture: Structured Git Commit Message Generator

A terminal-based application that helps generate well-structured git commit messages following standard conventions.

## Features

- Interactive TUI for commit message creation
- Standard-compliant commit message formatting
- Automatic analysis of git diffs
- Detection of breaking changes
- Configurable commit types and scopes
- Message preview with proper formatting
- Support for issue references

## Installation

```bash
cargo install scripture
```

## Usage

1. Stage your changes with `git add`
2. Run `scripture`
3. Navigate tabs with Tab/Shift+Tab
4. Edit commit details
5. Press Enter to save
6. Commit using `git commit -F commit.md`

## Configuration

Scripture uses a TOML configuration file located at `~/.config/scripture/scripture.toml`. The default configuration will be created automatically on first run.

You can customise:
- Commit types and their descriptions
- Scopes for different areas of the project
- Message templates
- Breaking change indicators
- Format rules (line lengths, etc.)

## Standards

Scripture follows these commit message standards:
- Subject line limited to 50 characters
- Body wrapped at 72 columns
- Imperative mood in subject
- Proper section separation
- Standard terminology for commit types
- Focus on what/why in the message body

## Licence

MIT
