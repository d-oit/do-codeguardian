---
description: Create a new git/github release with full validation
agent: release-agent
---

# Release Command

Automates the complete release process using git and GitHub CLI, ensuring code quality through CI/CD workflows before creating releases.

## Process Overview

1. **Pre-flight Checks**: Validate git status and branch
2. **Trigger CI/CD**: Push changes to trigger GitHub Actions workflows
3. **Monitor Workflows**: Watch lint, test, and build workflows for completion
4. **Verify Documentation**: Check docs and embedded commands
5. **Create Release**: Generate changelog and create GitHub release

## Usage

```bash
/release [version] [--alpha] [--dry-run]
```

## Arguments

- `version`: Optional version number (auto-increments if not provided)
- `--alpha`: Create alpha pre-release
- `--dry-run`: Show what would be done without making changes

## Workflow Steps

### 1. Git Validation
Ensure clean working directory and correct branch:
```bash
!`git status --porcelain`
!`git branch --show-current`
```

### 2. Version Determination
Determine next version based on git tags:
```bash
!`git tag --sort=-version:refname | head -1`
```

### 3. Push and Trigger CI/CD
Push changes to trigger GitHub Actions:
```bash
!`git push origin main`
```

### 4. Monitor Workflows
Watch GitHub Actions for completion:
```bash
!`gh run list --limit 1`
!`gh run watch $(gh run list --limit 1 --json databaseId -q '.[0].databaseId')`
```

### 5. Verify Documentation
Check all command files in docs for validity:
```bash
!`find docs/ .opencode/command/ -name "*.md" -exec grep -l "```" {} \; | head -5`
```

### 6. Generate Changelog
Create changelog from recent commits:
```bash
!`git log --oneline --since="1 month ago" | head -10`
```

### 7. Create GitHub Release
Create the release with generated notes:
```bash
!`gh release create $VERSION --generate-notes ${ALPHA_FLAG}`
```

## Configuration

Uses `.opencode/config.json` for release settings:
```json
{
  "release": {
    "defaultBranch": "main",
    "requireCleanGit": true,
    "autoIncrement": true,
    "alphaPrefix": "alpha",
    "stablePrefix": "v"
  }
}
```

## Examples

### Stable Release
```bash
/release
```

### Alpha Release
```bash
/release --alpha
```

### Specific Version
```bash
/release v1.2.3
```

### Dry Run
```bash
/release --dry-run
```

## Error Handling

- **Git Errors**: Validates repository state before proceeding
- **Workflow Failures**: Monitors CI/CD status and provides rerun options
- **Release Conflicts**: Checks for existing tags/releases
- **Permission Issues**: Verifies GitHub CLI authentication

## Security

- Uses GitHub CLI authentication
- Validates all operations before execution
- Provides rollback options for failed releases
- Logs all actions for audit trail

## Troubleshooting

### Common Issues

1. **Dirty Working Directory**: Commit or stash changes before release
2. **Workflow Timeouts**: Check GitHub Actions status and rerun if needed
3. **Authentication Errors**: Run `gh auth login` to authenticate
4. **Branch Mismatches**: Ensure you're on the correct release branch

### Debug Information

Use `--dry-run` to see all planned operations without executing them.