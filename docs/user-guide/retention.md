# Retention Policy Management

CodeGuardian includes a comprehensive retention policy engine that automatically manages analysis results, ensuring efficient storage usage while maintaining data integrity.

## Overview

The retention system provides:

- **Automatic cleanup** of old analysis results based on configurable policies
- **Data integrity verification** using BLAKE3 checksums
- **Size-based cleanup** to prevent disk space exhaustion
- **Repair mechanisms** for corrupted data
- **Integrity reporting** for audit and monitoring

## Configuration

Configure retention policies in your `codeguardian.toml`:

```toml
[retention]
enabled = true
results_dir = "analysis-results"
max_age_days = 30
max_size_mb = 500
min_results_to_keep = 10
enable_integrity_check = true
integrity_check_frequency_days = 7
enable_auto_repair = false
backup_corrupted_files = true
backup_dir = "analysis-results/backup"
enable_integrity_reporting = true
integrity_report_path = "analysis-results/integrity-report.json"
```

### Configuration Options

| Option | Description | Default |
|--------|-------------|---------|
| `enabled` | Enable retention policy engine | `true` |
| `results_dir` | Directory containing analysis results | `"analysis-results"` |
| `max_age_days` | Maximum age of results to keep (days) | `30` |
| `max_size_mb` | Maximum total size of results directory (MB) | `500` |
| `min_results_to_keep` | Minimum number of results to retain | `10` |
| `enable_integrity_check` | Enable data integrity verification | `true` |
| `integrity_check_frequency_days` | How often to check integrity (days) | `7` |
| `enable_auto_repair` | Automatically attempt to repair corrupted data | `false` |
| `backup_corrupted_files` | Backup corrupted files before repair | `true` |
| `backup_dir` | Directory for corrupted file backups | `"analysis-results/backup"` |
| `enable_integrity_reporting` | Generate integrity reports | `true` |
| `integrity_report_path` | Path for integrity report file | `"analysis-results/integrity-report.json"` |

## Usage

### Command Line Interface

Use the `retention` command to manage retention policies:

```bash
# Show retention status and current statistics
codeguardian retention

# Run cleanup operation
codeguardian retention --cleanup

# Check data integrity only
codeguardian retention --check-integrity

# Generate integrity report
codeguardian retention --report-integrity

# Dry run cleanup (show what would be cleaned)
codeguardian retention --cleanup --dry-run
```

### Automatic Operation

When enabled, retention operations run automatically during analysis operations. You can also integrate retention checks into your CI/CD pipeline:

```bash
# Daily integrity check
codeguardian retention --check-integrity --report-integrity

# Weekly cleanup
codeguardian retention --cleanup
```

## Data Integrity

CodeGuardian uses BLAKE3 cryptographic hashing to ensure data integrity:

- **Checksum Verification**: Each result file is hashed and verified
- **Corruption Detection**: Automatically identifies corrupted files
- **Backup and Recovery**: Corrupted files can be backed up before repair attempts
- **Audit Trail**: Integrity reports provide complete audit trails

### Integrity Report Format

```json
{
  "total_files": 150,
  "corrupted_files": ["/path/to/corrupted/result.json"],
  "valid_checksums": {
    "/path/to/valid/result.json": "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3"
  }
}
```

## Cleanup Policies

### Age-Based Cleanup

Files older than `max_age_days` are automatically removed, except for the minimum number of results specified by `min_results_to_keep`.

### Size-Based Cleanup

When the total size of the results directory exceeds `max_size_mb`, older files are removed to bring the size back within limits.

### Priority Order

1. **Preserve Minimum**: Always keep at least `min_results_to_keep` results
2. **Age First**: Remove files exceeding age limits
3. **Size Second**: If still over size limit, remove oldest files

## Security Considerations

- **Secure Hashing**: Uses BLAKE3 for cryptographic integrity verification
- **Safe Deletion**: Files are permanently removed - ensure backups if needed
- **Access Control**: Retention operations respect file system permissions
- **Audit Logging**: All operations are logged for security auditing

## Best Practices

1. **Regular Monitoring**: Enable integrity reporting and review regularly
2. **Backup Strategy**: Configure backup directory for corrupted files
3. **Size Limits**: Set appropriate size limits based on your storage capacity
4. **Retention Period**: Balance between storage efficiency and historical analysis needs
5. **CI/CD Integration**: Include retention checks in your automated pipelines

## Troubleshooting

### Common Issues

**High Disk Usage**
- Increase `max_age_days` or decrease `max_size_mb`
- Run manual cleanup: `codeguardian retention --cleanup`

**Integrity Failures**
- Check file system for corruption
- Review backup directory for corrupted files
- Disable auto-repair if issues persist

**Performance Impact**
- Adjust `integrity_check_frequency_days` for less frequent checks
- Run integrity checks during off-peak hours

### Logs and Diagnostics

Retention operations are logged with detailed information:

```
INFO  codeguardian::core::retention > Starting retention cleanup
INFO  codeguardian::core::retention > Removed 25 files by age (45.2 MB freed)
INFO  codeguardian::core::retention > Integrity check: 150 files checked, 0 corrupted
```

## API Reference

For programmatic access, use the `RetentionManager`:

```rust
use codeguardian::core::RetentionManager;
use codeguardian::config::retention::RetentionConfig;

let config = RetentionConfig::default();
let manager = RetentionManager::new(config);

// Run cleanup
let report = manager.cleanup().await?;

// Check integrity
let integrity_report = manager.check_integrity().await?;
```
