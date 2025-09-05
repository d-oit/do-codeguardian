# CI/CD Setup and Workflows Guide

This guide provides comprehensive documentation for the CodeGuardian CI/CD setup, including all workflows, their configurations, and best practices for team usage.

## Overview

## License Compliance

CodeGuardian implements comprehensive license compliance checking as part of its CI/CD pipeline to ensure all dependencies comply with organizational license policies and legal requirements.

### License Compliance Features

- **Automated License Scanning**: Uses `cargo-deny` to scan all Rust dependencies for license compliance
- **Policy-Based Configuration**: Configurable license allow/deny lists in `deny.toml`
- **Comprehensive Reporting**: Detailed reports showing license types, counts, and compliance status
- **CI/CD Integration**: License checks run on all PRs, pushes, and releases
- **Artifact Storage**: License reports stored as workflow artifacts for 30 days

### Supported License Policies

- **Allowed Licenses**: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-DFS-2016, CC0-1.0, 0BSD, Zlib, BSL-1.0, MPL-2.0
- **Copyleft Licenses**: GPL-2.0-only, GPL-2.0-or-later, LGPL-2.1-only, LGPL-2.1-or-later (allowed with warnings)
- **Denied Licenses**: GPL-3.0, LGPL-3.0, AGPL-3.0, MS-PL, JSON, CDDL, EPL
- **Unlicensed Dependencies**: Strictly denied

### License Check Integration

License compliance checks are integrated into multiple workflows:

- **Optimized CI**: Full license scanning with detailed reporting
- **CodeGuardian CI**: Basic license checks on PRs and pushes
- **Dependency Review**: GitHub-native license checking for additional coverage

### License Report Generation

The CI/CD pipeline generates comprehensive license reports including:
- Dependency license distribution
- Compliance status summary
- Violation details (if any)
- Recommendations for resolution

Reports are available as workflow artifacts and can be downloaded for compliance auditing.

CodeGuardian uses a sophisticated CI/CD pipeline with multiple specialized workflows that work together to ensure code quality, security, and performance. The system is designed for both speed and thoroughness, with different workflows handling different aspects of the development lifecycle.

## Workflow Architecture

### Core Workflows

#### 1. CodeGuardian CI (`do-codeguardian-ci.yml`)
**Purpose**: Main CI pipeline for code analysis and quality checks

**Triggers**:
- Pull requests to `main` and `develop` branches
- Pushes to `main` branch
- Weekly full scans (Mondays at 2 AM UTC)

**Jobs**:
- `do-codeguardian-pr`: Fast diff-only analysis for PRs
- `do-codeguardian-full`: Comprehensive analysis for main branch and scheduled runs
- `do-codeguardian-baseline`: Updates baseline for regression detection

**Key Features**:
- **PR Mode**: Analyzes only changed files for fast feedback
- **Full Mode**: Complete repository analysis with issue creation
- **Baseline Updates**: Automatic baseline maintenance for main branch

#### 2. Issue Triage (`issue-triage.yml`)
**Purpose**: Automated issue management and categorization

**Triggers**:
- Issues opened, edited, or reopened

**Features**:
- **Regex-based labeling**: Automatic label assignment based on content patterns
- **AI semantic labeling**: Uses GitHub Models for intelligent categorization
- **Duplicate detection**: Identifies and marks duplicate issues
- **Workflow summary**: Logs triage completion

#### 3. Turbo Nightly Analysis (`turbo-nightly.yml`)
**Purpose**: Comprehensive nightly security and code quality analysis

**Triggers**:
- Daily at 1 AM UTC
- Manual trigger via workflow dispatch

**Features**:
- **Aggressive analysis**: Deep security scanning with high parallelism
- **Performance metrics**: Detailed analysis timing and resource usage
- **Automated reporting**: Generates nightly analysis reports
- **Artifact retention**: 30-day retention for historical analysis

#### 4. Turbo Performance Monitor (`turbo-performance-monitor.yml`)
**Purpose**: Performance benchmarking and regression detection

**Triggers**:
- Daily at 6 AM UTC
- Manual trigger with benchmark type selection

**Benchmark Scenarios**:
- **Small codebase**: 50 files
- **Medium codebase**: 500 files
- **Large codebase**: 2000 files

**Features**:
- **Comparative analysis**: Standard vs Turbo mode performance comparison
- **Regression detection**: Automatic performance regression alerts
- **Matrix testing**: Multi-scenario performance validation
- **Speedup metrics**: Quantified performance improvements

#### 5. Turbo PR Analysis (`turbo-pr-analysis.yml`)
**Purpose**: Fast, focused security analysis for pull requests

**Triggers**:
- PR opened, synchronized, or reopened on `main`/`develop`

**Features**:
- **Changed files only**: Analyzes only modified files for speed
- **Real-time feedback**: Immediate PR comments with findings
- **Severity-based actions**: Critical issues block merges
- **Performance optimized**: Sub-5 second analysis times

#### 6. Turbo Release Validation (`turbo-release.yml`)
**Purpose**: Cross-platform validation for releases

**Triggers**:
- Release publication
- Manual validation with level selection

**Platforms**:
- Ubuntu Linux (x86_64)
- Windows (x86_64)
- macOS (x86_64)

**Validation Levels**:
- **Basic**: Core functionality tests
- **Comprehensive**: Full feature validation
- **Stress-test**: Performance under load

## Configuration

### Environment Variables

