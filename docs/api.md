# API Reference

## Output Formats

### JSON Format (Primary)

The JSON format is the source of truth for all analysis results:

```json
{
  "metadata": {
    "version": "1.2.0",
    "timestamp": "2024-01-01T12:00:00Z",
    "files_analyzed": 42,
    "analysis_time_seconds": 15.3,
    "codeguardian_version": "1.2.0"
  },
  "findings": [
    {
      "id": "sha256_hash_of_finding",
      "file_path": "src/main.rs",
      "line_number": 25,
      "column": 12,
      "severity": "high",
      "category": "security",
      "analyzer": "security_analyzer",
      "title": "Hardcoded secret detected",
      "description": "Potential hardcoded API key found",
      "code_snippet": "let api_key = \"sk-123456789\";",
      "confidence": 0.95,
      "cwe": "CWE-798",
      "owasp": "A02:2021-Cryptographic Failures",
      "recommendation": "Use environment variables or secure key management",
      "references": [
        "https://owasp.org/www-community/vulnerabilities/Use_of_hardcoded_passwords"
      ],
      "tags": ["secrets", "hardcoded", "api_key"],
      "ml_confidence": 0.89,
      "false_positive": false
    }
  ],
  "summary": {
    "total_findings": 15,
    "critical": 2,
    "high": 5,
    "medium": 6,
    "low": 2,
    "files_with_findings": 8,
    "analysis_coverage": 95.2
  },
  "files": [
    {
      "path": "src/main.rs",
      "language": "rust",
      "size_bytes": 2048,
      "lines": 120,
      "findings_count": 3,
      "analysis_time_ms": 45
    }
  ],
  "performance": {
    "total_time_seconds": 15.3,
    "average_file_time_ms": 365,
    "memory_peak_mb": 256,
    "cache_hit_rate": 0.85,
    "parallel_efficiency": 0.92
  }
}
```

### Finding Object Schema

```typescript
interface Finding {
  id: string;                    // SHA-256 hash of finding for stable IDs
  file_path: string;             // Relative path to file
  line_number: number;           // Line number (1-based)
  column: number;                // Column number (1-based)
  severity: "critical" | "high" | "medium" | "low" | "info";
  category: "security" | "performance" | "code_quality" | "dependency" | "integrity" | "naming" | "non_production";
  analyzer: string;              // Analyzer that produced the finding
  title: string;                 // Short title
  description: string;           // Detailed description
  code_snippet: string;          // Code context
  confidence: number;            // Confidence score (0.0-1.0)
  cwe?: string;                  // CWE identifier
  owasp?: string;                // OWASP category
  recommendation: string;        // How to fix
  references: string[];          // External references
  tags: string[];                // Categorization tags
  ml_confidence?: number;        // ML confidence if applicable
  false_positive: boolean;       // ML false positive flag
  resolved?: boolean;            // Resolution status
  resolved_at?: string;          // ISO timestamp
  resolved_by?: string;          // GitHub username
}
```

### Metadata Schema

```typescript
interface Metadata {
  version: string;               // CodeGuardian version
  timestamp: string;             // ISO 8601 timestamp
  files_analyzed: number;        // Total files processed
  analysis_time_seconds: number; // Total analysis time
  codeguardian_version: string;  // Version string
  config_hash?: string;          // Configuration hash
  git_commit?: string;           // Git commit hash
  git_branch?: string;           // Git branch name
}
```

### Summary Schema

```typescript
interface Summary {
  total_findings: number;
  critical: number;
  high: number;
  medium: number;
  low: number;
  info: number;
  files_with_findings: number;
  analysis_coverage: number;     // Percentage of files analyzed
  new_findings?: number;         // Compared to baseline
  resolved_findings?: number;    // Compared to baseline
}
```

## SARIF Format

Security-focused SARIF output for integration with security tools:

```json
{
  "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
  "version": "2.1.0",
  "runs": [
    {
      "tool": {
        "driver": {
          "name": "CodeGuardian",
          "version": "1.2.0",
          "informationUri": "https://github.com/d-oit/do-codeguardian",
          "rules": [
            {
              "id": "CG001",
              "name": "hardcoded_secret",
              "shortDescription": {
                "text": "Hardcoded secret detected"
              },
              "fullDescription": {
                "text": "Potential hardcoded API key or secret found in source code"
              },
              "help": {
                "text": "Use environment variables or secure key management systems"
              },
              "properties": {
                "category": "security",
                "severity": "high",
                "cwe": "CWE-798"
              }
            }
          ]
        }
      },
      "results": [
        {
          "ruleId": "CG001",
          "level": "error",
          "message": {
            "text": "Hardcoded API key detected"
          },
          "locations": [
            {
              "physicalLocation": {
                "artifactLocation": {
                  "uri": "src/main.rs"
                },
                "region": {
                  "startLine": 25,
                  "startColumn": 12,
                  "endLine": 25,
                  "endColumn": 35
                }
              }
            }
          ],
          "properties": {
            "confidence": 0.95,
            "category": "security"
          }
        }
      ]
    }
  ]
}
```

