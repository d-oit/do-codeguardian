---
name: comprehensive-analysis
description: Comprehensive codebase analysis with swarm intelligence for output systems, results organization, and optimization planning
version: 1.0.0
author: CodeGuardian Agent
tags: [analysis, swarm, optimization, planning]
---

# comprehensive-analysis

## Synopsis

```bash
opencode comprehensive-analysis [OPTIONS]
```

Perform comprehensive codebase analysis using swarm intelligence to evaluate output systems, results organization, and generate optimization plans.

## Description

The `comprehensive-analysis` command orchestrates a multi-agent swarm analysis of the codebase, focusing on:

- **Output System Analysis**: Evaluate current output formats, report generation, and data presentation
- **Results Organization**: Review folder structures, file organization, and data management
- **Performance Optimization**: Identify bottlenecks and optimization opportunities
- **Security Assessment**: Analyze security implications of current implementations
- **Best Practice Recommendations**: Generate actionable improvement suggestions
- **Implementation Planning**: Create detailed roadmaps for enhancements

The command utilizes 6 specialized agents working in coordination:
1. **Output Analyst**: Evaluates output formats and presentation
2. **Results Organizer**: Analyzes data organization and structure
3. **Performance Optimizer**: Identifies optimization opportunities
4. **Security Auditor**: Reviews security implications
5. **Best Practices Specialist**: Generates improvement recommendations
6. **Implementation Planner**: Creates detailed execution plans

## Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--scope` | `-s` | Analysis scope (output, results, performance, security, all) | all |
| `--swarm-size` | `-w` | Number of agents in swarm (1-10) | 6 |
| `--output-format` | `-f` | Output format for plans (markdown, json, yaml) | markdown |
| `--target-dir` | `-t` | Target directory for generated plan files | ./plans |
| `--verbose` | `-v` | Enable verbose mode for detailed agent coordination | false |
| `--report-only` | `-r` | Generate reports without creating plan files | false |
| `--integrate` | `-i` | Integrate with existing CodeGuardian commands | false |
| `--help` | `-h` | Display help information | - |
| `--version` | `-V` | Display version information | - |

## Examples

### Basic Comprehensive Analysis

```bash
# Run full comprehensive analysis with default settings
opencode comprehensive-analysis
```

This performs a complete analysis using all 6 agents, generating reports and plan files in the default `./plans` directory.

### Focused Analysis on Output Systems

```bash
# Analyze only output systems with detailed coordination
opencode comprehensive-analysis --scope output --verbose
```

Focuses the swarm intelligence on output formats, report generation, and presentation systems with detailed logging of agent interactions.

### Performance Optimization Analysis

```bash
# Analyze performance bottlenecks with custom swarm size
opencode comprehensive-analysis --scope performance --swarm-size 4 --output-format json
```

Uses 4 agents to focus on performance optimization, generating plans in JSON format for programmatic processing.

### Security-Focused Analysis

```bash
# Comprehensive security assessment
opencode comprehensive-analysis --scope security --target-dir ./security-plans
```

Directs the swarm to focus on security implications, generating security improvement plans in a dedicated directory.

### Integration with CodeGuardian

```bash
# Integrate analysis with existing CodeGuardian workflows
opencode comprehensive-analysis --integrate --scope all --verbose
```

Runs comprehensive analysis and automatically integrates findings with existing CodeGuardian commands and workflows.

### CI/CD Usage

```bash
# Generate optimization plans for CI/CD pipeline
opencode comprehensive-analysis --output-format yaml --target-dir ./ci-plans --report-only
```

Creates YAML-formatted plans suitable for CI/CD pipeline integration without modifying the codebase.

### Custom Swarm Configuration

```bash
# Large-scale analysis with maximum swarm size
opencode comprehensive-analysis --swarm-size 10 --scope all --verbose --target-dir ./comprehensive-plans
```

Utilizes all available agents for maximum analysis depth, generating comprehensive plans in a dedicated directory.

