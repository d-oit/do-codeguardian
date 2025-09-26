# CodeGuardian Performance Benchmark Suite

This directory contains a comprehensive performance benchmarking suite for the CodeGuardian security analysis tool. The suite provides automated performance regression detection, load testing, metrics collection, and optimization recommendations.

## 🚀 Benchmark Suites

### 1. Performance Regression Suite (`performance_regression_suite`)
**Purpose**: Automated detection of performance regressions in core functionality

**Benchmarks**:
- `performance_regression_detection` - Core functionality regression detection
- `load_testing_integration` - Load testing scenario integration
- `performance_metrics_collection` - Comprehensive metrics collection
- `optimization_recommendations` - Generate optimization suggestions
- `regression_alerting` - Automated alerting simulation

**Key Features**:
- Memory usage regression detection
- Processing time threshold monitoring
- Cache performance analysis
- Automated baseline comparison

### 2. Load Testing Benchmark (`load_testing_benchmark`)
**Purpose**: Performance validation under various load conditions

**Benchmarks**:
- `load_testing_scenarios` - Small, medium, and large repository scenarios
- `concurrent_processing_load` - Multi-threaded processing validation
- `memory_pressure_load` - Memory-intensive operation testing
- `sustained_load` - Long-duration performance stability
- `cache_performance_under_load` - Cache efficiency under load
- `adaptive_parallelism_under_load` - Dynamic scaling validation

**Key Features**:
- Repository size scaling (10 to 2000+ files)
- Concurrent operation testing (1-16 threads)
- Memory pressure simulation
- Sustained load testing

### 3. Performance Metrics Benchmark (`performance_metrics_benchmark`)
**Purpose**: Comprehensive performance data collection and analysis

**Benchmarks**:
- `comprehensive_metrics_collection` - Full metrics gathering
- `performance_analysis_insights` - Performance trend analysis
- `automated_reporting` - Performance report generation
- `regression_detection` - Statistical regression analysis
- `metrics_aggregation` - Performance statistics aggregation
- `performance_alerting` - Threshold-based alerting

**Key Features**:
- Real-time metrics collection
- Statistical analysis and trending
- Automated report generation
- Configurable alerting thresholds

### 4. Optimization Recommendations Benchmark (`optimization_recommendations_benchmark`)
**Purpose**: Automated optimization suggestions based on performance data

**Benchmarks**:
- `optimization_recommendations` - Generate specific recommendations
- `optimization_impact_analysis` - Before/after optimization comparison
- `specific_optimization_strategies` - Targeted optimization testing
- `recommendation_prioritization` - Priority-based recommendation ranking
- `implementation_effort_analysis` - Effort estimation for optimizations
- `cost_benefit_analysis` - ROI calculation for optimizations

**Key Features**:
- Priority-based recommendations (Critical, High, Medium, Low)
- Implementation effort estimation
- Cost-benefit analysis
- Impact prediction

## 🏃 Running Benchmarks

### Run All Benchmarks
```bash
cargo bench
```

### Run Specific Benchmark Suite
```bash
# Performance regression detection
cargo bench --bench performance_regression_suite

# Load testing
cargo bench --bench load_testing_benchmark

# Metrics collection
cargo bench --bench performance_metrics_benchmark

# Optimization recommendations
cargo bench --bench optimization_recommendations_benchmark
```

### Run with Custom Measurement Time
```bash
cargo bench --bench performance_regression_suite -- --measurement-time 30
```

### Generate Comparison Reports
```bash
cargo bench --bench performance_regression_suite -- --output-format json > results.json
```

## 📊 Performance Thresholds

The benchmark suite uses configurable performance thresholds:

| Metric | Default Threshold | Description |
|--------|------------------|-------------|
| Memory Usage | 200MB | Peak memory usage limit |
| Processing Time | 2000ms | Maximum analysis time per file |
| Cache Hit Rate | 70% | Minimum cache efficiency |
| Regression Threshold | 10% | Maximum allowed performance degradation |

Thresholds can be customized via `config/performance_thresholds.json`:

```json
{
  "memory_threshold_mb": 200,
  "time_threshold_ms": 2000,
  "cache_hit_rate": 0.7,
  "regression_threshold_percent": 10
}
```

## 🔍 Regression Detection

### Automated Regression Detection
The suite automatically detects performance regressions by:

1. **Baseline Comparison**: Comparing current performance against historical baselines
2. **Threshold Monitoring**: Alerting when metrics exceed predefined thresholds
3. **Trend Analysis**: Identifying gradual performance degradation over time
4. **Statistical Analysis**: Using statistical methods to confirm regressions

