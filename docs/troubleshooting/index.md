# 🔍 Troubleshooting Guide

This guide helps you resolve common issues and problems when using CodeGuardian. If you can't find a solution here, please check the [FAQ](faq.md) or create an issue on [GitHub](https://github.com/d-oit/codeguardian/issues).

## Quick Problem Resolution

### Installation Issues

#### "Command not found" after installation
```bash
# Check if CodeGuardian is installed
which codeguardian

# If not found, check Cargo bin directory
ls ~/.cargo/bin/codeguardian

# Add Cargo bin to PATH
export PATH="$HOME/.cargo/bin:$PATH"

# For permanent fix, add to shell profile
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
```

#### Permission denied during installation
```bash
# Install to user directory
cargo install --root ~/.local codeguardian

# Or use sudo for system installation
sudo cargo install codeguardian
```

#### Build failures during installation
```bash
# Update Rust toolchain
rustup update stable

# Clean and rebuild
cargo clean
cargo install codeguardian

# Check for missing system dependencies
# Ubuntu/Debian
sudo apt install build-essential pkg-config libssl-dev libgit2-dev

# CentOS/RHEL/Fedora
sudo dnf install gcc pkg-config openssl-devel libgit2-devel

# macOS
xcode-select --install
brew install pkg-config openssl git2

# Windows (using MSYS2 or similar)
pacman -S mingw-w64-x86_64-gcc mingw-w64-x86_64-pkg-config mingw-w64-x86_64-openssl

# Verify Rust version compatibility (requires Rust 1.70+ for edition 2021)
rustc --version
cargo --version

# Build with verbose output for debugging
cargo build --verbose

# Check for dependency conflicts
cargo tree
```

### Configuration Issues

#### "Configuration file not found"
```bash
# Create basic configuration
codeguardian init

# Create with specific template
codeguardian init --template security

# Specify config file path
codeguardian check . --config /path/to/codeguardian.toml
```

#### Invalid configuration syntax
```bash
# Validate configuration
codeguardian config validate

# Generate example configuration
codeguardian config generate > codeguardian.toml

# Check for TOML syntax errors
cat codeguardian.toml | python -c "import toml; toml.load(sys.stdin)"

# Validate specific sections
# Check [general] section
codeguardian check . --config codeguardian.toml --verbose | head -20

# Check analyzer-specific configurations
codeguardian check . --analyzer security --config codeguardian.toml
```

#### Memory and performance configuration issues
```toml
# codeguardian.toml - Memory limits
[general]
max_memory_mb = 512  # Increase if getting OOM errors
parallel_workers = 1  # Reduce for memory-constrained environments

[optimization]
enable_file_caching = true
max_parallel_workers = 2
max_memory_file_size = 10485760  # 10MB limit per file

[optimization.cache_cleanup]
enabled = true
max_age_days = 7
max_size_mb = 100
```

#### Analyzer-specific configuration problems
```toml
# Security analyzer configuration
[security_analyzer]
enabled = true
min_entropy_threshold = 3.5  # Adjust for false positives
check_hardcoded_secrets = true

# Performance analyzer configuration
[performance_analyzer]
enabled = true
max_complexity = 15  # Increase for complex codebases
max_function_length = 100  # Adjust based on coding standards

# ML configuration
[ml]
enabled = true
model_path = "enhanced-model.fann"
confidence_threshold = 0.8
```

#### Path and file pattern issues
```toml
# Correct exclude patterns
exclude_patterns = [
    "target/**",
    ".git/**",
    "node_modules/**",
    "**/*.min.js",
    "**/*.bundle.js"
]

# Include specific file types
include_patterns = [
    "**/*.rs",
    "**/*.js",
    "**/*.ts",
    "**/*.py"
]
```

#### Settings not applied
```bash
# Check configuration precedence
codeguardian config list

# Use environment variables
CODEGUARDIAN_LOG_LEVEL=debug codeguardian check .

# Verify configuration file location
codeguardian check . --config ./codeguardian.toml
```

