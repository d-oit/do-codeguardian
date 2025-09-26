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

### Model Validation

CodeGuardian provides comprehensive model validation capabilities to ensure models meet quality and performance standards before deployment.

#### Basic Model Validation

```bash
# Validate a trained model against test suites
codeguardian model-validation --model-path trained-model.fann --test-suites-dir test-suites/

# Validate with baseline comparison
codeguardian model-validation \
  --model-path new-model.fann \
  --baseline-model baseline-model.fann \
  --output-dir validation-reports/
```

#### Comprehensive Validation

```bash
# Full validation with all features enabled
codeguardian model-validation \
  --model-path production-model.fann \
  --test-suites-dir comprehensive-tests/ \
  --config-file validation-config.toml \
  --export-metrics \
  --enable-bias-detection \
  --enable-robustness-testing \
  --verbose
```

#### Validation Configuration

```toml
# validation-config.toml
[validation]
min_accuracy = 0.90
max_false_positive_rate = 0.10
max_inference_time_ms = 50.0

[validation.thresholds]
critical_severity = 0.95
high_severity = 0.85
medium_severity = 0.75
low_severity = 0.60

[validation.bias_detection]
enabled = true
protected_attributes = ["file_type", "project_size"]
fairness_metrics = ["demographic_parity", "equalized_odds"]

[validation.robustness]
enabled = true
noise_levels = [0.01, 0.05, 0.10]
adversarial_examples = true
```

#### Test Suite Generation

Generate test suites from existing findings:

```bash
# Generate comprehensive test suites
codeguardian model-validation \
  --findings-file analysis-results.json \
  --generate-test-suites \
  --output-dir generated-test-suites/
```

#### Detailed Process Explanation

The model validation process follows a comprehensive workflow to ensure model quality and reliability:

1. **Model Loading**: Load the specified FANN model and validate its structure
2. **Test Suite Preparation**: Load and preprocess test cases from the provided test suites directory
3. **Feature Extraction**: Apply the same feature extraction pipeline used in training
4. **Inference Execution**: Run model predictions on test data with performance monitoring
5. **Metrics Calculation**: Compute accuracy, precision, recall, F1-score, and false positive rates
6. **Bias Detection** (optional): Analyze predictions for bias across protected attributes
7. **Robustness Testing** (optional): Test model stability under various noise conditions
8. **Report Generation**: Produce detailed validation reports in specified formats
9. **Threshold Validation**: Compare results against configured quality thresholds

#### Output Formats

Model validation generates multiple output formats for different use cases:

- **JSON Report**: Comprehensive metrics and detailed results (`validation-report.json`)
- **Markdown Summary**: Human-readable summary with key findings (`validation-summary.md`)
- **CSV Metrics**: Tabular data for analysis and visualization (`validation-metrics.csv`)
- **HTML Dashboard**: Interactive web report with charts (`validation-dashboard.html`)
- **Log Files**: Detailed execution logs (`validation.log`)

#### Results Interpretation

Understanding validation results is crucial for model deployment decisions:

- **Accuracy**: Overall correctness rate (target: >90%)
- **Precision**: True positive rate among predicted positives (minimizes false alarms)
- **Recall**: True positive rate among actual positives (minimizes missed issues)
- **F1-Score**: Harmonic mean of precision and recall (balanced metric)
- **False Positive Rate**: Rate of incorrect positive predictions (target: <10%)
- **Bias Metrics**: Fairness scores across protected attributes (demographic parity, equalized odds)
- **Robustness Scores**: Performance stability under noise/adversarial conditions

#### Advanced Features

##### Bias Detection

Bias detection analyzes model fairness across protected attributes:

- **Protected Attributes**: File type, project size, language, severity levels
- **Fairness Metrics**:
  - Demographic Parity: Equal positive prediction rates across groups
  - Equalized Odds: Equal true positive and false positive rates
  - Predictive Parity: Equal precision across groups
- **Reporting**: Identifies biased predictions with confidence intervals

##### Robustness Testing

Robustness testing evaluates model stability:

- **Noise Injection**: Add random noise to input features at specified levels
- **Adversarial Examples**: Generate inputs designed to fool the model
- **Perturbation Analysis**: Test sensitivity to small input changes
- **Stress Testing**: Evaluate performance under high load conditions

#### CLI Options Coverage

Complete list of options for the `model-validation` command:

```bash
codeguardian model-validation [OPTIONS]

Options:
  -m, --model-path <PATH>          Path to the FANN model file
  -t, --test-suites-dir <DIR>      Directory containing test suites
  -b, --baseline-model <PATH>      Baseline model for comparison
  -o, --output-dir <DIR>           Output directory for reports [default: validation-reports/]
  -c, --config-file <FILE>         Validation configuration file
  -f, --findings-file <FILE>       Findings file for test suite generation
      --generate-test-suites       Generate test suites from findings
      --export-metrics             Export detailed metrics
      --enable-bias-detection      Enable bias detection analysis
      --enable-robustness-testing  Enable robustness testing
      --auto-deploy                Automatically deploy model if validation passes
      --fail-on-issues             Fail command if validation issues are found
      --min-accuracy <FLOAT>       Minimum required accuracy threshold
      --max-false-positive-rate <FLOAT> Maximum allowed false positive rate
      --max-inference-time-ms <FLOAT> Maximum allowed inference time in milliseconds
      --verbose                    Enable verbose output
  -h, --help                       Print help information
```