## Swarm Intelligence Workflow

The command implements a sophisticated swarm intelligence approach:

1. **Agent Initialization**: Spawns specialized agents based on analysis scope
2. **Parallel Analysis**: Agents work simultaneously on different aspects
3. **Cross-Agent Communication**: Agents share findings and coordinate efforts
4. **Consensus Building**: Agents reach agreement on recommendations
5. **Plan Generation**: Coordinated creation of implementation plans
6. **Quality Assurance**: Internal validation of generated plans

## Output Files

The command generates the following files in the target directory:

- `analysis-report.md`: Comprehensive analysis summary
- `optimization-plan.md`: Detailed implementation roadmap
- `best-practices.md`: Recommended improvements
- `security-assessment.md`: Security analysis results
- `performance-metrics.md`: Performance optimization findings
- `agent-coordination.log`: Detailed agent interaction log (verbose mode)

## Error Handling and Recovery

The command includes robust error handling:

- **Agent Failure Recovery**: Automatically respawns failed agents
- **Partial Analysis Completion**: Continues analysis even if some agents fail
- **Data Validation**: Validates all generated plans and reports
- **Rollback Capability**: Can revert changes if integration fails
- **Timeout Protection**: Prevents runaway analysis processes

## Security Considerations

- **Input Validation**: All user inputs are validated and sanitized
- **Access Control**: Respects file system permissions and access controls
- **Data Protection**: Generated plans contain no sensitive information
- **Audit Trail**: All agent actions are logged for security review
- **Safe Defaults**: Conservative settings prevent resource exhaustion

## Best Practices

### Swarm Configuration
- Use default swarm size (6) for balanced analysis depth and performance
- Increase swarm size only for complex, large-scale codebases
- Monitor resource usage when using maximum swarm size

### Analysis Scope
- Start with focused scopes for targeted improvements
- Use `--scope all` for comprehensive assessments
- Combine scopes for multi-faceted analysis

### Output Management
- Use appropriate output formats for your workflow (markdown for documentation, JSON/YAML for automation)
- Organize plan files in dedicated directories
- Review generated plans before implementation

### Integration
- Test integration with existing workflows in staging environments
- Use `--report-only` for initial assessments
- Enable verbose mode during initial integrations

## Performance Considerations

- **Resource Usage**: Swarm size directly impacts CPU and memory usage
- **Analysis Time**: Larger scopes and swarm sizes increase processing time
- **File I/O**: Plan generation involves significant file operations
- **Network**: May require network access for external validations

## Troubleshooting

### Common Issues

**Analysis Timeout**
```
Error: Analysis exceeded timeout limit
```
- Reduce swarm size or analysis scope
- Increase timeout settings if necessary
- Check system resources

**Permission Denied**
```
Error: Cannot write to target directory
```
- Verify write permissions on target directory
- Use `--target-dir` with appropriate permissions
- Check file system quotas

**Agent Coordination Failure**
```
Error: Swarm coordination failed
```
- Reduce swarm size to improve stability
- Enable verbose mode for debugging
- Check system resources and network connectivity

**Invalid Scope**
```
Error: Invalid analysis scope specified
```
- Use valid scope values: output, results, performance, security, all
- Check command syntax and option spelling

## Exit Codes

- `0`: Success
- `1`: General error
- `2`: Invalid arguments
- `3`: Permission denied
- `4`: Timeout exceeded
- `5`: Swarm coordination failure

## See Also

- `opencode analyze-output`: Focused output system analysis
- `opencode optimize-performance`: Performance optimization tools
- `opencode security-audit`: Security assessment commands
- `opencode generate-plans`: Plan generation utilities
- `codeguardian check`: CodeGuardian analysis integration
- `codeguardian report`: Report generation commands

## Version History

- **1.0.0**: Initial release with full swarm intelligence capabilities
- Comprehensive multi-agent analysis framework
- Support for all major analysis scopes
- Integration with CodeGuardian ecosystem
- Robust error handling and recovery mechanisms