```yaml
# Required for GitHub integration
GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
GITHUB_REPOSITORY: ${{ github.repository }}

# Optional performance tuning
CARGO_TERM_COLOR: always
```

### Permissions

```yaml
permissions:
  contents: read          # Read repository contents
  issues: write          # Create/update issues
  pull-requests: write   # Comment on PRs
  actions: read          # Read workflow runs
```

### Caching Strategy

```yaml
- uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target/
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

## Workflow Integration

### PR Workflow Sequence

1. **PR Opened/Synchronized**
   - Turbo PR Analysis runs (fast feedback)
   - CodeGuardian CI runs (comprehensive analysis)
   - Issue Triage runs if issues are created

2. **PR Merged to Main**
   - Full CodeGuardian CI analysis
   - Baseline update if changes detected
   - Performance monitoring triggered

3. **Nightly Schedule**
   - Comprehensive analysis across all workflows
   - Performance benchmarking
   - Report generation and artifact storage

### Integration Points

- **GitHub Issues**: Automatic labeling and duplicate detection
- **GitHub PRs**: Real-time analysis comments and status checks
- **GitHub Releases**: Cross-platform validation
- **Artifacts**: JSON reports, markdown summaries, performance metrics

## Best Practices

### For Developers

#### Commit Message Conventions
```bash
# Good examples
feat: add turbo mode performance optimizations
fix: resolve memory leak in parallel processing
docs: update CI/CD workflow documentation
security: patch vulnerability in dependency scanner

# Avoid
update code
fix bug
```

#### PR Preparation
- Run `do-codeguardian turbo . --aggressive` locally before pushing
- Address critical and high-severity issues before requesting review
- Include performance impact assessment for significant changes

#### Branch Strategy
- Use `develop` for feature development
- Create feature branches from `develop`
- Merge to `main` only through approved PRs

### For Maintainers

#### Workflow Monitoring
- Monitor nightly analysis reports for trends
- Review performance regression alerts
- Check cross-platform compatibility on releases

#### Issue Management
- Use automated labels as starting point for triage
- Review AI-generated labels for accuracy
- Leverage duplicate detection to reduce noise

#### Performance Maintenance
- Monitor benchmark results for performance degradation
- Update performance thresholds as codebase grows
- Optimize workflows for CI resource usage

### Security Considerations

#### Token Management
- Use `GITHUB_TOKEN` for all GitHub API interactions
- Never expose sensitive tokens in workflow logs
- Rotate tokens regularly according to security policy

#### Permissions
- Apply principle of least privilege
- Use read-only permissions where possible
- Audit workflow permissions regularly

#### Artifact Security
- Set appropriate retention periods for artifacts
- Clean up sensitive data from logs
- Use encrypted storage for sensitive results

## Troubleshooting

### Common Issues

#### Workflow Failures
```bash
# Check workflow logs
gh workflow view <workflow-name> --logs

# Re-run failed jobs
gh workflow run <workflow-name> --ref <branch>
```

#### Performance Issues
- Check resource limits in workflow configuration
- Monitor memory usage in performance reports
- Adjust parallelism settings based on CI capacity

#### Integration Problems
- Verify GitHub token permissions
- Check webhook configurations
- Validate repository settings

### Debugging Tips

#### Local Testing
```bash
# Test turbo analysis locally
cargo build --release
./target/release/do-do-codeguardian turbo . --format json --output test.json

# Validate configuration
./target/release/do-do-codeguardian check --help
```

#### Log Analysis
- Use artifact downloads for detailed results
- Check GitHub Actions logs for error details
- Monitor performance metrics in reports

## Customization

### Adding New Workflows

1. Create workflow file in `.github/workflows/`
2. Follow naming convention: `<feature>-<purpose>.yml`
3. Include appropriate triggers and permissions
4. Add documentation to this guide

### Modifying Existing Workflows

1. Test changes on feature branch first
2. Update this documentation
3. Consider impact on other workflows
4. Monitor for regressions after deployment

### Performance Tuning

```yaml
# Adjust parallelism based on CI capacity
--max-parallel 8    # For larger CI runners
--max-parallel 4    # For standard runners
--max-parallel 2    # For resource-constrained environments

# Memory limits
--memory-limit 2048 # 2GB for large analyses
--memory-limit 512  # 512MB for standard runs
```

## Monitoring and Metrics

### Key Metrics to Monitor

- **Analysis Speed**: Time to complete scans
- **False Positive Rate**: Accuracy of findings
- **CI Resource Usage**: CPU, memory, and time consumption
- **Workflow Success Rate**: Percentage of successful runs

### Reporting

- **Daily Reports**: Nightly analysis summaries
- **Performance Reports**: Benchmark comparisons
- **Release Reports**: Cross-platform validation results
- **PR Reports**: Real-time feedback on changes

### Alerting

- Performance regression alerts
- Critical security finding notifications
- Workflow failure alerts
- Resource usage warnings

## Future Enhancements

### Planned Improvements

- **Advanced AI Triage**: Enhanced issue categorization
- **Predictive Analysis**: ML-based issue prediction
- **Custom Rules**: Team-specific analysis rules
- **Integration APIs**: Third-party tool integration

### Contributing

To contribute to CI/CD improvements:
1. Follow the established patterns
2. Test thoroughly before proposing changes
3. Update documentation
4. Consider backward compatibility

---

*This documentation is maintained alongside the workflows. Please update it when making changes to the CI/CD setup.*
