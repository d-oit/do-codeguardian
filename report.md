# CodeGuardian Analysis Report

**Generated:** 2025-08-22 09:50:15 UTC
**Tool Version:** codeguardian v0.1.0
**Schema Version:** 1.0.0
**Config Hash:** `d359ab53`

## 📊 Summary

- **Files Scanned:** 97
- **Total Findings:** 3003
- **Scan Duration:** 3355ms

### Findings by Severity

| Severity | Count | Emoji |
|----------|-------|-------|
| critical | 19 | 🔴 |
| high | 261 | 🟠 |
| medium | 1236 | 🟡 |
| low | 1453 | 🔵 |
| info | 34 | ℹ️ |

### Findings by Analyzer

| Analyzer | Count |
|----------|-------|
| code_quality | 1259 |
| non_production | 538 |
| optimized-dependency | 347 |
| optimized-performance | 42 |
| optimized-quality | 422 |
| optimized-security | 15 |
| performance | 319 |
| security | 61 |

## 🔍 Detailed Findings

### 🔴 critical Issues

#### Potential hardcoded secret detected

- **ID:** `48ad9e925818092d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 19
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `bdc78e2a5d3d9ad7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 20
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `008d42ef21d5a090`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 21
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `be6a5032fd7a792e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 148
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `e3062782e85f7d59`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 104
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `a6fb5a7cbebf4223`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 34
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `397f78f65649e6f0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 20
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `fab2db3bc45bc5e7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 21
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `27ece490f7479a03`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 22
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `1a863e761fef74fa`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 149
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `45cee81b187d127f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 424
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `50b57079dbef6210`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 170
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `7a949e5f1f9b891b`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 152
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `2759ef46050e15cb`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 153
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `857b353da5edd062`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 154
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `3d9f175ba667b8e7`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 213
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `0a63d9f2979bee95`
- **File:** `/workspaces/do-codeguardian/src/utils/security.rs`
- **Line:** 8
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `7bd88dd798366957`
- **File:** `/workspaces/do-codeguardian/src/utils/security.rs`
- **Line:** 10
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Potential hardcoded secret detected

- **ID:** `0eb411b5cd9324f0`
- **File:** `/workspaces/do-codeguardian/src/utils/security.rs`
- **Line:** 12
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Line may contain hardcoded credentials or API keys
- **Suggestion:** Move secrets to environment variables or secure configuration

### 🟠 high Issues

#### High entropy string detected (entropy: 4.00)

- **ID:** `3b9140cd2f1e9268`
- **File:** `/workspaces/do-codeguardian/Cargo.lock`
- **Line:** 81
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### High entropy string detected (entropy: 4.02)

- **ID:** `086ee8cf33b05433`
- **File:** `/workspaces/do-codeguardian/Cargo.lock`
- **Line:** 654
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### High entropy string detected (entropy: 4.02)

- **ID:** `7bee43c8b8540240`
- **File:** `/workspaces/do-codeguardian/Cargo.lock`
- **Line:** 751
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### High entropy string detected (entropy: 4.01)

- **ID:** `d5531330297dc7dd`
- **File:** `/workspaces/do-codeguardian/Cargo.lock`
- **Line:** 1151
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### BUG comment found

- **ID:** `ffeff69612607579`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 32
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `071d771c118f56f3`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 59
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Dangerous function 'system' detected

- **ID:** `be041c45f173a8ad`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 70
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### BUG comment found

- **ID:** `3d83d6dfa5b92405`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 130
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `43b99ef6d0f26839`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 134
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Potential secret detected:     let api_key = "sk-1234567890abcdef"; // Potent

- **ID:** `7cfe31fdc15d5edb`
- **File:** `/workspaces/do-codeguardian/performance_benchmark.sh`
- **Line:** 57
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### Potential secret detected:     let password = "hardcoded_password"; // Anothe

- **ID:** `40e8201125d47647`
- **File:** `/workspaces/do-codeguardian/performance_benchmark.sh`
- **Line:** 58
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### Function 'analyze_line' has high cyclomatic complexity (16)

- **ID:** `42ba9840c5d0899a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 88
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'analyze_file_structure' has high cyclomatic complexity (11)

- **ID:** `2e370c9aea27c393`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 206
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' has high cyclomatic complexity (13)

- **ID:** `ff53f11adf960d03`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 263
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'analyze_function_complexity' has high cyclomatic complexity (13)

- **ID:** `952c001d43f3f723`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 266
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'for' has high cyclomatic complexity (13)

- **ID:** `97b853e66cadda86`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 272
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' has high cyclomatic complexity (17)

- **ID:** `45babe42cf42c2e6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 386
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_import_organization' has high cyclomatic complexity (17)

- **ID:** `632e58f75827ce10`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 389
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (17)

- **ID:** `ac7f667574140069`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 392
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' has high cyclomatic complexity (11)

- **ID:** `36c3097b7a8a8915`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 463
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'is_commented_code' has high cyclomatic complexity (83)

- **ID:** `fd9580c22cfbc415`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 480
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'extract_function_body' has high cyclomatic complexity (11)

- **ID:** `18b7248f41c1b684`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 533
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `1d2256014593d431`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 676
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Dangerous function 'system' detected

- **ID:** `518c6f4983f602c0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 751
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Function 'check_naming_for_language' has high cyclomatic complexity (19)

- **ID:** `13a8dc43b49b0298`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 783
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `ccd0d603037cfc7c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 27
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'Ok' has high cyclomatic complexity (28)

- **ID:** `ec86709fd636f5c0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 109
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_dependencies' has high cyclomatic complexity (28)

- **ID:** `2f0f31eee6cfb033`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 112
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'for' has high cyclomatic complexity (28)

- **ID:** `a7990033635b304b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 120
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (17)

- **ID:** `2783e21a1d278f9a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 151
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' has high cyclomatic complexity (14)

- **ID:** `f3f937ee43b6d18d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 194
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_rust_dependencies' has high cyclomatic complexity (14)

- **ID:** `ba63a33349f2dc79`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 197
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'for' has high cyclomatic complexity (14)

- **ID:** `89da9524c014bfe2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 204
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (11)

- **ID:** `e294dc5f9920e424`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 206
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' has high cyclomatic complexity (15)

- **ID:** `97031d4791449a04`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 101
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_json_formatting' has high cyclomatic complexity (15)

- **ID:** `bb926473b582aabe`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 104
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `1710009560e3d613`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 9
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `947b21c1109cb941`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 30
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a BUG comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `04be642f93ae65a1`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 31
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Debug statement found

- **ID:** `5845be6b9a17e8ab`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 31
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### BUG comment found

- **ID:** `735000fff99ff05c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 34
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `26c4a2c23df90475`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 39
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `4fd1018849c54a0d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 53
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'check_non_production_code' has high cyclomatic complexity (19)

- **ID:** `cb3c29e63a60c023`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 58
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'for' has high cyclomatic complexity (19)

- **ID:** `4c71b8a24f0071a0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 62
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `c1f23c985149aa98`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 69
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `90cf0d6b5b41d367`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 98
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `7e434908adeda166`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 99
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `d389eb7889223625`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 100
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Debug statement found

- **ID:** `44a10106998e50ed`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 100
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### BUG comment found

- **ID:** `612a244287292eb3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 109
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `2648b8293d53d32b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 113
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `42b9cfe7ac5dad73`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 116
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `7bed3a0aeedd9b19`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 119
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'contains_potential_secret' has high cyclomatic complexity (13)

- **ID:** `00de1a0b87bdd53c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 201
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'analyze_secret_context' has high cyclomatic complexity (63)

- **ID:** `ccce492ea2934f3e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 238
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### XXX comment found

- **ID:** `94c8ee22387fe00c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 317
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a xxx comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'analyze_security_optimized' has high cyclomatic complexity (24)

- **ID:** `36cf9b8313cc2e1f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 40
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (14)

- **ID:** `722191ebe3fa1023`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 70
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (12)

- **ID:** `a542cb8597a8db79`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 78
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'analyze_performance_optimized' has high cyclomatic complexity (22)

- **ID:** `a5bac9785a20b6ac`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 139
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (20)

- **ID:** `a664c0230e9ae7cf`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 157
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (18)

- **ID:** `1d5c070af3ad0eae`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 165
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (16)

- **ID:** `9f4f4027667ef90d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 173
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (14)

- **ID:** `d9cf48bf4720c436`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 181
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (12)

- **ID:** `a8bc0d5b7f0b5f45`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 189
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'analyze_quality_optimized' has high cyclomatic complexity (16)

- **ID:** `3c8db8a49d4470c8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 216
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (18)

- **ID:** `cb63b55dc30ca002`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 241
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (16)

- **ID:** `aa93876a1eca2311`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 252
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (14)

- **ID:** `0d7b3c0beb752bab`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 262
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (12)

- **ID:** `a21f419449da03b7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 270
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'test_complexity_calculation' has high cyclomatic complexity (17)

- **ID:** `2bb62d20242f66aa`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 445
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Potential XSS vulnerability

- **ID:** `775e4b7a72e569c5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 42
- **Analyzer:** security
- **Rule:** xss_vulnerability
- **Description:** XSS vulnerabilities can allow attackers to execute malicious scripts
- **Suggestion:** Sanitize user input and use safe DOM manipulation methods

#### Potential XSS vulnerability

- **ID:** `8ab8cc6df7010b73`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 42
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### Dangerous function 'exec' detected

- **ID:** `6c0167172070e312`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 46
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `8e33dba790f5ef4d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 46
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'shell_exec' detected

- **ID:** `dc3072f2842083c6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 46
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### BUG comment found

- **ID:** `2ee68ecc630898c3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 261
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'analyze_performance_issues' has high cyclomatic complexity (31)

- **ID:** `bc2d3c0d91244ab9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 62
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'for' has high cyclomatic complexity (29)

- **ID:** `bd3b4f63a928f944`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 67
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' has high cyclomatic complexity (18)

- **ID:** `7be735cad9879349`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 193
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'detect_nested_loops' has high cyclomatic complexity (18)

- **ID:** `947ea3ad70c36ebb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 196
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'is_in_loop_context' has high cyclomatic complexity (13)

- **ID:** `ec38e386b800d161`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 221
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_rust_performance' has high cyclomatic complexity (16)

- **ID:** `b21fa68809c7e5bc`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 287
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_javascript_performance' has high cyclomatic complexity (23)

- **ID:** `da9c8f4722359485`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 359
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_python_performance' has high cyclomatic complexity (16)

- **ID:** `21ff56a25dc78963`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 429
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_java_performance' has high cyclomatic complexity (11)

- **ID:** `92dbf569fb430b7c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 479
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_file_size_issues' has high cyclomatic complexity (13)

- **ID:** `3fddaeb85c7db5b1`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 527
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `f16ec50b78f332c8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 10
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Dangerous function 'eval' detected

- **ID:** `f7930e438fcb9b93`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 46
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'exec' detected

- **ID:** `4ab7c10a1ea8486a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 47
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `bad17369fb737f7f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 48
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'exec' detected

- **ID:** `175bd6849b6fe4ca`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 49
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'shell_exec' detected

- **ID:** `394965a70333810c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 49
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'passthru' detected

- **ID:** `81db410df7ec7208`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 50
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'popen' detected

- **ID:** `36d478264c0ad0f7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 51
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'proc_open' detected

- **ID:** `b8a07fd0b363247f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 52
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Function 'analyze_line_security' has high cyclomatic complexity (20)

- **ID:** `cc60f934db6e8af1`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 135
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Dangerous function 'exec' detected

- **ID:** `da0973e3d162ea6b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 227
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'exec' detected

- **ID:** `50c259da61c8a7ff`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 251
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `57210024343c899d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 251
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `e91802ad874dbdf6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 255
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Function 'analyze_secret_context' has high cyclomatic complexity (63)

- **ID:** `b8ff6ff3b9c3e2f1`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 315
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### XXX comment found

- **ID:** `e574a2058cece490`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 399
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a xxx comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Potential SQL injection vulnerability

- **ID:** `138a67f0e50d2c66`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 548
- **Analyzer:** security
- **Rule:** sql_injection
- **Description:** SQL injection vulnerabilities can allow attackers to access or modify data
- **Suggestion:** Use parameterized queries or prepared statements

#### Potential SQL injection vulnerability

- **ID:** `b7e24a714681c9d4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 548
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### Potential secret detected:         let code = r#"const apiKey = "sk-123456789

- **ID:** `0cdb2d08998d4042`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 559
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### Potential XSS vulnerability

- **ID:** `649d151e9f360a8f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 570
- **Analyzer:** security
- **Rule:** xss_vulnerability
- **Description:** XSS vulnerabilities can allow attackers to execute malicious scripts
- **Suggestion:** Sanitize user input and use safe DOM manipulation methods

#### Potential XSS vulnerability

- **ID:** `5a438f837d27e840`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 570
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### Potential SQL injection vulnerability

- **ID:** `45eab7525fea58fd`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs.new`
- **Line:** 242
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### Potential secret detected:         let code = r#"const apiKey = "sk-123456789

- **ID:** `a162ea35a7a3dda3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs.new`
- **Line:** 251
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### Potential XSS vulnerability

- **ID:** `839a2c072bb99a68`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs.new`
- **Line:** 260
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### Dangerous function 'eval' detected

- **ID:** `1d4fc26f8dd6f402`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 38
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'eval' detected

- **ID:** `ea24bfb45fb53425`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 41
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'eval' detected

- **ID:** `3bd4fc52afbc2192`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 45
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### BUG comment found

- **ID:** `c1d29ed209ceef53`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 12
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `6682ee24ccc44ff2`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 23
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'Ok' has high cyclomatic complexity (14)

- **ID:** `0494395202a75298`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 120
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' has high cyclomatic complexity (12)

- **ID:** `19ee3cda10943b43`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 144
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `a4c586bd75e9bd33`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 431
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `d0b730e182614e1f`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 438
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'Ok' has high cyclomatic complexity (18)

- **ID:** `3dec89146cef6585`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 67
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' has high cyclomatic complexity (17)

- **ID:** `4ed2671ce540b345`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 69
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (16)

- **ID:** `791abb3a37e56ae6`
- **File:** `/workspaces/do-codeguardian/src/cli/report.rs`
- **Line:** 130
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Dangerous function 'system' detected

- **ID:** `1c977face96f7f3e`
- **File:** `/workspaces/do-codeguardian/src/cli/report.rs`
- **Line:** 188
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Function 'create_default_network' has high cyclomatic complexity (11)

- **ID:** `d9bdedd62a4692be`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 80
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'for' has high cyclomatic complexity (15)

- **ID:** `f84b16d1d173299e`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 200
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Dangerous function 'eval' detected

- **ID:** `6ab69d6610ba68b3`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 230
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'exec' detected

- **ID:** `3a6d6e71bf5be5ad`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 230
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Function 'for' has high cyclomatic complexity (15)

- **ID:** `a2b611f6d39eab50`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 269
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'for' has high cyclomatic complexity (18)

- **ID:** `6f64996e2672e212`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 326
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `1f143b0c7e438820`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 257
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `988c25425b70125d`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 267
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `d060df0a402b1abd`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 277
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `9ce5186bbdd22ec4`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 5
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'performance_characteristics' has high cyclomatic complexity (12)

- **ID:** `42f7bf3f050777dc`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 55
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `72b8ad691d9c8072`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 109
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `fbcf3e83f00b0e20`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 117
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `db3cd65bf08426f3`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 124
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `96894bc45d5d09b4`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 131
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `12dd81c8110ec082`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 138
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Dangerous function 'system' detected

- **ID:** `4d8ac053748b0580`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 142
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `a50bebf209c007b2`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 148
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### BUG comment found

- **ID:** `a28ab78461d69ff1`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 152
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `66c3d6fbb0644d19`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 161
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Dangerous function 'system' detected

- **ID:** `5813b65cb00c6e1e`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 173
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `d7cc1933faab4b62`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 181
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `8499eb69f8ac1fd2`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 186
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `32c83ffeea677c3c`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 191
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Function 'get_recommendations' has high cyclomatic complexity (12)

- **ID:** `389f3de4cd92cdf8`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 232
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Dangerous function 'system' detected

- **ID:** `9cb1f7ccbecffb74`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 245
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `ad9add6b03745b80`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 249
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `169b8f16a03ced93`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 253
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `06751bc2062f4819`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 260
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### BUG comment found

- **ID:** `2f616c9a33a3af2d`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 4
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `28d2bf34488b2607`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 34
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `1287db0b3c99e27f`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 49
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'adaptive' has high cyclomatic complexity (11)

- **ID:** `02abd0c4f48db9d3`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 179
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Dangerous function 'system' detected

- **ID:** `a4dcc37e731fb58c`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 180
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `66c90978ae229134`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 281
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### BUG comment found

- **ID:** `3d94d67a3d75ff43`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 9
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `1601783a535f0527`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 38
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `99d4f1a2878c9951`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 45
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `0d1328a74a5a07d2`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 66
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `8a4e16375e58c54f`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 92
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `0f01e9677a67b9c8`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 99
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `29a3a94978cd94ce`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 118
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `64a93c4a6c78d4ba`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 119
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `b62937557dfe7eae`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 134
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `8b1ce4227eb8f664`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 161
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `69c2b2ff382fc732`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 182
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `c77be78266b65bca`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 214
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `2f496388e5ba2f5c`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 243
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `74f89dfbe9bb54ae`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 272
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `260d620d42b6a3f5`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 304
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `6d5e0fce7fb23c31`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 445
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `fb71e9db8f3ed814`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 9
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `a6e176d043860aed`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 77
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `803b971ce96bc593`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 12
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `adf22496aa8d1f15`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 20
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Dangerous function 'exec' detected

- **ID:** `5a9436ab3cf42c0e`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 45
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Function 'execute_gh_command' has high cyclomatic complexity (11)

- **ID:** `5d58b9b28e0f6fa8`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 45
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'sleep' has high cyclomatic complexity (18)

- **ID:** `db38e791d61b2195`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 76
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Err' has high cyclomatic complexity (17)

- **ID:** `befc9a8f4f7a3337`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 78
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Dangerous function 'exec' detected

- **ID:** `747244359d517887`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 134
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'exec' detected

- **ID:** `450d7f9aa0fcfcf1`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 164
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'exec' detected

- **ID:** `cddcd8c9a2fce3ea`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 195
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### BUG comment found

- **ID:** `5035889675aa7404`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 18
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `8480064503af33dd`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 226
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `8b69b144d9a1fb51`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 235
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'rule_category_confidence' has high cyclomatic complexity (19)

- **ID:** `86d1ff371f36ecf7`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 211
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'calculate_context_richness' has high cyclomatic complexity (16)

- **ID:** `2d1d64314b94def2`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 241
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `dc98856b522b37e6`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 8
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `064f7d129bbaed39`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 28
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `924a76c5a7099bb9`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 49
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `2f2b17ab21aa2c2c`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 67
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `6e42ef20e5fc0068`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 95
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `d0de775d40c7dc46`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 104
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `74847a4a5de30a66`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 113
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `d0162ce5a57d0402`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 129
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `00ee85e0d7258bda`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 139
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `5d72e13e2ac83c8b`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 149
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `97ce4891c7d23385`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 168
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'generate_report' has high cyclomatic complexity (35)

- **ID:** `0aae886553f8fb2c`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 282
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'update_classification_metrics' has high cyclomatic complexity (16)

- **ID:** `905208d2fbb74e5c`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 399
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_performance_alerts' has high cyclomatic complexity (20)

- **ID:** `baf9ac85bbbf595c`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 560
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'generate_recommendations' has high cyclomatic complexity (15)

- **ID:** `9a668910aa88b39a`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 638
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'new' has high cyclomatic complexity (11)

- **ID:** `603b3f36783c2f05`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 20
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has high cyclomatic complexity (26)

- **ID:** `162f54a878d3949e`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 33
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' has high cyclomatic complexity (22)

- **ID:** `9e0dc2ad906c4c24`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 52
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'record_feedback' has high cyclomatic complexity (14)

- **ID:** `43d6e89f41c3bc7a`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 78
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' has high cyclomatic complexity (13)

- **ID:** `36560740b023cdb7`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 80
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `e9efed1a724c7733`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 6
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `008639ea48bf5929`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 13
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `9c5364afa891fa0a`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 22
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `3d18dca051ed476a`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 137
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `137891dccd745c0e`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 191
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'heuristic_classification' has high cyclomatic complexity (13)

- **ID:** `f503899b69baff46`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 272
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `1fb4aa4912e440a8`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 288
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `788674142414c449`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 18
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'Ok' has high cyclomatic complexity (11)

- **ID:** `82849e5912574878`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 246
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `5011a30f650e241a`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 467
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `bb6a7cb0c46cf1c0`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 476
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `32e422a0be2617ed`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 485
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Potential secret detected:     let api_key = "sk-test-1234567890abcdef"; // T

- **ID:** `887762d002bcd57e`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 8
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

#### BUG comment found

- **ID:** `5b29ed57a506f66f`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 21
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `948b8193a2c01daa`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 22
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `6631678cb22a0e6d`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 23
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `b0d4730ab9f976b4`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 21
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `429976817d763b09`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 31
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `5cbb44682b4aca90`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 39
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `c193366051531843`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 54
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `e173dd592995f431`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 63
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `1f17e428baa64e69`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 8
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Dangerous function 'system' detected

- **ID:** `12f634845c867b09`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 13
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `8ee0dda610b54161`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 68
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `b6746c656a769323`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 81
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `f8dbfaeaeb361b90`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 94
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Function 'adjust_workers' has high cyclomatic complexity (11)

- **ID:** `4a5a05a1d9216ee2`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 114
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Dangerous function 'system' detected

- **ID:** `f05edc8644fd03d6`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 155
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### BUG comment found

- **ID:** `95cb73d0f12e67d7`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 189
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Dangerous function 'system' detected

- **ID:** `693b1231f3b5a38a`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 233
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `74aebe64a3e380f8`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 236
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### Dangerous function 'system' detected

- **ID:** `08027457677ee0a7`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 250
- **Analyzer:** security
- **Rule:** dangerous_function
- **Description:** Dangerous functions can lead to security vulnerabilities
- **Suggestion:** Avoid using dangerous functions or ensure proper input validation

#### BUG comment found

- **ID:** `a8cee0328c794ce2`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 59
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `428710680b87d660`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 180
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `a18ee9bdc5bf9c90`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 7
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `cd768c36ec625341`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 23
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `7ce058f5bfe6a475`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 31
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `eb5626ed6484b33e`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 69
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `a7e48df20789105a`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 79
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `1ec03a5067fdafa6`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 90
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'get_performance_summary' has high cyclomatic complexity (12)

- **ID:** `0af482318a7565e6`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 125
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_performance_thresholds' has high cyclomatic complexity (22)

- **ID:** `c113e627111fc720`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 196
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `23b5d49ffe58aef2`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 269
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `d60235e353499200`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 296
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Function 'Ok' has high cyclomatic complexity (14)

- **ID:** `a840b8126fc128d0`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 123
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' has high cyclomatic complexity (13)

- **ID:** `b45cb38e119a53b4`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 132
- **Analyzer:** code_quality
- **Rule:** high_complexity
- **Description:** High complexity functions are harder to understand, test, and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### BUG comment found

- **ID:** `f2047bcbc77fc3fd`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 271
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### BUG comment found

- **ID:** `7889d28a4c1e0e1d`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 285
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a bug comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Weak cryptographic algorithm detected

- **ID:** `d685e3ab9be51e80`
- **File:** `/workspaces/do-codeguardian/turbo-demo.sh`
- **Line:** 75
- **Analyzer:** optimized-security
- **Rule:** SEC-OPT

### 🟡 medium Issues

#### High cyclomatic complexity: 16

- **ID:** `6b42db4c9a77625c`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Blocking I/O operation detected

- **ID:** `bacd70567fb85f8e`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 114
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### High cyclomatic complexity: 16

- **ID:** `954aa42ef0e91950`
- **File:** `/workspaces/do-codeguardian/.github/gh-labels-creator.sh`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Inefficient collection operation in loop

- **ID:** `5817bfba9c77020f`
- **File:** `/workspaces/do-codeguardian/.github/gh-labels-creator.sh`
- **Line:** 49
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### High cyclomatic complexity: 29

- **ID:** `15c379134fe3e9b7`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-performance-monitor.yml`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### High cyclomatic complexity: 23

- **ID:** `96ff4263046a4ea2`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### High cyclomatic complexity: 26

- **ID:** `e42e53046e6de5ee`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/clean-code-developer.md`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Inefficient collection operation in loop

- **ID:** `df5488fee8113233`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/clean-code-developer.md`
- **Line:** 27
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `0d39bc80b4119184`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/clean-code-developer.md`
- **Line:** 29
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `84f5f0f0419a9f44`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/clean-code-developer.md`
- **Line:** 43
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `17ef9461991831bd`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 33
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `568d3faac090ce52`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 34
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `b06fa0456a66cbe3`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-research.md`
- **Line:** 4
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `82a3b224c76291db`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-research.md`
- **Line:** 65
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `6057ec89a6203e5c`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/general.md`
- **Line:** 8
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `93328d492f872250`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/general.md`
- **Line:** 26
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `4ae37a28cb1ee268`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/orchestrator.md`
- **Line:** 3
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `5c275110b4006ed8`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/orchestrator.md`
- **Line:** 24
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `d42b34a416c92977`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/orchestrator.md`
- **Line:** 29
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `87fcc42eafa50b1a`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/orchestrator.md`
- **Line:** 30
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `7c267691edfc52a7`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/orchestrator.md`
- **Line:** 40
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### High cyclomatic complexity: 20

- **ID:** `08558dd6448bbfef`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/performance-optimizer.md`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Inefficient collection operation in loop

- **ID:** `57fbf82e2b70fe5e`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/performance-optimizer.md`
- **Line:** 3
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `4bcbdd20799f5d93`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/performance-optimizer.md`
- **Line:** 24
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `a810efa3cfbee30c`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/performance-optimizer.md`
- **Line:** 26
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `992f0bca60ab1e64`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/performance-optimizer.md`
- **Line:** 41
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### High cyclomatic complexity: 16

- **ID:** `ac0d4444f139e5b9`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/security-reviewer.md`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Inefficient collection operation in loop

- **ID:** `021a375da75055b5`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/security-reviewer.md`
- **Line:** 24
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### High cyclomatic complexity: 35

- **ID:** `1752667dcf84f238`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### High cyclomatic complexity: 21

- **ID:** `42141d3055eba2de`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `c4bf2784fdb514c3`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 24
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `8c72e5d1a9b87156`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 25
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `1dcd8fa4fbf87c1e`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 26
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `64f3621498327ba9`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 32
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `0922cba8688570e1`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 36
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `ddfd2d3217ca7bb9`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 37
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `f7538d6d055253b6`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 44
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `ec13dd58e7e8bb74`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 45
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d4d91d770b5be636`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 46
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `726328f90381f488`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 48
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `90ab29fe310815dd`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 49
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `b4a27cbf0ee8b800`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 59
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `217b079291474d72`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 65
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `f78bdf1ca40bbb81`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 66
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `7d4751f65fd87a5b`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 74
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `4acd110ab9d852e8`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 79
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `c4b4652ef614a273`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 88
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `6667c65c64832c01`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 97
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `23f7a388c659b4f7`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 124
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Deep nesting detected

- **ID:** `3c69712069329e0e`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 137
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `520609c9363644e5`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 138
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `8e298b425cee6cfb`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 139
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `8a9b2e10715a3419`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 140
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `26dda7553178de60`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 141
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `6cff5ab62d26bec2`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 142
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `8ce06d056a80f9d9`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 143
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `d5d260131c8a3d24`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 144
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `9c609b92cb6c4d9e`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 146
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### unwrap() usage detected

- **ID:** `4ccb95a6fcf957dd`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 150
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `c0eb53e067cc33dd`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 188
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `8cc8ae99e3fc85bf`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 202
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `8fe673fb44e26dcc`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 218
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `2035a9c254cef604`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 223
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Function 'black_box' has moderate complexity (8)

- **ID:** `9c0d8bb373cf7b54`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 232
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### unwrap() usage detected

- **ID:** `56451cebfb30924f`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 239
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `4adf9273243c8613`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 252
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `878daa92b59b89e2`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 262
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `dd48ce23007274d4`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 273
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `90921819a3b6226c`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 285
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### High cyclomatic complexity: 29

- **ID:** `46c05847848c7af7`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Inefficient collection operation in loop

- **ID:** `3b898a38e3d46e15`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 5
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### High cyclomatic complexity: 22

- **ID:** `84edfdacb2912927`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Large file (505 lines)

- **ID:** `d37f7235f21f76c8`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 1
- **Analyzer:** code_quality
- **Rule:** large_file
- **Description:** Large files are harder to understand and maintain
- **Suggestion:** Consider breaking this file into smaller, more focused modules

#### High cyclomatic complexity: 27

- **ID:** `0624a6baea77c498`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `e4148661a0694eaf`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 27
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0c15511f0e5c9737`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 28
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `b84db9a540b92083`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 42
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `66bf3cd58246a79d`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 43
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0d69aadb53e3ff28`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 49
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `556d0bd11ef36f01`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 50
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `110c51420aeabd05`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 100
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `623cb1387312d846`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 101
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `9845bc859b36cb33`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 107
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c5b68158c75ff45a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 108
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `05f1fa74a7df91a1`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 111
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `1cbf6f7c48b3baae`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 112
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `258666785b9e097e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 113
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0a67121e4d44452a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 114
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `1f1ebde71d9d9367`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 115
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `4a7e6bdf6f6721b3`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 116
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0bff16eb9cc14d1a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 117
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `463e692f837946dc`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 118
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `91af01f4bab66a35`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 119
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `ea07190971c571c2`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 120
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `b335f97c09fe0f16`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 121
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `a1f4ca202c134ebd`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 122
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `72fb8e9aa5327b54`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 123
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e4c8f8b5c3460c03`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 126
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `179db8ade1696626`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 131
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `18b15ab7fab1fb0e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 132
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `a7bfb15aae4b1b1e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 143
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `ed0d9075692951f4`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 144
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7b28a5b1bfc8d23a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 145
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c99641775fd46b50`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 146
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `a875756336bb7150`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 147
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `507d56103f2f17a5`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 148
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `289d91cb25a4ec43`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 149
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `b70d3be61373aadc`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 155
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `ce2bf07091877c3c`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 161
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `6adcf075247511c4`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 162
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `47a93e631c751c9a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 167
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0566f68bf79088e6`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 168
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `ee0dfa461cc721c9`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 169
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `bca7f69a6ca1e270`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 170
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `4b5acd306a17f32e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 171
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0be90830cbdf5619`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 175
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `254d6e02408cf09d`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 183
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `6dded3edf4fa0d50`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 186
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `b5e826f23b36845e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 191
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0485b082ace35967`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 192
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d6964d81535f5618`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 197
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `225d111d74fcd775`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 211
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7dacca8fca96944a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 212
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `09d47f4516288ab0`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 215
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `2d99f1ddf171c972`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 230
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `623d202a526edccd`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 231
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0464d92900fa9238`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 232
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e1eda56dcd08f70a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 233
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `2523f27bb352b7b1`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 236
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `3b63912d25563d89`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 248
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `39715aa636c62a57`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 249
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `735f2f6abe425cce`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 250
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `f21c8ec6c07a3b39`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 251
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `edc2d43d5dd4c94d`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 253
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c1f5e9a1b9bfb8bb`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 258
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c14e0bacf72983f7`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 259
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `83c2f2d6ac328edb`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 267
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `eb63076a4bb6c547`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 275
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `9dbf64d60a48b8d3`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 278
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'create_test_finding' has moderate complexity (10)

- **ID:** `e34c81e7708faa20`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 282
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'create_test_finding' is too long (60 lines)

- **ID:** `0d8b7c85fbcb8b3a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 282
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'create_test_finding' has too many parameters (7)

- **ID:** `414d24e6dda35493`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 282
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'Some' has moderate complexity (10)

- **ID:** `4c5dd6f94b72ca86`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 289
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'Some' is too long (53 lines)

- **ID:** `197d69d2cd39cb11`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 289
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' has moderate complexity (10)

- **ID:** `8d97ee24819fe9c5`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 290
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'Some' is too long (52 lines)

- **ID:** `3d4db33155b95a7e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 290
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'create_test_finding' has moderate complexity (10)

- **ID:** `ae383f85f1afa83d`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 293
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'create_test_finding' has too many parameters (8)

- **ID:** `d09a90a788d1d5d1`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 293
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'Some' has moderate complexity (10)

- **ID:** `4b4c5567d1bf01dd`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 301
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'create_test_finding' has moderate complexity (8)

- **ID:** `195264547b29f91a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 304
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'create_test_finding' has too many parameters (6)

- **ID:** `be6dc70650a9ac29`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 304
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Nested loops detected - potential O(n²) complexity

- **ID:** `3750bc934defdcd1`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 318
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `861662005a519e2c`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 322
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `34b97e9b1f4f8409`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 323
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `40228a63c3e28a38`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 324
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e735ce5234853897`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 325
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `a45fd639ce1015ee`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 334
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `f6c5c02b256b09dd`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 337
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d5c94d642570800b`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 338
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `9cb2c787f242c2fa`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 339
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d6182abd5bbd7526`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 340
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `bd8886d07a333c62`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 343
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c48c70ed64eb7af4`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 344
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `13a81468b67488db`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 345
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `680662d385cd9540`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 346
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `1355c4189de89b1d`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 347
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `ef636e90092b96de`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 348
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `8a60599483302727`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 356
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'create_test_finding' is too long (61 lines)

- **ID:** `3833ff5974c8efb9`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 407
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'create_test_finding' has too many parameters (7)

- **ID:** `26b980fc198ac307`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 407
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'Some' is too long (54 lines)

- **ID:** `5b75deefef70ad56`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 414
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' is too long (53 lines)

- **ID:** `13477d1ec68779b7`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 415
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'create_test_finding' is too long (51 lines)

- **ID:** `73f82501e8bfb892`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 417
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'create_test_finding' has too many parameters (7)

- **ID:** `e63d60e39f60504e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 417
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'create_test_finding' has too many parameters (6)

- **ID:** `aaa833c514b43f60`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 427
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'create_test_finding' has too many parameters (9)

- **ID:** `5f00926499a9c057`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 441
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'create_test_finding' has too many parameters (8)

- **ID:** `e0193d412884dc70`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 451
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Nested loops detected - potential O(n²) complexity

- **ID:** `8f351084dbbe9923`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 464
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1f4f1b64f63fec42`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 469
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'create_test_finding' has too many parameters (9)

- **ID:** `36b1a12ac22a8ea2`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 477
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'create_test_finding' has too many parameters (9)

- **ID:** `36b1a12ac22a8ea2`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 477
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Debug statement found

- **ID:** `a10c69fe04ef67ff`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 24
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `f03b5c8644ceb00f`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 25
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e0958b83d67ceebd`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 28
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `1bcc6254940ec8a0`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 33
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `f2e3b0fce0fd6856`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 36
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `007a50b165afa3f5`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 44
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7bbde841cb972ac8`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 45
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0ad199975c485ec4`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 48
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `77f2f22a5222191d`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 50
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `f76bfdc684d9493a`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 60
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e895a7a61006915b`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 64
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `b82d1196301f47a6`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 72
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `8db0eb7c23d45ae2`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 75
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `9a7adad4a554dae0`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 79
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e8615a2d788aa71e`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 81
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e5a257dd6b6adfc3`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 84
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d0d66ac4d8c3e435`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 87
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `70512aaa1891ee37`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 88
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `2517f6374d82ce3f`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 89
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `18b5824a28cd481a`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 90
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `462f0d3025bf1d2d`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 91
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `6208bd764c86c4b6`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 130
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `272cf80c3bc2d696`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 134
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `cbd1eafdf200122d`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 156
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'for' has moderate complexity (9)

- **ID:** `321f057c4ba5ef69`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 158
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Debug statement found

- **ID:** `00a845f599669743`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 173
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `b3a4ab336b1a8f79`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 179
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `1833bbdf4a68ea98`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 180
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `76b59370e00fc2f8`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 181
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `cf5e740fd4a48285`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 182
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e9771ddf481e7367`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 193
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `5db7fdbfffe9b99d`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 210
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e44ddf677947bae9`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 222
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `befd2fe663bed77c`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 232
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `3bf1d7a734380133`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 243
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### High cyclomatic complexity: 16

- **ID:** `8aab37447aff4f6d`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### High cyclomatic complexity: 34

- **ID:** `ee51bdf01eb0156a`
- **File:** `/workspaces/do-codeguardian/performance_benchmark.sh`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### High cyclomatic complexity: 18

- **ID:** `e3579aaa4f6207ee`
- **File:** `/workspaces/do-codeguardian/performance_test.sh`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Inefficient collection operation in loop

- **ID:** `635bdb7684567613`
- **File:** `/workspaces/do-codeguardian/performance_test.sh`
- **Line:** 148
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### High cyclomatic complexity: 139

- **ID:** `2b685b0f3d0482f2`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### High cyclomatic complexity: 118

- **ID:** `febf1078f36966ee`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### High cyclomatic complexity: 118

- **ID:** `ae9eb88a0fd099ba`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### High cyclomatic complexity: 118

- **ID:** `8d1fe0db5f37e7b4`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### High cyclomatic complexity: 213

- **ID:** `a22e8092ea50d5a8`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### High cyclomatic complexity: 213

- **ID:** `1b91757c1faac43e`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Large file (940 lines)

- **ID:** `d3916d3befb8b4d3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 1
- **Analyzer:** code_quality
- **Rule:** large_file
- **Description:** Large files are harder to understand and maintain
- **Suggestion:** Consider breaking this file into smaller, more focused modules

#### High cyclomatic complexity: 144

- **ID:** `89d76c8083df9f76`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### unwrap() usage detected

- **ID:** `804f8e4c9a79af28`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 11
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `81da3fbf76345821`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 15
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `fa902414443082b4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 17
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `40bc0e6b2cc8ca01`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 20
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `2133675dce4d995e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 23
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Nested loops detected - potential O(n²) complexity

- **ID:** `668fa578aad1579f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 26
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `3113fdc55b688002`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 41
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'new' has moderate complexity (10)

- **ID:** `23d95e0c88f3b7d5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 48
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'analyze_line' is too long (117 lines)

- **ID:** `000b35287ba1b6a4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 88
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'analyze_line' has too many parameters (6)

- **ID:** `85c0376e4c17885c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 88
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Nested loops detected - potential O(n²) complexity

- **ID:** `820a4d3a4f6bfbd3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 117
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `9a6567ff2af4fe47`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 158
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'analyze_file_structure' is too long (59 lines)

- **ID:** `4d596420ceb0b397`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 206
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `59b21d049d737e30`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 229
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1dabba005e5072e4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 239
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Ok' is too long (102 lines)

- **ID:** `edc6b5c091362ac0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 263
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'analyze_function_complexity' is too long (102 lines)

- **ID:** `9bb4c9a92dd57292`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 266
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'for' is too long (93 lines)

- **ID:** `ef3788315fb0456b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 272
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Deep nesting detected

- **ID:** `ece002d71fd81b09`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 286
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `a09a1acd77c9498d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 287
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `d93d561c62022ffb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 292
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `053afca9342bae09`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 307
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `aa6f5a20d984df97`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 308
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `5b0678558d403ac9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 328
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `a2ca76e4ba7d3b85`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 329
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `8a9cffa9ed9e82c7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 352
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `a0f433dd66380c2c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 353
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `ca32a981082187c5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 375
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Ok' is too long (76 lines)

- **ID:** `cd5b5261c0d7eb3d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 386
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_import_organization' is too long (76 lines)

- **ID:** `fa0dbcc3dcfe7c9a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 389
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' is too long (70 lines)

- **ID:** `13ce8ac818b981e1`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 392
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'for' has moderate complexity (8)

- **ID:** `df5ce25b29a0f287`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 397
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Deep nesting detected

- **ID:** `ae8df5a1d437a0ff`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 400
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `7918e102f229d6ac`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 401
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `0edbbfcdf1c7de2b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 402
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `1dc93cd973380284`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 403
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `aa3e20f4bd6f5208`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 404
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `ed1892739b105fd7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 405
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `9bd67b40bb3bc3d5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 406
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `21690d7e60373e28`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 407
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `b34de3633579dcbc`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 408
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `0fa9378a1e79e517`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 409
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `1163ea281dd023b0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 410
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `d162562b1deb622d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 411
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `58ce83dafb6f59c2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 412
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `ac6ad6cbf9491ba3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 413
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `f244d10ff61e1625`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 414
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `50a51159b150f2ac`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 415
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `5475481257f5080f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 416
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `c373e7911d2b359a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 417
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `2a9cc357b255e164`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 418
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `81babe1cc4fe92aa`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 420
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `43aee316045670da`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 421
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `5ef245f2775ef2c2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 423
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Function 'for' has moderate complexity (9)

- **ID:** `66567060394d02f5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 430
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Deep nesting detected

- **ID:** `2cab15975189dc28`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 433
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `553044bf4aca5e3a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 434
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `664caa2fd9fca2a6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 435
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `2c579bb39400d2fe`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 436
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `3bf5737516d9b71a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 437
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `b53b970c6a35f6c9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 438
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `b2075bc1313cd66f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 439
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `564d4661d015efef`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 440
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `8d04bd31097a7731`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 441
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `d6eca78b0291f91c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 442
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `6374864a33990be4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 443
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `f356b7123d5cd73d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 444
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `d12fb757eb84323a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 445
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `6af5d088ab069e1c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 446
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `4e2d35d5ef4a8a2c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 447
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `505ef06b6287a8c3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 448
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `3162edebc6765c65`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 449
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `d49743d20b7aaf3d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 450
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `57485cf6251560cb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 452
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `8a8c402d41feade9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 453
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `0a4109a3acf9cd6f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 455
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Function 'is_acceptable_magic_number_context' has moderate complexity (9)

- **ID:** `eb11b35569b62cb4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 466
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'is_commented_code' is too long (383 lines)

- **ID:** `fb109d7554196a37`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 480
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2ac0b75ba9d0646f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 501
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `9c5ea44380b99ef0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 502
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### unwrap() usage detected

- **ID:** `ddb9fbbd538df9e2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 511
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `e77c68cc8d9d78cb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 512
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `0d363387e28e6cc5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 513
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `76b35cf322e2d3a6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 515
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2d836666660fe2ed`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 518
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `61ebeba4f2c790f1`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 519
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `502819e54a4e046d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 538
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1a75887ea18e8fa2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 542
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `5f7d5e481238a4b4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 551
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `22b7a57fe7fdca68`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 578
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `7d85e78f9e0a3f99`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 582
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2e4dda0144792ce8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 591
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'check_rust_quality' has moderate complexity (10)

- **ID:** `6f3aa0cfe7c9cf6b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 636
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `28f4cc06801b2f40`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 659
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `c790214fdeb7a3a2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 664
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `11ca0653d6a43eb9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 676
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'check_javascript_quality' has moderate complexity (8)

- **ID:** `e9136c4472185717`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 686
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `187bd7c4737783a3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 712
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2f3c4c1ab8d0ba51`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 724
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'check_python_quality' has moderate complexity (9)

- **ID:** `a065fdfc660afa13`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 731
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'check_python_quality' is too long (51 lines)

- **ID:** `7cfc96a2059539b8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 731
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `6d2ce704065af983`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 759
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'check_naming_for_language' is too long (79 lines)

- **ID:** `574d615bc71c147c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 783
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_naming_for_language' has too many parameters (6)

- **ID:** `fa336dc8afa2ac36`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 783
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### unwrap() usage detected

- **ID:** `9db567a1c738ac96`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 793
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Nested loops detected - potential O(n²) complexity

- **ID:** `7f159bc45a5660bb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 794
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### unwrap() usage detected

- **ID:** `3010eb6b12e0b4db`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 813
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Deep nesting detected

- **ID:** `0e05ad9dd5d6d237`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 816
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `72d952c366037ca5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 817
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `2cdbbe1aecae4168`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 818
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `81d9ff0f7bac29aa`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 819
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `86969f4548ecf17f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 820
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `7f22e157a8006771`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 821
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `84115d83881a4caf`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 822
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `8307e583a50b992d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 823
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `f1b8b51871767b73`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 824
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `a8f13251da2b57d5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 825
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `e8dfdffef4a6c825`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 826
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `f687d5e81114c489`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 827
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `a0a3e02274cf7b4a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 828
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### unwrap() usage detected

- **ID:** `c44ba17ac0c8702a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 837
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Deep nesting detected

- **ID:** `b534064ec526e273`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 840
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `bf54ac57c24952cf`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 841
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `f9501bba9351834e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 842
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `16c4d08e49ce2791`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 843
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `1958f8e100b7a936`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 844
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `53cda0021e4bd768`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 845
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `4d1429f2aae03329`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 846
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `40a5a2bd65b27413`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 847
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `e785f72ad22833dc`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 848
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `662ea6e0d49c7774`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 849
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `4f88d654c236445c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 850
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `54814549b4085ff4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 851
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `b08bed97200b831c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 852
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### unwrap() usage detected

- **ID:** `ee33b4efc9deb69d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 915
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Complex conditional expression

- **ID:** `e157a7681189fa5d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 922
- **Analyzer:** code_quality
- **Rule:** complex_condition
- **Description:** Complex conditions with multiple logical operators are hard to read and test
- **Suggestion:** Break complex conditions into smaller, named boolean variables

#### unwrap() usage detected

- **ID:** `6ade7394a0fe0047`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 926
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `5674536c8b0ad858`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 933
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `694b8005085655cb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 937
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### High cyclomatic complexity: 60

- **ID:** `2ecb52d64a092410`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### unwrap() usage detected

- **ID:** `cdf3fa8c088f1cc7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 11
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `66a5ee719ce762a4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 14
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Nested loops detected - potential O(n²) complexity

- **ID:** `39537abe54550f16`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 16
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `9769d4d55800f919`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 27
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `404400d08f9f681a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 34
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `bf96d3fb95035af5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 88
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `4c93bc05042d21d7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 105
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Ok' is too long (84 lines)

- **ID:** `c13db59d7d5e7730`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 109
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_dependencies' is too long (84 lines)

- **ID:** `dcac972127013cd2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 112
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_dependencies' has too many parameters (6)

- **ID:** `7e9e3ce1cc7271b5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 112
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'for' is too long (73 lines)

- **ID:** `7cad418b18b5af75`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 120
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `dbb3bc0ff2d8aab0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 121
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Some' has moderate complexity (9)

- **ID:** `aa46ee01fa681bfc`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 122
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Deep nesting detected

- **ID:** `4bb09b6495216df1`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 155
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `073c3f5496034f06`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 156
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `61eaaa68d9ee6b55`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 157
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `e44b11ba6aab4ffa`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 158
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `1591032a1c86e819`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 159
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `d2e0194d3d0df2f2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 160
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `cd99e113932b0d4c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 160
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `b0bf49e605b02f92`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 163
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `9e8c8542be13e986`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 167
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `3db499a9bbfebd0f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 175
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `f51ebddeb5acf287`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 176
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `f66f062baf907ad6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 177
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `5653d91bbe046add`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 178
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `668250b691d5adb8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 179
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `f6e41bab14f85c2c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 180
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `40d0df461344b9d4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 183
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `05279c3ea7fc80d6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 183
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `8affb35b059ff846`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 186
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `f399d40285112c7b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 186
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `4a13db5ad7e1d142`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 187
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Function 'Ok' is too long (59 lines)

