# CodeGuardian CLI Commands and Structure Analysis

**Date:** Thu Sep 25 2025  
**Version:** 0.2.1-alpha.2  
**Commit:** N/A  

## Overview

CodeGuardian is a comprehensive security-first code analysis tool built in Rust, featuring advanced static analysis, ML-powered false positive reduction, and extensive GitHub integration. The CLI provides a modular command structure with feature-gated advanced capabilities.

## Entry Points

### Main Binary
- **Name:** `do-codeguardian` (from package name)
- **Location:** `src/main.rs`
- **Features:** Supports multiple Cargo features for optional functionality

### Global Options
- `--config` (default: `codeguardian.toml`): Configuration file path
- `--verbose` (`-v`): Verbose output (multiple levels)
- `--quiet` (`-q`): Suppress non-error output
- `--ai-enhance`: Enable AI-enhanced analysis globally

## Available CLI Commands

### Core Commands (Always Available)
1. **check** - Primary code analysis command
2. **report** - Convert analysis results to different formats
3. **gh-issue** - Create/update GitHub issues from results
4. **init** - Initialize configuration files
5. **git-commit** - Enhanced git commit with security analysis
6. **git-commit-push** - Enhanced git commit and push
7. **turbo** - High-performance parallel analysis
8. **update-docs** - Update and maintain documentation
9. **remediation** - Automated remediation workflows
10. **integrations** - External system integrations
11. **bulk** - Bulk operations for multiple repositories
12. **retention** - Retention policy management
13. **tune-thresholds** - Tune monitoring thresholds
14. **ml-enhancements** - Advanced ML enhancements showcase

### Feature-Gated Commands
15. **train** (requires `ml` feature) - Train ML models
16. **training-data** (requires `ml` feature) - Collect training data
17. **metrics** (requires `ml` feature) - Analyze ML model performance
18. **dashboard** (requires `dashboard` feature) - Dashboard management
19. **release-monitoring** (requires `release-monitoring` feature) - Release monitoring
20. **feature-engineering** (requires `ml` feature) - Advanced feature engineering

## Key Components

### CLI Structure (`src/cli.rs`)
- Uses `clap` for argument parsing
- Modular subcommand structure
- Comprehensive argument validation
- Feature-gated command availability

### Core Engine (`src/core/guardian_engine.rs`)
- `GuardianEngine` struct as main analysis coordinator
- Integrates analyzers, caching, and AI processing
- Handles file discovery and parallel processing

### Command Handlers
- **CLI Commands:** Located in `src/cli/` directory
- **Special Commands:** Located in `src/commands/` directory
- Each command has dedicated module with argument parsing and execution logic

### Analyzer Registry (`src/analyzers/`)
- Plugin-based analyzer system
- Multiple analyzer types: security, performance, quality, etc.
- Configurable analyzer selection

## Data Flow

### Analysis Pipeline
1. **Input:** File paths, configuration, optional ML models
2. **Discovery:** File system traversal with security checks
3. **Analysis:** Parallel execution through analyzer registry
4. **Filtering:** ML-based false positive reduction (optional)
5. **Output:** JSON results with optional format conversions

### Storage Organization
- **Hierarchical Storage:** `analysis-results/{command}/{date}/` structure
- **Compression:** Optional result compression
- **Retention:** Configurable cleanup policies

## Configuration

### Configuration File
- **Default:** `codeguardian.toml`
- **Features:** Environment variable support, validation
- **Sections:** Analysis settings, ML configuration, output preferences

### Cargo Features
- **Default:** `git`, `security`, `logging`
- **Optional:** `ml`, `dashboard`, `release-monitoring`, `ast`, etc.
- **Impact:** Feature flags control command availability and functionality

## Error Handling

### Exception Patterns
- Uses `anyhow::Result<T>` for error propagation
- Logging integration with `tracing`
- Graceful degradation for optional features

### Recovery Mechanisms
- Configuration fallback to defaults
- Cache corruption recovery
- Partial analysis continuation on failures

## Performance Characteristics

### Parallel Processing
- Adaptive parallelism based on system resources
- Memory pooling for large-scale analysis
- Streaming analysis for big files

### Caching Strategy
- File-based caching with invalidation
- Performance monitoring and metrics
- Intelligent cache reuse

## Security Considerations

### Input Validation
- Path canonicalization and security checks
- File size limits (10MB default)
- Safe file system traversal

### Access Controls
- No arbitrary code execution
- Sandboxed analysis environment
- Secure temporary file handling

## Recommendations

### For Users
1. Start with `check` command for basic analysis
2. Enable `ml` features for advanced false positive reduction
3. Use `turbo` mode for large codebases
4. Configure hierarchical storage for result organization

### For Developers
1. Commands are well-modularized in separate files
2. Feature gating allows flexible builds
3. Extensive argument validation prevents misuse
4. Comprehensive error handling improves reliability

### Documentation Gaps
- Some advanced commands lack user guide documentation
- Feature requirements not clearly documented in help text
- Integration examples could be expanded

## Discrepancies Identified

### Between Code and Documentation
- **Missing in User Guide:** bulk, remediation, tune-thresholds, ml-enhancements
- **Feature-Gated Commands:** Not visible in default help output
- **Command Descriptions:** Some variations between code comments and help text

### Between Implementation and Usage
- All coded commands are functional
- Feature gating works as intended
- No broken or incomplete command implementations found

This analysis provides a complete foundation for understanding CodeGuardian's CLI architecture and command ecosystem.
