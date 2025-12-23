# Release Process

This document describes how to create a new release of Skills CLI.

## Prerequisites

1. Ensure all tests pass: `cargo test`
2. Ensure code is formatted: `cargo fmt`
3. Ensure no clippy warnings: `cargo clippy`
4. Update version in `Cargo.toml`
5. Update `CHANGELOG.md` (if exists)

## Creating a Release

### 1. Update Version

Edit `Cargo.toml`:

```toml
[package]
name = "skills"
version = "X.Y.Z"  # Update this
```

### 2. Commit Version Bump

```bash
git add Cargo.toml
git commit -m "Bump version to vX.Y.Z"
git push origin main
```

### 3. Create and Push Tag

```bash
# Create annotated tag
git tag -a vX.Y.Z -m "Release vX.Y.Z"

# Push tag to trigger release workflow
git push origin vX.Y.Z
```

### 4. Monitor Release Workflow

1. Go to GitHub Actions: `https://github.com/YOUR_USERNAME/skillscli/actions`
2. Watch the "Release" workflow
3. Wait for all builds to complete

### 5. Edit Release Notes

1. Go to Releases: `https://github.com/YOUR_USERNAME/skillscli/releases`
2. Find the new release (draft)
3. Edit the description using `.github/RELEASE_TEMPLATE.md`
4. Add notable changes
5. Publish the release

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (X.0.0): Breaking changes
- **MINOR** (0.Y.0): New features, backward compatible
- **PATCH** (0.0.Z): Bug fixes, backward compatible

## Release Checklist

- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Version updated in `Cargo.toml`
- [ ] CHANGELOG updated (if exists)
- [ ] Changes committed and pushed
- [ ] Tag created and pushed
- [ ] GitHub Actions workflow completed successfully
- [ ] Release notes updated
- [ ] Release published
- [ ] Binaries tested on at least one platform

## Supported Platforms

The release workflow builds binaries for:

- Linux x86_64 (glibc)
- Linux x86_64 (musl, static)
- macOS x86_64 (Intel)
- macOS aarch64 (Apple Silicon)
- Windows x86_64

## Troubleshooting

### Build Fails on a Platform

1. Check the GitHub Actions logs
2. Fix the issue
3. Delete the failed tag: `git tag -d vX.Y.Z && git push origin :refs/tags/vX.Y.Z`
4. Recreate and push the tag

### Need to Update Release

1. Edit the release on GitHub
2. Update description/assets as needed
3. No need to recreate tags for documentation changes

## Post-Release

1. Announce the release (if applicable)
2. Update documentation if needed
3. Monitor for issues/feedback
