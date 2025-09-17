# update-docs

## Synopsis
Update and maintain CodeGuardian documentation files, generating missing documentation and validating existing content.

## Description
The update-docs command maintains CodeGuardian's documentation by updating existing files, generating missing documentation, and ensuring consistency across all documentation sources.

## Syntax
```bash
codeguardian update-docs [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--force` | Force update all documentation files | `FLAG` | `false` | No |
| `--validate-only` | Only validate documentation without updating | `FLAG` | `false` | No |
| `--api` | Generate API documentation | `FLAG` | `false` | No |
| `--user-guide` | Generate user guide documentation | `FLAG` | `false` | No |
| `--config` | Generate configuration documentation | `FLAG` | `false` | No |

## Examples
```bash
# Update all documentation
codeguardian update-docs --force

# Validate documentation without changes
codeguardian update-docs --validate-only

# Update specific documentation types
codeguardian update-docs --api --user-guide

# Update configuration documentation only
codeguardian update-docs --config
```

## Documentation Types
- **API Documentation**: Technical API references and examples
- **User Guide**: Command documentation and usage examples
- **Configuration**: Configuration options and templates

## See Also
- [Configuration Guide](../configuration.md) - Configuration documentation
- [API Documentation](../api/index.md) - API reference
