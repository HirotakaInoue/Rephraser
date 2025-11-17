# Rephraser Architecture

## Overview

Rephraser is a macOS text processing tool that integrates with the system's right-click context menu. It uses LLM APIs to transform selected text based on customizable actions.

## System Architecture

```
User (Right-click) → Automator Quick Action
                         ↓
                    Rust CLI (rephraser)
                         ↓
                  [Config Manager] [Action Resolver]
                         ↓
                    LLM Client (Trait)
                    ├─ OpenAI
                    ├─ Anthropic
                    └─ Mock (for testing)
                         ↓
                    Output → Clipboard/Notification/Dialog
```

## Architecture Decision

### Chosen Approach: Automator + Rust CLI

**Why this approach:**
- Simple implementation
- Distributable via `cargo install`
- No code signing required
- No App Store review needed
- Native macOS UX via Services

**Alternatives considered:**
- Swift Extension + Rust FFI (too complex)
- Status Bar App + Global Hotkey (doesn't appear in right-click menu)

## System Components

### 1. CLI Parser (`src/cli/`)

**Responsibilities:**
- Parse command-line arguments
- Handle subcommands

**Commands:**
```bash
rephraser rephrase <action> <text>  # Execute text transformation
rephraser config init               # Initialize config file
rephraser config set <key> <value>  # Modify configuration
rephraser config show               # Display current configuration
rephraser list-actions              # List available actions
```

### 2. Config Manager (`src/config/`)

**Responsibilities:**
- Load/save configuration files
- Validate configuration
- Provide default settings

**Configuration Format (TOML):**
```toml
# ~/.rephraser/config.toml

[llm]
provider = "openai"  # "openai", "anthropic"
model = "gpt-4o-mini"
api_key_env = "OPENAI_API_KEY"

[llm.parameters]
temperature = 0.7
max_tokens = 500

[output]
method = "notification"  # "clipboard", "notification", "dialog"

[[actions]]
name = "polite"
display_name = "丁寧に"
prompt_template = """
以下のテキストを丁寧な表現に変換してください。

テキスト:
{text}

丁寧な表現:
"""

[[actions]]
name = "organize"
display_name = "整理する"
prompt_template = """
以下のテキストを論理的に整理してください。

テキスト:
{text}

整理されたテキスト:
"""
```

### 3. Action Resolver (`src/actions/`)

**Responsibilities:**
- Resolve action name to prompt template
- Substitute template variables (`{text}`)
- Provide built-in actions

**Template System:**
- Simple variable substitution: `{text}`, `{language}`, etc.
- Custom actions loaded from config

### 4. LLM Client (`src/llm/`)

**Trait Definition:**
```rust
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<String>;
}
```

**Implementations:**
- `OpenAiClient` - OpenAI API (GPT-4, GPT-3.5)
- `AnthropicClient` - Anthropic API (Claude)
- `MockLlmClient` - Mock for testing

### 5. Output Formatter (`src/output/`)

**Responsibilities:**
- Format LLM responses
- Handle output methods (clipboard/notification/dialog)
- Generate error messages

**Output Methods:**
- **Clipboard**: Copy result to clipboard via `pbcopy`
- **Notification**: Display via macOS Notification Center
- **Dialog**: Show in dialog box via `osascript`

## Data Flow

```
User Selection → Automator → CLI Input
                                 ↓
                          Load Config
                                 ↓
                       Resolve Action
                                 ↓
                      Build Prompt
                                 ↓
                  Call LLM API (async)
                                 ↓
                      Parse Response
                                 ↓
                    Format Output
                                 ↓
             Output via selected method
```

## Directory Structure

```
Rephraser/
├── Cargo.toml
├── README.md
├── LICENSE
├── .gitignore
│
├── docs/
│   └── architecture.md
│
├── src/
│   ├── main.rs                 # CLI entry point
│   ├── lib.rs                  # Library entry point
│   │
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── commands.rs         # CLI command definitions
│   │   └── args.rs             # Argument parsing
│   │
│   ├── config/
│   │   ├── mod.rs
│   │   ├── manager.rs          # Config load/save
│   │   ├── models.rs           # Config data structures
│   │   └── validation.rs       # Config validation
│   │
│   ├── actions/
│   │   ├── mod.rs
│   │   ├── resolver.rs         # Action resolution
│   │   ├── template.rs         # Prompt templating
│   │   └── builtin.rs          # Default actions
│   │
│   ├── llm/
│   │   ├── mod.rs
│   │   ├── client.rs           # Trait definition
│   │   ├── openai.rs           # OpenAI implementation
│   │   ├── anthropic.rs        # Anthropic implementation
│   │   └── mock.rs             # Mock implementation
│   │
│   ├── output/
│   │   ├── mod.rs
│   │   └── formatter.rs        # Output formatting
│   │
│   └── error.rs                # Error type definitions
│
├── tests/
│   ├── integration_test.rs
│   └── fixtures/
│       └── sample_config.toml
│
├── examples/
│   └── example_config.toml
│
├── automator/
│   ├── README.md               # Automator setup instructions
│   └── templates/              # Quick Action templates
│
└── install.sh                  # Installation script
```

## Technology Stack

### Core Dependencies

```toml
[dependencies]
# CLI framework
clap = { version = "4.5", features = ["derive"] }

# Configuration management
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# LLM API clients
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
tokio = { version = "1.40", features = ["full"] }
async-trait = "0.1"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Utilities
dirs = "5.0"

[dev-dependencies]
mockito = "1.5"  # HTTP mock server
```

### LLM Provider Support

**Phase 1 (Initial):**
- OpenAI (GPT-4, GPT-3.5-turbo)
- Anthropic (Claude Sonnet, Opus, Haiku)

**Phase 2 (Future):**
- Azure OpenAI
- Google Gemini
- Local LLM (Ollama)

## Implementation Roadmap

### Phase 1: Foundation (Mock Implementation)

**Goals:**
- Establish project structure
- Define traits and structs
- Implement mock LLM client
- Basic CLI functionality

**Deliverables:**
- Working CLI with mock responses
- Configuration system skeleton
- Action template system

### Phase 2: Core Features

**Goals:**
- Implement OpenAI client
- Implement Anthropic client
- Full configuration management
- Error handling

**Deliverables:**
- Real LLM integration
- Robust error handling
- Configuration commands

### Phase 3: macOS Integration

**Goals:**
- Create Automator Quick Actions
- Installation script
- Output method implementations

**Deliverables:**
- Right-click menu integration
- All output methods working
- Installation documentation

### Phase 4: Polish

**Goals:**
- Testing
- Documentation
- Performance optimization

**Deliverables:**
- Comprehensive tests
- User documentation
- Ready for distribution

## Technical Challenges & Solutions

### Challenge 1: Output Display from Automator

**Problem:** Automator Quick Action output UX is poor

**Solutions:**
1. **Clipboard**: Use `pbcopy` command
2. **Notification**: Use `osascript` for notification center
3. **Dialog**: Use `osascript` for alert dialog

**Implementation:**
```bash
# Automator Script
result=$(~/.cargo/bin/rephraser rephrase polite "$1")

# Based on output method in config:
# - clipboard:
echo "$result" | pbcopy

# - notification:
osascript -e "display notification \"$result\" with title \"Rephraser\""

# - dialog:
osascript -e "display dialog \"$result\" buttons {\"OK\"} default button 1"
```

### Challenge 2: API Key Security

**Problem:** Storing API keys in plain text is risky

**Solution (Phase 1):**
- Store environment variable name in config
- Read actual API key from environment at runtime

```toml
[llm]
api_key_env = "OPENAI_API_KEY"
```

**Future Enhancement:**
- macOS Keychain integration via `security` command

### Challenge 3: Multiple Action Selection

**Problem:** User needs to select from multiple actions

**Solution (Phase 1):**
- Create separate Quick Action for each action
- Installation script generates them automatically

```bash
# install.sh
for action in polite organize summarize; do
  create_quick_action "$action"
done
```

**Future Enhancement:**
- Single Quick Action with selection dialog

### Challenge 4: Long Text Processing

**Problem:** LLM token limits and slow responses

**Solutions:**
1. Input length validation (configurable max)
2. Streaming responses (future)
3. Progress notification

```rust
if input.len() > config.max_input_length {
    return Err(anyhow!("Input too long (max {} chars)",
        config.max_input_length));
}
```

## Security Considerations

1. **API Key Storage**: Use environment variables, never commit keys
2. **Input Validation**: Sanitize user input before sending to LLM
3. **Output Sanitization**: Escape special characters in notifications/dialogs
4. **Network Security**: Use HTTPS (rustls-tls)

## Testing Strategy

### Unit Tests
- Config parsing and validation
- Template substitution
- Mock LLM client

### Integration Tests
- End-to-end CLI flow with mock
- Configuration management
- Action resolution

### Manual Testing
- Real LLM API calls
- Automator integration
- macOS notification/dialog display

## Future Enhancements

1. **Advanced Features:**
   - Streaming responses
   - Batch processing
   - History/undo functionality

2. **UI Improvements:**
   - TUI for configuration
   - Web-based settings panel

3. **LLM Features:**
   - Custom system prompts
   - Temperature/parameter tuning per action
   - Model fallback chains

4. **Distribution:**
   - Homebrew formula
   - Pre-built binaries
   - Auto-update mechanism

## Glossary

- **Quick Action**: macOS Services that appear in right-click menus
- **Automator**: macOS tool for creating workflows
- **Trait**: Rust interface definition
- **LLM**: Large Language Model
- **TOML**: Configuration file format
