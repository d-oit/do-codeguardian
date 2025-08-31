---
description: Commit staged changes and push to remote repository with security analysis and validation
---

# git-commit-push

## Synopsis
CodeGuardian's enhanced git commit and push command that performs comprehensive security analysis, generates intelligent commit messages, validates code quality, commits changes, and pushes to the remote repository in a single operation.

## Description
The git-commit-push command combines the functionality of committing changes with pushing them to the remote repository, while incorporating CodeGuardian's security analysis and quality validation features. This command ensures that only secure, high-quality code is committed and pushed to the repository.

Key capabilities include:
- **Security Analysis**: Scans staged changes for security vulnerabilities before committing
- **Intelligent Commit Messages**: Automatically generates meaningful commit messages based on changes
- **Quality Validation**: Ensures code meets quality standards before pushing
- **Atomic Operations**: Commits and pushes in a coordinated manner to maintain repository consistency
- **Remote Validation**: Verifies remote repository accessibility and permissions
- **Conflict Prevention**: Checks for potential merge conflicts before pushing
- **GitHub Integration**: Provides additional context and validation when pushing to GitHub repositories

## Syntax
```bash
opencode git-commit-push [OPTIONS] [MESSAGE]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `MESSAGE` | Custom commit message | `STRING` | Auto-generated | No |
| `--dry-run` | Show what would be committed and pushed without making changes | `FLAG` | `false` | No |
| `--force` | Force push (use with caution) | `FLAG` | `false` | No |
| `--no-verify` | Skip pre-commit and pre-push hooks | `FLAG` | `false` | No |
| `--remote <REMOTE>` | Specify remote repository name | `STRING` | `origin` | No |
| `--branch <BRANCH>` | Specify branch to push to | `STRING` | current branch | No |
| `--skip-security` | Skip security analysis (not recommended) | `FLAG` | `false` | No |
| `--skip-quality` | Skip quality validation | `FLAG` | `false` | No |

## Examples

### Basic Usage
```bash
# Commit and push with auto-generated message
opencode git-commit-push

# Commit and push with custom message
opencode git-commit-push "Fix authentication vulnerability"

# Dry run to see what would be committed and pushed
opencode git-commit-push --dry-run
```

### Advanced Usage
```bash
# Push to specific remote and branch
opencode git-commit-push --remote upstream --branch main "Update documentation"

# Force push (use with caution)
opencode git-commit-push --force "Emergency security fix"

# Skip hooks for CI/CD environments
opencode git-commit-push --no-verify "Automated deployment"

# Skip security analysis for urgent fixes
opencode git-commit-push --skip-security "Hotfix for production issue"
```

### Integration Examples
```bash
# Use in CI/CD pipeline
opencode git-commit-push --no-verify --remote origin --branch develop "CI build $(date +%Y%m%d-%H%M%S)"

# Commit and push with security analysis
opencode git-commit-push "Implement secure password hashing"

# Batch operation with multiple files
git add src/security/ tests/
opencode git-commit-push "Add comprehensive security tests"
```

## Error Handling

### Common Errors
- **No Staged Changes**: Attempting to commit when no files are staged
  ```bash
  opencode git-commit-push
  # Error: No staged changes found. Use 'git add' to stage files.
  ```

- **Security Issues Found**: Security analysis detects vulnerabilities
  ```bash
  opencode git-commit-push
  # Error: Security issues found in staged changes. Review and fix before committing.
  ```

- **Remote Repository Issues**: Problems with remote repository access
  ```bash
  opencode git-commit-push
  # Error: Failed to push to remote 'origin'. Check network connectivity and permissions.
  ```

- **Merge Conflicts**: Remote branch has diverged
  ```bash
  opencode git-commit-push
  # Error: Push failed due to merge conflicts. Pull changes first.
  ```

### Recovery Procedures
1. **No Staged Changes**: Stage files before committing
   ```bash
   git add .
   opencode git-commit-push
   ```

2. **Security Issues**: Review and fix security issues
   ```bash
   opencode check --security
   # Fix identified issues
   opencode git-commit-push
   ```

3. **Remote Issues**: Verify remote configuration and permissions
   ```bash
   git remote -v
   git config --list | grep remote
   ```

4. **Merge Conflicts**: Pull and resolve conflicts
   ```bash
   git pull --rebase
   # Resolve conflicts
   opencode git-commit-push
   ```

## Security Considerations
- **Pre-Commit Security Analysis**: All staged changes are scanned for security vulnerabilities
- **Credential Protection**: Never stores or logs sensitive authentication information
- **Remote Verification**: Validates remote repository authenticity and permissions
- **Force Push Protection**: Warns about potential data loss with force push operations
- **Hook Integration**: Integrates with Git hooks for additional security checks
- **Audit Trail**: Maintains logs of all security analysis results and decisions

## Best Practices

### Security Best Practices
- **Always Run Security Analysis**: Never skip security checks unless absolutely necessary
- **Review Auto-Generated Messages**: Verify commit messages for accuracy and clarity
- **Use Specific Messages**: Provide descriptive commit messages for better tracking
- **Regular Remote Updates**: Keep remote repositories synchronized to avoid conflicts
- **Branch Protection**: Use protected branches and require reviews for critical changes

### Performance Optimization Tips
- **Stage Selectively**: Only stage files that are ready to be committed
- **Use Dry Run**: Test operations with `--dry-run` before actual execution
- **Batch Commits**: Group related changes into single commits for better history
- **Optimize Analysis**: Configure analysis rules for your project's specific needs

### Common Pitfalls to Avoid
- **Skipping Security Checks**: Never use `--skip-security` for production code
- **Force Pushing**: Avoid force push unless you understand the consequences
- **Generic Messages**: Don't use generic messages like "update" or "fix"
- **Large Commits**: Break large changes into smaller, focused commits
- **Ignoring Conflicts**: Always resolve merge conflicts before pushing

### Integration Recommendations
- **CI/CD Pipelines**: Use `--no-verify` in automated environments
- **Pre-commit Hooks**: Integrate with Git hooks for developer workflow
- **Branch Strategies**: Align with your team's branching and release strategy
- **Code Review**: Use in conjunction with pull request workflows

### Maintenance Guidelines
- **Regular Updates**: Keep CodeGuardian and Git updated for latest security features
- **Configuration Review**: Regularly review and update analysis configurations
- **Team Training**: Ensure team members understand security implications
- **Audit Logs**: Monitor commit and push activities for security compliance

## Configuration
The command respects CodeGuardian's configuration file (`codeguardian.toml`) for:
- Security analysis rules and severity levels
- Quality validation thresholds
- Remote repository settings
- Hook integration preferences
- Performance optimization settings

## Output
The command provides detailed output including:
- Files staged for commit
- Security analysis results
- Generated or provided commit message
- Push operation status
- Any warnings or errors encountered

## Exit Codes
- `0`: Success
- `1`: General error
- `2`: Security issues found
- `3`: No staged changes
- `4`: Remote repository error
- `5`: Merge conflict detected

## See Also
- [`opencode git-commit`](git-commit.md) - Commit changes without pushing
- [`opencode check`](check.md) - Run security and quality analysis
- [`opencode report`](report.md) - Generate detailed analysis reports
- [Git Best Practices](../agent/git-best-practices.md) - Git workflow recommendations
- [Configuration Guide](../user-guide/configuration.md) - Configure analysis settings