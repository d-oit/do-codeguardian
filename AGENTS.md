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
- **orchestrator**: Enhanced Orchestrator - coordinates complex multi-agent workflows with task coordination and swarm management
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

## Agent Selection Decision Framework

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

### Single Agent vs. Swarm Usage

- **Single Agent**: Use for focused, well-defined tasks with clear success criteria
- **Agent Swarm**: Use for complex, multi-faceted tasks requiring diverse expertise or cross-validation

## Swarm Patterns and Coordination

### Coordination Patterns

#### Parallel Execution
- Agents work simultaneously on independent subtasks
- Best for: Large-scale analysis, multiple file reviews, parallel testing
- Example: Multiple code-quality-reviewer agents analyzing different modules

#### Sequential Execution
- Agents process tasks in a defined order with data dependencies
- Best for: Pipeline workflows, dependency analysis, incremental improvements
- Example: code-research → security-auditor → performance-optimizer

#### Competitive Execution
- Multiple agents compete to solve the same problem
- Best for: Optimization problems, alternative solution generation
- Example: Multiple performance-optimizer agents proposing different optimization strategies

### Optimal Agent Combinations

| Scenario | Agent Combination | Coordination Pattern |
|----------|-------------------|---------------------|
| Security Code Review | security-auditor + code-quality-reviewer | Parallel |
| Performance Audit | performance-optimizer + benchmark-agent | Sequential |
| Release Preparation | release-agent + testing-engineer + documentation-specialist | Sequential |
| Bug Investigation | debug-findings-analyst + code-research | Parallel |

### Resource Allocation Guidelines

- **Load Balancing**: Distribute tasks based on agent capacity and current workload
- **Priority Queuing**: Critical tasks get higher priority in agent scheduling
- **Resource Monitoring**: Track agent resource usage and adjust allocation dynamically
- **Failover Planning**: Design redundant agents for critical operations

## False Positive Prevention Strategies

### Validation Agent Usage

- **Pre-Analysis Validation**: Use validation-agent to verify input data integrity
- **Cross-Verification Requirements**: For critical findings, require confirmation from multiple agents
- **Confidence Scoring**: Implement scoring system for findings reliability

### Cross-Verification Protocols

| Finding Type | Required Verification | Minimum Confidence Threshold |
|--------------|----------------------|------------------------------|
| Security Vulnerability | 2+ agents | 85% |
| Performance Issue | 1+ benchmark validation | 75% |
| Code Quality Issue | Peer review agent | 70% |
| Configuration Error | 2+ configuration agents | 80% |

### Confidence Scoring Guidelines

- **High Confidence (85-100%)**: Multiple corroborating sources, extensive testing
- **Medium Confidence (70-84%)**: Single strong indicator with supporting evidence
- **Low Confidence (<70%)**: Requires manual review and additional validation

## Parallel Processing Guidelines

### Best Practices for Concurrent Execution

- **Dependency Management**: Map task dependencies before parallel execution
- **Conflict Resolution**: Implement locking mechanisms for shared resources
- **Resource Pooling**: Use connection pools and resource limits to prevent exhaustion
- **Timeout Handling**: Set appropriate timeouts for long-running parallel tasks

### Performance Optimization Tips

- **Batch Processing**: Group similar tasks for efficient processing
- **Caching Strategies**: Cache intermediate results to reduce redundant computations
- **Asynchronous Patterns**: Use async/await for non-blocking operations
- **Memory Management**: Monitor and limit memory usage in parallel operations

## Performance and Optimization

### Agent Performance Metrics

- **Response Time**: Average time to complete tasks
- **Throughput**: Number of tasks processed per unit time
- **Resource Utilization**: CPU, memory, and I/O usage patterns
- **Accuracy Rate**: Percentage of correct findings vs. false positives

### Benchmarking Guidelines

- **Standard Benchmarks**: Use consistent test suites for performance comparison
- **Load Testing**: Simulate high-load scenarios to identify bottlenecks
- **Profiling**: Use profiling tools to identify performance hotspots
- **Regression Testing**: Monitor performance changes over time

### Caching Strategies

- **Result Caching**: Cache analysis results for unchanged files
- **Configuration Caching**: Cache parsed configuration data
- **Model Caching**: Cache ML models and training data
- **Dependency Caching**: Cache resolved dependencies and metadata

### Health Monitoring and Failover

- **Health Checks**: Regular monitoring of agent responsiveness and resource usage
- **Automatic Failover**: Switch to backup agents when primary agents fail
- **Circuit Breakers**: Temporarily disable failing agents to prevent cascade failures
- **Recovery Procedures**: Automated recovery and state synchronization

## Integration Patterns

### Agent Collaboration Workflows

#### Pipeline Pattern
```
Input → Agent A → Agent B → Agent C → Output
```
- Best for: Sequential processing with data transformation
- Example: Code parsing → Security analysis → Report generation

#### Fan-Out/Fan-In Pattern
```
Input → Agent A → ┌─ Agent B
                  ├─ Agent C
                  └─ Agent D → Aggregator → Output
```
- Best for: Parallel processing with result aggregation
- Example: Multi-aspect code analysis with consolidated reporting

#### Mediator Pattern
```
Agents ↔ Mediator ↔ Shared Resources
```
- Best for: Complex interactions between multiple agents
- Example: Orchestrator coordinating specialized agents

### Communication Protocols

- **Message Passing**: Use standardized message formats for inter-agent communication
- **Event-Driven**: Agents respond to events and emit completion events
- **Shared State**: Use shared data stores for coordination (with proper locking)
- **API Contracts**: Define clear interfaces for agent interactions

### Error Handling and Recovery

- **Graceful Degradation**: Continue operation with reduced functionality when agents fail
- **Retry Mechanisms**: Implement exponential backoff for transient failures
- **Compensation Actions**: Rollback changes when operations fail
- **Logging and Alerting**: Comprehensive logging for debugging and monitoring

Remember: This is a security-focused codebase. Always prioritize secure defaults, validate inputs, and handle errors gracefully.