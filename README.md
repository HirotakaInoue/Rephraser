# Rephraser

Transform text anywhere on macOS using AI-powered actions through the right-click context menu.

## Features

- 🔄 **AI-Powered Text Transformation** - Leverage OpenAI GPT or Anthropic Claude models
- 🖱️ **Right-Click Integration** - Access transformations directly from macOS context menu
- ⚙️ **Customizable Actions** - Define your own transformation prompts (polite, organize, summarize, translate, etc.)
- 🎯 **Multiple Output Methods** - Choose clipboard, notification, or dialog output
- 🦀 **Fast & Reliable** - Written in Rust for optimal performance

## Installation

### Prerequisites

- macOS (tested on Catalina and later)
- Rust toolchain (install from [rustup.rs](https://rustup.rs))
- OpenAI API key or Anthropic API key

### Install

```bash
# Clone the repository
git clone https://github.com/yourusername/rephraser.git
cd rephraser

# Install the binary
cargo install --path .

# Verify installation
rephraser --version
```

### Set up API key

Add your API key to your shell profile:

```bash
# For OpenAI
echo 'export OPENAI_API_KEY="sk-your-api-key"' >> ~/.zshrc
source ~/.zshrc

# For Anthropic
echo 'export ANTHROPIC_API_KEY="sk-ant-your-api-key"' >> ~/.zshrc
source ~/.zshrc
```

### Initialize configuration

```bash
rephraser config init
```

This creates `~/.rephraser/config.toml` with default settings.

## Usage

### Command Line

Transform text directly from the terminal:

```bash
rephraser rephrase polite "こんにちは"
# Output: こんにちは、お世話になっております。
```

List available actions:

```bash
rephraser list-actions
```

### macOS Quick Actions (Right-Click Menu)

Set up Quick Actions to transform text from anywhere on macOS:

1. Follow the setup guide in [`automator/README.md`](automator/README.md)
2. Select text in any application
3. Right-click → Services → "Rephraser - 丁寧に"
4. Result appears instantly

**Example workflow:**
- Writing an email → select informal text → right-click → "Rephraser - 丁寧に" → polite version copied to clipboard
- Reading a document → select long paragraph → right-click → "Rephraser - 要約" → concise summary in notification

## Configuration

Configuration file: `~/.rephraser/config.toml`

### Basic Configuration

```toml
[llm]
provider = "openai"          # or "anthropic"
model = "gpt-4o-mini"        # or "claude-3-5-sonnet-20241022"
api_key_env = "OPENAI_API_KEY"

[llm.parameters]
temperature = 0.7
max_tokens = 500

[output]
method = "clipboard"         # or "notification", "dialog"
```

### Define Custom Actions

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

### View/Edit Configuration

```bash
# Show current config
rephraser config show

# Show config file path
rephraser config path

# Edit manually
open ~/.rephraser/config.toml
```

## Supported LLM Providers

- **OpenAI**
- **Anthropic**

## Output Methods

- **clipboard**: Copy result to clipboard (paste with ⌘+V)
- **notification**: Show result in macOS Notification Center
- **dialog**: Display result in modal dialog box

## Development

### Build from Source

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Project Structure

```
rephraser/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── cli/                 # Command handling
│   ├── config/              # Configuration management
│   ├── actions/             # Action template resolution
│   ├── llm/                 # LLM provider implementations
│   └── output/              # Output method handlers
├── automator/               # macOS Quick Actions setup
├── docs/                    # Architecture documentation
└── examples/                # Configuration examples
```

See [docs/architecture.md](docs/architecture.md) for detailed architecture documentation.

## License

MIT

## Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - Command-line parsing
- [serde](https://github.com/serde-rs/serde) - Serialization
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime
