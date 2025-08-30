---
title: "ci-cd github-actions"
description: "Generate optimized GitHub Actions workflows for CodeGuardian integration"
category: "CI/CD Platform"
tags: ["github", "actions", "ci-cd", "automation", "workflows"]
---

# ci-cd github-actions

Generate comprehensive GitHub Actions workflows for CodeGuardian integration, including security scanning, automated testing, and deployment pipelines with optimized performance and security.

## Synopsis

```bash
codeguardian ci-cd github-actions [OPTIONS] [COMMAND]
```

## Description

The `ci-cd github-actions` command creates optimized GitHub Actions workflows for CodeGuardian, supporting various CI/CD patterns including pull request validation, scheduled security scans, and automated deployments.

### Key Features

- **Workflow optimization**: Generate efficient, fast CI/CD pipelines
- **Security integration**: Automated security scanning and validation
- **Multi-environment support**: Development, staging, and production workflows
- **Performance monitoring**: Build performance tracking and optimization
- **Compliance automation**: Automated compliance checking and reporting

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--workflow-type` | Type of workflow (pr,main,scheduled,release) | string | pr | No |
| `--output-dir` | Output directory for workflow files | string | .github/workflows | No |
| `--include-security` | Include security scanning steps | boolean | true | No |
| `--include-performance` | Include performance monitoring | boolean | false | No |
| `--runners` | GitHub runners to use | string[] | ubuntu-latest | No |
| `--cache-enabled` | Enable build caching | boolean | true | No |
| `--matrix-build` | Enable matrix builds for multiple environments | boolean | false | No |
| `--artifact-retention` | Artifact retention period in days | number | 30 | No |

## Commands

### generate
Generate GitHub Actions workflow files.

```bash
codeguardian ci-cd github-actions generate [OPTIONS]
```

### validate
Validate existing GitHub Actions workflows.

```bash
codeguardian ci-cd github-actions validate [OPTIONS]
```

### optimize
Optimize existing GitHub Actions workflows.

```bash
codeguardian ci-cd github-actions optimize [OPTIONS]
```

### monitor
Monitor GitHub Actions workflow performance.

```bash
codeguardian ci-cd github-actions monitor [OPTIONS]
```

## Examples

### Generate Pull Request Workflow

```bash
# Generate PR validation workflow
codeguardian ci-cd github-actions generate \
  --workflow-type pr \
  --include-security \
  --cache-enabled
```

### Generate Main Branch Workflow

```bash
# Generate main branch deployment workflow
codeguardian ci-cd github-actions generate \
  --workflow-type main \
  --include-performance \
  --matrix-build
```

### Generate Scheduled Security Scan

```bash
# Generate scheduled security scanning workflow
codeguardian ci-cd github-actions generate \
  --workflow-type scheduled \
  --runners ubuntu-latest,windows-latest
```

### Validate Existing Workflows

```bash
# Validate existing workflow files
codeguardian ci-cd github-actions validate \
  --output-dir .github/workflows
```

### Optimize Workflow Performance

```bash
# Optimize existing workflows for performance
codeguardian ci-cd github-actions optimize \
  --output-dir .github/workflows
```

## Pull Request Validation Workflow

### Basic PR Validation

```yaml
# .github/workflows/pr-validation.yml
name: PR Validation
on:
  pull_request:
    branches: [main, develop]
    types: [opened, synchronize, reopened]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Run CodeGuardian
        id: codeguardian
        run: |
          cargo install --git https://github.com/your-org/codeguardian
          codeguardian check --fail-on-issues --emit-gh --repo ${{ github.repository }}

      - name: Upload results
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: codeguardian-results
          path: results.json
          retention-days: 30
```

### Advanced PR Validation with Matrix

```yaml
# .github/workflows/pr-validation-matrix.yml
name: PR Validation Matrix
on:
  pull_request:
    branches: [main, develop]

jobs:
  validate:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta]
        exclude:
          - os: macos-latest
            rust: beta

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Run tests
        run: cargo test

      - name: Run CodeGuardian
        run: |
          cargo install --git https://github.com/your-org/codeguardian
          codeguardian check --fail-on-issues --format json > results-${{ matrix.os }}-${{ matrix.rust }}.json

      - name: Upload results
        uses: actions/upload-artifact@v4
        with:
          name: codeguardian-results-${{ matrix.os }}-${{ matrix.rust }}
          path: results-*.json