- **ID:** `fd0dd9eacdd298b9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 194
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'check_rust_dependencies' is too long (59 lines)

- **ID:** `c36339fb27cf71bc`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 197
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `5e549b53977b200e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 205
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `b9eae919d28bf9a0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 213
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `9f84596d2c33d8da`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 214
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `8722106db3c43bfe`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 215
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `544db31df5fb9b57`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 216
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `554f15ddf3b53020`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 217
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `6a5928a23c2d79fc`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 218
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `3ae1a37b2b37eefb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 221
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `90a41fc443350f38`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 222
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `da7a336a572ec31b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 225
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `10976675610477ab`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 226
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `c8fdc25d72c2df64`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 235
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `d7a430cc813eadaa`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 236
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `386bf58f143ec4a5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 237
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `6d1d75428af2ad82`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 238
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `03e3a29e18a0a88b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 239
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `98a06bd79aaf1fe0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 240
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `b72b2de4d7327f42`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 243
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `a5067d6cf44fe1a7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 244
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `009ce0bad37a2bdf`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 247
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `8cedacf67dd50cb4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 284
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `596be458bf20a730`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 295
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `f6d6fb9bf2a373e7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 323
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `47cf422f71c9c61e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 343
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### unwrap() usage detected

- **ID:** `34022993e5e73142`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 403
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `08784e4b4ff666e6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 423
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### High cyclomatic complexity: 19

- **ID:** `466c72925237afcb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/integrity.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Function 'check_file_integrity' has moderate complexity (9)

- **ID:** `1177eb7fa9705f5d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/integrity.rs`
- **Line:** 29
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `da04d63ecda300ff`
- **File:** `/workspaces/do-codeguardian/src/analyzers/integrity.rs`
- **Line:** 51
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `0e39fbb30e763d2d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/integrity.rs`
- **Line:** 78
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `feff43e958a8cd79`
- **File:** `/workspaces/do-codeguardian/src/analyzers/integrity.rs`
- **Line:** 83
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### High cyclomatic complexity: 23

- **ID:** `17c391f6670dfd95`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Function 'check_config_drift' has moderate complexity (10)

- **ID:** `1620c984b33ce47f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 21
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `aeb27e4663ceb660`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 25
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `24542193a27f779b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 30
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'analyze_config_content' has moderate complexity (8)

- **ID:** `6089a55c32a882eb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 80
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `8a60eb1a181ada0a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 84
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Ok' has moderate complexity (10)

- **ID:** `97767120ba1a2675`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 109
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Deep nesting detected

- **ID:** `0daa860cb1869a19`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 114
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `ff0d08f65f3016d6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 115
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `5f93668f255fc13c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 116
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `ac131fb89afc00a1`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 117
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `7af1aa30384ced8c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 118
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `389ea56dd3d79f84`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 119
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `e73be61f78ce4179`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 122
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `dd4b4afada269783`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 123
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `d79c01af24bacfc0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 126
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Function 'Ok' has moderate complexity (8)

- **ID:** `9685981f3ccb50f0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 149
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'check_yaml_formatting' has moderate complexity (8)

- **ID:** `eeba6f3ce400c069`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 152
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Deep nesting detected

- **ID:** `1cf49a923f3dde11`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 162
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `64c70a6b20085bdd`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 163
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `e791813846042c66`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 164
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `07b98599e8a46656`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 165
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `b345d1baa8378179`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 166
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `b7969f992ecc0ebf`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 167
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `36e32ab14640cb97`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 170
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `44b821aeb24eb9f1`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 170
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `ae69af1e9ebad6f8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 173
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `8beb3f844762dd21`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 174
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `74d6ef81ba53b187`
- **File:** `/workspaces/do-codeguardian/src/analyzers/mod.rs`
- **Line:** 63
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### High cyclomatic complexity: 96

- **ID:** `5a2423f9c1e09f82`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `35f3a19ee026f8b4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 9
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `f4d90317b39e3c5e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 26
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### unwrap() usage detected

- **ID:** `cd17510c47b86c5b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 28
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `86d77ff6be7bb8fe`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 30
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `a5f4c0139c467dac`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 31
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `fae8bd5ce4fd54cb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 34
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `93d4c931512e0739`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 34
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `ab133b3f06845205`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 39
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `3bdb54b092d7189f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 43
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `5700b6f08454e955`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 53
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'check_non_production_code' is too long (134 lines)

- **ID:** `008e4a6313bcef6e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 58
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1a55434907832f58`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 62
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'for' is too long (127 lines)

- **ID:** `cbb6ff8dde1ef624`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 62
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `e83b560a26ced33b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 65
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### unwrap() usage detected

- **ID:** `db31e767393caea8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 67
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### FIXME comment found

- **ID:** `b671f632735d67a9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 70
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a fixme comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Debug statement found

- **ID:** `f6624c3ae8296046`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 98
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7d97ab8175d5d401`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 99
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7e42923280be523e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 109
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c3358e921712f563`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 113
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c216638b35adcdac`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 116
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `6ce354449b7271e3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 119
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Deep nesting detected

- **ID:** `4863388ede0b36db`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 137
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `65c782ccd6f369b6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 154
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `f80727bc10a93534`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 160
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `e66e6135f7a19d46`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 216
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'analyze_secret_context' is too long (94 lines)

- **ID:** `83bcae18cd240012`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 238
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `76093dfe9f3e595d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 316
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `d44a80efff36b270`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 329
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### High cyclomatic complexity: 89

- **ID:** `1e4259dfbe98e296`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Function 'analyze_security_optimized' is too long (98 lines)

- **ID:** `7f16c4cdc2ed7d65`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 40
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `680b68de5a30e90f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 65
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `16723b768d098787`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 73
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `b0b515e61797b82f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 81
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Some' has moderate complexity (10)

- **ID:** `38e55bf72ad22c3b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 86
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `5ea452b77557924b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 89
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Some' has moderate complexity (8)

- **ID:** `bf5d047e38a39713`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 94
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `8bc1b8cfd46a460c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 97
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `68533ec772beb988`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 102
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `6ff2e244fc036e0a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 105
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `53615e24ec006c9a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 106
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `e79a6b89ad71399b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 107
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `0fcd92eda85fd638`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 108
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `4f28dfc77cd3baf6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 109
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `c38589be0bb4f234`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 110
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `86ed0fb58247f63b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 111
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `76e6e5caefae4a01`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 112
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Function 'analyze_performance_optimized' is too long (76 lines)

- **ID:** `566e2b71c7eea33f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 139
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' is too long (88 lines)

- **ID:** `c9211ba06eac1673`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 157
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `af024ec85dbdef7b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 160
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Some' is too long (80 lines)

- **ID:** `8d944b80f2075934`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 165
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `95c015ad8d80d391`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 168
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Some' is too long (72 lines)

- **ID:** `9aaec811f4970ac9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 173
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `e763852ba5e0c352`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 176
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Some' is too long (64 lines)

- **ID:** `cb47958ed91be300`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 181
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `4acce2505aef10bf`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 184
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Some' is too long (56 lines)

- **ID:** `6f5c77ad8819b542`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 189
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Inefficient collection operation in loop

- **ID:** `ac498bce78a2bc6b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 198
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Function 'analyze_quality_optimized' is too long (78 lines)

- **ID:** `aac530a9af974b5c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 216
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Some' is too long (109 lines)

- **ID:** `10d4a395c248b70d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 241
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Deep nesting detected

- **ID:** `1f63de1737299ab7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 242
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `9aa8bfa2b3ce3e43`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 247
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Some' is too long (98 lines)

- **ID:** `b3aab6123145162c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 252
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `662b0f1c70f4ba9f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 257
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Some' is too long (88 lines)

- **ID:** `84a4a337c54593d5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 262
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `55fb6d2674e2f5ea`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 265
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Some' is too long (80 lines)

- **ID:** `e893295f16d1683a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 270
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'analyze_dependencies_optimized' has moderate complexity (10)

- **ID:** `bb98521a91bfc85d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 295
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'analyze_dependencies_optimized' is too long (54 lines)

- **ID:** `c5463ad02e05f253`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 295
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `58c42f74cd3b7d4f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 318
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `b59e38069255f0e5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 369
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `5237ec0ceeb735d3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 384
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `35697a5d45e45dd8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 397
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### unwrap() usage detected

- **ID:** `36f96481490701fe`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 415
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Nested loops detected - potential O(n²) complexity

- **ID:** `b7619efcc1cab128`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 423
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1886bf4f3f711376`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 423
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### unwrap() usage detected

- **ID:** `261e0a90ff7ba52f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 426
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `7eb481fd92f3441b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 451
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### High cyclomatic complexity: 44

- **ID:** `9bf0811f769cfe22`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Nested loops detected - potential O(n²) complexity

- **ID:** `a42ca283848d84f2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 9
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `c99a0cb98a1496ab`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 12
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2ed0f53b3c80abea`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 20
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### unwrap() usage detected

- **ID:** `a98765a738738ed7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 35
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `abd61dff753a9abd`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 39
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `1c5ff4ea71af4c2e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 43
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `90682ea41bf4c022`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 47
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `02e977f25cd81d91`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 51
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `964de4ef70cfe52e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 55
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Function 'new' has moderate complexity (10)

- **ID:** `bec12a3d7e27d9f3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 69
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### unwrap() usage detected

- **ID:** `b197502351d716da`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 74
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `61dc67113ab9c9d7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 78
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `62d7fa93f48dac06`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 82
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `3c030e4a9fe2c161`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 86
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `c40260bf4fa09c0a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 90
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Function 'new' has moderate complexity (10)

- **ID:** `be9643115bbf0d2d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 105
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### unwrap() usage detected

- **ID:** `b5aca1b7583a72e7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 108
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `61d75e5001f293dd`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 112
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `c47536bc4059e207`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 118
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `876ce4009527474a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 124
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `155534ed17f4bfa2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 128
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `6ea58b4fd844f3e3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 146
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `6607c6b5800d8e90`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 149
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `ef7a0ac820d3d87b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 152
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `7bdce668369e0baf`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 155
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Function 'for' has moderate complexity (9)

- **ID:** `89e5a92dbe994bf3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 193
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `f23533f28ed69040`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 203
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2708f688d5c4e391`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 222
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `9d0c6878da3ed9fc`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 223
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1d6b9111d9239450`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 232
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'calculate_complexity_fast' has moderate complexity (10)

- **ID:** `757c7c8060555466`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 243
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `59310193b025aec3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 249
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `b94a9a55fcf9a1dc`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 250
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `14517cdda0187483`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 261
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'test_optimized_patterns' has moderate complexity (8)

- **ID:** `22d51646658f7bb7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 381
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `77061673c48bfbc7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 388
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2bbe0b4970cbfb6e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 388
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Nested loops detected - potential O(n²) complexity

- **ID:** `8f46d7f4c5202633`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 405
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Large file (651 lines)

- **ID:** `1d06c3114ef4a637`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 1
- **Analyzer:** code_quality
- **Rule:** large_file
- **Description:** Large files are harder to understand and maintain
- **Suggestion:** Consider breaking this file into smaller, more focused modules

#### High cyclomatic complexity: 112

- **ID:** `410af3a2e9c61988`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Nested loops detected - potential O(n²) complexity

- **ID:** `9646acb250fac084`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 19
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `09c78254e9e82cf4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 21
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### unwrap() usage detected

- **ID:** `05494457e67c7ec5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 21
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `8181f32d0905de60`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 25
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Regex compilation in loop detected

- **ID:** `7ade7ab5bfc73c95`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 28
- **Analyzer:** performance
- **Rule:** regex_in_loop
- **Description:** Compiling regex patterns repeatedly is expensive
- **Suggestion:** Compile regex patterns once outside the loop and reuse them

#### unwrap() usage detected

- **ID:** `d0c461b4b79676f7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 28
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `842af46c814de4d0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 32
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `42b61771e42c0c70`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 37
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Regex compilation in loop detected

- **ID:** `d2af86737bc14b09`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 41
- **Analyzer:** performance
- **Rule:** regex_in_loop
- **Description:** Compiling regex patterns repeatedly is expensive
- **Suggestion:** Compile regex patterns once outside the loop and reuse them

#### unwrap() usage detected

- **ID:** `b3d049afd9eeb21e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 41
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Function 'analyze_performance_issues' is too long (133 lines)

- **ID:** `66a11682f86a88a8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 62
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'for' is too long (122 lines)

- **ID:** `3ab1108fe7f66105`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 67
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2f9c2e092b627e31`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 70
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1b1825a1c7c0a340`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 87
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `30988717790f5dcd`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 105
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `f2b3275d22cb6986`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 118
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `e9f0137c0e97edb7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 122
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `57edf4246da76140`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 166
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `a7ed33eff5e630d4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 199
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Inefficient collection operation in loop

- **ID:** `55c5f086aea8ab6a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 199
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Nested loops detected - potential O(n²) complexity

- **ID:** `50126b23772a6d03`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 203
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2c685e62362d10d2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 210
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1f7c9ed9ce7e1dc0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 211
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `6e18aca225f89116`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 212
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `33e00aef0d5d3cb2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 226
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `25dc8d7dfac3ea95`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 227
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `ca1a43d38631587c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 228
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `d216e6e5e42f6be4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 241
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'check_rust_performance' is too long (71 lines)

- **ID:** `c2ab5b402dc91d0f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 287
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `fc2f1d90452fde37`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 315
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Ok' has moderate complexity (10)

- **ID:** `f02dad839977603a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 356
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'check_javascript_performance' is too long (69 lines)

- **ID:** `233cd45c80725b41`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 359
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'if' has moderate complexity (8)

- **ID:** `a0c26b8b2609ce8e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 368
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Inefficient collection operation in loop

- **ID:** `678debef97612600`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 369
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Nested loops detected - potential O(n²) complexity

- **ID:** `238e018bc43cfb04`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 401
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1639fa0d88b88d72`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 406
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `390a34be57dc44cb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 421
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Ok' has moderate complexity (8)

- **ID:** `78209073382e7f53`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 426
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `75c72ce2ee63b473`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 438
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `cc65606efeea46a8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 451
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `548fd90b3e4911de`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 455
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1eee734660520bb2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 467
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `fb4e4bb29319966d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 471
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `bbcfc090f0c481d4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 501
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `951dfa37af116e68`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 505
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `6274e38c81d338ab`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 544
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Inefficient collection operation in loop

- **ID:** `98a241ef12f9e2a2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 544
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Nested loops detected - potential O(n²) complexity

- **ID:** `8bf28e72d9e4c1ac`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 549
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `6cd300ff8d642c91`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 550
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `16f74ebf2d33d573`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 563
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `57a8f982477ae9c2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 566
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `37b1c4dd6727f70a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 622
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `517673893f1b0189`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 622
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### unwrap() usage detected

- **ID:** `c1c4142d38e170c7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 626
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `1e17f9e67f0554ae`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 637
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Blocking I/O operation detected

- **ID:** `583204ab4d4bc5d8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 644
- **Analyzer:** performance
- **Rule:** blocking_io
- **Description:** Blocking I/O operations can freeze the application and reduce responsiveness
- **Suggestion:** Use async/await patterns or non-blocking I/O operations

#### Blocking I/O operation detected

- **ID:** `691f9da90149d36a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 644
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### unwrap() usage detected

- **ID:** `1667de597e6d1cec`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 648
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Large file (577 lines)

- **ID:** `6f815397af17d9a2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 1
- **Analyzer:** code_quality
- **Rule:** large_file
- **Description:** Large files are harder to understand and maintain
- **Suggestion:** Consider breaking this file into smaller, more focused modules

#### High cyclomatic complexity: 108

- **ID:** `090faf84237bc586`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `54ce10458e14349c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 10
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `b2fe9af06325b696`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 27
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `f95366154f09456c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 29
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `d15e71f724d74eb9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 37
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'analyze_security_issues' has moderate complexity (9)

- **ID:** `49dbb4f810edebb3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 61
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'analyze_security_issues' is too long (73 lines)

- **ID:** `7d517af7382dae11`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 61
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' is too long (56 lines)

- **ID:** `0608f6c4c5ea82cb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 73
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `4125bce736cd4a96`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 77
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `ce9ff798b3427ef7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 78
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `a8419681817608b4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 88
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `0692d00baf26e410`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 94
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `e655b2acc10650a2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 95
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `ceb790d0580e5c73`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 96
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `420d121eb8bfb47c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 101
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `c1804fc51a935ee9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 102
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `6eac384fe8d63a1a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 103
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `e1f26b7a7959238a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 108
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `f7d510c4c53a4d7f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 109
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `2bf2c45e35254d69`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 110
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `7999f805ab528dcc`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 115
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `9f326b7f92d17257`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 116
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `dfecb02588efa343`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 117
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `48977058afe12e29`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 122
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `00b1585dedf633de`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 123
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `311bd1fbf041ae6b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 124
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Function 'Ok' is too long (57 lines)

- **ID:** `9ba1943beb46bafe`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 132
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'analyze_line_security' is too long (178 lines)

- **ID:** `d12705a193db34e8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 135
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `8ee6d9d0da53af18`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 286
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `21b1dcf2221b69dc`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 305
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Function 'analyze_secret_context' is too long (99 lines)

- **ID:** `805c2fc1d4f02ee9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 315
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1e7a98b3402be9d3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 398
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `b5899a6eaf044fbc`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 411
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'check_file_level_security_fast' has moderate complexity (9)

- **ID:** `41fb2be2f6f219ef`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 415
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `fd544313ef2ae000`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 422
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `44ca9ca611aa0ff0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 438
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `c926e038cedbdc47`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 439
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `08c05d864ea54dd2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 468
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `89d9c5426175c1f0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 482
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `8c2e4af5e2bb35fe`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 492
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'supports_file' has moderate complexity (9)

- **ID:** `d703177c3b9a6c52`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 498
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### unwrap() usage detected

- **ID:** `78e3b37376705bca`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 552
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Hardcoded secret in non-production code

- **ID:** `aa60ffeb5cd9f2ec`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 559
- **Analyzer:** security
- **Rule:** hardcoded_secret
- **Description:** Non-production secrets should be externalized or clearly documented
- **Suggestion:** Move secrets to environment variables or secure configuration

#### unwrap() usage detected

- **ID:** `d5aae76448d0172b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 563
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Inefficient string concatenation in loop

- **ID:** `dfc9573ae93e492b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 570
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### unwrap() usage detected

- **ID:** `110a67a9987b7a5d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 574
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### High cyclomatic complexity: 45

- **ID:** `dac28e14caa5e2ed`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs.new`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Inefficient collection operation in loop

- **ID:** `2d65f0ef53c6687e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs.new`
- **Line:** 156
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient string concatenation in loop

- **ID:** `5d03828307b5150a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs.new`
- **Line:** 260
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### High cyclomatic complexity: 22

- **ID:** `f5f699d12e30a8ed`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Function 'check' has moderate complexity (8)

- **ID:** `954221e159b47872`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 34
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Unsafe block detected

- **ID:** `7cc55a7795db25f2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 41
- **Analyzer:** security
- **Rule:** unsafe_block

#### Nested loops detected - potential O(n²) complexity

- **ID:** `6d8499ae5e47b9de`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 49
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Unsafe block detected

- **ID:** `fbb3f40c752ce761`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 55
- **Analyzer:** security
- **Rule:** unsafe_block

#### Unsafe block detected

- **ID:** `1586324893f0cf59`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 76
- **Analyzer:** security
- **Rule:** unsafe_block

#### Unsafe block detected

- **ID:** `87da29e6ae024fc5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 118
- **Analyzer:** security
- **Rule:** unsafe_block

#### Unsafe block detected

- **ID:** `7ebc0c58375b6268`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 136
- **Analyzer:** security
- **Rule:** unsafe_block

#### Unsafe block detected

- **ID:** `85cd159646d92a39`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 139
- **Analyzer:** security
- **Rule:** unsafe_block

#### High cyclomatic complexity: 49

- **ID:** `04545daf98f8557b`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `3a084583d569bd1e`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 12
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d05b697eb064be7f`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 23
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `4d0c7bf370a11622`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 42
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'load' has moderate complexity (8)

- **ID:** `3182a6eb47f984fa`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 66
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Debug statement found

- **ID:** `d881c15cb94f2ef8`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 77
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Deep nesting detected

- **ID:** `7a2d376b314a1118`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 78
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `81a5aeb5b6a60a87`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 79
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `6746eb072a3e3f87`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 80
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Debug statement found

- **ID:** `5150621e88aaccec`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 86
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7a8a996b41db38e3`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 91
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `a4fd8c34e61c06c8`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 109
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `5fd5bca7e43b7a55`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 119
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'Ok' is too long (65 lines)

- **ID:** `5092eb2f528c0f84`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 120
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'is_cached' has moderate complexity (9)

- **ID:** `649f395bb2146eff`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 171
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'cache_findings' has moderate complexity (8)

- **ID:** `8aa47d9cfc4f37cc`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 196
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'Ok' has moderate complexity (8)

- **ID:** `74e7798a11aa4c2c`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 204
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'should_cache_file' has moderate complexity (8)

- **ID:** `f7e153d4f58dcdb2`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 233
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### unwrap() usage detected

- **ID:** `f62c5e2ff19fa2fe`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 283
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `d7486a706e608b70`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 298
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `910dc4cfe3fea5c1`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 334
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `fa575c0abbe615ac`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 362
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `28d04c91c36bfa54`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 382
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `e0a6bc09c80cd8c6`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 431
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `ba851980afc65c45`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 438
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### High cyclomatic complexity: 24

- **ID:** `85c46e2a73c074e0`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `116807c77558dc6d`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 17
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `bc9b230c383bd80f`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 38
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `2f66ae127e194800`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 56
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c94bd06aef37d297`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 65
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7f46409bf7b71872`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 81
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `972cd5c9e139d6e0`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 99
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `fd626c0e2230d00f`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 100
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0e58b22149c95b94`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 101
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `8235e201553b5455`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 102
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `f5ff50f3e6bcf0e7`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 103
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0efebfe19975833f`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 106
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0503bd3711843bc2`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 115
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `dfd8db437add38d9`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 120
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `10ac6e2a664300eb`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 121
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `c0e1edc0b840bca9`
- **File:** `/workspaces/do-codeguardian/src/cli/check.rs`
- **Line:** 122
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Large file (640 lines)

- **ID:** `304d9a1cf687fc17`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 1
- **Analyzer:** code_quality
- **Rule:** large_file
- **Description:** Large files are harder to understand and maintain
- **Suggestion:** Consider breaking this file into smaller, more focused modules

#### High cyclomatic complexity: 64

- **ID:** `dfd8c9de73147fe3`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `a8294374e5f4ab23`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 24
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e58dfaf496da84b4`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 25
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `f5da0bb49a7adc30`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 26
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `46271c54d739a2c9`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 27
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `1de870b56bf39a93`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 28
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c5483902b7002ad8`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 49
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e1bcf25b7e9a92ce`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 54
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `5944eb179f043c9d`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 73
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'create_issue' has too many parameters (6)

- **ID:** `2457675cb730ba02`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 90
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Debug statement found

- **ID:** `c68cbcc245da4bad`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 107
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'update_issue' has too many parameters (6)

- **ID:** `094d787fe2f2153e`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 134
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Debug statement found

- **ID:** `4535303d9a31b0d4`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 151
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `656dc1c2360007fb`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 275
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Inefficient collection operation in loop

- **ID:** `b646e9f8850a1ee8`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 331
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Nested loops detected - potential O(n²) complexity

- **ID:** `75553c613ffc1ff8`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 333
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `70e3a1b1e04f5827`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 342
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `fc6663cdb0251084`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 385
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `4b1b20d540f7d925`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 581
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### High cyclomatic complexity: 16

- **ID:** `eac832d376e75e38`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `f8984dc9b56db121`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 11
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `aa38142543559988`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 12
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `61c37f8fd8bb59a7`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 29
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `a379716f86d5457f`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 30
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `f3c86474bd9cc87d`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 31
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `a9864934068d47bb`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 32
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `977b62b33ceaecec`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 33
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `02d0238535115c4a`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 44
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `5d954dae50cb7692`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 45
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `27d47d3e54f001a9`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 52
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `25159ff4080b42af`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 53
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'Ok' is too long (130 lines)

- **ID:** `1fb3cf57d7d23e67`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 67
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' is too long (128 lines)

- **ID:** `07fb4503ccb4a30d`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 69
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Debug statement found

- **ID:** `86b593f48f557183`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 80
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `2a6f8e7c1ae8391b`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 83
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `9ee3be4b82b981b4`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 84
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `56440dddfe90a454`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 85
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `04cc8ee2106294a6`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 86
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0a2af6839f7890b5`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 87
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `b2ccf45a2b5d6c58`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 88
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Unsafe block detected