### Regression Alerts
When regressions are detected, the system:
- Logs detailed regression information
- Creates GitHub issues (in CI/CD)
- Generates performance reports
- Provides specific optimization recommendations

## 📈 CI/CD Integration

### Automated Performance Monitoring
The suite integrates with GitHub Actions for automated performance monitoring:

- **Daily Performance Checks**: Scheduled runs to monitor performance trends
- **PR Performance Validation**: Performance checks on pull requests
- **Regression Alerts**: Automatic issue creation for performance regressions
- **Performance Reports**: Detailed reports with optimization recommendations

### Workflow Files
- `.github/workflows/performance-benchmark-suite.yml` - Main benchmark workflow
- `scripts/performance_regression_detector.sh` - Regression detection script

## 📋 Performance Metrics Collected

### Core Metrics
- **Processing Time**: File analysis duration
- **Memory Usage**: Peak memory consumption
- **CPU Utilization**: Processing efficiency
- **Cache Performance**: Hit rates and efficiency
- **I/O Operations**: File system interactions

### Derived Metrics
- **Throughput**: Files processed per second
- **Efficiency**: Resource utilization ratios
- **Scalability**: Performance under increased load
- **Stability**: Performance consistency over time

## 💡 Optimization Recommendations

The suite provides specific optimization recommendations based on performance analysis:

### Memory Optimizations
- Memory pool configuration tuning
- Streaming processing for large files
- Object reuse and pooling strategies

### CPU Optimizations
- Parallel processing improvements
- Algorithm complexity optimization
- Async operation optimization

### I/O Optimizations
- Caching strategy improvements
- Buffered I/O operations
- File access pattern optimization

### Cache Optimizations
- Cache size and eviction policy tuning
- Cache hit rate improvement strategies
- Memory-efficient caching approaches

## 🛠️ Configuration

### Benchmark Configuration
Benchmarks can be configured via environment variables:

```bash
# Set measurement time
export CRITERION_MEASUREMENT_TIME=30

# Set sample size
export CRITERION_SAMPLE_SIZE=100

# Enable verbose output
export CRITERION_VERBOSE=true
```

### Custom Test Data
The suite generates realistic test data automatically, but you can provide custom test repositories:

```bash
# Use custom test repository
export TEST_REPOSITORY_PATH=/path/to/custom/repo

# Set custom file sizes
export MAX_TEST_FILE_SIZE_KB=1024
```

## 📊 Reporting

### Performance Reports
The suite generates comprehensive performance reports including:

- **Executive Summary**: High-level performance status
- **Detailed Metrics**: Raw performance data and statistics
- **Trend Analysis**: Performance changes over time
- **Optimization Recommendations**: Specific improvement suggestions
- **Regression Details**: Information about detected regressions

### Report Formats
- **Markdown**: Human-readable reports
- **JSON**: Machine-readable data for automation
- **CSV**: Data for external analysis tools

## 🔧 Maintenance

### Updating Baselines
When legitimate performance changes occur, update baselines:

```bash
# Update benchmark baselines
cargo bench --bench performance_regression_suite -- --save-baseline new-baseline

# Compare against specific baseline
cargo bench --bench performance_regression_suite -- --baseline new-baseline
```

### Adding New Benchmarks
To add new benchmarks:

1. Create a new benchmark file in `benches/`
2. Add the benchmark configuration to `Cargo.toml`
3. Update CI/CD workflows if needed
4. Add documentation to this README

### Benchmark Maintenance
Regular maintenance tasks:

- Review and update performance thresholds
- Clean up old benchmark data
- Update test scenarios to reflect real usage patterns
- Validate benchmark accuracy and relevance

## 🚨 Troubleshooting

### Common Issues

**Benchmarks Failing**
- Check system resources (memory, CPU)
- Verify test data generation
- Review error logs for specific failures

**Inconsistent Results**
- Ensure stable system load during benchmarking
- Check for background processes affecting performance
- Verify consistent test data generation

**False Positives**
- Review threshold configurations
- Check for environmental factors
- Validate benchmark implementation

### Debug Mode
Run benchmarks in debug mode for detailed output:

```bash
# Enable debug logging
export RUST_LOG=debug

# Run with verbose output
cargo bench --bench performance_regression_suite -- --verbose
```

## 📚 Additional Resources

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [CodeGuardian Performance Plans](../plans/performance-optimization.md)
- [Load Testing Framework](../tests/load_testing_framework.rs)

## 🤝 Contributing

When contributing to the performance benchmark suite:

1. Follow the existing benchmark patterns
2. Add comprehensive documentation
3. Include appropriate error handling
4. Test benchmarks across different environments
5. Update this README with new functionality

---

For questions or issues with the performance benchmark suite, please create an issue in the CodeGuardian repository.
