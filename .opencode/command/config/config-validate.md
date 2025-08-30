---
title: "config validate"
description: "Validate CodeGuardian configuration files"
category: "Configuration Management"
tags: ["config", "validation", "configuration", "settings"]
---

# config validate

Validate CodeGuardian configuration files for syntax correctness, security settings, and best practices compliance.

## Synopsis

```bash
codeguardian config validate [OPTIONS] [FILES...]
```

## Description

The `config validate` command validates CodeGuardian configuration files, checking for syntax errors, security vulnerabilities in configuration, deprecated settings, and compliance with best practices.

### Key Features

- **Syntax validation**: Check TOML syntax and structure
- **Security validation**: Identify insecure configuration settings
- **Best practices**: Validate against recommended configurations
- **Deprecation warnings**: Alert about deprecated configuration options
- **Multi-file validation**: Validate multiple configuration files

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--config` | Path to configuration file to validate | string | codeguardian.toml | No |
| `--strict` | Enable strict validation (fail on warnings) | boolean | false | No |
| `--format` | Output format (text,json,sarif) | string | text | No |
| `--output` | Output file for validation results | string | stdout | No |
| `--fix` | Automatically fix fixable issues | boolean | false | No |
| `--baseline` | Baseline configuration for comparison | string | - | No |
| `--exclude-rules` | Validation rules to exclude | string[] | [] | No |
| `--include-rules` | Validation rules to include | string[] | [] | No |

## Examples

### Basic Configuration Validation

```bash
# Validate default configuration
codeguardian config validate
```

### Validate Specific Configuration File

```bash
# Validate specific configuration file
codeguardian config validate --config ./custom-config.toml
```

### Strict Validation with JSON Output

```bash
# Strict validation with JSON output
codeguardian config validate --strict --format json --output validation-results.json
```

### Auto-fix Configuration Issues

```bash
# Automatically fix fixable configuration issues
codeguardian config validate --fix
```

### Compare Against Baseline

```bash
# Compare configuration against baseline
codeguardian config validate --baseline ./baseline-config.toml
```

### Validate Multiple Files

```bash
# Validate multiple configuration files
codeguardian config validate config/dev.toml config/prod.toml config/staging.toml
```

## Validation Rules

### Syntax Validation

- **TOML syntax**: Validate correct TOML syntax and structure
- **Required fields**: Check for required configuration fields
- **Type validation**: Ensure correct data types for configuration values
- **Array validation**: Validate array structures and contents

### Security Validation

- **Secret exposure**: Check for exposed secrets in configuration
- **Permission settings**: Validate file permission configurations
- **Network security**: Check network-related security settings
- **Access control**: Validate access control configurations

### Best Practices Validation

- **Performance settings**: Check performance-related configurations
- **Resource limits**: Validate resource allocation settings
- **Logging configuration**: Check logging and monitoring settings
- **Caching configuration**: Validate caching configurations

### Deprecation Validation

- **Deprecated options**: Identify deprecated configuration options
- **Migration paths**: Suggest migration paths for deprecated settings
- **Version compatibility**: Check version compatibility of settings

## Validation Output

### Text Format Output

```text
Configuration Validation Results
================================

File: codeguardian.toml
Status: PASSED

Issues Found: 2

Warnings:
- Line 15: 'debug_mode' is deprecated, use 'log_level' instead
- Line 23: Insecure permission setting detected

Errors: 0

