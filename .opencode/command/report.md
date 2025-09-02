---
description: Generate formatted reports from CodeGuardian analysis results
---

# report

## Synopsis
Convert CodeGuardian analysis results to various output formats (Markdown, HTML, Text) for reporting and documentation purposes.

## Description
The report command transforms JSON analysis results from CodeGuardian scans into human-readable formats suitable for documentation, CI/CD pipelines, and team communication. It supports multiple output formats with rich formatting, severity-based organization, and comprehensive metadata inclusion.

Key capabilities include:
- **Multiple Output Formats**: Markdown, HTML, and plain text formats
- **Rich Content**: Includes findings summary, severity breakdowns, and detailed issue descriptions
- **Flexible Output**: Save to files or display to stdout
- **Security-First**: Handles sensitive data appropriately and includes security considerations
- **CI/CD Ready**: Designed for automated reporting in continuous integration workflows

## Syntax
```bash
codeguardian report [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--from <FILE>` | Input results file path | Path | `results.json` | No |
| `--md <FILE>` | Output file path for generated report | Path | - | No |
| `--format <FORMAT>` | Output format (markdown, html, text) | Enum | `markdown` | No |
| `--config <FILE>` | Configuration file path | Path | `codeguardian.toml` | No |
| `--verbose` | Enable verbose output | Flag | - | No |
| `--quiet` | Suppress all output except errors | Flag | - | No |

### Format Options
- **`markdown`**: GitHub-flavored Markdown with tables, emojis, and structured sections
- **`html`**: Styled HTML with CSS for web viewing and printing
- **`text`**: Plain text format for terminals and simple documentation

## Examples

### Basic Usage
```bash
# Generate Markdown report from default results
codeguardian report

# Generate HTML report
codeguardian report --format html --md security-audit.html

# Generate plain text report
codeguardian report --format text --md audit.txt
```

### Advanced Usage
```bash
# Complete analysis and reporting workflow
codeguardian check . \
  --format json \
  --out analysis-results.json \
  --config security.toml \
  --parallel 8

codeguardian report \
  --from analysis-results.json \
  --md security-report.md \
  --format markdown

# Generate multiple formats from same results
codeguardian report --from results.json --format markdown --md report.md
codeguardian report --from results.json --format html --md report.html
codeguardian report --from results.json --format text --md report.txt

# Custom input file with verbose output
codeguardian report --from custom-results.json --md custom-report.md --verbose
```

## Error Handling

### Common Errors
- **File Not Found Error**: Input results file does not exist
  ```bash
  codeguardian report --from nonexistent.json
  # Error: No such file or directory (os error 2)
  ```

- **Invalid JSON Format Error**: Results file is not valid JSON or corrupted
  ```bash
  codeguardian report --from corrupted.json
  # Error: expected value at line 1 column 1
  ```

- **Permission Denied Error**: Insufficient permissions to write to output directory
  ```bash
  codeguardian report --md /root/report.md
  # Error: Permission denied (os error 13)
  ```

- **Unsupported Format Error**: Invalid format specified
  ```bash
  codeguardian report --format invalid
  # Error: Invalid value 'invalid' for '--format <FORMAT>'
  ```

### Recovery Procedures
1. **File Issues**: Ensure results file exists from previous analysis
   ```bash
   codeguardian check . --format json --out results.json
   codeguardian report --from results.json --md report.md
   ```

2. **Permission Issues**: Use current directory or user-writable location
   ```bash
   codeguardian report --md ./report.md
   codeguardian report --md ~/reports/security-report.md
   ```

3. **Format Issues**: Use valid format option
   ```bash
   codeguardian report --format markdown --md report.md
   ```

4. **Validation**: Use verbose flag to see detailed operation progress
   ```bash
   codeguardian report --from results.json --md report.md --verbose
   ```

## Security Considerations
- **File Path Security**: Commands validate and sanitize file paths to prevent directory traversal
- **JSON Parsing**: Secure JSON parsing prevents malicious payload execution
- **Size Limits**: Large result files are handled efficiently to prevent resource exhaustion
- **Sensitive Data Handling**: Reports exclude sensitive configuration details
- **Safe HTML Generation**: HTML output uses safe encoding to prevent XSS
- **File Permissions**: Generated reports have appropriate file permissions (644)

## Best Practices

### Security Considerations
- **Report Sanitization**: Always review generated reports for sensitive data before sharing
- **Access Control**: Store reports in secure locations with appropriate permissions
- **Content Validation**: Verify report content matches expected analysis scope
- **Distribution Security**: Use encrypted channels for sensitive report distribution

### Performance Optimization Tips
- **Format Selection**: Choose Markdown for GitHub integration, HTML for web viewing
- **Batch Processing**: Generate multiple formats from single analysis run
- **Incremental Updates**: Use existing results files to avoid re-analysis
- **Output Compression**: Compress large reports for storage and transfer efficiency

### Common Pitfalls to Avoid
- **Missing Results File**: Ensure analysis results exist before generating reports
- **Incorrect File Paths**: Use absolute paths or verify relative path correctness
- **Format Mismatches**: Match report format to target audience (Markdown for developers, HTML for management)
- **Large Report Handling**: Split very large reports or use summary formats
- **Permission Conflicts**: Ensure write permissions for output directories

### Integration Recommendations
- **CI/CD Pipelines**: Integrate report generation into automated build processes
- **Documentation Systems**: Publish HTML reports to internal documentation platforms
- **Team Communication**: Use Markdown reports in pull requests and issues
- **Compliance Workflows**: Generate standardized reports for security audits
- **Dashboard Integration**: Feed report data into monitoring and alerting systems

### Maintenance Guidelines
- **Template Updates**: Keep report templates updated with new finding types
- **Format Standards**: Maintain consistent formatting across different report types
- **Archival Strategy**: Implement report retention policies for compliance
- **Quality Assurance**: Regularly validate report accuracy against known test cases
- **User Feedback**: Incorporate team feedback to improve report usefulness

## See Also
- [`codeguardian check`](check.md) - Run code analysis and generate results
- [`codeguardian gh-issue`](gh-issue.md) - Create GitHub issues from analysis results
- [`codeguardian init`](init.md) - Initialize CodeGuardian configuration
- [`codeguardian turbo`](turbo.md) - High-performance analysis for large codebases
- [Configuration Guide](../configuration.md) - Configure analysis settings
- [CI/CD Setup Guide](../user-guide/ci-cd-setup.md) - Integrate with CI/CD pipelines
