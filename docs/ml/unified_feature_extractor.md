# Unified Feature Extractor

The Unified Feature Extractor is a comprehensive, configurable system that combines the base and enhanced feature extraction implementations into a single, maintainable solution.

## Overview

The unified extractor provides:

- **Multiple Extraction Modes**: Basic, Enhanced, AST-only, and Custom configurations
- **Runtime Configuration**: Change feature sets and settings without recompilation
- **Security Features**: File size limits, path validation, and timeout handling
- **Caching System**: LRU cache with TTL support for performance optimization
- **Backward Compatibility**: Drop-in replacement for existing feature extractors
- **Comprehensive Error Handling**: Detailed error types with actionable messages
- **Performance Monitoring**: Metrics collection and analysis

## Architecture

```rust
// Core components
UnifiedFeatureExtractor     // Main extractor with mode switching
├── FeatureConfig          // Runtime configuration
├── ExtractionMode         // Basic, Enhanced, AstOnly, Custom
├── FeatureSet            // Base, Ast, Complexity, Security
├── FileCache             // LRU cache with TTL
└── FeatureExtractionMetrics // Performance tracking
```

## Extraction Modes

### Basic Mode
- **Features**: 8 basic features (severity, file type, analyzer confidence, etc.)
- **Use Case**: Lightweight extraction for simple ML models
- **Performance**: Fastest, minimal memory usage

```rust
let config = FeatureConfig {
    mode: ExtractionMode::Basic,
    ..Default::default()
};
```

### Enhanced Mode
- **Features**: 24 features (8 base + 16 AST features)
- **Use Case**: Full feature extraction with AST analysis
- **Performance**: Moderate speed with comprehensive analysis

```rust
let config = FeatureConfig {
    mode: ExtractionMode::Enhanced,
    feature_sets: vec![FeatureSet::Base, FeatureSet::Ast],
    ..Default::default()
};
```

### AST-Only Mode
- **Features**: 16 AST-based features only
- **Use Case**: Specialized models focusing on code structure
- **Performance**: Fast AST analysis without basic features

```rust
let config = FeatureConfig {
    mode: ExtractionMode::AstOnly,
    ..Default::default()
};
```

### Custom Mode
- **Features**: User-defined combination of feature sets
- **Use Case**: Specialized feature combinations for specific use cases
- **Performance**: Depends on selected feature sets

```rust
let config = FeatureConfig {
    mode: ExtractionMode::Custom,
    feature_sets: vec![FeatureSet::Base, FeatureSet::Security],
    ..Default::default()
};
```

## Configuration

### Security Configuration

```rust
let security_config = SecurityConfig {
    max_file_size: 10 * 1024 * 1024, // 10MB limit
    allowed_extensions: vec!["rs".to_string(), "py".to_string()],
    prevent_traversal: true,
    operation_timeout: Duration::from_secs(30),
};
```

### Cache Configuration

```rust
let cache_config = CacheConfig {
    max_cache_size: 1000,           // Max cached files
    ttl: Duration::from_secs(3600), // 1 hour TTL
    compression: false,             // Cache compression
};
```

### Performance Configuration

```rust
let performance_config = PerformanceConfig {
    parallel_processing: true,
    batch_size: 10,
    memory_limit: 100 * 1024 * 1024, // 100MB
};
```

## Usage Examples

### Basic Usage

```rust
use codeguardian::ml::unified_feature_extractor::{UnifiedFeatureExtractor, FeatureConfig, ExtractionMode};

// Create with default configuration
let mut extractor = UnifiedFeatureExtractor::new();

// Or with custom configuration
let config = FeatureConfig {
    mode: ExtractionMode::Enhanced,
    ..Default::default()
};
let mut extractor = UnifiedFeatureExtractor::with_config(config);

// Extract features
let features = extractor.extract_features(&finding).await?;
println!("Extracted {} features", features.len());
```

### Runtime Configuration Changes

```rust
// Update configuration at runtime
let new_config = FeatureConfig {
    mode: ExtractionMode::Basic,
    security: SecurityConfig {
        max_file_size: 5 * 1024 * 1024, // 5MB
        ..Default::default()
    },
    ..Default::default()
};

extractor.update_config(new_config).await?;
```

### Feature Importance Analysis

```rust
// Analyze which features contribute most to the finding
let analysis = extractor.analyze_feature_importance(&finding).await?;
println!("{}", analysis);

// Output:
// Feature Importance Analysis (Mode: Enhanced):
//   Total features: 24
//   Top contributing features:
//     1. severity_score: 15.2%
//     2. file_type_relevance: 12.8%
//     3. analyzer_confidence: 10.5%
```

### Cache Management

```rust
// Get cache statistics
let stats = extractor.get_cache_stats().await;
println!("{}", stats);

// Clear cache if needed
extractor.clear_cache().await;
```

## Migration Guide

### From Base Feature Extractor

```rust
// Old way
let extractor = FeatureExtractor::new();
let features = extractor.extract_features(&finding)?;

// New way
let config = FeatureConfig {
    mode: ExtractionMode::Basic,
    ..Default::default()
};
let mut extractor = UnifiedFeatureExtractor::with_config(config);
let features = extractor.extract_features(&finding).await?;
```

### From Enhanced Feature Extractor