- **ID:** `9dae1d0aae7d4a39`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 164
- **Analyzer:** security
- **Rule:** unsafe_block

#### Nested loops detected - potential O(n²) complexity

- **ID:** `8c0434d20810c23f`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 184
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `2a5766b7d4628b03`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 192
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `3a8ab748f787436a`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 193
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `9856cc44b91f46e2`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 9
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `faecde4bb3ce4297`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 10
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'Ok' has moderate complexity (8)

- **ID:** `c9f1157c1e4eda31`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 11
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Debug statement found

- **ID:** `b01525fa97490fb3`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 18
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `1a2ced6cd71fd8fa`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 19
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `3b4663a8a8b90a19`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 27
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `48e026d4088e0f64`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 33
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `52ca83ef699f6920`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 41
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `6a086e518aba16c4`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 50
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0472c054ec8c16bc`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 51
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0b5a1d9707b5e6f4`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 55
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `04b8fc3baf4cbeb3`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 63
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `472e8a6b2b8586ff`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 74
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `bd4703e11b5303a1`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 86
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `617b1caa88357c0e`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 93
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `8dc8e6aa572e7272`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 96
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### High cyclomatic complexity: 20

- **ID:** `360d46d05359a5ff`
- **File:** `/workspaces/do-codeguardian/src/cli/report.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `7cb1245d630c2406`
- **File:** `/workspaces/do-codeguardian/src/cli/report.rs`
- **Line:** 22
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `5c51d9ec4a2f9da0`
- **File:** `/workspaces/do-codeguardian/src/cli/report.rs`
- **Line:** 114
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `5a606696b2719a8f`
- **File:** `/workspaces/do-codeguardian/src/cli/report.rs`
- **Line:** 129
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `b49d51fb939e81f5`
- **File:** `/workspaces/do-codeguardian/src/cli/report.rs`
- **Line:** 141
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'for' has moderate complexity (8)

- **ID:** `8e3efbceddef4d06`
- **File:** `/workspaces/do-codeguardian/src/cli/report.rs`
- **Line:** 246
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### High cyclomatic complexity: 34

- **ID:** `fe878feb3f871523`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `4f02836207a89c6d`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 18
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `72a18a8e9c016094`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 63
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `789a4db1c6726354`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 70
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e273e179c0197d99`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 112
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `be3e36f65439a642`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 119
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `7247bc2ee7b385c5`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 150
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'print_training_summary' has too many parameters (6)

- **ID:** `9a9cd58e9d4f7853`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 192
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'print_training_summary' has too many parameters (6)

- **ID:** `9a9cd58e9d4f7853`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 192
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Debug statement found

- **ID:** `ff2c885670260470`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 202
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `4f25720c19cb11a4`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 203
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `089ef74c78056574`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 204
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `95d68b756a3923d6`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 205
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `540155844d5b0338`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 206
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c7d69d658b58bd53`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 207
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `21da66a2d959629d`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 209
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d2e291e67521afa8`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 211
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `8b35e935e308282b`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 212
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `8bd1a30ed7bb922b`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 213
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `66b577d37b8eac92`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 214
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `f57dd22a941d9b91`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 215
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `eea0d9c15dd19502`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 216
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `8463ff436b5024cd`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 217
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `bf0f4cc189ed9c44`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 218
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `217547e974f2fa95`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 219
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `3c2faf60209549a9`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 220
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d70bea0735b219eb`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 224
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `106111363c04e36b`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 225
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `285626100021482c`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 226
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0218fbd0bf11a0ab`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 230
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c57990e75d8442e1`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 240
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d896f2de076f3d87`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 250
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `0f7b48f3e773a37f`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 251
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `b1a22a2a15a40ca6`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 252
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `9077c236ff0634fb`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 253
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d0f93a5fc002fb98`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 254
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7b4fd94759b08c56`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 255
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `5a747f46b4c0b811`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 256
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c2b6c1ce6c5aca39`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 257
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `f7da06ddbf2e8053`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 258
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `68b58256f0971d1b`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 259
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e39d6110272646d7`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 260
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `f5525b5b2b3ff886`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 261
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7a1df22d6226505a`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 262
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### High cyclomatic complexity: 76

- **ID:** `0d1979077828acc8`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Nested loops detected - potential O(n²) complexity

- **ID:** `6a266fa6db270e85`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 26
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `ea3bbdf8bd70106a`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 46
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `6087ee21428fe89d`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 59
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `961404f7af4a2bbe`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 60
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `e1a19a5a30091977`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 78
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `6345c7e07e4d263e`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 80
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `d4741744f2b804a6`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 200
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `23acd88a19d63320`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 251
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `17ba8138c0645e37`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 269
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### FIXME comment found

- **ID:** `9f610314997ea23a`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 272
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a FIXME comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### HACK comment found

- **ID:** `a19b127cb093fe2c`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 273
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a HACK comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### FIXME comment found

- **ID:** `1fed9a741d29d827`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 275
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a FIXME comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Nested loops detected - potential O(n²) complexity

- **ID:** `6eff0f7446cc95de`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 330
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Inefficient collection operation in loop

- **ID:** `ddbbcc6bb93b0072`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 330
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient string concatenation in loop

- **ID:** `3fcd8b4c7c0afdb8`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 351
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Debug statement found

- **ID:** `6d1f6abd086d76ca`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 387
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `22233e1f99ad0b6c`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 388
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `57f1a53c0f8c6d26`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 389
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `46d9c625be33d15c`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 390
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `051800221f508ae3`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 391
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `2e603c4854b43fef`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 392
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `00accd7e7632e725`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 393
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `4abff7319805b672`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 401
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `0aee8f37723143b8`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 402
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `d1b6615c98d2fe1b`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 403
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `18ea5b34a6814577`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 413
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7be8b0f691fd6d79`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 415
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `a59f7115791ed9a5`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 423
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d432b320917cd8ff`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 425
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `32c70a0b1273f27f`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 68
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1408206ab1ee9d0a`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 116
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `59cdf2cfad08ec52`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 257
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `276f63386f4da951`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 263
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `10c767503dcacc4f`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 267
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7d40a29fe2f76f66`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 277
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `3f39277f180ab43b`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 283
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `eaa15af6ac1bd37e`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 297
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `85049cc35e6e454f`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 307
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### High cyclomatic complexity: 29

- **ID:** `df6a3f8f50d080ea`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `18cb392916e367de`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 5
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1edcdc24e4a55ad9`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 45
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'performance_characteristics' is too long (52 lines)

- **ID:** `0d9a4eae1814045e`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 55
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Debug statement found

- **ID:** `d5831efc43197b1c`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 109
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `b6c0b50f5512dfdd`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 117
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `186691e47a55d3ac`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 124
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `b324a912b92b7f78`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 128
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `40e5c88aaa8a563d`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 131
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `f83b0ac0b8410f96`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 138
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `16e381d50cf6c332`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 152
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `18c2aa8c56ceac4a`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 161
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `7e02aa8eadc79684`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 195
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `50473138c98d172a`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 246
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `38cc2125cbb9e5fe`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 274
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `b3e386d34a06caf5`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 275
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### High cyclomatic complexity: 46

- **ID:** `916e240394b87644`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `0a447e985bf11176`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 4
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2f0a2a059ebd51bc`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 12
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1bd9c1eeaa0629bf`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 15
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `998b5c6c151bf76c`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 21
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `42bcc708c2c5f4ea`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 34
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7ad46fbc20b165da`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 49
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `f52da9a2924b8ffc`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 80
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `30c9ab82df667bad`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 91
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `02caca2805a993ef`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 103
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `e6c67d18c7c41d53`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 108
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `d876f7c261d02786`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 110
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `3cb558a18dc96d41`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 111
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `123e747328a9c0e9`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 122
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `9ac598c679df6793`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 161
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `220dcaa88dc514e5`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 164
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Err' has moderate complexity (8)

- **ID:** `27d5c201223585b0`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 253
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'get_recommendations' has moderate complexity (10)

- **ID:** `6ac6b104d0dade65`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 276
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2c33ad763229cb1b`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 323
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `5153b4bc59763885`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 329
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### High cyclomatic complexity: 35

- **ID:** `ee2a928d372fa48b`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `28635a316a90614e`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 9
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `976f9d278e6ba5ad`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 38
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `7574c41cfa9c6077`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 45
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `5748f94d2e552566`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 66
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `1eb64b89ac7b780a`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 92
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `ea2df7c0b77c1ea6`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 99
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `fb7a61f80fb4eb0a`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 118
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `299899cee4852c0d`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 119
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `5ee176e29974c917`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 134
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Unsafe block detected

- **ID:** `34d4a20506aa1e4e`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 138
- **Analyzer:** security
- **Rule:** unsafe_block

#### Unsafe block detected

- **ID:** `180b9ac287a475da`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 149
- **Analyzer:** security
- **Rule:** unsafe_block

#### Debug statement found

- **ID:** `3763682ce2e9034e`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 161
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `e04d8cab454edd09`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 182
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `fd9976b0af24f628`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 186
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `b7c79b8d2f0d8aa8`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 188
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `09abb5ab6d32d5cb`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 190
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `7df7e9eb4fd3b058`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 192
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `73dc721b514f5aa3`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 196
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `92e7d359e25f053b`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 214
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `806368c93d90a9c3`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 218
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `caeee6a4665cc35e`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 220
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `d0e5db5ee250a430`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 222
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `0cc083dc60590a44`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 243
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `f3aa26e1591b7b43`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 247
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `779ec0681116f288`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 249
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `20e85bbeecf452e3`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 251
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `79ca122381385cef`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 253
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `ebd6e6d4437fa075`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 255
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `3b9c2e22cbe7368d`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 272
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `5813a267fc6cbbc4`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 276
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `ba12c28c50d24dff`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 278
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `e70a399aa150d078`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 280
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `ac771649f73b06a5`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 282
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `2dbcd8fa7256cca3`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 304
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'Some' is too long (51 lines)

- **ID:** `aa8e09ff42f122b5`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 338
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' has moderate complexity (9)

- **ID:** `f92066143b295615`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 391
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'Ok' is too long (52 lines)

- **ID:** `e24ebfcc3d7f3a93`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 391
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'minimal' has moderate complexity (8)

- **ID:** `fe0c2cc79a7bdd2e`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 395
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### High cyclomatic complexity: 42

- **ID:** `0ecde9ed7f026488`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Function 'Ok' has moderate complexity (8)

- **ID:** `b75fdae48b61257e`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 86
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'analyze_files' has moderate complexity (10)

- **ID:** `a90036d1d0ee43e0`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 136
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'analyze_files' is too long (90 lines)

- **ID:** `0cd8ba3f7e9d79be`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 136
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `f6154da0252c035e`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 168
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### unwrap() usage detected

- **ID:** `fcef0d805aa83952`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 187
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `685ee70b8dbff987`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 191
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `7457195d34fa811b`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 208
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `63ede7491539c3ce`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 221
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Nested loops detected - potential O(n²) complexity

- **ID:** `5c6f20df4cd97a95`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 236
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Deep nesting detected

- **ID:** `2befe4636bb6b421`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 322
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `c5a57d2b2f23593a`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 323
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### unwrap() usage detected

- **ID:** `f184cb909556c806`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 323
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Deep nesting detected

- **ID:** `72cbd90bec44e581`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 324
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `6b4409eda923840c`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 325
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `75700cb86ef62869`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 326
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Debug statement found

- **ID:** `2c0d5757480257e2`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 334
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'analyze_single_file_optimized' has too many parameters (6)

- **ID:** `dc0bf645185f7ab3`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 343
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Nested loops detected - potential O(n²) complexity

- **ID:** `883174c399912aba`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 364
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `48744c169295c03e`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 386
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `4e37f2d47c39c773`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 397
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2c6121c8f5b78bc8`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 415
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `688f96f7e2ed9ae6`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 445
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `95790a26bab542b4`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 8
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `3c9bb82ea606fa2d`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 9
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `b4abe55ed96aa506`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 77
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `555a34a98b8840c8`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 183
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `e44934e4191ebbed`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 192
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `a21da2a6d8f2ab2d`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 201
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'create_or_update_issue' has too many parameters (6)

- **ID:** `9a620923dfa25b32`
- **File:** `/workspaces/do-codeguardian/src/github.rs`
- **Line:** 5
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### High cyclomatic complexity: 28

- **ID:** `ee7d8e807c395716`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `f4376f125f3d0a93`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 12
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `dbcb4d28f0c8c5cc`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 20
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'Err' has moderate complexity (9)

- **ID:** `54ec51be8464e4da`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 57
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Debug statement found

- **ID:** `8dd67303ce85d5c3`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 67
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `77871c5adc23d211`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 71
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Deep nesting detected

- **ID:** `6a6ef23b5d29da5d`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 72
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `95c307b4a4e2d1be`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 73
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `0feeaa01d8078f27`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 74
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Function 'sleep' is too long (123 lines)

- **ID:** `6cb57231fe5c4241`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 76
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Err' is too long (121 lines)

- **ID:** `e5af0e9bd6c4c715`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 78
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'create_issue' has too many parameters (6)

- **ID:** `01aad975dcdca3b5`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 144
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'update_issue' has too many parameters (6)

- **ID:** `1f41801a5d951b28`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 176
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'wait_if_needed' has moderate complexity (8)

- **ID:** `7059298d5199e252`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 210
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Debug statement found

- **ID:** `2c42d9f283481077`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 223
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `8557ab743b276a56`
- **File:** `/workspaces/do-codeguardian/src/main.rs`
- **Line:** 45
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### High cyclomatic complexity: 28

- **ID:** `ef3353823e7998d5`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `4d29cea3eb3ecf7e`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 18
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `95163c5757b48a4d`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 98
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Ok' has moderate complexity (9)

- **ID:** `c52608d1098acfb8`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 117
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'Ok' is too long (58 lines)

- **ID:** `a1c8a4fd1c347c5b`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 117
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'train_batch' has moderate complexity (9)

- **ID:** `c95b50719a1bb7eb`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 121
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'train_batch' is too long (57 lines)

- **ID:** `f4db0036fd896b01`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 121
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `5fb8981e371c5159`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 130
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `5df87cdeb578981c`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 135
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `a0ecaa168b632993`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 138
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `6587a68c31281249`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 156
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `817690e4790afb81`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 167
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'adjust_learning_rate' has moderate complexity (8)

- **ID:** `4f344dcba8879edf`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 180
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Debug statement found

- **ID:** `29e444423ce4c6f2`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 226
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `93116a1c06f67674`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 235
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `21cb3d3a0fed52d0`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 258
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1b961401b51de995`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 259
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `82574cbab10c5c32`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 263
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### High cyclomatic complexity: 47

- **ID:** `265538299af66786`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Nested loops detected - potential O(n²) complexity

- **ID:** `4e21c9993d7e12fd`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 11
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'extract_features' has moderate complexity (8)

- **ID:** `779b217d7f4c4101`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 48
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'rule_specificity_score' has moderate complexity (9)

- **ID:** `fa719c69e74ab708`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 136
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `f75cd28c4ee20c28`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 198
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `86ac85e3cd40f2a4`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 229
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `ab280d15392e9ec2`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 234
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Some' has moderate complexity (9)

- **ID:** `140b17e2f08385b0`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 245
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2d42900253e80861`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 247
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `0a672a9a5131a2bc`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 251
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `3960c78175d32da3`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 260
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `74413c504d403b1f`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 263
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### unwrap() usage detected

- **ID:** `9a60191c3ed824b9`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 294
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Large file (836 lines)

- **ID:** `da38714a248bed38`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 1
- **Analyzer:** code_quality
- **Rule:** large_file
- **Description:** Large files are harder to understand and maintain
- **Suggestion:** Consider breaking this file into smaller, more focused modules

#### High cyclomatic complexity: 61

- **ID:** `ba8625bda3ecb37f`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `da234938fe044db9`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 8
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `2063f600a35e799d`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 28
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `fb4b68cccc09f2b0`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 49
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `b9bfe23757241a6a`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 67
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `630b65168138cf1c`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 95
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `6ab121c2deac034c`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 104
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `585acc0efa98f276`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 113
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `aed53222c312666d`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 129
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `12eaba0de62ac8b4`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 139
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `9aa6b9a1141ac7b3`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 149
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `ca6a018a96e8b579`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 163
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `7f8b755a81f72af9`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 168
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'record_inference' has too many parameters (6)

- **ID:** `68899ab92b96420a`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 202
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'record_training' has too many parameters (10)

- **ID:** `f6bdd49715031e7c`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 240
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Function 'generate_report' is too long (79 lines)

- **ID:** `aa0ed29209607adf`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 282
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### unwrap() usage detected

- **ID:** `3969f381ac9a5aae`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 387
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Function 'update_classification_metrics' is too long (53 lines)

- **ID:** `ef99f90cc8bf90d6`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 399
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'update_temporal_metrics' has moderate complexity (10)

- **ID:** `a3106a4787dd96bd`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 506
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'update_temporal_metrics' is too long (53 lines)

- **ID:** `bd81975d14261306`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 506
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### unwrap() usage detected

- **ID:** `0cf719e63e6efdd1`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 521
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `1c8873a67aa46611`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 541
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `4f327607dd4d04d4`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 543
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Nested loops detected - potential O(n²) complexity

- **ID:** `9471bbb34a551c9e`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 580
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `3c2ea22ba74a555d`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 594
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'add_alert' has too many parameters (7)

- **ID:** `5db0c3c563bb502b`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 609
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Nested loops detected - potential O(n²) complexity

- **ID:** `0db24e8114ef965b`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 662
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `0c309569918b0933`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 786
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `9490b02900c67408`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 802
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### High cyclomatic complexity: 30

- **ID:** `a7c20fa6848c0fd5`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Function 'Some' is too long (65 lines)

- **ID:** `c7dcd8facf36d5da`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 33
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'predict_relevance' has moderate complexity (10)

- **ID:** `31e3663a56f22336`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 50
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Function 'record_training_completion' has too many parameters (10)

- **ID:** `2dee1f9e961ae7a5`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 160
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### High cyclomatic complexity: 29

- **ID:** `e6dcd96281647176`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `c95c647a749e2ea2`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 6
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `3b6b1e4a7f3ccf56`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 13
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `5bffd535068feceb`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 22
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'add_feedback' has too many parameters (6)

- **ID:** `f0bcf956633a752d`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 60
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### Nested loops detected - potential O(n²) complexity

- **ID:** `1239e81c3c43ecde`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 94
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `9730f7393878b8c7`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 137
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `9375a72f7059c5ff`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 191
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `f1d0e29909798e64`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 208
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `895bda8ac5ad44a4`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 240
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `e8661945be92a91c`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 256
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `0612bc908e81ed3a`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 288
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `7f90302cc2d2f3ee`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 329
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### High cyclomatic complexity: 27

- **ID:** `5d4824894107930d`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Nested loops detected - potential O(n²) complexity

- **ID:** `715b94d4df5010de`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 10
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `4a4e807e955ff002`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 20
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `dd94c5c74157600f`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 30
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `e52450f0d841586f`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 33
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'analyzer' is too long (88 lines)

- **ID:** `8eb549f7b533af55`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 175
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Nested loops detected - potential O(n²) complexity

- **ID:** `0796ead750057adb`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 202
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'callback' is too long (53 lines)

- **ID:** `c45c7344f5e060e2`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 210
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Blocking I/O operation detected

- **ID:** `1532ab130b631fa3`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 234
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Function 'test_performance_engine' has moderate complexity (10)

- **ID:** `1a1f7438650f264b`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 327
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### unwrap() usage detected

- **ID:** `63164b389805373a`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 328
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `f45b1dc2d241c93f`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 333
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `1bcf48ccfd28cbf8`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 341
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `ff8024e8927777a0`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 366
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### High cyclomatic complexity: 31

- **ID:** `35cb30ae4db629fc`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Inefficient collection operation in loop

- **ID:** `3750388ecf4481b6`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 5
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `bb599e6a6b4f440e`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 160
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Inefficient collection operation in loop

- **ID:** `a20c8612b9a6de2a`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 230
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Large file (593 lines)

- **ID:** `da26bfad6f0d8856`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 1
- **Analyzer:** code_quality
- **Rule:** large_file
- **Description:** Large files are harder to understand and maintain
- **Suggestion:** Consider breaking this file into smaller, more focused modules

#### High cyclomatic complexity: 59

- **ID:** `c4746fb8659953c7`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Nested loops detected - potential O(n²) complexity

- **ID:** `bbabd940498bc20b`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 17
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `3a0a9cbaf5233aaf`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 18
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `0a51ae2334e6f88e`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 102
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `65d9bb65e12d7c26`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 112
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `c059f2fe2dd78ed5`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 153
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `adc7f954100df709`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 190
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `d7419991624a79ab`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 235
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `76d7dea010d35a94`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 266
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Function 'Ok' has moderate complexity (10)

- **ID:** `5bd733e66a1610b7`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 266
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Blocking I/O operation detected

- **ID:** `57dd37ff7196f011`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 357
- **Analyzer:** optimized-performance
- **Rule:** PERF-OPT

#### Nested loops detected - potential O(n²) complexity

- **ID:** `96774a2f9cfec22a`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 388
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `2d41398f2459adc8`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 410
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### unwrap() usage detected

- **ID:** `b2515f13054e4d8b`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 416
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Nested loops detected - potential O(n²) complexity

- **ID:** `bca0dfae6ae723c5`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 438
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `c0a00e7147c7af68`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 444
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `9c5359292d686566`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 447
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `faef733fd459eaa8`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 467
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `392a7fbde681e9b6`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 476
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `5b6b4c0b74d3d583`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 485
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Deep nesting detected

- **ID:** `0500d7c16c1e11cb`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 529
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `5c2c5588c382e7cb`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 530
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### Deep nesting detected

- **ID:** `b22207bd71586628`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 531
- **Analyzer:** code_quality
- **Rule:** deep_nesting
- **Description:** Deep nesting makes code harder to understand and test
- **Suggestion:** Consider extracting nested logic into separate functions

#### unwrap() usage detected

- **ID:** `b40ccfbb4cdc1b99`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 552
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `5a0c4cfee3caa17b`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 556
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `af13fe80bf83463f`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 575
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `ae27a5c5566e2555`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 21
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `dc18a9dfac4bed55`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 22
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `1eeee3d60ac3ccb5`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 23
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `5567e4ceb62007fd`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 28
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `3e7eb995e8f536a3`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 35
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `262a0046bc744ee1`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 21
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d2929f7983565b1a`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 31
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `187b02d2fec9e94a`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 39
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `d0db4133837c2088`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 54
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `859af34a70739493`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 63
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'new' has too many parameters (7)

- **ID:** `3cb7b9bbeb38f147`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 133
- **Analyzer:** code_quality
- **Rule:** too_many_parameters
- **Description:** Functions with many parameters are hard to use and test
- **Suggestion:** Consider using a struct/object to group related parameters

#### High cyclomatic complexity: 20

- **ID:** `46e55b379fa3c829`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `99bf6b38fddc20bf`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 8
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `62217cc1b70129dd`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 94
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `689a7a23296fb50c`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 146
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `a7411cd076e731a4`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 155
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `d60959b06c7a3c01`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 189
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `467994191e1eb244`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 236
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'Ok' is too long (58 lines)

- **ID:** `0913981458c3f771`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 241
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### High cyclomatic complexity: 31

- **ID:** `f984b9b9ee81ed0e`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `7a7b478349dd54ae`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 59
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `0f57ffe4e3bb25e0`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 82
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `252b00521b7ebef7`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 147
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `d51acb0f973296d2`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 163
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `3ff5a59a9571b512`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 180
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### High cyclomatic complexity: 28

