use regex::Regex;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    /// Security patterns - optimized for performance
    pub static ref SECURITY_PATTERNS: SecurityPatterns = SecurityPatterns::new();
    
    /// Performance patterns - optimized for common cases
    pub static ref PERFORMANCE_PATTERNS: PerformancePatterns = PerformancePatterns::new();
    
    /// Code quality patterns - simplified for speed
    pub static ref QUALITY_PATTERNS: QualityPatterns = QualityPatterns::new();
    
    /// Dependency patterns - minimal overhead
    pub static ref DEPENDENCY_PATTERNS: DependencyPatterns = DependencyPatterns::new();
}

pub struct SecurityPatterns {
    // Optimized secret detection - combined patterns for fewer regex operations
    pub secrets_combined: Regex,
    pub sql_injection_fast: Regex,
    pub xss_fast: Regex,
    pub command_injection_fast: Regex,
    pub weak_crypto_fast: Regex,
    pub high_entropy_check: Regex,
}

impl SecurityPatterns {
    fn new() -> Self {
        Self {
            // Combined secret patterns - single regex instead of multiple
            secrets_combined: Regex::new(
                r#"(?i)(?:password|secret|token|api[_-]?key|private[_-]?key|auth[_-]?token|jwt[_-]?secret)\s*[:=]\s*["']([^"']{8,})["']|AKIA[0-9A-Z]{16}|ghp_[0-9a-zA-Z]{36}|xox[baprs]-[0-9a-zA-Z-]{10,48}"#
            ).unwrap(),
            
            // Simplified SQL injection pattern - focus on most common cases
            sql_injection_fast: Regex::new(
                r#"(?i)(?:select|insert|update|delete)\s+.*\+.*["']|query\s*\(\s*["'].*\+"#
            ).unwrap(),
            
            // Simplified XSS pattern
            xss_fast: Regex::new(
                r#"(?i)innerHTML\s*=.*\+|document\.write\s*\(.*\+|dangerouslySetInnerHTML"#
            ).unwrap(),
            
            // Simplified command injection
            command_injection_fast: Regex::new(
                r#"(?i)(?:system|exec|shell_exec)\s*\(.*\$|Runtime\.getRuntime\(\)\.exec\(.*\+"#
            ).unwrap(),
            
            // Simplified weak crypto
            weak_crypto_fast: Regex::new(
                r#"(?i)(?:md5|sha1|des|rc4)\s*\(|MessageDigest\.getInstance\s*\(\s*["'](?:MD5|SHA1)"#
            ).unwrap(),
            
            // High entropy string detection (optimized)
            high_entropy_check: Regex::new(
                r#"["'][a-zA-Z0-9+/=]{32,}["']"#
            ).unwrap(),
        }
    }
}

pub struct PerformancePatterns {
    pub nested_loops_fast: Regex,
    pub string_concat_fast: Regex,
    pub blocking_io_fast: Regex,
    pub inefficient_collections_fast: Regex,
    pub memory_leaks_fast: Regex,
}

impl PerformancePatterns {
    fn new() -> Self {
        Self {
            // Simplified nested loop detection
            nested_loops_fast: Regex::new(
                r"for\s*\([^{]*\{[^}]*for\s*\(|while\s*\([^{]*\{[^}]*while\s*\("
            ).unwrap(),
            
            // Optimized string concatenation detection
            string_concat_fast: Regex::new(
                r#"\+=\s*["']|String\s*\+\s*["']|\+\s*["'][^"']*["']"#
            ).unwrap(),
            
            // Simplified blocking I/O detection
            blocking_io_fast: Regex::new(
                r"\.(?:read|write|sleep)\s*\(|(?:Thread|time)\.sleep|fs\.(?:readFileSync|writeFileSync)"
            ).unwrap(),
            
            // Optimized collection operations
            inefficient_collections_fast: Regex::new(
                r"\.(?:contains|indexOf)\s*\([^)]*\).*(?:for|while)|in\s+\w+.*for"
            ).unwrap(),
            
            // Memory leak patterns
            memory_leaks_fast: Regex::new(
                r"setInterval\s*\(|addEventListener\s*\([^,]*,[^)]*\)|\.on\s*\([^,]*,"
            ).unwrap(),
        }
    }
}

