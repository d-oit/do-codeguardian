---
title: "debug mode"
description: "Enable debug mode for detailed CodeGuardian analysis and troubleshooting"
category: "Troubleshooting and Diagnostics"
tags: ["debug", "troubleshooting", "diagnostics", "analysis", "logging"]
---

# debug mode

Enable comprehensive debug mode for CodeGuardian analysis, providing detailed logging, performance metrics, and diagnostic information to help troubleshoot issues and optimize performance.

## Synopsis

```bash
codeguardian debug mode [OPTIONS] [COMMAND]
```

## Description

The `debug mode` command enables enhanced debugging capabilities for CodeGuardian, including verbose logging, performance profiling, memory analysis, and detailed diagnostic output. This mode is essential for troubleshooting analysis issues, performance problems, and configuration errors.

### Key Features

- **Verbose logging**: Detailed logging of all analysis steps
- **Performance profiling**: Comprehensive performance metrics
- **Memory analysis**: Memory usage tracking and leak detection
- **Diagnostic output**: Detailed diagnostic information and reports
- **Interactive debugging**: Step-through analysis capabilities

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--enable` | Enable debug mode | boolean | false | No |
| `--level` | Debug level (info,debug,trace) | string | debug | No |
| `--output` | Debug output file | string | debug.log | No |
| `--profile` | Enable performance profiling | boolean | false | No |
| `--memory` | Enable memory analysis | boolean | false | No |
| `--interactive` | Enable interactive debugging | boolean | false | No |
| `--timeout` | Debug session timeout (seconds) | number | 300 | No |
| `--filter` | Debug filter patterns | string[] | [] | No |

## Commands

### enable
Enable debug mode for analysis.

```bash
codeguardian debug mode enable [OPTIONS]
```

### disable
Disable debug mode.

```bash
codeguardian debug mode disable [OPTIONS]
```

### status
Show current debug mode status.

```bash
codeguardian debug mode status [OPTIONS]
```

### analyze
Analyze debug output and generate reports.

```bash
codeguardian debug mode analyze [OPTIONS]
```

## Examples

### Enable Basic Debug Mode

```bash
# Enable debug mode for analysis
codeguardian debug mode enable
```

### Enable Comprehensive Debugging

```bash
# Enable full debugging with profiling
codeguardian debug mode enable \
  --level trace \
  --profile \
  --memory \
  --output detailed-debug.log
```

### Run Analysis with Debug Mode

```bash
# Run analysis with debug mode enabled
codeguardian check --verbose --debug
```

### Analyze Debug Output

```bash
# Analyze debug log file
codeguardian debug mode analyze --input debug.log --output analysis-report.md
```

### Interactive Debugging Session

```bash
# Start interactive debugging session
codeguardian debug mode enable --interactive --timeout 600
```

## Debug Output Analysis

### Performance Profiling Output

```json
{
  "performance_profile": {
    "total_analysis_time": "45.2s",
    "memory_peak": "256MB",
    "cpu_usage": "85%",
    "analysis_breakdown": {
      "file_parsing": "12.3s",
      "security_checks": "18.7s",
      "dependency_analysis": "8.9s",
      "report_generation": "5.3s"
    },
    "bottlenecks": [
      {
        "component": "security_checks",
        "duration": "18.7s",
        "recommendation": "Consider parallel processing for security checks"
      }
    ]
  }
}
```

### Memory Analysis Output

```json
{
  "memory_analysis": {
    "peak_usage": "256MB",
    "average_usage": "128MB",
    "memory_leaks": [],
    "allocation_breakdown": {
      "file_cache": "64MB",
      "analysis_engine": "128MB",
      "report_generator": "32MB",
      "other": "32MB"
    },
    "recommendations": [
      "Consider increasing cache size limit",
      "Implement memory pooling for analysis engine"
    ]
  }
}
```

### Diagnostic Report

```markdown
# CodeGuardian Debug Analysis Report

## Summary
- Analysis completed successfully
- Total execution time: 45.2 seconds
- Peak memory usage: 256 MB
- Issues found: 3 warnings, 0 errors

## Performance Analysis

### Timing Breakdown
- File parsing: 12.3s (27%)
- Security analysis: 18.7s (41%)
- Dependency analysis: 8.9s (20%)
- Report generation: 5.3s (12%)

### Recommendations
1. **Parallel Processing**: Security analysis is the bottleneck. Consider enabling parallel processing.
2. **Caching**: Implement more aggressive caching to reduce file parsing time.
3. **Memory Optimization**: Peak memory usage is high. Consider streaming for large files.

## Memory Analysis

### Memory Usage
- Peak: 256 MB
- Average: 128 MB
- No memory leaks detected

### Allocation by Component
- Analysis Engine: 128 MB (50%)
- File Cache: 64 MB (25%)
- Report Generator: 32 MB (12.5%)
- Other: 32 MB (12.5%)

## Issues and Warnings

