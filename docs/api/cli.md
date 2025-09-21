# CLI API Documentation

This document provides comprehensive API documentation for CodeGuardian's command-line interface.

## Table of Contents

- [CLI Structure](#cli-structure)
- [Commands](#commands)
- [Options](#options)
- [Configuration](#configuration)
- [Examples](#examples)

## CLI Structure

CodeGuardian uses a hierarchical command structure with subcommands for different operations.

### Main Command

```bash
codeguardian [OPTIONS] <COMMAND>
```

### Available Commands

- `check` - Run code analysis (primary command)
- `report` - Convert results to different formats
- `gh-issue` - Create or update GitHub issues
- `init` - Initialize configuration
- `git-commit` - Perform enhanced git commit with security analysis
- `git-commit-push` - Perform enhanced git commit and push with security analysis
- `turbo` - Run high-performance parallel analysis (turbo mode)
- `train` - Train machine learning model for false positive reduction
- `metrics` - Analyze ML model performance metrics
- `update-docs` - Update and maintain documentation
- `dashboard` - Dashboard management and monitoring (new in v0.2.0)
- `remediation` - Automated remediation workflows (new in v0.2.0)
- `integrations` - External system integrations (new in v0.2.0)
- `bulk` - Bulk operations for multiple repositories and codebases
- `retention` - Retention policy management
- `tune-thresholds` - Tune monitoring thresholds for different environments
- `release-monitoring` - Release monitoring and metrics collection

## Commands

### Analyze Command

```bash
codeguardian analyze [OPTIONS] <FILES>...
```

Analyze specific files for security issues.

**Arguments:**
- `<FILES>...`: Files to analyze

**Options:**
- `-c, --config <FILE>`: Configuration file path
- `-o, --output <FORMAT>`: Output format (json, html, sarif, yaml, text)
- `-v, --verbose`: Enable verbose output
- `--severity <LEVEL>`: Minimum severity level (low, medium, high, critical)

**Example:**
```bash
codeguardian analyze src/main.rs src/lib.rs --output json --verbose
```

### Scan Command

```bash
codeguardian scan [OPTIONS] [PATH]
```

Scan directories or repositories for security issues.

**Arguments:**
- `[PATH]`: Directory or repository path (default: current directory)

**Options:**
- `-c, --config <FILE>`: Configuration file path
- `-o, --output <FORMAT>`: Output format
- `-p, --pattern <PATTERN>`: File pattern to scan (e.g., "*.rs", "*.js")
- `-i, --ignore <PATTERN>`: Patterns to ignore
- `--parallel <NUM>`: Number of parallel workers
- `--cache`: Enable caching for faster scans

**Example:**
```bash
codeguardian scan . --pattern "*.rs" --parallel 4 --cache
```

### Check Command

```bash
codeguardian check [OPTIONS]
```

Check current state, configuration, and system health.

**Options:**
- `-c, --config <FILE>`: Configuration file path
- `--validate-config`: Validate configuration file
- `--system-health`: Check system health
- `--dependencies`: Check dependencies

**Example:**
```bash
codeguardian check --validate-config --system-health
```

### Dashboard Command (New in v0.2.0)

```bash
codeguardian dashboard [OPTIONS]
```

Start the web-based dashboard for monitoring duplicates.

**Options:**
- `-c, --config <FILE>`: Configuration file path
- `-h, --host <HOST>`: Host to bind to (default: 127.0.0.1)
- `-p, --port <PORT>`: Port to bind to (default: 8080)
- `--refresh <SECONDS>`: Dashboard refresh interval
- `--open`: Open dashboard in default browser

**Example:**
```bash
codeguardian dashboard --host 0.0.0.0 --port 3000 --open
```

### Remediate Command (New in v0.2.0)

```bash
codeguardian remediate [OPTIONS] <COMMAND>
```

Run automated remediation workflows.

**Subcommands:**
- `create <FINDINGS_FILE>`: Create remediation workflow from findings
- `list`: List active remediation workflows
- `approve <WORKFLOW_ID>`: Approve pending workflow
- `reject <WORKFLOW_ID> <REASON>`: Reject workflow with reason
- `status <WORKFLOW_ID>`: Get workflow status

**Options:**
- `-c, --config <FILE>`: Configuration file path
- `--auto-approve`: Auto-approve low-risk workflows
- `--dry-run`: Show what would be done without executing

**Examples:**
```bash
# Create workflow from findings
codeguardian remediate create findings.json

# List active workflows
codeguardian remediate list

# Approve workflow
codeguardian remediate approve wf-12345
```

### Integrate Command (New in v0.2.0)

```bash
codeguardian integrate [OPTIONS] <COMMAND>
```

Manage external system integrations.

**Subcommands:**
- `setup <SYSTEM>`: Set up integration with external system
- `test <SYSTEM>`: Test integration connectivity
- `sync`: Synchronize data with external systems
- `webhook`: Manage webhook configurations

**Supported Systems:**
- `jira`
- `confluence`
- `jenkins`
- `gitlab`
- `bitbucket`
- `azure-devops`

**Examples:**
```bash
# Set up Jira integration
codeguardian integrate setup jira

# Test GitLab integration
codeguardian integrate test gitlab

# Synchronize with all systems
codeguardian integrate sync
```

### Relationships Command (New in v0.2.0)

```bash
codeguardian relationships [OPTIONS] <COMMAND>
```

Manage artifact relationships and traceability.

**Subcommands:**
- `discover <ARTIFACT>`: Discover relationships for artifact
- `search <QUERY>`: Search for relationships
- `visualize`: Generate relationship visualization
- `analyze <ARTIFACT>`: Analyze impact of changes
- `stats`: Show relationship statistics

**Examples:**
```bash
# Discover relationships for a file
codeguardian relationships discover src/main.rs

# Search for duplicate relationships
codeguardian relationships search --type duplicate --min-strength 0.8

# Generate visualization
codeguardian relationships visualize --output graph.png
```

### Config Command

```bash
codeguardian config [OPTIONS] <COMMAND>
```

Configuration management.

**Subcommands:**
- `init`: Initialize default configuration
- `validate`: Validate current configuration
- `show`: Show current configuration
- `set <KEY> <VALUE>`: Set configuration value
- `get <KEY>`: Get configuration value

**Examples:**
```bash
# Initialize configuration
codeguardian config init

# Set output format
codeguardian config set output.format json

# Show current config
codeguardian config show
```

### Output Command

```bash
codeguardian output [OPTIONS] <COMMAND>
```

Output formatting and management.

**Subcommands:**
- `format <INPUT> <FORMAT>`: Format analysis results
- `store <RESULTS>`: Store results in database
- `retrieve <ID>`: Retrieve stored results
- `export <QUERY>`: Export results matching query
- `metrics`: Show output metrics

**Examples:**
```bash
# Format results as HTML
codeguardian output format results.json html

# Store results
codeguardian output store results.json

# Export high-severity issues
codeguardian output export --severity high --format sarif
```

## Options

### Global Options

- `-c, --config <FILE>`: Path to configuration file
- `-v, --verbose`: Enable verbose output
- `-q, --quiet`: Suppress output
- `--log-level <LEVEL>`: Set log level (error, warn, info, debug, trace)
- `--no-color`: Disable colored output
- `--help`: Show help information
- `--version`: Show version information

### Configuration File

CodeGuardian supports TOML configuration files:

```toml
[security]
enable_advanced_analysis = true
max_file_size_mb = 10

[output]
format = "json"
pretty_print = true

[integrations]
enabled = true

[integrations.jira]
enabled = true
base_url = "https://your-domain.atlassian.net"
username = "your-username"

[dashboard]
enabled = true
host = "127.0.0.1"
port = 8080

[remediation]
enabled = true
auto_approve_low_risk = true

[relationships]
enabled = true
auto_discovery_enabled = true
```

## Examples

### Basic File Analysis

```bash
# Analyze a single file
codeguardian analyze src/main.rs

# Analyze multiple files with JSON output
codeguardian analyze src/*.rs --output json --verbose

# Scan entire project
codeguardian scan . --pattern "*.rs" --ignore "target/*"
```

### Advanced Analysis with Custom Configuration

```bash
# Use custom configuration file
codeguardian analyze src/ --config custom.toml --parallel 8

# Analyze with specific severity threshold
codeguardian scan . --severity high --output sarif
```

### Dashboard and Monitoring

```bash
# Start dashboard
codeguardian dashboard --host 0.0.0.0 --port 3000

# Check system health
codeguardian check --system-health --dependencies
```

### Integration Management

```bash
# Set up multiple integrations
codeguardian integrate setup jira
codeguardian integrate setup gitlab
codeguardian integrate setup jenkins

# Test all integrations
codeguardian integrate test jira
codeguardian integrate test gitlab

# Synchronize data
codeguardian integrate sync
```

### Remediation Workflows

```bash
# Create remediation workflow from analysis results
codeguardian analyze . --output json > findings.json
codeguardian remediate create findings.json

# Monitor and manage workflows
codeguardian remediate list
codeguardian remediate status wf-12345
codeguardian remediate approve wf-12345
```

### Relationship Analysis

```bash
# Discover relationships in codebase
codeguardian relationships discover src/

# Find duplicate code relationships
codeguardian relationships search --type duplicate --min-strength 0.9

# Analyze impact of changes
codeguardian relationships analyze src/main.rs --change-type modify

# Generate relationship visualization
codeguardian relationships visualize --output relationships.dot
```

### Bulk Operations

```bash
# Scan multiple repositories for duplicates
codeguardian bulk scan repo1 repo2 repo3 --output bulk-results.json

# Scan with high concurrency and skip errors
codeguardian bulk scan repos.txt --concurrency 8 --skip-errors
```

### Threshold Tuning

```bash
# Show current threshold configurations
codeguardian tune-thresholds --show-current

# Generate tuning recommendations based on historical data
codeguardian tune-thresholds --recommend --metrics-file historical.json

# Apply recommendations automatically with high confidence
codeguardian tune-thresholds --apply-recommendations --confidence-threshold 0.9
```

## Exit Codes

- `0`: Success
- `1`: General error
- `2`: Configuration error
- `3`: File access error
- `4`: Analysis error
- `5`: Integration error
- `6`: Remediation error

## Environment Variables

- `CODEGUARDIAN_CONFIG`: Path to configuration file
- `CODEGUARDIAN_LOG_LEVEL`: Log level (error, warn, info, debug, trace)
- `CODEGUARDIAN_CACHE_DIR`: Cache directory path
- `CODEGUARDIAN_OUTPUT_DIR`: Output directory path