```rust
// Old way
let mut extractor = EnhancedFeatureExtractor::new();
let features = extractor.extract_enhanced_features(&finding).await?;

// New way
let config = FeatureConfig {
    mode: ExtractionMode::Enhanced,
    ..Default::default()
};
let mut extractor = UnifiedFeatureExtractor::with_config(config);
let features = extractor.extract_features(&finding).await?;
```

### ML Classifier Integration

The ML classifier automatically uses the unified extractor:

```rust
// The classifier now uses unified extraction internally
let mut classifier = MLClassifier::new(model_path);

// Features are extracted using the configured mode
let features = classifier.extract_features(&finding).await?;
```

## Error Handling

The unified extractor provides comprehensive error types:

```rust
use codeguardian::ml::unified_feature_extractor::FeatureExtractionError;

match extractor.extract_features(&finding).await {
    Ok(features) => println!("Success: {} features", features.len()),
    Err(e) => match e.downcast_ref::<FeatureExtractionError>() {
        Some(FeatureExtractionError::FileTooLarge { size, limit }) => {
            println!("File too large: {} bytes (limit: {} bytes)", size, limit);
        }
        Some(FeatureExtractionError::UnsupportedFileType { extension }) => {
            println!("Unsupported file type: {}", extension);
        }
        Some(FeatureExtractionError::DirectoryTraversal { path }) => {
            println!("Directory traversal detected: {}", path);
        }
        _ => println!("Other error: {}", e),
    }
}
```

## Performance Considerations

### Caching Strategy
- **LRU Eviction**: Removes least recently used entries when cache is full
- **TTL Support**: Automatic expiration of stale cache entries
- **Size Limits**: Prevents unbounded memory growth

### Memory Management
- **File Size Limits**: Prevents processing of extremely large files
- **Batch Processing**: Configurable batch sizes for parallel operations
- **Memory Limits**: Configurable memory limits for processing

### Monitoring
- **Metrics Collection**: Tracks extraction time, cache hits/misses, errors
- **Performance Analysis**: Identifies bottlenecks and optimization opportunities
- **Logging**: Comprehensive logging for debugging and monitoring

## Feature Sets

### Base Features (8)
1. `severity_score` - Finding severity (0.0-1.0)
2. `file_type_relevance` - File type importance (0.0-1.0)
3. `analyzer_confidence` - Analyzer reliability (0.0-1.0)
4. `message_length` - Message detail level (0.0-1.0)
5. `line_position` - Code location importance (0.0-1.0)
6. `has_description` - Presence of description (0.0 or 1.0)
7. `has_suggestion` - Presence of suggestion (0.0 or 1.0)
8. `rule_specificity` - Rule naming specificity (0.0-1.0)

### AST Features (16)
- Code structure analysis
- Function complexity metrics
- Import patterns
- Control flow analysis
- Data flow patterns
- Security-relevant constructs

### Complexity Features (4)
- Cyclomatic complexity
- Code line count
- Branching factor
- Nesting depth

### Security Features (4)
- Entropy analysis
- Pattern matching
- Keyword detection
- Security score

## Best Practices

### Configuration
1. **Start Simple**: Use Basic mode for initial testing
2. **Gradual Adoption**: Enable AST features incrementally
3. **Monitor Performance**: Use metrics to identify bottlenecks
4. **Security First**: Configure appropriate file size and type limits

### Error Handling
1. **Handle File Errors**: Check for file access and size issues
2. **Validate Configuration**: Ensure feature sets are compatible
3. **Monitor Cache**: Watch for cache misses and adjust TTL as needed
4. **Log Performance**: Track extraction times for optimization

### Maintenance
1. **Regular Cache Clearing**: Clear cache periodically for fresh analysis
2. **Update Configurations**: Adjust settings based on usage patterns
3. **Monitor Metrics**: Use metrics to identify performance issues
4. **Security Updates**: Keep security configurations current

## Troubleshooting

### Common Issues

**High Memory Usage**
```rust
// Reduce cache size
let config = FeatureConfig {
    cache: CacheConfig {
        max_cache_size: 500, // Reduce from default 1000
        ..Default::default()
    },
    ..Default::default()
};
```

**Slow Performance**
```rust
// Disable parallel processing if causing issues
let config = FeatureConfig {
    performance: PerformanceConfig {
        parallel_processing: false,
        ..Default::default()
    },
    ..Default::default()
};
```

**Cache Misses**
```rust
// Increase TTL for better cache hit rates
let config = FeatureConfig {
    cache: CacheConfig {
        ttl: Duration::from_secs(7200), // 2 hours instead of 1
        ..Default::default()
    },
    ..Default::default()
};
```

**File Size Errors**
```rust
// Increase file size limit for large codebases
let config = FeatureConfig {
    security: SecurityConfig {
        max_file_size: 50 * 1024 * 1024, // 50MB instead of 10MB
        ..Default::default()
    },
    ..Default::default()
};
```

## Future Enhancements

- **Machine Learning Integration**: Direct model training support
- **Advanced Caching**: Compression and distributed caching
- **Real-time Metrics**: Live performance dashboards
- **Plugin Architecture**: Extensible feature extractors
- **Configuration Profiles**: Predefined configurations for common use cases
