# CodeGuardian End-to-End Test Suite

This directory contains comprehensive end-to-end tests for the CodeGuardian CLI tool, covering complete user workflows from command invocation to output generation.

## Test Structure

### Core CLI Tests (`e2e_cli_tests.rs`)
- **Basic CLI functionality**: Help, version, argument parsing
- **Output formats**: JSON, Markdown, SARIF
- **File filtering**: Include/exclude patterns
- **Configuration**: Custom config files
- **Error handling**: Invalid paths, malformed configs

### Integration Tests (`e2e_integration_tests.rs`)
- **Real-world projects**: Rust, JavaScript, Python codebases
- **Multi-language analysis**: Cross-language security detection
- **Project structure**: Complex directory hierarchies

### Workflow Tests (`e2e_workflow_tests.rs`)
- **Complete analysis workflow**: Init → Check → Report
- **CI/CD pipeline simulation**: Automated analysis flows
- **Report generation**: Multiple output formats

### Performance Tests (`e2e_performance_tests.rs`)
- **Large codebase handling**: 50+ files, concurrent analysis
- **Memory usage**: Large files, deep directories
- **Parallel processing**: Multi-worker performance

### Error Scenario Tests (`e2e_error_scenarios.rs`)
- **Edge cases**: Permission denied, corrupted files
- **Resilience**: Large files, deep nesting, symlinks
- **Recovery**: Graceful handling of interruptions

### Feature Tests (`e2e_feature_tests.rs`)
- **ML integration**: False positive reduction with RUV-FANN
- **Enhanced caching**: Intelligent file caching with mtime/hash checking
- **GitHub integration**: Issue creation with rate limiting and retry logic
- **Custom patterns**: User-defined security rules and patterns
- **Baseline mode**: Incremental analysis with diff support
- **Turbo mode**: High-performance analysis testing
- **Streaming analysis**: Large file processing validation
- **Security enhancements**: Enhanced security checks and validation

### Test Utilities (`e2e_test_runner.rs`)
- **Helper functions**: Project creation, git setup
- **Comprehensive workflows**: Full development cycle testing

## Running Tests

### All E2E Tests
```bash
cargo test --test e2e_cli_tests
cargo test --test e2e_integration_tests
cargo test --test e2e_workflow_tests
cargo test --test e2e_performance_tests
cargo test --test e2e_error_scenarios
cargo test --test e2e_feature_tests
```

### Specific Test Categories
```bash
# Basic CLI functionality
cargo test --test e2e_cli_tests test_cli_help_command

# Performance testing
cargo test --test e2e_performance_tests test_large_codebase_performance

# Error handling
cargo test --test e2e_error_scenarios test_permission_denied_handling

# Feature testing
cargo test --test e2e_feature_tests test_ml_integration_workflow
```

### All Integration Tests
```bash
cargo test --test "*e2e*"
```

## Test Coverage

### CLI Commands Tested
- ✅ `codeguardian check` - Core analysis functionality with enhanced options
- ✅ `codeguardian report` - Report format conversion with new formats
- ✅ `codeguardian init` - Configuration initialization with enhanced defaults
- ✅ `codeguardian turbo` - High-performance analysis with new performance options
- ✅ `codeguardian gh-issue` - GitHub integration with rate limiting
- ✅ `codeguardian train` - ML model training and validation
- ✅ `codeguardian metrics` - ML model performance metrics
- ✅ `codeguardian --help` - Help documentation
- ✅ `codeguardian --version` - Version information

### Output Formats Tested
- ✅ JSON - Machine-readable results
- ✅ Markdown - Human-readable reports
- ✅ SARIF - Static analysis standard format

### File Types Tested
- ✅ Rust (`.rs`) - Primary language support
- ✅ JavaScript (`.js`) - Web development
- ✅ Python (`.py`) - Scripting and data science
- ✅ TypeScript (`.ts`) - Typed JavaScript
- ✅ Configuration files (`.toml`, `.json`, `.yaml`)

### Security Issues Tested
- ✅ Hardcoded secrets and API keys with enhanced pattern detection
- ✅ SQL injection vulnerabilities with context-aware analysis
- ✅ XSS vulnerabilities with improved detection algorithms
- ✅ Command injection with shell escaping validation
- ✅ Weak cryptography with modern standard compliance
- ✅ Non-production code markers with escalation tracking
- ✅ Path traversal vulnerabilities with canonical path validation
- ✅ Resource exhaustion with file size and memory limits
- ✅ Information disclosure with secret redaction
- ✅ Authentication bypass with enhanced security checks

### Performance Scenarios
- ✅ Large codebases (50+ files)
- ✅ Concurrent analysis (multi-worker)
- ✅ Memory-intensive operations
- ✅ Deep directory structures
- ✅ Large individual files

### Error Conditions
- ✅ Permission denied
- ✅ Invalid file formats
- ✅ Corrupted files
- ✅ Network timeouts
- ✅ Configuration errors
- ✅ Resource exhaustion

## Test Data

Tests use temporary directories and dynamically generated content to ensure:
- **Isolation**: No interference between tests
- **Reproducibility**: Consistent results across runs
- **Cleanup**: Automatic resource cleanup
- **Realism**: Representative real-world scenarios

## Dependencies

The E2E tests require additional dependencies:
```toml
[dev-dependencies]
assert_cmd = "2.0"    # CLI testing framework
predicates = "3.0"    # Output validation
tempfile = "3.13"     # Temporary file management
```

## Best Practices

### Test Design
- **Single responsibility**: Each test focuses on one scenario
- **Clear naming**: Test names describe the scenario
- **Comprehensive assertions**: Validate both success and failure cases
- **Resource cleanup**: Use temporary directories

### Performance Considerations
- **Timeout limits**: Tests complete within reasonable time
- **Resource bounds**: Memory and CPU usage monitoring
- **Parallel safety**: Tests can run concurrently

### Maintenance
- **Regular updates**: Keep tests current with CLI changes
- **Documentation**: Clear test purpose and expectations
- **Debugging**: Helpful error messages and logging

## Continuous Integration

These tests are designed to run in CI environments:
- **Fast execution**: Most tests complete in seconds
- **Reliable**: Minimal flakiness and external dependencies
- **Comprehensive**: Cover critical user workflows
- **Informative**: Clear failure reporting

## Contributing

When adding new E2E tests:
1. Choose the appropriate test file based on category
2. Follow existing naming conventions
3. Use helper functions from `e2e_test_runner.rs`
4. Include both positive and negative test cases
5. Document complex test scenarios
6. Ensure tests are deterministic and fast