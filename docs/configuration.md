# Configuration Guide

CodeGuardian works out of the box with sensible defaults, but can be customized via `codeguardian.toml`.

This guide is organized into focused sections for easier navigation:

## 📋 Configuration Sections

- **[Configuration Basics](configuration-basics.md)**: Core settings including output, security, analysis, git, files, severity, and all analyzer configurations
- **[Configuration Advanced](configuration-advanced.md)**: Advanced features including AI, integrations (GitHub/GitLab), optimization, retention, dashboard, and remediation
- **[Configuration Presets](configuration-presets.md)**: Preset configurations (minimal, security, CI) plus examples for security-focused, performance-focused, and CI/CD configurations
- **[CLI Reference](cli-reference.md)**: CLI options (hierarchical storage, AI enhancement, integration options), configuration validation process, practical examples, troubleshooting, and automated validation scripts

## 🚀 Quick Start

For basic usage, CodeGuardian works with default settings. For customization:

1. **Basic Setup**: See [Configuration Basics](configuration-basics.md) for core settings
2. **Advanced Features**: Check [Configuration Advanced](configuration-advanced.md) for integrations and optimization
3. **Presets**: Use [Configuration Presets](configuration-presets.md) for common scenarios
4. **CLI Options**: Review [CLI Reference](cli-reference.md) for command-line overrides

## 📖 Examples

### Minimal Configuration
```toml
[output]
directory = "build/analysis-results"
format = "json"

[security]
enabled = true
min_severity = "medium"
```

### Full Configuration
See the detailed sections linked above for comprehensive examples.

## 🔧 Configuration Validation

CodeGuardian validates configuration on startup. Use `--verbose` to see validation details:

```bash
codeguardian check --config codeguardian.toml --verbose
```

For more details on validation, see [CLI Reference](cli-reference.md).

---

*This overview provides navigation to detailed configuration sections. Choose the appropriate section based on your needs.*</content>
</xai:function_call