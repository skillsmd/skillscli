# Quick Homebrew Setup Guide

This is a quick reference for setting up Homebrew distribution for `skills` CLI.

## Repository Information

- **Main Repository**: https://github.com/skillsmd/skillscli
- **Tap Repository** (to create): https://github.com/skillsmd/homebrew-skills

## Step-by-Step Setup

### 1. Create Tap Repository

Go to GitHub and create a new repository:
- Name: `homebrew-skills` (MUST start with `homebrew-`)
- Description: "Homebrew tap for skills CLI"
- Public repository

### 2. Initialize Tap

```bash
# Clone the tap repository
git clone https://github.com/skillsmd/homebrew-skills.git
cd homebrew-skills

# Create Formula directory
mkdir -p Formula

# Copy formula from main repo
cp ../skillscli/Formula/skills.rb Formula/

# Initial commit
git add Formula/skills.rb
git commit -m "Add skills formula (with placeholder checksums)"
git push origin main
```

### 3. Create a Release

In your main skillscli repository:

```bash
# Tag version
git tag v0.1.0
git push origin v0.1.0

# This will trigger GitHub Actions to build binaries for:
# - macOS ARM64 (Apple Silicon)
# - macOS x86_64 (Intel)
# - Linux x86_64
# - Linux x86_64-musl
# - Windows x86_64
```

### 4. Generate SHA256 Checksums

After the release builds complete (check: https://github.com/skillsmd/skillscli/releases):

```bash
# Download the release assets
curl -LO https://github.com/skillsmd/skillscli/releases/download/v0.1.0/skills-macos-aarch64.tar.gz
curl -LO https://github.com/skillsmd/skillscli/releases/download/v0.1.0/skills-macos-x86_64.tar.gz
curl -LO https://github.com/skillsmd/skillscli/releases/download/v0.1.0/skills-linux-x86_64.tar.gz

# Generate checksums
shasum -a 256 *.tar.gz

# Output will look like:
# a1b2c3d4... skills-macos-aarch64.tar.gz
# e5f6g7h8... skills-macos-x86_64.tar.gz
# i9j0k1l2... skills-linux-x86_64.tar.gz
```

### 5. Update Formula with Real Checksums

Edit `homebrew-skills/Formula/skills.rb` and replace the `REPLACE_WITH_ACTUAL_SHA256` placeholders:

```ruby
class Skills < Formula
  desc "A CLI for managing skills for AI coding assistants"
  homepage "https://github.com/skillsmd/skillscli"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/skillsmd/skillscli/releases/download/v0.1.0/skills-macos-aarch64.tar.gz"
      sha256 "a1b2c3d4..."  # ← Paste actual checksum here
    else
      url "https://github.com/skillsmd/skillscli/releases/download/v0.1.0/skills-macos-x86_64.tar.gz"
      sha256 "e5f6g7h8..."  # ← Paste actual checksum here
    end
  end

  on_linux do
    url "https://github.com/skillsmd/skillscli/releases/download/v0.1.0/skills-linux-x86_64.tar.gz"
    sha256 "i9j0k1l2..."  # ← Paste actual checksum here
  end

  def install
    bin.install "skills"
  end

  test do
    system "#{bin}/skills", "--version"
    system "#{bin}/skills", "--help"
  end
end
```

### 6. Test Locally

```bash
# In homebrew-skills directory
brew audit --new-formula Formula/skills.rb

# Test installation
brew install --build-from-source Formula/skills.rb

# Verify it works
skills --version
skills --help

# Uninstall test version
brew uninstall skills
```

### 7. Commit and Push

```bash
cd homebrew-skills
git add Formula/skills.rb
git commit -m "Update skills formula with real checksums for v0.1.0"
git push origin main
```

### 8. Done! Users Can Now Install

```bash
# Users can now install with:
brew tap skillsmd/skills
brew install skills

# Or directly:
brew install skillsmd/skills/skills
```

## User Installation Commands

Once the tap is set up, users install with:

```bash
# Method 1: Via tap (recommended)
brew tap skillsmd/skills
brew install skills

# Method 2: Direct formula URL
brew install https://raw.githubusercontent.com/skillsmd/homebrew-skills/main/Formula/skills.rb

# Verify installation
skills --version
```

## Updating for New Releases

When you release v0.2.0 (or any new version):

1. **Create release**: `git tag v0.2.0 && git push origin v0.2.0`
2. **Download new binaries**: Update URLs with new version
3. **Generate new checksums**: Run `shasum -a 256 *.tar.gz`
4. **Update formula**: Change version and sha256 values
5. **Test**: `brew audit && brew install --build-from-source`
6. **Push**: `git commit -m "Update to v0.2.0" && git push`

Users update with:
```bash
brew update
brew upgrade skills
```

## Quick Links

- Main repo: https://github.com/skillsmd/skillscli
- Releases: https://github.com/skillsmd/skillscli/releases
- Tap repo: https://github.com/skillsmd/homebrew-skills (create this)
- Formula: https://github.com/skillsmd/homebrew-skills/blob/main/Formula/skills.rb

## Support

For detailed instructions, see:
- [HOMEBREW_TAP.md](HOMEBREW_TAP.md) - Complete guide
- [INSTALLATION.md](INSTALLATION.md) - User installation guide
