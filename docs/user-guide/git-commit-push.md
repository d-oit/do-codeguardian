# git-commit-push

## Synopsis
Perform enhanced git commit and push with integrated CodeGuardian security analysis for complete pre-deployment validation.

## Description
The git-commit-push command provides a comprehensive workflow that analyzes code, commits changes, and pushes them to the remote repository. It ensures that only secure, high-quality code reaches the remote repository.

## Syntax
```bash
codeguardian git-commit-push [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--message <MESSAGE>` | Custom commit message | `STRING` | - | No |
| `--amend` | Amend the last commit instead of creating a new one | `FLAG` | `false` | No |
| `--push` | Push to remote after committing | `FLAG` | `true` | No |
| `--no-verify` | Skip pre-commit hooks | `FLAG` | `false` | No |

## Examples
```bash
# Commit and push with analysis
codeguardian git-commit-push --message "Add user authentication"

# Amend last commit
codeguardian git-commit-push --amend --message "Fix typo in authentication"

# Skip pre-commit hooks
codeguardian git-commit-push --no-verify --message "Hotfix deployment"
```

## Workflow
1. Analyzes staged files for security and quality issues
2. If issues found, prevents commit and push
3. Commits changes with analysis approval
4. Pushes to configured remote repository
5. Provides detailed feedback on any issues found

## See Also
- [`codeguardian git-commit`](git-commit.md) - Commit only with analysis
- [`codeguardian check`](check.md) - Run standalone analysis