### Analysis Issues

#### No findings reported
```bash
# Check supported file types
codeguardian check . --verbose

# Include specific file patterns
codeguardian check . --include "*.rs" --include "*.js"

# Check file permissions
ls -la src/

# Verify analyzer configuration
codeguardian check . --analyzer security --verbose
```

#### Analysis timeout errors
```bash
# Increase timeout
codeguardian check . --timeout 60

# Use streaming for large files
codeguardian check . --streaming-threshold 1

# Reduce parallel workers
codeguardian check . --max-parallel 2
```

#### Memory limit exceeded
```bash
# Increase memory limit
codeguardian check . --memory-limit 2048

# Enable streaming analysis
codeguardian check . --streaming-threshold 5

# Use turbo mode with optimizations
codeguardian turbo . --memory-limit 4096
```

#### Large file handling issues
```bash
# Enable streaming analysis
codeguardian check . --streaming-threshold 1

# Increase memory limit
codeguardian check . --memory-limit 4096

# Exclude large files
codeguardian check . --exclude "*.min.js" --exclude "*.bundle.js"
```

### ML and Performance Issues

#### ML model not working
```bash
# Check ML model path
ls -la enhanced-model.fann

# Disable ML features
codeguardian check . --no-ml

# Train new model
codeguardian train --model-path enhanced-model.fann --epochs 2000

# Use ML with lower confidence threshold
codeguardian check . --ml-model enhanced-model.fann --ml-threshold 0.7
```

#### Performance degradation
```bash
# Enable caching and optimization
codeguardian check . --cache-enabled --streaming-threshold 5

# Use turbo mode with optimized settings
codeguardian turbo . --max-parallel 4 --memory-limit 1024

# Clear corrupted cache
codeguardian cache clear
rm -rf ~/.cache/codeguardian/

# Monitor system resources during scan
codeguardian check . --metrics | tee performance.log

# Profile with system tools
timeout 60s perf record -g codeguardian check . --max-parallel 1
perf report
```

#### Memory usage optimization
```bash
# Adjust memory settings based on system
codeguardian check . \
  --memory-limit 512 \
  --streaming-threshold 10 \
  --max-parallel 2

# Monitor memory usage in real-time
codeguardian check . --metrics &
watch -n 5 'ps aux | grep codeguardian'

# Use memory profiling
valgrind --tool=massif --massif-out-file=massif.out codeguardian check . --max-parallel 1
ms_print massif.out | head -50
```

#### Large codebase handling
```bash
# For very large codebases (>100k files)
codeguardian check . \
  --streaming-threshold 1 \
  --max-parallel 1 \
  --memory-limit 2048 \
  --timeout 3600

# Exclude unnecessary directories
codeguardian check . \
  --exclude "node_modules/**" \
  --exclude "target/**" \
  --exclude ".git/**" \
  --exclude "build/**"

# Use incremental analysis
codeguardian check . --diff HEAD~1
```

#### Parallel processing issues
```bash
# Adjust parallelism based on CPU cores
nproc  # Check available cores
codeguardian check . --max-parallel $(nproc)

# For systems with limited resources
codeguardian check . --max-parallel 1 --streaming-threshold 5

# Monitor CPU usage
codeguardian check . --metrics &
htop -p $(pgrep codeguardian)
```

#### High memory usage
```bash
# Monitor memory usage
codeguardian check . --metrics

# Enable memory optimization
codeguardian check . --memory-limit 1024

# Use streaming for large files
codeguardian check . --streaming-threshold 5

# Reduce parallel workers
codeguardian check . --max-parallel 2
```

### GitHub Integration Issues

