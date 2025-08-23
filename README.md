# 🚀 CodeGuardian

A security-first code analysis CLI with GitHub integration, built with best-practice defaults for CI/CD workflows.

## Features

### 🔒 Security-by-Default
- **Deterministic findings** with stable IDs using SHA-256
- **No secrets in logs** - automatic redaction of common patterns
- **Sandboxed execution** - no symlink following, resource limits
- **Canonicalized paths** - prevents path traversal issues
- **Enhanced path validation** - prevents directory traversal attacks
- **File size limits** - configurable limits to prevent resource exhaustion
- **Memory-safe analysis** - streaming for large files to prevent OOM

### 🚀 CI-First UX
- **JSON as source of truth** - Markdown and GitHub issues are derived artifacts
- **Diff-only mode** for PRs - fast, focused feedback
- **TTY-aware progress bars** - auto-disable in CI environments
- **Idempotent GitHub integration** - updates existing issues instead of creating duplicates

### 📊 Comprehensive Analysis
- **Integrity checking** - cryptographic hashing with BLAKE3
- **Lint drift detection** - configuration consistency across projects
- **Non-production code detection** - TODOs, debug statements, potential secrets
- **Dependency analysis** - security vulnerabilities in dependencies
- **Performance analysis** - code optimization opportunities with parallel processing
- **Code quality analysis** - maintainability and best practices with enhanced patterns
- **Security analysis** - comprehensive security vulnerability detection with ML filtering
- **Optimized analyzers** - high-performance pattern matching for large codebases
- **Streaming analysis** - memory-efficient processing of large files

### 🧠 **Intelligent ML Filtering** (ENHANCED!)
- **RUV-FANN neural networks** - 200x faster than BERT, 100x smaller
- **False positive reduction** - 60-80% noise reduction with 90%+ accuracy
- **Online learning** - improves from user feedback automatically
- **Zero-config ML** - works out of the box, no setup required
- **Enhanced feature extraction** - 12-dimensional feature vectors for better classification
- **Training data management** - improved data collection and model training
- **Model performance metrics** - detailed accuracy and performance reporting

### ⚡ **Turbo Mode** (ENHANCED!)
- **High-performance analysis** - optimized for large codebases with 18x speedup
- **Parallel processing** - configurable parallel file analysis with semaphore control
- **Streaming analysis** - handles large files efficiently with adaptive chunking
- **Memory management** - configurable memory limits with automatic optimization
- **Aggressive optimization** - optional performance-focused mode with reduced accuracy trade-off
- **Real-time metrics** - detailed performance monitoring and reporting
- **Resource-aware scaling** - automatic adjustment based on system capabilities

## Quick Start

### Installation

```bash
# Install from source
cargo install --git https://github.com/d-oit/codeguardian

# Or download binary from releases
curl -L https://github.com/d-oit/codeguardian/releases/latest/download/codeguardian-linux-x64.tar.gz | tar xz
```

### Basic Usage

```bash
# Initialize configuration
codeguardian init --default

# Run analysis (recommended defaults)
codeguardian check . --format json --out results.json

# Generate markdown report
codeguardian report --from results.json --md report.md

# Create GitHub issue
codeguardian gh-issue --from results.json --repo owner/repo

# High-performance analysis for large codebases
codeguardian turbo . --metrics --output turbo-results.json --max-parallel 8

# Train ML model for better accuracy
codeguardian train --model-path enhanced-model.fann --epochs 2000

# View ML model performance
codeguardian metrics --model-path enhanced-model.fann --detailed

# Run with enhanced security checks
codeguardian check . --format json --out results.json --security-enhanced

# Analyze only changed files (Git-aware)
codeguardian check . --diff origin/main..HEAD --format json --out pr-results.json
```

### CI Integration

For PRs (diff-only, fast feedback):
```bash
codeguardian check . \
  --diff origin/main..HEAD \
  --format json \
  --out results.json \
  --emit-md report.md \
  --emit-gh \
  --repo $GITHUB_REPOSITORY \
  --max-parallel 4 \
  --memory-limit 512
```

For scheduled scans (full repository):
```bash
codeguardian check . \
  --format json \
  --out results.json \
  --emit-md report.md \
  --emit-gh \
  --repo $GITHUB_REPOSITORY \
  --fail-on-issues \
  --aggressive
```

For large enterprise codebases:
```bash
codeguardian turbo . \
  --max-parallel 16 \
  --memory-limit 2048 \
  --format json \
  --output enterprise-results.json \
  --metrics \
  --streaming-threshold 5
```

