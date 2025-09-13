# 🤖 Machine Learning Guide

CodeGuardian uses machine learning to enhance duplicate detection accuracy and reduce false positives. This guide covers ML configuration, training, and optimization.

## Overview

The ML system provides intelligent classification of code analysis findings, improving the accuracy of duplicate detection through:

- False positive reduction using neural networks
- Feature extraction from code patterns
- Online learning from user feedback
- Multi-language AST analysis (when enabled)

## Prerequisites

### Feature Requirements

```toml
[features]
ml = ["fann"]                    # Basic ML support
ml-enhanced = ["ml", "ast"]      # Enhanced with AST analysis
```

### Dependencies

- **FANN**: Fast Artificial Neural Network library
- **AST Analysis**: Requires `syn` and `quote` crates (optional)

## Configuration

### Basic ML Configuration

```toml
[ml]
enabled = true
model_path = "enhanced-model.fann"
model_type = "ruf_fann"
online_learning = true
learning_rate = 0.01
epochs = 2000
bootstrap_training = true
confidence_threshold = 0.8
auto_threshold_adjustment = true
feedback_loop = true
```

### Enhanced Configuration

```toml
[ml]
# Model settings
model_path = "models/codeguardian-v2.fann"
model_type = "ruf_fann"

# Training parameters
learning_rate = 0.001
epochs = 5000
batch_size = 32

# Feature extraction
feature_extraction = "enhanced"
include_context = true
context_window = 5

# Performance tuning
cache_enabled = true
max_cache_size_mb = 256
parallel_processing = true
```

## Model Training

### Initial Training

```bash
# Train ML model with existing data
codeguardian train --model-path enhanced-model.fann --epochs 2000

# Use bootstrap training for new projects
codeguardian train --bootstrap --model-path bootstrap-model.fann
```

### Online Learning

The ML system automatically improves through user feedback:

```rust
// Programmatic feedback
let ml_classifier = MLClassifier::new(Some("model.fann"));
ml_classifier.record_feedback(&finding, true).await?; // True positive
ml_classifier.record_feedback(&finding, false).await?; // False positive
```

### Training Data Collection

```bash
# Collect training data from analysis runs
codeguardian check . --collect-training-data --output training.json

# Generate synthetic training data
codeguardian train --generate-synthetic --output synthetic-training.json
```

## Feature Extraction

### Basic Features (8 dimensions)

- File type reliability score
- Analyzer confidence score
- Severity level encoding
- Code pattern complexity
- Context similarity score
- Historical accuracy
- False positive indicators
- Domain-specific patterns

### Enhanced Features (24 dimensions)

When AST analysis is enabled:

- Abstract Syntax Tree patterns
- Control flow complexity
- Data flow analysis
- Semantic similarity metrics
- Language-specific constructs
- Code structure analysis

## Model Management

### Model Versioning

```bash
# List available models
codeguardian ml models list

# Deploy new model version
codeguardian ml models deploy --model new-model.fann --environment production

# Rollback to previous version
codeguardian ml models rollback --model codeguardian --version v1.2.3
```

### A/B Testing

```bash
# Start A/B test between models
codeguardian ml ab-test start \
  --model-a current-model.fann \
  --model-b new-model.fann \
  --traffic-split 50-50 \
  --duration-days 7
```

### Model Comparison

```bash
# Compare model performance
codeguardian ml models compare \
  --models model1.fann model2.fann \
  --metrics accuracy precision recall f1-score
```

## Performance Optimization

### Caching

```toml
[ml]
cache_enabled = true
max_cache_size_mb = 512
cache_ttl_hours = 24
```

### Parallel Processing

```toml
[ml]
parallel_processing = true
max_parallel_tasks = 8
batch_size = 64
```

### Memory Management

```toml
[ml]
memory_optimization = true
streaming_batch_size = 1000
garbage_collection_interval = 1000
```

## Monitoring and Metrics

### Model Performance

```bash
# Get ML performance metrics
codeguardian ml metrics --model current-model.fann

# Monitor false positive rate
codeguardian ml metrics --false-positive-rate
```

### Training Progress

```bash
# Monitor training progress
codeguardian train --progress --log-interval 100
```

## Integration Examples

### CI/CD Pipeline

```yaml
- name: ML-Enhanced Analysis
  run: |
    codeguardian check . \
      --ml-model enhanced-model.fann \
      --confidence-threshold 0.8 \
      --format json \
      --out results.json

- name: Update ML Model
  run: |
    codeguardian train \
      --incremental \
      --feedback-file feedback.json \
      --model-path enhanced-model.fann
```

### Pre-commit Hooks

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Run ML-enhanced analysis
codeguardian check --staged-only --ml-enabled

# Update model with any feedback
codeguardian ml update --feedback-file .codeguardian/feedback.json
```

## Troubleshooting

### Common Issues

**Model not loading**
- Verify model file exists and is readable
- Check FANN library installation
- Ensure model was trained with compatible version

**Poor accuracy**
- Increase training epochs
- Add more diverse training data
- Adjust confidence threshold
- Enable AST analysis for better features

**Performance issues**
- Enable caching
- Reduce feature extraction complexity
- Use parallel processing
- Monitor memory usage

**Training failures**
- Check training data quality
- Verify feature extraction is working
- Reduce learning rate if training diverges
- Use bootstrap training for initial models

## Best Practices

### Model Training

1. **Start with bootstrap training** for new projects
2. **Use diverse training data** covering different code patterns
3. **Enable online learning** for continuous improvement
4. **Regularly retrain** models with new data

### Performance Tuning

1. **Enable caching** for repeated analyses
2. **Use appropriate batch sizes** for your hardware
3. **Monitor memory usage** and adjust limits
4. **Parallelize** when possible

### Maintenance

1. **Version control models** like code
2. **Monitor performance metrics** regularly
3. **A/B test** before deploying new models
4. **Backup models** before major updates

## Advanced Usage

### Custom Feature Extractors

```rust
use codeguardian::ml::FeatureExtractor;

struct CustomExtractor;

impl FeatureExtractor for CustomExtractor {
    fn extract_features(&self, finding: &Finding) -> Result<Vec<f32>> {
        // Custom feature extraction logic
        Ok(vec![/* features */])
    }
}
```

### Model Customization

```rust
use codeguardian::ml::fann_classifier::NetworkConfig;

let config = NetworkConfig {
    input_size: 24,
    hidden_layers: vec![64, 32],
    output_size: 1,
    activation_function: "sigmoid".to_string(),
};

let classifier = FannClassifier::new(config)?;
```

## Next Steps

- [Configuration Guide](../configuration.md) - ML configuration options
- [Training Guide](training.md) - Advanced training techniques
- [API Reference](../api.md) - ML API documentation
