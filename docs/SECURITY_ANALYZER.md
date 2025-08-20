# Security Analyzer
# Updated with optimized patterns and performance improvements

The security analyzer component has been refactored to improve performance and maintainability:

1. Core security analysis is now in `src/analyzers/security_analyzer.rs`
2. Language-specific checks have been moved to `src/analyzers/security_checks.rs`
3. Performance patterns are stored in `src/analyzers/optimized_patterns.rs`
4. An optimized analyzer is available in `src/analyzers/optimized_analyzer.rs`

To update the analyzer:

1. Replace existing security_analyzer.rs with the optimized version
2. Add the new security_checks.rs file
3. Update mod.rs to include both modules
4. Add optimized_patterns.rs for pattern definitions
5. Add optimized_analyzer.rs for high-performance analysis

The refactoring improves:
- Performance through pattern caching and optimized regex patterns
- Maintainability by separating concerns into different modules
- Extensibility for new security checks with language-specific handlers
- Accuracy with context-aware secret detection (test vs production code)

## Key Components

### SecurityAnalyzer (`src/analyzers/security_analyzer.rs`)
The main security analyzer that implements the `Analyzer` trait. It features:
- Pattern caching for improved performance
- Context-aware secret detection that distinguishes between test and production code
- Language-specific security checks integration
- Support for multiple file types

### SecurityChecks (`src/analyzers/security_checks.rs`)
Language-specific security check handlers:
- JavaScriptSecurity: Checks for unsafe eval() usage and innerHTML with user data
- PythonSecurity: Checks for unsafe pickle deserialization
- PhpSecurity: Checks for file inclusion vulnerabilities with $_ variables
- JavaSecurity: Checks for unsafe object deserialization
- RustSecurity: Checks for unsafe blocks

### OptimizedPatterns (`src/analyzers/optimized_patterns.rs`)
Performance-optimized regex patterns and utilities:
- Combined regex patterns for faster matching
- FileType enum with support detection methods
- PatternCache for caching pattern match results
- AnalysisOptimizer with fast entropy and complexity calculations

### OptimizedAnalyzer (`src/analyzers/optimized_analyzer.rs`)
High-performance analyzer that combines all checks with optimizations:
- Early termination for performance
- Limited findings per file (default: 50)
- Optimized line-by-line analysis
- Support for security, performance, code quality, and dependency analysis