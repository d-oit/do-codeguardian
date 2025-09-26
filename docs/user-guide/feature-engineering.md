# 🔬 Feature Engineering Guide

## Synopsis

The `feature-engineering` command provides advanced feature engineering capabilities for machine learning models, including automated feature generation, selection, and optimization.

## Description

CodeGuardian's feature engineering system enhances ML model performance through intelligent feature creation, selection, and optimization. It supports multiple generation strategies and selection methods to extract the most informative features from code analysis data.

## Syntax

```bash
codeguardian feature-engineering [OPTIONS] --input <PATH>
```

## Options

| Option | Description | Default | Type |
|--------|-------------|---------|------|
| `-i, --input` | Input file or directory to analyze | Required | Path |
| `--auto-generation` | Enable automated feature generation | `true` | Boolean |
| `--feature-selection` | Enable feature selection | `true` | Boolean |
| `--max-features` | Maximum number of generated features | `1000` | Integer |
| `--selection-threshold` | Feature selection threshold (0.0-1.0) | `0.01` | Float |
| `--strategies` | Generation strategies (comma-separated) | Auto-detect | List |
| `--selection-methods` | Selection methods (comma-separated) | Auto-detect | List |
| `--detailed` | Output detailed analysis | - | Flag |
| `--benchmark` | Benchmark feature engineering performance | - | Flag |

## Feature Generation Strategies

### Polynomial Features
Generate polynomial combinations of existing features to capture non-linear relationships.

```bash
codeguardian feature-engineering --input src/ --strategies polynomial
```

### Interaction Features
Create interaction terms between features to model feature dependencies.

```bash
codeguardian feature-engineering --input src/ --strategies interaction
```

### Statistical Features
Generate statistical aggregations and transformations.

```bash
codeguardian feature-engineering --input src/ --strategies statistical
```

### Domain-Specific Features
Create features specific to code analysis domain (complexity metrics, pattern counts).

```bash
codeguardian feature-engineering --input src/ --strategies domain_specific
```

### Text Features
Generate text-based features from code comments, variable names, and strings.

```bash
codeguardian feature-engineering --input src/ --strategies text
```

## Feature Selection Methods

### Univariate Selection
Select features based on univariate statistical tests.

```bash
codeguardian feature-engineering --input src/ --selection-methods univariate
```

### Recursive Feature Elimination (RFE)
Iteratively remove features based on model performance.

```bash
codeguardian feature-engineering --input src/ --selection-methods rfe
```

### LASSO Regularization
Use L1 regularization to automatically select relevant features.

```bash
codeguardian feature-engineering --input src/ --selection-methods lasso
```

### Mutual Information
Select features based on mutual information with target variable.

```bash
codeguardian feature-engineering --input src/ --selection-methods mutual_info
```

### Variance Threshold
Remove features with low variance (constant or near-constant values).

```bash
codeguardian feature-engineering --input src/ --selection-methods variance
```

## Examples

### Basic Feature Engineering

Generate and select features from a source directory:

```bash
codeguardian feature-engineering --input src/ --detailed
```

### Custom Strategy Combination

Use specific generation strategies and selection methods:

```bash
codeguardian feature-engineering \
  --input src/ \
  --strategies polynomial,interaction,domain_specific \
  --selection-methods rfe,mutual_info \
  --max-features 500
```

### Performance Benchmarking

Benchmark feature engineering performance:

```bash
codeguardian feature-engineering --input src/ --benchmark
```

### High-Precision Selection

Use strict selection criteria for high-quality features:

```bash
codeguardian feature-engineering \
  --input src/ \
  --selection-threshold 0.05 \
  --max-features 200 \
  --detailed
```

### Large-Scale Processing

Process large codebases with optimized settings:

```bash
codeguardian feature-engineering \
  --input large-project/ \
  --auto-generation true \
  --max-features 2000 \
  --strategies polynomial,statistical
```

## Output

The command outputs detailed information about the feature engineering process:

### Feature Generation Summary
```
=== Feature Engineering Metrics ===
Total features generated: 1,247
Features selected: 156
Generation time: 234ms
Selection time: 89ms
Cache hits: 892
Cache misses: 45
```

### Top Features (Detailed Mode)
```
=== Finding 1 ===
Rule: SQL_INJECTION_RISK
Message: Potential SQL injection vulnerability
Severity: High
Generated 156 features

Top 5 features:
  Feature 23: 0.8234
  Feature 67: 0.7891
  Feature 12: 0.7456
  Feature 89: 0.6789
  Feature 34: 0.6234
```

### Benchmark Results
```
=== Benchmark Results ===
Processed 45 findings
Total features generated: 8,234
Average features per finding: 183.1
Total processing time: 2.3s
Average time per finding: 51ms
Cache hit ratio: 95.2%
```

## Configuration

Feature engineering can be configured through the main configuration file:

```toml
[ml.feature_engineering]
auto_generation = true
feature_selection = true
max_features = 1000
selection_threshold = 0.01
cv_folds = 5

[ml.feature_engineering.strategies]
polynomial = { degree = 2, include_bias = false }
interaction = { max_combinations = 3 }
statistical = { include_moments = true }
domain_specific = { complexity_metrics = true }
text = { ngram_range = [1, 2] }

[ml.feature_engineering.selection]
univariate = { score_func = "f_classif", k = 100 }
rfe = { n_features = 50, step = 1 }
lasso = { alpha = 0.01, max_iter = 1000 }
mutual_info = { n_neighbors = 3 }
variance = { threshold = 0.0 }
```

## Performance Considerations

### Memory Usage
- Large feature sets can consume significant memory
- Use `--max-features` to limit memory usage
- Enable caching for repeated analyses

### Processing Time
- Feature generation scales with input size
- Selection methods have different time complexities
- Use `--benchmark` to profile performance

### Quality vs. Quantity
- More features don't always improve model performance
- Use appropriate selection thresholds
- Consider domain expertise in strategy selection

## See Also

- [ML Guide](ml.md) - General ML functionality
- [Train Command](train.md) - Model training with engineered features
- [Metrics Command](metrics.md) - Feature performance evaluation
- [Configuration Guide](configuration.md) - ML configuration options