Recommendations:
1. Update deprecated 'debug_mode' setting
2. Review and secure permission settings
3. Consider enabling audit logging
```

### JSON Format Output

```json
{
  "file": "codeguardian.toml",
  "status": "PASSED",
  "summary": {
    "total_issues": 2,
    "warnings": 2,
    "errors": 0,
    "fixed": 0
  },
  "issues": [
    {
      "type": "deprecation",
      "severity": "warning",
      "line": 15,
      "message": "'debug_mode' is deprecated, use 'log_level' instead",
      "rule": "deprecated-option",
      "fixable": true
    },
    {
      "type": "security",
      "severity": "warning",
      "line": 23,
      "message": "Insecure permission setting detected",
      "rule": "insecure-permission",
      "fixable": true
    }
  ],
  "recommendations": [
    "Update deprecated 'debug_mode' setting",
    "Review and secure permission settings",
    "Consider enabling audit logging"
  ]
}
```

## Auto-fix Functionality

### Supported Fixes

The `--fix` option can automatically resolve certain configuration issues:

- **Deprecated options**: Automatically migrate deprecated settings
- **Syntax corrections**: Fix minor syntax issues
- **Default values**: Add missing default values
- **Security improvements**: Apply secure default settings

### Fix Examples

```bash
# Before fix
[logging]
debug_mode = true  # Deprecated

# After fix
[logging]
level = "debug"    # Updated
```

```bash
# Before fix
[security]
audit_log = false  # Missing

# After fix
[security]
audit_log = true   # Added
```

## Configuration Templates

### Generate Valid Configuration

```bash
# Generate a valid configuration template
codeguardian config template > codeguardian.toml
```

### Validate Against Template

```bash
# Compare against recommended template
codeguardian config validate --baseline <(codeguardian config template)
```

## Integration with CI/CD

### GitHub Actions Integration

```yaml
# .github/workflows/config-validation.yml
name: Configuration Validation
on:
  pull_request:
    paths:
      - 'codeguardian.toml'
      - '.codeguardian/**'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Validate Configuration
        run: codeguardian config validate --strict --format sarif > config-validation.sarif
      - name: Upload SARIF
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: config-validation.sarif
```

### GitLab CI Integration

```yaml
# .gitlab-ci.yml
config_validation:
  stage: validate
  script:
    - codeguardian config validate --strict --format json > config-validation.json
  artifacts:
    reports:
      junit: config-validation.json
  only:
    changes:
      - codeguardian.toml
      - .codeguardian/**/*
```

## Best Practices

### Configuration Management

- **Version control**: Keep configuration files under version control
- **Environment separation**: Use different configurations for different environments
- **Regular validation**: Validate configurations regularly
- **Documentation**: Document configuration settings and their purposes

### Security Considerations

- **Secret management**: Never store secrets in configuration files
- **Access control**: Limit access to configuration files
- **Audit logging**: Enable audit logging for configuration changes
- **Backup**: Maintain backups of working configurations

### Validation Strategy

- **Pre-commit hooks**: Validate configuration before commits
- **CI/CD integration**: Validate in CI/CD pipelines
- **Automated fixes**: Use auto-fix for routine corrections
- **Manual review**: Review validation results manually

## Error Handling

### Common Issues

- **File not found**: Ensure configuration file exists
  ```bash
  ls -la codeguardian.toml
  ```

- **Permission denied**: Check file permissions
  ```bash
  chmod 644 codeguardian.toml
  ```

- **Invalid syntax**: Fix TOML syntax errors
  ```bash
  codeguardian config validate --format text
  ```

### Troubleshooting

1. **Check file permissions**:
   ```bash
   ls -la codeguardian.toml
   ```

2. **Validate syntax manually**:
   ```bash
   python -c "import tomllib; tomllib.load(open('codeguardian.toml', 'rb'))"
   ```

3. **Compare with template**:
   ```bash
   codeguardian config template > template.toml
   diff codeguardian.toml template.toml
   ```

4. **Check for hidden characters**:
   ```bash
   cat -A codeguardian.toml | head -20
   ```

## See Also

- [`codeguardian config migrate`](config-migrate.md) - Migrate configuration between versions
- [`codeguardian config env`](config-env.md) - Manage environment-specific configurations
- [`codeguardian config template`](config-template.md) - Generate configuration templates
- [`codeguardian init`](../../../commands/init.md) - Initialize configuration