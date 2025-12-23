# Installation Guide

This guide provides multiple ways to install the `skills` CLI tool.

## Table of Contents
- [Homebrew (macOS/Linux)](#homebrew-macoslinux)
- [Direct Download](#direct-download)
- [From Source](#from-source)

---

## Homebrew (macOS/Linux)

### Option 1: Install from Homebrew Tap (Recommended)

Once you've created a Homebrew tap repository, users can install with:

```bash
# Add your tap
brew tap skillsmd/skills

# Install skills
brew install skills
```

### Option 2: Install from Formula URL

Users can install directly from the formula file:

```bash
brew install https://raw.githubusercontent.com/skillsmd/skills/main/Formula/skills.rb
```

### Updating via Homebrew

```bash
# Update tap
brew update

# Upgrade skills
brew upgrade skills
```

### Uninstalling

```bash
brew uninstall skills
```

---

## Setting Up Your Homebrew Tap

If you want to distribute via Homebrew, follow these steps:

### 1. Create a Tap Repository

Create a new GitHub repository named `homebrew-skills` (must start with `homebrew-`).

### 2. Add the Formula

Copy the formula to your tap repository:

```bash
# In your homebrew-skills repository
mkdir -p Formula
cp Formula/skills.rb Formula/
```

### 3. Update the Formula

Before publishing, you need to:

1. **Replace placeholders** in `Formula/skills.rb`:
   - `skillsmd` → your GitHub username
   - `REPLACE_WITH_ACTUAL_SHA256` → actual SHA256 checksums

2. **Generate SHA256 checksums** after creating a release:

```bash
# Download your release assets
curl -LO https://github.com/skillsmd/skills/releases/download/v0.1.0/skills-macos-aarch64.tar.gz
curl -LO https://github.com/skillsmd/skills/releases/download/v0.1.0/skills-macos-x86_64.tar.gz
curl -LO https://github.com/skillsmd/skills/releases/download/v0.1.0/skills-linux-x86_64.tar.gz

# Generate checksums
shasum -a 256 skills-macos-aarch64.tar.gz
shasum -a 256 skills-macos-x86_64.tar.gz
shasum -a 256 skills-linux-x86_64.tar.gz
```

3. **Update the formula** with the actual SHA256 values.

### 4. Commit and Push

```bash
git add Formula/skills.rb
git commit -m "Add skills formula"
git push origin main
```

### 5. Test the Formula

```bash
# Audit the formula
brew audit --new-formula Formula/skills.rb

# Test installation
brew install --build-from-source Formula/skills.rb

# Test the binary
skills --version
skills --help
```

---

## Direct Download

### macOS (Apple Silicon)

```bash
# Download
curl -LO https://github.com/skillsmd/skills/releases/latest/download/skills-macos-aarch64.tar.gz

# Extract
tar xzf skills-macos-aarch64.tar.gz

# Move to PATH
sudo mv skills /usr/local/bin/

# Verify
skills --version
```

### macOS (Intel)

```bash
# Download
curl -LO https://github.com/skillsmd/skills/releases/latest/download/skills-macos-x86_64.tar.gz

# Extract
tar xzf skills-macos-x86_64.tar.gz

# Move to PATH
sudo mv skills /usr/local/bin/

# Verify
skills --version
```

### Linux

```bash
# Download
curl -LO https://github.com/skillsmd/skills/releases/latest/download/skills-linux-x86_64.tar.gz

# Extract
tar xzf skills-linux-x86_64.tar.gz

# Move to PATH
sudo mv skills /usr/local/bin/

# Verify
skills --version
```

### Linux (musl)

For Alpine Linux or static binary:

```bash
# Download
curl -LO https://github.com/skillsmd/skills/releases/latest/download/skills-linux-x86_64-musl.tar.gz

# Extract
tar xzf skills-linux-x86_64-musl.tar.gz

# Move to PATH
sudo mv skills /usr/local/bin/

# Verify
skills --version
```

### Windows

```powershell
# Download from releases page
# https://github.com/skillsmd/skills/releases/latest/download/skills-windows-x86_64.exe.zip

# Extract the ZIP file
Expand-Archive -Path skills-windows-x86_64.exe.zip -DestinationPath .

# Add to PATH (optional)
# Move skills.exe to a directory in your PATH
```

---

## From Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- Git

### Build and Install

```bash
# Clone the repository
git clone https://github.com/skillsmd/skills.git
cd skills

# Build release binary
cargo build --release

# Install (Unix-like systems)
sudo cp target/release/skills /usr/local/bin/

# Or install using cargo
cargo install --path .

# Verify
skills --version
```

### Development Build

```bash
# Clone and build
git clone https://github.com/skillsmd/skills.git
cd skills

# Run without installing
cargo run -- --help

# Build for development
cargo build

# Run tests
cargo test
```

---

## Shell Completions (Optional)

Generate shell completions for your shell:

### Bash

```bash
# Generate completion
skills --generate-completion bash > /usr/local/etc/bash_completion.d/skills

# Or for user-level
skills --generate-completion bash > ~/.bash_completion.d/skills
```

### Zsh

```bash
# Generate completion
skills --generate-completion zsh > /usr/local/share/zsh/site-functions/_skills
```

### Fish

```bash
# Generate completion
skills --generate-completion fish > ~/.config/fish/completions/skills.fish
```

---

## Verification

After installation, verify that `skills` is properly installed:

```bash
# Check version
skills --version

# Check help
skills --help

# Test installation command
skills install --help
```

---

## Troubleshooting

### Command not found

If you get "command not found" after installation:

1. **Check if the binary is in PATH**:
   ```bash
   echo $PATH
   ls -la /usr/local/bin/skills
   ```

2. **Make sure it's executable**:
   ```bash
   chmod +x /usr/local/bin/skills
   ```

3. **Restart your shell** or reload your shell configuration:
   ```bash
   source ~/.bashrc  # or ~/.zshrc
   ```

### Permission denied

If you get permission errors:

```bash
# Make the binary executable
chmod +x skills

# Or use sudo when moving to system directories
sudo mv skills /usr/local/bin/
```

### macOS "cannot be opened because the developer cannot be verified"

On macOS, you may need to allow the binary:

```bash
# Option 1: Remove quarantine attribute
xattr -d com.apple.quarantine /usr/local/bin/skills

# Option 2: Allow in System Preferences
# System Preferences → Security & Privacy → General → "Allow anyway"
```

---

## Updating

### Via Homebrew

```bash
brew update
brew upgrade skills
```

### Manual Update

Download and install the latest release following the [Direct Download](#direct-download) instructions.

### From Source

```bash
cd skills
git pull origin main
cargo build --release
sudo cp target/release/skills /usr/local/bin/
```

---

## Uninstallation

### Via Homebrew

```bash
brew uninstall skills
```

### Manual Removal

```bash
# Remove binary
sudo rm /usr/local/bin/skills

# Remove configuration (optional)
rm -rf ~/.skills
```

---

## Next Steps

After installation, check out the [README](README.md) for usage examples and documentation.
