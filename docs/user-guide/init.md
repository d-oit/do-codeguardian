# init

## Synopsis
Initialize CodeGuardian configuration with default settings or predefined templates for quick setup and customization.

## Description
The init command creates a new CodeGuardian configuration file with sensible defaults. It supports different templates for various use cases and provides an interactive setup wizard for guided configuration.

## Syntax
```bash
codeguardian init [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--default` | Initialize with default configuration | `FLAG` | `false` | No |
| `--template <TEMPLATE>` | Template to use | `STRING` | - | No |

### Available Templates
- `minimal`: Basic configuration with essential exclusions
- `security`: Security-focused configuration with strict settings
- `ci`: CI/CD optimized configuration with appropriate timeouts

## Examples
```bash
# Initialize with default configuration
codeguardian init --default

# Use security template
codeguardian init --template security

# Use CI/CD template
codeguardian init --template ci

# Interactive setup (default)
codeguardian init
```

## See Also
- [Configuration Guide](../configuration.md) - Configuration options and settings