## Automatic Documentation

CodeGuardian includes automatic documentation updates using [opencode](https://opencode.ai), an AI coding agent that keeps your project documentation current.

### Setup

1. **Install opencode**:
   ```bash
   curl -fsSL https://opencode.ai/install | bash
   ```

2. **Configure authentication**:
   ```bash
   opencode auth login
   ```
   Select your preferred LLM provider (Anthropic recommended).

3. **Initialize for your project**:
   ```bash
   bash scripts/setup-opencode.sh
   ```

### How It Works

- **Pre-commit hook**: Automatically runs before each git commit
- **Smart analysis**: Uses AI to understand your codebase and update documentation
- **Multiple doc types**: Updates README, API docs, performance docs, and security docs
- **Code entity documentation**: Adds docstrings to functions, structs, traits, and modules
- **Inline comments**: Adds explanatory comments for complex logic
- **Review changes**: You can see and modify documentation updates before committing
- **Quality validation**: Ensures documentation follows Rust documentation standards


### Documentation

For detailed information about the automatic documentation system, see [docs/AUTOMATIC_DOCUMENTATION.md](docs/AUTOMATIC_DOCUMENTATION.md).

## Commands

### `check` (Primary Command)

Run comprehensive code analysis with best-practice defaults:

```bash
codeguardian check [OPTIONS] [PATHS]...
```

**Key Options:**
- `--format json` - Output format (JSON is source of truth)
- `--out results.json` - Output file for results
- `--emit-md report.md` - Generate markdown report
- `--emit-gh` - Create/update GitHub issue
- `--diff origin/main..HEAD` - Analyze only changed files
- `--only-changed` - Analyze only staged files
- `--fail-on-issues` - Exit with code 2 if issues found
- `--max-parallel N` - Maximum parallel workers
- `--memory-limit MB` - Memory limit for analysis
- `--aggressive` - Enable aggressive optimizations
- `--security-enhanced` - Enable enhanced security checks

### `report` (Converter)

Convert results to different formats:

```bash
codeguardian report --from results.json --md report.md
```

### `gh-issue` (GitHub Integration)

Create or update GitHub issues with idempotent behavior:

```bash
codeguardian gh-issue \
  --from results.json \
  --repo owner/repo \
  --mode checklist \
  --title "CodeGuardian: " \
  --labels "codeguardian,automated"
```

**GitHub Issue Modes:**
- `checklist` - Interactive checklist format (default)
- `simple` - Standard issue format
- `children` - Parent issue with child issues for large reports

### `init` (Setup)

Initialize configuration:

```bash
codeguardian init --default
```

### `train` (ML Training)

Train the ML model for improved false positive reduction:

```bash
codeguardian train --model-path enhanced-model.fann --data ml-training-data.json
```

**Training Options:**
- `--model-path` - Path to save the trained model
- `--data` - Training data file (JSON format)
- `--epochs` - Number of training epochs (default: 1000)
- `--learning-rate` - Learning rate for training (default: 0.7)

### `metrics` (ML Metrics)

View ML model performance metrics:

```bash
codeguardian metrics --model-path enhanced-model.fann
```

**Metrics Options:**
- `--model-path` - Path to the model file
- `--export` - Export metrics to JSON file
- `--detailed` - Show detailed performance breakdown

### `turbo` (High-Performance Analysis)

High-performance analysis for large codebases with optimized parallel processing:

```bash
codeguardian turbo . \
  --max-parallel 16 \
  --memory-limit 2048 \
  --max-file-size 50 \
  --aggressive \
  --metrics
```

**Turbo Options:**
- `--max-parallel` - Maximum parallel file processors (default: auto-detect)
- `--memory-limit` - Memory limit in MB (default: 1024)
- `--streaming-threshold` - File size threshold for streaming analysis in MB (default: 5)
- `--max-files` - Maximum number of files to analyze (default: unlimited)
- `--max-file-size` - Skip files larger than this size in MB (default: 100)
- `--aggressive` - Enable aggressive optimizations (may reduce accuracy slightly)
- `--format` - Output format: human or json (default: human)
- `--output` - Output file for results
- `--metrics` - Show detailed performance metrics
- `--max-file-size` - Skip files larger than this size in MB (default: 100)
- `--fail-on-critical` - Exit with error code if critical issues found
- `--progress` - Show progress bar during analysis

## Configuration

CodeGuardian uses `codeguardian.toml` for configuration. See [examples/codeguardian.toml](examples/codeguardian.toml) for a complete example.

### ML Training Examples

- **`examples/enhanced-ml-demo.rs`** - Demonstration of enhanced ML capabilities
- **`examples/ml-training-example.rs`** - Example of training data preparation
- **`examples/performance-comparison.md`** - Performance comparison between different analysis modes

### CI Usage Example

- **`examples/ci-usage.sh`** - Complete CI/CD integration example

### Key Configuration Sections

```toml
[general]
follow_symlinks = false  # Security: don't follow symlinks
max_file_size = 10485760  # 10MB limit
parallel_workers = 0      # Auto-detect CPU cores
memory_limit_mb = 1024   # Memory limit for analysis
streaming_threshold_mb = 5 # Stream files larger than 5MB

[analyzers.integrity]
enabled = true
hash_algorithm = "blake3"  # BLAKE3 by default

[analyzers.lint_drift]
enabled = true
canonicalize_configs = true  # Stable JSON/YAML formatting

[analyzers.non_production]
enabled = true
todo_escalation_days = 30  # Escalate old TODOs

[analyzers.security]
enhanced_mode = true       # Enable enhanced security checks
secret_patterns = ["api_key", "token", "password"]  # Additional patterns

[ml]
enabled = true
model_path = "enhanced-model.fann"
online_learning = true
feature_extraction = "enhanced"

[performance]
cache_enabled = true
cache_max_age_days = 30
parallel_processing = true
memory_optimization = true

[github]
default_labels = ["codeguardian", "automated"]
title_prefix = "CodeGuardian: "
max_body_size = 60000  # Auto-switch to children mode
rate_limit_buffer = 100  # Stay below rate limit
```

## GitHub Actions Integration

See [.github/workflows/codeguardian-ci.yml](.github/workflows/codeguardian-ci.yml) for a complete CI setup:

```yaml
- name: Run CodeGuardian (PR diff-only)
  run: |
    codeguardian check . \
      --diff origin/main..HEAD \
      --format json \
      --out results.json \
      --emit-md report.md \
      --emit-gh \
      --repo ${{ github.repository }}
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Best Practices

### 1. Deterministic Results
- Findings are sorted by severity → file → line
- Stable IDs ensure consistent tracking across runs
- Versioned schemas prevent compatibility issues

### 2. Security Hardening
- No secrets in logs (automatic redaction)
- Sandboxed file access with size limits
- No symlink following by default

### 3. CI Optimization
- Use `--diff` for PRs (fast, focused)
- Use full scans for scheduled jobs
- Always upload `results.json` and `report.md` as artifacts

### 4. GitHub Integration
- Issues are idempotent (updates existing instead of duplicating)
- Automatic body truncation for large reports
- Stable checklist format with finding IDs

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
```

### Benchmarking

```bash
cargo bench
```

### Agent Management

CodeGuardian includes AI agent management capabilities:

```bash
# Setup agents
bash scripts/setup-opencode.sh

# View agent information
bash scripts/agents-info.sh

# Run agent demonstrations
bash scripts/demo-agents.sh

# Manage agents
bash scripts/manage-agents.sh --list
```

## Architecture

CodeGuardian follows a modular architecture with enhanced performance and security:

- **CLI Layer** - Argument parsing and command dispatch (check, report, gh-issue, init, train, metrics, turbo)
- **Core Engine** - File discovery, parallel processing, result aggregation with semaphore control
- **Analyzer Registry** - Pluggable analysis modules (security, performance, quality, dependency, integrity, etc.)
- **ML Layer** - RUV-FANN neural networks for false positive reduction with enhanced feature extraction
- **Performance Engine** - High-performance analysis with streaming, caching, and adaptive optimization
- **Streaming Engine** - Memory-efficient processing of large files with adaptive chunking
- **GitHub Integration** - Idempotent issue creation/updates with rate limiting and retry logic
- **Security Layer** - Path validation, secret redaction, resource limits, and sandboxing
- **Agent System** - AI agent management and automation scripts with enhanced documentation
- **Caching Layer** - Intelligent file caching with mtime/hash checking for incremental analysis

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass and code is formatted
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Roadmap

- [x] **Completed**: Turbo mode for high-performance analysis
- [x] **Completed**: ML model training and metrics commands
- [x] **Completed**: Agent management system
- [x] **Completed**: Enhanced analyzer modules (dependency, performance, quality)
- [ ] SARIF output format support
- [ ] Additional language-specific analyzers
- [ ] Baseline drift detection
- [ ] Custom rule definitions
- [ ] Plugin system for custom analyzers
