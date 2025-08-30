---
description: Initialize CodeGuardian configuration with interactive setup and templates
---

# init

## Synopsis
CodeGuardian's configuration initialization command that creates a tailored `codeguardian.toml` configuration file through interactive prompts or predefined templates, ensuring optimal analysis settings for different project types and security requirements.

## Description
The init command serves as the first step in setting up CodeGuardian for a project, providing an intelligent configuration wizard that adapts to your project's characteristics. It supports multiple initialization modes including interactive setup, template-based configuration, and default settings.

Key capabilities include:
- **Interactive Configuration**: Guided setup with project-specific questions and recommendations
- **Template System**: Pre-built configurations for Rust, Python, JavaScript/TypeScript, and security-focused projects
- **Project Type Detection**: Automatic pattern matching for different programming languages and frameworks
- **Security-First Defaults**: Built-in security configurations with appropriate file patterns and analysis rules
- **CI/CD Optimization**: Specialized configurations for continuous integration environments

## Syntax
```bash
codeguardian init [OPTIONS]
```

## Options
| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--default` | Initialize with default configuration | `FLAG` | `false` | No |
| `--template <TEMPLATE>` | Use predefined template | `STRING` | - | No |

### Template Values
- `minimal`: Basic configuration with essential security checks
- `security`: Enhanced security-focused configuration with strict rules
- `ci`: Optimized configuration for CI/CD pipelines with performance tuning

## Examples

### Basic Usage
```bash
# Interactive configuration setup
codeguardian init

# Use default configuration
codeguardian init --default

# Use security-focused template
codeguardian init --template security

# Use CI-optimized template
codeguardian init --template ci

# Use minimal template
codeguardian init --template minimal
```

### Advanced Usage
```bash
# Initialize for a Rust project with security focus
codeguardian init --template security

# Set up CI configuration for automated analysis
codeguardian init --template ci

# Create minimal configuration for quick setup
codeguardian init --template minimal
```

## Error Handling

### Common Errors
- **Permission Denied**: Cannot write configuration file to current directory
  ```bash
  codeguardian init
  # Error: Permission denied (os error 13)
  ```

- **Invalid Template**: Specified template does not exist
  ```bash
  codeguardian init --template nonexistent
  # Error: Unknown template: nonexistent
  ```

- **Configuration Exists**: Configuration file already exists
  ```bash
  codeguardian init
  # Error: Configuration file already exists at codeguardian.toml
  ```

### Recovery Procedures
1. **Permission Issues**: Change to a writable directory or run with appropriate permissions
   ```bash
   cd ~/my-project
   codeguardian init
   ```

2. **Template Errors**: Use a valid template name or omit the template option for interactive setup
   ```bash
   codeguardian init --template minimal
   ```

3. **Existing Configuration**: Remove existing file or use a different location
   ```bash
   rm codeguardian.toml
   codeguardian init
   ```

## Security Considerations
- **Input Validation**: All user inputs are validated and sanitized during interactive setup
- **File Permissions**: Configuration files are created with appropriate permissions (644)
- **Template Security**: Predefined templates include security best practices and safe defaults
- **Path Validation**: File paths and patterns are validated to prevent directory traversal
- **Configuration Auditing**: Generated configurations can be reviewed before use
- **No Credential Storage**: Configuration files never store sensitive credentials or tokens

## Best Practices

### Security Best Practices
- **Review Generated Configuration**: Always review the generated `codeguardian.toml` file before use
- **Use Security Template**: For production projects, use the `security` template as a starting point
- **Regular Configuration Updates**: Periodically review and update configuration as project evolves
- **Template Customization**: Customize templates to match your organization's security policies

### Performance Optimization Tips
- **CI Template for Pipelines**: Use the `ci` template for CI/CD environments to optimize analysis speed
- **Minimal Template for Development**: Use the `minimal` template during development for faster feedback
- **Project-Specific Patterns**: Customize file patterns to match your project's structure
- **Incremental Configuration**: Start with minimal configuration and gradually add rules

### Common Pitfalls to Avoid
- **Skipping Interactive Setup**: Don't skip the interactive setup unless you have specific requirements
- **Using Wrong Template**: Choose the appropriate template based on your project type and environment
- **Ignoring Configuration Review**: Always review generated configuration files for accuracy
- **Overly Broad Patterns**: Avoid overly broad include/exclude patterns that may miss important files

### Integration Recommendations
- **Team Standardization**: Use consistent templates across similar projects in your organization
- **CI/CD Integration**: Use the `ci` template for automated analysis in CI/CD pipelines
- **Pre-commit Hooks**: Integrate with pre-commit hooks for developer workflow
- **Configuration Versioning**: Version control your configuration files for consistency

### Maintenance Guidelines
- **Regular Template Updates**: Update templates when new security rules become available
- **Configuration Backups**: Keep backups of working configurations for quick recovery
- **Team Training**: Ensure team members understand configuration options and their impact
- **Documentation Updates**: Update project documentation when configuration changes significantly

## See Also
- [`codeguardian check`](check.md) - Run code analysis with the initialized configuration
- [Configuration Guide](../configuration.md) - Detailed configuration options and customization
- [CI/CD Setup Guide](../user-guide/ci-cd-setup.md) - CI/CD integration with configuration templates
- [Security Configuration](../user-guide/security-configuration.md) - Security-focused configuration examples