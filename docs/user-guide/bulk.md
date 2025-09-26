# bulk

## Synopsis
Perform batch operations across multiple repositories, codebases, and integration systems with controlled concurrency and comprehensive result aggregation.

## Description
The bulk command enables efficient processing of multiple targets simultaneously, supporting repository scanning, codebase processing, integration operations, and consolidated reporting. It provides progress tracking, error handling, and result consolidation for large-scale operations.

Key capabilities include:
- **Multi-Repository Scanning**: Scan multiple repositories for duplicates and issues
- **Batch Codebase Processing**: Process multiple codebases with various operations
- **Integration Operations**: Perform bulk operations on external systems (Jira, GitLab, etc.)
- **Consolidated Reporting**: Generate unified reports from multiple sources
- **Progress Tracking**: Real-time progress monitoring with detailed statistics
- **Error Resilience**: Configurable error handling and continuation options

## Syntax
```bash
codeguardian bulk <SUBCOMMAND> [OPTIONS]
```

## Subcommands

### Scan
Scan multiple repositories for duplicates and issues.

```bash
codeguardian bulk scan [OPTIONS] <REPOSITORIES>...
```

#### Scan Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--repositories <REPOS>` | Repository paths or URLs to scan | `STRING[]` | - | Yes |
| `--format <FORMAT>` | Output format for results | `ReportFormat` | `json` | No |
| `--output <FILE>` | Output file for consolidated results | `PATH` | - | No |
| `--concurrency <NUM>` | Maximum concurrent operations | `usize` | `4` | No |
| `--skip-errors` | Skip repositories that fail to process | `FLAG` | `false` | No |
| `--recursive` | Include subdirectories in scan | `FLAG` | `false` | No |
| `--duplicate-types <TYPES>` | Types of duplicates to detect | `DuplicateType[]` | `code,config,docs` | No |

### Process
Process multiple codebases simultaneously.

```bash
codeguardian bulk process [OPTIONS] <CODEBASES>...
```

#### Process Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--codebases <PATHS>` | Codebase directories to process | `PATH[]` | - | Yes |
| `--operation <OP>` | Processing operation to perform | `ProcessOperation` | `analyze` | No |
| `--output-dir <DIR>` | Output directory for results | `PATH` | - | No |
| `--workers <NUM>` | Maximum parallel workers | `usize` | `8` | No |
| `--continue-on-error` | Continue processing on errors | `FLAG` | `false` | No |

### Integration
Batch operations on integration systems.

```bash
codeguardian bulk integration [OPTIONS]
```

#### Integration Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--system <SYSTEM>` | Integration system to target | `IntegrationType` | - | Yes |
| `--operation <OP>` | Bulk operation to perform | `BulkIntegrationOperation` | - | Yes |
| `--input-file <FILE>` | Input file with operation data | `PATH` | - | Yes |
| `--batch-size <NUM>` | Batch size for operations | `usize` | `10` | No |
| `--dry-run` | Dry run (validate without executing) | `FLAG` | `false` | No |

### Report
Generate consolidated reports from multiple sources.

```bash
codeguardian bulk report [OPTIONS] <SOURCES>...
```

#### Report Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--sources <PATHS>` | Source directories containing reports | `PATH[]` | - | Yes |
| `--output <FILE>` | Output file for consolidated report | `PATH` | - | Yes |
| `--format <FORMAT>` | Report format | `ReportFormat` | `html` | No |
| `--detailed` | Include detailed findings | `FLAG` | `false` | No |
| `--merge-duplicates` | Merge duplicate findings across sources | `FLAG` | `false` | No |

## Examples

### Repository Scanning
```bash
# Scan multiple repositories for duplicates
codeguardian bulk scan \
  --repositories repo1/ repo2/ https://github.com/org/repo3 \
  --output bulk-scan-results.json \
  --concurrency 8 \
  --skip-errors

# Scan with specific duplicate types
codeguardian bulk scan \
  --repositories . \
  --duplicate-types code,dependencies,workflows \
  --recursive \
  --format json
```

### Codebase Processing
```bash
# Analyze multiple codebases
codeguardian bulk process \
  --codebases src/project1 src/project2 src/project3 \
  --operation analyze \
  --output-dir bulk-results \
  --workers 12

# Clean multiple codebases
codeguardian bulk process \
  --codebases codebase1/ codebase2/ codebase3/ \
  --operation clean \
  --continue-on-error
```

### Integration Operations
```bash
# Bulk create Jira issues
codeguardian bulk integration \
  --system jira \
  --operation create-issues \
  --input-file issues.json \
  --batch-size 20

# Dry run for GitLab issue updates
codeguardian bulk integration \
  --system gitlab \
  --operation update-issues \
  --input-file updates.json \
  --dry-run
```

