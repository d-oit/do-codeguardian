# CodeGuardian Configuration Guide

## Configuration Files

- `codeguardian.toml` - Main configuration file with production defaults
- `codeguardian.dev.toml` - Development environment overrides
- `codeguardian.prod.toml` - Production environment overrides

## Usage

```bash
# Default configuration
codeguardian analyze

# Development mode
codeguardian analyze --config codeguardian.dev.toml

# Production mode
codeguardian analyze --config codeguardian.prod.toml
```

## Key Settings

### Security
- `max_file_size`: 10MB limit prevents resource exhaustion
- `entropy_threshold`: 4.5 for secret detection sensitivity
- `min_severity`: "low" for comprehensive reporting

### Performance
- `max_parallel_workers`: 4 concurrent workers (adjust based on system)
- `enable_file_caching`: true for faster repeated analyses
- `max_findings_per_file`: 50 to prevent overwhelming output

### Analyzers
All analyzers are enabled by default for comprehensive analysis:
- Security analysis with vulnerability detection
- Performance analysis with complexity checks
- Code quality analysis with duplication detection
- Integrity checking with Blake3 hashing

## Environment Variables

- `CODEGUARDIAN_GITHUB_TOKEN`: For GitHub integration
- `CODEGUARDIAN_GITLAB_TOKEN`: For GitLab integration

## Optimization Recommendations

1. **For CI/CD**: Use production config with `fail_on_issues = true`
2. **For Development**: Use dev config with verbose output
3. **For Large Codebases**: Increase `max_parallel_workers` and `max_memory_file_size`
4. **For Security-Focused**: Enable all analyzers and set low severity thresholds
