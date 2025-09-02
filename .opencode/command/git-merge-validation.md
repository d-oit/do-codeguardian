---
title: "git-merge-validation"
description: "Validate merges against security policies and code quality standards"
category: "Git Integration"
tags: ["git", "merge", "validation", "security", "policies"]
---

# git-merge-validation

Perform comprehensive validation of Git merges against CodeGuardian security policies and code quality standards, ensuring that only compliant code enters protected branches.

## Synopsis

```bash
codeguardian git-merge-validation [OPTIONS] [COMMAND]
```

## Description

The `git-merge-validation` command validates Git merge operations against comprehensive security and code quality policies. It analyzes the merge commit, checks for policy compliance, and provides detailed reports on any violations found.

### Key Features

- **Merge analysis**: Comprehensive analysis of merge commits
- **Policy enforcement**: Validate against custom security policies
- **Risk assessment**: Assess security risk of merge operations
- **Compliance reporting**: Generate detailed compliance reports
- **Automated validation**: Integrate with merge workflows

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--merge-commit` | Specific merge commit to validate | string | HEAD | No |
| `--policy-file` | Path to security policy file | string | security-policy.toml | No |
| `--baseline` | Baseline commit for comparison | string | HEAD~1 | No |
| `--output-format` | Output format (json,markdown,table) | string | markdown | No |
| `--fail-on-violations` | Exit with error on policy violations | boolean | true | No |
| `--include-risk-assessment` | Include security risk assessment | boolean | true | No |
| `--max-violations` | Maximum violations to report | number | 50 | No |
| `--exclude-patterns` | Patterns to exclude from validation | string[] | [] | No |

## Commands

### validate
Validate a merge commit against policies.

```bash
codeguardian git-merge-validation validate [OPTIONS]
```

### analyze
Analyze merge impact and security implications.

```bash
codeguardian git-merge-validation analyze [OPTIONS]
```

### report
Generate comprehensive merge validation report.

```bash
codeguardian git-merge-validation report [OPTIONS]
```

### policy-check
Check merge against specific security policies.

```bash
codeguardian git-merge-validation policy-check [OPTIONS]
```

## Examples

### Basic Merge Validation

```bash
# Validate the latest merge commit
codeguardian git-merge-validation validate
```

### Specific Merge Commit

```bash
# Validate a specific merge commit
codeguardian git-merge-validation validate --merge-commit abc123
```

### Policy-Based Validation

```bash
# Validate against custom security policy
codeguardian git-merge-validation validate \
  --policy-file ./custom-security-policy.toml \
  --fail-on-violations
```

### Risk Assessment

```bash
# Include detailed risk assessment
codeguardian git-merge-validation analyze \
  --include-risk-assessment \
  --output-format json
```

### Generate Compliance Report

```bash
# Generate detailed compliance report
codeguardian git-merge-validation report \
  --merge-commit def456 \
  --output-format markdown > merge-report.md
```

## Validation Policies

### Security Policy Structure

```toml
# security-policy.toml
[security]
max_critical_vulnerabilities = 0
max_high_vulnerabilities = 2
max_medium_vulnerabilities = 10

[code_quality]
max_complexity_score = 15
require_tests = true
max_duplicate_lines = 5

[dependencies]
allow_dev_dependencies = true
max_vulnerable_dependencies = 0
require_dependency_audit = true

[compliance]
require_license_check = true
max_license_violations = 0
require_security_headers = true
```

### Validation Checks

- **Security vulnerabilities**: Scan for known security issues
- **Code quality metrics**: Check complexity, duplication, test coverage
- **Dependency analysis**: Validate third-party dependencies
- **License compliance**: Ensure license compatibility
- **Security headers**: Check for required security headers

## Risk Assessment

### Risk Levels

- **Critical**: Immediate security threat, block merge
- **High**: Significant security concern, require review
- **Medium**: Moderate risk, monitor closely
- **Low**: Minor issue, log for awareness
- **Info**: Informational, no action required

### Risk Factors

```json
{
  "security_risk": {
    "level": "high",
    "score": 8.5,
    "factors": [
      "Critical vulnerability introduced",
      "Privilege escalation possible",
      "Data exposure risk"
    ]
  },
  "code_quality_risk": {
    "level": "medium",
    "score": 6.2,
    "factors": [
      "High complexity functions",
      "Insufficient test coverage"
    ]
  }
}
```

## Best Practices

### Security Considerations

- **Zero-trust validation**: Validate every merge operation
- **Policy as code**: Define security policies in version control
- **Automated enforcement**: Never allow manual bypass of critical checks
- **Regular policy updates**: Keep security policies current

### Performance Optimization

- **Incremental validation**: Only validate changed components
- **Parallel processing**: Run checks concurrently
- **Caching**: Cache policy evaluations for repeated patterns
- **Selective scanning**: Skip validation for documentation-only changes

### Workflow Integration

- **CI/CD integration**: Validate merges in automated pipelines
- **Manual review**: Require human review for high-risk merges
- **Audit logging**: Maintain detailed audit logs of validations
- **Feedback loops**: Use validation results to improve policies

## Error Handling

### Common Issues

- **Policy file not found**: Ensure policy file exists and is accessible
  ```bash
  ls -la security-policy.toml
  ```

- **Merge commit not found**: Verify commit hash is correct
  ```bash
  git log --oneline -10
  ```

- **Permission denied**: Ensure proper access to repository
  ```bash
  git remote -v
  ```

### Troubleshooting

1. **Check merge commit**:
   ```bash
   git show --stat HEAD
   git log --merges -5
   ```

2. **Validate policy file**:
   ```bash
   codeguardian git-merge-validation policy-check --policy-file security-policy.toml
   ```

3. **Test validation on subset**:
   ```bash
   codeguardian git-merge-validation validate --max-violations 5
   ```

4. **Check repository access**:
   ```bash
   git ls-remote origin
   ```

## Integration Examples

### GitHub Actions Integration

```yaml
# .github/workflows/merge-validation.yml
name: Merge Validation
on:
  pull_request_target:
    types: [opened, synchronize, ready_for_review]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - name: Validate Merge
        id: validation
        run: |
          codeguardian git-merge-validation validate \
            --merge-commit ${{ github.event.pull_request.merge_commit_sha }} \
            --fail-on-violations \
            --output-format json > validation-result.json
      - name: Upload Results
        uses: actions/upload-artifact@v4
        with:
          name: merge-validation
          path: validation-result.json
```

### Git Hook Integration

```bash
# .git/hooks/pre-merge-commit
#!/bin/sh

# Validate merge before completion
codeguardian git-merge-validation validate --fail-on-violations

if [ $? -ne 0 ]; then
    echo "Merge validation failed. Please address security concerns."
    exit 1
fi
```

### CI/CD Pipeline Integration

```yaml
# .gitlab-ci.yml
stages:
  - validate
  - merge

merge_validation:
  stage: validate
  script:
    - codeguardian git-merge-validation validate --fail-on-violations
  only:
    - merge_requests
  artifacts:
    reports:
      junit: validation-results.xml
```

## See Also

- [`codeguardian git-branch-protection`](git-branch-protection.md) - Branch protection setup
- [`codeguardian git-hooks-setup`](git-hooks-setup.md) - Git hooks configuration
- [`codeguardian check`](../../../commands/check.md) - Primary analysis command
- [Git Merge Strategies](https://git-scm.com/docs/git-merge)