- **ID:** `9e1b6d04c6025238`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 1
- **Analyzer:** optimized-quality
- **Rule:** QUAL-COMPLEXITY

#### Debug statement found

- **ID:** `74a80beabaa9aecd`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 7
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `fc28290404c02909`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 23
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `37e1f88b55a0db11`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 31
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Nested loops detected - potential O(n²) complexity

- **ID:** `808be5a29e60bd76`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 50
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Nested loops detected - potential O(n²) complexity

- **ID:** `75fecdc053512054`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 61
- **Analyzer:** performance
- **Rule:** nested_loops
- **Description:** Nested loops can lead to quadratic time complexity and poor performance with large datasets
- **Suggestion:** Consider using more efficient algorithms, hash maps, or breaking early when possible

#### Debug statement found

- **ID:** `6006dcde8d13e9b6`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 69
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `c0cbda7a6514fc30`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 79
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `86d30984807bb4d1`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 90
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `2cd55a977251c2a2`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 110
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `f15bf0082d21c233`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 126
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `225832a73ccd7262`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 162
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `666e741c49b6163d`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 163
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `a2a013fca0d4cd16`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 171
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `b85e284388e5a121`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 181
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `92143b8139e61592`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 183
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Function 'check_performance_thresholds' is too long (61 lines)

- **ID:** `6882d9b4aba14050`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 196
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### unwrap() usage detected

- **ID:** `e4bbbb415bf78e97`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 259
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `2c13cb226a9f59f3`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 269
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `a8c28d9c30c5567a`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 296
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `fc8d703318de84bb`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 332
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `a6291420cca0656f`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 346
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `42be416e79e35a6f`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 350
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `04a0149a1d01ad73`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 351
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `b7e963bafcd6a7bc`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 352
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `ad96ea7c72373652`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 353
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `bbf34d70c650b1af`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 354
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `5ec09c990aa9e4cb`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 355
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `28d354df579a4468`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 356
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `91e6751ada298c16`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 357
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `8ac504556d01a669`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 358
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `a2f6f3599a234d7c`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 361
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `a557240be895f885`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 369
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `86b3115fa96e1499`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 372
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `95eade4663d6822e`
- **File:** `/workspaces/do-codeguardian/src/utils/progress.rs`
- **Line:** 29
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `86876e2b5969ac89`
- **File:** `/workspaces/do-codeguardian/src/utils/security.rs`
- **Line:** 10
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `0e120d7513b3f258`
- **File:** `/workspaces/do-codeguardian/src/utils/security.rs`
- **Line:** 11
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `763a2f1a4862dee3`
- **File:** `/workspaces/do-codeguardian/src/utils/security.rs`
- **Line:** 12
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `3271082ed6750b91`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 30
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `0b9f1abd8e3e495d`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 40
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `4e9d44e5c27206c3`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 45
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `e8835895d77614ba`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 48
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `0eb6ac4a21237789`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 58
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `b26429005f3d898b`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 66
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `28cb774b03bef386`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 69
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `645cb19f2a4fbd58`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 77
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `bac5dadb2936e778`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 96
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `48096f70bca084fa`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 101
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `377802514367a4a3`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 107
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `66effa0ecdfe315c`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 115
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `41ce7c62c4156a2d`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 117
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Function 'Ok' is too long (165 lines)

- **ID:** `b9698221bd5ed87c`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 123
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### Function 'Ok' is too long (156 lines)

- **ID:** `dd7a298f7e6b3321`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 132
- **Analyzer:** code_quality
- **Rule:** long_function
- **Description:** Long functions are harder to understand and maintain
- **Suggestion:** Break this function into smaller, more focused functions

#### unwrap() usage detected

- **ID:** `3812c70b571e986f`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 136
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `ad1b29fd3469b5d5`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 145
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `6ee9b6720c300aba`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 168
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `17bee6a79e8141d7`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 183
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `063a4efa4720521f`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 192
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Function 'test_performance_config_integration' has moderate complexity (9)

- **ID:** `3338f3f8a66a3559`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 196
- **Analyzer:** code_quality
- **Rule:** moderate_complexity
- **Description:** Consider simplifying this function
- **Suggestion:** Look for opportunities to extract helper functions

#### Debug statement found

- **ID:** `ba5dbf2a6463057b`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 225
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `7bad8ff64d94e7ec`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 236
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `82d96a2663a8ca9e`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 245
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `d09aa7ba30b5b29e`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 253
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `2b261a955ee205a3`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 254
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `96870e03ffdeb158`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 258
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### Debug statement found

- **ID:** `bbe8bbdf68c20cc0`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 265
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `69d9b4683e5206ba`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 266
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `08b9c68f3711725e`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 267
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `51c1cf0411e00932`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 271
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `712e854f4b21c76e`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 272
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `0c752a4ff7a50b80`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 273
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### Debug statement found

- **ID:** `cb7916100b58d0fb`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 285
- **Analyzer:** non_production
- **Rule:** debug_statement
- **Description:** Debug statements should not be present in production code
- **Suggestion:** Remove debug statements or replace with proper logging

#### unwrap() usage detected

- **ID:** `72691952a1fce0c6`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 290
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `0d4384bf160203eb`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 291
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `0edbcfe9651a855f`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 299
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

#### unwrap() usage detected

- **ID:** `b5aba515fac77c89`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 300
- **Analyzer:** code_quality
- **Rule:** unwrap_usage
- **Description:** unwrap() can panic; consider using proper error handling
- **Suggestion:** Use expect(), match, or the ? operator for better error handling

### 🔵 low Issues

#### Magic number detected - consider using named constant

- **ID:** `4e2aadde3b701b4a`
- **File:** `/workspaces/do-codeguardian/.github/SECURITY.md`
- **Line:** 33
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `648eafcbce3ca753`
- **File:** `/workspaces/do-codeguardian/.github/SECURITY.md`
- **Line:** 33
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `5927948dc7854e9f`
- **File:** `/workspaces/do-codeguardian/.github/SECURITY.md`
- **Line:** 34
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8f4c9a2d0911d996`
- **File:** `/workspaces/do-codeguardian/.github/SECURITY.md`
- **Line:** 34
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `13983883e6e36eef`
- **File:** `/workspaces/do-codeguardian/.github/SECURITY.md`
- **Line:** 35
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `dbf11d2e1a7712bf`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 46
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `cb8a7c58f7a02f28`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 47
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7872edf8b5b1c1be`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 48
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `2d4209d87daff322`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 58
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7ecd184cc051f18a`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 59
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `425ee7e41c8f3d84`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 60
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `ac448b8a24887562`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 66
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `af1f0121e128dd18`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 69
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7f7f47eda6c7bead`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 70
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3a3d12915120306b`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 71
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c1bf6ba64b7955c9`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 117
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `5dd6a5d000f6a0c4`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 118
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `551b86cb07eaef44`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 119
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3a05ba932057c3c4`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 137
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `f4bddac52ca4ab08`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 232
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7d2030338d933641`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 233
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `353a523f842058ac`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 234
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `6377132ff6a86938`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 235
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `b363039f4734d00a`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 239
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `f7ad398dacdaa081`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 240
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `9b6f8996e220d736`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 241
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `fbc70e85bd5e34a5`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 242
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c86df015bbb04d0c`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 248
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `16b7575bf6950725`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 251
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `d0c598c461845fb0`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 254
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `f5108e8970660f59`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 260
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `6ebff3ea114fd315`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 268
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `5cea789c7c287a3b`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 270
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `bcd86e98096f348a`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 273
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `2ac32c2587922771`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 278
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `a3d33065f5f202fe`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 294
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `f4a082bb64aa1ed6`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 332
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `9992fb387733f030`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 333
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ae2c0352ad34eca7`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 334
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ca0440a8b3d5310a`
- **File:** `/workspaces/do-codeguardian/.github/TURBO_CI_GUIDE.md`
- **Line:** 335
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `d4e22a7595bd9a03`
- **File:** `/workspaces/do-codeguardian/.github/gh-labels-creator.sh`
- **Line:** 4
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `a75d94cfab8bb99f`
- **File:** `/workspaces/do-codeguardian/.github/gh-labels-creator.sh`
- **Line:** 15
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `d93f855c9e9b1252`
- **File:** `/workspaces/do-codeguardian/.github/gh-labels-creator.sh`
- **Line:** 25
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3912bafdcd36d2ae`
- **File:** `/workspaces/do-codeguardian/.github/labeler.yml`
- **Line:** 4
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c1fe953f98083cc5`
- **File:** `/workspaces/do-codeguardian/.github/labeler.yml`
- **Line:** 13
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `6855afd00802bb1e`
- **File:** `/workspaces/do-codeguardian/.github/labeler.yml`
- **Line:** 36
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c58af24e3ec3b14c`
- **File:** `/workspaces/do-codeguardian/.github/workflows/codeguardian-ci.yml`
- **Line:** 9
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c42ad19da23244f0`
- **File:** `/workspaces/do-codeguardian/.github/workflows/codeguardian-ci.yml`
- **Line:** 62
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e33376d3798a0f56`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-nightly.yml`
- **Line:** 6
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `d15abb07ad09fee2`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-nightly.yml`
- **Line:** 54
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `81b62a967b0c95c6`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-nightly.yml`
- **Line:** 55
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e71405eb4c84a9ad`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-nightly.yml`
- **Line:** 56
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `568d6c9c4de3323f`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-nightly.yml`
- **Line:** 62
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `25cf7b7079f682b8`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-performance-monitor.yml`
- **Line:** 5
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c37ffb148a9552a3`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-performance-monitor.yml`
- **Line:** 68
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ad27e5e85f813a05`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-performance-monitor.yml`
- **Line:** 95
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `bbb50d6e6118e433`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-performance-monitor.yml`
- **Line:** 164
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ee244894f4a5ee1c`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-performance-monitor.yml`
- **Line:** 178
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e7c754fb3655a443`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-performance-monitor.yml`
- **Line:** 190
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `b8c7db09803e49fa`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-performance-monitor.yml`
- **Line:** 213
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `19a26482d7c97705`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 33
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `f796113cd5b902e0`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 34
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e04559c0baea286e`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 36
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c597d022fd8ce2c3`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 48
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `2dbaf50fdb5795e7`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 84
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `994a096cfecef497`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 113
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `100e412e43b0745b`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 150
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `4e934003dc6ec4a9`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 153
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1968743143be77fb`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 154
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ad09a4e8413896f9`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 155
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `0f47b148d8a980af`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 156
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c4979a054b4b0b75`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 173
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ebb69d283fee4796`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 174
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `08d5145dd3e24227`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 175
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `452bd144cb7a12cf`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 176
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `da9dab4f128d4557`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 177
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `9d4e591803f8443f`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 178
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `449cf8a76de3bc86`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 187
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `364d36f947e10b83`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 188
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `00251edd1233bc29`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 189
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `0c5061df4651769c`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 190
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `9c6e84cb32c9f784`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 195
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `4935a7e06d2f2d82`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-pr-analysis.yml`
- **Line:** 214
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `5b9b8767ffa8e17e`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-release.yml`
- **Line:** 72
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `df4aadcf0546c374`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-release.yml`
- **Line:** 81
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `6c05a05fee9b0d91`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-release.yml`
- **Line:** 127
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `a0ef2d4a4b3b0824`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-release.yml`
- **Line:** 135
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `db5272abea238fcf`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-release.yml`
- **Line:** 138
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `f15b884dc427ae23`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-release.yml`
- **Line:** 140
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `73a497c66436af08`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-release.yml`
- **Line:** 146
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `bd52dd21b688856b`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-security-analysis.yml`
- **Line:** 9
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `26227200106cdbbc`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-security-analysis.yml`
- **Line:** 57
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `82d5f61b306c5fa1`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-security-analysis.yml`
- **Line:** 58
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `40c9fda00877cadc`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-security-analysis.yml`
- **Line:** 60
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `189c8a4191ce0273`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-security-analysis.yml`
- **Line:** 79
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `b42428aa6a48cbf4`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-security-analysis.yml`
- **Line:** 96
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `d995c6319907d103`
- **File:** `/workspaces/do-codeguardian/.github/workflows/turbo-security-analysis.yml`
- **Line:** 130
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Naming convention violation detected

- **ID:** `5ece25d9d9188736`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 10
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `6422061a91a84bd1`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 10
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `bead6c4b321776de`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 33
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ff041d5da72bfdeb`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 34
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c849a816f7ab2c49`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 35
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `4f361820ae29a428`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 36
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8397c3176186106a`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 37
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `9b0c566c98db1f2f`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 38
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3410d5ba7adf49da`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 39
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e0898b0b699fbf9f`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 40
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8ecfd2bae11e9207`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 41
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7a0fb2f85ee6df99`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 43
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `14b58becb88806ce`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 44
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `cabc20983b4753ab`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-consolidator.md`
- **Line:** 45
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `3559f4ea73d9bc18`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-research.md`
- **Line:** 66
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c1e9d0c37283c702`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-research.md`
- **Line:** 93
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `640a8977b51a648b`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-research.md`
- **Line:** 94
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `5c95964e10a04bd7`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-research.md`
- **Line:** 95
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `5986b36446295e71`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-research.md`
- **Line:** 96
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `86fb7fea8bac189b`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-research.md`
- **Line:** 97
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `98a39a15eef79267`
- **File:** `/workspaces/do-codeguardian/.opencode/agent/code-research.md`
- **Line:** 98
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `04a2a6a742fa0c9f`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 8
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `81da2c8056c7ea74`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 9
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ff9d065aa7027c2b`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 10
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `9074d1132af9ae94`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 13
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c96017311b2061c4`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 14
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1c6c06671c33270a`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 15
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c0c60abf99bb50a8`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 16
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8493ed7bb468ff31`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 19
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `edea44d2db77bd76`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 20
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `d042da52601508f4`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 21
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `f2b42c3c1cf534ef`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 22
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e581a29964983699`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 38
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7408a9dd03f8c237`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 39
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `77a9088e3511e882`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 40
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8a81c979fecd57f4`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 41
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3d5d5a1d7cebec42`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 107
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c39ab7ce75e15821`
- **File:** `/workspaces/do-codeguardian/AGENTS.md`
- **Line:** 156
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `cf7e4a7d2a384ae9`
- **File:** `/workspaces/do-codeguardian/Cargo.lock`
- **Line:** 284
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `691f433da14a1709`
- **File:** `/workspaces/do-codeguardian/Cargo.lock`
- **Line:** 294
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `4c261c172d75c5bc`
- **File:** `/workspaces/do-codeguardian/Cargo.lock`
- **Line:** 306
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `55534b5fd3831c30`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 8
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `839651467a98d8b8`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 8
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `671d47c1a722f189`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 9
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `eaa0de9e3e95c1a3`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 10
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ba47613f6ab0f7ec`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 11
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1a49161c2363975c`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 14
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `411727aaf83a8e85`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 15
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `49212c7a0ecbcd63`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 16
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ff843962ff40b254`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 17
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `79a775522e483afd`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 20
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `73a536a33df4328b`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 21
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `78873eb181f5e0db`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 22
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `f06bbeb2e8e21a85`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 25
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c4bc49129c6a0b72`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 26
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `cab814e348675da7`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 27
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `dc13b77dbc2c375f`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 28
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `570408e6f2035820`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 92
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `b2562651a27eddf1`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 122
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `46c14cf2b9e9b648`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 231
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `2685e54d537bb18a`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 232
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `a43fd9c7f13013fb`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 233
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `83085546a0216bcd`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 234
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3dc18f2167a14319`
- **File:** `/workspaces/do-codeguardian/README.md`
- **Line:** 235
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e885fea2c1669db8`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 6
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c1f1a7121b040ea9`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 31
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `2c8cf4033979adeb`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 32
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `16ac313fa1a57176`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 33
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `11f0c077e756ef9b`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 34
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `a5a52846fbc12da5`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 35
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `76967b54d1d7ba82`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 36
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `646a06560ecec42c`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 37
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `4ff10d923b2097df`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 175
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `e40d5f7e77c1f939`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 176
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `84d0a4506fb247de`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 207
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `2080a7d6683e7d96`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 214
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1a9329fcc6c62fdd`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 276
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `a345090c5abfe32a`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 277
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `460da8e29643e817`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 278
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `fd977a56400716f5`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 279
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3a93471a957eeefb`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 280
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `dde2326da3875704`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 281
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `127aa8968c030a42`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 282
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `5b30eb9ce7e1e35d`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 293
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3ccd20f1124e68ac`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 295
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e8dcf605eff04e5f`
- **File:** `/workspaces/do-codeguardian/RUV_FANN_VS_BERT.md`
- **Line:** 297
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Import statement after non-import code

- **ID:** `f7faeada97d29005`
- **File:** `/workspaces/do-codeguardian/benches/hashing_benchmark.rs`
- **Line:** 16
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `159a8efec3512b0e`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 13
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `ff2435029c1f1622`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 14
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `a7cae09a86ee344c`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 15
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `2a0ffc29aca9101d`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 16
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `4c796cefeea0822b`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 17
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `f9777ca66a01c86f`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 24
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `668f70e099c01ffb`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 32
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `519735f7a5ea08b8`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 33
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `1a1cfe02941514bd`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 34
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `88dfd08796b7f0d3`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 43
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `7f8472e3862b9615`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 60
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 4 times

- **ID:** `4c9c4a1d9591ac44`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 74
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `e66475b165fdd509`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 115
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `8c64387ac4117cc2`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 124
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Potentially unnecessary clone() call

- **ID:** `688f7259f116b14c`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 202
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `648733dbf18985d6`
- **File:** `/workspaces/do-codeguardian/benches/performance_benchmark.rs`
- **Line:** 245
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `355f4106e9517dcf`
- **File:** `/workspaces/do-codeguardian/codeguardian.toml`
- **Line:** 3
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1c2c3922dffa9bc5`
- **File:** `/workspaces/do-codeguardian/codeguardian.toml`
- **Line:** 7
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `2ca4ace5e0f2cfce`
- **File:** `/workspaces/do-codeguardian/codeguardian.toml`
- **Line:** 8
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `cfe640fcef16cf84`
- **File:** `/workspaces/do-codeguardian/codeguardian.toml`
- **Line:** 10
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `fed268f7ec4b61f3`
- **File:** `/workspaces/do-codeguardian/codeguardian.toml`
- **Line:** 42
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `35a14dc2239f2bfa`
- **File:** `/workspaces/do-codeguardian/codeguardian.toml`
- **Line:** 76
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `2df3df00db67d294`
- **File:** `/workspaces/do-codeguardian/codeguardian.toml`
- **Line:** 77
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `2be309524c0ea71f`
- **File:** `/workspaces/do-codeguardian/codeguardian.toml`
- **Line:** 78
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `d1733edc9631eaf8`
- **File:** `/workspaces/do-codeguardian/codeguardian.toml`
- **Line:** 94
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `acc580e8fa9b337c`
- **File:** `/workspaces/do-codeguardian/demo_enhanced_ml.sh`
- **Line:** 52
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `eff494eca0e07d74`
- **File:** `/workspaces/do-codeguardian/demo_enhanced_ml.sh`
- **Line:** 53
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `dfbbef8018584495`
- **File:** `/workspaces/do-codeguardian/demo_enhanced_ml.sh`
- **Line:** 55
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `722bd5d0b3adaa25`
- **File:** `/workspaces/do-codeguardian/demo_enhanced_ml.sh`
- **Line:** 56
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `32beb3e3211e698c`
- **File:** `/workspaces/do-codeguardian/demo_enhanced_ml.sh`
- **Line:** 65
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `33a7227f18767410`
- **File:** `/workspaces/do-codeguardian/demo_enhanced_ml.sh`
- **Line:** 106
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3ca8d1ae2fd94979`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 11
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `bf95ce49c69ac59d`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 21
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `d8348e1d7f645fa5`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 22
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `23852f1f96de98fd`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 23
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e41ad488acaf9bb7`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 24
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `b0b7f88711e6a6c5`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 25
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c794bd385eca6a3a`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 37
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `78c592bec5036854`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 39
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `68b9a74c6956f9cf`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 44
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `9d5d11fd8dc3a9a5`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 59
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3dfa10e20f934a26`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 66
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `17ac684d5d02ebf0`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 74
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `a830d923a8f564cd`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 79
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `b502a064a72effbc`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 94
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ce5aba3a8c084a54`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 112
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `2c8c1cffb0530dd8`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 134
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `40d5e9442104e335`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 137
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ea841b753ebc301e`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 143
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Naming convention violation detected

