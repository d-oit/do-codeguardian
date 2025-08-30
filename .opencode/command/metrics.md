---
description: View and analyze ML model performance metrics and training statistics
---

# metrics

## Synopsis
CodeGuardian's ML model performance analysis command that displays comprehensive metrics, training statistics, and performance insights for trained machine learning models used in false positive reduction and analysis optimization.

## Description
The metrics command provides detailed insights into the performance and effectiveness of CodeGuardian's ML models, including training statistics, inference performance, classification accuracy, and temporal performance trends. It supports multiple output formats and helps optimize model performance for different use cases.

Key capabilities include:
- **Training Metrics**: Detailed training statistics including error rates, convergence data, and training duration
- **Inference Performance**: Real-time performance metrics for model predictions and throughput
- **Classification Accuracy**: Precision, recall, F1-score, and accuracy metrics with confusion matrix analysis
- **Temporal Analysis**: Performance trends over time with alerting for performance degradation
- **Model Validation**: Comprehensive validation metrics for model reliability and effectiveness
- **Export Capabilities**: JSON export for programmatic analysis and integration

## Syntax
```bash
codeguardian metrics [OPTIONS] <COMMAND>
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--model-path <PATH>` | Path to the ML model file | `STRING` | `codeguardian-model.fann` | No |
| `--quiet` | Suppress all output except errors | `FLAG` | `false` | No |

### Commands
- `show`: Display detailed metrics report with all available statistics
- `export`: Export metrics to JSON file for external analysis
- `summary`: Show condensed metrics summary with key performance indicators

## Examples

### Basic Usage
```bash
# Show detailed metrics for default model
codeguardian metrics show

# Show metrics for specific model
codeguardian metrics --model-path my-model.fann show

# Export metrics to JSON file
codeguardian metrics export --output model-performance.json

# Show condensed metrics summary
codeguardian metrics summary
```

### Advanced Usage
```bash
# Analyze model performance after training
codeguardian train --bootstrap --model-path enhanced-model.fann
codeguardian metrics --model-path enhanced-model.fann show

# Export metrics for CI/CD analysis
codeguardian metrics export --output ci-metrics.json --quiet

# Monitor model performance over time
codeguardian metrics show | grep -E "(Accuracy|F1|Training)"
```

## Error Handling

### Common Errors
- **Model File Not Found**: Specified model file does not exist
  ```bash
  codeguardian metrics --model-path nonexistent.fann show
  # Error: Model file not found: nonexistent.fann
  ```

- **Metrics Not Available**: Model exists but no metrics data is available
  ```bash
  codeguardian metrics show
  # Error: ML classifier not enabled or no metrics available
  ```

- **Invalid Model Format**: Model file is corrupted or in invalid format
  ```bash
  codeguardian metrics --model-path corrupted.fann show
  # Error: Failed to load model: invalid format
  ```

### Recovery Procedures
1. **Missing Model**: Train a model first or use the default model path
   ```bash
   codeguardian train --bootstrap
   codeguardian metrics show
   ```

2. **No Metrics Data**: Run analysis with ML model to collect metrics
   ```bash
   codeguardian check . --ml-model codeguardian-model.fann
   codeguardian metrics show
   ```

3. **Corrupted Model**: Retrain the model from scratch
   ```bash
   rm codeguardian-model.fann
   codeguardian train --bootstrap
   ```

## Security Considerations
- **Model File Access**: Read-only access to model files prevents accidental modification
- **Metrics Data Privacy**: Performance metrics don't contain sensitive code content
- **Export Security**: JSON exports can be safely shared for analysis without exposing source code
- **Path Validation**: Model file paths are validated to prevent directory traversal attacks
- **Resource Limits**: Metrics analysis is bounded to prevent resource exhaustion
- **No Credential Exposure**: Metrics data never includes authentication tokens or credentials

## Best Practices

### Security Best Practices
- **Model File Protection**: Store trained models securely and version control them appropriately
- **Metrics Auditing**: Regularly review model performance metrics for signs of degradation
- **Access Control**: Limit access to model files and metrics data based on need-to-know
- **Export Review**: Review exported metrics before sharing with external parties

### Performance Optimization Tips
- **Regular Metrics Monitoring**: Monitor model performance metrics after each training cycle
- **Accuracy Thresholds**: Set minimum accuracy thresholds for production model deployment
- **Training Data Quality**: Use high-quality training data to improve model performance
- **Model Retraining**: Retrain models periodically to maintain accuracy as code patterns evolve

### Common Pitfalls to Avoid
- **Ignoring Performance Degradation**: Don't ignore declining accuracy metrics over time
- **Over-training**: Avoid over-training models which can lead to reduced generalization
- **Insufficient Training Data**: Ensure adequate training data for reliable model performance
- **Model Staleness**: Don't use outdated models without retraining on current code patterns

### Integration Recommendations
- **CI/CD Integration**: Include metrics analysis in CI/CD pipelines for continuous monitoring
- **Automated Alerts**: Set up alerts for significant performance metric changes
- **Model Versioning**: Version control models and track performance across versions
- **Team Dashboards**: Create dashboards for team visibility into model performance

### Maintenance Guidelines
- **Regular Retraining**: Retrain models quarterly or when performance degrades significantly
- **Metrics Archival**: Archive historical metrics for trend analysis and performance tracking
- **Model Validation**: Validate models against known good and bad code patterns
- **Performance Baselines**: Establish performance baselines for comparison and alerting

## See Also
- [`codeguardian train`](train.md) - Train ML models for false positive reduction
- [`codeguardian check`](check.md) - Run analysis with ML model integration
- [ML Model Training Guide](../user-guide/ml-training.md) - Advanced ML training techniques
- [Performance Monitoring](../user-guide/performance-monitoring.md) - Model performance monitoring best practices