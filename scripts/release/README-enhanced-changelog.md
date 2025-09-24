# Enhanced Changelog Generator

This document describes the enhanced changelog generation script for CodeGuardian, which provides advanced features for generating comprehensive and well-formatted changelogs from git commits.

## Overview

The `generate-changelog-enhanced.sh` script builds upon the original changelog generator with the following enhancements:

- **Conventional Commits Support**: Full support for conventional commit format with breaking change detection
- **Advanced Categorization**: Breaking changes, deprecations, removals, performance improvements, and more
- **Commit Scopes**: Support for scoped commits (e.g., `feat(api): add new endpoint`)
- **PR Integration**: Automatic PR number detection from commit messages and git notes
- **Author Information**: Optional inclusion of author names and emails
- **Multiple Output Formats**: Support for both Markdown and JSON output
- **Configuration File**: TOML-based configuration for customization

## Features

### Breaking Change Detection

The script automatically detects breaking changes through:

1. **Exclamation Mark**: `feat(api)!: breaking change description`
2. **BREAKING CHANGE Footer**: Commits with `BREAKING CHANGE:` in the body
3. **Breaking Keywords**: Detection of breaking terms in commit descriptions

### Enhanced Categorization

Beyond the standard conventional commit types, the script recognizes:

- **Breaking Changes**: ⚠️ Marked with warning emoji
- **Security Fixes**: Dedicated security section
- **Deprecations**: Features marked as deprecated
- **Removals**: Features that have been removed
- **Performance**: Performance-related improvements
- **Documentation**: Documentation updates
- **Testing**: Test-related changes
- **CI/CD**: Build and deployment changes

### Commit Scopes

Scoped commits are properly formatted:

```markdown
### Added
- **api**: add new authentication endpoint (abc123) - John Doe
- **ui**: improve button styling (def456) - Jane Smith
```

### PR Information

When available, PR numbers are automatically included:

```markdown
- add new feature ([#123](abc123)) - John Doe
```

### Author Information

Optional author information can be included:

```bash
./generate-changelog-enhanced.sh --version 1.2.3 --include-authors
```

## Usage

### Basic Usage

```bash
# Generate changelog for version 1.2.3 since last tag
./generate-changelog-enhanced.sh --version 1.2.3

# Generate changelog since specific tag
./generate-changelog-enhanced.sh --version 1.2.3 --since v1.2.2

# Generate changelog for specific date
./generate-changelog-enhanced.sh --version 1.2.3 --date 2024-01-15
```

### Advanced Options

```bash
# Include author information
./generate-changelog-enhanced.sh --version 1.2.3 --include-authors

# Include PR information
./generate-changelog-enhanced.sh --version 1.2.3 --include-pr-info

# Generate JSON output
./generate-changelog-enhanced.sh --version 1.2.3 --output-format json

# Use custom configuration file
./generate-changelog-enhanced.sh --version 1.2.3 --config my-config.toml

# Enable verbose logging
./generate-changelog-enhanced.sh --version 1.2.3 --verbose

# Dry run (preview only)
./generate-changelog-enhanced.sh --version 1.2.3 --dry-run
```

### Configuration File

Create a `changelog-config.toml` file in the `scripts/release/` directory:

```toml
# Output format: "markdown" or "json"
output_format = "markdown"

# Include author information
include_authors = false

# Include PR information
include_pr_info = false

# Enable verbose logging
verbose = false

# Custom categorization rules (future enhancement)
[categorization]
security_keywords = ["security", "vulnerability", "cve"]
performance_keywords = ["performance", "optimize", "speed"]
deprecation_keywords = ["deprecat", "obsolete"]
removal_keywords = ["remov", "delet", "drop"]

# Custom section names
[sections]
breaking_changes = "⚠️ Breaking Changes"
features = "Added"
bug_fixes = "Fixed"
security = "Security"
performance = "Performance"
deprecated = "Deprecated"
removed = "Removed"
```

## Conventional Commit Examples

The script supports the full conventional commit specification:

### Basic Types
- `feat: add new feature`
- `fix: resolve bug`
- `docs: update documentation`
- `style: format code`
- `refactor: restructure code`
- `perf: improve performance`
- `test: add tests`
- `chore: update dependencies`

### Scoped Commits
- `feat(api): add new endpoint`
- `fix(ui): resolve button issue`
- `refactor(core): optimize algorithm`

### Breaking Changes
- `feat(api)!: change authentication method`
- `fix!: remove deprecated function`

  ```text
  fix: update API response format

  BREAKING CHANGE: The response format has changed from XML to JSON
  ```

### Special Sections
- `security: fix authentication vulnerability`
- `perf: optimize database queries`

## Output Formats

### Markdown (Default)

```markdown
## [1.2.3] - 2024-01-15

### ⚠️ Breaking Changes
- **api**: change authentication method (abc123) - John Doe

### Added
- **ui**: add new dashboard component (def456) - Jane Smith
- implement user preferences ([#123](ghi789)) - Bob Johnson

### Fixed
- resolve memory leak in cache (jkl012) - Alice Brown

### Security
- fix SQL injection vulnerability (mno345) - Security Team

### Performance
- optimize database queries (pqr678) - Performance Team

### Deprecated
- mark old API endpoints as deprecated (stu901) - API Team

### Removed
- remove unused legacy code (vwx234) - Cleanup Team
```

### JSON

```json
{
  "version": "1.2.3",
  "date": "2024-01-15",
  "changes": [
    {
      "type": "feat",
      "scope": "api",
      "breaking": true,
      "description": "change authentication method",
      "hash": "abc123",
      "date": "2024-01-14",
      "author": "John Doe",
      "email": "john@example.com",
      "pr": "123",
      "section": "Breaking Changes",
      "category": "Features"
    }
  ]
}
```

## Integration with Release Workflow

The enhanced changelog generator integrates seamlessly with the existing release workflow:

```bash
# In automated release script
CHANGELOG_SCRIPT="scripts/release/generate-changelog-enhanced.sh"
VERSION="1.2.3"

# Generate changelog with all enhancements
"$CHANGELOG_SCRIPT" --version "$VERSION" --include-authors --include-pr-info

# Or use configuration file
"$CHANGELOG_SCRIPT" --version "$VERSION" --config release-config.toml
```

## Backward Compatibility

The enhanced script maintains backward compatibility with the original interface:

- Same basic command-line arguments
- Same output file location (`CHANGELOG.md`)
- Same confirmation prompts
- Existing scripts continue to work

## Migration Guide

To migrate from the original script:

1. **Replace the script call**:
   ```diff
   - ./scripts/release/generate-changelog.sh --version 1.2.3
   + ./scripts/release/generate-changelog-enhanced.sh --version 1.2.3
   ```

2. **Add configuration file** (optional):
   ```bash
   cp scripts/release/changelog-config.toml.example scripts/release/changelog-config.toml
   # Edit configuration as needed
   ```

3. **Update automation scripts**:
   ```diff
   - CHANGELOG_SCRIPT="$SCRIPT_DIR/generate-changelog.sh"
   + CHANGELOG_SCRIPT="$SCRIPT_DIR/generate-changelog-enhanced.sh"
   ```

## Troubleshooting

### Common Issues

1. **No commits found**: Ensure you're on the correct branch and the tag exists
2. **PR information not showing**: Check that commit messages contain PR references or git notes are configured
3. **Configuration not loading**: Verify the TOML file syntax and path

### Debug Mode

Enable verbose logging to troubleshoot issues:

```bash
./generate-changelog-enhanced.sh --version 1.2.3 --verbose
```

### Testing

Test the script without modifying files:

```bash
./generate-changelog-enhanced.sh --version 1.2.3 --dry-run
```

## Contributing

When contributing to the changelog generator:

1. Maintain backward compatibility
2. Add tests for new features
3. Update this documentation
4. Follow the existing code style
5. Test with various commit formats

## Future Enhancements

Planned improvements:

- Custom categorization rules via configuration
- Integration with GitHub API for richer PR information
- Support for multiple changelog files
- Template customization
- Release notes generation
- Change impact analysis