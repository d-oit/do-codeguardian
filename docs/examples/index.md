# 💡 Usage Examples

This directory contains practical examples and configuration templates to help you get the most out of CodeGuardian. Each example includes copy-paste ready code and detailed explanations.

## 🚀 Quick Start Examples

### Basic Security Analysis
```bash
# Quick security scan of current directory
codeguardian check .

# Analyze with JSON output
codeguardian check . --format json --out results.json

# Generate human-readable report
codeguardian report --from results.json --md security-report.md
```

### CI/CD Integration
```bash
# PR analysis with GitHub integration
codeguardian check . \
  --diff origin/main..HEAD \
  --format json \
  --out pr-results.json \
  --emit-gh \
  --repo owner/repo \
  --ml-model enhanced-model.fann
```

### High-Performance Analysis
```bash
# Turbo mode for large codebases
codeguardian turbo . \
  --max-parallel 16 \
  --memory-limit 2048 \
  --metrics \
  --format json \
  --output turbo-results.json
```

## 📁 Example Categories

### [🔧 Configuration Templates](templates.md)
- Security-focused configurations
- Performance-optimized settings
- CI/CD pipeline configurations
- Enterprise security templates

### [🔗 CI/CD Examples](ci-cd-examples.md)
- GitHub Actions workflows
- GitLab CI/CD pipelines
- Jenkins pipeline configurations
- Azure DevOps integration

### [🛡️ Security Examples](security-examples.md)
- Custom security rules
- Vulnerability scanning patterns
- Secret detection configurations
- Compliance-focused analysis

### [⚡ Performance Examples](performance-examples.md)
- Large codebase optimization
- Memory usage optimization
- Parallel processing configuration
- Caching strategies

### [🔍 Advanced Analysis](advanced-analysis.md)
- ML model training and usage
- Custom analyzer development
- Plugin integration
- Advanced reporting

## 🎯 Common Use Cases

### 1. Development Workflow
```bash
# Pre-commit analysis
codeguardian check . --format json --out pre-commit-results.json

# Quick security check
codeguardian check . --analyzer security --fail-on-issues

# Full analysis with ML
codeguardian check . --ml-model enhanced-model.fann --ml-threshold 0.8
```

### 2. Pull Request Analysis
```bash
# Analyze only changed files
codeguardian check . --diff origin/main..HEAD --format json --out pr-analysis.json

# Create GitHub issues for findings
codeguardian gh-issue --from pr-analysis.json --repo owner/repo --mode checklist

# Generate PR comment
codeguardian report --from pr-analysis.json --md pr-comment.md
```

### 3. Security Audit
```bash
# Comprehensive security audit
codeguardian check . \
  --config security-audit.toml \
  --format json \
  --out security-audit.json \
  --emit-md security-report.md \
  --emit-gh \
  --repo owner/repo \
  --fail-on-issues
```

### 4. Performance Optimization
```bash
# Performance analysis
codeguardian turbo . \
  --metrics \
  --format json \
  --output performance-analysis.json

# Memory usage analysis
codeguardian check . \
  --memory-limit 1024 \
  --streaming-threshold 5 \
  --format json \
  --out memory-analysis.json
```

### 5. Enterprise Integration
```bash
# Enterprise-scale analysis
codeguardian turbo . \
  --max-parallel 32 \
  --memory-limit 8192 \
  --streaming-threshold 10 \
  --cache-enabled \
  --format json \
  --output enterprise-analysis.json

# Custom reporting
codeguardian report \
  --from enterprise-analysis.json \
  --template enterprise-template.md \
  --md enterprise-report.md \
  --html enterprise-report.html
```

## 📋 Example Files

### Configuration Examples

#### `codeguardian.security.toml`
```toml
[general]
max_file_size = 10485760
parallel_workers = 4
memory_limit_mb = 1024

[analyzers.security]
enabled = true
check_secrets = true
check_vulnerabilities = true
check_hardcoded_secrets = true
entropy_threshold = 4.5

[ml]
enabled = true
model_path = "enhanced-model.fann"
confidence_threshold = 0.8
```