- **ID:** `69322a4b001787b2`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 157
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `4f9cf97a43f25dce`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 158
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7ba957853ec82105`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 187
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `25994947f0780262`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 246
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1f63e243c50c1f47`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 258
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c860e68bbdcff8b2`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 262
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `a2585f881b319bb7`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 266
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7b03f36248dfccbd`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 272
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e66370f08302068e`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 273
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `5f1b3673ecf41f2d`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 295
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `0125f0a5dd3274e3`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 302
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1383247df9c6fd43`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 303
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `859ef2e959785d05`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 304
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `99776a5d03a13743`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 305
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `356fada7fbf90ec1`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 306
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `79de6b88ca73f2af`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 310
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1882d70d9f5161ec`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 311
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ca9d6fdfb4d0b897`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 312
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `cb8a22cdad3f9766`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 313
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `b9e75d87ea94b8f7`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 319
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `a9b4cb1fba8971cc`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 319
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `d0ffd6d1f9d1fabf`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 320
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `0dd0b304ad56ec60`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 321
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1eb51e7906d612f4`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 322
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8aa6ba7321692616`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 323
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3968a3f9e0b24cc8`
- **File:** `/workspaces/do-codeguardian/docs/ML_ENHANCEMENTS.md`
- **Line:** 328
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `25bf60dd4bcc5833`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 9
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7149ad0a966343a1`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 10
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `9184bdf65b14b66f`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 11
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `aeb8f820774e5603`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 12
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `4da7eb7b55fc0718`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 54
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `825b04ea1847296b`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 55
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7e5fb41a0f11a64e`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 62
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `a0bbc584b6388b65`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 63
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `a977c0006fd7a3df`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 70
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1076e78208fbb003`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 71
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7577197809bbb5ba`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 78
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `9f68a5d3fa8230ea`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 79
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `37a9e9ec1b4412dd`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 86
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `2d1df2bdbfc610d9`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 87
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8d785aeba8e8d65d`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 115
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `40513f0516e10bb2`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 116
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ad7eca9482883257`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 117
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c5f99499352498a7`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 118
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `d6fd656d227b634c`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 119
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `785c826f6880cfb6`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 120
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8f61f79737b01389`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 121
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `a71708d2242ddfed`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 127
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e5d515136c52da25`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 128
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `843d78c095b86502`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 129
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `4d12715f46909a41`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 130
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8e790e638dd4e46d`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 131
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1114df9e2229c52e`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 154
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `516f05eb5ab43fd3`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 155
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `5398c8e9c80afdb9`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 156
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8dcba6f124d853a3`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 157
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c4bf41c8c359cfc6`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 158
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8d71596617c483c0`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 225
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `9fd2f1b0ee449cf6`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 226
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `6ad0e97ece6f9530`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 227
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7c2b06de52cbc1c4`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 228
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `90eb138c1297312e`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 247
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `b69da09bcc7529e8`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 248
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `cdf02472758d7232`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 249
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7c3c5c5747ee4884`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 250
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1818b16ca0cf0f65`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 251
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `2a5ece58183644f0`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 252
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ac720746cc9931e6`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 276
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e376e910189f0828`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 325
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `d5a889d355b358b7`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 369
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `74d7924a1b216bc3`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 370
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `b7a454c22ca644bf`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 371
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `b724d0164a94ced1`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 375
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `5ad3a0e767ef82f6`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 376
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3997bfef305fa914`
- **File:** `/workspaces/do-codeguardian/docs/PERFORMANCE_OPTIMIZATION_GUIDE.md`
- **Line:** 377
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `52a514f352872f24`
- **File:** `/workspaces/do-codeguardian/examples/ci-usage.sh`
- **Line:** 70
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `d9c5727af203e76c`
- **File:** `/workspaces/do-codeguardian/examples/codeguardian.toml`
- **Line:** 17
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `914869c69c789044`
- **File:** `/workspaces/do-codeguardian/examples/codeguardian.toml`
- **Line:** 18
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8be480b3e45f41dd`
- **File:** `/workspaces/do-codeguardian/examples/codeguardian.toml`
- **Line:** 19
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `5e64dc3a93992915`
- **File:** `/workspaces/do-codeguardian/examples/codeguardian.toml`
- **Line:** 20
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7319d82e92940b2c`
- **File:** `/workspaces/do-codeguardian/examples/codeguardian.toml`
- **Line:** 21
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `79c6a0f5bf876dde`
- **File:** `/workspaces/do-codeguardian/examples/codeguardian.toml`
- **Line:** 22
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `01faee7f12d11c69`
- **File:** `/workspaces/do-codeguardian/examples/codeguardian.toml`
- **Line:** 23
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `958e747fdaa09582`
- **File:** `/workspaces/do-codeguardian/examples/codeguardian.toml`
- **Line:** 64
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `de18d8961c5cfa48`
- **File:** `/workspaces/do-codeguardian/examples/codeguardian.toml`
- **Line:** 65
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected

- **ID:** `8830745b332a3297`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 6
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `2185e2e5944f3237`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 23
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `bfef71e07962cef6`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 49
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `f556a08c258eff35`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 52
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `a7cf53bfb5d768af`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 62
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `89aeb9d026484a62`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 79
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `9f342a28200c767a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 95
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `d194e778a6e8f9f4`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 95
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `0d8c73bbdd10420a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 96
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `3a412c0bbbf28d8c`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 96
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `c02a1f32027e03fc`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 111
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b9d650a6f0833b58`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 121
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `fa196e58085c6ff3`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 122
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3301084739932608`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 123
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `fed9cb238e82d7e0`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 136
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f1664999ee45ae30`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 137
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `01622be46b5d5669`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 139
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `c897a3a151108b4f`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 140
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `f5e76dae92fe1f84`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 144
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ae9b265bda776df1`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 145
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `335a66f19116861e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 146
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `4299bbbca68d5c9e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 154
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `daae895624ec3bd5`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 162
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `a819e7d07a3c95b2`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 182
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `4e1ed294cd65b219`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 194
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `985d6b85f79b4208`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 199
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `a4e99bf61cd85419`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 200
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `edc1d5b09689d805`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 208
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `08c4119f0b144c30`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 217
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `368f3c7bd42e33c1`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 218
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `8455fb6aff96ddf6`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 220
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `c541d97034e1dc1c`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 226
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `882dc8df76973f7a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 237
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f421284946de018c`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 241
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `32377a63d37129c0`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 245
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f57a633de55c706c`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 251
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `0c3cf128463a3ac5`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 262
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `00789b5d723d6c17`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 272
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `63af83890dae8da0`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 287
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `f23d56f536113d5c`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 287
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `d2530cde7c29a47e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 298
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `c213a5cff1394c32`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 298
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `e69d129723de0a71`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 299
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `728e78bf3cca2b4a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 309
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `5f46e0658a17e82e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 309
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `e13defd37295f1e3`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 338
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `c8fbb8ac7a59ba33`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 339
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ec9c0ab327a5fbb8`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 344
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `9fd2d68e0906b53f`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 363
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `793309fcd4264c9e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 364
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `2c744defa9d52135`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 365
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ffa72bf7e2d624da`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 366
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `0cbbbeb8dce61370`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 367
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `a4eb06c473b072c4`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 370
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `7186eaf123763962`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 371
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `46fccaff0dfb9cac`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 372
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b42e1127dcb58643`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 373
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `aad3226e8bfcb6c1`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 374
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b28925fe44ce680e`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 379
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `6f8e8bf889f08da9`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 380
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b64c0c4866f21949`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 381
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `15af1ab173cf6c18`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 382
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `246d7af9f9e96dbf`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 383
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `94727da209ac5f6a`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 386
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `8ecb9acdb2f10352`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 387
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b80f691e1fdbaa14`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 388
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `d4e44f82a34e740f`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 389
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `aea995568df14e36`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 390
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ebf342e4bf46ed35`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 412
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `a57c24a723b89755`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 412
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `2c17f537923c1554`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 422
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `07b0c44b4521c809`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 422
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `311d2c8b9b5845cf`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 432
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `317263ce57777796`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 432
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `997004f9c19681a2`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 446
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `56996ffc1d27fec5`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 447
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `376bd265ae95ae83`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 447
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `b0185ff42241f879`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 456
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `24492ef633df0903`
- **File:** `/workspaces/do-codeguardian/examples/enhanced-ml-demo.rs`
- **Line:** 456
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Commented-out code detected

- **ID:** `969e2f80196b0bb6`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 9
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `4ba8162532c3a270`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 11
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Import statement after non-import code

- **ID:** `c60308c4d1b9e4ac`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 19
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `5338053fc7729e57`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 20
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `1b070b1d47b0be8f`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 53
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `2375d73c18890570`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 70
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b072d7c6b3d80406`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 104
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `4bed6c7bfba465a9`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 104
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `33b9e9f172262813`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 114
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### TODO comment found

- **ID:** `b9329a29174f14f5`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 122
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Magic number detected

- **ID:** `9462b05bd4d7b83c`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 125
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### TODO comment found

- **ID:** `d0b85d7d1c841516`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 126
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Magic number detected

- **ID:** `c5824a7b1cf57d9b`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 133
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `96e981438fb13f21`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 133
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Import statement after non-import code

- **ID:** `8d17fb13278952a4`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 151
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `e007ef3109e4a523`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 177
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `73ab1f7d6164e02d`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 189
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### TODO comment found

- **ID:** `008a581868a87b53`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 198
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `b504f1775461205c`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 202
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `a0612219fef5c598`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 204
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `897736bd79b6a84a`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 221
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `d6e3f5a5c9dc47fe`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 222
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Magic number detected

- **ID:** `fdb540b0bd73afdc`
- **File:** `/workspaces/do-codeguardian/examples/ml-training-example.rs`
- **Line:** 225
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Permissive version range detected - consider pinning versions

- **ID:** `cc49b5ae6b152439`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 9
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `898430cd7693a4b5`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 18
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `0eb55a1cdde413b7`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 35
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `123131bd832a846a`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 39
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1077d4f8b1b32c63`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 46
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c310c542c1d3a840`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 61
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `73db23dc0b719179`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 65
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `22178a5e9dbe81c3`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 72
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3add12268e7e0e0b`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 99
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `6a2361d01d1087ef`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 110
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `06efac507cc51a5c`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 117
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `8e3e4007b610c337`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 123
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a26f173638d1cd09`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 137
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `65866b132ba7640c`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 147
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `f6c4de300aa945b4`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 153
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a84828ac2af082dc`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 211
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `131329ffa904c03d`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 237
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `937897c1dd24f1ed`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 238
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `8c2d3580941ef53c`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 238
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `ae77a9a4d70b20e2`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 239
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3a4fd6bc188d0ddd`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 239
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `4606a9ade46cee0e`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 240
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `6e9cd17640af6d18`
- **File:** `/workspaces/do-codeguardian/examples/performance-comparison.md`
- **Line:** 241
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `cda089c3f5471adb`
- **File:** `/workspaces/do-codeguardian/performance_benchmark.sh`
- **Line:** 95
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `0fbd6fdc27327228`
- **File:** `/workspaces/do-codeguardian/performance_benchmark.sh`
- **Line:** 109
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3a9c6acabac60724`
- **File:** `/workspaces/do-codeguardian/performance_benchmark.sh`
- **Line:** 137
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `0985aa2dd61c2577`
- **File:** `/workspaces/do-codeguardian/performance_benchmark.sh`
- **Line:** 139
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3a437d8bb37cc3dc`
- **File:** `/workspaces/do-codeguardian/performance_benchmark.sh`
- **Line:** 237
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `1f470ff3a30b1016`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 10
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `8eeb637882d107ff`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 11
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `675c391d0db2664a`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 20
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a20aff059ce234a5`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 22
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a832140b81a5d7a0`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 29
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `404e1ee0cd7b25bb`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 31
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ac9cfc1450e85eba`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 32
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `35bd44d566c2e325`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 35
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `6f58cc5b3050d7a0`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 85
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d7b31dde322cc4c7`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 95
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `008b55f60110fb98`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 105
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `3d74d55669007c88`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 135
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `42b3e5e9612bfc37`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 255
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d9be1e59e9b55574`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 279
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `3bf58878ad757d88`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 309
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `7aba6f291c06d0b6`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 317
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d8c1f549962b26bf`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 325
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `bc40053ba4a615b7`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 335
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `106c8de55b3fa5cd`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 385
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `8a0fbe45062ba13c`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 395
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `11e4b7d73e302ac0`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 405
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `4b11ec3aba9a6e82`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 415
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `df4c35805455adac`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 425
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `b8683022eb834a0b`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 485
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `05741d03c9df0791`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 575
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `055df45d76ece6df`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 595
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `c2898fc949a0fdab`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 605
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `4e2bdeaa7572e096`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 615
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `253c150c7834d48b`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 625
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a7236ab063fc30bd`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 635
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `073f772404f3380b`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 655
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `3674176278b628e0`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 665
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `b5f265211f43a221`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 715
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `c47fc5908e655680`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 725
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `e60b28ba4594e4fa`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 735
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `113710336cb9a629`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 775
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `3b17500c1871679d`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 785
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ade7ffaee39ed4c8`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 915
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `f5a9adf790cd53e0`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 925
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `e295b826ccd94a21`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 935
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `e46153cfb98015d3`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 943
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `36da54ba6757ec83`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 953
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `7d3938e11fdd68a2`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 963
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `c904f856a1a293c8`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 983
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d2e615eae671fed5`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 993
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `94d32a7b6c4f9f7e`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 1043
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `fff5faec479ee6f9`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 1053
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `143b1c541c07e343`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 1073
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `4a5c81cd23eefbfa`
- **File:** `/workspaces/do-codeguardian/report.md`
- **Line:** 1103
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `f2c69210d33d7d81`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 16
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d0614ec0b143e850`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 29
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `e43a87f23d54db80`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 55
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `e0d43bba73e4430a`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 68
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `cf1a4cf55d950466`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 185
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ad32bc2f7d885f4b`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 228
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `53e03c7f7a037740`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 241
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `bfd58f13aae160cb`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 306
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `17b7f9d6b2b2be70`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 319
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `9e6a53d0dd4a8b35`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 332
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `aed608de23ffdde1`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 345
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `91acaf176c6172a8`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 358
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `56e2fb93faea0210`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 360
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `1c9e73d0245c17ce`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 440
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a5e370824a716134`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 442
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `2913caa1253bb9bf`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 453
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `80e71eacca71a561`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 455
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d1d5f800557d9d22`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 468
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `1446a4ecd67db48d`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 557
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d4c4fc7e37efde60`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 628
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a36bedd7541b4f9b`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 641
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `c424b51d5ae55038`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 654
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d68c36cb3e87b4ac`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 669
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `7db34a618828dc28`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 684
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `026940f873cd02dd`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 699
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `3a8c152bf11ac16d`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 714
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `677104f1ef09d5c4`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 727
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `f24ed6e72cdb2b08`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 757
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `567de02ef238b9ff`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 772
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `06ccdf1d90aedbc6`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 800
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0deaab16fcc34e1d`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 813
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ee62c3e319253a1f`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 826
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `2d4b9f0ba5735173`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 839
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `8bcc98e7302ce7c3`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 852
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a99b1151ef18b5d8`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 893
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `c321c9711ae075dc`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 895
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `705fea69967fbdae`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 960
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `260bf5668b43c574`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 971
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `8ec585f63939fa67`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 984
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `668b384dbe1b688c`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 1010
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `280bd7dbe5db1ee4`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 1023
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `c2caa904ae593f0e`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 1036
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `f89790430d8fd563`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 1049
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `5398633a9875e14f`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 1062
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d1ad30df8f0bd79c`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 1075
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `36c921ae7c48548c`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 1088
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `50fcc46aa89a21e0`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 1101
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `81e6cbdb669ae789`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 1114
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `52712c9496998f81`
- **File:** `/workspaces/do-codeguardian/results-ml-enhanced.json`
- **Line:** 1179
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a810a45d98aca529`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 16
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a3e5dddfcac8a822`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 29
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0f44f9e1b794aed6`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 55
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `833dbaad4b73f970`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 68
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `617765eeebeee5d4`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 185
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `be8d0929c709fabf`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 228
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `02695672c614400d`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 241
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `b459ca5c25ad1b4f`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 306
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `37092167123e798a`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 319
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `2d6200f6ed134e12`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 332
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ccd6e3159d60250a`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 345
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `9a52f4c1f61e7b71`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 358
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d544a7b8d3552cdf`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 360
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `86d5d6af62468e71`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 440
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `6c186da7351a4bb3`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 442
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `6e9f55aff1a9ab20`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 453
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `3c5d45fda2b3d1c3`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 455
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ede34bd71dadad87`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 468
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `4162ea870db6d1cd`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 557
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `be36a00326602b87`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 628
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `4c808adce1ca8484`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 641
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `157524b23156734f`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 654
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ddc553781a7f84d9`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 669
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `df1c8059dfb6463c`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 684
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `17f4fd8e5ef93482`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 699
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `b35dcd3c58f897c8`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 714
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `256479a6401f8ccf`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 727
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `289ce60e9348cd15`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 757
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `87d7ff6b6e528ec1`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 772
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `5cebb53b0c3912de`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 800
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `29623e3f3ba3f835`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 813
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `57e5ec9e15b520a4`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 826
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d3f9fa5cb1f59498`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 839
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `77e410246e70e71b`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 852
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `c04378b0b84c187b`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 893
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `8a76cd2ae4c9fda8`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 895
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `7af6411bf109e419`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 960
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `97e2d3ccd2353d76`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 971
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `8333c88408bb86f6`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 984
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `92e356a7052c072b`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 1010
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `702b362681081ad3`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 1023
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `4edd3150bce9fa8f`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 1036
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `5ab7d7441d5dae2a`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 1049
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `2761495fa77083a2`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 1062
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `efe1d01b90c8ba28`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 1075
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `1359651a9cb0bb99`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 1088
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `997ec2ae44749895`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 1101
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `2f0f52f280e6bd8d`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 1114
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a8feccd1ca421189`
- **File:** `/workspaces/do-codeguardian/results-no-ml.json`
- **Line:** 1179
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a0d5f294bfbb7121`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 16
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `26dc0df30608eb9b`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 29
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d0fa84c4f013c731`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 55
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `4a62a787dd777304`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 68
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `4e2a5db24516df4b`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 185
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `109a3b5cc1857549`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 228
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a9ec94d61e1bf327`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 241
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a1c6f91c75e67d06`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 306
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `01838b6d8c127ad5`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 319
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `2f8b2dabecd47cf3`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 332
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0c1b2ba644f35708`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 345
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `50669554269e5550`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 358
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a4be095bddaaa547`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 360
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a53ed3c8ba0c2e06`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 440
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a4afffb53ac52d6d`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 442
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `2976adf5893af403`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 453
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `25281523f10ceabe`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 455
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `66af315b5917c663`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 468
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `e2d1e1aae39b0041`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 557
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `b26808ab08f060ec`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 628
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `f817e7ae1f66c5ee`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 641
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a385ecf97c3c18f8`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 654
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `9263ffc621fca3e0`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 669
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `428fc2a8ff2674e8`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 684
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `9e8d1a40e1edb4a2`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 699
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `177c91b52f4352b0`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 714
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `525d4f2d521f7c24`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 727
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `fae06960861a8e7a`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 757
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ee1448c6382953c5`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 772
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `edb37f881065372d`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 800
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `b8e940e2075abe17`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 813
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0f5e6079ad118fea`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 826
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `b49967d9b2c51c39`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 839
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `f3e45362d42eb620`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 852
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a2674e91ec364489`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 893
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `da9d1b41d1e91b05`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 895
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `66bbe67cfacaf2a1`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 960
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `9066a55adff0d936`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 971
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `45bc275f711f38e7`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 984
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `31e7e7ab67516615`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 1010
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `8b0b497dbdfaac36`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 1023
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `dcecb7523ae82edb`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 1036
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `78533e617a63f995`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 1049
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `486a19a701e71931`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 1062
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `7c9d7683a43119b0`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 1075
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `64b9f658452d1f88`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 1088
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `f986a2bd22f35f0d`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 1101
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `38436bb7155b710a`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 1114
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `47c18287aa893d66`
- **File:** `/workspaces/do-codeguardian/results-standard.json`
- **Line:** 1179
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `6868317527deca34`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 91
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `33ab857e467bb7ef`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 136
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a3404f5e1a6063c1`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 166
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `473349fb037e79e3`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 314
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `7f877460c193992d`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 353
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0d326ec555093ce2`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 368
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `67c276762fce07fa`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 426
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `8a26b2ced31276ea`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 439
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `33dbf8e707a188cf`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 452
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `452d53b9be2a301e`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 478
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `8d3044969df6a4a8`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 491
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0e163add3a3f7752`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 504
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `5ffd160c4cca5c00`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 517
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `309f02abffeca5f7`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 530
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a25f998574c767ce`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 543
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a1aa7a839387cc62`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 556
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a3493d016bf37166`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 571
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `bf0bb20f7207e270`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 582
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `af37f6028f9c5c53`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 595
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `83d632791dafda14`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 623
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `22bdd8a02bea290b`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 636
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `e18751550a72f831`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 653
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `74796dee0bd43772`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 666
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `60a53a393a73eaef`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 679
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `23a0cbf9002a8832`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 839
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `2f9340b36e964ad5`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 854
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d563fe7e5e6b9173`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 869
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `e16a9f34c818e61a`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 884
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `f7fcb2bf235d6c9b`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 897
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `4eeb20b5cc4840d5`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 910
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `fd7921e2888029a5`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 925
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d55467f9eb157b14`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 940
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `dcf3129c041c7d29`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1056
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0273629702eac0e9`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1058
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `b4c1caca7cf56a6e`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1086
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ec34ff4defa8561c`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1110
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `f32d65e01e00af52`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1125
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0574c7d36de7de72`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1227
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `761398f92d64dae2`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1240
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `35744c6e3cb7cefc`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1266
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ae29aeec496d8064`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1279
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `498757f6aa867994`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1292
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ff1c29ddcdb8d641`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1305
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d8094c909c603d4f`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1318
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `b3db44dc5d4a6eb6`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1331
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `4c2f1baae3ce4c7d`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1359
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `7b24690fa248da93`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1372
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `4784a2ee71b0203e`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1374
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `43c1220a3673d2e8`
- **File:** `/workspaces/do-codeguardian/results.json`
- **Line:** 1424
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `48886fe20b0b6477`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 91
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0fa53c2e8e4a3f27`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 136
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `cae2276f5dd134b6`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 166
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `76cbb2efae4fb6f1`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 314
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `5000a160af12d513`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 353
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `1fe58c9e5b90f4c6`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 368
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `275116e753761065`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 426
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `34439cf20ad49437`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 439
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `44de786834d983fc`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 452
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `9a6c6f7fc77163da`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 478
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `3ed5f2bc018b115e`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 491
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `30083569719b52f0`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 504
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `60f4a08eb1e31f49`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 517
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a25cc057c39393f7`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 530
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `fd54d4856fa265c5`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 543
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `3ae6dc1a46821d6b`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 556
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `19fdc6d321316415`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 571
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `239f4a4d91edc2c3`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 582
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `bb8e537a468e802b`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 595
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `96fdb750165fde24`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 623
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `dd8eb2565aaefebe`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 636
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `64dba5dedeb154d9`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 653
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `c384195530f4d567`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 666
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `c5a768afe3ba1ff6`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 679
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `094d2283f4994fab`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 839
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `8530eb962c292f46`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 854
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `aa4a4ef1939e22d3`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 869
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `d3e4d2da3ec09c8e`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 884
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `b055474f7956a8ad`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 897
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `ea64ecb7af52ce00`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 910
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `2cbeb50f137da043`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 925
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `f352d4639df13e1a`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 940
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `2a2a368ca3d7498e`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1056
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `e886df7cd3e3d873`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1058
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `af3a2ebf8822c5e1`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1086
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `feca58338d7c4f21`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1110
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `197371aea1811504`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1125
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0f9d4f102e91db0a`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1227
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `3b3af1b203b3ddd8`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1240
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `a5adba103d1a0e25`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1266
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0910addeb80c5531`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1279
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `71e4d36e49912482`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1292
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `cf4511d5cf238a1b`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1305
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `94a0be294e59a0a1`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1318
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0420d241f9114671`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1331
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `0d836c8df3e7e648`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1359
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `2fddbd4e0c2c7ea6`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1372
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `cf5f17ddcf983213`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1374
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected - consider using named constant

- **ID:** `56dad23dfb80d104`
- **File:** `/workspaces/do-codeguardian/self-analysis.json`
- **Line:** 1424
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `f080daf219c45857`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 11
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f413fd365dfe000c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 15
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `22b3a918eaee9997`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 70
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 5 times

- **ID:** `e51013d9b7fe56cf`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 75
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 6 times

- **ID:** `28a2eb55fcde3f21`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 94
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 10 times

- **ID:** `f134929d874dee56`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 95
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 22 times

- **ID:** `5046802e482fd067`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 106
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `dc702cc29a10ebe5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 139
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `7a276aee3563c031`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 160
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `e0e39e84db0b7762`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 160
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `11715036932fb09a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 210
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 4 times

- **ID:** `d22adc12fc69cdbe`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 230
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `3021735f717d33cb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 231
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `7fbbaff2feb575de`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 277
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `57c4ecf49a5498a8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 319
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 4 times

- **ID:** `1c6e5139b547179c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 376
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `45191d4dde497bf9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 506
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Commented-out code detected

- **ID:** `f008d046ce75dc00`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 712
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Import statement after non-import code

- **ID:** `c7c01336abae12b0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 905
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `ce83cde0027d248f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 906
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Duplicate line found 3 times

- **ID:** `55496556b60700cd`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 910
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `0bcf0d813b14393c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 911
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `cfd60788374f2b78`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 913
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Complex conditional statement - consider refactoring

- **ID:** `e1117713c9a6d9e0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/code_quality_analyzer.rs`
- **Line:** 922
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Duplicate line found 6 times

- **ID:** `512b3059da183bd1`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 71
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `76b8bd2504f34dc7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 117
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Potentially unnecessary clone() call

- **ID:** `294884b91e0bbcb5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 126
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Duplicate line found 9 times

- **ID:** `a36d1656f430c6a0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 133
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Import statement after non-import code

- **ID:** `ad66137ef11683fe`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 386
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `54d05fe393445fd8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 387
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `326d332e7f51e622`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 395
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3d9113466b5a8d11`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 396
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `63283400e5e9d95d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 416
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Single-letter variable name

- **ID:** `40b10cd9e408db4f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/integrity.rs`
- **Line:** 79
- **Analyzer:** code_quality
- **Rule:** single_letter_var
- **Description:** Single-letter variables reduce code readability
- **Suggestion:** Use descriptive variable names

#### Naming convention violation detected

- **ID:** `6335f4aa749c03b7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/integrity.rs`
- **Line:** 79
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Duplicate line found 4 times

- **ID:** `498f3cc36d0ec683`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 22
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### TODO comment found

- **ID:** `aa358275885e378c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 83
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `5842bf2f114ac1be`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 85
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `cf032a30011d92ee`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 89
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Duplicate line found 5 times

- **ID:** `b286cd286727ebfe`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 91
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### TODO comment found