#### Authentication failed
```bash
# Check GitHub token
echo $GITHUB_TOKEN

# Verify token permissions and scope
curl -H "Authorization: token $GITHUB_TOKEN" https://api.github.com/user
curl -H "Authorization: token $GITHUB_TOKEN" https://api.github.com/user/repos

# Set token with proper scope
export GITHUB_TOKEN=your_personal_access_token

# Use GitHub CLI for authentication
gh auth login
gh auth status

# Check token permissions for repository
gh repo view owner/repo --permission
```

#### Repository access and permissions
```bash
# Verify repository exists and is accessible
gh repo view owner/repo

# Check if token has required permissions
curl -H "Authorization: token $GITHUB_TOKEN" \
  https://api.github.com/repos/owner/repo

# Test issue creation permissions
gh issue list --repo owner/repo --limit 1

# Check organization permissions if applicable
gh org list
```

#### Rate limit exceeded
```bash
# Check current rate limit status
curl -H "Authorization: token $GITHUB_TOKEN" https://api.github.com/rate_limit

# Wait for rate limit reset
sleep 3600

# Use GitHub App authentication for higher limits (5000 requests/hour)
# Configure GitHub App in codeguardian.toml
[github]
app_id = "your_app_id"
private_key_path = "/path/to/private-key.pem"
installation_id = "your_installation_id"

# Use personal access token with higher limits
# Create token with repo scope for private repos
export GITHUB_TOKEN=your_high_limit_token

# Implement retry logic in CI
- name: Run CodeGuardian with retry
  uses: nick-invision/retry@v2
  with:
    timeout_minutes: 10
    max_attempts: 3
    command: codeguardian check . --emit-gh --repo owner/repo
```

#### Issue creation and management failures
```bash
# Test issue creation manually
gh issue create \
  --repo owner/repo \
  --title "Test CodeGuardian Issue" \
  --body "Testing issue creation" \
  --label "codeguardian,test"

# Check existing issues to avoid duplicates
gh issue list --repo owner/repo --label codeguardian --state open

# Verify issue template configuration
codeguardian gh-issue --dry-run --from results.json --repo owner/repo

# Handle large issue bodies (GitHub 65536 char limit)
codeguardian gh-issue \
  --from results.json \
  --repo owner/repo \
  --mode simple \
  --summary-max-chars 500
```

#### Webhook and automation issues
```bash
# Verify webhook delivery
gh api repos/owner/repo/hooks

# Check webhook payload format
curl -X POST \
  -H "Content-Type: application/json" \
  -H "X-GitHub-Event: pull_request" \
  -d '{"action":"opened"}' \
  http://your-webhook-endpoint

# Test webhook signature verification
# Ensure webhook secret is configured correctly
export GITHUB_WEBHOOK_SECRET=your_secret
```

#### Issue creation failed
```bash
# Check repository permissions
gh repo view owner/repo --permission

# Verify repository exists
gh repo view owner/repo

# Check issue template configuration
codeguardian gh-issue --dry-run --from results.json --repo owner/repo

# Use simple issue mode
codeguardian gh-issue --from results.json --repo owner/repo --mode simple
```

#### Repository not found
```bash
# Check repository name format
# Should be: owner/repository
codeguardian gh-issue --repo owner/repo --dry-run

# Verify repository exists and is accessible
gh repo clone owner/repo /tmp/test-repo

# Check organization permissions
gh org list
```

### Output and Reporting Issues

#### JSON output malformed
```bash
# Validate JSON output
codeguardian check . --format json --out results.json
cat results.json | jq .

# Check for special characters in file paths
codeguardian check . --format json | python -m json.tool

# Use pretty print
codeguardian check . --format json --pretty
```

#### Report generation failed
```bash
# Check input file
cat results.json | head -20

# Generate with verbose output
codeguardian report --from results.json --md report.md --verbose

# Check template syntax
codeguardian report --from results.json --template custom-template.md

# Generate minimal report
codeguardian report --from results.json --format markdown
```

