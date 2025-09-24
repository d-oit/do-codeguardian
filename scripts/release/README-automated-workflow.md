# Automated Release Workflow Script

The `automated-release-workflow.sh` script provides a comprehensive, automated release process for CodeGuardian that combines version detection, changelog generation, and release creation with automatic semantic versioning based on commit analysis.

## Features

- **Automatic Semantic Versioning**: Analyzes commit messages to determine appropriate version bumps (major, minor, patch)
- **Conventional Commits Support**: Recognizes breaking changes, features, and fixes
- **Comprehensive Validation**: Runs tests, clippy, and formatting checks
- **Release Artifact Building**: Creates optimized release binaries
- **Changelog Generation**: Automatically generates structured changelogs
- **Release Notes Creation**: Generates detailed release notes from PRs and commits
- **GitHub Integration**: Creates GitHub releases with proper tagging
- **Crates.io Publishing**: Publishes to Rust's package registry
- **Dry Run Support**: Test the entire workflow without making changes
- **CI/CD Ready**: Designed to work seamlessly with existing GitHub Actions

## Prerequisites

- Git repository with clean working directory
- Rust toolchain installed
- GitHub CLI (`gh`) for GitHub operations (optional)
- Cargo credentials for crates.io publishing (optional)

## Usage

### Basic Usage (Automatic Version Detection)

```bash
# Auto-detect version bump from commits
./scripts/release/automated-release-workflow.sh

# Force specific version bump
./scripts/release/automated-release-workflow.sh --minor
./scripts/release/automated-release-workflow.sh --major
./scripts/release/automated-release-workflow.sh --patch

# Set custom version
./scripts/release/automated-release-workflow.sh --version 1.2.3
```

### Advanced Options

```bash
# Analyze commits since specific reference
./scripts/release/automated-release-workflow.sh --since v1.0.0

# Dry run mode (no actual changes)
./scripts/release/automated-release-workflow.sh --dry-run

# Skip specific steps
./scripts/release/automated-release-workflow.sh --skip-validation
./scripts/release/automated-release-workflow.sh --skip-build
./scripts/release/automated-release-workflow.sh --skip-publish
./scripts/release/automated-release-workflow.sh --skip-github-release

# Combine options
./scripts/release/automated-release-workflow.sh --minor --dry-run --skip-publish
```

## How Version Detection Works

The script analyzes commit messages between the current HEAD and the latest tag (or a specified reference) to determine the appropriate version bump:

### Commit Analysis Rules

1. **Breaking Changes** → Major version bump
   - Commits with `!` (e.g., `feat!: breaking change`)
   - Commits containing `BREAKING CHANGE:`

2. **New Features** → Minor version bump
   - Commits starting with `feat` or `feature`
   - Commits mentioning "add" or "new"

3. **Fixes/Default** → Patch version bump
   - Commits starting with `fix` or `bug`
   - All other commits default to patch

### Examples

```bash
# These commits would trigger a major version bump:
feat!: add new authentication system
fix: security vulnerability (BREAKING CHANGE: changes API)

# These would trigger a minor version bump:
feat: add user profile management
feature: implement dark mode

# These would trigger a patch version bump:
fix: resolve memory leak
chore: update dependencies
```

## Workflow Steps

The script executes the following steps in order:

1. **Prerequisites Validation**
   - Check git repository status
   - Verify required scripts exist
   - Ensure clean working directory

2. **Commit Analysis** (if auto-detecting version)
   - Analyze commits since last tag
   - Determine version bump type

3. **Validation Suite**
   - Run `cargo check`
   - Execute test suite
   - Run clippy checks
   - Verify code formatting

4. **Release Artifact Building**
   - Build optimized release binary
   - Create versioned artifact

5. **Version Management**
   - Update `Cargo.toml` version
   - Create version bump commit

6. **Documentation Generation**
   - Generate structured changelog
   - Create detailed release notes

7. **Git Operations**
   - Create annotated git tag
   - Push commits and tags

8. **Publishing**
   - Publish to crates.io (if enabled)
   - Create GitHub release (if enabled)

## Configuration

The script uses these configuration variables (can be overridden):

```bash
# Repository settings
GIT_REPO_ROOT="$(git rev-parse --show-toplevel)"
GITHUB_REPO="${GITHUB_REPO:-d-oit/do-codeguardian}"

# File paths
CARGO_TOML="$GIT_REPO_ROOT/Cargo.toml"
CHANGELOG_MD="$GIT_REPO_ROOT/CHANGELOG.md"
```

## Integration with CI/CD

### GitHub Actions Integration

The script is designed to work with the existing `.github/workflows/automated-release.yml` workflow. It can be triggered:

- Manually via workflow dispatch
- Automatically on tag pushes
- As part of automated release pipelines

### Example GitHub Actions Step

```yaml
- name: Run Automated Release
  run: |
    ./scripts/release/automated-release-workflow.sh \
      --version ${{ inputs.custom_version }} \
      --skip-github-release  # Let the workflow handle GitHub release
```

## Output and Artifacts

The script generates:

- **Release Binary**: `artifacts/codeguardian-{version}`
- **Changelog**: Updated `CHANGELOG.md`
- **Release Notes**: Temporary file for GitHub release
- **Git Tag**: `v{version}` with annotation
- **Crates.io Release**: Published package (if enabled)

## Error Handling

The script includes comprehensive error handling:

- Validates prerequisites before starting
- Stops on first error (set -e)
- Provides clear error messages
- Supports dry-run mode for testing
- Cleans up temporary files on exit

## Security Considerations

- Requires clean git working directory
- Validates version format
- Uses secure git operations
- Supports dry-run for safe testing
- No hardcoded secrets

## Troubleshooting

### Common Issues

1. **Dirty Working Directory**
   ```
   Error: Working directory is not clean
   Solution: Commit or stash changes first
   ```

2. **Missing Dependencies**
   ```
   Error: Required tool not found: gh
   Solution: Install GitHub CLI or use --skip-github-release
   ```

3. **Version Conflicts**
   ```
   Error: Invalid version format
   Solution: Use semantic versioning (major.minor.patch)
   ```

### Debug Mode

Run with verbose output:

```bash
bash -x ./scripts/release/automated-release-workflow.sh --dry-run
```

## Examples

### Development Release

```bash
# Test the workflow without making changes
./scripts/release/automated-release-workflow.sh --dry-run

# Create a patch release
./scripts/release/automated-release-workflow.sh --patch

# Release a new feature
./scripts/release/automated-release-workflow.sh --minor
```

### Production Release

```bash
# Full automated release
./scripts/release/automated-release-workflow.sh

# Custom version release
./scripts/release/automated-release-workflow.sh --version 2.0.0

# Skip publishing for testing
./scripts/release/automated-release-workflow.sh --skip-publish
```

## Related Scripts

- `release-manager.sh`: Manual release orchestration
- `version-bump.sh`: Version management utilities
- `generate-changelog.sh`: Changelog generation
- `generate-release-notes.sh`: Release notes from PRs

## Contributing

When modifying this script:

1. Maintain backward compatibility
2. Update this documentation
3. Test with `--dry-run` mode
4. Follow the existing error handling patterns
5. Add appropriate logging for new features