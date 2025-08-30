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

## Available Agents

The CodeGuardian project includes specialized AI agents for various development and management tasks. These agents are located in the `.opencode/agent/` directory and follow a standardized format for integration with development workflows.

### Core Development Agents
- **github-discussions-manager**: Manages GitHub Discussions, including creation, moderation, and community engagement using GitHub CLI
- **github-issue-manager**: Handles GitHub Issues management, creation, updates, and organization
- **github-pr-manager**: Manages GitHub Pull Requests, reviews, and merge processes
- **github-label-manager**: Manages GitHub labels for categorization and workflow automation
- **github-projects-manager**: Handles GitHub Projects for roadmap and task management
- **github-workflow-manager**: Manages GitHub Actions workflows and CI/CD pipelines

### Specialized Agents
- **code-quality-reviewer**: Reviews code for quality, maintainability, and best practices
- **security-auditor**: Performs security audits and identifies vulnerabilities
- **performance-optimizer**: Optimizes CodeGuardian performance and resource efficiency
- **testing-engineer**: Manages testing, generates tests, and ensures code quality
- **dependency-agent**: Manages dependencies, security audits, and license compliance
- **release-agent**: Handles releases, versioning, and deployment automation

### Development Support Agents
- **clean-code-developer**: Ensures code adheres to clean code principles and Rust conventions
- **code-consolidator**: Consolidates and refactors code for better maintainability
- **code-research**: Researches end-to-end execution flows and complex interactions
- **codebase-doc-updater**: Maintains comprehensive documentation across the codebase
- **configuration-agent**: Manages configuration files and optimization
- **debug-findings-analyst**: Analyzes systematic investigation findings for debugging

### Infrastructure & CI/CD Agents
- **build-ci-optimizer**: Optimizes build processes and CI/CD pipelines
- **benchmark-agent**: Manages performance benchmarks and analysis
- **ml-training-specialist**: Handles ML training and model optimization
- **documentation-specialist**: Generates and maintains technical documentation
- **github-docs-specialist**: Maintains GitHub repository documentation
- **github-workflow-optimizer**: Optimizes GitHub Actions for performance and security

### Utility Agents
- **general**: General-purpose agent for research and multi-step tasks
- **orchestrator**: Coordinates complex multi-agent workflows
- **ai-persona-creation-specialist**: Creates specialized AI personas (manual activation only)

Each agent includes detailed specifications, usage examples, and integration protocols. Agents can be invoked through the Task tool with appropriate parameters for their specific domain expertise.

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