- **ID:** `9b2c73ab0522901f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 93
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `5edabf787f3515e7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/lint_drift.rs`
- **Line:** 96
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Import statement after non-import code

- **ID:** `58c5bfd477803e23`
- **File:** `/workspaces/do-codeguardian/src/analyzers/mod.rs`
- **Line:** 12
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `cf9557dba8e84859`
- **File:** `/workspaces/do-codeguardian/src/analyzers/mod.rs`
- **Line:** 13
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `b76c0c563786be37`
- **File:** `/workspaces/do-codeguardian/src/analyzers/mod.rs`
- **Line:** 14
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### TODO comment found

- **ID:** `ea35bac1e9ae240d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 27
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `e2b6f2f6d3c42cc7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 28
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `e762da7b9d2b70fd`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 38
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `bf6c57024cc65aa9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 52
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `d1a2ef42f045d7bd`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 65
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `ebc8be9b07894696`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 66
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `ca397af229884084`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 71
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `7cd962ad51620eff`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 78
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Duplicate line found 4 times

- **ID:** `922719393c2d2822`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 80
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `aefe27e220c189bd`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 266
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `c22852d546c0d947`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 288
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `b209db53329ea8ee`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 293
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `04dfd9cd360d8580`
- **File:** `/workspaces/do-codeguardian/src/analyzers/non_production.rs`
- **Line:** 319
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `5cc994325f8d4e28`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 22
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `8ef004153eb7d902`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 24
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 4 times

- **ID:** `83eb13cc1fd9783e`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 41
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `cb91ef4d0eba0cc6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 42
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `c92a06f6acc5cc64`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 45
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `9487bd95486bb3c3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 53
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `f53df80852b78fc5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 61
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 5 times

- **ID:** `efb9f304bd40986f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 122
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 5 times

- **ID:** `72f1bb33c9937843`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 126
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `4ec0b2607be4211c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 131
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `ecb94cd6cce518b3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 221
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `1182ea1a6bcc41a2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 240
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `73c1d90c49558281`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 359
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `2f2b09234e2104e7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 397
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `f3af24f9f7cb79fe`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 406
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `fb2999a35fe1df7a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 407
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Duplicate line found 4 times

- **ID:** `4d10f1998a2911bf`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 411
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Hardcoded secret in non-production code

- **ID:** `841c85d9147b9067`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 412
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Non-production secrets should be externalized or clearly documented
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Duplicate line found 3 times

- **ID:** `70dea5b51badf0e1`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 413
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `5f49aa8e317c1c43`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 447
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `48ac22c83f4f833b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 456
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `2b0ece0d62eabf80`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 457
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3f90ea1dde2a02be`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 34
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `81048574eb53b9fe`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 54
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `8470b90d62b79616`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 107
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `6a248c8463df50bb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 204
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `7d99c91017eaa5b6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 219
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Single-letter variable name

- **ID:** `43b891520cde1e69`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 234
- **Analyzer:** code_quality
- **Rule:** single_letter_var
- **Description:** Single-letter variables reduce code readability
- **Suggestion:** Use descriptive variable names

#### Naming convention violation detected

- **ID:** `2dadf66d9635a38b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 234
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Import statement after non-import code

- **ID:** `630669ddd7931df2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 334
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `a1c8222f2d2cb98a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 335
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `882dcefa05649123`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 353
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `3b3273c5fc00c403`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 378
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Hardcoded secret in non-production code

- **ID:** `7e662112d8c5f9e3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 385
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Non-production secrets should be externalized or clearly documented
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Magic number detected

- **ID:** `6d251fe540eab82d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 391
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 7 times

- **ID:** `afd6a03904e23eb4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 63
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 18 times

- **ID:** `3bb8f8b714e388dc`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 78
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Potentially inefficient collection operation

- **ID:** `29966e5d3c761420`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 199
- **Analyzer:** performance
- **Rule:** inefficient_collection_ops
- **Description:** Linear search operations in loops can lead to O(n²) complexity
- **Suggestion:** Consider using HashSet, HashMap, or other O(1) lookup data structures

#### Magic number detected

- **ID:** `9edbe4862f0e2c22`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 201
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `1b8cf4878cd0dcd7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 222
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `552fa451be30db05`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 223
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `0d986719c9e3dfa7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 240
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 5 times

- **ID:** `67ecc4a50cc62564`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 259
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Vector without capacity hint

- **ID:** `cd46c137b4d25bb0`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 316
- **Analyzer:** performance
- **Rule:** inefficient_vec_growth
- **Description:** Growing vectors without capacity hints can cause multiple reallocations
- **Suggestion:** Use Vec::with_capacity() if you know the approximate size

#### Inefficient string building pattern

- **ID:** `7792f1ac1fe82b2c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 337
- **Analyzer:** performance
- **Rule:** string_building
- **Description:** Building strings character by character can be inefficient
- **Suggestion:** Consider using format! macro or collecting into String

#### Commented-out code detected

- **ID:** `8a305c862ee7b21b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 406
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Magic number detected

- **ID:** `143e58c73dcfbbe5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 534
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `24f9c01ed12c9cae`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 551
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `24159cd317f91e7a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 616
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `0f7b653698a34b18`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 617
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Duplicate line found 3 times

- **ID:** `ed8e029d4e0d4edb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 621
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `1e9e17b6016e0fd8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 624
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Potentially unnecessary clone() call

- **ID:** `2d3032fff93bf6ca`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 633
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `ebfd3b2564ec6d26`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 55
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `9b544d8b6021b1d3`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 66
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 8 times

- **ID:** `6a0be2417f73e72c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 175
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `3386a62cfbc0c392`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 279
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `0154bf76e7338969`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 279
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Duplicate line found 4 times

- **ID:** `c6c1928243427500`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 348
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `a610a1ac32317a99`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 370
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `fb02ee7efa860140`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 375
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `ca1eb652784a232b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 401
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `a925e948ae4e0e88`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 438
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ef4ebcc94e23a173`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 464
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Single-letter variable name

- **ID:** `769975a574d0c286`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 477
- **Analyzer:** code_quality
- **Rule:** single_letter_var
- **Description:** Single-letter variables reduce code readability
- **Suggestion:** Use descriptive variable names

#### Naming convention violation detected

- **ID:** `37e44e5b8df63e3f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 477
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Import statement after non-import code

- **ID:** `d6836de3328524c2`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 542
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `54e045d89c90fa5a`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 543
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Duplicate line found 3 times

- **ID:** `35bd6deb2b1e9cf4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 547
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `1659cfb43f8ff5f8`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 550
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `c1fd5d4583383e7d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 551
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Hardcoded secret in non-production code

- **ID:** `f8ce840f619c774c`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs`
- **Line:** 559
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Non-production secrets should be externalized or clearly documented
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Permissive version range detected - consider pinning versions

- **ID:** `cb917d9b7cfe07cb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs.new`
- **Line:** 187
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Naming convention violation detected

- **ID:** `9fd319eb3a748b90`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs.new`
- **Line:** 194
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e87816c61d9598a4`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs.new`
- **Line:** 195
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1209d8e8b601afa7`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs.new`
- **Line:** 226
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `dd3dff67d5059227`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs.new`
- **Line:** 236
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `50d41f5a5789fde9`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_analyzer.rs.new`
- **Line:** 242
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Duplicate line found 5 times

- **ID:** `a1e702f1e7b558bb`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 34
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 5 times

- **ID:** `b20094d285a96824`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 35
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 6 times

- **ID:** `593bf222778ec59d`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 39
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 6 times

- **ID:** `6d13da3322a7f19f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/security_checks.rs`
- **Line:** 43
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `e4547fab46e74e17`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 52
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `6935c41c4406c2a3`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 130
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `7f1aa3e7adf5b905`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 193
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `d169fed944d7959b`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 236
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `1fae9f970530905e`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 263
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `81094d3bbf913419`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 285
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `4ca24c659784a341`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 305
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `c01349eb91ec87da`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 313
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `9b00c0ee6661d40c`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 317
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b545d72410cc5469`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 328
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `7e0741a37798e9d1`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 410
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `98950c8e5dc4d33c`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 427
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `7afa6d99188a2086`
- **File:** `/workspaces/do-codeguardian/src/cache.rs`
- **Line:** 456
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `14bdd84213f170f4`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 41
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 5 times

- **ID:** `19d2cc0ac2050c34`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 58
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `98af4e20b40b0407`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 73
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `878651574faf9742`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 77
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `54cbcb8502eeeca2`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 197
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 10 times

- **ID:** `0a552ec239cf0952`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 217
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `fdb9104f22ce1d3e`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 227
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `9bec1a84f775ed8c`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 251
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `2ee4afceccc49f46`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 252
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 4 times

- **ID:** `dd4fc34520686148`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 288
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `b97c8ff4e3ff1d1e`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 393
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `3f73034055a2c7a5`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 505
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `6748f692981ab1f9`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 524
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `360490f23173ed92`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 588
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Import statement after non-import code

- **ID:** `92955d6316fd6773`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 55
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `eee0347fbf0e362b`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 99
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `8908b3e2fcbe8b76`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 101
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ad09dc879b90379f`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 103
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `be8a0b0124d1b515`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 105
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `cbcb422f1ee366c5`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 114
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `d4b42e3ba016a54a`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 119
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `0665430937189edb`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 184
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `5286447b68a607e9`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 185
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `a4ee651a54de5452`
- **File:** `/workspaces/do-codeguardian/src/cli/init.rs`
- **Line:** 190
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `e44a89df207f2297`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 76
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f64e53abddf39ead`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 77
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `fe8b3a7fe957e582`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 78
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `5197dec3f359ac50`
- **File:** `/workspaces/do-codeguardian/src/cli/metrics.rs`
- **Line:** 79
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 6 times

- **ID:** `f674fcfd5de50861`
- **File:** `/workspaces/do-codeguardian/src/cli/report.rs`
- **Line:** 35
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `3ef94841d46cdae0`
- **File:** `/workspaces/do-codeguardian/src/cli/report.rs`
- **Line:** 197
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 7 times

- **ID:** `9d425c70a03de9e1`
- **File:** `/workspaces/do-codeguardian/src/cli/report.rs`
- **Line:** 216
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `280fff1343784996`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 51
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Potentially unnecessary clone() call

- **ID:** `397b69fa0bc24b87`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 144
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `df1dd3caece85c89`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 182
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `db505fb8c600ba21`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 183
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `0f395771cd7467e7`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 185
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ad50f605ee86254d`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 224
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `6d8c08be6e9a3e86`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 229
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f3d0e1d07efd20d3`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 254
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Commented-out code detected

- **ID:** `8a9cd4b58e6f6208`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 15
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `f7f05115c42257ce`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 19
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Magic number detected

- **ID:** `5ae686b2eacda614`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 23
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Commented-out code detected

- **ID:** `cf8afdd16608dba3`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 23
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `ed46a925bc4103d2`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 27
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `2fd1da6099bfc394`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 30
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `e4f398e0e8b3e853`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 31
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Magic number detected

- **ID:** `f386ccbdbed10c5c`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 35
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Commented-out code detected

- **ID:** `358ac2a4d2f2190f`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 35
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `4decee7e158b186e`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 43
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Magic number detected

- **ID:** `6a5cb55a18d6cd4c`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 60
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `49c176e75fff0fc9`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 64
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `cebbf11c8944e416`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 64
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `24b3887e076a2a3d`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 73
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `291f452e800f5457`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 119
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `8e23c0ff672a4479`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 127
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `6a7d8e743b7113de`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 129
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `3457d5da37a19315`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 134
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 4 times

- **ID:** `df37da07b9887e7f`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 184
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `226d3fbaabc0dc84`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 200
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 8 times

- **ID:** `addee64c5ef7cc74`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 206
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `0b0cf688001559a6`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 254
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### TODO comment found

- **ID:** `2191178b65263341`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 268
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `44b30cc398fcdb26`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 271
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `5f5f8fe67a71ef0a`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 283
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### TODO comment found

- **ID:** `f142f7882cfd9a5d`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 287
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Single-letter variable name

- **ID:** `b3e6480a62f7a2bf`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 419
- **Analyzer:** code_quality
- **Rule:** single_letter_var
- **Description:** Single-letter variables reduce code readability
- **Suggestion:** Use descriptive variable names

#### Naming convention violation detected

- **ID:** `a42b6cc5c5ebce81`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 419
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Commented-out code detected

- **ID:** `e0482a4558d4b169`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 25
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `fe08a29b356e656f`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 29
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `f6e6cf3182c8aa61`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 61
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `d432534004a1f9df`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 65
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `8ed01a7524daaaa4`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 69
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `9e4a2f0fd69656e0`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 81
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `3602c698d2cd6cf5`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 85
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `eb4812187a5b5f8d`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 89
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `b68ca8b8aa73f487`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 104
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `4d7229570db39c5e`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 105
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `dac974434698a18f`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 124
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `341913b73510ce0f`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 132
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `66fb95cb359e0555`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 139
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `ce32d31e3c3e0780`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 143
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `9eeb12ed6b970146`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 147
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `4daf21b35883e1a6`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 151
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `126cc7762a9fa975`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 155
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Magic number detected

- **ID:** `0fd85c21b6778faa`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 167
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Commented-out code detected

- **ID:** `757a348e4c8616f4`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 167
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Magic number detected

- **ID:** `992e4d042a547c6c`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 171
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Commented-out code detected

- **ID:** `1a9091d3d87d5b5c`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 171
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `dc9bd6f6ba283068`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 193
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `ab06ded4771b319b`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 197
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `08336226d9236479`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 201
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Magic number detected

- **ID:** `04a2ae185833f812`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 205
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Commented-out code detected

- **ID:** `84d457b26c038198`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 205
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `9347fb7b93c1643c`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 213
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `b43882e3e5f348f2`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 232
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `22fed502d908bd85`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 250
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Duplicate line found 3 times

- **ID:** `afea5d4444ff0eef`
- **File:** `/workspaces/do-codeguardian/src/cli.rs`
- **Line:** 288
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `2227a6d291720744`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 58
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `973e003febc51d91`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 59
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 5 times

- **ID:** `87b43064b33b1261`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 60
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `c72c1bd0549f37e8`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 61
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `734e097a9e2fe1d8`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 64
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `79ee964f8afa1ae2`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 71
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `cc2fe0c317a0b6cc`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 208
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `bc9dc505118fe202`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 213
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f9bb9ad90fa4f8cd`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 218
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f3d77b20b41b0970`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 265
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f312530a43294c24`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 268
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `cfb41d17cbeb811b`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 268
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Import statement after non-import code

- **ID:** `91830c3e4c570e75`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 289
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Duplicate line found 6 times

- **ID:** `cb81349e55ee7d09`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 293
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `93c182b3812e0a90`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 295
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `aa885d94751212c8`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 304
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `388709907a6fb567`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 312
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f019a13e66e25954`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 313
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `84045ebc7d70ee38`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 322
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `ba11f83d8df7a6b9`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 322
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `92646445c05b6b15`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 338
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `9ca21c6eedb845f2`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 350
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `24285a13831e3479`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 65
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 5 times

- **ID:** `756e75b4a8879b23`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 67
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 6 times

- **ID:** `5b0ffbc146fbbed8`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 68
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `c954ee556f631cf0`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 70
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `fb23ec90a9a1bac7`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 71
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `dda2a177be065f50`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 72
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3ec41e14758797f2`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 73
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `56daf37753c5feaa`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 85
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `133d20dffb3bb7b6`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 86
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `a1b3d792826227af`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 95
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `95bd00a9df2b6c75`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 96
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `67837be79c7baa23`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 97
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f501d5d3cc5588cb`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 108
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `a916dd2f28e68df3`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 109
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `5f91f1b950fc08ef`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 110
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f9985922211a8c3b`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 111
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `a26bc23e8b20d08d`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 112
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 5 times

- **ID:** `4c17b33239008f64`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 113
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `1556135a8b151b92`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 116
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 5 times

- **ID:** `1cf8c806c3541d57`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 119
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `48f6ee6cb9addcd7`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 122
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `678b9f4f472ad94a`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 123
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `68b4048a65dffa64`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 134
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `079287d30592ceae`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 135
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `8aa8e488af29863d`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 136
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `508fdeadfce6618d`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 137
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `6ddb1418b18c9107`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 140
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `9756ff4d324df901`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 141
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `23d3a98b238e09ec`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 142
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `30cc8bf4bc6ef28c`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 146
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `e8a7000f25429601`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 147
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b1d3cd06a08ef421`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 148
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `7a61bc81df303274`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 158
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `28d79f24068c4021`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 159
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `c9e87509133d956b`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 160
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `cf9978ca0f56ae5b`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 161
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `9ac142c8462acde0`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 162
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `51f41ca9982757ea`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 166
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `6f457b63874c9dd3`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 167
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `542c021d04cd7ee0`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 172
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `37d11ce8a02d0aad`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 173
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `85a3fc6d26d45f78`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 183
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `a28081a907b6be30`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 186
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `59da12a12140403a`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 189
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b44f81591488a075`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 204
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `baad23656fc102bc`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 205
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `6c52452fceb876ab`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 206
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f01f50f871c4c58a`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 207
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `d08eb5469587a9a2`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 211
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `e2c109c02f714be5`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 216
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `96b2979c22176c1f`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 217
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `2ed95fcc2d13b5a5`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 218
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `281c0cc04e3f6146`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 229
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `5d4707be5dbd319a`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 230
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `0a879a8655a6207e`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 231
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `4276034242f6073e`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 232
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `b6eb993edcfd5b86`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 232
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `fc0438802c778c1a`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 233
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `df3ab0cd5f7b1c16`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 237
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `483a9899a6525721`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 238
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `23fb5a42e36fa8c2`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 242
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f611019762588939`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 243
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f957616373ba8e54`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 244
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `5af3b7dded1ebc49`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 260
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3b1f19ddb4f7b36e`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 287
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `d0d10b25b1ff3474`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 291
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `63de1774218e1555`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 300
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `1f7a99cc0b3e0b6c`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 310
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `13d6f91cedbe407e`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 311
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `89c7a84e032ae091`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 312
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `71c0f782dad6079a`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 338
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Duplicate line found 4 times

- **ID:** `70966ebd50e487da`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 343
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `f6deeeb846b08837`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 352
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `c46e4401ab206bbe`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 352
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `cb9e1d9299a497c6`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 369
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `cdac0bec0b7ee1ca`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 370
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `8178ef31415e09b4`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 373
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `0e42039a472da758`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 378
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `0a1d78b229fee591`
- **File:** `/workspaces/do-codeguardian/src/config/performance.rs`
- **Line:** 395
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 11 times

- **ID:** `30dfc8e1e0b6146b`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 20
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `d0a6f573fd9b49dd`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 22
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `52bd490953b8bfb5`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 23
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `a356cf5b489042a4`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 25
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 4 times

- **ID:** `a6a788ddbb396690`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 60
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### TODO comment found

- **ID:** `efd9b366155214ec`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 113
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Magic number detected

- **ID:** `8d6c4f68b821ae81`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 153
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `a46a2d9627a90d3f`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 154
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `0c12c3c57ed693ea`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 176
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `61f6b8b7b3aaed77`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 177
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b9d8483396b6582a`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 208
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `b6d06ec997f8c458`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 208
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `34922db3b3fafa43`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 237
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `73204af85f623809`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 238
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `c606cb1649fbf467`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 299
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `7f427915900b687d`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 368
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `54ee190eb8a2bcb3`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 398
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `2b0e277a22467ae7`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 399
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `610279963cca6fcb`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 399
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `98a0ca0cf7e5f16b`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 401
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### TODO comment found

- **ID:** `c5db9f4c07c88ae7`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 422
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Duplicate line found 3 times

- **ID:** `7f2628950a88cab0`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 429
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `b7b06b18c5b6df57`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 430
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `da3359c9a837f73d`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 431
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `258903fb28a0f1c0`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 432
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `1633c4664d2db712`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 433
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `35078a28a32ec29a`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 438
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `e0abb2717acb0293`
- **File:** `/workspaces/do-codeguardian/src/config.rs`
- **Line:** 439
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `369eef463061c1dd`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 17
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `a50ec83cc4db543b`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 18
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `e1e0e47d705fbf2a`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 19
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `a50e7f560ed86db4`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 98
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `10e29f2844d5de54`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 127
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `97d94ed3a174167e`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 143
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `e5c3fa40696bce3b`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 154
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `8cc07fe866bbe5d5`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 154
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `b7f958058d822d70`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 196
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 4 times

- **ID:** `15105ff1c7052d4f`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 260
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `8de0c6db8de76dc4`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 288
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `06b4e7ca4a7b2d2e`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 325
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Import statement after non-import code

- **ID:** `24a2289256173c99`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 377
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `9b16c18502cd672e`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 398
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `af51625a566e0f87`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 419
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `171f81c6ded7933e`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 421
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Import statement after non-import code

- **ID:** `2747a3d77fe37b18`
- **File:** `/workspaces/do-codeguardian/src/core.rs`
- **Line:** 433
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Duplicate line found 3 times

- **ID:** `a6eef78a8b180f77`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 23
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `1458deb2614e2da4`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 100
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `d5617cc74267d7ef`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 149
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `03d524ebbd53dff2`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 150
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `9e1fff37fad0df81`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 151
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `be74e42a4649cf9d`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 152
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `089f67ae86cc3030`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 153
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `8f0f2a6d547d9a6a`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 185
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `26eb87740d540587`
- **File:** `/workspaces/do-codeguardian/src/error.rs`
- **Line:** 187
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Potentially unnecessary clone() call

- **ID:** `af3923702c7d090d`
- **File:** `/workspaces/do-codeguardian/src/github.rs`
- **Line:** 16
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `3a9ffe06754666bc`
- **File:** `/workspaces/do-codeguardian/src/github.rs`
- **Line:** 21
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `59620da2673bb1a2`
- **File:** `/workspaces/do-codeguardian/src/github.rs`
- **Line:** 22
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ae4060f95a05f109`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 36
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `5ffa0327160bfb5d`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 40
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `1111d5e5c716526c`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 99
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3b5fa59eab369728`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 107
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `290f1c51bff77c54`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 108
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b06011e418b6f932`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 109
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `fb57940702065c35`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 214
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `7e0f663ad528e305`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 222
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `8e6bbd717bf2bcd2`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 236
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `e363546f1c5b7d8d`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 253
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `903eb66a4b916c3d`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 255
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `2021cdb8f315200b`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 267
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `fa49814034d23785`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 278
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `317ad378eb68dbac`
- **File:** `/workspaces/do-codeguardian/src/github_api.rs`
- **Line:** 293
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `5060db7ca69b61d0`
- **File:** `/workspaces/do-codeguardian/src/main.rs`
- **Line:** 19
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `127e1de9e53e423d`
- **File:** `/workspaces/do-codeguardian/src/main.rs`
- **Line:** 20
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Commented-out code detected

- **ID:** `b5564dc4794bf1c9`
- **File:** `/workspaces/do-codeguardian/src/main.rs`
- **Line:** 23
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Commented-out code detected

- **ID:** `f7735b987ca5bcdc`
- **File:** `/workspaces/do-codeguardian/src/main.rs`
- **Line:** 24
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Magic number detected

- **ID:** `abd7f8356f30344c`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 51
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `d51ed2bf8a8b6b3f`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 61
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `9b7e6805cc76c0a8`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 62
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f8eed78b7c3074c2`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 72
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `79f401298730eeb4`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 166
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `631f0eb1be77cd52`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 189
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `4475a02b3d7ab897`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 210
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `428108d22110bf91`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 262
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `44f0afdf209bf1db`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 263
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `78a9be6c3cb3093a`
- **File:** `/workspaces/do-codeguardian/src/ml/fann_classifier.rs`
- **Line:** 265
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `4ae7c9184c55ee69`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 37
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ead21ac08836c9af`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 38
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f9b6ce428342cd29`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 39
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `facba33443be82d9`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 47
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `612bbf9e20f9f9a8`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 49
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `5473af5e7792dfd8`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 86
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `83269bf26c2de9f6`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 89
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `8305caf92294afad`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 92
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3e68eb3da993a32a`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 126
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ce8f55ce977104b3`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 131
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3b333dacfefcea5c`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 132
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3d2b992b0da4fcfa`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 138
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `4ddb48d697fb4e96`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 152
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `8895f0dc6686fdb3`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 166
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `71d47cd1229340a0`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 167
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Single-letter variable name

- **ID:** `01bf9c2163ab6786`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 189
- **Analyzer:** code_quality
- **Rule:** single_letter_var
- **Description:** Single-letter variables reduce code readability
- **Suggestion:** Use descriptive variable names

