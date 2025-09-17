# gh-issue

## Synopsis
Create or update GitHub issues from CodeGuardian analysis results with support for multiple issue modes and automatic lifecycle management.

## Description
The gh-issue command integrates CodeGuardian findings directly into GitHub's issue tracking system, enabling automated security and code quality issue management. It supports different issue formats and can update existing issues based on analysis results.

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
| `--summary-auto <TEMPLATE>` | Auto-generate summary | `STRING` | - | No |
| `--summary-max-chars <NUM>` | Maximum characters in summary | `usize` | `800` | No |
| `--summary-max-issues <NUM>` | Maximum issues to include in summary | `usize` | `10` | No |
| `--dry-run` | Dry run mode (print commands without executing) | `FLAG` | `false` | No |

### GhMode Values
- `checklist`: Checklist format with checkboxes
- `simple`: Simple issue format
- `children`: Children mode for large reports

## Examples
```bash
# Create GitHub issue from analysis results
codeguardian gh-issue --repo myorg/myrepo

# Use checklist mode for detailed tracking
codeguardian gh-issue --repo myorg/myrepo --mode checklist

# Custom title and labels
codeguardian gh-issue --repo myorg/myrepo \
  --title "Security Analysis: " \
  --labels "security,codeguardian"

# Dry run to preview issue creation
codeguardian gh-issue --repo myorg/myrepo --dry-run

# Use custom summary
codeguardian gh-issue --repo myorg/myrepo \
  --summary-auto "Weekly security scan results"
```

## See Also
- [`codeguardian check`](check.md) - Run code analysis
- [`codeguardian report`](report.md) - Generate reports from results
