# Setting Up Homebrew Tap

This guide explains how to create and maintain a Homebrew tap for the `skills` CLI.

## What is a Homebrew Tap?

A Homebrew tap is a third-party repository that allows users to install software not in Homebrew's main repository. It's the recommended way to distribute software via Homebrew.

## Quick Setup

### 1. Create the Tap Repository

Create a new GitHub repository named `homebrew-skills`:

```bash
# The name MUST start with "homebrew-"
# For example: https://github.com/skillsmd/homebrew-skills
```

### 2. Initialize the Tap

```bash
# Clone your new tap repository
git clone https://github.com/skillsmd/homebrew-skills.git
cd homebrew-skills

# Create Formula directory
mkdir -p Formula

# Copy the formula from your main repo
cp ../skills/Formula/skills.rb Formula/
```

### 3. Update the Formula

Before publishing, you need to update `Formula/skills.rb` with:

1. **Your GitHub username**:
   ```ruby
   homepage "https://github.com/skillsmd/skills"
   ```

2. **Actual SHA256 checksums** for each platform (see below)

### 4. Generate SHA256 Checksums

After creating a GitHub release with binaries:

```bash
# Download release assets
curl -LO https://github.com/skillsmd/skills/releases/download/v0.1.0/skills-macos-aarch64.tar.gz
curl -LO https://github.com/skillsmd/skills/releases/download/v0.1.0/skills-macos-x86_64.tar.gz
curl -LO https://github.com/skillsmd/skills/releases/download/v0.1.0/skills-linux-x86_64.tar.gz

# Generate checksums
shasum -a 256 *.tar.gz

# Example output:
# abc123... skills-macos-aarch64.tar.gz
# def456... skills-macos-x86_64.tar.gz
# ghi789... skills-linux-x86_64.tar.gz
```

### 5. Update Formula with Checksums

Edit `Formula/skills.rb` and replace `REPLACE_WITH_ACTUAL_SHA256` with the actual values:

```ruby
on_macos do
  if Hardware::CPU.arm?
    url "https://github.com/skillsmd/skills/releases/download/v0.1.0/skills-macos-aarch64.tar.gz"
    sha256 "abc123..."  # ← Replace this
  else
    url "https://github.com/skillsmd/skills/releases/download/v0.1.0/skills-macos-x86_64.tar.gz"
    sha256 "def456..."  # ← Replace this
  end
end

on_linux do
  url "https://github.com/skillsmd/skills/releases/download/v0.1.0/skills-linux-x86_64.tar.gz"
  sha256 "ghi789..."  # ← Replace this
end
```

### 6. Test the Formula

```bash
# Audit the formula
brew audit --new-formula Formula/skills.rb

# Test installation locally
brew install --build-from-source Formula/skills.rb

# Test the binary
skills --version
skills --help

# Uninstall test
brew uninstall skills
```

### 7. Commit and Push

```bash
git add Formula/skills.rb
git commit -m "Add skills v0.1.0 formula"
git push origin main
```

### 8. Users Can Now Install

Once pushed, users can install with:

```bash
# Add your tap
brew tap skillsmd/skills

# Install
brew install skills
```

Or directly:

```bash
brew install skillsmd/skills/skills
```

---

## Updating for New Releases

When you release a new version:

### 1. Update Version and URLs

Edit `Formula/skills.rb`:

```ruby
version "0.2.0"  # ← New version

# Update URLs
url "https://github.com/skillsmd/skills/releases/download/v0.2.0/skills-macos-aarch64.tar.gz"
```

### 2. Generate New Checksums

```bash
# Download new release assets
curl -LO https://github.com/skillsmd/skills/releases/download/v0.2.0/skills-macos-aarch64.tar.gz
curl -LO https://github.com/skillsmd/skills/releases/download/v0.2.0/skills-macos-x86_64.tar.gz
curl -LO https://github.com/skillsmd/skills/releases/download/v0.2.0/skills-linux-x86_64.tar.gz

# Generate checksums
shasum -a 256 *.tar.gz
```

### 3. Update SHA256 Values

Replace the `sha256` values in the formula with the new checksums.

### 4. Test and Push

```bash
# Test
brew audit Formula/skills.rb
brew install --build-from-source Formula/skills.rb
skills --version  # Should show 0.2.0

# Commit and push
git add Formula/skills.rb
git commit -m "Update skills to v0.2.0"
git push origin main
```

### 5. Users Update

Users can now update to the new version:

```bash
brew update
brew upgrade skills
```

---

## Automation with GitHub Actions

You can automate formula updates using GitHub Actions.

