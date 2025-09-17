# git-commit

## Synopsis
Perform enhanced git commit with integrated CodeGuardian security analysis to ensure code quality before committing changes.

## Description
The git-commit command combines standard git commit functionality with CodeGuardian's security and code quality analysis. It automatically analyzes staged changes and prevents commits that would introduce security issues or code quality problems.

## Syntax
```bash
codeguardian git-commit [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--message <MESSAGE>` | Custom commit message | `STRING` | - | No |

## Examples
```bash
# Commit with analysis (will prompt for message)
codeguardian git-commit

# Commit with custom message
codeguardian git-commit --message "Fix security vulnerability in authentication"

# Interactive commit message
codeguardian git-commit
```

## Workflow
1. Analyzes all staged files for security and quality issues
2. If issues are found, displays them and prevents commit
3. If no issues, proceeds with normal git commit
4. Supports all standard git commit options

## See Also
- [`codeguardian git-commit-push`](git-commit-push.md) - Commit and push with analysis
- [`codeguardian check`](check.md) - Run standalone analysis
