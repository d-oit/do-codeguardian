---
description: Train ML models for enhanced false positive reduction and analysis accuracy
---

# train

## Synopsis
CodeGuardian's ML model training command that creates and trains neural networks to improve analysis accuracy by learning from codebase patterns, reducing false positives by up to 80% through advanced machine learning techniques and comprehensive training data generation.

## Description
The train command implements CodeGuardian's advanced ML pipeline, using FANN (Fast Artificial Neural Network) library to train models that can distinguish between true security issues and false positives. It supports multiple training modes including bootstrap training from existing codebases, synthetic data generation, and incremental model improvement.

Key capabilities include:
- **Bootstrap Training**: Generate training data automatically from codebase analysis
- **Synthetic Data Generation**: Create artificial training examples to improve model generalization
- **Balanced Training**: Ensure equal representation of positive and negative examples
- **Enhanced Feature Extraction**: 12-dimensional feature vectors for comprehensive pattern recognition
- **Adaptive Learning**: Dynamic learning rate adjustment and early stopping for optimal convergence
- **Model Persistence**: Save and load trained models for reuse across analysis sessions

## Syntax
```bash
codeguardian train [OPTIONS] [PATHS]...
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--paths <PATHS>` | Paths to analyze for training data | `PATH` | `.` | No |
| `--model-path <PATH>` | Path to save/load the trained model | `STRING` | `codeguardian-model.fann` | No |
| `--dataset <PATH>` | Path to save/load training dataset | `STRING` | - | No |
| `--epochs <NUM>` | Number of training epochs | `u32` | `100` | No |
| `--bootstrap` | Generate training data from codebase analysis | `FLAG` | `false` | No |
| `--synthetic-samples <NUM>` | Number of synthetic training samples | `usize` | `0` | No |
| `--balanced` | Use balanced training data (equal true/false positives) | `FLAG` | `false` | No |
| `--quiet` | Suppress all output except errors | `FLAG` | `false` | No |
| `--verbose` | Show detailed training progress | `FLAG` | `false` | No |

## Examples

### Basic Usage
```bash
# Bootstrap training from current codebase
codeguardian train --bootstrap

# Train with specific paths
codeguardian train src/ tests/ --bootstrap

# Generate synthetic training data
codeguardian train --synthetic-samples 1000

# Balanced training for better accuracy
codeguardian train --bootstrap --balanced
```

### Advanced Usage
```bash
# Complete training pipeline with custom model
codeguardian train . \
  --bootstrap \
  --synthetic-samples 500 \
  --balanced \
  --epochs 200 \
  --model-path enhanced-model.fann \
  --dataset training-data.json \
  --verbose

# Incremental training on existing model
codeguardian train . \
  --bootstrap \
  --model-path existing-model.fann

# Training for CI/CD with specific dataset
codeguardian train . \
  --dataset ci-training-data.json \
  --model-path ci-model.fann \
  --quiet
```

## Error Handling

### Common Errors
- **Insufficient Training Data**: Not enough examples for effective training
  ```bash
  codeguardian train .
  # Warning: Dataset has only 5 examples. Consider using --bootstrap or --synthetic-samples
  ```

- **Model File Access Error**: Cannot write to specified model path
  ```bash
  codeguardian train --model-path /root/model.fann
  # Error: Permission denied (os error 13)
  ```

- **Configuration Missing**: No configuration file found for analysis
  ```bash
  codeguardian train --bootstrap
  # Warning: No configuration file found, using defaults
  ```

### Recovery Procedures
1. **Data Insufficiency**: Use bootstrap mode or generate synthetic samples
   ```bash
   codeguardian train --bootstrap --synthetic-samples 1000
   ```

2. **Permission Issues**: Use a writable directory for model files
   ```bash
   codeguardian train --model-path ./my-model.fann
   ```

3. **Configuration Issues**: Create a configuration file first
   ```bash
   codeguardian init --default
   codeguardian train --bootstrap
   ```

## Security Considerations
- **Training Data Privacy**: Training data is generated from local codebase analysis only
- **Model File Security**: Trained models can be safely shared without exposing source code
- **Path Validation**: All file paths are validated to prevent directory traversal
- **Resource Limits**: Training is bounded to prevent resource exhaustion attacks
- **No External Data**: Training uses only local codebase data, no external dependencies
- **Model Integrity**: Trained models include integrity checks and version information

## Best Practices

### Security Best Practices
- **Local Training Only**: Train models only on trusted, local codebases
- **Model Validation**: Always validate trained models before production use
- **Access Control**: Limit access to trained models based on organizational policies
- **Regular Retraining**: Retrain models periodically to adapt to new security patterns

### Performance Optimization Tips
- **Bootstrap for Quick Start**: Use `--bootstrap` for initial model training on existing codebases
- **Synthetic Data Enhancement**: Add synthetic samples to improve model generalization
- **Balanced Training**: Use `--balanced` for better performance on imbalanced datasets
- **Incremental Training**: Build upon existing models rather than training from scratch

### Common Pitfalls to Avoid
- **Insufficient Training Data**: Don't train models with too few examples (< 10)
- **Unbalanced Datasets**: Avoid highly imbalanced training data without using `--balanced`
- **Over-training**: Don't use excessively high epoch counts without monitoring convergence
- **Ignoring Warnings**: Pay attention to training warnings about data quality and convergence

### Integration Recommendations
- **CI/CD Pipeline Integration**: Include model training in automated pipelines
- **Model Versioning**: Version control trained models for reproducibility
- **Team Collaboration**: Share trained models across team members for consistent analysis
- **Performance Monitoring**: Monitor model performance metrics after deployment

### Maintenance Guidelines
- **Quarterly Retraining**: Retrain models every quarter to maintain accuracy
- **Performance Tracking**: Track model performance metrics over time
- **Dataset Archival**: Archive training datasets for future reference and improvement
- **Model Validation**: Regularly validate models against known good and bad patterns

## See Also
- [`codeguardian check`](check.md) - Use trained models in analysis
- [`codeguardian metrics`](metrics.md) - Analyze model performance after training
- [ML Training Guide](../user-guide/ml-training.md) - Advanced training techniques and best practices
- [Model Optimization](../user-guide/model-optimization.md) - Optimizing ML models for production use