```

## Main Branch Deployment Workflow

### Production Deployment

```yaml
# .github/workflows/deploy-main.yml
name: Deploy Main
on:
  push:
    branches: [main]
  workflow_dispatch:

jobs:
  security-scan:
    runs-on: ubuntu-latest
    outputs:
      scan-results: ${{ steps.scan.outputs.results }}
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Run security scan
        id: scan
        run: |
          codeguardian check --fail-on-issues --format sarif > security-results.sarif
          echo "results=security-results.sarif" >> $GITHUB_OUTPUT

      - name: Upload SARIF results
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: security-results.sarif

  build:
    needs: security-scan
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Build release
        run: cargo build --release

      - name: Run tests
        run: cargo test --release

      - name: Create release archive
        run: |
          tar -czf codeguardian-${{ github.sha }}.tar.gz target/release/codeguardian
          echo "ARTIFACT_PATH=codeguardian-${{ github.sha }}.tar.gz" >> $GITHUB_ENV

      - name: Upload build artifact
        uses: actions/upload-artifact@v4
        with:
          name: codeguardian-binary
          path: ${{ env.ARTIFACT_PATH }}

  deploy:
    needs: [security-scan, build]
    runs-on: ubuntu-latest
    environment: production
    steps:
      - name: Download artifact
        uses: actions/download-artifact@v4
        with:
          name: codeguardian-binary

      - name: Deploy to production
        run: |
          # Deployment logic here
          echo "Deploying to production..."
```

## Scheduled Security Scanning

### Weekly Security Audit

```yaml
# .github/workflows/weekly-security-scan.yml
name: Weekly Security Scan
on:
  schedule:
    - cron: '0 2 * * 1'  # Every Monday at 2 AM UTC
  workflow_dispatch:

jobs:
  security-audit:
    runs-on: ubuntu-latest
    permissions:
      security-events: write
      contents: read
      actions: read

    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Run comprehensive security scan
        run: |
          codeguardian check \
            --fail-on-issues \
            --format sarif \
            --output security-scan-results.sarif \
            --include "src/**" \
            --exclude "tests/**"

      - name: Upload SARIF results
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: security-scan-results.sarif
          category: codeguardian-security-scan

      - name: Generate security report
        run: |
          codeguardian report \
            --from security-scan-results.sarif \
            --format markdown \
            --output security-report.md

      - name: Create security issue
        if: failure()
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const report = fs.readFileSync('security-report.md', 'utf8');

            await github.rest.issues.create({
              owner: context.repo.owner,
              repo: context.repo.repo,
              title: 'Weekly Security Scan Results',
              body: report,
              labels: ['security', 'automated']
            });
```

## Release Workflow

### Automated Release

```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Run final security check
        run: |
          codeguardian check --fail-on-issues --format json > final-check.json

      - name: Build release binaries
        run: |
          # Build for multiple platforms
          cargo build --release --target x86_64-unknown-linux-gnu
          cargo build --release --target x86_64-apple-darwin
          cargo build --release --target x86_64-pc-windows-msvc

      - name: Create release archives
        run: |
          # Linux
          tar -czf codeguardian-${{ github.ref_name }}-linux-x64.tar.gz \
            -C target/x86_64-unknown-linux-gnu/release codeguardian

          # macOS
          tar -czf codeguardian-${{ github.ref_name }}-macos-x64.tar.gz \
            -C target/x86_64-apple-darwin/release codeguardian

          # Windows
          cd target/x86_64-pc-windows-msvc/release
          zip ../../../codeguardian-${{ github.ref_name }}-windows-x64.zip codeguardian.exe
          cd ../../..

      - name: Create GitHub release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            codeguardian-*-linux-x64.tar.gz
            codeguardian-*-macos-x64.tar.gz
            codeguardian-*-windows-x64.zip
          generate_release_notes: true
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Performance Monitoring Workflow

### Build Performance Tracking

