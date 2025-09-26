# 🎯 Threshold Tuning Guide

## Synopsis

The `tune-thresholds` command provides intelligent threshold tuning for different deployment environments, enabling optimized monitoring and alerting based on historical performance data.

## Description

CodeGuardian's threshold tuning system automatically adjusts monitoring thresholds for different environments (development, staging, production, enterprise) based on performance metrics, security requirements, and expected load profiles. This ensures optimal alert sensitivity while minimizing false positives.

## Syntax

```bash
codeguardian tune-thresholds [OPTIONS]
```

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `-e, --environment` | Environment to tune thresholds for | String | `production` | No |
| `--show-current` | Show current threshold configurations | Flag | - | No |
| `--recommend` | Generate tuning recommendations based on historical data | Flag | - | No |
| `--apply-recommendations` | Apply recommended thresholds automatically | Flag | - | No |
| `--metrics-file` | Path to historical metrics data (JSON file) | Path | - | No |
| `--output-format` | Output format for recommendations (json, table, yaml) | String | `table` | No |
| `--confidence-threshold` | Minimum confidence level for applying recommendations (0.0-1.0) | Float | `0.8` | No |
| `--export-file` | Export tuned thresholds to file | Path | - | No |
| `--import-file` | Import thresholds from file | Path | - | No |
| `--analyze` | Show detailed analysis of current performance | Flag | - | No |

## Examples

### Show Current Thresholds

Display current threshold configurations for the production environment:

```bash
codeguardian tune-thresholds --environment production --show-current
```

### Generate Recommendations

Generate tuning recommendations based on historical metrics:

```bash
codeguardian tune-thresholds --environment staging --recommend --metrics-file metrics.json
```

### Apply Recommendations Automatically

Apply high-confidence recommendations automatically:

```bash
codeguardian tune-thresholds --environment production --apply-recommendations --confidence-threshold 0.9
```

### Export Tuned Thresholds

Export tuned thresholds to a configuration file:

```bash
codeguardian tune-thresholds --environment production --export-file production-thresholds.json
```

### Import Thresholds

Import threshold configurations from a file:

```bash
codeguardian tune-thresholds --import-file staging-thresholds.json
```

### Detailed Analysis

Perform detailed analysis of current performance:

```bash
codeguardian tune-thresholds --environment production --analyze --output-format json
```

## Environment Profiles

CodeGuardian supports the following predefined environment profiles:

### Development Environment
- **Performance**: Relaxed thresholds for development workflow
- **Security**: Basic security monitoring
- **Load**: Single developer usage patterns

### Staging Environment  
- **Performance**: Production-like thresholds with tolerance for testing
- **Security**: Enhanced security monitoring
- **Load**: Moderate concurrent usage

### Production Environment
- **Performance**: Strict performance requirements
- **Security**: Maximum security monitoring
- **Load**: High concurrent usage patterns

### Enterprise Environment
- **Performance**: Ultra-strict enterprise-grade requirements
- **Security**: Zero-tolerance security policies
- **Load**: Enterprise-scale traffic patterns

## Output Formats

### Table Format (Default)
Human-readable table showing recommendations with confidence scores.

### JSON Format
Machine-readable format suitable for automation:

```json
{
  "environment": "production",
  "recommendations": [
    {
      "metric": "response_time",
      "current_threshold": 100,
      "recommended_threshold": 85,
      "confidence": 0.92,
      "rationale": "Historical data shows 95th percentile at 75ms"
    }
  ]
}
```

### YAML Format
Configuration-friendly format for infrastructure as code.

## Confidence Scoring

Recommendations include confidence scores (0.0-1.0) based on:

- **Data Quality**: Amount and consistency of historical data
- **Statistical Significance**: Confidence intervals and variance
- **Environment Stability**: Historical performance consistency
- **External Factors**: Known system changes or deployments

## See Also

- [Configuration Guide](configuration.md) - Environment configuration
- [Metrics Guide](metrics.md) - Performance metrics collection
- [Release Monitoring](release-monitoring-configuration.md) - Continuous monitoring setup