#### Test Suite Details

Test suites are structured collections of labeled examples for validation:

- **Directory Structure**:
  ```
  test-suites/
  ├── positive/           # True positive examples
  │   ├── finding1.json
  │   └── finding2.json
  ├── negative/           # True negative examples
  │   ├── clean1.json
  │   └── clean2.json
  └── metadata.json       # Suite metadata and labels
  ```

- **Finding Format**: JSON representation of analysis findings with ground truth labels
- **Coverage Requirements**: Test suites should cover various code patterns, languages, and severity levels
- **Size Recommendations**: Minimum 1000 examples per suite for reliable validation

#### Programmatic Usage

Use model validation programmatically in Rust applications:

```rust
use codeguardian::ml::validation::{ModelValidator, ValidationConfig, ValidationReport};

let config = ValidationConfig {
    model_path: "production-model.fann".into(),
    test_suites_dir: "test-suites/".into(),
    enable_bias_detection: true,
    enable_robustness_testing: true,
    output_formats: vec!["json".into(), "markdown".into()],
    ..Default::default()
};

let validator = ModelValidator::new(config)?;
let report: ValidationReport = validator.validate().await?;

println!("Accuracy: {:.2}%", report.metrics.accuracy * 100.0);
println!("False Positive Rate: {:.2}%", report.metrics.false_positive_rate * 100.0);

// Access detailed results
for result in &report.detailed_results {
    println!("Test case {}: predicted={}, actual={}",
             result.test_case_id, result.prediction, result.ground_truth);
}
```

#### Best Practices

- **Regular Validation**: Validate models before each deployment
- **Comprehensive Test Suites**: Use diverse, representative test data
- **Threshold Setting**: Set appropriate quality thresholds for your use case
- **Bias Monitoring**: Regularly check for bias in model predictions
- **Version Control**: Keep validation reports with model versions
- **Automated Validation**: Integrate validation into CI/CD pipelines
- **Performance Benchmarking**: Compare against baseline models
- **Incremental Testing**: Validate changes incrementally during development

#### Troubleshooting

**Validation fails to load model**
- Ensure model file exists and is readable
- Check FANN library compatibility
- Verify model was trained with current CodeGuardian version

**Poor validation metrics**
- Review test suite quality and diversity
- Check for overfitting during training
- Adjust confidence thresholds
- Enable enhanced feature extraction

**Bias detection shows high bias scores**
- Analyze protected attributes in training data
- Balance training data across groups
- Consider debiasing techniques
- Review feature extraction for bias introduction

**Robustness testing failures**
- Increase training data diversity
- Use data augmentation techniques
- Implement regularization during training
- Consider ensemble methods for stability

**Performance issues during validation**
- Reduce test suite size for quick checks
- Enable parallel processing
- Increase timeout values for large models
- Use streaming validation for memory efficiency

#### Examples

##### Basic Validation with Custom Thresholds

```bash
codeguardian model-validation \
  --model-path my-model.fann \
  --test-suites-dir validation-tests/ \
  --config-file strict-validation.toml \
  --output-dir reports/ \
  --min-accuracy 0.95 \
  --max-false-positive-rate 0.05 \
  --max-inference-time-ms 50.0
```

##### Bias Detection Analysis

```bash
codeguardian model-validation \
  --model-path production-model.fann \
  --enable-bias-detection \
  --protected-attributes file_type project_size language \
  --fairness-metrics demographic_parity equalized_odds \
  --output-dir bias-reports/
```

##### Robustness Testing Suite

```bash
codeguardian model-validation \
  --model-path robust-model.fann \
  --enable-robustness-testing \
  --noise-levels 0.01 0.05 0.10 \
  --adversarial-examples \
  --stress-test \
  --max-inference-time-ms 100.0 \
  --fail-on-issues
```

##### CI/CD Integration

```yaml
- name: Model Validation
  run: |
    codeguardian model-validation \
      --model-path models/current.fann \
      --test-suites-dir tests/validation/ \
      --baseline-model models/baseline.fann \
      --enable-bias-detection \
      --enable-robustness-testing \
      --export-metrics \
      --output-dir validation-results/ \
      --auto-deploy \
      --fail-on-issues \
      --min-accuracy 0.90 \
      --max-false-positive-rate 0.10

    # Fail pipeline if validation fails
    if [ $? -ne 0 ]; then
      echo "Model validation failed"
      exit 1
    fi
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
