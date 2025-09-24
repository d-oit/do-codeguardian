# Training Data Collection Pipeline

## Overview

The CodeGuardian Training Data Collection Pipeline provides a comprehensive system for collecting, labeling, and managing high-quality training datasets for machine learning models. This pipeline is essential for improving the accuracy and reducing false positives in security analysis.

## Features

✅ **Automated Labeling Strategies**
- Heuristic-based classification using domain knowledge
- Severity-based labeling for quick categorization
- File type analysis for context-aware decisions
- Analyzer-based confidence scoring

✅ **Interactive Labeling Interface**
- Terminal-based UI for manual review
- Real-time progress tracking
- Quality metrics and session statistics
- Undo/redo functionality

✅ **Multiple Export Formats**
- JSON for CodeGuardian native format
- CSV for spreadsheet analysis
- TensorFlow-compatible format for ML frameworks

✅ **Quality Validation**
- Dataset balance ratio monitoring
- Minimum example thresholds
- Confidence distribution analysis
- Automated quality scoring

## Quick Start

### 1. Collect Findings

First, run analysis to collect findings:

```bash
# Analyze your codebase
codeguardian check . --format json --out findings.json

# Or use an existing findings file
```

### 2. Automated Collection

Run automated training data collection:

```bash
codeguardian training-data \
  --input-file findings.json \
  --output-dir data/training \
  --min-examples 100 \
  --export-formats "json,csv"
```

### 3. Interactive Labeling

For higher quality data, use interactive labeling:

```bash
codeguardian training-data \
  --input-file findings.json \
  --output-dir data/training \
  --interactive \
  --min-examples 500
```

### 4. Train Model

Use collected data to train a model:

```bash
codeguardian train \
  --training-data data/training/training_data.json \
  --model-path models/custom-model.fann \
  --epochs 1000 \
  --validate \
  --cross-validate
```

## Detailed Usage

### Command Line Options

```
codeguardian training-data [OPTIONS]

OPTIONS:
    -i, --input-file <FILE>              Input findings file (JSON format)
    -s, --source-path <PATH>             Source path to analyze directly
    -o, --output-dir <DIR>               Output directory [default: data/training]
    -c, --config-file <FILE>             Configuration file for pipeline
    
    --min-examples <N>                   Minimum examples to collect [default: 100]
    --target-balance <RATIO>             Target balance ratio [default: 1.0]
    --include-low-confidence             Include low-confidence labels
    --skip-manual-review                 Skip manual review requirement
    
    --export-formats <FORMATS>           Export formats (comma-separated)
                                        [default: json,csv]
    --labeling-strategies <STRATEGIES>   Labeling strategies to use
                                        [default: heuristic,severity_based]
    
    --interactive                        Enable interactive labeling mode
    --validate-only                      Validate existing training data
```

### Configuration File

Create a configuration file to customize the collection process:

```json
{
  "min_examples": 500,
  "target_balance_ratio": 1.0,
  "include_low_confidence": false,
  "require_manual_review": true,
  "export_formats": ["json", "csv", "tfrecord"],
  "labeling_strategies": [
    "heuristic",
    "severity_based",
    "file_type_based",
    "analyzer_based"
  ]
}
```

## Labeling Strategies

### Heuristic Strategy
Uses domain knowledge and patterns to classify findings:

- **True Positives**: Critical/High severity security issues, integrity violations
- **False Positives**: TODOs in test files, debug statements in examples
- **Confidence**: 0.8

### Severity-Based Strategy
Simple classification based on finding severity:

- **Critical/High**: True Positive
- **Medium**: Uncertain (requires review)
- **Low/Info**: False Positive
- **Confidence**: 0.7

### File Type Strategy
Context-aware labeling based on file location:

- **Test Files**: Likely false positives
- **Documentation**: Usually false positives
- **Source Code**: More likely true positives
- **Confidence**: 0.6

### Analyzer-Based Strategy
Uses analyzer reliability for classification:

- **Integrity Analyzer**: High confidence true positives
- **Security Analyzer**: High confidence true positives
- **Style Analyzer**: Often false positives
- **Confidence**: 0.9

## Interactive Labeling

The interactive labeling interface provides a terminal-based UI for manual review:

### Controls
- `y` - Mark as True Positive (real issue)
- `n` - Mark as False Positive (false alarm)
- `s` - Skip this finding
- `p` - Go to previous finding
- `?` - Show help
- `q` - Quit and save progress

### Guidelines

**True Positive (y)** - Mark if:
- Finding represents a real security issue
- Code quality problem that should be fixed
- Performance issue with impact
- Compliance violation

**False Positive (n)** - Mark if:
- Finding is incorrect or misleading
- Code is acceptable in this context
- Test code with intentional patterns
- Documentation or comments

**Skip (s)** - Use when:
- Uncertain about the classification
- Need more context to decide
- Complex edge case

## Export Formats