pub struct QualityPatterns {
    pub magic_numbers_fast: Regex,
    pub complex_conditions_fast: Regex,
    pub commented_code_fast: Regex,
    pub function_signatures_fast: Regex,
    pub naming_violations_fast: Regex,
}

impl QualityPatterns {
    fn new() -> Self {
        Self {
            // Optimized magic number detection (exclude common acceptable numbers)
            magic_numbers_fast: Regex::new(
                r"\b(?!(?:0|1|2|3|4|5|10|16|24|32|60|64|100|128|256|512|1000|1024)\b)\d{2,}\b"
            ).unwrap(),
            
            // Simplified complex condition detection
            complex_conditions_fast: Regex::new(
                r"if\s*\([^)]*(?:&&||\|\|)[^)]*(?:&&|\|\|)[^)]*\)"
            ).unwrap(),
            
            // Fast commented code detection
            commented_code_fast: Regex::new(
                r"^\s*(?://|#)\s*(?:function|def|class|if|for|while|var|let|const)\s+"
            ).unwrap(),
            
            // Combined function signature patterns
            function_signatures_fast: Regex::new(
                r"(?:fn|function|def|public|private|protected)\s+(\w+)\s*\(([^)]*)\)"
            ).unwrap(),
            
            // Fast naming convention checks
            naming_violations_fast: Regex::new(
                r"\b[a-hk-wz]\s*=|fn\s+[A-Z]\w*|function\s+\w*_\w*"
            ).unwrap(),
        }
    }
}

pub struct DependencyPatterns {
    pub package_json_deps: Regex,
    pub cargo_toml_deps: Regex,
    pub version_ranges: Regex,
    pub git_deps: Regex,
}

impl DependencyPatterns {
    fn new() -> Self {
        Self {
            // Optimized package.json dependency extraction
            package_json_deps: Regex::new(
                r#""([^"]+)":\s*"([^"]+)""#
            ).unwrap(),
            
            // Cargo.toml dependency patterns
            cargo_toml_deps: Regex::new(
                r#"^(\w+)\s*=\s*(?:\{[^}]*\}|["'][^"']*["'])"#
            ).unwrap(),
            
            // Version range detection
            version_ranges: Regex::new(
                r"[\*\^~]|>=|<=|\|\|"
            ).unwrap(),
            
            // Git dependency detection
            git_deps: Regex::new(
                r#"git\s*=\s*['\"][^'\"]*['\"]"#
            ).unwrap(),
        }
    }
}

/// Performance optimization utilities
pub struct AnalysisOptimizer;

