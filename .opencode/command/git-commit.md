---
description: Enhanced git commit command with security-conscious features and CodeGuardian integration
---

# git-commit

## Synopsis
Enhanced git commit command that integrates CodeGuardian security analysis, provides intelligent commit message generation, validates code quality before committing, and ensures security standards are maintained throughout the development workflow.

## Description
The git-commit command extends standard git commit functionality with comprehensive security and code quality validation. It automatically runs CodeGuardian analysis on staged changes, generates intelligent commit messages based on code changes, validates against security policies, and provides detailed feedback before allowing commits.

Key capabilities include:
- **Security-First Validation**: Automatic CodeGuardian analysis before commits
- **Intelligent Message Generation**: AI-powered commit message suggestions based on code changes
- **Policy Enforcement**: Configurable commit policies and validation rules
- **Change Analysis**: Detailed analysis of what files changed and their impact
- **Security Scanning**: Pre-commit security vulnerability detection
- **Quality Gates**: Code quality validation with configurable thresholds
- **Audit Trail**: Comprehensive logging of commit validation results
- **Integration Ready**: Seamless integration with CI/CD pipelines and workflows

## Syntax
```bash
codeguardian git-commit [OPTIONS] [-- MESSAGE]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--message <MESSAGE>` | Commit message (overrides auto-generation) | `STRING` | - | No |
| `--generate-message` | Generate commit message from code changes | `FLAG` | `true` | No |
| `--security-scan` | Run security analysis before commit | `FLAG` | `true` | No |
| `--quality-check` | Run code quality validation | `FLAG` | `true` | No |
| `--fail-on-security` | Fail commit if security issues found | `FLAG` | `true` | No |
| `--fail-on-quality` | Fail commit if quality issues found | `FLAG` | `false` | No |
| `--allow-empty` | Allow empty commits | `FLAG` | `false` | No |
| `--amend` | Amend previous commit | `FLAG` | `false` | No |
| `--no-verify` | Skip pre-commit hooks and validation | `FLAG` | `false` | No |
| `--dry-run` | Show what would be committed without committing | `FLAG` | `false` | No |
| `--verbose` | Show detailed analysis output | `FLAG` | `false` | No |
| `--config <FILE>` | Path to CodeGuardian configuration | `PATH` | `codeguardian.toml` | No |
| `--output <FILE>` | Save analysis results to file | `PATH` | - | No |
| `--timeout <SECONDS>` | Maximum analysis time in seconds | `NUMBER` | `300` | No |
| `--exclude <PATTERN>` | Patterns to exclude from analysis | `STRING` | - | No |
| `--include <PATTERN>` | Patterns to include in analysis | `STRING` | - | No |

## Examples

### Basic Usage
```bash
# Commit with automatic message generation and security validation
codeguardian git-commit

# Commit with custom message
codeguardian git-commit --message "Fix authentication vulnerability"

# Dry run to preview changes
codeguardian git-commit --dry-run
```

### Advanced Usage
```bash
# Commit with comprehensive validation and detailed output
codeguardian git-commit \
  --generate-message \
  --security-scan \
  --quality-check \
  --verbose \
  --output commit-analysis.json

# Security-focused commit with strict validation
codeguardian git-commit \
  --fail-on-security \
  --fail-on-quality \
  --timeout 600 \
  --config security-focused.toml

# Commit specific file types only
codeguardian git-commit \
  --include "*.rs" \
  --include "*.toml" \
  --exclude "test/**"
```

### Integration Examples
```bash
# CI/CD pipeline integration
codeguardian git-commit \
  --no-verify \
  --fail-on-security \
  --output ci-results.json

# Pre-commit hook integration
codeguardian git-commit \
  --security-scan \
  --quality-check \
  --fail-on-security
```

## Analysis Output

### Commit Analysis Report
The command provides detailed analysis of staged changes:

```json
{
  "commit_analysis": {
    "files_changed": 5,
    "lines_added": 127,
    "lines_deleted": 23,
    "risk_assessment": "LOW",
    "security_issues": 0,
    "quality_score": 8.7
  },
  "security_findings": [],
  "quality_findings": [
    {
      "type": "style",
      "severity": "minor",
      "message": "Line too long (120 characters)"
    }
  ],
  "suggested_message": "feat: implement user authentication with JWT tokens\n\n- Add JWT token validation\n- Implement user session management\n- Add security headers"
}
```

### Security Validation Results
- **Vulnerability Scanning**: Detects security vulnerabilities in code changes
- **Secret Detection**: Identifies potential credential leaks
- **Dependency Analysis**: Checks for vulnerable dependencies
- **Code Injection Prevention**: Validates input sanitization

### Quality Assessment
- **Code Style**: Enforces consistent coding standards
- **Complexity Analysis**: Measures code complexity metrics
- **Test Coverage**: Validates test coverage requirements
- **Documentation**: Checks for adequate code documentation

## Error Handling

### Common Errors
- **Security Issues Found**: Commit blocked due to security vulnerabilities
  ```bash
  codeguardian git-commit
  # Error: Security issues detected:
  # - Potential SQL injection in user_input.rs:45
  # - Hardcoded credentials in config.rs:12
  # Commit aborted. Fix issues or use --no-verify to override.
  ```

- **Quality Threshold Not Met**: Commit blocked due to quality standards
  ```bash
  codeguardian git-commit --fail-on-quality
  # Error: Quality score 6.2 below threshold 8.0
  # - Code complexity too high in main.rs
  # - Missing documentation for public functions
  ```

- **Analysis Timeout**: Analysis takes too long
  ```bash
  codeguardian git-commit --timeout 60
  # Error: Analysis timeout after 60 seconds
  # Try increasing timeout or reducing analysis scope
  ```

- **No Staged Changes**: Attempting to commit without staged files
  ```bash
  codeguardian git-commit
  # Error: No staged changes to commit
  # Stage files first: git add <files>
  ```

### Recovery Procedures
1. **Fix Security Issues**: Address identified security vulnerabilities
   ```bash
   # Review security findings
   codeguardian git-commit --dry-run --verbose

   # Fix identified issues
   vim vulnerable_file.rs

   # Retry commit
   codeguardian git-commit
   ```

2. **Improve Code Quality**: Address quality issues
   ```bash
   # Check quality details
   codeguardian git-commit --dry-run --quality-check --verbose

   # Run formatter
   cargo fmt

   # Add missing tests
   cargo test
   ```

3. **Handle Large Commits**: Break down large commits
   ```bash
   # Analyze commit size
   git diff --cached --stat

   # Stage files in smaller groups
   git reset HEAD .
   git add file1.rs file2.rs
   codeguardian git-commit --message "Add authentication module"
   ```

4. **Override Validation**: Use with caution for urgent commits
   ```bash
   # Skip validation (use only when necessary)
   codeguardian git-commit --no-verify --message "Hotfix: emergency security patch"
   ```

## Security Considerations
- **Input Validation**: All commit messages and file paths are validated
- **Secret Detection**: Automatic scanning for credentials and sensitive data
- **Path Traversal Protection**: Prevents directory traversal attacks in file paths
- **Command Injection Prevention**: Sanitizes all shell commands and arguments
- **Audit Logging**: Comprehensive logging of all commit validation activities
- **Permission Validation**: Ensures proper git repository permissions
- **File Size Limits**: Prevents analysis of excessively large files
- **Timeout Protection**: Prevents resource exhaustion through analysis timeouts

## Best Practices

### Security-First Approach
- **Always Enable Security Scanning**: Keep `--security-scan` enabled for all commits
- **Regular Secret Audits**: Periodically audit repository for leaked credentials
- **Dependency Updates**: Keep dependencies updated to address security vulnerabilities
- **Code Review Integration**: Use with pull request reviews for additional validation

