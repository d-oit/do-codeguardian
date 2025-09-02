---
description: High-performance analysis for large codebases with parallel processing and streaming
---

# turbo

## Synopsis
CodeGuardian's high-performance analysis mode optimized for large-scale codebases, featuring parallel processing, memory-efficient streaming analysis, intelligent file filtering, and aggressive performance optimizations to deliver fast, comprehensive security analysis at scale.

## Description
The turbo command provides CodeGuardian's performance-optimized analysis engine designed for large codebases and high-throughput environments. It implements advanced parallelization techniques, streaming analysis for large files, and intelligent resource management to deliver significant performance improvements while maintaining analysis accuracy.

Key capabilities include:
- **Parallel Processing**: Multi-core analysis with automatic worker scaling (up to 32 cores)
- **Streaming Analysis**: Memory-efficient processing of large files without loading entire content
- **Intelligent Filtering**: Smart file type detection and size-based filtering
- **Memory Management**: Configurable memory limits and garbage collection optimization
- **Performance Monitoring**: Real-time metrics and throughput reporting
- **Scalable Architecture**: Designed for codebases from thousands to millions of files

## Syntax
```bash
codeguardian turbo [OPTIONS] [PATHS]...
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--paths <PATHS>` | Target directories or files to analyze | `PATH` | `.` | No |
| `--max-parallel <NUM>` | Maximum number of parallel file processors | `usize` | `0` (auto) | No |
| `--memory-limit <MB>` | Memory limit in MB for analysis | `usize` | `1024` | No |
| `--streaming-threshold <MB>` | File size threshold for streaming analysis | `u64` | `5` | No |
| `--max-files <NUM>` | Maximum number of files to analyze (0 = unlimited) | `usize` | `0` | No |
| `--max-file-size <MB>` | Skip files larger than this size | `u64` | `100` | No |
| `--aggressive` | Enable aggressive optimizations (may reduce accuracy slightly) | `FLAG` | `false` | No |
| `--format <FORMAT>` | Output format | `STRING` | `human` | No |
| `--output <FILE>` | Output file for results | `PATH` | - | No |
| `--metrics` | Show detailed performance metrics | `FLAG` | `false` | No |

## Examples

### Basic Usage
```bash
# Analyze current directory with turbo mode
codeguardian turbo

# Analyze specific large codebase
codeguardian turbo /path/to/large/codebase

# Analyze multiple directories
codeguardian turbo src/ tests/ docs/
```

### Advanced Usage
```bash
# High-performance analysis with custom settings
codeguardian turbo . \
  --max-parallel 16 \
  --memory-limit 4096 \
  --max-files 50000 \
  --metrics \
  --format json \
  --output turbo-results.json

# Memory-constrained analysis
codeguardian turbo . \
  --memory-limit 512 \
  --streaming-threshold 1 \
  --max-file-size 50

# CI/CD optimized analysis
codeguardian turbo . \
  --max-parallel 8 \
  --quiet \
  --format json \
  --output ci-results.json
```

## Error Handling

### Common Errors
- **Memory Limit Exceeded**: Analysis requires more memory than allocated
  ```bash
  codeguardian turbo --memory-limit 256 large-file.sql
  # Error: Memory limit exceeded during analysis
  ```

- **File Access Denied**: Cannot read files due to permission restrictions
  ```bash
  codeguardian turbo /root/private/
  # Error: Permission denied (os error 13)
  ```

- **Resource Exhaustion**: System runs out of file handles or threads
  ```bash
  codeguardian turbo --max-parallel 100 huge-codebase/
  # Error: Too many open files
  ```

### Recovery Procedures
1. **Memory Issues**: Increase memory limit or reduce parallel workers
   ```bash
   codeguardian turbo --memory-limit 2048 --max-parallel 4
   ```

2. **Permission Issues**: Run with appropriate permissions or analyze accessible directories
   ```bash
   sudo codeguardian turbo /var/log/
   ```

3. **Resource Limits**: Reduce parallelism and file limits
   ```bash
   codeguardian turbo --max-parallel 2 --max-files 1000
   ```

## Security Considerations
- **Resource Limits**: Strict memory and file size limits prevent resource exhaustion attacks
- **File Type Validation**: Only analyzable file types are processed to prevent malicious content execution
- **Path Sanitization**: All file paths are validated and sanitized
- **Parallel Safety**: Thread-safe analysis prevents race conditions and data corruption
- **Streaming Security**: Large file streaming prevents memory-based attacks
- **Output Validation**: Analysis results are validated before output

## Best Practices

### Security Best Practices
- **Resource Monitoring**: Monitor system resources during large-scale analysis
- **Access Control**: Ensure appropriate file system permissions for analysis
- **Output Validation**: Always validate analysis results before acting on findings
- **Network Isolation**: Run analysis in isolated environments when possible

### Performance Optimization Tips
- **Auto-scaling**: Let CodeGuardian automatically determine optimal parallel workers
- **Memory Tuning**: Adjust memory limits based on system capabilities and workload
- **File Size Limits**: Set appropriate file size limits to balance coverage and performance
- **Streaming Threshold**: Optimize streaming threshold based on typical file sizes

### Common Pitfalls to Avoid
- **Over-parallelization**: Don't set excessively high parallel worker counts
- **Memory Over-allocation**: Avoid allocating more memory than system capacity
- **Ignoring File Limits**: Set reasonable file limits to prevent runaway analysis
- **Mixed Workloads**: Don't mix turbo mode with other resource-intensive processes

### Integration Recommendations
- **CI/CD Pipelines**: Use turbo mode for large-scale CI/CD analysis
- **Scheduled Scans**: Implement regular turbo scans for comprehensive coverage
- **Distributed Analysis**: Combine with distributed systems for massive codebases
- **Performance Monitoring**: Integrate with monitoring systems for analysis metrics

### Maintenance Guidelines
- **System Tuning**: Regularly tune system parameters for optimal turbo performance
- **Resource Planning**: Plan system resources based on expected codebase growth
- **Performance Baselines**: Establish performance baselines for comparison
- **Regular Optimization**: Update turbo settings as codebase and system capabilities evolve

## See Also
- [`codeguardian check`](check.md) - Standard analysis mode for smaller codebases
- [Performance Tuning Guide](../user-guide/performance-tuning.md) - Advanced performance optimization
- [Large Codebase Analysis](../user-guide/large-codebase-analysis.md) - Best practices for large-scale analysis
- [CI/CD Performance](../user-guide/ci-cd-performance.md) - Performance optimization for CI/CD pipelines