### JSON Format
Native CodeGuardian format with complete metadata:

```json
{
  "examples": [
    {
      "finding_id": "security_sql_injection_1",
      "features": [1.0, 0.9, 0.95, 0.8, 0.7, 1.0, 1.0, 0.85],
      "is_true_positive": true,
      "feedback_source": "UserFeedback",
      "timestamp": "2024-01-15T10:30:00Z"
    }
  ],
  "version": "1.0.0",
  "created_at": "2024-01-15T10:00:00Z"
}
```

### CSV Format
Spreadsheet-compatible format for analysis:

```csv
finding_id,feature_0,feature_1,...,is_true_positive,feedback_source,timestamp
security_sql_injection_1,1.0,0.9,...,true,UserFeedback,2024-01-15T10:30:00Z
```

### TensorFlow Format
ML framework-compatible JSON structure:

```json
[
  {
    "features": [1.0, 0.9, 0.95, 0.8, 0.7, 1.0, 1.0, 0.85],
    "label": 1,
    "metadata": {
      "finding_id": "security_sql_injection_1",
      "timestamp": "2024-01-15T10:30:00Z"
    }
  }
]
```

## Quality Validation

The pipeline automatically validates training data quality:

### Balance Ratio
- **Target**: 0.5 - 2.0 (balanced dataset)
- **Warning**: 0.1 - 10.0 (moderate imbalance)
- **Error**: < 0.1 or > 10.0 (severe imbalance)

### Minimum Examples
- **Per Class**: 10 minimum
- **Recommended**: 100+ per class
- **Production**: 1000+ per class

### Quality Score
Calculated from balance and quantity metrics:
- **1.0**: Perfect balance and sufficient data
- **0.8+**: Good quality
- **0.6+**: Acceptable quality
- **< 0.6**: Needs improvement

## Best Practices

### 1. Start with Automated Labeling
```bash
# Get initial dataset quickly
codeguardian training-data \
  --input-file findings.json \
  --output-dir data/training \
  --labeling-strategies "heuristic,severity_based,analyzer_based"
```

### 2. Review and Refine Interactively
```bash
# Manually review uncertain cases
codeguardian training-data \
  --input-file findings.json \
  --output-dir data/training \
  --interactive \
  --include-low-confidence
```

### 3. Collect from Multiple Projects
```bash
# Analyze different codebases
for repo in repo1 repo2 repo3; do
  codeguardian check "$repo" --out "${repo}_findings.json"
  codeguardian training-data \
    --input-file "${repo}_findings.json" \
    --output-dir "data/training/${repo}"
done
```

### 4. Regular Quality Validation
```bash
# Validate existing datasets
codeguardian training-data \
  --output-dir data/training \
  --validate-only
```

### 5. Incremental Training
```bash
# Combine multiple datasets
cat data/training/*/training_data.json > combined_training.json

# Train with combined data
codeguardian train \
  --training-data combined_training.json \
  --continue-training \
  --validate
```

## Automation Script

Use the provided automation script for complete pipeline:

```bash
# Automated collection
./scripts/collect_training_data.sh automated

# Interactive collection
./scripts/collect_training_data.sh interactive

# Validate existing data
./scripts/collect_training_data.sh validate
```

## Troubleshooting

### Common Issues

**"No findings to label"**
- Ensure input file contains findings array
- Check file path and permissions
- Verify findings are in correct JSON format

**"Low quality score"**
- Collect more diverse examples
- Balance true/false positive ratios
- Use interactive labeling for uncertain cases

**"Interactive mode not working"**
- Ensure terminal supports ANSI colors
- Check crossterm dependency is available
- Try running in different terminal emulator

**"Export failed"**
- Check output directory permissions
- Ensure sufficient disk space
- Verify export format is supported

### Debug Mode

Enable debug logging for troubleshooting:

```bash
RUST_LOG=debug codeguardian training-data \
  --input-file findings.json \
  --output-dir data/training
```

## Integration Examples

### CI/CD Pipeline
```yaml
- name: Collect Training Data
  run: |
    codeguardian check . --out findings.json
    codeguardian training-data \
      --input-file findings.json \
      --output-dir artifacts/training \
      --skip-manual-review
```

### Programmatic Usage
```rust
use do_codeguardian::cli::training_data::TrainingDataCollectionPipeline;

let mut pipeline = TrainingDataCollectionPipeline::new();
let stats = pipeline.collect_training_data(
    config,
    findings,
    &output_dir,
).await?;
```

## Related Commands

- `codeguardian check` - Analyze code and generate findings
- `codeguardian train` - Train ML models with collected data
- `codeguardian metrics` - Evaluate model performance
- `codeguardian bulk` - Process multiple repositories

## Contributing

To improve the training data collection pipeline:

1. Add new labeling strategies in `src/cli/training_data.rs`
2. Implement additional export formats
3. Enhance quality validation metrics
4. Improve interactive UI features

See [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines.