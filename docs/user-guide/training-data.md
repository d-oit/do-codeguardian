# training-data

## Synopsis
Collect and manage training data for machine learning models used in CodeGuardian's false positive reduction and analysis accuracy improvements.

## Description
The training-data command provides comprehensive training data collection and management capabilities for CodeGuardian's ML-powered analysis features. It supports multiple data sources, labeling strategies, and export formats to build high-quality training datasets for anomaly detection and false positive reduction models.

Key capabilities include:
- **Multi-source Data Collection**: Gather training data from analysis results or direct codebase scanning
- **Flexible Labeling Strategies**: Heuristic, severity-based, and file-type based labeling approaches
- **Balance Control**: Maintain target ratios between true positives and false positives
- **Multiple Export Formats**: JSON, CSV, and TFRecord support for different ML frameworks
- **Interactive Mode**: Manual review and labeling capabilities
- **Validation**: Built-in data quality checks and validation

## Syntax
```bash
codeguardian training-data [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--input-file <FILE>, -i <FILE>` | Input file containing findings (JSON format) | `PATH` | - | No |
| `--source-path <PATH>, -s <PATH>` | Source path to analyze (alternative to input file) | `PATH` | - | No |
| `--output-dir <DIR>, -o <DIR>` | Output directory for training data | `PATH` | `data/training` | No |
| `--config-file <FILE>, -c <FILE>` | Configuration file for collection pipeline | `PATH` | - | No |
| `--min-examples <NUM>` | Minimum number of examples to collect | `usize` | `100` | No |
| `--target-balance <RATIO>` | Target balance ratio (true_positives / false_positives) | `f64` | `1.0` | No |
| `--include-low-confidence` | Include low-confidence labels | `FLAG` | `false` | No |
| `--skip-manual-review` | Skip manual review requirement | `FLAG` | `false` | No |
| `--export-formats <FORMATS>` | Export formats (comma-separated: json,csv,tfrecord) | `STRING` | `json,csv` | No |
| `--labeling-strategies <STRATEGIES>` | Labeling strategies to use (comma-separated) | `STRING` | `heuristic,severity_based,file_type_based` | No |
| `--interactive` | Enable interactive labeling mode | `FLAG` | `false` | No |
| `--validate-only` | Validate existing training data | `FLAG` | `false` | No |

### Labeling Strategies Values
- `heuristic`: Rule-based labeling using predefined patterns
- `severity_based`: Labeling based on finding severity levels
- `file_type_based`: Labeling based on file type and extension
- `manual`: Manual review and labeling by users

### Export Formats Values
- `json`: JSON format for general ML frameworks
- `csv`: CSV format for spreadsheet analysis and some ML tools
- `tfrecord`: TensorFlow Record format for TensorFlow models

## Examples

### Basic Usage
```bash
# Collect training data from analysis results
codeguardian training-data --input-file analysis-results.json

# Analyze source code directly for training data
codeguardian training-data --source-path src/

# Collect minimum examples with default settings
codeguardian training-data --source-path . --min-examples 200
```

### Advanced Usage
```bash
# Comprehensive training data collection with custom configuration
codeguardian training-data \
  --source-path src/ \
  --output-dir custom-training-data \
  --config-file training-config.toml \
  --min-examples 500 \
  --target-balance 2.0 \
  --export-formats json,csv,tfrecord \
  --labeling-strategies heuristic,severity_based,manual

# Interactive labeling session
codeguardian training-data \
  --input-file findings.json \
  --interactive \
  --include-low-confidence

# Validate existing training data
codeguardian training-data \
  --input-file existing-training.json \
  --validate-only
```

### Integration with ML Training
```bash
# Collect training data for model training pipeline
codeguardian training-data \
  --source-path . \
  --output-dir data/training \
  --export-formats json

# Then train the model
codeguardian train \
  --training-data data/training/dataset.json \
  --epochs 1000
```

## Training Data Collection Process

1. **Data Source Selection**: Choose between analysis results file or direct codebase scanning
2. **Feature Extraction**: Extract relevant code patterns, metrics, and contextual information
3. **Labeling**: Apply selected labeling strategies to classify findings
4. **Balance Adjustment**: Ensure target ratio between positive and negative examples
5. **Quality Validation**: Check data quality and remove invalid or corrupted examples
6. **Export**: Save training data in specified formats for ML model training

## Output Structure

The command creates the following directory structure in the output directory:

```
output-dir/
├── dataset.json          # Main training dataset
├── dataset.csv           # CSV format (if requested)
├── tfrecords/            # TFRecord files (if requested)
├── metadata.json         # Collection metadata and statistics
├── validation_report.json # Data quality validation results
└── config.toml          # Configuration used for collection
```

## Configuration File

You can provide a custom configuration file to control the collection process:

```toml
[collection]
min_examples = 500
target_balance = 1.5
include_low_confidence = false
skip_manual_review = true

[labeling]
strategies = ["heuristic", "severity_based"]
export_formats = ["json", "csv"]

[validation]
enable_quality_checks = true
min_confidence_threshold = 0.7
```

## Error Handling

### Common Errors
- **Input File Not Found**: Specified input file does not exist or is not readable
- **Invalid JSON Format**: Input findings file contains invalid JSON
- **Insufficient Examples**: Not enough examples collected to meet minimum requirements
- **Permission Denied**: Cannot write to specified output directory
- **Configuration Error**: Invalid configuration file format or values

## Security Considerations
- **Data Sanitization**: All collected training data is sanitized to remove sensitive information
- **Access Control**: Training data collection respects file permissions and access controls
- **Data Privacy**: No personal or sensitive data is included in training datasets
- **Audit Trail**: All collection operations are logged for audit purposes

## See Also
- [`codeguardian train`](train.md) - Train ML model using collected training data
- [`codeguardian check`](check.md) - Generate analysis results for training data collection
- [`codeguardian metrics`](metrics.md) - Analyze ML model performance metrics