#### Naming convention violation detected

- **ID:** `a983dea767bd8d11`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 189
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `994053740fe560f0`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 205
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Commented-out code detected

- **ID:** `6b4eb215a1e7b4f4`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 205
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Magic number detected

- **ID:** `528ac68307786372`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 206
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `faa0699f5aa9c3e7`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 222
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `b53ac15ba3211e75`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 248
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `39f2c2afea04f472`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 275
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `92a998e96700fbc5`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 276
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `5d0100ea58cbb8b3`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 277
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `4525504eb4252b03`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 288
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `c77c263631e327f6`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 288
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `5cf74694571c793d`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 296
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `8f1ea0e6e38f7a12`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 304
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `7eee1e0482119e36`
- **File:** `/workspaces/do-codeguardian/src/ml/feature_extractor.rs`
- **Line:** 305
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `2634de11d3e024c1`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 85
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `a1b840e57b00fda5`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 196
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `7b4b5849d1d55aef`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 218
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### TODO comment found

- **ID:** `e219cc98c24b36bc`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 259
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Duplicate line found 6 times

- **ID:** `0041ce3a2c671377`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 292
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `1cee8bb78ade6219`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 338
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `32b8e7ae59c5f562`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 369
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `95ed137bd57b43b6`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 381
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `30c9266d84037354`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 385
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `e7ccdb2158a1df1a`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 388
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `81edcfb909d3e9ff`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 442
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `c2632a3b4b32ff0d`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 443
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `43e5b8ec9ded19ec`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 458
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `04d2e978aeb923d2`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 485
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `85990884c332dee8`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 508
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `71a9377128e22c73`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 510
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `2661c2385c31d6ef`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 532
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `d94094f65ec1c43d`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 533
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `1353eb16792d69d1`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 552
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `766a5d2ac266a08e`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 553
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `2df97e634e9b0f91`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 570
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3ab5ea904ac60efc`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 574
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `25e8f73f3dc218fa`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 581
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `826dff4ee04d6e50`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 590
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `fcaa800800c3b4f5`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 644
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `8fcb4e3e4a33ba00`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 645
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `c8f80a2d8036e831`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 655
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b220887a7122d618`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 694
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `c4dccc4c99f6257a`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 738
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `ada5eaec9d89de11`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 758
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Duplicate line found 4 times

- **ID:** `ef5a619941b28ddc`
- **File:** `/workspaces/do-codeguardian/src/ml/metrics.rs`
- **Line:** 768
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Import statement after non-import code

- **ID:** `192632012efe94a2`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 6
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `38e0ac479defc9f7`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 7
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `1f5b2331fd14f4c8`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 9
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Commented-out code detected

- **ID:** `8e5c4181f4f7c3d7`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 49
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Duplicate line found 3 times

- **ID:** `2e9c034e59421ff0`
- **File:** `/workspaces/do-codeguardian/src/ml/mod.rs`
- **Line:** 67
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Potentially unnecessary clone() call

- **ID:** `6617b3481ef8ef50`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 68
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `f934fab3f935765c`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 84
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `f992b9f7441ee197`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 96
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `98935e50b6dfcc56`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 98
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Duplicate line found 5 times

- **ID:** `b9f8e6420a11a715`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 117
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `57cea6459d6491d2`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 119
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `6d8b1aa009b133e9`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 125
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### TODO comment found

- **ID:** `8f30bddeec869a21`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 131
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Magic number detected

- **ID:** `8aaef7cc512e5a70`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 138
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `e3822de31a212308`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 145
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### TODO comment found

- **ID:** `6565d4a6ae4b6ee8`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 282
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a todo comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Import statement after non-import code

- **ID:** `ab3f13944eb6c539`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 305
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `6e4ee0e848f2b6c4`
- **File:** `/workspaces/do-codeguardian/src/ml/training_data.rs`
- **Line:** 326
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `775bba4f73443284`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 30
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3e8bb986545afc6b`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 31
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `71f8c2e80f915c1a`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 33
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 4 times

- **ID:** `36cef98df3048748`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 69
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Potentially unnecessary clone() call

- **ID:** `50fd2cfcf0a0b075`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 89
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `ec12184b500726d3`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 90
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `ed7a96913e10ad52`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 91
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Duplicate line found 3 times

- **ID:** `cb933d1a04477877`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 91
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Potentially unnecessary clone() call

- **ID:** `5ac515648a7cccfc`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 102
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Duplicate line found 3 times

- **ID:** `61fc6339bacf1e46`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 126
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `d9560d64d711bc2f`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 127
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `961f283a6cf8d374`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 129
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `3af05844fd421cb3`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 135
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Potentially unnecessary clone() call

- **ID:** `f29116aa13ce2501`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 142
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `c0a65bb58571f791`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 143
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `91680c73a2e4649f`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 144
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `eb918cb2749f6e7c`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 170
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `cf31389bcdac762b`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 171
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `adb599a309f56535`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 204
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Import statement after non-import code

- **ID:** `9b0c39e5b86c1c1c`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 225
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `07ba5a275f54291b`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 226
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `b13d726c0b2e3ed7`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 252
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f10a349fe94a7e57`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 253
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `2abc3a330372ba29`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 260
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `88faedd86fd2ec68`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 276
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `8ff7af3876a728c8`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 322
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `5325acab30f20f97`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 323
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `ee9d4fad1b5ad82f`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 324
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `8381bce7f3f0d52e`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 331
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `83c09625ef051f7f`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 338
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `e7ef6df0a1ba8010`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 338
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Single-letter variable name

- **ID:** `71c8260347976337`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 351
- **Analyzer:** code_quality
- **Rule:** single_letter_var
- **Description:** Single-letter variables reduce code readability
- **Suggestion:** Use descriptive variable names

#### Naming convention violation detected

- **ID:** `a7859770e4c9ec62`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 351
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `51c7de6037fab062`
- **File:** `/workspaces/do-codeguardian/src/performance/mod.rs`
- **Line:** 368
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Permissive version range detected - consider pinning versions

- **ID:** `2168cbc536388732`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 11
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ec5ce5645bbb748f`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 13
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `b14aa9ac15987727`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 14
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `e2a84c2d00a264ea`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 14
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `644d6a67b8f6f468`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 15
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `2ef00c0ad785306b`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 26
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `b5b586debc24c09f`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 28
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `0ed1afd7af05e669`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 29
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `14cf25dce5b0e3b7`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 30
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `58dba058ec3cf54f`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 41
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `7310a7beff961e9f`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 43
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `09f3419a7be11291`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 44
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `bb4db21ab82c3ca3`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 45
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `485327f56ca902ad`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 56
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `a3f26fe46f8b5183`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 57
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `daf640f4c8d51f07`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 58
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `38a278caf5fad5c8`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 62
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `efc755d22283b038`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 64
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `0bd9f692257fca90`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 65
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `044c94ca865dd369`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 66
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `ae6aa3979b440df3`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 80
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `253e4a8f0621ce2b`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 81
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `478fb531ab868deb`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 82
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected - consider using named constant

- **ID:** `a62f3fa4991d78e8`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 114
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `01d6febed1d8c7b2`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 134
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `daac4b467b5fcbf7`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 135
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `be1551f641f8d3fd`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 136
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `3b6bbf8ac01a6635`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 160
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `5c22faa86bf1da34`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 161
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `b40a11b113d3426d`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 162
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `5c13aeea5bcc25e5`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 163
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `04d71d68555ea322`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 167
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `addffdfc20ec072d`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 168
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `cca82734e0042eed`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 169
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `cc45e0ed58dbd039`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 199
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `cbdd569f700c10ba`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 200
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1fab5029d384e483`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 201
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `4b065d3c62308cdf`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 202
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `dcc31cd2edfd765b`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 206
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `22d48218aa9a7e98`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 207
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `9580b90f6b973616`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 208
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `f713869eda073ec9`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 237
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `1c3a864024f8bc86`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 238
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `d47100567f8bb8fd`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 239
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `c1646470719e106f`
- **File:** `/workspaces/do-codeguardian/src/performance/optimizations.md`
- **Line:** 240
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Magic number detected

- **ID:** `5c2f92efcf2ac7ec`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 47
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3ba732e5036bc9c2`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 102
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 8 times

- **ID:** `b2dae11ff61eab7e`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 117
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 7 times

- **ID:** `fdeae2b12c16817a`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 119
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `3e04badf114f5e3c`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 145
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `8e5d0f2f4c12e741`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 146
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `15cad00bb81faeb0`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 147
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `0333b5d99624f295`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 148
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 4 times

- **ID:** `83eacdb36c834014`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 149
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 7 times

- **ID:** `d23edc8a1654a365`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 155
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `26e5e25efdb72e4a`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 160
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 6 times

- **ID:** `4295926cac71cd93`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 161
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `fc5593e3f1914eb7`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 165
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 6 times

- **ID:** `53b2bda7c27d391c`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 185
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `106a7c6a2bfd0f4f`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 190
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Single-letter variable name

- **ID:** `b14b5497daa5cb85`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 192
- **Analyzer:** code_quality
- **Rule:** single_letter_var
- **Description:** Single-letter variables reduce code readability
- **Suggestion:** Use descriptive variable names

#### Naming convention violation detected

- **ID:** `9a6686f3db7d80fb`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 192
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Single-letter variable name

- **ID:** `73749e95dfc9ea2a`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 193
- **Analyzer:** code_quality
- **Rule:** single_letter_var
- **Description:** Single-letter variables reduce code readability
- **Suggestion:** Use descriptive variable names

#### Naming convention violation detected

- **ID:** `4cf16a294f403431`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 193
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `87ab72a96c2941b5`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 207
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 4 times

- **ID:** `c7c70f0c4e8c57b0`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 207
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `4370d3cd5aeb280f`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 241
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ca157b6e7ef53ad0`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 268
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `f861133c81fccae4`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 270
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `91d08b68fdeddedb`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 271
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `63de4c98f9e288d2`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 286
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `78a85b2b8f2da0fd`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 309
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `343890511bd8eda2`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 332
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `f6d5e883a796b6b0`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 349
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Potentially unnecessary clone() call

- **ID:** `3cd3f94719d9731e`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 411
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `fcec4d222a18f8fe`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 412
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `d6ec3f8497ee6387`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 461
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `eab0c08e96fbb08a`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 502
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `274d6ec70ca0676c`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 503
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `233c4459d1fef9f8`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 511
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `73b0f36be0fe4fa0`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 512
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Commented-out code detected

- **ID:** `111326295472e384`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 523
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Magic number detected

- **ID:** `1daf5dc0a23204b0`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 530
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `310b5612028a914a`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 539
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `afa5ff5fc6e74848`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 545
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `01160f53c8364c51`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 546
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `a314fe97269aaade`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 547
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `4b1c109d96b3d482`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 555
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `0d774865bdfd6b43`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 577
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `9c55aa9a5d452b09`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 585
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `637cb8393e3b8d73`
- **File:** `/workspaces/do-codeguardian/src/streaming.rs`
- **Line:** 589
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### TODO comment found

- **ID:** `a2a973c8640baa16`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 8
- **Analyzer:** non_production
- **Rule:** todo_comment
- **Description:** Line contains a TODO comment that should be resolved before production
- **Suggestion:** Resolve the issue or create a proper issue tracker entry

#### Hardcoded secret in test code

- **ID:** `569690bd04fb7eb3`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 8
- **Analyzer:** security
- **Rule:** hardcoded_secret
- **Description:** Test secrets should use mock values or be clearly marked as test data
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Import statement after non-import code

- **ID:** `b87a570ee167301b`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 15
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `a467c1d0842916f9`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 16
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `6d3ac783b517eb8d`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 27
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `92bacba1f7e64346`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 28
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `567eda0a13ddae3f`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 29
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `7089fe9fb5a222c5`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 40
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Magic number detected

- **ID:** `2ed12abd5f282b67`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 10
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `62b7759079927d49`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 79
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `1c3f31af6c1fa659`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 99
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `486051d3cbe99cdb`
- **File:** `/workspaces/do-codeguardian/src/types.rs`
- **Line:** 104
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Commented-out code detected

- **ID:** `2817bf81aae5026a`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 34
- **Analyzer:** code_quality
- **Rule:** commented_code
- **Description:** Commented-out code clutters the codebase and should be removed
- **Suggestion:** Remove commented-out code; use version control to track changes

#### Magic number detected

- **ID:** `f6824115cae4e62e`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 79
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ad3a7b37bf6f3328`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 80
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Potentially unnecessary clone() call

- **ID:** `f30de0a3c311df3f`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 95
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `7a622d00ac087942`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 99
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `c3fcf0e36fc13138`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 155
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `389c1c5ce5a6f7c5`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 280
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `ce003bbe0e50a51d`
- **File:** `/workspaces/do-codeguardian/src/utils/adaptive_parallelism.rs`
- **Line:** 286
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `2020fd174c7da81e`
- **File:** `/workspaces/do-codeguardian/src/utils/git.rs`
- **Line:** 6
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 3 times

- **ID:** `388577e37d9544e0`
- **File:** `/workspaces/do-codeguardian/src/utils/git.rs`
- **Line:** 10
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `70ab56cf201734bb`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 26
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `a9636a24a1a3d13b`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 48
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `6a0ddee316c8dc18`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 71
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `502157f1ea6078b5`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 143
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `43ad228f50c34858`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 157
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `4e332dce3ba01d65`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 188
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `1fae75fb27a6bddc`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 189
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Duplicate line found 3 times

- **ID:** `f5dc00452df63537`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 198
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `e80ac7a3f195cfe6`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 199
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Duplicate line found 3 times

- **ID:** `c422b94548377708`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 201
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `70ff0046378f2a6d`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 209
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `10f4da750441dca1`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 212
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `143cf43b3ba0aef9`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 232
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `f9c210053d83d28e`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 249
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `6cebd00d4d687e9f`
- **File:** `/workspaces/do-codeguardian/src/utils/memory_pool.rs`
- **Line:** 253
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Duplicate line found 3 times

- **ID:** `6bb8214b95f3f1e0`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 40
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Potentially unnecessary clone() call

- **ID:** `4d791278d01478bf`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 187
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Potentially unnecessary clone() call

- **ID:** `efdb35ee21f2f9fc`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 188
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Duplicate line found 4 times

- **ID:** `b19731d32077ce12`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 205
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Duplicate line found 4 times

- **ID:** `9a97ec049182411d`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 206
- **Analyzer:** code_quality
- **Rule:** duplicate_lines
- **Description:** Duplicate lines indicate potential code duplication
- **Suggestion:** Consider extracting common logic into a shared function

#### Magic number detected

- **ID:** `23ed27e3bac052d1`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 211
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `0aadf8a4485c662a`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 223
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `340168691d3066e6`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 235
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3480235b0e6f53ef`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 250
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `4147acc091b0c67b`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 262
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `2dadf499556b4a4d`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 263
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Single-letter variable name

- **ID:** `d59200a78de682f1`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 350
- **Analyzer:** code_quality
- **Rule:** single_letter_var
- **Description:** Single-letter variables reduce code readability
- **Suggestion:** Use descriptive variable names

#### Naming convention violation detected

- **ID:** `4dad5a0b850a0c52`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 350
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Magic number detected

- **ID:** `05a4b92e874e52c3`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 355
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `2fd17ae65376a194`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 378
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `8db8d3d0e7bf5644`
- **File:** `/workspaces/do-codeguardian/src/utils/progress.rs`
- **Line:** 28
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `379f3b23cf871450`
- **File:** `/workspaces/do-codeguardian/src/utils/progress.rs`
- **Line:** 32
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `de4417504c7df0c5`
- **File:** `/workspaces/do-codeguardian/src/utils/security.rs`
- **Line:** 11
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `b253dd1a6e66af77`
- **File:** `/workspaces/do-codeguardian/src/utils/security.rs`
- **Line:** 12
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Import statement after non-import code

- **ID:** `c04bd7844e9eb136`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 13
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `5250eeac1ee73245`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 14
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `7538007a5f6c7a79`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 15
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Import statement after non-import code

- **ID:** `fc520b85c1e69783`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 19
- **Analyzer:** code_quality
- **Rule:** misplaced_import
- **Description:** Imports should be grouped at the top of the file
- **Suggestion:** Move all imports to the top of the file

#### Potentially unnecessary clone() call

- **ID:** `541e2291ef07ed2a`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 89
- **Analyzer:** performance
- **Rule:** unnecessary_clone
- **Description:** Cloning can be expensive; consider using references or borrowing
- **Suggestion:** Use references (&) or borrowing instead of cloning when possible

#### Magic number detected

- **ID:** `68a34473808efa78`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 212
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `3dc7bd29efcd8ccb`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 251
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `83d07a9ed78c6fb4`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 285
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `51ed786d9e3bb553`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 286
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected

- **ID:** `0fda2a0ed2520144`
- **File:** `/workspaces/do-codeguardian/tests/integration_tests.rs`
- **Line:** 288
- **Analyzer:** code_quality
- **Rule:** magic_number
- **Description:** Magic numbers make code harder to understand and maintain
- **Suggestion:** Replace magic numbers with named constants

#### Magic number detected - consider using named constant

- **ID:** `480f64a3007ed6b7`
- **File:** `/workspaces/do-codeguardian/turbo-demo.sh`
- **Line:** 42
- **Analyzer:** optimized-quality
- **Rule:** QUAL-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `cf27b58009051ab6`
- **File:** `/workspaces/do-codeguardian/turbo-demo.sh`
- **Line:** 202
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `48cc179d9f28bf02`
- **File:** `/workspaces/do-codeguardian/turbo-demo.sh`
- **Line:** 203
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `99348e147072334c`
- **File:** `/workspaces/do-codeguardian/turbo-demo.sh`
- **Line:** 204
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

#### Permissive version range detected - consider pinning versions

- **ID:** `35d71d623c23cda7`
- **File:** `/workspaces/do-codeguardian/turbo-demo.sh`
- **Line:** 205
- **Analyzer:** optimized-dependency
- **Rule:** DEP-OPT

### ℹ️ info Issues

#### Long line detected (127 characters)

- **ID:** `666bdf6682050b65`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 162
- **Analyzer:** performance
- **Rule:** long_line
- **Description:** Very long lines can impact readability and code review efficiency
- **Suggestion:** Consider breaking long lines for better readability

#### Long line (127 characters)

- **ID:** `6b4253f2ef3a922b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/dependency_analyzer.rs`
- **Line:** 162
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line detected (212 characters)

- **ID:** `3159e234b38c3d11`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 448
- **Analyzer:** performance
- **Rule:** long_line
- **Description:** Very long lines can impact readability and code review efficiency
- **Suggestion:** Consider breaking long lines for better readability

#### Long line (212 characters)

- **ID:** `e151388e615ab7a5`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_analyzer.rs`
- **Line:** 448
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line detected (206 characters)

- **ID:** `d3585fd80e79f997`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 34
- **Analyzer:** performance
- **Rule:** long_line
- **Description:** Very long lines can impact readability and code review efficiency
- **Suggestion:** Consider breaking long lines for better readability

#### Long line (206 characters)

- **ID:** `3b7e3f9d035bf791`
- **File:** `/workspaces/do-codeguardian/src/analyzers/optimized_patterns.rs`
- **Line:** 34
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line detected (123 characters)

- **ID:** `8f45a12dfbcbac80`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 32
- **Analyzer:** performance
- **Rule:** long_line
- **Description:** Very long lines can impact readability and code review efficiency
- **Suggestion:** Consider breaking long lines for better readability

#### Long line (123 characters)

- **ID:** `9a6abc5988b6c557`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 32
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (144 characters)

- **ID:** `48f21c7701938f5f`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 82
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (136 characters)

- **ID:** `84385747e37b7844`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 83
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (136 characters)

- **ID:** `57bb5b2a7f981900`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 100
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (128 characters)

- **ID:** `5b39cee68c84137b`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 101
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (129 characters)

- **ID:** `df5dd61bd9a963b6`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 145
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (128 characters)

- **ID:** `98e61d67e3534952`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 161
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (141 characters)

- **ID:** `3ef7704a0c08a088`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 162
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (127 characters)

- **ID:** `6b726e028d351815`
- **File:** `/workspaces/do-codeguardian/src/analyzers/performance_analyzer.rs`
- **Line:** 544
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line detected (123 characters)

- **ID:** `cf7da646267853ac`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 383
- **Analyzer:** performance
- **Rule:** long_line
- **Description:** Very long lines can impact readability and code review efficiency
- **Suggestion:** Consider breaking long lines for better readability

#### Long line (123 characters)

- **ID:** `928dd8708cc637e8`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 383
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (165 characters)

- **ID:** `5fb8739fc7dd70a6`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 567
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (147 characters)

- **ID:** `6febd12ce1a6b8d0`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 571
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (126 characters)

- **ID:** `9c8ad1b2348f853d`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 618
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (143 characters)

- **ID:** `5914524a6266b8fe`
- **File:** `/workspaces/do-codeguardian/src/cli/gh_issue.rs`
- **Line:** 621
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line detected (134 characters)

- **ID:** `746ff2bebb51942c`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 70
- **Analyzer:** performance
- **Rule:** long_line
- **Description:** Very long lines can impact readability and code review efficiency
- **Suggestion:** Consider breaking long lines for better readability

#### Long line (134 characters)

- **ID:** `8f2ce37eddf1769d`
- **File:** `/workspaces/do-codeguardian/src/cli/train.rs`
- **Line:** 70
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line detected (132 characters)

- **ID:** `c1544e5bb3886f7e`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 419
- **Analyzer:** performance
- **Rule:** long_line
- **Description:** Very long lines can impact readability and code review efficiency
- **Suggestion:** Consider breaking long lines for better readability

#### Long line (132 characters)

- **ID:** `e9689f73a54b0fb0`
- **File:** `/workspaces/do-codeguardian/src/cli/turbo.rs`
- **Line:** 419
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line detected (123 characters)

- **ID:** `60f774bf9cdd1d55`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 250
- **Analyzer:** performance
- **Rule:** long_line
- **Description:** Very long lines can impact readability and code review efficiency
- **Suggestion:** Consider breaking long lines for better readability

#### Long line (123 characters)

- **ID:** `08beea1e0d15b3d8`
- **File:** `/workspaces/do-codeguardian/src/config/optimization_presets.rs`
- **Line:** 250
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Hardcoded secret in test code

- **ID:** `007d3670957bae93`
- **File:** `/workspaces/do-codeguardian/src/test_security_sample.rs`
- **Line:** 8
- **Analyzer:** non_production
- **Rule:** potential_secret
- **Description:** Test secrets should use mock values or be clearly marked as test data
- **Suggestion:** Move secrets to environment variables or secure configuration

#### Long line detected (129 characters)

- **ID:** `7d2eeb8b764fbe4e`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 162
- **Analyzer:** performance
- **Rule:** long_line
- **Description:** Very long lines can impact readability and code review efficiency
- **Suggestion:** Consider breaking long lines for better readability

#### Long line (129 characters)

- **ID:** `777ef1abfccc2424`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 162
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line (128 characters)

- **ID:** `cc990182549710f7`
- **File:** `/workspaces/do-codeguardian/src/utils/performance_monitor.rs`
- **Line:** 163
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

#### Long line detected (121 characters)

- **ID:** `2aa3322af88ec26b`
- **File:** `/workspaces/do-codeguardian/src/utils/security.rs`
- **Line:** 10
- **Analyzer:** performance
- **Rule:** long_line
- **Description:** Very long lines can impact readability and code review efficiency
- **Suggestion:** Consider breaking long lines for better readability

#### Long line (121 characters)

- **ID:** `cfb4d75abddf9f6a`
- **File:** `/workspaces/do-codeguardian/src/utils/security.rs`
- **Line:** 10
- **Analyzer:** code_quality
- **Rule:** long_line
- **Description:** Long lines reduce readability and can indicate complex logic
- **Suggestion:** Break long lines into multiple lines or simplify the expression

---
*Generated by CodeGuardian - Security-first code analysis*
