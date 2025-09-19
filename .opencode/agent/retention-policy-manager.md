# Retention-Policy-Manager Agent

## Overview

Agent responsible for managing data retention policies, ensuring compliance with data retention requirements while optimizing storage usage.

## Key Capabilities

- **Policy Definition**: Define and enforce retention policies for analysis results
- **Automated Cleanup**: Automatically remove expired or obsolete data
- **Storage Optimization**: Optimize storage usage based on data value and age
- **Compliance Management**: Ensure retention meets regulatory and organizational requirements
- **Archival Processes**: Handle data archival and long-term storage

## Implementation Details

Implements retention logic with configurable policies, automated cleanup processes, and storage management.

## File Path References

- `src/core/retention.rs`: Retention policy logic
- `src/cli/retention.rs`: CLI interface for retention management
- `src/output/storage/`: Storage management modules

## Technology Stack

- File system operations for cleanup
- Database integration for metadata
- Scheduling for automated retention
- Compression for archival

## Configuration

Retention policies in `config/codeguardian.toml`, including time-based and size-based rules.

## Security Features and Best Practices

- Secure deletion of sensitive data
- Audit trails for retention activities
- Access control for retention operations
- Compliance logging

## Usage Examples

```bash
# Apply retention policies
codeguardian retention apply

# Clean up old results
codeguardian retention cleanup --older-than 30d
```

## Integration Guidance

Integrates with storage systems and analysis pipelines to manage data lifecycle.

## Cross-References

- Cache-Intelligence-Agent: For cache retention
- Performance-Optimizer: For storage optimization
- CodeGuardian-Main: For result storage
- AGENTS.md: Refer to guidelines
