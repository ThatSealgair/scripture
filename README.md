[![Crates.io](https://img.shields.io/crates/v/scripture.svg)](https://crates.io/crates/scripture)
[![Documentation](https://docs.rs/scripture/badge.svg)](https://docs.rs/scripture/)

# Scripture

A robust command-line tool for managing, validating, and generating standardised Git commit messages. This tool helps maintain consistent commit message standards across your project by providing automated message generation, validation, and formatting capabilities.

## Features

- **Automated Commit Message Generation**: Analyses staged changes and generates structured commit messages
- **Message Validation**: Ensures commit messages follow standard conventions and best practices
- **Breaking Change Detection**: Automatically identifies and highlights breaking changes
- **Customisable Templates**: Supports structured sections for references, testing, dependencies, and more
- **Standard Verb Enforcement**: Maintains consistent terminology in commit messages

## Installation

To install the tool, ensure you have Rust and Cargo installed on your system. Then:

```bash
cargo install scripture
```

## Usage

### Generating a Commit Message

Simply stage your changes and run the tool:

```bash
git add .
scripture
```

This will:
1. Analyse your staged changes
2. Generate a structured commit message in `commit.md`
3. Display the message and usage instructions

### Validating a Commit Message

Validate a commit message string:
```bash
scripture -m "Add new feature"
```

Or validate a commit message file:
```bash
scripture -f path/to/message.txt
```

## Commit Message Structure

Generated commit messages follow this structure:

1. Subject line (max 50 characters)
   - Starts with a standard verb
   - Capitalised
   - No trailing full stop

2. Message body sections:
   - References [Required]
   - Changes Overview [Required]
   - Breaking Changes [Required if any]
   - Testing Instructions [Optional]
   - Dependencies [Optional]

### Standard Verbs

The tool enforces these standard verbs:
- **Add**: Create a capability, e.g., feature, test, dependency
- **Cut**: Remove a capability, e.g., feature, test, dependency
- **Fix**: Fix an issue, e.g., bug, typo, error, misstatement

### Breaking Change Detection

The tool automatically identifies breaking changes based on keywords such as:
- remove
- delete
- deprecate
- break
- change
- rename
- refactor
- drop
- migrate

## Message Validation Rules

Commit messages are validated against these rules:
- Subject line must not exceed 50 characters
- Subject must start with a standard verb
- Subject line must be capitalised
- No full stop at the end of the subject line
- Blank line between subject and body
- Body lines must not exceed 72 characters

## Configuration

The tool uses a default configuration that defines:
- Standard verbs and their descriptions
- Change indicators and verb mappings
- Message template sections
- Breaking change indicators

## Template Sections

Generated commit messages include these sections:

### References Section
```markdown
# References [Required]
# Link to related tickets, docs, or discussions
Closes #
Relates to #
See also:
```

### Testing Section
```markdown
# Testing Instructions [Optional]
# Describe how to test these changes
1. Steps to test
2. Expected outcomes
3. Edge cases to verify
```

### Dependencies Section
```markdown
# Dependencies [Optional]
# List any prerequisite changes or dependencies
- [ ] Database migrations
- [ ] Configuration updates
- [ ] External service changes
```

## Error Handling

The tool provides clear error messages for:
- Invalid commit message format
- Missing staged changes
- File read/write errors
- Invalid message structure

## Contributing

Contributions are welcome! Please ensure your commits follow the standards enforced by this tool.

## License

MIT License

Copyright (c) 2024 [Author Name]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
