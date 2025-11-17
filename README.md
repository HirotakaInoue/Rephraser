# Rephraser

macOS text transformation tool with LLM integration. Transform selected text using customizable actions through the right-click context menu.

## Features

- 🔄 Transform text using LLM (OpenAI GPT, Anthropic Claude)
- 🖱️ macOS right-click menu integration via Services
- ⚙️ Customizable actions (丁寧に, 整理する, 要約, etc.)
- 🎯 Multiple output methods (clipboard, notification, dialog)
- 🦀 Written in Rust for performance and reliability

## Project Status

**Current Phase: Mock Implementation**

This is the initial architecture phase. The project structure and interfaces are complete, with mock implementations for testing the design.

### Completed ✅

- [x] Project structure and architecture design
- [x] CLI framework with clap
- [x] Configuration management (TOML)
- [x] Action template system
- [x] LLM Client trait and Mock implementation
- [x] Output handler framework
- [x] Documentation

### In Progress 🚧

- [ ] OpenAI API client implementation
- [ ] Anthropic API client implementation
- [ ] macOS Services integration (Automator)
- [ ] Output methods (actual clipboard/notification/dialog)

## Quick Start

### Build and Test

```bash
# Build the project
cargo build

# Initialize configuration
cargo run -- config init

# List available actions
cargo run -- list-actions

# Test with mock LLM (requires config provider = "mock")
cargo run -- rephrase polite "こんにちは"
```

### Configuration

Configuration file is located at `~/.rephraser/config.toml`.

Example configuration:

```toml
[llm]
provider = "mock"  # "openai", "anthropic", or "mock"
model = "gpt-4o-mini"
api_key_env = "OPENAI_API_KEY"

[llm.parameters]
temperature = 0.7
max_tokens = 500

[output]
method = "notification"  # "clipboard", "notification", or "dialog"

[[actions]]
name = "polite"
display_name = "丁寧に"
prompt_template = """
以下のテキストを丁寧な表現に変換してください。

テキスト:
{text}

丁寧な表現:
"""
```

## Architecture

See [docs/architecture.md](docs/architecture.md) for detailed architecture documentation.

### System Overview

```
User (Right-click) → Automator Quick Action
                         ↓
                    Rust CLI (rephraser)
                         ↓
                  [Config] [Actions] [LLM Client]
                         ↓
                    Output Handler
```

### Core Components

- **CLI**: Command-line interface (clap)
- **Config Manager**: TOML configuration management
- **Action Resolver**: Template-based prompt generation
- **LLM Client**: Trait-based abstraction for multiple providers
  - Mock (for testing)
  - OpenAI (TODO)
  - Anthropic (TODO)
- **Output Handler**: Multiple output methods

## CLI Commands

```bash
# Transform text
rephraser rephrase <action> <text>

# Configuration
rephraser config init          # Initialize config file
rephraser config show          # Show current config
rephraser config set <k> <v>   # Set config value (TODO)
rephraser config path          # Show config file path

# List actions
rephraser list-actions
```

## Development

### Project Structure

```
Rephraser/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library root
│   ├── cli/              # CLI commands
│   ├── config/           # Configuration
│   ├── actions/          # Action resolution
│   ├── llm/              # LLM clients
│   ├── output/           # Output handlers
│   └── error.rs          # Error types
├── docs/
│   └── architecture.md   # Architecture documentation
├── examples/
│   └── example_config.toml
└── tests/
```

### Testing

```bash
# Run tests
cargo test

# Run with mock provider
# 1. Edit ~/.rephraser/config.toml, set provider = "mock"
# 2. Run commands
cargo run -- rephrase polite "test text"
```

### Adding Custom Actions

Edit your config file (`~/.rephraser/config.toml`):

```toml
[[actions]]
name = "translate"
display_name = "翻訳"
prompt_template = """
以下のテキストを英語に翻訳してください。

テキスト:
{text}

翻訳:
"""
```

## Next Steps

1. **LLM Integration**
   - Implement OpenAI API client
   - Implement Anthropic API client
   - Add error handling and retries

2. **macOS Integration**
   - Create Automator Quick Actions
   - Implement actual output methods (clipboard, notification, dialog)
   - Create installation script

3. **Polish**
   - Add comprehensive tests
   - Performance optimization
   - User documentation

## Contributing

This is currently in early development. Contributions welcome once the basic implementation is complete.

## License

MIT
