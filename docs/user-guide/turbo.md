# turbo

## Synopsis
Run high-performance parallel analysis for large codebases with optimized resource utilization and advanced performance features.

## Description
The turbo command provides CodeGuardian's high-performance analysis mode optimized for large-scale codebases. It uses advanced parallel processing, memory optimization, and streaming analysis to deliver fast, comprehensive results.

## Syntax
```bash
codeguardian turbo [OPTIONS] [PATHS]...
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--paths <PATHS>` | Paths to analyze (files or directories) | `PATH` | `.` | No |
| `--max-parallel <NUM>` | Maximum number of parallel workers | `usize` | `0` | No |
| `--memory-limit <MB>` | Memory limit in MB (0 = no limit) | `usize` | `0` | No |
| `--format <FORMAT>` | Output format (json, human, sarif) | `OutputFormat` | `json` | No |
| `--output <FILE>` | Output file for results (when not specified, automatically placed in `build/analysis-results/turbo/{date}/turbo-results.json`) | `PATH` | `turbo-results.json` | No |
| `--metrics` | Enable metrics output | `FLAG` | `false` | No |
| `--aggressive` | Aggressive analysis mode (more thorough but slower) | `FLAG` | `false` | No |
| `--diff <SPEC>` | Only analyze changed files (git diff) | `STRING` | - | No |
| `--only-staged` | Only analyze staged files | `FLAG` | `false` | No |
| `--fail-on-issues` | Exit with non-zero code if issues are found | `FLAG` | `false` | No |
| `--baseline <FILE>` | Baseline file for drift analysis | `PATH` | - | No |

## Automatic File Placement

When no `--output` option is specified, CodeGuardian automatically organizes result files in dated subfolders for better organization and historical tracking:

```
build/analysis-results/turbo/{YYYY-MM-DD}/turbo-results.json
```

**Benefits:**
- **Prevents Overwrites**: Each run creates a unique dated folder
- **Historical Tracking**: Easy access to previous analysis results
- **Organized Storage**: Command-specific subfolders keep results separated
- **CI/CD Friendly**: Predictable paths for automation scripts

**Example Paths:**
- `build/analysis-results/turbo/2025-09-19/turbo-results.json`
- `build/analysis-results/turbo/2025-09-18/turbo-results.json`

## Examples
```bash
# Turbo analysis of large codebase
codeguardian turbo . --max-parallel 32

# Memory-constrained turbo analysis
codeguardian turbo . --memory-limit 4096 --max-parallel 8

# Turbo analysis with metrics
codeguardian turbo . --metrics --output turbo-metrics.json

# Aggressive turbo analysis for thorough checking
codeguardian turbo . --aggressive --fail-on-issues
```

## Performance Features
- **Adaptive Parallelism**: Automatically scales workers based on system resources
- **Memory Pooling**: Efficient memory management for large file processing
- **Streaming Analysis**: Process large files without loading entirely into memory
- **Intelligent Caching**: Cache analysis results for faster subsequent runs
- **Resource Monitoring**: Real-time monitoring of CPU and memory usage

## See Also
- [`codeguardian check`](check.md) - Standard analysis mode
