# CodeGuardian Performance Optimization Guide

This guide provides comprehensive information about the performance optimizations implemented in CodeGuardian and how to use them effectively.

## Overview

CodeGuardian has been optimized through multiple phases to provide enterprise-grade performance for code analysis. The optimizations focus on four key areas:

1. **Critical Fixes** - Core engine improvements and async I/O
2. **Performance Enhancements** - Persistent caching and adaptive parallelism
3. **Memory Optimization** - Efficient memory allocation patterns
4. **Advanced Streaming** - Multi-format file processing

## Quick Start

### Automatic Optimization

CodeGuardian can automatically detect your environment and apply optimal settings:

```rust
use codeguardian::config::optimization_presets::{ConfigurationOptimizer, OptimizationScenario};

let optimizer = ConfigurationOptimizer::new();
let optimal_config = optimizer.get_optimized_config();

// Or analyze your codebase
let scenario = optimizer.analyze_codebase(file_count, total_size_mb);
let config = scenario.get_config();
```

### Manual Configuration

Choose from predefined optimization scenarios:

```rust
use codeguardian::config::{Config, PerformanceConfig};

// Development environment - fast feedback
let dev_config = Config::development_optimized();

// CI/CD environment - consistent performance
let ci_config = Config::ci_optimized();

// Large codebase - maximum throughput
let large_config = Config::large_codebase_optimized();

// Memory-constrained environment
let memory_config = Config::memory_constrained_optimized();
```

## Optimization Scenarios

### Development Environment
- **Use Case**: Fast feedback during development
- **Characteristics**:
  - Fast scan speeds
  - Moderate memory usage
  - Standard analysis depth
  - Adaptive parallelism

### CI/CD Environment
- **Use Case**: Automated code analysis in CI pipelines
- **Characteristics**:
  - Balanced performance
  - Consistent results
  - Fixed parallelism
  - Optimized for reliability

### Large Codebase
- **Use Case**: Enterprise-scale codebases
- **Characteristics**:
  - Maximum throughput
  - High memory usage
  - Optimized analyzers
  - Maximum parallelism

### Memory-Constrained
- **Use Case**: Systems with limited RAM
- **Characteristics**:
  - Minimal memory usage
  - Streaming analysis
  - Reduced parallelism
  - Conservative resource usage

### Security Audit
- **Use Case**: Thorough security analysis
- **Characteristics**:
  - Maximum analysis depth
  - All security checks enabled
  - Longer analysis time
  - Comprehensive reporting

## Performance Monitoring

### Real-time Monitoring

Enable performance monitoring to track optimization effectiveness:

```rust
use codeguardian::utils::performance_monitor::{create_performance_monitor, create_performance_dashboard};

let monitor = create_performance_monitor();
let dashboard = create_performance_dashboard(monitor.clone());

// Start monitoring
dashboard.start().await;

// Monitor will automatically track metrics and generate alerts
```

### Performance Metrics

The system tracks comprehensive metrics:

- **Scan Duration**: Time to analyze files
- **Files Processed**: Number of files analyzed per second
- **Cache Hit Rate**: Percentage of cache hits vs misses
- **Memory Usage**: Peak and average memory consumption
- **CPU Usage**: Processor utilization during analysis
- **Parallel Workers**: Number of active worker threads
- **Streaming Operations**: Files processed via streaming

### Alerts and Thresholds

The system generates alerts for performance issues:

- **High Memory Usage**: Memory usage exceeds 1GB
- **High CPU Usage**: CPU usage exceeds 90%
- **Slow Scan Performance**: Scan takes longer than 60 seconds
- **Low Cache Hit Rate**: Cache hit rate below 50%
- **Memory Pool Exhaustion**: Memory pools running low

## Caching System

### Persistent Caching

CodeGuardian uses advanced persistent caching:

```rust
use codeguardian::cache::FileCache;

// Cache is automatically loaded and saved
let cache = FileCache::load().await?;

// Cache supports compression (default: enabled)
let compressed_cache = FileCache::new_with_config(true, true, 10000);

// Manual cache management
cache.perform_maintenance(7, 100).await?; // 7 days, 100MB limit
```

### Cache Features

- **Automatic Compression**: Reduces disk space by ~70%
- **Integrity Verification**: Ensures cache consistency
- **Migration Support**: Handles version upgrades
- **Size Management**: Automatic cleanup of old entries
- **Performance Statistics**: Detailed cache performance metrics

### Cache Configuration

```rust
// Configure cache behavior
let cache_config = FileCache::new_with_config(
    true,   // compression enabled
    true,   // auto-save enabled
    10000,  // max entries
);
```

## Memory Optimization

### Memory Pool System

CodeGuardian uses intelligent memory pooling:

```rust
use codeguardian::utils::memory_pool::thread_local_pools;

// Initialize thread-local pools
thread_local_pools::init();

// Use pooled string buffers
let buffer = thread_local_pools::get_string_buffer();
// ... use buffer ...
thread_local_pools::put_string_buffer(buffer);

// Use pooled vector allocations
let findings = thread_local_pools::get_findings_vec();
// ... use findings vector ...
thread_local_pools::put_findings_vec(findings);
```

### Pre-allocated Vectors

Analyzers use pre-allocated vectors to reduce reallocations:

```rust
// Instead of Vec::new()
let findings = Vec::with_capacity(20); // Estimate based on typical file size
```

## Adaptive Parallelism

### System Load Monitoring

CodeGuardian adapts parallelism based on system load:

```rust
use codeguardian::utils::adaptive_parallelism::{AdaptiveParallelismController, SystemLoadMonitor};

let controller = AdaptiveParallelismController::new(1, 8, 4);
let monitor = SystemLoadMonitor::new(controller.clone());

// Start monitoring
monitor.start_monitoring().await;

// Controller automatically adjusts worker count based on load
let current_workers = controller.current_workers();
```

### Load Metrics

The system monitors:
- **CPU Usage**: Processor utilization
- **Memory Usage**: RAM consumption
- **I/O Wait**: Disk I/O blocking
- **Load Average**: System load average

## Streaming Analysis

### Multi-Format Support

CodeGuardian supports streaming for various file types:

```rust
use codeguardian::streaming::StreamingAnalyzer;

let streaming = StreamingAnalyzer::new();

// Automatically detects file type and uses appropriate streaming method
let findings = streaming.analyze_large_file(file_path, analyzer_fn).await?;
```

### Supported Formats

- **Text Files**: Line-by-line processing
- **JSON Files**: Structure-aware parsing
- **CSV Files**: Row-by-row processing
- **Log Files**: Entry-based parsing
- **Compressed Files**: Automatic decompression
- **Binary Files**: Chunk-based processing

### Adaptive Chunking

Streaming uses adaptive chunk sizes:

```rust
let config = streaming.get_streaming_config(file_path);
println!("Optimal chunk size: {} bytes", config.chunk_size);
println!("Parallel processing: {}", config.use_parallel);
```

## Configuration Optimization

### Performance Configuration

Fine-tune performance settings:

```rust
use codeguardian::config::PerformanceConfig;

// Create custom performance configuration
let mut perf_config = PerformanceConfig::default();
perf_config.max_parallel_workers = 8;
perf_config.max_memory_file_size = 50 * 1024 * 1024; // 50MB
perf_config.pattern_cache_size = 2000;
perf_config.enable_optimized_analyzers = true;

// Validate configuration
perf_config.validate()?;
```

### Configuration Validation

All configurations are validated:

```rust
let config = Config::load("codeguardian.toml")?;
config.validate()?; // Returns error if invalid

// Get configuration recommendations
let recommendations = config.performance.get_recommendations();
for rec in recommendations {
    println!("Recommendation: {}", rec);
}
```

## Benchmarking

### Running Benchmarks

Use the built-in benchmarking suite:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark group
cargo bench --bench performance_benchmark -- bench_cache_operations

# Run with flamegraph for profiling
cargo flamegraph --bench performance_benchmark
```

### Custom Benchmarks

Create custom benchmarks:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn custom_benchmark(c: &mut Criterion) {
    c.bench_function("my_analysis", |b| {
        b.iter(|| {
            // Your benchmark code here
            black_box(analyze_code());
        });
    });
}
```

## Integration Testing

### Full Pipeline Testing

Test the complete optimization pipeline:

```rust
use codeguardian::tests::integration_tests;

#[tokio::test]
async fn test_full_optimization_pipeline() {
    // Tests all optimization components working together
    integration_tests::test_full_optimization_pipeline().await;
}
```

### Component Testing

Test individual optimization components:

```rust
#[tokio::test]
async fn test_cache_optimization() {
    integration_tests::test_cache_optimization_integration().await;
}

#[tokio::test]
async fn test_streaming_optimization() {
    integration_tests::test_streaming_optimization_integration().await;
}
```

## Performance Tuning

### Memory Tuning

- **Increase memory pools**: Larger pools reduce allocation overhead
- **Adjust streaming thresholds**: Lower thresholds for memory-constrained systems
- **Monitor memory usage**: Use performance dashboard for real-time monitoring

### CPU Tuning

- **Adjust worker counts**: Match parallelism to available cores
- **Tune adaptive thresholds**: Adjust load thresholds for your environment
- **Monitor CPU usage**: Watch for throttling and adjust accordingly

### I/O Tuning

- **Cache optimization**: Increase cache size for frequently analyzed codebases
- **Streaming configuration**: Adjust chunk sizes based on storage performance
- **Compression settings**: Balance compression ratio vs CPU overhead

## Troubleshooting

### Common Issues

1. **High Memory Usage**
   - Reduce `max_memory_file_size`
   - Enable streaming for large files
   - Use memory-constrained configuration

2. **Slow Performance**
   - Check cache hit rate (>50% recommended)
   - Verify parallel worker utilization
   - Monitor system load and adjust thresholds

3. **Cache Inefficiency**
   - Clear cache if corrupted: `rm .codeguardian-cache.json.gz`
   - Adjust cache size limits
   - Check file modification patterns

4. **Streaming Issues**
   - Verify file type detection
   - Check chunk size configuration
   - Monitor I/O performance

### Performance Alerts

Respond to performance alerts:

- **High Memory Usage**: Switch to memory-optimized configuration
- **Low Cache Hit Rate**: Review caching strategy and file patterns
- **Slow Scan Performance**: Check system resources and configuration
- **Parallelism Issues**: Verify worker count and system load

## Best Practices

### Development Workflow

1. **Use Development Configuration**: Fast feedback during development
2. **Enable Performance Monitoring**: Track optimization effectiveness
3. **Regular Cache Maintenance**: Clean up stale cache entries
4. **Monitor Resource Usage**: Watch memory and CPU consumption

### CI/CD Integration

1. **Use CI Configuration**: Consistent performance in automated environments
2. **Cache Persistence**: Maintain cache between CI runs
3. **Resource Limits**: Set appropriate timeouts and memory limits
4. **Performance Baselines**: Monitor for performance regressions

### Large-Scale Deployment

1. **Use Large Codebase Configuration**: Optimized for scale
2. **Distributed Caching**: Consider shared cache for multiple instances
3. **Resource Monitoring**: Implement comprehensive monitoring
4. **Performance Tuning**: Regular performance analysis and tuning

## API Reference

### Core Components

- `ConfigurationOptimizer`: Automatic configuration optimization
- `PerformanceMonitor`: Real-time performance tracking
- `AdaptiveParallelismController`: Dynamic worker management
- `FileCache`: Persistent caching with compression
- `StreamingAnalyzer`: Multi-format streaming analysis
- `GlobalMemoryPools`: Memory pool management

### Configuration Options

- `PerformanceConfig`: Core performance settings
- `OptimizationScenario`: Predefined optimization profiles
- `StreamingConfig`: Streaming analysis configuration
- `CacheConfig`: Cache behavior settings

## Contributing

When contributing performance optimizations:

1. **Add Benchmarks**: Include benchmarks for new features
2. **Update Documentation**: Document performance characteristics
3. **Integration Tests**: Add tests for optimization components
4. **Performance Monitoring**: Include performance metrics
5. **Configuration Options**: Make optimizations configurable

## Support

For performance optimization support:

1. **Check Documentation**: Review this guide and API documentation
2. **Run Benchmarks**: Use built-in benchmarks to identify bottlenecks
3. **Monitor Performance**: Use the performance dashboard for insights
4. **Configuration Tuning**: Experiment with different optimization scenarios
5. **Community Support**: Check GitHub issues and discussions

---

*This guide covers the comprehensive performance optimization features implemented in CodeGuardian. For the latest updates and additional optimization techniques, check the project documentation and release notes.*