# 📋 License Compliance Guide

This guide provides comprehensive information about CodeGuardian's license compliance features, including automated dependency license checking, policy configuration, and reporting mechanisms.

## Overview

CodeGuardian implements comprehensive license compliance checking to ensure all dependencies comply with organizational license policies and legal requirements. The system uses `cargo-deny` for Rust projects to scan dependencies and enforce license policies.

## Key Features

- **Automated License Scanning**: Comprehensive scanning of all Rust dependencies
- **Policy-Based Configuration**: Flexible license allow/deny lists
- **CI/CD Integration**: Seamless integration with GitHub Actions
- **Detailed Reporting**: Comprehensive reports with license distribution and compliance status
- **Violation Detection**: Automatic detection and blocking of non-compliant dependencies

## Configuration

### deny.toml Configuration

The license compliance policy is configured in `deny.toml` at the project root:

```toml
[licenses]
# The lint level for crates which do not have a detectable license
unlicensed = "deny"
# List of explicitly allowed licenses
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
    "CC0-1.0",
    "0BSD",
    "Zlib",
    "BSL-1.0",
    "MPL-2.0",
    "GPL-2.0-only",
    "GPL-2.0-or-later",
    "LGPL-2.1-only",
    "LGPL-2.1-or-later",
]
# List of explicitly disallowed licenses
deny = [
    "GPL-3.0-only",
    "GPL-3.0-or-later",
    "LGPL-3.0-only",
    "LGPL-3.0-or-later",
    "AGPL-3.0-only",
    "AGPL-3.0-or-later",
    "MS-PL",
    "JSON",
    "CDDL-1.0",
    "CDDL-1.1",
    "EPL-1.0",
    "EPL-2.0",
]
# Lint level for when no license is detected
no-license = "deny"
# Lint level for when a copyleft license is detected
copyleft = "warn"
# Confidence threshold for license detection
confidence-threshold = 0.8
```

### License Policy Categories

#### Allowed Licenses (Permissive)
- **MIT**: Most permissive open-source license
- **Apache-2.0**: Business-friendly with patent protection
- **BSD-2-Clause/BSD-3-Clause**: Simple and permissive
- **ISC**: Simplified BSD-style license
- **CC0-1.0**: Public domain dedication
- **0BSD**: BSD Zero Clause License

#### Conditionally Allowed (Copyleft with Warnings)
- **GPL-2.0/LGPL-2.1**: Copyleft licenses (allowed but flagged)
- **MPL-2.0**: Mozilla Public License (file-level copyleft)

#### Denied Licenses (Restrictive)
- **GPL-3.0+/LGPL-3.0+/AGPL**: Strong copyleft, incompatible with proprietary software
- **MS-PL**: Microsoft Public License
- **JSON**: JSON License (problematic)
- **CDDL/EPL**: Common Development and Distribution License

## CI/CD Integration

### Automated License Checking

License compliance checks are automatically run in CI/CD pipelines:

#### GitHub Actions Integration

```yaml
- name: License Compliance Check
  run: |
    cargo install cargo-deny --locked
    cargo deny check licenses

- name: Generate License Report
  run: |
    ./scripts/generate-license-report.sh

- name: Upload License Results
  uses: actions/upload-artifact@v4
  with:
    name: license-results-${{ github.run_id }}
    path: license-results.json
    retention-days: 30
```

### Workflow Integration Points

License checks are integrated into multiple workflows:

1. **Pull Request Checks**: Fast license validation on PRs
2. **Main Branch Protection**: Comprehensive checks before merge
3. **Release Validation**: Full compliance verification for releases
4. **Scheduled Audits**: Weekly comprehensive license audits

## Reporting and Monitoring

### License Reports

The CI/CD pipeline generates detailed license reports including:

- **License Distribution**: Count of dependencies by license type
- **Compliance Status**: Pass/fail status with detailed reasoning
- **Violation Details**: Specific issues and affected dependencies
- **Recommendations**: Suggested remediation steps

