# CodeGuardian Agent Guide

## Repository Information

Repository information is available in `.opencode/agent-config.json` or can be obtained dynamically:

```bash
# Get repository URL from git remote
git remote get-url origin

# Or use the helper script
./.opencode/get-repo-info.sh github             # Returns GitHub URL
./.opencode/get-repo-info.sh issues             # Returns issues URL
./.opencode/get-repo-info.sh docs               # Returns documentation URL
./.opencode/get-repo-info.sh ci-badge           # Returns CI badge URL
./.opencode/get-repo-info.sh codecov-badge      # Returns codecov badge URL
./.opencode/get-repo-info.sh downloads-badge    # Returns downloads badge URL
./.opencode/get-repo-info.sh contributors-badge # Returns contributors badge URL
./.opencode/get-repo-info.sh last-commit-badge  # Returns last commit badge URL
./.opencode/get-repo-info.sh actions            # Returns GitHub Actions URL
```

## Build, Lint, and Test Commands

### Primary Commands
- **Build**: `cargo build --release`
- **Build debug**: `cargo build`
- **Run all tests**: `cargo test`
- **Run specific test**: `cargo test <test_name>`
- **Format code**: `cargo fmt`
- **Lint with clippy**: `cargo clippy -- -D warnings`

### Single Test Execution
```bash
cargo test test_function_name  # Run specific test function
cargo test --test <test_file>  # Run tests in specific file
```

## Code Style Guidelines

### General Conventions
- **Edition**: Rust 2021 Edition
- **Naming**: snake_case for functions/variables, PascalCase for types/structs, SCREAMING_SNAKE_CASE for constants
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
```

### Error Handling Patterns
```rust
// Use anyhow::Result for application code
pub async fn analyze_files(&self, files: &[PathBuf]) -> Result<AnalysisResults> {
    // Implementation
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
        assert!(result.is_ok());
    }
}
```

Remember: This is a security-focused codebase. Always prioritize secure defaults, validate inputs, and handle errors gracefully.