#### `codeguardian.ci.toml`
```toml
[general]
max_file_size = 5242880
parallel_workers = 2
memory_limit_mb = 512
tty_output = false

[performance]
cache_enabled = true
parallel_processing = true

[output]
default_format = "json"
color_output = false
```

### Script Examples

#### `analyze-pr.sh`
```bash
#!/bin/bash
# Analyze pull request changes

if [ -z "$GITHUB_TOKEN" ]; then
    echo "Error: GITHUB_TOKEN not set"
    exit 1
fi

# Analyze changed files
codeguardian check . \
  --diff origin/main..HEAD \
  --format json \
  --out pr-results.json \
  --emit-gh \
  --repo "$GITHUB_REPOSITORY" \
  --ml-model enhanced-model.fann

# Check for critical issues
critical_count=$(jq '.summary.critical' pr-results.json)
if [ "$critical_count" -gt 0 ]; then
    echo "Critical security issues found: $critical_count"
    exit 1
fi
```

#### `security-audit.sh`
```bash
#!/bin/bash
# Comprehensive security audit

echo "Starting security audit..."

# Run analysis
codeguardian check . \
  --config security-audit.toml \
  --format json \
  --out security-audit.json \
  --emit-md security-report.md

# Generate compliance report
codeguardian report \
  --from security-audit.json \
  --template compliance-template.md \
  --md compliance-report.md

# Check compliance thresholds
high_count=$(jq '.summary.high' security-audit.json)
if [ "$high_count" -gt 10 ]; then
    echo "Warning: High number of high-severity issues: $high_count"
fi

echo "Security audit completed. Reports generated."
```

### Docker Examples

#### `Dockerfile`
```dockerfile
FROM codeguardian/codeguardian:latest

# Copy custom configuration
COPY codeguardian.toml /etc/codeguardian.toml

# Copy custom rules
COPY custom-rules.json /etc/codeguardian/rules.json

# Copy ML model
COPY enhanced-model.fann /etc/codeguardian/model.fann

# Set environment
ENV CODEGUARDIAN_CONFIG=/etc/codeguardian.toml
ENV CODEGUARDIAN_ML_MODEL=/etc/codeguardian/model.fann

# Run analysis
CMD ["check", "/workspace", "--format", "json", "--out", "/workspace/results.json"]
```

#### `docker-compose.yml`
```yaml
version: '3.8'
services:
  codeguardian:
    image: codeguardian/codeguardian:latest
    volumes:
      - .:/workspace
      - ./codeguardian.toml:/etc/codeguardian.toml
    environment:
      - CODEGUARDIAN_MEMORY_LIMIT_MB=2048
      - GITHUB_TOKEN=${GITHUB_TOKEN}
    command: ["check", "/workspace", "--format", "json", "--out", "/workspace/results.json"]
```

## 🎨 Template System

### Custom Report Templates

#### `security-report-template.md`
```markdown
# Security Analysis Report

**Generated:** {{ timestamp }}
**Files Analyzed:** {{ files_analyzed }}
**Total Findings:** {{ summary.total_findings }}

## Executive Summary

- Critical Issues: {{ summary.critical }}
- High Issues: {{ summary.high }}
- Medium Issues: {{ summary.medium }}
- Low Issues: {{ summary.low }}

## Findings by Category

{% for category, findings in findings_by_category.items() %}
### {{ category|title }} Issues ({{ findings|length }})

{% for finding in findings %}
#### {{ finding.title }}
- **Severity:** {{ finding.severity|title }}
- **File:** {{ finding.file_path }}:{{ finding.line_number }}
- **Description:** {{ finding.description }}
- **Recommendation:** {{ finding.recommendation }}
{% endfor %}
{% endfor %}

## Compliance Status

- **Security Score:** {{ security_score }}%
- **Compliance Level:** {{ compliance_level }}
- **Risk Assessment:** {{ risk_assessment }}
```

### GitHub Issue Templates

#### `github-issue-template.md`
```markdown
## Security Finding: {{ title }}

**Severity:** {{ severity|upper }}
**File:** {{ file_path }}:{{ finding.line_number }}
**Category:** {{ category }}

### Description
{{ description }}

### Code Snippet
```{{ language }}
{{ code_snippet }}
```

### Recommendation
{{ recommendation }}

### Additional Information
- **Confidence:** {{ confidence }}%
- **CWE ID:** {{ cwe_id }}
- **Rule ID:** {{ rule_id }}

### References
{% for ref in references %}
- {{ ref }}
{% endfor %}

---
*This issue was automatically created by CodeGuardian*
```

