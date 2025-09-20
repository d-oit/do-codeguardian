# CodeGuardian Release Management System

This directory contains scripts and workflows for managing CodeGuardian releases, including version bumping, changelog generation, release notes creation, and deployment coordination.

## 📁 Files Overview

- `release-manager.sh` - Main release orchestration script
- `version-bump.sh` - Automated version bumping script
- `generate-changelog.sh` - Changelog generation from git commits
- `generate-release-notes.sh` - Release notes generation from PRs
- `README.md` - This documentation file

## 🚀 Quick Start

### Automated Release (Recommended)

Use the GitHub Actions workflow for fully automated releases:

1. Go to GitHub Actions → "🚀 Automated Release"
2. Click "Run workflow"
3. Choose bump type (patch/minor/major) or specify custom version
4. Optionally enable dry-run mode
5. Click "Run workflow"

### Manual Release

For manual releases, use the main release script:

```bash
# Patch release
./scripts/release/release-manager.sh --patch

# Minor release
./scripts/release/release-manager.sh --minor

# Major release
./scripts/release/release-manager.sh --major

# Custom version
./scripts/release/release-manager.sh --version 1.2.3

# Dry run
./scripts/release/release-manager.sh --patch --dry-run
```

## 📋 Detailed Usage

### Version Bumping

```bash
# Bump patch version
./scripts/release/version-bump.sh --patch --commit --tag

# Bump minor version
./scripts/release/version-bump.sh --minor --commit --tag

# Set specific version
./scripts/release/version-bump.sh --version 2.0.0 --commit --tag
```

### Changelog Generation

```bash
# Generate changelog for version 1.2.3 since tag v1.2.2
./scripts/release/generate-changelog.sh --version 1.2.3 --since v1.2.2

# Generate changelog with custom date
./scripts/release/generate-changelog.sh --version 1.2.3 --date 2024-01-15
```

### Release Notes Generation

```bash
# Generate release notes from PRs
./scripts/release/generate-release-notes.sh \
  --version 1.2.3 \
  --since v1.2.2 \
  --until HEAD \
  --output release-notes.md
```

## 🔧 Configuration

### Environment Variables

- `GITHUB_REPO` - GitHub repository (default: d-oit/do-codeguardian)
- `CRATES_IO_TOKEN` - Token for publishing to crates.io (GitHub secret)

### GitHub Secrets Required

For automated releases, set these secrets in your GitHub repository:

- `CRATES_IO_TOKEN` - API token for crates.io publication
- `GITHUB_TOKEN` - Automatically provided by GitHub Actions

## 📊 Release Process

The automated release process follows these steps:

1. **Pre-release Validation**
   - Run tests, clippy, and formatting checks
   - Validate version and release requirements

2. **Build Release Artifacts**
   - Build for multiple platforms (Linux, macOS, Windows)
   - Create optimized release binaries

3. **Generate Release Content**
   - Update CHANGELOG.md with new entries
   - Generate detailed release notes from PRs

4. **Publish to Crates.io**
   - Publish the crate to crates.io
   - Handle authentication and validation

5. **Create GitHub Release**
   - Create GitHub release with artifacts
   - Upload release binaries for all platforms

6. **Post-release Cleanup**
   - Generate deployment reports
   - Send notifications

## 🌍 Deployment

### Staging Deployment

Triggered automatically for all releases or manually via workflow dispatch:

```bash
# Via GitHub Actions
- Go to "🚀 Deployment Pipeline"
- Choose environment: staging
- Optionally specify version
```

### Production Deployment

Triggered automatically for GitHub releases or manually:

```bash
# Via GitHub Actions
- Go to "🚀 Deployment Pipeline"
- Choose environment: production
- Optionally specify version
```

## 🐛 Troubleshooting

### Common Issues

1. **Version bump fails**
   - Ensure working directory is clean
   - Check that Cargo.toml exists and is valid

2. **Changelog generation fails**
   - Ensure git history is available
   - Check conventional commit format in recent commits

3. **Release notes generation fails**
   - Install GitHub CLI: `gh auth login`
   - Ensure repository access permissions

4. **Crates.io publication fails**
   - Check `CRATES_IO_TOKEN` secret
   - Verify crate ownership on crates.io

### Debug Mode

Run scripts with verbose output:

```bash
# Enable debug logging
export DEBUG=1

# Run with verbose output
bash -x ./scripts/release/release-manager.sh --patch --dry-run
```

## 📝 Conventional Commits

For best results, use conventional commit format:

```bash
# Features
feat: add new security scanner

# Bug fixes
fix: resolve memory leak in analyzer

# Documentation
docs: update installation guide

# Performance
perf: optimize file processing speed

# Maintenance
chore: update dependencies
refactor: restructure core modules
```

## 🔗 Integration

### CI/CD Integration

The release system integrates with:

- **GitHub Actions** - Automated workflows
- **GitHub Releases** - Release management
- **Crates.io** - Rust package registry
- **Docker** - Container deployment
- **Kubernetes** - Orchestration (optional)

### Notification Integration

Add webhooks or integrations for:

- Slack notifications
- Discord notifications
- Email notifications
- Jira integration
- Monitoring alerts

## 📈 Monitoring

Monitor release health with:

- Release success/failure rates
- Deployment time metrics
- Artifact download statistics
- User feedback and issue tracking

## 🤝 Contributing

When contributing to the release system:

1. Test changes in dry-run mode first
2. Update documentation for new features
3. Follow existing code style and patterns
4. Add appropriate error handling

## 📞 Support

For issues with the release system:

1. Check the troubleshooting section
2. Review GitHub Actions logs
3. Create an issue with detailed error information
4. Include version and environment details
