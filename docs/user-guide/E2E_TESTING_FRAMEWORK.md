# CodeGuardian End-to-End Testing Framework

## Overview

This document describes the comprehensive end-to-end testing framework implemented for CodeGuardian, providing robust testing capabilities for complete workflows, CLI integration, error handling, performance regression detection, and cross-platform validation.

## Components

### 1. E2ETestRunner with Scenario Management (`tests/e2e_test_runner.rs`)

#### Features
- **Structured Test Scenarios**: Define test scenarios with setup, test, and cleanup phases
- **Parallel Execution**: Run scenarios concurrently for faster test execution
- **Result Reporting**: Generate detailed JSON reports with performance metrics
- **Baseline Tracking**: Track performance baselines and detect regressions
- **Platform Filtering**: Filter scenarios by supported platforms
- **Category Organization**: Organize tests by functionality categories

#### Key Structures
```rust
pub struct TestScenario {
    pub name: String,
    pub description: String,
    pub category: ScenarioCategory,
    pub priority: TestPriority,
    pub platforms: Vec<String>,
    pub timeout: Duration,
    pub setup_steps: Vec<TestStep>,
    pub test_steps: Vec<TestStep>,
    pub cleanup_steps: Vec<TestStep>,
    pub expected_results: ExpectedResults,
    pub tags: Vec<String>,
}
```

#### Usage Example
```rust
let scenario = TestScenario {
    name: "full_development_workflow".to_string(),
    description: "Complete development workflow from project creation to security analysis".to_string(),
    category: ScenarioCategory::BasicWorkflow,
    priority: TestPriority::Critical,
    platforms: vec!["linux".to_string(), "macos".to_string(), "windows".to_string()],
    timeout: Duration::from_secs(300),
    // ... setup, test, cleanup steps
    expected_results: ExpectedResults {
        success: true,
        exit_code: Some(0),
        output_contains: vec!["hardcoded_secret".to_string()],
        files_created: vec!["codeguardian.toml".to_string()],
        performance_requirements: Some(PerformanceRequirements {
            max_duration: Duration::from_secs(120),
            max_memory_mb: Some(500),
            min_files_per_second: Some(1.0),
        }),
    },
    tags: vec!["workflow".to_string(), "security".to_string(), "git".to_string()],
};
```

### 2. Enhanced CLI Integration Testing (`tests/e2e_cli_tests.rs`)

#### Test Coverage
- **Basic Commands**: Help, version, core functionality
- **Configuration Management**: Config file parsing, validation, environment variables
- **Check Command**: All output formats (human, JSON, SARIF), multiple files, diff mode
- **Git Integration**: git-commit, git-commit-push commands
- **Report Generation**: Convert results to various formats (Markdown, HTML, JSON)
- **GitHub Integration**: Issue creation and management
- **Performance Testing**: Large file handling, parallel processing, scaling
- **Error Handling**: Invalid paths, malformed configs, edge cases
- **Output Consistency**: Format validation and consistency

#### Key Test Functions
- `test_cli_help_and_basic_commands()` - Basic CLI functionality
- `test_cli_config_file_handling()` - Configuration file processing
- `test_cli_check_command_comprehensive()` - Check command variations
- `test_cli_git_integration()` - Git workflow integration
- `test_cli_report_generation()` - Report format conversion
- `test_cli_performance_and_scaling()` - Performance characteristics
- `test_cli_error_handling_and_edge_cases()` - Error scenarios
- `test_cli_configuration_validation()` - Config validation
- `test_cli_output_formatting_consistency()` - Output format testing

### 3. Error Handling Validation (`tests/e2e_error_scenarios.rs`)

#### Error Categories Tested
- **Filesystem Errors**: Permission denied, non-existent files, corrupted files
- **File Content Issues**: Binary files, empty files, invalid UTF-8, large files
- **Directory Structures**: Deep nesting, symlinks, concurrent access
- **Resource Exhaustion**: Memory pressure, file handle limits, CPU limits
- **Network Errors**: GitHub API failures, timeout handling
- **Configuration Errors**: Malformed TOML, invalid values, missing files
- **Encoding Issues**: Unicode handling, character set problems
- **Environmental Factors**: System limits, timezone issues, locale settings
- **Signal Handling**: Interruption recovery, graceful shutdown

