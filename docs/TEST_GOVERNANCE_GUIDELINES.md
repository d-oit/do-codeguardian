# Test Governance Guidelines

## 1. Introduction

This document outlines the comprehensive guidelines for managing the CodeGuardian test suite. It establishes best practices, standards, and procedures to ensure high-quality, maintainable, and reliable tests that support the project's security-first approach and performance requirements.

The guidelines cover naming conventions, organizational patterns, maintenance procedures, and quality standards to maintain consistency across the codebase and facilitate collaboration among contributors.

## 2. Naming Conventions

### Test Function Names
- Use descriptive, snake_case names that clearly indicate what is being tested
- Prefix with `test_` followed by the behavior or scenario
- Include the function/method name being tested when applicable
- Use underscores to separate words for readability

**Examples:**
```rust
#[test]
fn test_default_configuration() { ... }

#[test]
fn test_configuration_from_toml() { ... }

#[test]
fn test_real_world_rust_project() { ... }

#[tokio::test]
async fn test_analyze_file_with_vulnerabilities() { ... }
```

### Test Module Names
- Use snake_case for module names
- Group related tests in modules with descriptive names
- Use `tests` module for unit tests within source files

**Examples:**
```rust
mod configuration_tests {
    // Test functions here
}

mod security_analyzer_tests {
    // Test functions here
}
```

### Test File Names
- Use descriptive names ending with `_tests.rs` for integration tests
- Use `_test.rs` for unit tests in the same directory as the code
- Group related functionality in appropriately named files

**Examples:**
- `configuration_tests.rs`
- `e2e_integration_tests.rs`
- `security_testing_suite.rs`
- `performance_regression_tests.rs`

## 3. Organization Patterns

### Test Types and Structure

#### Unit Tests
- Located in `src/` directories alongside the code they test
- Named `*_test.rs` or within `#[cfg(test)]` modules
- Test individual functions, structs, and modules in isolation
- Use mocks/stubs for external dependencies

#### Integration Tests
- Located in `tests/` directory
- Test interactions between multiple components
- Named `*_tests.rs`
- May require real file system operations or external services

#### End-to-End (E2E) Tests
- Located in `tests/` directory with `e2e_` prefix
- Test complete workflows from CLI to output
- Use `assert_cmd` for CLI testing
- Include real-world scenarios and edge cases

#### Performance Tests
- Located in `benches/` for benchmarking
- Use `criterion` or `divan` for measurements
- Test against thresholds defined in `performance_thresholds.json`

### Directory Structure
```
tests/
├── unit/                    # Unit tests (if separated)
├── integration/            # Integration tests
├── e2e/                    # End-to-end tests
├── performance/            # Performance tests
├── fixtures/               # Test data and fixtures
├── test_config.toml        # Test configuration
└── README.md               # Test documentation
```

### Test Fixtures and Data
- Store test data in `tests/fixtures/` or `uat_test_files/`
- Use `tempfile::TempDir` for temporary test directories
- Create realistic test scenarios that mirror production use cases

## 4. Best Practices

### Writing Effective Tests

#### Test Structure
```rust
#[test]
fn test_feature_behavior() {
    // Arrange: Set up test data and preconditions
    let input = create_test_input();
    let config = Config::default();

    // Act: Execute the code under test
    let result = analyze_code(input, &config);

    // Assert: Verify the expected outcomes
    assert!(result.is_ok());
    let analysis = result.unwrap();
    assert_eq!(analysis.issues.len(), 1);
    assert!(analysis.issues[0].severity >= Severity::Medium);
}
```

#### Async Testing
- Use `#[tokio::test]` for async functions
- Handle timeouts appropriately for long-running operations
- Use `tokio::time::timeout` for operations that might hang

```rust
#[tokio::test]
async fn test_async_file_analysis() {
    let timeout = Duration::from_secs(30);
    let result = tokio::time::timeout(timeout, analyze_file_async(path)).await;
    assert!(result.is_ok());
}
```

#### Error Testing
- Test both success and failure paths
- Use `assert_matches!` for pattern matching on Results/Options
- Test specific error types and messages

```rust
#[test]
fn test_invalid_configuration() {
    let result = Config::from_file("nonexistent.toml");
    assert!(result.is_err());
    assert_matches!(result.unwrap_err().downcast_ref::<ConfigError>(),
        Some(ConfigError::FileNotFound(_)));
}
```

#### Property-Based Testing
- Use `proptest` for testing with generated inputs
- Define strategies for valid and invalid inputs
- Test edge cases and boundary conditions

```rust
proptest! {
    #[test]
    fn test_file_size_limits(size in 0..MAX_FILE_SIZE) {
        let content = vec![b'A'; size];
        let result = analyze_content(&content);
        // Test passes for all sizes within limit
        prop_assert!(result.is_ok() || size > MAX_FILE_SIZE);
    }
}
```

### Test Isolation and Cleanup
- Each test should be independent and not rely on others
- Use unique temporary directories for file operations
- Clean up resources in test teardown (automatic with `TempDir`)
- Avoid shared mutable state between tests

### Mocking and Stubbing
- Use `mockito` for HTTP service mocking
- Create test doubles for external dependencies
- Prefer integration tests over extensive mocking when possible

## 5. Standards

### Code Quality Standards
- All tests must pass `cargo clippy -- -D warnings`
- Follow `rustfmt` formatting standards (4 spaces, 100 char width)
- Use meaningful variable names and avoid abbreviations
- Add documentation comments for complex test scenarios

### Test Coverage Requirements
- Maintain >90% test coverage for new code
- Include tests for error paths and edge cases
- Cover both happy path and failure scenarios
- Use `tarpaulin` for coverage analysis

### Performance Standards
- Tests must complete within defined time thresholds
- Memory usage should not exceed configured limits
- No flaky tests that fail intermittently
- Performance tests must meet benchmarks in `performance_thresholds.json`

### Security Testing Standards
- Test for common vulnerabilities (hardcoded secrets, injection, etc.)
- Validate input sanitization and bounds checking
- Test security analyzer accuracy and false positive rates
- Include tests for path traversal and resource exhaustion attacks

## 6. Maintenance Procedures

### Adding New Tests
1. Identify the appropriate test type (unit/integration/e2e)
2. Follow naming conventions and organizational patterns
3. Add tests for new features before implementation (TDD)
4. Include both positive and negative test cases
5. Update documentation and test ownership boundaries

### Updating Existing Tests
1. Run full test suite before making changes
2. Update tests when refactoring code to maintain coverage
3. Review test failures for regressions or updated behavior
4. Update performance baselines when optimizing code
5. Remove obsolete tests that no longer provide value

### Test Suite Maintenance
- Regularly review and refactor flaky tests
- Update test data to reflect current production scenarios
- Monitor test execution time and optimize slow tests
- Archive outdated test fixtures and replace with current data
- Review test coverage reports and identify gaps

### CI/CD Integration
- All tests run on every PR and commit
- Use GitHub Actions for automated testing
- Fail builds on test failures or coverage drops
- Generate test reports and coverage badges
- Alert on performance regressions

### Test Ownership
- Assign test ownership based on component responsibility
- Document test ownership in `TEST_OWNERSHIP_BOUNDARIES.md`
- Regular rotation of test maintenance responsibilities
- Clear escalation paths for test failures

## 7. Quality Standards

### Test Quality Metrics
- **Test Coverage**: >90% line and branch coverage
- **Test Execution Time**: <60 seconds for full suite
- **Memory Usage**: <200MB during test execution
- **Flaky Test Rate**: <1% failure rate
- **Performance Regression**: <10% degradation threshold

### Code Review Standards
- All test code requires peer review
- Review focuses on test completeness and correctness
- Ensure tests follow established patterns
- Validate test isolation and cleanup procedures

### Documentation Standards
- Document complex test scenarios with comments
- Maintain test README files in test directories
- Update documentation when adding new test types
- Include examples of test patterns in contributor guides

### Reliability Standards
- Tests must be deterministic and reproducible
- No dependency on external services without proper mocking
- Handle platform-specific differences appropriately
- Test both single-threaded and multi-threaded scenarios

## 8. Tools and Integration

### Primary Testing Tools
- **cargo test**: Standard Rust testing framework
- **tokio::test**: Async testing support
- **assert_cmd**: CLI command testing
- **predicates**: Assertion predicates for CLI output
- **tempfile**: Temporary file/directory creation

### Advanced Testing Tools
- **proptest**: Property-based testing
- **criterion**: Benchmarking and performance testing
- **mockito**: HTTP service mocking
- **fake**: Test data generation

### Coverage and Analysis
- **tarpaulin**: Code coverage analysis
- **cargo-audit**: Security vulnerability checking
- **cargo clippy**: Linting and code quality
- **cargo bench**: Performance benchmarking

### CI/CD Commands
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_function_name

# Run with coverage
cargo tarpaulin --out Html

# Run benchmarks
cargo bench

# Run linting
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### Configuration Files
- `Cargo.toml`: Test dependencies and features
- `performance_thresholds.json`: Performance baselines
- `test_config.toml`: Test-specific configuration
- `.rustfmt.toml`: Formatting standards

## 9. Common Patterns and Anti-Patterns

### Recommended Patterns
- Use `#[test]` for synchronous tests
- Use `#[tokio::test]` for asynchronous tests
- Use descriptive assertion messages
- Group related tests in modules
- Use fixtures for complex test setup

### Anti-Patterns to Avoid
- Tests that depend on external state
- Hardcoded paths or absolute directories
- Tests that modify shared resources
- Ignoring test failures with `#[ignore]`
- Tests that are slow or resource-intensive

### Test Data Management
- Use realistic but minimal test data
- Avoid large files in version control
- Generate test data programmatically when possible
- Clean up test artifacts automatically

## 10. Troubleshooting and Support

### Common Issues
- **Flaky Tests**: Review for race conditions or external dependencies
- **Slow Tests**: Profile and optimize or move to integration suite
- **Test Failures**: Check for environmental differences or recent changes
- **Coverage Gaps**: Add tests for uncovered code paths

### Getting Help
- Review existing test examples in the codebase
- Consult `TEST_OWNERSHIP_BOUNDARIES.md` for component owners
- Check GitHub issues for similar test-related problems
- Reach out to the testing team for complex scenarios

This document serves as the authoritative guide for test governance in CodeGuardian. Regular reviews and updates ensure these guidelines remain current with evolving best practices and project needs.