## 🔧 Integration Examples

### GitHub Actions
```yaml
name: Security Analysis
on: [push, pull_request]

jobs:
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          fetch-depth: 0

      - name: Run CodeGuardian
        uses: d-oit/codeguardian-action@v1
        with:
          args: |
            check . \
              --diff origin/main..HEAD \
              --format json \
              --out results.json \
              --emit-gh \
              --repo ${{ github.repository }} \
              --ml-model enhanced-model.fann
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### GitLab CI
```yaml
stages:
  - security

codeguardian:
  stage: security
  image: codeguardian/codeguardian:latest
  script:
    - |
      if [ -n "$CI_MERGE_REQUEST_TARGET_BRANCH_NAME" ]; then
        codeguardian check . \
          --diff origin/$CI_MERGE_REQUEST_TARGET_BRANCH_NAME..HEAD \
          --format json \
          --out results.json \
          --emit-md report.md
      else
        codeguardian check . \
          --format json \
          --out results.json \
          --emit-md report.md
      fi
  artifacts:
    paths:
      - results.json
      - report.md
```

### Jenkins Pipeline
```groovy
pipeline {
    agent {
        docker {
            image 'codeguardian/codeguardian:latest'
            args '-v $WORKSPACE:/workspace -w /workspace'
        }
    }
    stages {
        stage('Security Analysis') {
            steps {
                sh '''
                    codeguardian check . \
                      --format json \
                      --out results.json \
                      --emit-md report.md \
                      --max-parallel 4
                '''
            }
        }
    }
    post {
        always {
            archiveArtifacts artifacts: 'results.json,report.md', fingerprint: true
        }
    }
}
```

## 📊 Advanced Examples

### Custom Analyzer Development
```rust
use codeguardian::analyzers::{Analyzer, Finding, FindingSeverity};
use codeguardian::types::FileInfo;

pub struct CustomSecurityAnalyzer;

impl Analyzer for CustomSecurityAnalyzer {
    fn name(&self) -> &'static str {
        "custom-security"
    }

    fn analyze(&self, file: &FileInfo, content: &str) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Custom analysis logic
        if content.contains("unsafe") {
            findings.push(Finding {
                id: "custom-unsafe-usage".to_string(),
                title: "Unsafe code usage detected".to_string(),
                description: "Found usage of unsafe code".to_string(),
                severity: FindingSeverity::Medium,
                file_path: file.path.clone(),
                line_number: 1,
                code_snippet: content.lines().next().unwrap_or("").to_string(),
                recommendation: "Review unsafe code usage for security implications".to_string(),
                category: "security".to_string(),
                confidence: 0.9,
                cwe_id: Some("CWE-242".to_string()),
                references: vec!["https://cwe.mitre.org/data/definitions/242.html".to_string()],
            });
        }

        findings
    }

    fn supported_file_types(&self) -> Vec<&'static str> {
        vec!["rs"]
    }
}
```

### ML Model Training
```bash
# Prepare training data
codeguardian train \
  --model-path custom-model.fann \
  --epochs 5000 \
  --learning-rate 0.001 \
  --bootstrap \
  --validation-split 0.2

# Use trained model
codeguardian check . \
  --ml-model custom-model.fann \
  --ml-threshold 0.8 \
  --online-learning
```

### Performance Benchmarking
```bash
# Run performance benchmarks
codeguardian turbo . \
  --metrics \
  --format json \
  --output benchmark-results.json \
  --max-parallel 16

# Compare with baseline
codeguardian benchmark compare \
  --baseline baseline-results.json \
  --current benchmark-results.json \
  --output comparison-report.md
```

These examples should help you get started with CodeGuardian and adapt it to your specific needs. Each example is designed to be practical and can be used as-is or modified for your requirements.

---

<div align="center">

**[⬅️ Back to Documentation](../README.md)** | **[🔧 Configuration Templates](templates.md)** | **[🔗 CI/CD Examples](ci-cd-examples.md)**

</div>
