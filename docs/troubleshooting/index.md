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
# Update Rust
rustup update stable

# Clean and rebuild
cargo clean
cargo install codeguardian

# Check for missing dependencies
# Ubuntu/Debian
sudo apt install build-essential pkg-config libssl-dev

# macOS
xcode-select --install
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

# Check for syntax errors
cat codeguardian.toml | python -c "import toml; toml.load(sys.stdin)"
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
# Enable caching
codeguardian check . --cache-enabled

# Use turbo mode
codeguardian turbo . --max-parallel 8

# Clear cache if corrupted
codeguardian cache clear

# Check system resources
top -p $(pgrep codeguardian)
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

# Verify token permissions
curl -H "Authorization: token $GITHUB_TOKEN" https://api.github.com/user

# Set token in environment
export GITHUB_TOKEN=your_token_here

# Use GitHub CLI authentication
gh auth login
```

#### Rate limit exceeded
```bash
# Check rate limit status
curl -H "Authorization: token $GITHUB_TOKEN" https://api.github.com/rate_limit

# Wait for rate limit reset
sleep 3600

# Use GitHub App authentication for higher limits
# Configure GitHub App in codeguardian.toml
[github]
app_id = "your_app_id"
private_key_path = "/path/to/private-key.pem"
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

#### GitHub Actions failures
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

# Save debug output
codeguardian check . --verbose 2>&1 | tee debug.log

# Check system information
codeguardian doctor
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