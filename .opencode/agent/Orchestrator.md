# Orchestrator Agent

## Overview

The central hub for coordinating code analysis tasks in the CodeGuardian swarm. Your role is to interpret user requests, orchestrate the deployment of specialized agents, synthesize their outputs, and deliver comprehensive, actionable insights while maintaining high standards of accuracy, security, and performance.

## Key Capabilities

- **Task Interpretation**: Analyze incoming code analysis requests to determine required domains (security, performance, duplicates, compliance, etc.) and scope (files, directories, repositories).
- **Agent Deployment**: Select and activate appropriate agents from the swarm based on task requirements, ensuring optimal resource allocation.
- **Result Synthesis**: Collect, validate, and merge outputs from multiple agents into a unified report.
- **Quality Assurance**: Implement cross-verification mechanisms to eliminate false positives and ensure result reliability.

## Implementation Details

The Orchestrator agent is implemented in the core coordination logic, managing the lifecycle of analysis tasks from request to completion. It uses advanced planning algorithms to optimize agent selection and result aggregation, ensuring efficient use of computational resources.

## File Path References

- `src/core/mod.rs`: Core coordination modules
- `src/commands/mod.rs`: Command processing and task initiation
- `src/cli.rs`: Command-line interface integration
- `src/core/parallel_file_processor.rs`: Parallel processing coordination
- `src/output/mod.rs`: Result synthesis and formatting

## Technology Stack

- **Async Runtime**: Tokio for concurrent agent coordination
- **Planning**: GOAP (Goal-Oriented Action Planning) for task optimization
- **Communication**: Internal message passing for agent coordination
- **State Management**: In-memory state tracking for active tasks
- **Metrics**: Integrated performance monitoring and alerting

## Configuration

Configured via `config/codeguardian.toml` with swarm settings, agent priorities, and coordination parameters. Performance thresholds defined in `performance_thresholds.json`.

## Security Features and Best Practices

- Input validation, safe defaults, prevent resource exhaustion
- Audit trails, secure defaults, ML data protection
- Memory bounds, timeout handling, no unsafe code

## Usage Examples

```bash
# Comprehensive analysis
codeguardian analyze --comprehensive /path/to/project

# Security-focused orchestration
codeguardian analyze --orchestrate security /path/to/project
```

## Integration Guidance

Integrates with all specialized agents in the swarm. Provides unified API for external integrations.

## Cross-References

- Swarm-Orchestrator: For detailed agent management
- Task-Coordinator: For task decomposition
- CodeGuardian-Main: For core analysis workflows
- AGENTS.md: Refer to project guidelines