---
description: Create and manage GitHub issues from CodeGuardian analysis results
---

# gh-issue

## Synopsis
CodeGuardian's GitHub integration command that automatically creates, updates, and manages GitHub issues from analysis results, supporting multiple issue modes, lifecycle management, and seamless integration with GitHub workflows for tracking security findings and code quality issues.

## Description
The gh-issue command bridges CodeGuardian's analysis results with GitHub's issue tracking system, providing intelligent issue management with support for different analysis contexts (PRs, pushes, scheduled scans). It implements sophisticated issue lifecycle management, duplicate detection, and multiple presentation modes to ensure effective communication of analysis findings to development teams.

Key capabilities include:
- **Context-Aware Issue Creation**: Automatic issue titles and content based on GitHub event context
- **Multiple Issue Modes**: Checklist, simple, and children modes for different use cases
- **Issue Lifecycle Management**: Smart updating, closing, and reopening of issues
- **Duplicate Detection**: Intelligent detection and handling of duplicate issues
- **Rich Content Generation**: Markdown formatting with severity indicators and actionable information
- **GitHub API Integration**: Secure, rate-limited integration with GitHub's REST API

## Syntax
```bash
codeguardian gh-issue [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--from <FILE>` | Input results file | `PATH` | `results.json` | No |
| `--repo <REPO>` | GitHub repository (owner/repo) | `STRING` | - | Yes |
| `--mode <MODE>` | GitHub issue mode | `GhMode` | `checklist` | No |
| `--title <TITLE>` | Issue title prefix | `STRING` | `CodeGuardian: ` | No |
| `--labels <LABELS>` | Issue labels | `STRING` | `codeguardian,automated` | No |
| `--summary-from <FILE>` | Manual summary file | `PATH` | - | No |
| `--summary-auto <PROVIDER>` | Auto-generate summary | `STRING` | - | No |
| `--summary-max-chars <NUM>` | Maximum characters in summary | `usize` | `800` | No |
| `--summary-max-issues <NUM>` | Maximum issues to include | `usize` | `10` | No |
| `--dry-run` | Dry run mode (print commands without executing) | `FLAG` | `false` | No |

### GhMode Values
- `checklist`: Checklist format with checkboxes for tracking resolution progress
- `simple`: Simple issue format with direct markdown report inclusion
- `children`: Children mode creating separate issues for high-priority findings

## Examples

### Basic Usage
```bash
# Create GitHub issue from analysis results
codeguardian gh-issue --repo myorg/myrepo

# Use specific results file
codeguardian gh-issue --from analysis-results.json --repo myorg/myrepo

# Dry run to preview issue creation
codeguardian gh-issue --repo myorg/myrepo --dry-run
```

### Advanced Usage
```bash
# Create checklist-style issue with custom labels
codeguardian gh-issue \
  --repo myorg/myrepo \
  --mode checklist \
  --labels "codeguardian,security,automated" \
  --title "Security Analysis: "

# Use children mode for large reports
codeguardian gh-issue \
  --repo myorg/myrepo \
  --mode children \
  --summary-max-issues 20

# Auto-generate AI summary
codeguardian gh-issue \
  --repo myorg/myrepo \
  --summary-auto openai \
  --summary-max-chars 1000
```

## Error Handling

### Common Errors
- **Repository Not Found**: Specified repository doesn't exist or is inaccessible
  ```bash
  codeguardian gh-issue --repo nonexistent/repo
  # Error: Repository not found or access denied
  ```

- **GitHub Token Missing**: No authentication token provided
  ```bash
  codeguardian gh-issue --repo myorg/myrepo
  # Error: GitHub token not found. Set GITHUB_TOKEN environment variable
  ```

- **Results File Missing**: Specified results file doesn't exist
  ```bash
  codeguardian gh-issue --from nonexistent.json --repo myorg/myrepo
  # Error: No such file or directory
  ```

### Recovery Procedures
1. **Authentication Issues**: Set GitHub token and verify permissions
   ```bash
   export GITHUB_TOKEN=your_token_here
   codeguardian gh-issue --repo myorg/myrepo
   ```

2. **Repository Access**: Verify repository exists and token has proper permissions
   ```bash
   curl -H "Authorization: token $GITHUB_TOKEN" \
        https://api.github.com/repos/myorg/myrepo
   ```

3. **File Issues**: Generate results file first or use default path
   ```bash
   codeguardian check . --format json --out results.json
   codeguardian gh-issue --repo myorg/myrepo
   ```

## Security Considerations
- **Token Security**: GitHub tokens are handled securely and never logged in plain text
- **Repository Validation**: Repository names are validated to prevent injection attacks
- **Content Sanitization**: Issue content is sanitized to prevent XSS in GitHub interface
- **Rate Limiting**: API calls are rate-limited to prevent abuse
- **Permission Validation**: Token permissions are validated before API operations
- **HTTPS Only**: All GitHub API communications use HTTPS with certificate validation

## Best Practices

### Security Best Practices
- **Token Management**: Use fine-grained personal access tokens with minimal required permissions
- **Token Rotation**: Regularly rotate GitHub tokens and revoke compromised ones
- **Repository Access**: Limit repository access to necessary team members only
- **Issue Content Review**: Review generated issue content before creation in sensitive repositories

### Performance Optimization Tips
- **Batch Processing**: Use children mode for large result sets to avoid GitHub API limits
- **Incremental Updates**: Leverage issue updating instead of creating duplicates
- **Summary Optimization**: Use appropriate summary lengths to balance information and readability
- **Label Strategy**: Use consistent labeling schemes across repositories

### Common Pitfalls to Avoid
- **Missing Repository Specification**: Always specify the target repository
- **Insufficient Token Permissions**: Ensure tokens have both `repo` and `issues` permissions
- **Large Issue Content**: Use children mode for reports that exceed GitHub's size limits
- **Duplicate Issue Creation**: Let the system handle duplicate detection rather than manual checks

### Integration Recommendations
- **GitHub Actions**: Integrate with GitHub Actions for automated issue creation on PRs/pushes
- **Branch Protection**: Use with branch protection rules for mandatory security checks
- **Team Workflows**: Integrate into team workflows for consistent issue tracking
- **Dashboard Integration**: Feed issue data into project dashboards and reporting systems

### Maintenance Guidelines
- **Token Health Monitoring**: Monitor token expiration and permission changes
- **Issue Template Updates**: Update issue templates as analysis capabilities evolve
- **Label Standardization**: Maintain consistent labeling across repositories
- **Performance Monitoring**: Track API usage and optimize for rate limits

## See Also
- [`codeguardian check`](check.md) - Generate analysis results for issue creation
- [`codeguardian report`](report.md) - Convert results to different formats
- [GitHub Integration Guide](../user-guide/github-integration.md) - Complete GitHub workflow integration
- [Issue Management](../user-guide/issue-management.md) - Advanced issue lifecycle management