### Report Formats

Reports are available in multiple formats:

- **JSON**: Machine-readable format for automation
- **Markdown**: Human-readable format for documentation
- **GitHub Summary**: Inline workflow summaries
- **Artifacts**: Downloadable reports with 30-day retention

### Monitoring and Alerts

- **PR Comments**: Automatic comments on license violations
- **Workflow Status**: Clear pass/fail indicators
- **Artifact Storage**: Historical reports for auditing
- **Scheduled Reports**: Weekly compliance summaries

## Usage Examples

### Running License Checks Locally

```bash
# Install cargo-deny
cargo install cargo-deny

# Check license compliance
cargo deny check licenses

# Generate detailed report
cargo deny list

# Check all policies (licenses, advisories, bans)
cargo deny check
```

### Custom License Policies

Modify `deny.toml` to customize license policies:

```toml
[licenses]
# Allow additional licenses for specific use cases
allow = [
    "MIT",
    "Apache-2.0",
    # Add custom allowed licenses here
]

# Deny specific problematic licenses
deny = [
    "GPL-3.0-only",
    # Add custom denied licenses here
]
```

### Handling License Violations

When license violations are detected:

1. **Review the Report**: Check `license-results.json` for details
2. **Identify Issues**: Note which dependencies have problematic licenses
3. **Find Alternatives**: Look for dependencies with compatible licenses
4. **Update Configuration**: Modify `deny.toml` if policy needs adjustment
5. **Re-test**: Run checks again to verify compliance

## Best Practices

### License Policy Management

1. **Regular Reviews**: Review license policies quarterly
2. **Dependency Updates**: Keep dependencies updated for security and license compliance
3. **Alternative Evaluation**: Consider license compatibility when choosing dependencies
4. **Documentation**: Document license decisions and rationales

### CI/CD Optimization

1. **Caching**: Use Rust caching to speed up license checks
2. **Parallel Execution**: Run license checks in parallel with other CI jobs
3. **Artifact Management**: Configure appropriate retention for license reports
4. **Notification**: Set up alerts for license policy violations

### Compliance Automation

1. **Automated PR Checks**: Ensure all PRs pass license compliance
2. **Branch Protection**: Require license checks for main branch merges
3. **Release Gates**: Include license compliance in release processes
4. **Audit Trails**: Maintain historical license compliance records

## Troubleshooting

### Common Issues

#### "cargo-deny not found"
```bash
# Install cargo-deny
cargo install cargo-deny --locked

# Or use the fallback script
./scripts/license-check-fallback.sh
```

#### License detection failures
- Check dependency metadata
- Verify license declarations in Cargo.toml
- Consider manual license specification

#### False positives
- Review confidence thresholds in deny.toml
- Check license detection accuracy
- Update deny.toml configuration

### Getting Help

- **Documentation**: Check this guide and related docs
- **GitHub Issues**: Report bugs and request features
- **Community**: Join discussions for license compliance topics
- **Support**: Contact maintainers for urgent issues

## Integration with Other Tools

### Complementary Tools

- **cargo-audit**: Security vulnerability scanning
- **cargo-outdated**: Dependency update checking
- **cargo-license**: Alternative license checking
- **cargo-tree**: Dependency tree visualization

### Enterprise Integration

- **License Scanners**: Integrate with enterprise license management tools
- **Compliance Databases**: Connect with organizational compliance systems
- **Audit Systems**: Feed license data into audit and compliance platforms
- **Policy Engines**: Integrate with policy-as-code frameworks

## Future Enhancements

### Planned Features

- **License Attribution**: Automated NOTICE file generation
- **SPDX Integration**: Enhanced SPDX license identifier support
- **Custom Rules**: Organization-specific license policies
- **License Analytics**: Advanced license usage analytics
- **Integration APIs**: REST APIs for license compliance data

---

*This guide is maintained alongside the license compliance implementation. Please update it when making changes to license policies or workflows.*
