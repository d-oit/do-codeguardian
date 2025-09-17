# report

## Synopsis
Convert CodeGuardian analysis results to different output formats including Markdown, HTML, and plain text for reporting and documentation purposes.

## Description
The report command transforms JSON analysis results into human-readable formats suitable for documentation, CI/CD integration, and stakeholder communication. It supports multiple output formats with rich formatting and comprehensive issue presentation.

## Syntax
```bash
codeguardian report [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--from <FILE>` | Input results file | `PATH` | `results.json` | No |
| `--md <FILE>` | Output markdown file | `PATH` | - | No |
| `--format <FORMAT>` | Output format | `ReportFormat` | `markdown` | No |

### ReportFormat Values
- `markdown`: Markdown format
- `html`: HTML format
- `text`: Plain text format

## Examples
```bash
# Generate markdown report from default results file
codeguardian report --md analysis-report.md

# Generate HTML report
codeguardian report --format html --md report.html

# Generate plain text report
codeguardian report --format text --md report.txt

# Use custom input file
codeguardian report --from custom-results.json --md custom-report.md
```

## See Also
- [`codeguardian check`](check.md) - Run code analysis
- [`codeguardian gh-issue`](gh-issue.md) - Create GitHub issues from results
