# 📊 Code Coverage Guide

## Overview

CodeGuardian uses comprehensive code coverage reporting to ensure high-quality, well-tested code. We use `cargo-tarpaulin` for coverage analysis with multiple output formats and quality gates.

## Coverage Targets

| Metric | Target | Minimum |
|--------|--------|---------|
| **Line Coverage** | 85% | 80% |
| **Branch Coverage** | 80% | 75% |
| **Function Coverage** | 90% | 85% |

## Quick Start

### 1. Install Coverage Tools

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Or use the provided script
./scripts/coverage.sh
```

### 2. Generate Coverage Report

```bash
# Basic coverage report
cargo tarpaulin --all-features --out Html

# Comprehensive coverage with all formats
cargo tarpaulin \
    --all-features \
    --workspace \
    --out Html,Xml,Json \
    --output-dir coverage/
```

### 3. View Results

- **HTML Report**: `coverage/tarpaulin-report.html`
- **XML Report**: `coverage/cobertura.xml` (for CI/CD)
- **JSON Report**: `coverage/tarpaulin-report.json` (for automation)

## Configuration

### Tarpaulin Configuration (`tarpaulin.toml`)

```toml
[tool.tarpaulin.coverage]
line = 85.0
branch = 80.0
function = 90.0

[tool.tarpaulin.report]
out = ["Html", "Xml", "Json", "Stdout"]
output-dir = "coverage/"

[tool.tarpaulin.run]
timeout = 120
exclude = [
    "target/*",
    "tests/fixtures/*",
    "benches/*",
    "examples/*",
    "tmp_*"
]
features = "full"
```

## CI/CD Integration

### GitHub Actions

Coverage is automatically generated on:
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop`

The workflow includes:
1. **Coverage Generation** - Creates HTML, XML, and JSON reports
2. **Quality Gate** - Enforces minimum coverage thresholds
3. **Artifact Upload** - Stores reports for download
4. **Codecov Integration** - Uploads to codecov.io for tracking

### Quality Gates

Coverage quality gates will **fail the build** if:
- Line coverage < 80%
- Branch coverage < 75%
- Function coverage < 85%

## Coverage Analysis

### Understanding Coverage Types

#### Line Coverage
- **What**: Percentage of code lines executed during tests
- **Target**: 85%
- **Focus**: Ensure all code paths are tested

#### Branch Coverage
- **What**: Percentage of conditional branches taken
- **Target**: 80%
- **Focus**: Test both true/false conditions

#### Function Coverage
- **What**: Percentage of functions called during tests
- **Target**: 90%
- **Focus**: Ensure all public APIs are tested

### Coverage Exclusions

Files excluded from coverage analysis:
- `target/*` - Build artifacts
- `tests/fixtures/*` - Test data files
- `benches/*` - Benchmark code
- `examples/*` - Example code
- `tmp_*` - Temporary files

### Module-Specific Guidelines

#### Core Modules (95%+ target)
- `src/security.rs` - Security analysis core
- `src/analyzers/security_analyzer.rs` - Security patterns
- `src/ml/enhanced_feature_extractor.rs` - ML features

#### Standard Modules (85%+ target)
- `src/analyzers/*.rs` - Analysis modules
- `src/cli/*.rs` - CLI commands
- `src/core.rs` - Core functionality

#### Utility Modules (75%+ target)
- `src/utils/*.rs` - Helper utilities
- `src/github_api.rs` - External API integration

## Improving Coverage

### 1. Identify Uncovered Code

```bash
# Generate detailed HTML report
cargo tarpaulin --all-features --out Html --output-dir coverage/

# Open the HTML report to see uncovered lines
open coverage/tarpaulin-report.html
```

### 2. Add Missing Tests

Focus on:
- **Error handling paths** - Test failure scenarios
- **Edge cases** - Boundary conditions and unusual inputs
- **Integration points** - Module interactions
- **Public APIs** - All exported functions

### 3. Property-Based Testing

Use property-based tests for comprehensive coverage:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_analyzer_robustness(input in ".*") {
        // Test with random inputs
        let result = analyzer.analyze(&input);
        prop_assert!(result.is_ok());
    }
}
```

## Coverage Reports

### Local Development

```bash
# Quick coverage check
cargo tarpaulin --lib --out Stdout

# Full coverage with HTML report
./scripts/coverage.sh
```

### CI/CD Pipeline

Coverage reports are automatically:
1. Generated on every PR
2. Uploaded to Codecov
3. Stored as GitHub artifacts
4. Used for quality gate enforcement

### Coverage Trends

Track coverage trends over time:
- **Codecov Dashboard** - Historical coverage data
- **GitHub Artifacts** - Per-build coverage reports
- **PR Comments** - Coverage diff on pull requests

## Troubleshooting

### Common Issues

#### Low Coverage in New Modules
```bash
# Check what's not covered
cargo tarpaulin --all-features --out Html
# Review HTML report for specific uncovered lines
```

#### Timeout Issues
```bash
# Increase timeout for complex tests
cargo tarpaulin --timeout 180
```

#### Feature-Specific Coverage
```bash
# Test specific features
cargo tarpaulin --features "ml,ast" --out Html
```

### Best Practices

1. **Write tests first** - TDD approach ensures coverage
2. **Test error paths** - Don't just test happy paths
3. **Use integration tests** - Test module interactions
4. **Mock external dependencies** - Focus on your code
5. **Regular coverage checks** - Don't let coverage drift

## Coverage Metrics Dashboard

### Current Status
- **Overall Coverage**: Target 85%
- **Security Modules**: Target 95%
- **ML Modules**: Target 90%
- **CLI Modules**: Target 85%

### Quality Gates
- ✅ **Minimum Line Coverage**: 80%
- ✅ **Minimum Branch Coverage**: 75%
- ✅ **Minimum Function Coverage**: 85%

---

## Quick Commands

```bash
# Install coverage tools
cargo install cargo-tarpaulin

# Generate coverage report
./scripts/coverage.sh

# Quick coverage check
cargo tarpaulin --lib --out Stdout

# Coverage with specific features
cargo tarpaulin --features "ml,ast" --out Html

# Coverage for specific module
cargo tarpaulin --lib --packages do-codeguardian --out Html
```

For more information, see the [GitHub Actions workflow](.github/workflows/coverage.yml) and [tarpaulin configuration](tarpaulin.toml).
