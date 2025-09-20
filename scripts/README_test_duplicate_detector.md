# Test Duplicate Detector

A bash script for automated test duplication detection in Rust projects. This script analyzes test files to identify duplicate code patterns, similar test setups, and redundant assertions.

## Features

- **Automatic Test File Discovery**: Finds all test files in the `tests/` directory and source files containing `#[test]` attributes
- **Test Function Extraction**: Parses Rust test functions and extracts their components
- **Duplicate Detection**: Identifies identical test functions based on code similarity
- **Setup and Assertion Analysis**: Extracts and compares test setup code and assertions
- **Multiple Output Formats**: Supports both text and JSON output formats
- **Configurable Similarity Threshold**: Adjustable threshold for duplicate detection
- **Comprehensive Reporting**: Generates detailed reports with refactoring recommendations

## Usage

### Basic Usage

```bash
# Run with default settings (text output, threshold 0.7)
./scripts/test_duplicate_detector.sh

# Generate JSON output
FORMAT=json ./scripts/test_duplicate_detector.sh

# Use custom similarity threshold
THRESHOLD=0.8 ./scripts/test_duplicate_detector.sh

# Specify custom output directory
OUTPUT_DIR=/path/to/output ./scripts/test_duplicate_detector.sh
```

### Environment Variables

- `THRESHOLD`: Similarity threshold (0.0-1.0, default: 0.7)
- `FORMAT`: Output format (`text` or `json`, default: `text`)
- `OUTPUT_DIR`: Output directory (default: `target/test-analysis`)

### Examples

```bash
# Generate JSON report with high similarity threshold
THRESHOLD=0.9 FORMAT=json ./scripts/test_duplicate_detector.sh

# Analyze with custom output location
OUTPUT_DIR=./reports ./scripts/test_duplicate_detector.sh
```

## Output

### Text Format

The script generates a human-readable text report with:
- Summary statistics (total tests, duplicates found)
- Detailed duplicate information with similarity scores
- Shared patterns between duplicate tests
- Refactoring recommendations

### JSON Format

The script generates a machine-readable JSON report with:
- Test statistics
- Array of duplicate pairs with metadata
- Recommendations array

## Sample Output

### Text Report
```
Test Duplication Analysis Report
================================
Generated: Sat Sep 20 09:49:46 UTC 2025
Project: /workspaces/do-codeguardian

Summary:
--------
Total test functions: 16
Duplicates found: 0
Identical functions: 0
Similar functions: 0

Recommendations:
---------------
• No significant test duplicates found
  → Good job maintaining test quality!
```

### JSON Report
```json
{
  "total_tests": 16,
  "duplicates_found": 0,
  "identical_count": 0,
  "similar_count": 0,
  "duplicates": [],
  "recommendations": [
    "No significant test duplicates found. Good job maintaining test quality!"
  ]
}
```

## Algorithm

The script uses a simplified duplicate detection algorithm:

1. **Test Function Extraction**: Parses Rust files to extract test functions marked with `#[test]`
2. **Code Analysis**: Extracts setup patterns and assertion patterns from each test
3. **Similarity Calculation**: Compares test function bodies for identical code
4. **Pattern Matching**: Identifies shared setup and assertion patterns
5. **Reporting**: Generates reports with actionable recommendations

## Limitations

- Currently focuses on identical duplicate detection
- Uses simplified parsing (not full AST analysis)
- Limited to Rust test files
- Does not detect near-duplicate code with minor variations

## Future Enhancements

- Advanced similarity algorithms (AST-based comparison)
- Near-duplicate detection with fuzzy matching
- Cross-language test analysis
- Integration with CI/CD pipelines
- Automated refactoring suggestions

## Integration

This script can be integrated into:
- Pre-commit hooks
- CI/CD pipelines
- Code quality checks
- Development workflows

## Dependencies

- Bash shell
- Standard Unix tools (grep, sed, awk, find)
- No external dependencies required

## Files

- `scripts/test_duplicate_detector.sh`: Main script
- `scripts/README_test_duplicate_detector.md`: This documentation
- `target/test-analysis/`: Output directory (created automatically)</content>
</xai:function_call<parameter name="write">
<parameter name="filePath">/workspaces/do-codeguardian/scripts/README_test_duplicate_detector.md