### Performance Optimization
- **Incremental Analysis**: Only analyze changed files for faster validation
- **Parallel Processing**: Utilize multiple cores for large codebases
- **Caching Strategies**: Cache analysis results for repeated patterns
- **Selective Scanning**: Use include/exclude patterns to focus analysis scope

### Workflow Integration
- **Pre-commit Hooks**: Integrate with git hooks for automatic validation
- **CI/CD Pipelines**: Use in automated pipelines for consistent validation
- **Branch Protection**: Require validation for protected branches
- **Team Standards**: Establish team-wide commit standards and policies

### Quality Assurance
- **Consistent Standards**: Enforce coding standards across the team
- **Test Coverage**: Maintain adequate test coverage for all changes
- **Documentation**: Ensure proper documentation for public APIs
- **Code Reviews**: Use automated validation as part of code review process

## Integration Examples

### GitHub Actions Integration
```yaml
# .github/workflows/commit-validation.yml
name: Commit Validation
on: [push, pull_request]

jobs:
  validate-commit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Validate Commit
        run: |
          codeguardian git-commit --dry-run --security-scan --quality-check --fail-on-security
```

### Pre-commit Hook Integration
```bash
#!/bin/sh
# .git/hooks/pre-commit

# Run CodeGuardian validation
codeguardian git-commit --dry-run --security-scan --fail-on-security

# Exit with validation result
exit $?
```

### CI/CD Pipeline Integration
```yaml
# .gitlab-ci.yml
stages:
  - validate
  - commit

validate_commit:
  stage: validate
  script:
    - codeguardian git-commit --dry-run --security-scan --quality-check --output validation-results.json
  artifacts:
    reports:
      codeguardian: validation-results.json

automated_commit:
  stage: commit
  script:
    - codeguardian git-commit --no-verify --message "Automated deployment commit"
  only:
    - main
```

## Configuration

### Commit Policies Configuration
```toml
# codeguardian.toml
[git-commit]
security_scan = true
quality_check = true
fail_on_security = true
fail_on_quality = false
timeout_seconds = 300
max_file_size_mb = 10

[git-commit.quality]
min_score = 8.0
max_complexity = 15
require_tests = true
require_docs = false

[git-commit.security]
block_high_severity = true
scan_secrets = true
scan_dependencies = true
custom_rules = ["no_hardcoded_passwords", "validate_input_sanitization"]
```

### Environment Variables
```bash
# Security configuration
export CODEGUARDIAN_COMMIT_FAIL_ON_SECURITY=true
export CODEGUARDIAN_COMMIT_TIMEOUT=600

# Quality thresholds
export CODEGUARDIAN_QUALITY_MIN_SCORE=8.5
export CODEGUARDIAN_QUALITY_MAX_COMPLEXITY=10

# Integration settings
export CODEGUARDIAN_GIT_HOOKS_ENABLED=true
export CODEGUARDIAN_CI_MODE=true
```

## Troubleshooting

### Common Issues
1. **Slow Analysis**: Reduce scope or increase timeout
2. **False Positives**: Adjust configuration or use ML model
3. **Hook Conflicts**: Check existing git hooks for conflicts
4. **Permission Issues**: Verify git repository permissions

### Debug Mode
```bash
# Enable verbose debugging
codeguardian git-commit --verbose --dry-run

# Save detailed analysis
codeguardian git-commit --output debug-analysis.json --verbose
```

### Performance Tuning
```bash
# Optimize for large repositories
codeguardian git-commit \
  --exclude "node_modules/**" \
  --exclude "target/**" \
  --timeout 600 \
  --parallel 4
```

## See Also
- [`codeguardian check`](analysis.md) - Primary code analysis command
- [`codeguardian git-hooks-setup`](git-hooks-setup.md) - Set up Git hooks
- [`codeguardian git-workflow`](git-workflow.md) - Git workflow analysis
- [`codeguardian report`](report.md) - Generate analysis reports
- [Configuration Guide](../configuration.md) - Configuration options
- [CI/CD Setup Guide](../user-guide/ci-cd-setup.md) - CI/CD integration