#### HTML report not displaying correctly
```bash
# Check HTML file
cat report.html | head -20

# Open in browser
open report.html

# Check for missing assets
ls -la docs/assets/

# Regenerate with different template
codeguardian report --from results.json --html report.html --template minimal
```

### Docker Issues

#### Container permission errors
```bash
# Run with current user
docker run --rm -u $(id -u):$(id -g) -v $(pwd):/workspace codeguardian/codeguardian check /workspace

# Use Docker with sudo
sudo docker run --rm -v $(pwd):/workspace codeguardian/codeguardian check /workspace

# Check volume permissions
ls -la $(pwd)
```

#### Docker memory issues
```bash
# Set memory limit
docker run --rm --memory=2g -v $(pwd):/workspace codeguardian/codeguardian check /workspace

# Use streaming analysis
docker run --rm -v $(pwd):/workspace codeguardian/codeguardian check /workspace --streaming-threshold 5

# Monitor container resources
docker stats $(docker ps -q)
```

#### Custom configuration in Docker
```bash
# Mount configuration file
docker run --rm \
  -v $(pwd):/workspace \
  -v $(pwd)/codeguardian.toml:/etc/codeguardian.toml \
  codeguardian/codeguardian check /workspace

# Use environment variables
docker run --rm \
  -v $(pwd):/workspace \
  -e CODEGUARDIAN_MEMORY_LIMIT_MB=2048 \
  codeguardian/codeguardian check /workspace
```

### CI/CD Integration Issues

#### GitHub Actions workflow failures
```yaml
# Check workflow permissions in .github/workflows/codeguardian-ci.yml
permissions:
  contents: read
  issues: write
  pull-requests: write
  actions: read  # For workflow artifacts

# Ensure proper token scope
env:
  GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

# Handle diff-only mode failures
- name: Run CodeGuardian (PR diff-only)
  run: |
    # Check if diff exists before running
    if git diff --quiet HEAD~1; then
      echo "No changes detected, skipping analysis"
      exit 0
    fi

    ./target/release/codeguardian check . \
      --diff origin/main..HEAD \
      --format json \
      --out results.json
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

#### CI build cache issues
```bash
# Clear Rust cache in CI
- name: Clear cache
  run: |
    cargo clean
    rm -rf ~/.cargo/registry/cache/
    rm -rf ~/.cargo/git/db/

# Use incremental builds
- name: Build with incremental
  run: cargo build --release --incremental
```

#### Artifact upload failures
```yaml
# Ensure artifacts directory exists
- name: Create artifacts directory
  run: mkdir -p artifacts

# Upload with proper error handling
- name: Upload results
  uses: actions/upload-artifact@v4
  if: always()
  with:
    name: codeguardian-results
    path: |
      results.json
      report.md
    retention-days: 30
```

#### Scheduled job failures
```yaml
# Add timeout and error handling for scheduled runs
- name: Run scheduled scan
  timeout-minutes: 30
  run: |
    ./target/release/codeguardian check . \
      --format json \
      --out results.json \
      --timeout 1800 \
      --fail-on-issues
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

#### GitLab CI issues
```yaml
# Check workflow permissions
permissions:
  contents: read
  issues: write
  pull-requests: write

# Verify token scope
env:
  GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

# Add error handling
- name: Run CodeGuardian
  continue-on-error: true
  run: |
    codeguardian check . --format json --out results.json || echo "Analysis failed"
```

#### GitLab CI issues
```yaml
# Check GitLab token permissions
variables:
  GITLAB_TOKEN: $GITLAB_API_TOKEN

# Use proper image
image: codeguardian/codeguardian:latest

# Handle merge request diffs
script:
  - |
    if [ -n "$CI_MERGE_REQUEST_TARGET_BRANCH_NAME" ]; then
      codeguardian check . --diff origin/$CI_MERGE_REQUEST_TARGET_BRANCH_NAME..HEAD
    fi
```