### Warning 1: Slow Analysis
- Location: security_checks.rs:145
- Description: Regex compilation taking excessive time
- Recommendation: Pre-compile regex patterns

### Warning 2: High Memory Usage
- Location: file_parser.rs:89
- Description: Large file loaded entirely into memory
- Recommendation: Implement streaming parser

### Warning 3: Inefficient Loop
- Location: dependency_analyzer.rs:234
- Description: Nested loop with O(n²) complexity
- Recommendation: Optimize algorithm or use more efficient data structures
```

## Interactive Debugging

### Interactive Commands

When interactive mode is enabled, you can use the following commands:

```bash
# Start analysis step by step
step

# Continue to next breakpoint
continue

# Show current analysis state
status

# Display variable values
print <variable>

# Set breakpoint
breakpoint <file>:<line>

# Show performance metrics
perf

# Show memory usage
memory

# Generate diagnostic report
diagnose

# Exit interactive mode
exit
```

### Interactive Session Example

```bash
$ codeguardian debug mode enable --interactive
Debug mode enabled. Starting interactive session...

codeguardian-debug> step
Stepping to next analysis phase...
Current phase: File Parsing
Files processed: 0/150

codeguardian-debug> continue
Continuing analysis...
Phase completed: File Parsing (12.3s)

codeguardian-debug> perf
Performance Metrics:
- Current phase: Security Analysis
- Time elapsed: 18.7s
- Memory usage: 192MB
- CPU usage: 85%

codeguardian-debug> diagnose
Generating diagnostic report...
Report saved to: debug-diagnostic-2025-01-15.md

codeguardian-debug> exit
Exiting debug mode...
```

## Best Practices

### Debug Mode Usage

- **Targeted debugging**: Use specific debug levels for focused analysis
- **Resource monitoring**: Monitor system resources during debug sessions
- **Log management**: Manage debug log sizes to prevent disk space issues
- **Performance impact**: Be aware of performance impact in production environments

### Troubleshooting Strategy

- **Start simple**: Begin with basic debug mode and increase verbosity as needed
- **Isolate issues**: Use debug filters to focus on specific components
- **Compare runs**: Compare debug output between working and failing runs
- **Document findings**: Document debugging findings for future reference

### Performance Considerations

- **Debug overhead**: Debug mode adds significant performance overhead
- **Resource usage**: Monitor CPU, memory, and disk usage during debug sessions
- **Log rotation**: Implement log rotation for long-running debug sessions
- **Cleanup**: Clean up debug files and logs after analysis

## Error Handling

### Common Issues

- **Debug file permissions**: Ensure write permissions for debug output files
  ```bash
  chmod 644 debug.log
  ```

- **Disk space**: Monitor disk space for large debug log files
  ```bash
  df -h
  du -sh debug.log
  ```

- **Performance impact**: Debug mode may slow down analysis significantly
  ```bash
  # Use timeout to prevent runaway processes
  timeout 300 codeguardian debug mode enable
  ```

### Troubleshooting

1. **Check debug file creation**:
   ```bash
   ls -la debug.log
   ```

2. **Verify debug mode status**:
   ```bash
   codeguardian debug mode status
   ```

3. **Monitor system resources**:
   ```bash
   top -p $(pgrep codeguardian)
   ```

4. **Analyze debug output**:
   ```bash
   codeguardian debug mode analyze --input debug.log
   ```

## Integration with CI/CD

### GitHub Actions Integration

```yaml
# .github/workflows/debug-analysis.yml
name: Debug Analysis
on:
  workflow_dispatch:
    inputs:
      debug_level:
        description: 'Debug level'
        required: true
        default: 'debug'
      enable_profiling:
        description: 'Enable profiling'
        required: false
        default: true

jobs:
  debug:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run Debug Analysis
        run: |
          codeguardian debug mode enable \
            --level ${{ github.event.inputs.debug_level }} \
            --profile ${{ github.event.inputs.enable_profiling }} \
            --output debug-analysis.log
      - name: Analyze Debug Output
        run: |
          codeguardian debug mode analyze \
            --input debug-analysis.log \
            --output debug-report.md
      - name: Upload Debug Report
        uses: actions/upload-artifact@v4
        with:
          name: debug-report
          path: debug-report.md
```

### GitLab CI Integration

```yaml
# .gitlab-ci.yml
debug_analysis:
  stage: debug
  script:
    - codeguardian debug mode enable --level trace --profile --memory
    - codeguardian debug mode analyze --input debug.log --output debug-report.md
  artifacts:
    paths:
      - debug.log
      - debug-report.md
    expire_in: 1 week
  only:
    - manual
```

## See Also

- [`codeguardian debug logs`](debug-logs.md) - Analyze log files
- [`codeguardian debug profile`](debug-profile.md) - Profile analysis performance
- [`codeguardian debug health`](debug-health.md) - Run health checks
- [`codeguardian performance monitor`](../../../performance/performance-monitor.md) - Monitor performance