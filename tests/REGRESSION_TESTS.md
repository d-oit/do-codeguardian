# CodeGuardian Regression Test Suite

This directory contains comprehensive regression tests to prevent re-introduction of fixed issues and ensure the stability of improvements made to the CodeGuardian CLI.

## Test Files Overview

### 📋 `regression_tests.rs`
**Purpose**: High-level CLI regression tests to prevent re-introduction of previously fixed bugs.

**Key Test Cases**:
- `test_git_conflict_analyzer_ignores_test_content()` - Ensures git conflict analyzer doesn't flag test code with conflict markers
- `test_ai_content_analyzer_ignores_documentation()` - Validates AI analyzer ignores legitimate documentation TODOs
- `test_debug_statement_detection_accuracy()` - Confirms debug statement detection doesn't over-flag legitimate code
- `test_config_loading_graceful_degradation()` - Tests graceful handling of missing configuration files
- `test_performance_analyzer_accuracy()` - Ensures performance analyzer maintains accuracy
- `test_json_output_schema_consistency()` - Validates JSON output schema stability
- `test_sarif_output_format()` - Confirms SARIF output format works correctly
- `test_parallel_processing_stability()` - Tests parallel processing without race conditions

### 🔬 `analyzer_improvement_tests.rs`
**Purpose**: Unit tests for specific analyzer improvements and false positive fixes.

**Key Test Cases**:
- **Git Conflict Analyzer Tests**:
  - `test_git_conflict_analyzer_ignores_test_modules()` - Test module detection
  - `test_git_conflict_analyzer_detects_real_conflicts()` - Ensure real conflicts still detected
  - `test_git_conflict_analyzer_ignores_test_files()` - File path-based test detection
  - `test_git_conflict_analyzer_handles_malformed_conflicts()` - Malformed conflict handling

- **AI Content Analyzer Tests**:
  - `test_ai_content_analyzer_ignores_documentation_comments()` - Documentation comment handling
  - `test_ai_content_analyzer_ignores_test_content()` - Test context detection
  - `test_ai_content_analyzer_ignores_string_literals()` - String literal detection
  - `test_ai_content_analyzer_detects_real_incomplete_code()` - Real issue detection

- **Performance Tests**:
  - `test_performance_regression_file_processing()` - Processing speed validation
  - `test_analyzer_memory_usage_stability()` - Memory leak prevention

### 🖥️ `cli_regression_integration_tests.rs`
**Purpose**: End-to-end CLI integration tests validating real-world usage scenarios.

### ⚙️ `test_config.toml`
**Purpose**: Consistent test configuration ensuring reliable test execution.

## 🚀 Running Tests

### Quick Test Run
```bash
# Run all regression tests
cargo test --test regression_tests

# Run analyzer improvement tests
cargo test --test analyzer_improvement_tests

# Run CLI integration tests
cargo test --test cli_regression_integration_tests
```

### Comprehensive Validation
```bash
# Run the full regression test suite
./scripts/run_regression_tests.sh
```

## 📊 What These Tests Validate

### ✅ Fixed Issues Prevention
1. **Git Conflict False Positives**: Ensures test code with conflict markers doesn't trigger false alarms
2. **AI Content Over-flagging**: Validates documentation and test TODOs are properly ignored
3. **Debug Statement Accuracy**: Confirms proper logging constructs aren't flagged as debug statements
4. **Configuration Graceful Degradation**: Tests proper fallback to defaults with missing configs

### ✅ Core Functionality Preservation
1. **Real Issue Detection**: Ensures genuine security and quality issues are still caught
2. **Performance Maintenance**: Validates analysis speed hasn't regressed
3. **Output Format Stability**: Confirms JSON, SARIF, and human formats remain consistent
4. **Parallel Processing**: Tests multi-threaded analysis stability

## 🔍 Expected Test Results

### After Improvements
- **Reduced False Positives**: 60-80% reduction in false positive detections
- **Maintained Accuracy**: Real security issues still detected at same rate
- **Stable Performance**: Analysis time remains consistent
- **Better UX**: More informative output, fewer spurious warnings
