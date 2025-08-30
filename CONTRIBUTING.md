# Contributing Guide

We welcome contributions! This guide will help you get started with contributing to CodeGuardian.

## Development Setup

### Prerequisites

- **Rust 1.70+** with 2021 edition
- **Git** for version control
- **Cargo** for building and testing
- **GitHub account** for contributing

### Quick Start

```bash
# Clone the repository
git clone https://github.com/d-oit/codeguardian
cd codeguardian

# Build the project
cargo build

# Run tests
cargo test

# Check code quality
cargo clippy -- -D warnings
cargo fmt -- --check
```

### Development Workflow

1. **Fork** the repository on GitHub
2. **Clone** your fork locally
3. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
4. **Make** your changes
5. **Test** your changes (`cargo test`)
6. **Format** your code (`cargo fmt`)
7. **Lint** your code (`cargo clippy -- -D warnings`)
8. **Commit** your changes (`git commit -m 'Add amazing feature'`)
9. **Push** to your branch (`git push origin feature/amazing-feature`)
10. **Open** a Pull Request

## Code Style Guidelines

### Rust Standards

CodeGuardian follows Rust's official style guidelines with additional project-specific conventions:

```rust
// ✅ Good: Clear naming and documentation
/// Validates the configuration file and returns detailed errors.
pub async fn validate_config(config_path: &Path) -> Result<ValidationResult, ConfigError> {
    // Implementation
}

// ❌ Bad: Unclear naming and missing documentation
pub fn check(path: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    // Implementation
}
```

### Naming Conventions

