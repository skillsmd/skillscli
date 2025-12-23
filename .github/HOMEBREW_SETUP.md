# Homebrew Formula Auto-Update Setup

This repository includes a GitHub Actions workflow that automatically updates the Homebrew formula with correct checksums when a new release is published.

## Prerequisites

You need to create a Personal Access Token (PAT) with write access to the `skillsmd/homebrew-skills` repository.

## Setup Instructions

### 1. Create a Personal Access Token

1. Go to GitHub Settings → Developer settings → Personal access tokens → [Tokens (classic)](https://github.com/settings/tokens)
2. Click "Generate new token" → "Generate new token (classic)"
3. Give it a descriptive name like "Homebrew Formula Update Token"
4. Set expiration (recommended: 90 days or 1 year)
5. Select the following scopes:
   - `repo` (Full control of private repositories)
     - This includes: `repo:status`, `repo_deployment`, `public_repo`, `repo:invite`, `security_events`
6. Click "Generate token"
7. **Copy the token immediately** (you won't be able to see it again)

### 2. Add Token to Repository Secrets

1. Go to this repository's Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Name: `HOMEBREW_TAP_TOKEN`
4. Value: Paste the Personal Access Token from step 1
5. Click "Add secret"

## How It Works

The workflow ([.github/workflows/update-homebrew.yml](.github/workflows/update-homebrew.yml)) is triggered automatically when:
- A new release is published on GitHub
- Manually via workflow_dispatch (for testing or re-running)

The workflow will:
1. Download the release assets for all platforms (macOS ARM64, macOS x86_64, Linux x86_64)
2. Calculate SHA256 checksums for each asset
3. Clone the `skillsmd/homebrew-skills` repository
4. Update the `Formula/skills.rb` file with:
   - New version number
   - Updated download URLs
   - Correct SHA256 checksums for each platform
5. Commit and push the changes to the homebrew-skills repository

## Manual Trigger

You can manually trigger the workflow from the Actions tab:

1. Go to Actions → Update Homebrew Formula
2. Click "Run workflow"
3. Enter the version number (e.g., `0.1.2`)
4. Click "Run workflow"

## Testing

Before creating a real release, you can test the workflow using the manual trigger with an existing release version.

## Troubleshooting

### Error: "Resource not accessible by integration"

This means the `HOMEBREW_TAP_TOKEN` secret is missing or invalid. Follow the setup instructions above.

### Error: "Failed to download release assets"

This means the release assets haven't been uploaded yet. Ensure the release workflow completes before the homebrew update workflow runs.

### Error: "Permission denied"

The Personal Access Token doesn't have write access to the homebrew-skills repository. Ensure:
- The token has `repo` scope
- The token owner has write access to `skillsmd/homebrew-skills`

## Release Process

When creating a new release:

1. Update version in `Cargo.toml`
2. Commit the version change
3. Create and push a new tag: `git tag v0.1.X && git push origin v0.1.X`
4. The release workflow will build and upload assets
5. The homebrew update workflow will automatically update the formula
6. Verify the formula was updated at https://github.com/skillsmd/homebrew-skills/blob/main/Formula/skills.rb