#### Jenkins pipeline errors
```groovy
// Check Jenkins agent
agent {
    docker {
        image 'codeguardian/codeguardian:latest'
        args '-v $WORKSPACE:/workspace -w /workspace'
    }
}

// Handle workspace permissions
steps {
    sh '''
        chmod -R 755 .
        codeguardian check . --format json --out results.json
    '''
}
```

### Network and Connectivity Issues

#### Proxy configuration
```bash
# Set proxy environment variables
export HTTP_PROXY=http://proxy.example.com:8080
export HTTPS_PROXY=http://proxy.example.com:8080

# Configure in codeguardian.toml
[network]
proxy = "http://proxy.example.com:8080"
no_proxy = "localhost,127.0.0.1"
```

#### SSL certificate errors
```bash
# Disable SSL verification (not recommended for production)
export SSL_VERIFY=false

# Update CA certificates
# Ubuntu/Debian
sudo apt install ca-certificates
sudo update-ca-certificates

# CentOS/RHEL
sudo yum install ca-certificates
sudo update-ca-trust
```

#### DNS resolution issues
```bash
# Check DNS resolution
nslookup github.com

# Use different DNS server
echo "nameserver 8.8.8.8" | sudo tee /etc/resolv.conf

# Add to hosts file
echo "140.82.121.4 github.com" | sudo tee -a /etc/hosts
```

### Platform-Specific Issues

#### Windows-specific problems
```powershell
# Check execution policy
Get-ExecutionPolicy

# Allow script execution
Set-ExecutionPolicy RemoteSigned

# Check PATH
$env:PATH

# Add to PATH
$env:PATH += ";C:\Users\$env:USERNAME\.cargo\bin"
```

#### macOS-specific issues
```bash
# Check Xcode command line tools
xcode-select --install

# Fix Homebrew permissions
brew doctor

# Check Gatekeeper
spctl --status

# Allow downloaded applications
spctl --add /path/to/codeguardian
```

#### Linux-specific issues
```bash
# Check system libraries
ldd $(which codeguardian)

# Install missing dependencies
# Ubuntu/Debian
sudo apt install libssl-dev pkg-config

# CentOS/RHEL
sudo yum install openssl-devel pkgconfig

# Check SELinux
sestatus
setenforce 0  # Temporarily disable if needed
```

### Advanced Troubleshooting

#### Debug Mode
```bash
# Enable debug logging
CODEGUARDIAN_LOG_LEVEL=debug codeguardian check .

# Save debug output with timestamps
codeguardian check . --verbose 2>&1 | ts '%Y-%m-%d %H:%M:%S' | tee debug.log

# Check system information and diagnostics
codeguardian doctor

# Debug specific analyzers
codeguardian check . --analyzer security --verbose
codeguardian check . --analyzer performance --verbose

# Debug configuration loading
CODEGUARDIAN_LOG_LEVEL=debug codeguardian config validate
```

#### Dependency and environment debugging
```bash
# Check Rust and Cargo versions
rustc --version
cargo --version
rustup show

# Verify system dependencies
ldd $(which codeguardian)  # Linux
otool -L $(which codeguardian)  # macOS

# Check for conflicting installations
which -a codeguardian
ls -la ~/.cargo/bin/codeguardian

# Debug dependency resolution
cargo tree --duplicates
cargo update
```

#### Performance Profiling
```bash
# Enable performance metrics
codeguardian check . --metrics

# Profile with external tools
perf record codeguardian check .
perf report

# Memory profiling
valgrind --tool=massif codeguardian check .
```

#### Log Analysis
```bash
# Search for specific errors
grep "ERROR" debug.log

# Check for warnings
grep "WARN" debug.log

# Analyze performance
grep "performance" debug.log | sort -k2 -n
```

#### System Diagnostics
```bash
# Check system resources
df -h          # Disk space
free -h        # Memory
top -p $(pgrep codeguardian)  # Process info

# Network diagnostics
ping github.com
curl -I https://api.github.com

# File system checks
find . -type f -size +100M  # Large files
ls -la ~/.cache/codeguardian  # Cache directory
```

