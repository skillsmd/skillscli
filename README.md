# Skills CLI

A command-line tool for managing and installing Claude skills from various marketplaces.

[![CI](https://github.com/YOUR_USERNAME/skillscli/workflows/CI/badge.svg)](https://github.com/YOUR_USERNAME/skillscli/actions)
[![Release](https://github.com/YOUR_USERNAME/skillscli/workflows/Release/badge.svg)](https://github.com/YOUR_USERNAME/skillscli/releases)

## Features

- 🔍 **Search skills** across multiple marketplaces
- 📦 **Install skills** by name or GitHub URL
- 🌐 **Manage marketplaces** - add custom skill repositories
- 🎯 **Multiple targets** - support for Codex and Copilot
- 🌍 **Global & Local** installation options

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/YOUR_USERNAME/skillscli/releases).

#### Linux / macOS

```bash
# Download and extract (replace VERSION and PLATFORM)
curl -L https://github.com/YOUR_USERNAME/skillscli/releases/download/vVERSION/skills-PLATFORM.tar.gz | tar xz

# Move to a directory in your PATH
sudo mv skills /usr/local/bin/

# Verify installation
skills --help
```

#### Windows

Download `skills-windows-x86_64.exe.zip` from the releases page, extract it, and add the directory to your PATH.

### Build from Source

```bash
git clone https://github.com/YOUR_USERNAME/skillscli.git
cd skillscli
cargo build --release
sudo cp target/release/skills /usr/local/bin/
```

## Quick Start

### Search for Skills

```bash
# Search for skills in all configured markets
skills search pptx

# Output:
# Found 1 skill(s):
#   • pptx (anthropics/skills)
#     URL: https://github.com/anthropics/skills/tree/main/skills/pptx
```

### Install a Skill

```bash
# Install by skill name (from market)
skills install pptx -t codex

# Install globally (to ~/.codex/skills/)
skills install pptx -t codex -g

# Install from GitHub URL
skills install https://github.com/anthropics/skills/tree/main/skills/pptx -t codex
```

### Manage Marketplaces

```bash
# Add a custom marketplace
skills market add https://github.com/makenotion/notion-cookbook/tree/main/skills/claude

# Search within markets
skills market search meeting

# List is stored in ~/.skills/market.json
```

## Usage

### Commands

```
skills
├── install <skill-name-or-url> -t <type> [-g]
│   Install a skill by name or GitHub URL
│
├── search <query>
│   Search for skills in configured markets
│
└── market
    ├── add <url>
    │   Add a new marketplace
    │
    └── search <query>
        Search within marketplaces
```

### Options

- `-t, --type <TYPE>`: Target type (codex, copilot) - **required**
- `-g, --global`: Install globally to `~/.{type}/skills/` instead of `./.{type}/skills/`

### Examples

```bash
# Install a skill locally for Codex
skills install frontend-design -t codex

# Install globally for Copilot
skills install meeting-intelligence -t copilot -g

# Search for document-related skills
skills search doc

# Add Notion's cookbook to marketplaces
skills market add https://github.com/makenotion/notion-cookbook/tree/main/skills/claude

# Install from a specific marketplace
skills install meeting-intelligence -t codex
```

## Configuration

Skills are managed through `~/.skills/market.json`:

```json
[
  {
    "name": "makenotion/notion-cookbook",
    "url": "https://github.com/makenotion/notion-cookbook/tree/main/skills/claude"
  }
]
```

The default Anthropic skills marketplace (`anthropics/skills`) is always included.

## Default Marketplaces

- **Anthropic Skills**: [github.com/anthropics/skills](https://github.com/anthropics/skills) (default)
- Add your own with `skills market add <url>`

## Installation Locations

| Target | Global (`-g`) | Local (default) |
|--------|---------------|-----------------|
| Codex | `~/.codex/skills/` | `./.codex/skills/` |
| Copilot | `~/.copilot/skills/` | `./.copilot/skills/` |

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

### Project Structure

```
skillscli/
├── src/
│   └── main.rs          # Main CLI implementation
├── .github/
│   └── workflows/
│       ├── ci.yml       # CI workflow
│       └── release.yml  # Release workflow
├── Cargo.toml
├── CLAUDE.md            # Development guidance
└── README.md
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [clap](https://github.com/clap-rs/clap) for CLI parsing
- Uses [reqwest](https://github.com/seanmonstar/reqwest) for HTTP requests
- Skills from [Anthropic](https://github.com/anthropics/skills) and community contributors