### Report Consolidation
```bash
# Generate consolidated HTML report
codeguardian bulk report \
  --sources results/repo1 results/repo2 results/repo3 \
  --output consolidated-report.html \
  --format html \
  --detailed \
  --merge-duplicates

# Create JSON report for CI/CD
codeguardian bulk report \
  --sources scan-results/ \
  --output ci-report.json \
  --format json
```

## Supported Values

### DuplicateType Values
- `code`: Code duplicates
- `config`: Configuration file duplicates
- `docs`: Documentation duplicates
- `dependencies`: Dependency duplicates
- `workflows`: Workflow/CI duplicates
- `all`: All duplicate types

### ProcessOperation Values
- `analyze`: Security and quality analysis
- `validate`: Validation checks
- `clean`: Code cleanup operations
- `optimize`: Performance optimization
- `report`: Generate reports

### IntegrationType Values
- `jira`: Atlassian Jira
- `confluence`: Atlassian Confluence
- `gitlab`: GitLab
- `github`: GitHub
- `jenkins`: Jenkins CI/CD

### BulkIntegrationOperation Values
- `create-issues`: Create issues/tickets
- `update-issues`: Update existing issues
- `search-duplicates`: Find duplicate issues
- `sync-status`: Synchronize status
- `generate-reports`: Generate reports

## Progress Tracking

The bulk command provides real-time progress tracking:

```
🔍 Starting bulk scan of 5 repositories
████████████████████████████████████████ 100% [00:02:15]
✅ Bulk scan completed

📊 Bulk Operation Summary
========================
Operation: bulk_scan
Total Processed: 5
Successful: 4
Failed: 1
Duration: 135.2s
Success Rate: 80.0%

📈 Summary:
  Total Findings: 1,247
  Repositories: 5
  Files Processed: 12,543
```

## Error Handling

### Common Errors
- **Repository Access Failed**: Cannot access specified repository
- **Invalid Format**: Unsupported output or input format
- **Concurrency Limit Exceeded**: Too many concurrent operations
- **Integration Unavailable**: Target integration system not accessible
- **Resource Exhaustion**: Memory or file handle limits exceeded

### Error Recovery
- **Skip Errors**: Continue processing other items when one fails
- **Retry Logic**: Automatic retry for transient failures
- **Partial Results**: Save successful results even if some operations fail
- **Detailed Logging**: Comprehensive error reporting for troubleshooting

## Performance Optimization

### Concurrency Control
- **Adaptive Scaling**: Automatically adjust worker count based on system resources
- **Resource Limits**: Prevent resource exhaustion with configurable limits
- **Load Balancing**: Distribute work evenly across available workers

### Memory Management
- **Streaming Processing**: Process large files without loading entirely in memory
- **Result Batching**: Process results in batches to manage memory usage
- **Cleanup**: Automatic cleanup of temporary files and resources

## Output Formats

### JSON Output Structure
```json
{
  "operation_type": "bulk_scan",
  "total_processed": 5,
  "successful": 4,
  "failed": 1,
  "duration_seconds": 135.2,
  "results": [
    {
      "success": true,
      "item_id": "repo1",
      "operation_index": 0,
      "message": "Scanned 2456 files, found 89 findings",
      "data": { ... }
    }
  ],
  "errors": [
    {
      "item": "repo5",
      "error": "Repository access denied"
    }
  ],
  "summary": {
    "total_findings": 1247,
    "duplicate_count": 234,
    "repositories_processed": 5,
    "files_processed": 12543,
    "performance_metrics": {
      "avg_processing_time_ms": 1250.5,
      "throughput_files_per_second": 92.8,
      "memory_usage_mb": 512.3,
      "cpu_utilization_percent": 78.5
    }
  }
}
```

## Security Considerations
- **Access Control**: Respect repository and system access permissions
- **Data Sanitization**: Remove sensitive information from bulk results
- **Audit Trail**: Log all bulk operations for compliance
- **Rate Limiting**: Respect API rate limits for integration systems
- **Secure Storage**: Encrypt sensitive data in output files

## Integration with CI/CD

### GitHub Actions Example
```yaml
- name: Bulk Security Scan
  run: |
    codeguardian bulk scan \
      --repositories . \
      --output bulk-results.json \
      --concurrency 4

- name: Generate Report
  run: |
    codeguardian bulk report \
      --sources bulk-results.json \
      --output security-report.html \
      --format html
```

### Jenkins Pipeline Example
```groovy
stage('Bulk Analysis') {
    steps {
        sh '''
            codeguardian bulk process \
              --codebases ${{CODEBASES}} \
              --operation analyze \
              --output-dir bulk-analysis \
              --workers 8
        '''
    }
}
```

## See Also
- [`codeguardian check`](check.md) - Single codebase analysis
- [`codeguardian report`](report.md) - Generate reports from analysis results
- [`codeguardian integrations`](integrations.md) - External system integrations