impl AnalysisOptimizer {
    /// Fast file type detection using extension lookup
    pub fn get_file_type(path: &std::path::Path) -> Option<FileType> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| match ext {
                "rs" => Some(FileType::Rust),
                "js" | "jsx" => Some(FileType::JavaScript),
                "ts" | "tsx" => Some(FileType::TypeScript),
                "py" => Some(FileType::Python),
                "java" => Some(FileType::Java),
                "cpp" | "cc" | "cxx" => Some(FileType::Cpp),
                "c" => Some(FileType::C),
                "go" => Some(FileType::Go),
                "php" => Some(FileType::Php),
                "rb" => Some(FileType::Ruby),
                "json" => Some(FileType::Json),
                "toml" => Some(FileType::Toml),
                "yaml" | "yml" => Some(FileType::Yaml),
                _ => None,
            })
    }
    
    /// Fast line-based analysis with early termination
    pub fn analyze_lines_optimized<F>(content: &str, mut analyzer: F) -> Vec<(usize, String)>
    where
        F: FnMut(usize, &str) -> Option<String>,
    {
        let mut results = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Skip empty lines and comments early
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("#") {
                continue;
            }
            
            if let Some(finding) = analyzer(line_num, line) {
                results.push((line_num, finding));
                
                // Early termination for performance - limit findings per file
                if results.len() >= 50 {
                    break;
                }
            }
        }
        
        results
    }
    
    /// Optimized entropy calculation for secret detection
    pub fn calculate_entropy_fast(s: &str) -> f64 {
        if s.len() < 8 {
            return 0.0;
        }
        
        let mut char_counts = [0u32; 256];
        let mut total_chars = 0u32;
        
        // Count characters (ASCII only for performance)
        for byte in s.bytes() {
            char_counts[byte as usize] += 1;
            total_chars += 1;
        }
        
        // Calculate entropy
        let mut entropy = 0.0;
        let total_f = total_chars as f64;
        
        for &count in &char_counts {
            if count > 0 {
                let p = count as f64 / total_f;
                entropy -= p * p.log2();
            }
        }
        
        entropy
    }
    
    /// Fast complexity calculation using simple heuristics
    pub fn calculate_complexity_fast(content: &str) -> usize {
        let mut complexity = 1;
        
        // Count decision points efficiently
        complexity += content.matches("if ").count();
        complexity += content.matches("else").count();
        complexity += content.matches("while ").count();
        complexity += content.matches("for ").count();
        complexity += content.matches("switch ").count();
        complexity += content.matches("case ").count();
        complexity += content.matches("catch ").count();
        complexity += content.matches("&&").count();
        complexity += content.matches("||").count();
        
        complexity
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Java,
    Cpp,
    C,
    Go,
    Php,
    Ruby,
    Json,
    Toml,
    Yaml,
}

impl FileType {
    /// Check if file type supports specific analysis
    pub fn supports_security_analysis(&self) -> bool {
        matches!(self, 
            FileType::Rust | FileType::JavaScript | FileType::TypeScript | 
            FileType::Python | FileType::Java | FileType::Cpp | FileType::C |
            FileType::Go | FileType::Php | FileType::Ruby
        )
    }
    
    pub fn supports_performance_analysis(&self) -> bool {
        matches!(self, 
            FileType::Rust | FileType::JavaScript | FileType::TypeScript | 
            FileType::Python | FileType::Java | FileType::Cpp | FileType::C |
            FileType::Go
        )
    }
    
    pub fn supports_dependency_analysis(&self) -> bool {
        matches!(self, FileType::Json | FileType::Toml | FileType::Yaml)
    }
}

/// Optimized pattern matching with caching
pub struct PatternCache {
    cache: HashMap<String, bool>,
    max_size: usize,
}

impl PatternCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size),
            max_size,
        }
    }
    
    pub fn check_pattern(&mut self, pattern: &Regex, text: &str) -> bool {
        let key = format!("{}:{}", pattern.as_str(), text);
        
        if let Some(&result) = self.cache.get(&key) {
            return result;
        }
        
        let result = pattern.is_match(text);
        
        // Simple cache eviction - clear when full
        if self.cache.len() >= self.max_size {
            self.cache.clear();
        }
        
        self.cache.insert(key, result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_optimized_patterns() {
        // Test that optimized patterns still catch issues
        assert!(SECURITY_PATTERNS.secrets_combined.is_match("api_key = \"sk-1234567890abcdef\""));
        assert!(PERFORMANCE_PATTERNS.nested_loops_fast.is_match("for (i=0; i<n; i++) { for (j=0; j<m; j++) {"));
        assert!(QUALITY_PATTERNS.magic_numbers_fast.is_match("timeout = 5000"));
    }
    
    #[test]
    fn test_entropy_calculation() {
        let entropy = AnalysisOptimizer::calculate_entropy_fast("abcdefghijklmnop");
        assert!(entropy > 3.0);
        
        let low_entropy = AnalysisOptimizer::calculate_entropy_fast("aaaaaaaaaaaaaaaa");
        assert!(low_entropy < 1.0);
    }
    
    #[test]
    fn test_complexity_calculation() {
        let code = "if (a) { if (b) { while (c) { for (d) { } } } }";
        let complexity = AnalysisOptimizer::calculate_complexity_fast(code);
        assert!(complexity > 4);
    }
}