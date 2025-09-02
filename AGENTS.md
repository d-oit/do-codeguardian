# AGENTS.md

## Project Overview
CodeGuardian is a Rust-based security analysis platform with integrated machine learning capabilities for automated code analysis, vulnerability detection, and performance optimization. It combines traditional static analysis with ML-powered false positive reduction using RUV-FANN neural networks.

## Setup Commands
- **Install Rust toolchain**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Install dependencies**: `cargo build`
- **Run development server**: `cargo run -- --help`
- **Run tests**: `cargo test`
- **Format code**: `cargo fmt`
- **Lint code**: `cargo clippy -- -D warnings`

## Code Style
- **Rust 2021 Edition** with strict compiler settings
- **Naming**: snake_case for functions/variables, PascalCase for types/structs, SCREAMING_SNAKE_CASE for constants
- **Error Handling**: Use `anyhow::Result<T>` for application errors, `thiserror::Error` for library error types
- **Security-First**: Always validate inputs, use safe defaults, prevent resource exhaustion
- **Memory Safety**: Leverage Rust's ownership system, avoid unsafe code unless absolutely necessary
- **Performance**: Use async/await patterns, parallel processing where appropriate, memory pooling
- **Code Size**: Keep functions under 50-100 lines, files under 300-700 lines
- **Single Responsibility Principle**: Each file should have one clear purpose
- **Bounded Contexts**: Divide large systems into smaller, focused contexts
- **Extract Class/Method**: Use refactoring techniques to break down large files
- **Module splitting**: Split large modules into focused, cohesive units

### Duplicate Code Avoidance
- **Extract Method**: Move duplicated code fragments into reusable methods
- **Extract Class**: Create new classes for shared functionality
- **Extract Superclass**: Use inheritance for common behavior
- **Template Method pattern**: Define algorithm skeletons in base classes
- **Trait-based extraction**: Use Rust traits for code reuse across agent types
- **Component composition**: Build reusable React components to avoid duplication

## Testing Instructions
- **Unit Tests**: Run `cargo test` for all tests, `cargo test <test_name>` for specific tests
- **Integration Tests**: Use `cargo test --test <integration_test>` for end-to-end testing
- **Performance Benchmarks**: Run `cargo bench` or use `./scripts/performance_analysis.sh`
- **Security Testing**: Focus on input validation, path traversal, and resource limits
- **ML Testing**: Validate model accuracy, false positive rates, and training data integrity
- **CI Testing**: All tests must pass before merging, including security and performance checks

## Security Considerations
- **Input Validation**: Always validate file paths, sizes, and content before processing
- **Resource Limits**: Implement file size limits (10MB default), memory bounds, and timeout handling
- **Path Security**: Use canonical paths, prevent directory traversal attacks
- **Data Handling**: Secure ML training data, avoid exposing sensitive information
- **Audit Trails**: Log all analysis operations for compliance and debugging
- **Configuration Security**: Validate configuration files, use secure defaults

## Agent Development Guidelines

### Setup and Development Environment
- **Rust Toolchain**: Use Rust 2021 Edition with latest stable compiler (1.70+)
- **Key Dependencies**: Tokio 1.40 for async, Clap 4.5 for CLI, Serde for serialization, git2 0.19 for Git integration, RUV-FANN for ML
- **Development Workflow**: Use `cargo build` for development, `cargo build --release` for production
- **ML Integration**: RUV-FANN neural networks for false positive reduction, online learning capabilities
- **Security Features**: BLAKE3 hashing for integrity, configuration drift detection, enhanced secret scanning

### Code Style and Conventions
- **Rust 2021 Patterns**: Use modern async/await, pattern matching, and ownership semantics
- **Security-First Coding**: Validate all inputs, implement resource limits, use safe memory practices
- **Memory Safety**: Leverage Rust's borrow checker, avoid unsafe blocks, use smart pointers appropriately
- **Error Handling**: Comprehensive error handling with `anyhow` for application code, `thiserror` for libraries
- **Performance Guidelines**: Optimize for CI/CD environments (<30s analysis time), target <100MB memory usage
- **Code Organization**: Modular structure with clear separation between analyzers, ML components, and utilities

### Testing and Quality Assurance
- **Unit Testing**: Test individual analyzers (src/analyzers/) with mock data and edge cases
- **Integration Testing**: Test ML pipeline (src/ml/) with real training data and validation
- **Performance Benchmarking**: Use benches/ directory for comprehensive performance testing
- **Security Testing**: Validate input sanitization, path security, and resource exhaustion prevention
- **Quality Assurance**: Run clippy lints, format checks, and comprehensive test suites before commits

### CI/CD and Deployment
- **GitHub Actions**: Use workflows in .github/workflows/ for automated testing and releases
- **Release Management**: Automated versioning with cargo-release, security scanning in CI
- **Performance Monitoring**: Track analysis speed, memory usage, and accuracy metrics in CI
- **Deployment**: Cross-platform builds for Linux, macOS, Windows with optimized binaries

