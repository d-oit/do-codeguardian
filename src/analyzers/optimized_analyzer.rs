use super::{Analyzer, optimized_patterns::*};
use crate::types::{Finding, Severity};
use anyhow::Result;
use std::path::Path;

/// High-performance analyzer using optimized patterns and caching
pub struct OptimizedAnalyzer {
    pattern_cache: PatternCache,
    enable_early_termination: bool,
    max_findings_per_file: usize,
}

impl OptimizedAnalyzer {
    pub fn new() -> Self {
        Self {
            pattern_cache: PatternCache::new(1000), // Cache up to 1000 pattern matches
            enable_early_termination: true,
            max_findings_per_file: 50,
        }
    }

    pub fn with_cache_size(mut self, cache_size: usize) -> Self {
        self.pattern_cache = PatternCache::new(cache_size);
        self
    }

    pub fn with_max_findings(mut self, max_findings: usize) -> Self {
        self.max_findings_per_file = max_findings;
        self
    }

    fn analyze_security_optimized(&mut self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        let file_type = AnalysisOptimizer::get_file_type(file_path);
        
        // Only analyze if file type supports security analysis
        if let Some(ft) = file_type {
            if !ft.supports_security_analysis() {
                return findings;
            }
        }

        // Use optimized line analysis with early termination
        let security_findings = AnalysisOptimizer::analyze_lines_optimized(content, |line_num, line| {
            // Check for secrets using combined pattern
            if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.secrets_combined, line) {
                return Some(format!("Potential secret detected: {}", 
                    line.chars().take(50).collect::<String>()));
            }

            // Check for SQL injection
            if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.sql_injection_fast, line) {
                return Some("Potential SQL injection vulnerability".to_string());
            }

            // Check for XSS
            if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.xss_fast, line) {
                return Some("Potential XSS vulnerability".to_string());
            }

            // Check for command injection
            if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.command_injection_fast, line) {
                return Some("Potential command injection vulnerability".to_string());
            }

            // Check for weak crypto
            if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.weak_crypto_fast, line) {
                return Some("Weak cryptographic algorithm detected".to_string());
            }

            // High entropy check for potential secrets
            if self.pattern_cache.check_pattern(&SECURITY_PATTERNS.high_entropy_check, line) {
                // Extract the potential secret for entropy analysis
                if let Some(captures) = SECURITY_PATTERNS.high_entropy_check.captures(line) {
                    if let Some(secret_match) = captures.get(0) {
                        let entropy = AnalysisOptimizer::calculate_entropy_fast(secret_match.as_str());
                        if entropy > 4.0 {
                            return Some(format!("High entropy string detected (entropy: {:.2})", entropy));
                        }
                    }
                }
            }

            None
        });

        // Convert to Finding objects
        for (line_num, message) in security_findings {
            findings.push(Finding::new(
                "optimized-security",
                "SEC-OPT",
                Severity::High,
                file_path.to_path_buf(),
                line_num as u32 + 1,
                message,
            ));

            if findings.len() >= self.max_findings_per_file {
                break;
            }
        }

        findings
    }

    fn analyze_performance_optimized(&mut self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        let file_type = AnalysisOptimizer::get_file_type(file_path);
        
        // Only analyze if file type supports performance analysis
        if let Some(ft) = file_type {
            if !ft.supports_performance_analysis() {
                return findings;
            }
        }

        let performance_findings = AnalysisOptimizer::analyze_lines_optimized(content, |line_num, line| {
            // Check for nested loops
            if self.pattern_cache.check_pattern(&PERFORMANCE_PATTERNS.nested_loops_fast, line) {
                return Some("Nested loops detected - potential O(n²) complexity".to_string());
            }

            // Check for inefficient string concatenation
            if self.pattern_cache.check_pattern(&PERFORMANCE_PATTERNS.string_concat_fast, line) {
                return Some("Inefficient string concatenation in loop".to_string());
            }

            // Check for blocking I/O
            if self.pattern_cache.check_pattern(&PERFORMANCE_PATTERNS.blocking_io_fast, line) {
                return Some("Blocking I/O operation detected".to_string());
            }

            // Check for inefficient collections
            if self.pattern_cache.check_pattern(&PERFORMANCE_PATTERNS.inefficient_collections_fast, line) {
                return Some("Inefficient collection operation in loop".to_string());
            }

            // Check for memory leaks
            if self.pattern_cache.check_pattern(&PERFORMANCE_PATTERNS.memory_leaks_fast, line) {
                return Some("Potential memory leak - event listener without cleanup".to_string());
            }

            None
        });

        // Convert to Finding objects
        for (line_num, message) in performance_findings {
            findings.push(Finding::new(
                "optimized-performance",
                "PERF-OPT",
                Severity::Medium,
                file_path.to_path_buf(),
                line_num as u32 + 1,
                message,
            ));

            if findings.len() >= self.max_findings_per_file {
                break;
            }
        }

        findings
    }

    fn analyze_quality_optimized(&mut self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();

        // Calculate complexity first
        let complexity = AnalysisOptimizer::calculate_complexity_fast(content);
        if complexity > 15 {
            findings.push(Finding::new(
                "optimized-quality",
                "QUAL-COMPLEXITY",
                Severity::Medium,
                file_path.to_path_buf(),
                1,
                format!("High cyclomatic complexity: {}", complexity),
            ));
        }

        let quality_findings = AnalysisOptimizer::analyze_lines_optimized(content, |line_num, line| {
            // Check for magic numbers
            if self.pattern_cache.check_pattern(&QUALITY_PATTERNS.magic_numbers_fast, line) {
                // Skip common acceptable numbers
                if !line.contains("0") && !line.contains("1") && !line.contains("100") {
                    return Some("Magic number detected - consider using named constant".to_string());
                }
            }

            // Check for complex conditions
            if self.pattern_cache.check_pattern(&QUALITY_PATTERNS.complex_conditions_fast, line) {
                return Some("Complex conditional statement - consider refactoring".to_string());
            }

            // Check for commented code
            if self.pattern_cache.check_pattern(&QUALITY_PATTERNS.commented_code_fast, line) {
                return Some("Commented code detected - consider removing".to_string());
            }

            // Check for naming violations
            if self.pattern_cache.check_pattern(&QUALITY_PATTERNS.naming_violations_fast, line) {
                return Some("Naming convention violation detected".to_string());
            }

            None
        });

        // Convert to Finding objects
        for (line_num, message) in quality_findings {
            findings.push(Finding::new(
                "optimized-quality",
                "QUAL-OPT",
                Severity::Low,
                file_path.to_path_buf(),
                line_num as u32 + 1,
                message,
            ));

            if findings.len() >= self.max_findings_per_file {
                break;
            }
        }

        findings
    }

    fn analyze_dependencies_optimized(&mut self, content: &str, file_path: &Path) -> Vec<Finding> {
        let mut findings = Vec::new();
        let file_type = AnalysisOptimizer::get_file_type(file_path);
        
        // Only analyze if file type supports dependency analysis
        if let Some(ft) = file_type {
            if !ft.supports_dependency_analysis() {
                return findings;
            }
        }

        let dependency_findings = AnalysisOptimizer::analyze_lines_optimized(content, |line_num, line| {
            // Check for version ranges that might be too permissive
            if self.pattern_cache.check_pattern(&DEPENDENCY_PATTERNS.version_ranges, line) {
                return Some("Permissive version range detected - consider pinning versions".to_string());
            }

            // Check for git dependencies
            if self.pattern_cache.check_pattern(&DEPENDENCY_PATTERNS.git_deps, line) {
                return Some("Git dependency detected - consider using published versions".to_string());
            }

            None
        });

        // Convert to Finding objects
        for (line_num, message) in dependency_findings {
            findings.push(Finding::new(
                "optimized-dependency",
                "DEP-OPT",
                Severity::Low,
                file_path.to_path_buf(),
                line_num as u32 + 1,
                message,
            ));

            if findings.len() >= self.max_findings_per_file {
                break;
            }
        }

        findings
    }
}

