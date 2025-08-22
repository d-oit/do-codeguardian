# CodeGuardian Agent Guide

This guide provides essential information for AI agents working with the CodeGuardian codebase.

## Build, Lint, and Test Commands

### Primary Build Commands
- **Build**: `cargo build --release`
- **Build with debug**: `cargo build`
- **Clean build**: `cargo clean && cargo build`

### Testing Commands
- **Run all tests**: `cargo test`
- **Run specific test**: `cargo test <test_name>`
- **Run tests with output**: `cargo test -- --nocapture`
- **Run benchmarks**: `cargo bench`

### Linting and Formatting
- **Format code**: `cargo fmt`
- **Check formatting**: `cargo fmt -- --check`
- **Lint with clippy**: `cargo clippy -- -D warnings`
- **Lint specific package**: `cargo clippy --package <package_name>`

### Single Test Execution
To run a specific test function:
```bash
cargo test test_function_name
```

To run tests in a specific file:
```bash
cargo test --test <test_file>
```

## Code Style Guidelines

### General Conventions
- **Edition**: Rust 2021 Edition
- **Naming**: snake_case for functions/variables, PascalCase for types/structs, SCREAMING_SNAKE_CASE for constants
- **Documentation**: Use comprehensive doc comments for public APIs
- **Error Handling**: Use `anyhow::Result<T>` for application errors, `thiserror::Error` for library error types

### Imports and Organization
```rust
// Standard library imports first
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

// External crate imports (alphabetized)
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

// Internal module imports
use crate::config::Config;
use crate::types::{AnalysisResults, Finding};
use crate::utils::progress::ProgressReporter;
```

### Error Handling Patterns
```rust
// Use anyhow::Result for application code
pub async fn analyze_files(&self, files: &[PathBuf]) -> Result<AnalysisResults> {
    // Implementation
    Ok(results)
}

// Use custom error types for libraries
#[derive(thiserror::Error, Debug)]
pub enum GuardianError {
    #[error("I/O error: {message}")]
    Io { message: String, #[source] source: std::io::Error },

    #[error("Security violation: {message}")]
    Security { message: String, severity: SecuritySeverity },
}
```

### Async/Await Patterns
```rust
// Use tokio for async runtime
#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load(&path).await?;
    process_files(config).await
}

pub async fn process_files(config: Config) -> Result<()> {
    // Implementation with proper error propagation
    Ok(())
}
```

### Security-First Patterns
```rust
// Always validate paths and check file sizes
pub fn should_analyze_file(&self, path: &Path) -> bool {
    // Skip hidden files (except specific ones)
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if name.starts_with('.') && name != ".gitignore" {
            return false;
        }
    }

    // Check file size limits (security: prevent huge files)
    if let Ok(metadata) = path.metadata() {
        if metadata.len() > 10 * 1024 * 1024 { // 10MB limit
            return false;
        }
    }

    true
}

// Use canonicalized paths to prevent path traversal
let safe_path = canonicalize_path_safe(path);
```

### Thread Safety and Concurrency
```rust
// Use Arc<Mutex<>> for shared mutable state
let cache = Arc::new(Mutex::new(FileCache::new()));

// For read-heavy workloads, consider RwLock
let config = Arc::new(RwLock::new(Config::default()));

// Use scoped threads for CPU-intensive work
let results: Vec<_> = files
    .par_iter()
    .map(|file| analyze_file(file))
    .collect();
```

### Memory Management
```rust
// Use streaming for large files
if StreamingAnalyzer::should_use_streaming(file_path) {
    self.analyze_large_file_streaming(file_path, analyzer_registry, streaming_analyzer)
} else {
    // Standard in-memory analysis for smaller files
    let content = std::fs::read(file_path)?;
    analyzer_registry.analyze_file(file_path, &content)
}
```

### Performance Optimizations
- Use `rayon` for CPU-parallel workloads
- Implement caching with `lru` crate for frequently accessed data
- Use `once_cell` for lazy static initialization
- Profile with `criterion` for benchmarks

### Testing Patterns
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_analyze_file() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.rs");

        // Test implementation
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_async_function() {
        // Async test implementation
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Configuration Management
```rust
// Use serde for configuration serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub analyzers: AnalyzerConfig,
}

// Load with validation
impl Config {
    pub async fn load(path: &Path) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let config: Self = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }
}
```

### Logging and Progress Reporting
```rust
// Use indicatif for progress bars
let progress = ProgressReporter::new();
progress.start_scan(files.len());

// TTY-aware output
if std::io::stdout().is_terminal() {
    // Show progress bar
} else {
    // Simple text output for CI
}
```

Remember: This is a security-focused codebase. Always prioritize secure defaults, validate inputs, and handle errors gracefully. Use the established patterns for consistency and maintainability.