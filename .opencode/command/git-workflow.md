---
title: "git-workflow"
description: "Analyze and optimize Git workflows for security and code quality"
category: "Git Integration"
tags: ["git", "workflow", "analysis", "security", "optimization"]
---

# git-workflow

Analyze Git repository workflows and provide recommendations for integrating CodeGuardian analysis into development processes, ensuring security and quality standards are maintained throughout the software development lifecycle.

## Synopsis

```bash
codeguardian git-workflow [OPTIONS] [COMMAND]
```

## Description

The `git-workflow` command analyzes your Git repository's workflow patterns and provides actionable recommendations for integrating CodeGuardian security and code quality analysis. It examines commit patterns, branch strategies, and collaboration workflows to identify opportunities for improvement.

### Key Features

- **Workflow analysis**: Examine commit patterns and branch strategies
- **Security integration**: Recommend security checkpoints in workflows
- **Performance optimization**: Optimize analysis placement for efficiency
- **Collaboration insights**: Identify team workflow improvements
- **Automation suggestions**: Propose automated analysis integration points

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--analyze-commits` | Number of recent commits to analyze | number | 100 | No |
| `--branch-pattern` | Branch naming pattern to analyze | string | feature/*,bugfix/*,hotfix/* | No |
| `--output-format` | Output format (json,markdown,table) | string | markdown | No |
| `--recommend-only` | Only show recommendations, skip analysis | boolean | false | No |
| `--config` | Path to CodeGuardian configuration file | string | codeguardian.toml | No |
| `--include-security` | Include security-specific workflow recommendations | boolean | true | No |

## Commands

### analyze
Analyze current Git workflow and provide recommendations.

```bash
codeguardian git-workflow analyze [OPTIONS]
```

### optimize
Generate optimized workflow configuration.

```bash
codeguardian git-workflow optimize [OPTIONS]
```

### validate
Validate workflow against security and quality standards.

```bash
codeguardian git-workflow validate [OPTIONS]
```

## Examples

### Basic Workflow Analysis

```bash
# Analyze recent workflow patterns
codeguardian git-workflow analyze
```

### Deep Commit Analysis

```bash
# Analyze last 500 commits with detailed output
codeguardian git-workflow analyze --analyze-commits 500 --output-format json
```

### Branch-Specific Analysis

```bash
# Analyze specific branch patterns
codeguardian git-workflow analyze --branch-pattern "release/*,main"
```

### Generate Optimization Report

```bash
# Generate workflow optimization recommendations
codeguardian git-workflow optimize --output-format markdown > workflow-optimization.md
```

### Security-Focused Validation

```bash
# Validate workflow against security standards
codeguardian git-workflow validate --include-security
```

## Analysis Output

### Workflow Metrics

The command provides comprehensive workflow metrics:

```json
{
  "commit_frequency": {
    "daily_average": 12.5,
    "peak_hours": ["10:00-11:00", "14:00-15:00"],
    "weekend_activity": 0.3
  },
  "branch_strategy": {
    "feature_branches": 85,
    "hotfix_branches": 12,
    "long_running_branches": 2
  },
  "collaboration": {
    "active_contributors": 8,
    "code_review_rate": 0.92,
    "merge_conflicts": 0.05
  }
}
```

### Security Recommendations

- **Pre-commit analysis**: Implement automatic security scanning
- **Branch protection**: Require CodeGuardian checks for protected branches
- **Merge validation**: Validate security policies before merges
- **Access control**: Implement role-based access for sensitive operations

### Performance Optimization

- **Incremental analysis**: Only analyze changed files
- **Parallel processing**: Utilize multiple cores for faster analysis
- **Caching strategies**: Cache analysis results for repeated patterns
- **Selective scanning**: Skip analysis for documentation-only changes

## Best Practices

### Security Integration

- **Zero-trust approach**: Validate every code change
- **Defense in depth**: Multiple security layers in workflow
- **Continuous monitoring**: Regular workflow security audits
- **Incident response**: Clear procedures for security incidents

### Performance Considerations

- **Analysis placement**: Strategic placement of analysis steps
- **Resource allocation**: Appropriate compute resources for analysis
- **Caching optimization**: Maximize cache hit rates
- **Parallel execution**: Concurrent analysis where possible

### Team Collaboration

- **Consistent standards**: Uniform workflow across team
- **Knowledge sharing**: Document workflow decisions
- **Training programs**: Regular security awareness training
- **Feedback loops**: Continuous improvement processes

## Error Handling

### Common Issues

- **Repository access**: Ensure proper Git repository access
  ```bash
  git remote -v
  git status
  ```

- **Analysis timeout**: Large repositories may require longer analysis time
  ```bash
  codeguardian git-workflow analyze --analyze-commits 50
  ```

- **Branch pattern mismatch**: Verify branch naming conventions
  ```bash
  git branch -a | grep -E "feature|bugfix|hotfix"
  ```

### Troubleshooting

1. **Check Git repository status**:
   ```bash
   git log --oneline -10
   git branch -a
   ```

2. **Verify CodeGuardian configuration**:
   ```bash
   codeguardian check --dry-run
   ```

3. **Test analysis on subset**:
   ```bash
   codeguardian git-workflow analyze --analyze-commits 10
   ```

## Integration Examples

### GitHub Actions Integration

```yaml
# .github/workflows/workflow-analysis.yml
name: Workflow Analysis
on:
  schedule:
    - cron: '0 2 * * 1'  # Weekly on Monday
  workflow_dispatch:

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - name: Analyze Git Workflow
        run: codeguardian git-workflow analyze --output-format json > workflow-report.json
      - name: Upload Report
        uses: actions/upload-artifact@v4
        with:
          name: workflow-report
          path: workflow-report.json
```

### Pre-commit Integration

```bash
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: codeguardian-workflow
        name: CodeGuardian Workflow Check
        entry: codeguardian git-workflow validate
        language: system
        pass_filenames: false
```

## See Also

- [`codeguardian git-hooks-setup`](git-hooks-setup.md) - Set up Git hooks
- [`codeguardian check`](../../../commands/check.md) - Primary analysis command
- [`codeguardian ci-cd github-actions`](../../../ci-cd/github-actions.md) - GitHub Actions integration
- [Git Workflow Best Practices](https://git-scm.com/book/en/v2/Git-Branching-Branching-Workflows)