## Report Command

Convert analysis results to various formats:

```bash
# Generate Markdown report
do-codeguardian report --from results.json --md report.md

# Generate HTML report
do-codeguardian report --from results.json --html report.html

# Generate both formats
do-codeguardian report --from results.json --md report.md --html report.html

# Custom template
do-codeguardian report --from results.json --template custom-template.md --out custom-report.md
```

### Report Options

- `--from FILE` - Input JSON results file
- `--md FILE` - Generate Markdown report
- `--html FILE` - Generate HTML report
- `--template FILE` - Use custom template
- `--out FILE` - Output file (for custom templates)
- `--format FORMAT` - Report format (markdown, html, custom)
- `--include-severity LEVEL` - Include only specified severity levels
- `--exclude-severity LEVEL` - Exclude specified severity levels
- `--include-category CATEGORY` - Include only specified categories
- `--exclude-category CATEGORY` - Exclude specified categories
- `--group-by CATEGORY` - Group findings by category
- `--sort-by FIELD` - Sort findings by field (severity, file, line)
- `--limit NUM` - Limit number of findings per report

## GitHub Issues API

### Issue Creation

Create GitHub issues from analysis results:

```bash
# Create issues with checklist format
do-codeguardian gh-issue --from results.json --repo owner/repo --mode checklist

# Create simple issues
do-codeguardian gh-issue --from results.json --repo owner/repo --mode simple

# Create parent-child issue structure
do-codeguardian gh-issue --from results.json --repo owner/repo --mode children
```

### Issue Modes

#### Checklist Mode
Creates interactive checklist issues with progress tracking:

```markdown
## Security Analysis Results

- [ ] Fix hardcoded secret in src/main.rs:25
- [ ] Address SQL injection vulnerability in src/db.rs:42
- [ ] Resolve memory leak in src/cache.rs:18

**Progress: 0/3 completed**
```

#### Simple Mode
Creates individual issues for each finding:

```markdown
### Hardcoded Secret
**File:** src/main.rs:25
**Severity:** High
**Description:** Potential hardcoded API key found

**Code:**
```rust
let api_key = "sk-123456789";
```

**Recommendation:** Use environment variables or secure key management
```

#### Children Mode
Creates a parent issue with child issues for each finding:

```
Parent Issue: Security Analysis Results (3 findings)
├── Child Issue: Hardcoded secret in src/main.rs
├── Child Issue: SQL injection in src/db.rs
└── Child Issue: Memory leak in src/cache.rs
```

### GitHub Issue Options

- `--from FILE` - Input JSON results file
- `--repo OWNER/REPO` - GitHub repository
- `--mode checklist|simple|children` - Issue format
- `--title-prefix PREFIX` - Custom title prefix
- `--labels LABELS` - Comma-separated labels
- `--assignees USERS` - Comma-separated assignees
- `--milestone MILESTONE` - Milestone name or number
- `--dry-run` - Preview without creating issues
- `--update-existing` - Update existing issues
- `--close-resolved` - Close resolved issues

### Issue Templates

Customize issue templates using variables:

```markdown
## {{severity|title}} Security Finding

**File:** {{file_path}}:{{line_number}}
**Category:** {{category}}
**Analyzer:** {{analyzer}}
**Confidence:** {{confidence}}%

### Description
{{description}}

### Code
```{{language}}
{{code_snippet}}
```

### Recommendation
{{recommendation}}

### References
{{references}}

---
*Generated by CodeGuardian {{version}} on {{timestamp}}*
```

## Metrics API

View ML model performance metrics:

```bash
# Show model metrics
do-codeguardian metrics --model enhanced-model.fann

# Show training metrics
do-codeguardian metrics --model enhanced-model.fann --training

# Export metrics to JSON
do-codeguardian metrics --model enhanced-model.fann --format json --out reports/metrics.json
```

### Metrics Output

```json
{
  "model_info": {
    "path": "enhanced-model.fann",
    "created": "2024-01-01T10:00:00Z",
    "last_trained": "2024-01-01T12:00:00Z",
    "training_samples": 10000,
    "features": 25,
    "hidden_layers": 2,
    "neurons_per_layer": 64
  },
  "performance": {
    "accuracy": 0.92,
    "precision": 0.89,
    "recall": 0.94,
    "f1_score": 0.91,
    "false_positive_rate": 0.08,
    "false_negative_rate": 0.06
  },
  "training_metrics": {
    "epochs": 2000,
    "final_error": 0.023,
    "training_time_seconds": 450,
    "cross_validation_score": 0.88
  },
  "usage_stats": {
    "total_predictions": 15420,
    "correct_predictions": 14180,
    "false_positives": 850,
    "false_negatives": 390,
    "online_learning_updates": 234
  }
}
```

## Configuration API

### Initialize Configuration

```bash
# Initialize with preset
do-codeguardian init --template security

# List available templates
do-codeguardian init --list

# Interactive configuration
do-codeguardian init --interactive

# Custom configuration file
do-codeguardian init --config custom.toml
```

### Validate Configuration

```bash
# Validate configuration
do-codeguardian validate --config do-codeguardian.toml

# Check environment
do-codeguardian doctor

# Show current config
do-codeguardian config --show
```

## Training API

Train ML models for better accuracy:

```bash
# Train with default settings
do-codeguardian train --model-path enhanced-model.fann

# Advanced training
do-codeguardian train \
  --model-path enhanced-model.fann \
  --epochs 2000 \
  --bootstrap

# Continue training existing model
do-codeguardian train --model-path existing-model.fann --continue
```

### Training Options

- `--model-path PATH` - Output model file path
- `--epochs NUM` - Number of training epochs
- `--bootstrap` - Use bootstrap sampling
- `--continue` - Continue training existing model
- `--cross-validation` - Enable cross-validation
- `--early-stopping` - Enable early stopping
- `--verbose` - Show training progress

## Cache API

Manage analysis cache and performance data:

```bash
# Show cache statistics
do-codeguardian cache --stats

# Clear cache
do-codeguardian cache --clear

# Set cache size limit
do-codeguardian cache --max-size 1024

# Export cache data
do-codeguardian cache --export cache.json

# Import cache data
do-codeguardian cache --import cache.json
```

## Export/Import API

Export and import analysis data and models:

```bash
# Export analysis results
do-codeguardian export --from results.json --format json --out export.json

# Export ML model
do-codeguardian export --model enhanced-model.fann --format fann --out model.fann

# Import analysis data
do-codeguardian import --from export.json --format json

# Import ML model
do-codeguardian import --model model.fann --format fann
```

## Programmatic Integration

### Using CodeGuardian as a Library

```rust
use do-codeguardian::{Config, CodeGuardian, AnalysisResults};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::load("do-codeguardian.toml").await?;

    // Initialize CodeGuardian
    let guardian = CodeGuardian::new(config).await?;

    // Run analysis
    let results = guardian.analyze_path(".").await?;

    // Process results
    for finding in &results.findings {
        println!("Found: {} in {}", finding.title, finding.file_path);
    }

    // Generate report
    let report = guardian.generate_report(&results, ReportFormat::Markdown).await?;
    std::fs::write("report.md", report)?;

    Ok(())
}
```

### Custom Analyzer Integration

```rust
use do-codeguardian::analyzers::{Analyzer, Finding, Severity};
use async_trait::async_trait;

pub struct CustomAnalyzer;

#[async_trait]
impl Analyzer for CustomAnalyzer {
    async fn analyze_file(
        &self,
        file_path: &std::path::Path,
        content: &str,
        config: &Config,
    ) -> Result<Vec<Finding>, Box<dyn std::error::Error>> {
        let mut findings = Vec::new();

        // Custom analysis logic here
        if content.contains("TODO") {
            findings.push(Finding {
                id: "custom_todo".to_string(),
                file_path: file_path.to_string_lossy().to_string(),
                line_number: 1,
                severity: Severity::Low,
                title: "TODO comment found".to_string(),
                description: "Remove TODO comments before production".to_string(),
                ..Default::default()
            });
        }

        Ok(findings)
    }

    fn name(&self) -> &str {
        "custom_analyzer"
    }
}
```

## Error Handling

CodeGuardian uses structured error handling with detailed context:

```rust
use do-codeguardian::error::CodeGuardianError;

match do-codeguardian.analyze_path(".").await {
    Ok(results) => {
        // Process results
    }
    Err(CodeGuardianError::ConfigError(msg)) => {
        eprintln!("Configuration error: {}", msg);
    }
    Err(CodeGuardianError::AnalysisError(msg)) => {
        eprintln!("Analysis error: {}", msg);
    }
    Err(CodeGuardianError::IoError(err)) => {
        eprintln!("I/O error: {}", err);
    }
    Err(err) => {
        eprintln!("Unknown error: {}", err);
    }
}
```

## Webhook Integration

CodeGuardian can send results to webhooks for integration with external systems:

```bash
# Send to webhook
do-codeguardian check . --webhook-url https://example.com/webhook --webhook-secret secret

# Custom webhook headers
do-codeguardian check . --webhook-url https://example.com/webhook --webhook-headers "X-API-Key: key"
```

### Webhook Payload

```json
{
  "event": "analysis_complete",
  "repository": "owner/repo",
  "commit": "abc123",
  "timestamp": "2024-01-01T12:00:00Z",
  "summary": {
    "total_findings": 15,
    "critical": 2,
    "high": 5
  },
  "results_url": "https://example.com/results.json"
}
```