### Common Error Messages and Solutions

#### "Failed to compile CodeGuardian"
```
error: linking with `cc` failed: exit code: 1
```
```bash
# Install missing system libraries
# Ubuntu/Debian
sudo apt install build-essential pkg-config libssl-dev libgit2-dev

# macOS
brew install pkg-config openssl git2

# Check for 32-bit vs 64-bit issues
uname -m
rustup target list --installed
```

#### "Permission denied" errors
```bash
# Fix file permissions
chmod +x ~/.cargo/bin/codeguardian

# Run with sudo if necessary (not recommended)
sudo -u $(whoami) codeguardian check .

# Check directory permissions
ls -la /path/to/project
chmod -R 755 /path/to/project
```

#### "No such file or directory" for configuration
```bash
# Create default configuration
codeguardian init

# Specify full path to config
codeguardian check . --config /full/path/to/codeguardian.toml

# Check if file exists
ls -la codeguardian.toml
```

#### "Memory allocation failed" or OOM errors
```bash
# Reduce memory usage
codeguardian check . --memory-limit 256 --max-parallel 1

# Use streaming for large files
codeguardian check . --streaming-threshold 1

# Monitor memory usage
free -h  # Linux
vm_stat  # macOS
```

#### "Timeout exceeded" errors
```bash
# Increase timeout values
codeguardian check . --timeout 1800

# Reduce analysis scope
codeguardian check . --exclude "node_modules/**" --max-parallel 1

# Use diff mode for faster analysis
codeguardian check . --diff HEAD~1
```

#### GitHub API errors
```bash
# "Bad credentials"
# Check token validity
curl -H "Authorization: token $GITHUB_TOKEN" https://api.github.com/user

# "Repository not found"
gh repo view owner/repo

# "Issues are disabled for this repository"
# Enable issues in repository settings
gh repo edit owner/repo --enable-issues=true
```

### Getting Professional Help

If you continue to experience issues:

1. **Gather Information**
   ```bash
   # System information
   uname -a
   codeguardian --version
   cargo --version

   # Configuration
   cat codeguardian.toml

   # Debug logs
   CODEGUARDIAN_LOG_LEVEL=debug codeguardian check . 2>&1 | head -100
   ```

2. **Create a Detailed Issue**
   - Use the [GitHub issue template](https://github.com/d-oit/codeguardian/issues/new)
   - Include system information and configuration
   - Attach debug logs and error messages
   - Describe steps to reproduce the issue

3. **Contact Support**
   - Enterprise customers: Use dedicated support channels
   - Community: Use [GitHub Discussions](https://github.com/d-oit/codeguardian/discussions)

### Prevention Best Practices

#### Regular Maintenance
```bash
# Update CodeGuardian regularly
cargo install codeguardian --force

# Clear cache periodically
codeguardian cache clear

# Validate configuration
codeguardian config validate
```

#### Monitoring and Alerts
```bash
# Monitor analysis results
codeguardian check . --format json | jq '.summary.total_findings'

# Set up alerts for critical findings
codeguardian check . --fail-on-issues --min-severity critical

# Track performance metrics
codeguardian check . --metrics | tee metrics.log
```

#### Backup and Recovery
```bash
# Backup configuration
cp codeguardian.toml codeguardian.toml.backup

# Backup ML models
cp enhanced-model.fann enhanced-model.fann.backup

# Export analysis results
codeguardian check . --format json --out backup-results.json
```

This comprehensive troubleshooting guide should help resolve most issues. For additional support, don't hesitate to reach out to the community or create an issue on GitHub.

---

<div align="center">

**[⬅️ Back to Documentation](../README.md)** | **[❓ FAQ](faq.md)** | **[🐛 Report Issues](https://github.com/d-oit/codeguardian/issues)**

</div>