#### Key Test Functions
- `test_filesystem_permission_errors()` - File permission handling
- `test_corrupted_file_handling()` - Malformed file processing
- `test_large_file_handling()` - Large file processing
- `test_deep_directory_structures()` - Complex directory trees
- `test_symlink_handling()` - Symbolic link processing
- `test_concurrent_access_handling()` - Multi-threaded access
- `test_memory_pressure_handling()` - Memory constraint handling
- `test_encoding_and_charset_issues()` - Character encoding
- `test_resource_exhaustion_handling()` - System resource limits

### 4. Performance Regression Testing (`tests/e2e_performance_tests.rs`)

#### Features
- **Baseline Tracking**: Store and compare performance baselines
- **Regression Detection**: Automatic detection of performance degradation
- **Scaling Tests**: Test performance with increasing load
- **Memory Profiling**: Monitor memory usage patterns
- **Concurrent Performance**: Test parallel processing efficiency
- **Incremental Analysis**: Test diff-mode performance
- **CI/CD Optimization**: Test performance in automated environments
- **ML Performance**: Test machine learning feature performance
- **Continuous Monitoring**: Ongoing performance tracking

#### Key Components
```rust
struct PerformanceRegressionDetector {
    baselines: HashMap<String, PerformanceBaseline>,
    baseline_file: PathBuf,
    regression_threshold_percent: f64,
}

struct PerformanceResult {
    scenario_name: String,
    duration: Duration,
    file_count: usize,
    total_size_bytes: u64,
    throughput_files_per_sec: f64,
    throughput_bytes_per_sec: f64,
    memory_usage_mb: Option<u64>,
    cpu_usage_percent: Option<f64>,
}
```

#### Key Test Functions
- `test_large_codebase_performance()` - Large project analysis
- `test_memory_usage_large_files()` - Memory consumption testing
- `test_concurrent_analysis_performance()` - Parallel processing
- `test_incremental_analysis_performance()` - Diff analysis speed
- `test_memory_pressure_performance()` - Memory constraint testing
- `test_mixed_filetype_performance()` - Multi-language projects
- `test_performance_scaling()` - Scalability testing
- `test_ci_cd_performance()` - CI/CD environment optimization
- `test_continuous_performance_monitoring()` - Ongoing monitoring

### 5. Cross-Platform Validation (`tests/cross_platform_tests.rs`)

#### Platform Support
- **Path Handling**: Cross-platform path separators, long paths, Unicode paths
- **File Permissions**: Unix permissions, Windows ACLs, access control
- **Line Endings**: CRLF vs LF handling, mixed line endings
- **Case Sensitivity**: Case-insensitive filesystems, case-preserving systems
- **File Attributes**: Hidden files, system files, special file types
- **Unicode Support**: International characters, emoji, multi-byte sequences
- **Reserved Names**: Platform-specific reserved filenames
- **Environment Variables**: Cross-platform environment handling
- **Timezones**: Timestamp handling across timezones
- **Resource Limits**: Platform-specific limits and constraints

#### Platform-Specific Tests
- **Windows**: Path separators, reserved names, long paths
- **Linux**: File permissions, symbolic links, resource limits
- **macOS**: Resource forks, extended attributes, HFS+ features

#### Key Test Functions
- `test_path_handling_consistency()` - Path normalization
- `test_line_ending_handling()` - Line ending processing
- `test_file_permission_handling()` - Permission systems
- `test_environment_variable_handling()` - Environment integration
- `test_unicode_handling()` - International character support
- `test_case_sensitivity_handling()` - Case handling
- `test_large_file_handling()` - Large file support
- `test_concurrent_file_access()` - Multi-process access
- `test_path_separator_normalization()` - Path standardization

## Test Execution

### Running Tests