### Security and Compliance
- **Secure Coding**: Input validation, path canonicalization, resource limits, audit logging
- **ML Data Handling**: Secure training data management, model validation, privacy protection
- **Audit Requirements**: Comprehensive logging, deterministic results, compliance reporting
- **Vulnerability Management**: Regular dependency updates, security scanning, patch management

### Performance Optimization
- **Memory Usage**: Target <100MB peak usage, use memory pools in src/performance/memory_pool.rs
- **Parallel Processing**: Leverage src/core/parallel_file_processor.rs for concurrent analysis
- **Caching Strategies**: Intelligent caching in src/cache/ with result persistence
- **Streaming Analysis**: Use src/streaming.rs for large file processing without full memory load

## Available Agents

### Core Development Agents
- **github-discussions-manager**: Manages GitHub Discussions, including creation, moderation, and community engagement using GitHub CLI
- **github-issue-manager**: Handles GitHub Issues management, creation, updates, and organization
- **github-pr-manager**: Manages GitHub Pull Requests, reviews, and merge processes
- **github-label-manager**: Manages GitHub labels for categorization and workflow automation
- **github-projects-manager**: Handles GitHub Projects for roadmap and task management
- **github-workflow-manager**: Enhanced GitHub Workflow Manager - manages and optimizes GitHub Actions workflows and CI/CD pipelines

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
- **configuration-agent**: Manages configuration files and optimization
- **configuration-validator**: Validates codeguardian.toml and configuration integrity
- **debug-findings-analyst**: Analyzes systematic investigation findings for debugging

### Infrastructure & CI/CD Agents
- **analyzer-orchestrator**: Coordinates src/analyzers/ modules for comprehensive code analysis
- **build-ci-optimizer**: Optimizes build processes and CI/CD pipelines
- **benchmark-agent**: Manages performance benchmarks and analysis
- **ml-pipeline-manager**: Manages src/ml/ workflow and ML pipelines
- **ml-training-specialist**: Handles ML training and model optimization
- **cache-intelligence-agent**: Manages src/cache.rs for intelligent caching
- **streaming-processor**: Optimizes src/streaming.rs for performance
- **documentation-specialist**: Enhanced Documentation Specialist - generates and maintains comprehensive documentation
- **github-docs-specialist**: Maintains GitHub repository documentation

### Utility Agents
- **general**: General-purpose agent for research and multi-step tasks
- **orchestrator**: Enhanced Orchestrator - provides analysis and recommendations for coordinating complex multi-agent workflows
- **swarm-orchestrator**: Dynamic coordinator for managing agent swarms and parallel processing strategies
- **analyzer-orchestrator**: Coordinates multiple CodeGuardian analyzers for comprehensive code analysis
- **ai-persona-creation-specialist**: Creates specialized AI personas (manual activation only)

## Agent Selection Framework

### Decision Matrix for Agent Selection

| Task Type | Complexity | Recommended Agent(s) | Rationale |
|-----------|------------|----------------------|-----------|
| Code Review | Simple | code-quality-reviewer | Single agent sufficient for basic quality checks |
| Code Review | Complex | code-quality-reviewer + security-auditor | Multi-agent for comprehensive analysis |
| Security Audit | Critical | security-auditor + validation-agent | Cross-verification for high-stakes findings |
| Performance Optimization | Simple | performance-optimizer | Specialized domain expertise |
| Performance Optimization | Complex | performance-optimizer + benchmark-agent | Data-driven optimization with metrics |
| Testing | Simple | testing-engineer | Standard test generation and execution |
| Testing | Complex | testing-engineer + code-research | Research-driven test coverage |
| Documentation | Simple | documentation-specialist | Focused documentation tasks |
| Documentation | Complex | documentation-specialist + codebase-doc-updater | Comprehensive documentation maintenance |

### Selection Criteria
- **Task Complexity**: Simple (single file/component), Complex (multiple components), Critical (security/safety impact)
- **Domain Expertise**: Match agent specialization to task requirements
- **Resource Constraints**: Consider computational resources and time budgets
- **Risk Level**: Higher risk tasks require multiple agents for validation
- **Agent Hierarchy**: Prefer specialized agents for domain-specific tasks; use the general agent only for truly cross-domain research or when no specialist exists

### Agent Selection Guidance
The orchestrator should always prioritize specialized agents over the general agent for domain-specific tasks. Specialized agents provide deeper expertise, better accuracy, and more efficient processing for their designated domains. Only recommend the general agent for truly cross-cutting research tasks that span multiple domains without fitting into any specialist category, or when no appropriate specialist exists.

This ensures optimal performance and quality by leveraging the right tool for each job.

## Repository Information
Repository information is available in `.opencode/agent-config.json` or can be obtained dynamically using helper scripts in `.opencode/get-repo-info.sh`.

Remember: This is a security-focused codebase. Always prioritize secure defaults, validate inputs, and handle errors gracefully.
