---
title: "git-branch-protection"
description: "Set up and manage branch protection rules with CodeGuardian integration"
category: "Git Integration"
tags: ["git", "branch-protection", "security", "github", "gitlab"]
---

# git-branch-protection

Configure branch protection rules that integrate CodeGuardian analysis requirements, ensuring that security and code quality standards are enforced before code can be merged into protected branches.

## Synopsis

```bash
codeguardian git-branch-protection [OPTIONS] [COMMAND]
```

## Description

The `git-branch-protection` command helps configure branch protection rules for Git hosting platforms (GitHub, GitLab, etc.) that require CodeGuardian analysis to pass before allowing merges. This ensures that security vulnerabilities and code quality issues are caught before they reach production.

### Key Features

- **Platform integration**: Support for GitHub, GitLab, and Bitbucket
- **Flexible rules**: Configurable protection rules based on branch patterns
- **Status checks**: Automatic CodeGuardian status check integration
- **Review requirements**: Enforce code review processes
- **Security policies**: Implement security-specific protection rules

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--platform` | Git platform (github,gitlab,bitbucket) | string | auto-detect | No |
| `--repo` | Repository in owner/repo format | string | auto-detect | No |
| `--branches` | Branch patterns to protect | string[] | main,develop | No |
| `--require-reviews` | Minimum number of required reviews | number | 1 | No |
| `--require-status-checks` | Require status checks to pass | boolean | true | No |
| `--enforce-admins` | Apply rules to administrators | boolean | false | No |
| `--allow-force-pushes` | Allow force pushes to protected branches | boolean | false | No |
| `--allow-deletions` | Allow branch deletions | boolean | false | No |

## Commands

### setup
Set up branch protection rules for specified branches.

```bash
codeguardian git-branch-protection setup [OPTIONS]
```

### update
Update existing branch protection rules.

```bash
codeguardian git-branch-protection update [OPTIONS]
```

### list
List current branch protection rules.

```bash
codeguardian git-branch-protection list [OPTIONS]
```

### validate
Validate branch protection configuration.

```bash
codeguardian git-branch-protection validate [OPTIONS]
```

## Examples

### Basic Branch Protection Setup

```bash
# Setup protection for main and develop branches
codeguardian git-branch-protection setup --branches main,develop
```

### GitHub-Specific Configuration

```bash
# Setup GitHub branch protection with reviews
codeguardian git-branch-protection setup \
  --platform github \
  --repo myorg/myrepo \
  --require-reviews 2 \
  --enforce-admins
```

### Advanced Security Rules

```bash
# Setup strict protection for production branches
codeguardian git-branch-protection setup \
  --branches main,production/* \
  --require-reviews 3 \
  --require-status-checks \
  --allow-force-pushes false \
  --allow-deletions false
```

### List Current Protection Rules

```bash
# List all branch protection rules
codeguardian git-branch-protection list
```

### Validate Configuration

```bash
# Validate protection rules against security standards
codeguardian git-branch-protection validate --platform github
```

## Platform-Specific Configuration

### GitHub Branch Protection

```json
{
  "required_status_checks": {
    "strict": true,
    "contexts": ["codeguardian/check", "codeguardian/security"]
  },
  "enforce_admins": false,
  "required_pull_request_reviews": {
    "required_approving_review_count": 2,
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": true
  },
  "restrictions": null,
  "allow_force_pushes": false,
  "allow_deletions": false
}
```

### GitLab Branch Protection

```yaml
# .gitlab-ci.yml integration
stages:
  - test
  - security
  - deploy

codeguardian_check:
  stage: security
  script:
    - codeguardian check --fail-on-issues
  only:
    - merge_requests
  artifacts:
    reports:
      junit: results.xml
```

### Status Check Integration

The command automatically sets up status checks that must pass:

```bash
# GitHub status check example
codeguardian check --emit-gh --repo owner/repo --fail-on-issues
```

## Best Practices

### Security Considerations

- **Admin enforcement**: Apply rules to administrators for consistency
- **Review requirements**: Require multiple reviews for critical branches
- **Status check validation**: Ensure all required checks are passing
- **Access control**: Implement principle of least privilege

### Performance Optimization

- **Selective protection**: Only protect branches that need protection
- **Efficient checks**: Use incremental analysis for faster feedback
- **Parallel execution**: Run checks concurrently when possible
- **Caching**: Cache analysis results to reduce redundant work

### Workflow Integration

- **CI/CD integration**: Ensure protection rules work with CI/CD pipelines
- **Emergency procedures**: Document bypass procedures for critical fixes
- **Monitoring**: Monitor protection rule effectiveness
- **Updates**: Regularly review and update protection rules

## Error Handling

### Common Issues

- **Platform API access**: Ensure proper authentication and permissions
  ```bash
  # GitHub: Set GITHUB_TOKEN
  export GITHUB_TOKEN=your_token_here

  # GitLab: Set GITLAB_TOKEN
  export GITLAB_TOKEN=your_token_here
  ```

- **Repository not found**: Verify repository exists and is accessible
  ```bash
  git remote -v
  ```

- **Insufficient permissions**: Check user permissions for branch protection
  ```bash
  # GitHub: Check repository settings
  # GitLab: Check project permissions
  ```

### Troubleshooting

1. **Verify platform connection**:
   ```bash
   codeguardian git-branch-protection list --platform github --repo owner/repo
   ```

2. **Check authentication**:
   ```bash
   # Test API access
   curl -H "Authorization: token $GITHUB_TOKEN" https://api.github.com/user
   ```

3. **Validate repository access**:
   ```bash
   git ls-remote origin
   ```

4. **Test protection rules**:
   ```bash
   codeguardian git-branch-protection validate --platform github
   ```

## Integration with CI/CD

### GitHub Actions Example

```yaml
# .github/workflows/branch-protection.yml
name: Branch Protection
on:
  pull_request_target:
    types: [opened, synchronize, reopened]

jobs:
  codeguardian:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run CodeGuardian
        id: codeguardian
        run: codeguardian check --fail-on-issues --emit-gh
      - name: Update Branch Protection
        if: github.event_name == 'pull_request_target'
        run: |
          codeguardian git-branch-protection update \
            --platform github \
            --repo ${{ github.repository }}
```

### GitLab CI Example

```yaml
# .gitlab-ci.yml
stages:
  - validate
  - protect

validate_branch:
  stage: validate
  script:
    - codeguardian check --fail-on-issues
  only:
    - merge_requests

update_protection:
  stage: protect
  script:
    - codeguardian git-branch-protection update --platform gitlab
  only:
    - main
    - develop
  dependencies:
    - validate_branch
```

## See Also

- [`codeguardian git-hooks-setup`](git-hooks-setup.md) - Set up Git hooks
- [`codeguardian git-workflow`](git-workflow.md) - Analyze Git workflows
- [`codeguardian ci-cd github-actions`](../../../ci-cd/github-actions.md) - GitHub Actions integration
- [GitHub Branch Protection](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/managing-a-branch-protection-rule)