- **Functions**: `snake_case` (e.g., `analyze_file`, `validate_config`)
- **Variables**: `snake_case` (e.g., `file_path`, `config_data`)
- **Types/Structs**: `PascalCase` (e.g., `AnalysisResult`, `ConfigError`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_FILE_SIZE`)
- **Modules**: `snake_case` (e.g., `security_analyzer`, `performance_check`)

### Documentation

- **Public APIs** must have comprehensive doc comments
- **Complex functions** should explain their purpose and algorithm
- **Error conditions** should be documented
- **Examples** should be provided for complex APIs

```rust
/// Analyzes a file for security vulnerabilities using multiple analyzers.
///
/// This function runs all enabled security analyzers on the provided file content
/// and returns a list of findings sorted by severity.
///
/// # Arguments
///
/// * `file_path` - Path to the file being analyzed
/// * `content` - File content as a string
/// * `config` - Analysis configuration
///
/// # Returns
///
/// Returns a `Result` containing a vector of `Finding` objects or an error.
///
/// # Examples
///
/// ```rust
/// use codeguardian::analyze_file;
///
/// let findings = analyze_file(
///     Path::new("src/main.rs"),
///     "fn main() { println!(\"Hello\"); }",
///     &config
/// ).await?;
/// ```
pub async fn analyze_file(
    file_path: &Path,
    content: &str,
    config: &Config,
) -> Result<Vec<Finding>, AnalysisError> {
    // Implementation
}
```

### Error Handling

Use `anyhow::Result<T>` for application errors and `thiserror::Error` for library errors:

```rust
// Application code
pub async fn process_file(file_path: &Path) -> Result<AnalysisResult> {
    let content = tokio::fs::read_to_string(file_path)
        .await
        .context("Failed to read file")?;

    // Process content
    Ok(result)
}

// Library errors
#[derive(thiserror::Error, Debug)]
pub enum AnalysisError {
    #[error("I/O error: {message}")]
    Io { message: String, #[source] source: std::io::Error },

    #[error("Security violation: {message}")]
    Security { message: String, severity: SecuritySeverity },
}
```

### Async/Await Patterns

- Use `tokio` for async runtime
- Prefer `async fn` for async functions
- Use `Stream` for processing multiple items
- Handle cancellation with `select!`

```rust
pub async fn analyze_files_parallel(
    files: Vec<PathBuf>,
    config: &Config,
) -> Result<Vec<AnalysisResult>> {
    let mut tasks = vec![];

    for file in files {
        let config = config.clone();
        let task = tokio::spawn(async move {
            analyze_file(&file, &config).await
        });
        tasks.push(task);
    }

    let results = futures::future::join_all(tasks).await;
    // Process results
    Ok(results)
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

        // Write test content
        std::fs::write(&file_path, "fn main() { println!(\"test\"); }").unwrap();

        // Test implementation
        let result = analyze_file(&file_path, &config).await;
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

## Areas for Contribution

### 🔍 New Analyzers

Add new security, performance, or code quality checks:

```rust
// Example: New security analyzer
pub struct CustomSecurityAnalyzer;

#[async_trait]
impl Analyzer for CustomSecurityAnalyzer {
    async fn analyze_file(
        &self,
        file_path: &Path,
        content: &str,
        config: &Config,
    ) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Your analysis logic here
        if content.contains("unsafe_pattern") {
            findings.push(Finding {
                id: "custom_security_check".to_string(),
                file_path: file_path.to_string_lossy().to_string(),
                line_number: 1,
                severity: Severity::High,
                title: "Custom security issue detected".to_string(),
                description: "Found unsafe pattern in code".to_string(),
                // ... other fields
            });
        }

        Ok(findings)
    }

    fn name(&self) -> &str {
        "custom_security_analyzer"
    }
}
```

### 🌐 Platform Support

- **CI/CD platform integrations** (Jenkins, CircleCI, Azure DevOps)
- **Cloud provider support** (AWS, GCP, Azure)
- **Container registry integrations** (Docker Hub, ECR, GCR)

### 📊 Output Formats

- **New report formats** (PDF, XML, custom JSON schemas)
- **Integration formats** (JUnit XML, Checkstyle XML)
- **Visualization** (charts, graphs, interactive dashboards)

### 🧠 ML Improvements

- **Enhanced false positive detection** algorithms
- **New feature extraction** methods
- **Model training** improvements
- **Online learning** enhancements

### 📚 Documentation

- **Improve guides** and tutorials
- **Add examples** for complex features
- **Translate documentation** to other languages
- **Create video tutorials** and walkthroughs

### 🐛 Bug Fixes

- **Fix security vulnerabilities** in the codebase
- **Improve error handling** and edge cases
- **Optimize performance** bottlenecks
- **Enhance stability** and reliability

### 🛡️ Security

- **Vulnerability research** and pattern discovery
- **Security enhancement** implementations
- **Cryptographic improvements**
- **Secure coding practices**

### ⚡ Performance

- **Algorithm optimization** for faster analysis
- **Memory usage** improvements
- **Parallel processing** enhancements
- **Caching strategy** improvements

### 🔧 DevOps

- **CI/CD improvements** and automation
- **Deployment automation** for different platforms
- **Monitoring and alerting** integration
- **Container optimization**

## Pull Request Process

### 1. Pre-Submission Checklist

- [ ] **Tests pass**: `cargo test`
- [ ] **Code formatted**: `cargo fmt`
- [ ] **Linting passes**: `cargo clippy -- -D warnings`
- [ ] **Documentation updated**: Update docs for new features
- [ ] **Changelog updated**: Add entry to CHANGELOG.md
- [ ] **Security reviewed**: Check for security implications

### 2. PR Description Template

```markdown
## Description

[Brief description of the changes]

## Changes

- [List of changes made]
- [Breaking changes, if any]
- [New features added]

## Testing

- [How the changes were tested]
- [Test cases added]
- [Performance impact]

## Related Issues

- Closes #[issue_number]
- Related to #[issue_number]

## Checklist

- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Code formatted with `cargo fmt`
- [ ] Linting passes with `cargo clippy`
- [ ] Security implications reviewed
```

### 3. Code Review Process

1. **Automated checks** run on PR
2. **Maintainer review** for code quality and architecture
3. **Security review** for security-sensitive changes
4. **Testing review** for test coverage and correctness
5. **Documentation review** for completeness

### 4. Merging

- **Squash and merge** for clean history
- **Delete branch** after merge
- **Update milestones** and project boards

## Development Environment

### Recommended Tools

- **VS Code** with Rust Analyzer extension
- **CLion** with Rust plugin
- **IntelliJ IDEA** with Rust plugin
- **Vim/Neovim** with rust.vim plugin

### VS Code Configuration

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.allTargets": false,
  "rust-analyzer.cargo.allFeatures": true,
  "rust-analyzer.procMacro.enable": true,
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll": true,
    "source.organizeImports": true
  }
}
```

### Debugging

```rust
// Add debug logging
use tracing::{info, warn, error};

info!("Starting analysis of file: {}", file_path.display());
warn!("Potential issue found: {}", issue_description);
error!("Analysis failed: {}", error);
```

### Performance Profiling

```bash
# Profile with cargo flamegraph
cargo install flamegraph
cargo flamegraph --bin codeguardian

# Profile with perf
perf record target/release/codeguardian check .
perf report
```

## Community Guidelines

### Communication

- **Be respectful** and inclusive
- **Use clear language** and provide context
- **Help others** when possible
- **Ask for help** when needed

### Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

### Recognition

Contributors are recognized in:
- **CHANGELOG.md** for significant contributions
- **GitHub Contributors** list
- **Release notes** for major features
- **Documentation** acknowledgments

## Getting Help

### Resources

- 📖 **[Documentation](docs/)** - Comprehensive user guides
- ❓ **[FAQ](docs/troubleshooting/faq.md)** - Frequently asked questions
- 🐛 **[Issue Tracker](https://github.com/d-oit/codeguardian/issues)** - Bug reports and feature requests
- 💬 **[Discussions](https://github.com/d-oit/codeguardian/discussions)** - Community discussions

### Asking Questions

When asking for help:
- **Provide context** about what you're trying to do
- **Include error messages** and stack traces
- **Share your configuration** and environment
- **Describe what you've tried** already

### Reporting Bugs

When reporting bugs:
- **Use the bug report template**
- **Include reproduction steps**
- **Provide system information**
- **Attach relevant files** (logs, configs)

### Feature Requests

When requesting features:
- **Explain the use case** and problem being solved
- **Provide examples** of how it would be used
- **Consider implementation complexity**
- **Check for existing solutions**

## License

By contributing to CodeGuardian, you agree that your contributions will be licensed under the same license as the project (MIT License).

## Acknowledgments

Thank you for contributing to CodeGuardian! Your efforts help make code analysis better for everyone in the developer community.