```yaml
# .github/workflows/performance-monitor.yml
name: Performance Monitor
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  performance-check:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Build with timing
        run: |
          echo "Build start: $(date +%s)" > build-timing.txt
          cargo build --release --timings
          echo "Build end: $(date +%s)" >> build-timing.txt

      - name: Run performance benchmarks
        run: |
          cargo bench > benchmarks.txt

      - name: Analyze build performance
        run: |
          codeguardian performance benchmark \
            --input build-timing.txt \
            --output build-performance.json

      - name: Upload performance results
        uses: actions/upload-artifact@v4
        with:
          name: performance-results
          path: |
            build-performance.json
            benchmarks.txt
            cargo-timing.html

      - name: Comment PR with performance results
        if: github.event_name == 'pull_request'
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const performance = JSON.parse(fs.readFileSync('build-performance.json', 'utf8'));

            const comment = `## Build Performance Results
            - Build time: ${performance.build_time}s
            - Peak memory: ${performance.peak_memory}MB
            - Cache hit rate: ${performance.cache_hit_rate}%

            [View detailed report](${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }})`;

            await github.rest.issues.createComment({
              owner: context.repo.owner,
              repo: context.repo.repo,
              issue_number: context.issue.number,
              body: comment
            });
```

## Best Practices

### Security Considerations

- **Token security**: Use GitHub secrets for sensitive data
- **Permission scoping**: Limit workflow permissions to minimum required
- **Dependency scanning**: Scan dependencies for vulnerabilities
- **Artifact verification**: Verify artifact integrity before deployment
- **Audit logging**: Log all security-related actions

### Performance Optimization

- **Caching strategies**: Cache dependencies and build artifacts
- **Parallel execution**: Run jobs in parallel when possible
- **Conditional execution**: Skip unnecessary steps based on conditions
- **Resource optimization**: Use appropriate runner sizes
- **Artifact management**: Clean up old artifacts regularly

### Workflow Management

- **Modular workflows**: Break complex workflows into reusable components
- **Environment separation**: Use different environments for different stages
- **Error handling**: Implement proper error handling and notifications
- **Documentation**: Document workflow purposes and maintenance procedures
- **Version control**: Keep workflow files under version control

## Error Handling

### Common Issues

- **Permission errors**: Check workflow permissions
  ```yaml
  permissions:
    contents: read
    security-events: write
  ```

- **Token expiration**: Refresh GitHub tokens regularly
  ```bash
  # Use GITHUB_TOKEN or personal access tokens
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  ```

- **Rate limiting**: Implement retry logic for API calls
  ```yaml
  - name: API call with retry
    uses: nick-invision/retry@v2
    with:
      timeout_minutes: 10
      max_attempts: 3
      command: curl -X POST https://api.github.com/repos/.../issues
  ```

### Troubleshooting

1. **Check workflow logs**:
   ```bash
   # View workflow run logs in GitHub Actions UI
   ```

2. **Validate workflow syntax**:
   ```bash
   codeguardian ci-cd github-actions validate
   ```

3. **Test workflow locally**:
   ```bash
   # Use act for local testing
   act -j validate
   ```

4. **Monitor resource usage**:
   ```bash
   # Check runner resource usage in workflow logs
   ```

## Integration with CodeGuardian

### Automated Issue Creation

```yaml
# .github/workflows/codeguardian-issues.yml
name: CodeGuardian Issues
on:
  push:
    branches: [main]

jobs:
  create-issues:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Run CodeGuardian
        run: |
          codeguardian check --emit-gh --repo ${{ github.repository }}

      - name: Create GitHub issues
        run: |
          codeguardian gh-issue \
            --from results.json \
            --repo ${{ github.repository }} \
            --mode checklist \
            --title "CodeGuardian: Security Issues Found"
```

### Performance Regression Detection

```yaml
# .github/workflows/performance-regression.yml
name: Performance Regression
on:
  pull_request:
    branches: [main]

jobs:
  performance-check:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout base
        uses: actions/checkout@v4
        with:
          ref: ${{ github.base_ref }}

      - name: Benchmark base
        run: |
          codeguardian performance benchmark --output base-performance.json

      - name: Checkout PR
        uses: actions/checkout@v4
        with:
          ref: ${{ github.head_ref }}

      - name: Benchmark PR
        run: |
          codeguardian performance benchmark --output pr-performance.json

      - name: Compare performance
        run: |
          codeguardian performance compare \
            --baseline base-performance.json \
            --current pr-performance.json \
            --threshold 10 \
            --output comparison.md
```

## See Also

- [`codeguardian ci-cd gitlab-ci`](gitlab-ci.md) - GitLab CI/CD integration
- [`codeguardian ci-cd jenkins`](jenkins.md) - Jenkins pipeline integration
- [`codeguardian check`](../../../commands/check.md) - Primary analysis command
- [GitHub Actions Documentation](https://docs.github.com/en/actions)