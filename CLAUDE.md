# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

skillscli is a Rust-based command-line interface application built using:
- **clap** (v4.5) for command-line argument parsing with derive macros
- **anyhow** for ergonomic error handling
- **tokio** for async runtime support

## Development Commands

### Building and Running

```bash
# Build the project
cargo build

# Build with optimizations (release mode)
cargo build --release

# Run the CLI
cargo run

# Run with arguments
cargo run -- --verbose
cargo run -- --help
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

### Code Quality

```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Lint with all warnings
cargo clippy -- -W clippy::all
```

## Architecture

### Main Entry Point
- **src/main.rs**: Contains the CLI definition using clap's derive macros and the main application logic

### CLI Structure
The CLI uses clap's `Parser` derive macro for argument parsing. The `Cli` struct defines available command-line options and arguments. To add new options:
1. Add fields to the `Cli` struct with appropriate `#[arg()]` attributes
2. Handle the new options in the `main()` function

### Error Handling
The project uses `anyhow::Result<()>` as the return type for `main()` and should be used throughout for propagating errors with the `?` operator.

### Async Support
tokio is included with the "full" feature set for async/await support. To use async in main:
```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // async code here
}
```

## Adding Dependencies

Edit `Cargo.toml` and add dependencies under `[dependencies]`, then run:
```bash
cargo build
```

## Project Conventions

- Use `anyhow::Result` for error handling in functions that can fail
- Use clap's derive macros for CLI argument definitions
- Keep CLI logic in `main.rs` until the project grows to require modules
