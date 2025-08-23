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

### Test Development Guidelines

#### Writing Effective Tests
- **Single Responsibility**: Each test should focus on one specific scenario
- **Clear Naming**: Use descriptive names that explain what the test validates
- **Comprehensive Assertions**: Check both success and failure conditions
- **Resource Cleanup**: Always clean up temporary files and directories
- **Error Handling**: Test error conditions and edge cases

#### Test Structure Best Practices
```rust
#[tokio::test]
async fn test_specific_feature_scenario() {
    // Arrange - Set up test environment
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = create_test_file(&temp_dir, "test.rs", TEST_CODE);

    // Act - Execute the functionality being tested
    let result = run_codeguardian(&["check", temp_dir.path().to_str().unwrap()]);

    // Assert - Verify expected outcomes
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("expected finding"));
}
```

#### Performance Testing
- **Timeout Limits**: Tests should complete within reasonable time limits
- **Resource Bounds**: Monitor memory and CPU usage during tests
- **Scalability**: Ensure tests work with different codebase sizes
- **Parallel Safety**: Tests should be safe to run in parallel

### Test Categories

#### Unit Tests
Located in individual module files (e.g., `src/analyzers/security_analyzer.rs`):
- Test individual functions and methods
- Mock external dependencies
- Focus on business logic

#### Integration Tests
Located in `tests/integration_tests.rs`:
- Test component interactions
- Use real dependencies where possible
- Validate data flow between components

#### End-to-End Tests
Located in `tests/e2e_*_tests.rs` files:
- Test complete user workflows
- Use temporary directories and files
- Validate CLI behavior and output

### Testing Tools and Dependencies

#### Development Dependencies
```toml
[dev-dependencies]
# Testing frameworks
assert_cmd = "2.0"        # CLI testing
predicates = "3.0"        # Output validation
tempfile = "3.13"         # Temporary file management

# Async testing
tokio-test = "0.4"        # Async test utilities

# Mocking
mockito = "1.0"           # HTTP mocking
mockall = "0.11"          # Mock generation

# Property testing
proptest = "1.0"          # Property-based testing
```

#### Testing Commands
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_function_name

# Run tests with output
cargo test -- --nocapture

# Run benchmarks
cargo bench

# Run tests with coverage
cargo tarpaulin --out Html
```

### CI/CD Testing

#### GitHub Actions Testing
```yaml
- name: Run Tests
  run: |
    cargo test --verbose
    cargo test --doc
    cargo test --test e2e_*

- name: Run Benchmarks
  run: |
    cargo bench
    # Compare against baseline if available

- name: Test Coverage
  run: |
    cargo tarpaulin --out Xml
    # Upload coverage reports
```

#### Performance Regression Testing
- **Baseline Comparison**: Compare performance against known good baselines
- **Memory Profiling**: Monitor memory usage during test execution
- **Load Testing**: Test with various codebase sizes and complexities
- **Resource Monitoring**: Track CPU, memory, and I/O during tests

### Debugging Test Failures

#### Common Issues
1. **Flaky Tests**: Tests that pass/fail intermittently
2. **Resource Leaks**: Tests that don't clean up properly
3. **Timing Issues**: Tests that depend on specific timing
4. **Environment Dependencies**: Tests that require specific environments

#### Debugging Techniques
```rust
// Enable debug logging in tests
std::env::set_var("RUST_LOG", "debug");

// Add debug output to tests
println!("Debug: {:?}", debug_info);

// Use temporary directories for isolation
let temp_dir = tempfile::tempdir().unwrap();
println!("Test directory: {:?}", temp_dir.path());
```

#### Test Isolation
- **Unique Names**: Use unique identifiers for test resources
- **Port Management**: Use dynamic ports for network tests
- **File System Isolation**: Use separate directories for each test
- **Database Isolation**: Use separate databases or in-memory instances

### Test Data Management

#### Test Data Sources
- **Static Test Files**: Fixed test cases in `tests/data/`
- **Generated Content**: Dynamically generated test files
- **Real-world Examples**: Sanitized examples from real projects
- **Edge Cases**: Minimal examples that trigger specific conditions

#### Test Data Best Practices
- **Deterministic**: Test data should produce consistent results
- **Minimal**: Use smallest possible test cases
- **Representative**: Reflect real-world usage patterns
- **Versioned**: Keep test data in version control

### Continuous Testing

#### Pre-commit Hooks
```bash
# Install pre-commit hooks
cargo install cargo-husky

# Run tests before commits
cargo test --lib
```

#### Automated Testing
- **Pull Request Testing**: Run full test suite on PRs
- **Nightly Testing**: Run extended tests nightly
- **Release Testing**: Comprehensive testing before releases
- **Performance Monitoring**: Track performance over time

### Test Metrics and Reporting

#### Key Metrics
- **Test Coverage**: Line and branch coverage percentages
- **Test Duration**: Time taken to run test suites
- **Flakiness Rate**: Percentage of tests that are flaky
- **Success Rate**: Overall test pass/fail rates

#### Reporting
- **JUnit XML**: For CI/CD integration
- **HTML Reports**: Human-readable test reports
- **Coverage Reports**: Detailed coverage analysis
- **Performance Reports**: Test execution performance metrics

### Advanced Testing Topics

#### Fuzz Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_with_random_input(input in any::<String>()) {
        // Test with random input
        let result = process_input(&input);
        // Assert invariants
        assert!(result.is_valid());
    }
}
```

#### Load Testing
```rust
#[tokio::test]
async fn test_concurrent_analysis() {
    let mut handles = vec![];

    for _ in 0..100 {
        let handle = tokio::spawn(async {
            // Simulate concurrent analysis
            run_analysis().await
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}
```

#### Integration Testing with External Services
```rust
#[tokio::test]
async fn test_github_integration() {
    // Mock GitHub API
    let mut server = mockito::Server::new();

    let _mock = server.mock("GET", "/repos/owner/repo/issues")
        .with_status(200)
        .with_body(r#"[]"#)
        .create();

    // Test GitHub integration
    let result = create_github_issue(&server.url()).await;
    assert!(result.is_ok());
}
```

This comprehensive testing approach ensures CodeGuardian maintains high quality and reliability across all features and use cases.