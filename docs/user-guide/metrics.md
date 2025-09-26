# metrics

## Synopsis
Analyze and display performance metrics for trained machine learning models used in CodeGuardian's false positive reduction system.

## Description
The metrics command provides comprehensive analysis and reporting of ML model performance metrics. It evaluates model accuracy, precision, recall, and other key performance indicators to ensure optimal false positive reduction while maintaining security analysis effectiveness.

Key capabilities include:
- **Model Performance Analysis**: Detailed accuracy, precision, recall, and F1-score metrics
- **Threshold Optimization**: Recommendations for optimal ML confidence thresholds
- **Comparative Analysis**: Performance comparison across different model versions
- **Export Capabilities**: JSON and structured output for integration with monitoring systems
- **Validation Metrics**: Cross-validation results and statistical significance testing

## Syntax
```bash
codeguardian metrics [OPTIONS] <SUBCOMMAND>
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--model-path <PATH>` | Path to the ML model file | `PATH` | `codeguardian-model.fann` | No |
| `--quiet` | Suppress all output except errors | `FLAG` | `false` | No |

## Subcommands

### Show
Display detailed metrics for the specified model.

```bash
codeguardian metrics show
```

### Export
Export metrics to a JSON file for external analysis or monitoring.

```bash
codeguardian metrics export [OPTIONS]
```

#### Export Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--output <FILE>` | Output file for JSON export | `PATH` | `reports/metrics.json` | No |

## Examples

### Basic Usage
```bash
# Show metrics for default model
codeguardian metrics show

# Show metrics for specific model
codeguardian metrics show --model-path custom-model.fann

# Export metrics to custom location
codeguardian metrics export --output model-metrics.json
```

### Advanced Usage
```bash
# Analyze model performance quietly
codeguardian metrics show --model-path production-model.fann --quiet

# Export metrics for monitoring dashboard
codeguardian metrics export \
  --model-path latest-model.fann \
  --output reports/model-performance-$(date +%Y%m%d).json
```

### Integration with CI/CD
```bash
# Check model performance in CI pipeline
codeguardian metrics show --model-path $MODEL_PATH || exit 1

# Archive metrics for trend analysis
codeguardian metrics export --output metrics/$(git rev-parse --short HEAD).json
```

## Metrics Output

The metrics command provides the following key performance indicators:

### Classification Metrics
- **Accuracy**: Overall correctness of predictions
- **Precision**: True positive rate among positive predictions
- **Recall**: True positive rate among actual positives
- **F1-Score**: Harmonic mean of precision and recall
- **Specificity**: True negative rate

### Threshold Analysis
- **ROC Curve**: Receiver Operating Characteristic analysis
- **AUC Score**: Area Under the Curve for model discrimination
- **Optimal Threshold**: Recommended confidence threshold for best balance
- **Precision-Recall Curve**: Trade-off analysis between precision and recall

### Validation Metrics
- **Cross-Validation Scores**: Performance across different data folds
- **Confidence Intervals**: Statistical uncertainty bounds
- **Statistical Significance**: p-values for performance differences

## Output Format

### JSON Export Structure
```json
{
  "model_info": {
    "path": "codeguardian-model.fann",
    "training_date": "2025-09-25T10:30:00Z",
    "version": "1.0.0"
  },
  "classification_metrics": {
    "accuracy": 0.945,
    "precision": 0.892,
    "recall": 0.867,
    "f1_score": 0.879,
    "specificity": 0.956
  },
  "threshold_analysis": {
    "optimal_threshold": 0.723,
    "auc_score": 0.934,
    "recommended_thresholds": {
      "high_precision": 0.85,
      "balanced": 0.723,
      "high_recall": 0.65
    }
  },
  "validation_results": {
    "cross_validation_mean": 0.938,
    "cross_validation_std": 0.012,
    "confidence_interval_95": [0.914, 0.962]
  }
}
```

## Threshold Recommendations

The metrics command provides threshold recommendations based on different use cases:

- **High Precision**: Minimize false positives (recommended for strict security environments)
- **Balanced**: Optimal trade-off between precision and recall
- **High Recall**: Minimize false negatives (recommended for comprehensive scanning)

## Error Handling

### Common Errors
- **Model File Not Found**: Specified model file does not exist or is not readable
- **Invalid Model Format**: Model file is corrupted or not a valid FANN format
- **Insufficient Test Data**: Not enough test data available for reliable metrics
- **Configuration Error**: Invalid metrics calculation parameters

## Security Considerations
- **Model Validation**: Ensures ML models meet security and performance standards
- **Data Privacy**: Metrics calculation doesn't expose sensitive training data
- **Audit Trail**: All metrics calculations are logged for compliance
- **Access Control**: Metrics access respects model file permissions

## Feature Requirements
- **ML Feature**: Requires the `ml` feature to be enabled during compilation
- **Model File**: Valid trained FANN model file must be available
- **Test Data**: Access to validation/test datasets for metrics calculation

## See Also
- [`codeguardian train`](train.md) - Train ML model for false positive reduction
- [`codeguardian training-data`](training-data.md) - Collect training data for models
- [`codeguardian check`](check.md) - Use trained model in security analysis