impl Analyzer for OptimizedAnalyzer {
    fn name(&self) -> &str {
        "optimized-analyzer"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        // Convert bytes to string, handling potential encoding issues
        let content_str = String::from_utf8_lossy(content);
        let mut analyzer = self.clone(); // Clone to get mutable access
        
        let mut all_findings = Vec::new();

        // Run all optimized analyses
        all_findings.extend(analyzer.analyze_security_optimized(&content_str, file_path));
        all_findings.extend(analyzer.analyze_performance_optimized(&content_str, file_path));
        all_findings.extend(analyzer.analyze_quality_optimized(&content_str, file_path));
        all_findings.extend(analyzer.analyze_dependencies_optimized(&content_str, file_path));

        // Limit total findings per file for performance
        if all_findings.len() > self.max_findings_per_file {
            all_findings.truncate(self.max_findings_per_file);
        }

        Ok(all_findings)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        // Support all file types that have optimized patterns
        if let Some(file_type) = AnalysisOptimizer::get_file_type(file_path) {
            file_type.supports_security_analysis() || 
            file_type.supports_performance_analysis() || 
            file_type.supports_dependency_analysis()
        } else {
            // Fallback to basic text analysis for unknown file types
            file_path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| !matches!(ext, "bin" | "exe" | "dll" | "so" | "dylib"))
                .unwrap_or(false)
        }
    }
}

impl Clone for OptimizedAnalyzer {
    fn clone(&self) -> Self {
        Self {
            pattern_cache: PatternCache::new(1000), // Fresh cache for cloned instance
            enable_early_termination: self.enable_early_termination,
            max_findings_per_file: self.max_findings_per_file,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_optimized_security_analysis() {
        let analyzer = OptimizedAnalyzer::new();
        let content = b"api_key = \"sk-1234567890abcdef\"\npassword = \"secret123\"";
        let path = PathBuf::from("test.js");
        
        let findings = analyzer.analyze(&path, content).unwrap();
        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.message.contains("secret")));
    }

    #[test]
    fn test_optimized_performance_analysis() {
        let analyzer = OptimizedAnalyzer::new();
        let content = b"for (i=0; i<n; i++) { for (j=0; j<m; j++) { process(i, j); } }";
        let path = PathBuf::from("test.js");
        
        let findings = analyzer.analyze(&path, content).unwrap();
        assert!(findings.iter().any(|f| f.message.contains("Nested loops")));
    }

    #[test]
    fn test_file_type_filtering() {
        let analyzer = OptimizedAnalyzer::new();
        
        // Should support JavaScript
        assert!(analyzer.supports_file(&PathBuf::from("test.js")));
        
        // Should support Rust
        assert!(analyzer.supports_file(&PathBuf::from("test.rs")));
        
        // Should not support binary files
        assert!(!analyzer.supports_file(&PathBuf::from("test.bin")));
    }

    #[test]
    fn test_complexity_calculation() {
        let analyzer = OptimizedAnalyzer::new();
        let complex_content = b"if (a) { if (b) { while (c) { for (d) { if (e) { } } } } }";
        let path = PathBuf::from("test.js");
        
        let findings = analyzer.analyze(&path, complex_content).unwrap();
        assert!(findings.iter().any(|f| f.message.contains("complexity")));
    }
}