### Option 1: Manual Trigger

Create `.github/workflows/update-formula.yml` in your tap repository:

```yaml
name: Update Formula

on:
  workflow_dispatch:
    inputs:
      version:
        description: 'Version to update to (e.g., 0.2.0)'
        required: true
      macos_arm_sha:
        description: 'SHA256 for macOS ARM64'
        required: true
      macos_x64_sha:
        description: 'SHA256 for macOS x86_64'
        required: true
      linux_sha:
        description: 'SHA256 for Linux x86_64'
        required: true

jobs:
  update:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Update formula
        run: |
          sed -i 's/version ".*"/version "${{ github.event.inputs.version }}"/' Formula/skills.rb
          sed -i 's|download/v.*/skills-macos-aarch64|download/v${{ github.event.inputs.version }}/skills-macos-aarch64|' Formula/skills.rb
          sed -i 's|download/v.*/skills-macos-x86_64|download/v${{ github.event.inputs.version }}/skills-macos-x86_64|' Formula/skills.rb
          sed -i 's|download/v.*/skills-linux-x86_64|download/v${{ github.event.inputs.version }}/skills-linux-x86_64|' Formula/skills.rb
          # Update SHA256 values (simplified example)

      - name: Commit changes
        run: |
          git config user.name "GitHub Actions"
          git config user.email "actions@github.com"
          git add Formula/skills.rb
          git commit -m "Update skills to v${{ github.event.inputs.version }}"
          git push
```

### Option 2: Automatic Update Script

Create a script in your main repository to update the tap:

```bash
#!/bin/bash
# scripts/update-homebrew-tap.sh

VERSION=$1

# Download release assets
curl -LO "https://github.com/skillsmd/skills/releases/download/v${VERSION}/skills-macos-aarch64.tar.gz"
curl -LO "https://github.com/skillsmd/skills/releases/download/v${VERSION}/skills-macos-x86_64.tar.gz"
curl -LO "https://github.com/skillsmd/skills/releases/download/v${VERSION}/skills-linux-x86_64.tar.gz"

# Generate checksums
MACOS_ARM_SHA=$(shasum -a 256 skills-macos-aarch64.tar.gz | cut -d' ' -f1)
MACOS_X64_SHA=$(shasum -a 256 skills-macos-x86_64.tar.gz | cut -d' ' -f1)
LINUX_SHA=$(shasum -a 256 skills-linux-x86_64.tar.gz | cut -d' ' -f1)

echo "macOS ARM64: $MACOS_ARM_SHA"
echo "macOS x86_64: $MACOS_X64_SHA"
echo "Linux x86_64: $LINUX_SHA"

# Clone tap repository
git clone https://github.com/skillsmd/homebrew-skills.git tap
cd tap

# Update formula
sed -i '' "s/version \".*\"/version \"${VERSION}\"/" Formula/skills.rb
sed -i '' "s|download/v.*/skills-macos-aarch64|download/v${VERSION}/skills-macos-aarch64|" Formula/skills.rb
sed -i '' "s|download/v.*/skills-macos-x86_64|download/v${VERSION}/skills-macos-x86_64|" Formula/skills.rb
sed -i '' "s|download/v.*/skills-linux-x86_64|download/v${VERSION}/skills-linux-x86_64|" Formula/skills.rb

# Update SHA256 values (you'll need to do this manually or with more sophisticated sed)

# Commit and push
git add Formula/skills.rb
git commit -m "Update skills to v${VERSION}"
git push origin main

cd ..
rm -rf tap
```

---

## Best Practices

1. **Version Numbers**: Always use semantic versioning (e.g., 0.1.0, 1.0.0)
2. **Testing**: Always test the formula before pushing to the tap
3. **Checksums**: Never commit without valid SHA256 checksums
4. **Documentation**: Keep this guide updated with your actual repository names
5. **Automation**: Consider automating updates for faster releases

---

## Troubleshooting

### Formula fails audit

```bash
brew audit --new-formula Formula/skills.rb
# Fix any issues reported
```

### Binary doesn't work after installation

```bash
# Check if binary is executable
ls -la $(brew --prefix)/bin/skills

# Check architecture
file $(brew --prefix)/bin/skills

# Test manually
$(brew --prefix)/bin/skills --version
```

### SHA256 mismatch

```bash
# Regenerate checksums
curl -LO <url>
shasum -a 256 <file>.tar.gz
# Update formula with correct value
```

---

## Resources

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [How to Create and Maintain a Tap](https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap)
- [Homebrew Formula Reference](https://rubydoc.brew.sh/Formula)
