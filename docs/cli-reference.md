# CLI Reference

This document covers CLI options, environment variables, and configuration validation for CodeGuardian.

## CLI Options

CodeGuardian supports various command-line options that complement configuration file settings. The following options are available for the `check` command and provide additional control over analysis behavior.

### Hierarchical Storage Options

CodeGuardian supports hierarchical storage organization for better result management and querying:

```bash
# Enable hierarchical storage with custom directory
codeguardian check --hierarchical-storage --storage-dir ./analysis-storage .

# Use project-based storage strategy
codeguardian check --storage-strategy by_project --project-name my-project .

# Enable compression for stored results
codeguardian check --storage-compress --repository-url https://github.com/org/repo .
```

#### Hierarchical Storage Options

- `--hierarchical-storage` (default: true) - Enable hierarchical storage organization instead of flat file storage
- `--storage-dir <DIR>` (default: "analysis-results") - Base directory for hierarchical storage
- `--storage-strategy <STRATEGY>` (default: "hierarchical_time_based") - Storage organization strategy:
  - `by_date` - Organize by analysis date
  - `by_project` - Organize by project name
  - `hybrid` - Combine date and project organization
  - `hierarchical_time_based` - Time-based hierarchical structure
- `--storage-compress` (default: false) - Enable compression for stored analysis results
- `--project-name <NAME>` - Project name for storage organization (can also use `CODEGUARDIAN_PROJECT` env var)
- `--repository-url <URL>` - Repository URL for storage organization

### AI Enhancement Options

Enable AI-powered analysis features for enhanced security insights:

```bash
# Enable AI-enhanced analysis globally
codeguardian --ai-enhance check .

# Enable AI enhancement for specific check command
codeguardian check --ai-enhance .

# Use custom ML model with threshold
codeguardian check --ml-model ./custom-model.fann --ml-threshold 0.8 .
```

#### AI Enhancement Options

- `--ai-enhance` - Enable AI-enhanced analysis with semantic insights and recommendations (can be set globally or per command)
- `--ml-threshold <THRESHOLD>` - ML confidence threshold for anomaly detection (0.0-1.0)
- `--ml-model <PATH>` - Path to trained ML model file (.fann format) for enhanced detection

### Integration Options

Configure external system integrations via CLI:

```bash
# Analyze with GitHub integration
codeguardian check --emit-gh --repo owner/repo --labels "security,automated" .

# Use custom GitHub token
CODEGUARDIAN_GITHUB_TOKEN=token codeguardian check --emit-gh --repo owner/repo .
```

## Configuration Validation

CodeGuardian provides comprehensive validation to ensure your configuration is correct and compatible with your environment. Validation checks include:

- **Syntax validation**: Ensures TOML syntax is correct
- **Schema validation**: Verifies all required fields are present and have valid values
- **Environment compatibility**: Checks for required dependencies and permissions
- **Integration validation**: Validates external service configurations (GitHub, GitLab)
- **Performance validation**: Ensures performance settings are reasonable for your system

### Validation Process

#### Step-by-Step Configuration Validation

1. **Load Configuration**
    ```bash
    # Load and parse configuration file
    codeguardian check --config codeguardian.toml --dry-run
    ```

2. **Syntax and Schema Validation**
    ```bash
    # Attempt to load configuration - any errors will be reported
    codeguardian check --config codeguardian.toml --verbose
    ```

3. **Environment Compatibility Check**
    ```bash
    # Check system resources and dependencies
    codeguardian check . --parallel 1 --verbose
    ```

4. **Integration Testing**
    ```bash
    # Test GitHub integration (if configured)
    codeguardian check . --emit-gh --dry-run

    # Test GitLab integration (if configured)
    # (Note: GitLab integration requires custom setup)
    ```

5. **Performance Validation**
    ```bash
    # Test with small codebase first
    codeguardian check --paths src/main.rs --verbose

    # Gradually increase scope
    codeguardian check --paths src/ --parallel 2
    ```

#### Practical Examples

**Basic Configuration Validation:**
```bash
# Validate basic configuration with minimal analysis
codeguardian check --config codeguardian.toml --paths . --format json --verbose
```

**Integration Validation:**
```bash
# Test GitHub integration without creating issues
export CODEGUARDIAN_GITHUB_TOKEN="your_token_here"
codeguardian check --config codeguardian.toml --emit-gh --repo owner/repo --dry-run
```

**Performance Validation:**
```bash
# Test performance settings
codeguardian check --config codeguardian.toml --parallel 4 --memory-limit 1024 --verbose
```

**Security Configuration Validation:**
```bash
# Validate security analyzers
codeguardian check --config codeguardian.toml --fail-on-issues --min-severity medium
```

#### Common Validation Issues and Solutions

**Configuration File Not Found:**
```bash
# Error: Configuration file not found
# Solution: Create configuration file
codeguardian init --default
```

**Invalid TOML Syntax:**
```bash
# Error: Failed to parse config file: TOML syntax error
# Solution: Validate TOML syntax
cat codeguardian.toml | python3 -c "import toml; toml.load(stdin)"
```

**Missing Required Fields:**
```bash
# Error: Missing required configuration fields
# Solution: Check against schema and add missing fields
codeguardian init --template security
```

**Integration Configuration Errors:**
```bash
# GitHub token not configured
export CODEGUARDIAN_GITHUB_TOKEN="ghp_..."
codeguardian check --emit-gh --repo owner/repo
```

**Performance Issues:**
```bash
# Reduce parallel workers if memory issues occur
codeguardian check --parallel 2 --memory-limit 512
```

#### Automated Validation Scripts

Create a validation script for CI/CD:

```bash
#!/bin/bash
# validate-config.sh

echo "🔍 Validating CodeGuardian configuration..."

# Check configuration file exists
if [ ! -f "codeguardian.toml" ]; then
    echo "❌ Configuration file not found"
    exit 1
fi

# Validate TOML syntax
if ! python3 -c "import toml; toml.load('codeguardian.toml')" 2>/dev/null; then
    echo "❌ Invalid TOML syntax"
    exit 1
fi

# Test configuration loading
if ! codeguardian check --config codeguardian.toml --dry-run --quiet; then
    echo "❌ Configuration validation failed"
    exit 1
fi

# Test with sample file
echo "📁 Testing with sample analysis..."
if codeguardian check --config codeguardian.toml --paths README.md --quiet; then
    echo "✅ Configuration validation successful"
else
    echo "❌ Configuration test failed"
    exit 1
fi
```

**Run validation script:**
```bash
chmod +x validate-config.sh
./validate-config.sh
```

#### Configuration Health Checks

**Check Configuration Health:**
```bash
# Run analysis and check for configuration-related warnings
codeguardian check . --verbose 2>&1 | grep -i "config\|warning\|error"
```

**Validate Analyzer Configurations:**
```bash
# Test each analyzer individually
codeguardian check --config codeguardian.toml --paths . --verbose | grep "analyzer"
```

**Memory and Performance Validation:**
```bash
# Monitor resource usage during analysis
codeguardian check . --parallel 4 --verbose --metrics
```

For basic configuration options like output, security, analysis, and files, see [configuration-basics.md](configuration-basics.md).

For advanced features like AI, integrations, dashboard, and remediation, see [configuration-advanced.md](configuration-advanced.md).

For preset configurations and templates, see [configuration-presets.md](configuration-presets.md).