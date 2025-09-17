# train

## Synopsis
Train machine learning model for enhanced false positive reduction and improved analysis accuracy.

## Description
The train command trains CodeGuardian's ML model using historical analysis data to improve false positive detection. It supports various training configurations and can bootstrap training data for new installations.

## Syntax
```bash
codeguardian train [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--model-path <PATH>` | Path to save the trained model | `PATH` | `codeguardian-model.fann` | No |
| `--epochs <NUM>` | Number of training epochs | `u32` | `1000` | No |
| `--bootstrap` | Generate synthetic training data for cold start | `FLAG` | `false` | No |
| `--training-data <FILE>` | Path to existing training data (JSON format) | `PATH` | - | No |
| `--continue-training` | Continue training from existing model | `FLAG` | `false` | No |
| `--validate` | Validate model performance after training | `FLAG` | `false` | No |
| `--enhanced` | Use AST-enhanced features | `FLAG` | `false` | No |

## Examples
```bash
# Train new model with default settings
codeguardian train

# Bootstrap training for new installation
codeguardian train --bootstrap --epochs 2000

# Continue training existing model
codeguardian train --continue-training --model-path existing-model.fann

# Train with validation
codeguardian train --validate

# Use enhanced AST features
codeguardian train --enhanced --training-data custom-data.json
```

## Training Process
1. **Data Collection**: Gather analysis results and user feedback
2. **Feature Extraction**: Extract relevant features from code patterns
3. **Model Training**: Train neural network on labeled data
4. **Validation**: Test model performance on held-out data
5. **Deployment**: Save trained model for use in analysis

## See Also
- [`codeguardian check`](check.md) - Use trained model in analysis