```bash
# Run all E2E tests
cargo test --test e2e_*

# Run specific test suites
cargo test --test e2e_cli_tests
cargo test --test e2e_error_scenarios
cargo test --test e2e_performance_tests
cargo test --test cross_platform_tests

# Run with verbose output
cargo test --test e2e_test_runner -- --nocapture

# Run specific test function
cargo test --test e2e_cli_tests test_cli_help_and_basic_commands
```

### Test Configuration

Tests can be configured through environment variables:

```bash
# Enable performance regression detection
export CODEGUARDIAN_TEST_PERFORMANCE_REGRESSION=true

# Set performance regression threshold
export CODEGUARDIAN_PERFORMANCE_THRESHOLD=15.0

# Enable cross-platform testing
export CODEGUARDIAN_CROSS_PLATFORM_TEST=true

# Set test timeout
export CODEGUARDIAN_TEST_TIMEOUT=300
```

### CI/CD Integration

The testing framework is designed for CI/CD environments:

- **Parallel Execution**: Tests can run in parallel for faster execution
- **Result Reporting**: JSON output for integration with CI tools
- **Performance Baselines**: Automatic baseline tracking and regression detection
- **Cross-Platform**: Tests run consistently across different platforms
- **Resource Aware**: Tests respect system resource constraints

## Test Categories

### By Functionality
- **Basic Workflow**: Complete development workflows
- **Security Analysis**: Security scanning capabilities
- **Git Integration**: Git workflow integration
- **Performance**: Performance characteristics and scaling
- **Error Handling**: Robustness under failure conditions
- **Cross-Platform**: Platform compatibility
- **CI/CD**: Automated environment integration
- **ML Training**: Machine learning features
- **Dashboard**: Web interface functionality
- **Bulk Operations**: Multi-repository operations

### By Priority
- **Critical**: Core functionality that must work
- **High**: Important features with broad impact
- **Medium**: Standard features
- **Low**: Nice-to-have features

## Performance Monitoring

### Baseline Management
- Automatic baseline creation on first run
- Baseline updates when performance improves
- Regression alerts when performance degrades
- Historical performance tracking

### Metrics Collected
- **Duration**: Analysis time for different scenarios
- **Throughput**: Files and bytes processed per second
- **Memory Usage**: Peak memory consumption
- **CPU Usage**: Processing efficiency
- **File Counts**: Scalability with project size
- **Error Rates**: Reliability under various conditions

### Regression Detection
- Configurable regression thresholds
- Multiple regression types (duration, memory, throughput)
- Detailed regression reports
- Historical trend analysis

## Error Handling Strategy

### Graceful Degradation
- Tests continue when non-critical components fail
- Partial success scenarios are handled
- Resource exhaustion is managed gracefully
- Network failures don't break local functionality

### Comprehensive Coverage
- Filesystem edge cases
- Network and API failures
- Configuration errors
- Encoding and parsing issues
- Resource constraints
- Platform-specific limitations

## Cross-Platform Compatibility

### Path Handling
- Automatic path separator normalization
- Unicode path support
- Long path handling (Windows extended paths)
- Case sensitivity awareness

### File System Features
- Permission model abstraction
- Symbolic link handling
- Special file type recognition
- Filesystem-specific limitations

### Environment Integration
- Platform-specific environment variables
- Locale and encoding settings
- Timezone handling
- System resource availability

## Future Enhancements

### Planned Features
- **Distributed Testing**: Run tests across multiple machines
- **Load Testing**: Simulate high-concurrency scenarios
- **Fuzz Testing**: Automated input generation and testing
- **Integration Testing**: Test with external services
- **Performance Profiling**: Detailed performance analysis tools
- **Test Orchestration**: Complex multi-system test scenarios

### Monitoring and Analytics
- **Test Analytics**: Success rates, failure patterns, performance trends
- **Automated Reporting**: Generate comprehensive test reports
- **Alerting**: Notify on test failures or performance regressions
- **Historical Analysis**: Long-term trend analysis and forecasting

This comprehensive E2E testing framework ensures CodeGuardian's reliability, performance, and cross-platform compatibility through systematic, automated testing of all critical workflows and edge cases.
