use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::path::Path;
use std::collections::HashMap;

/// Analyzer for code quality issues, maintainability problems, and code smells
pub struct CodeQualityAnalyzer {
    // Patterns for detecting code smells
    #[allow(dead_code)]
    long_parameter_list_pattern: Regex,
    #[allow(dead_code)]
    duplicate_code_pattern: Regex,
    magic_number_pattern: Regex,
    #[allow(dead_code)]
    dead_code_pattern: Regex,
    complex_condition_pattern: Regex,
    // Complexity tracking
    complexity_keywords: Vec<String>,
}

impl CodeQualityAnalyzer {
    pub fn new() -> Self {
        Self {
            long_parameter_list_pattern: Regex::new(r"(?:fn|function|def|public|private|protected)\s+\w+\s*\([^)]{100,}\)").unwrap(),
            duplicate_code_pattern: Regex::new(r"(.{50,})\n.*(.{50,})").unwrap(),
            magic_number_pattern: Regex::new(r"\b\d{2,}\b").unwrap(),
            dead_code_pattern: Regex::new(r"(?i)(unreachable|dead|unused|deprecated)").unwrap(),
            complex_condition_pattern: Regex::new(r"if\s*\([^)]*&&[^)]*&&[^)]*\)|if\s*\([^)]*\|\|[^)]*\|\|[^)]*\)").unwrap(),
            complexity_keywords: vec![
                "if".to_string(), "else".to_string(), "while".to_string(), "for".to_string(),
                "switch".to_string(), "case".to_string(), "catch".to_string(), "&&".to_string(), "||".to_string()
            ],
        }
    }

    fn analyze_code_quality(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let content_str = String::from_utf8_lossy(content);
        let lines: Vec<&str> = content_str.lines().collect();

        // Analyze each line
        for (line_num, line) in lines.iter().enumerate() {
            let line_number = (line_num + 1) as u32;
            findings.extend(self.analyze_line(file_path, line, line_number, &lines)?);
        }

        // File-level analysis
        findings.extend(self.analyze_file_structure(file_path, &lines)?);
        findings.extend(self.analyze_function_complexity(file_path, &content_str)?);
        findings.extend(self.analyze_naming_conventions(file_path, &lines)?);

        Ok(findings)
    }

    fn analyze_line(&self, file_path: &Path, line: &str, line_number: u32, _all_lines: &[&str]) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for magic numbers
        if self.magic_number_pattern.is_match(line) && !self.is_acceptable_magic_number_context(line) {
            findings.push(
                Finding::new(
                    "code_quality",
                    "magic_number",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "Magic number detected".to_string(),
                )
                .with_description("Magic numbers make code harder to understand and maintain".to_string())
                .with_suggestion("Replace magic numbers with named constants".to_string())
            );
        }

        // Check for complex conditions
        if self.complex_condition_pattern.is_match(line) {
            findings.push(
                Finding::new(
                    "code_quality",
                    "complex_condition",
                    Severity::Medium,
                    file_path.to_path_buf(),
                    line_number,
                    "Complex conditional expression".to_string(),
                )
                .with_description("Complex conditions with multiple logical operators are hard to read and test".to_string())
                .with_suggestion("Break complex conditions into smaller, named boolean variables".to_string())
            );
        }

        // Check for long lines
        if line.len() > 120 {
            findings.push(
                Finding::new(
                    "code_quality",
                    "long_line",
                    Severity::Info,
                    file_path.to_path_buf(),
                    line_number,
                    format!("Long line ({} characters)", line.len()),
                )
                .with_description("Long lines reduce readability and can indicate complex logic".to_string())
                .with_suggestion("Break long lines into multiple lines or simplify the expression".to_string())
            );
        }

        // Check for deep nesting (count leading whitespace)
        let indent_level = line.len() - line.trim_start().len();
        if indent_level > 24 { // More than 6 levels of 4-space indentation
            findings.push(
                Finding::new(
                    "code_quality",
                    "deep_nesting",
                    Severity::Medium,
                    file_path.to_path_buf(),
                    line_number,
                    "Deep nesting detected".to_string(),
                )
                .with_description("Deep nesting makes code harder to understand and test".to_string())
                .with_suggestion("Consider extracting nested logic into separate functions".to_string())
            );
        }

        // Check for commented-out code
        if self.is_commented_code(line) {
            findings.push(
                Finding::new(
                    "code_quality",
                    "commented_code",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "Commented-out code detected".to_string(),
                )
                .with_description("Commented-out code clutters the codebase and should be removed".to_string())
                .with_suggestion("Remove commented-out code; use version control to track changes".to_string())
            );
        }

        // Language-specific checks
        findings.extend(self.check_language_specific_quality(file_path, line, line_number)?);

        Ok(findings)
    }

    fn analyze_file_structure(&self, file_path: &Path, lines: &[&str]) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check file length
        if lines.len() > 500 {
            findings.push(
                Finding::new(
                    "code_quality",
                    "large_file",
                    Severity::Medium,
                    file_path.to_path_buf(),
                    1,
                    format!("Large file ({} lines)", lines.len()),
                )
                .with_description("Large files are harder to understand and maintain".to_string())
                .with_suggestion("Consider breaking this file into smaller, more focused modules".to_string())
            );
        }

        // Check for duplicate lines
        let mut line_counts: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.len() > 20 && !trimmed.starts_with("//") && !trimmed.starts_with("#") {
                line_counts.entry(trimmed.to_string()).or_default().push(i + 1);
            }
        }

        for (_line_content, occurrences) in line_counts {
            if occurrences.len() > 2 {
                findings.push(
                    Finding::new(
                        "code_quality",
                        "duplicate_lines",
                        Severity::Low,
                        file_path.to_path_buf(),
                        occurrences[0] as u32,
                        format!("Duplicate line found {} times", occurrences.len()),
                    )
                    .with_description("Duplicate lines indicate potential code duplication".to_string())
                    .with_suggestion("Consider extracting common logic into a shared function".to_string())
                );
            }
        }

        // Check import/include organization
        findings.extend(self.check_import_organization(file_path, lines)?);

        Ok(findings)
    }

    fn analyze_function_complexity(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Simple function extraction and complexity analysis
        let functions = self.extract_functions(content);
        
        for (func_name, func_content, start_line) in functions {
            let complexity = self.calculate_cyclomatic_complexity(&func_content);
            let line_count = func_content.lines().count();

            // Check cyclomatic complexity
            if complexity > 10 {
                findings.push(
                    Finding::new(
                        "code_quality",
                        "high_complexity",
                        Severity::High,
                        file_path.to_path_buf(),
                        start_line,
                        format!("Function '{}' has high cyclomatic complexity ({})", func_name, complexity),
                    )
                    .with_description("High complexity functions are harder to understand, test, and maintain".to_string())
                    .with_suggestion("Break this function into smaller, more focused functions".to_string())
                );
            } else if complexity > 7 {
                findings.push(
                    Finding::new(
                        "code_quality",
                        "moderate_complexity",
                        Severity::Medium,
                        file_path.to_path_buf(),
                        start_line,
                        format!("Function '{}' has moderate complexity ({})", func_name, complexity),
                    )
                    .with_description("Consider simplifying this function".to_string())
                    .with_suggestion("Look for opportunities to extract helper functions".to_string())
                );
            }

            // Check function length
            if line_count > 50 {
                findings.push(
                    Finding::new(
                        "code_quality",
                        "long_function",
                        Severity::Medium,
                        file_path.to_path_buf(),
                        start_line,
                        format!("Function '{}' is too long ({} lines)", func_name, line_count),
                    )
                    .with_description("Long functions are harder to understand and maintain".to_string())
                    .with_suggestion("Break this function into smaller, more focused functions".to_string())
                );
            }

            // Check parameter count
            let param_count = self.count_parameters(&func_content);
            if param_count > 5 {
                findings.push(
                    Finding::new(
                        "code_quality",
                        "too_many_parameters",
                        Severity::Medium,
                        file_path.to_path_buf(),
                        start_line,
                        format!("Function '{}' has too many parameters ({})", func_name, param_count),
                    )
                    .with_description("Functions with many parameters are hard to use and test".to_string())
                    .with_suggestion("Consider using a struct/object to group related parameters".to_string())
                );
            }
        }

        Ok(findings)
    }

    fn analyze_naming_conventions(&self, file_path: &Path, lines: &[&str]) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        for (line_num, line) in lines.iter().enumerate() {
            let line_number = (line_num + 1) as u32;

            // Check for single-letter variable names (except common ones)
            if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
                findings.extend(self.check_naming_for_language(file_path, line, line_number, ext)?);
            }
        }

        Ok(findings)
    }

    fn check_import_organization(&self, file_path: &Path, lines: &[&str]) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            match ext {
                "rs" => {
                    // Check Rust import organization
                    let mut found_non_import = false;
                    for (line_num, line) in lines.iter().enumerate() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("use ") {
                            if found_non_import {
                                findings.push(
                                    Finding::new(
                                        "code_quality",
                                        "misplaced_import",
                                        Severity::Low,
                                        file_path.to_path_buf(),
                                        (line_num + 1) as u32,
                                        "Import statement after non-import code".to_string(),
                                    )
                                    .with_description("Imports should be grouped at the top of the file".to_string())
                                    .with_suggestion("Move all imports to the top of the file".to_string())
                                );
                            }
                        } else if !trimmed.is_empty() && !trimmed.starts_with("//") && !trimmed.starts_with("#") {
                            found_non_import = true;
                        }
                    }
                }
                "py" => {
                    // Check Python import organization
                    let mut found_non_import = false;
                    for (line_num, line) in lines.iter().enumerate() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
                            if found_non_import {
                                findings.push(
                                    Finding::new(
                                        "code_quality",
                                        "misplaced_import",
                                        Severity::Low,
                                        file_path.to_path_buf(),
                                        (line_num + 1) as u32,
                                        "Import statement after non-import code".to_string(),
                                    )
                                    .with_description("Imports should be at the top of the file".to_string())
                                    .with_suggestion("Follow PEP 8: group imports at the top".to_string())
                                );
                            }
                        } else if !trimmed.is_empty() && !trimmed.starts_with("#") && !trimmed.starts_with("\"\"\"") {
                            found_non_import = true;
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(findings)
    }

    fn is_acceptable_magic_number_context(&self, line: &str) -> bool {
        // Context where magic numbers might be acceptable
        line.contains("test") || 
        line.contains("example") || 
        line.contains("const") ||
        line.contains("static") ||
        line.contains("final") ||
        line.contains("readonly") ||
        line.contains("version") ||
        line.contains("port") ||
        line.contains("timeout")
    }

    fn is_commented_code(&self, line: &str) -> bool {
        let trimmed = line.trim();
        if !trimmed.starts_with("//") && !trimmed.starts_with("#") {
            return false;
        }

        // Remove comment markers
        let content = trimmed.trim_start_matches("//").trim_start_matches("#").trim();
        
        // Look for code patterns
        content.contains("=") && (
            content.contains("(") || 
            content.contains("{") || 
            content.contains(";") ||
            content.contains("function") ||
            content.contains("def ") ||
            content.contains("class ") ||
            content.contains("if ") ||
            content.contains("for ") ||
            content.contains("while ")
        )
    }

    fn extract_functions(&self, content: &str) -> Vec<(String, String, u32)> {
        let mut functions = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        // Simple function extraction (this could be more sophisticated)
        let function_patterns = [
            Regex::new(r"(?m)^(?:pub\s+)?fn\s+(\w+)").unwrap(), // Rust
            Regex::new(r"(?m)^(?:async\s+)?function\s+(\w+)").unwrap(), // JavaScript
            Regex::new(r"(?m)^def\s+(\w+)").unwrap(), // Python
            Regex::new(r"(?m)^(?:public|private|protected)?\s*(?:static\s+)?(?:\w+\s+)*(\w+)\s*\(").unwrap(), // Java/C#
        ];

        for (line_num, line) in lines.iter().enumerate() {
            for pattern in &function_patterns {
                if let Some(captures) = pattern.captures(line) {
                    if let Some(func_name) = captures.get(1) {
                        let start_line = (line_num + 1) as u32;
                        let func_content = self.extract_function_body(&lines, line_num);
                        functions.push((func_name.as_str().to_string(), func_content, start_line));
                    }
                }
            }
        }

        functions
    }

    fn extract_function_body(&self, lines: &[&str], start_line: usize) -> String {
        let mut body = String::new();
        let mut brace_count = 0;
        let mut in_function = false;

        for line in lines.iter().skip(start_line) {
            body.push_str(line);
            body.push('\n');

            for ch in line.chars() {
                match ch {
                    '{' => {
                        brace_count += 1;
                        in_function = true;
                    }
                    '}' => {
                        brace_count -= 1;
                        if in_function && brace_count == 0 {
                            return body;
                        }
                    }
                    _ => {}
                }
            }

            // For Python, use indentation
            if !in_function && line.trim().is_empty() {
                continue;
            }
            if in_function && !line.starts_with(' ') && !line.starts_with('\t') && !line.trim().is_empty() {
                return body;
            }
        }

        body
    }

    fn calculate_cyclomatic_complexity(&self, content: &str) -> usize {
        let mut complexity = 1; // Base complexity

        for keyword in &self.complexity_keywords {
            complexity += content.matches(keyword).count();
        }

        complexity
    }

    fn count_parameters(&self, func_content: &str) -> usize {
        // Simple parameter counting
        if let Some(params_start) = func_content.find('(') {
            if let Some(params_end) = func_content.find(')') {
                let params_str = &func_content[params_start + 1..params_end];
                if params_str.trim().is_empty() {
                    return 0;
                }
                return params_str.split(',').count();
            }
        }
        0
    }

    fn check_language_specific_quality(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            match ext {
                "rs" => findings.extend(self.check_rust_quality(file_path, line, line_number)?),
                "js" | "ts" => findings.extend(self.check_javascript_quality(file_path, line, line_number)?),
                "py" => findings.extend(self.check_python_quality(file_path, line, line_number)?),
                _ => {}
            }
        }

        Ok(findings)
    }

    fn check_rust_quality(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for unwrap() usage
        if line.contains(".unwrap()") && !line.contains("test") {
            findings.push(
                Finding::new(
                    "code_quality",
                    "unwrap_usage",
                    Severity::Medium,
                    file_path.to_path_buf(),
                    line_number,
                    "unwrap() usage detected".to_string(),
                )
                .with_description("unwrap() can panic; consider using proper error handling".to_string())
                .with_suggestion("Use expect(), match, or the ? operator for better error handling".to_string())
            );
        }

        // Check for expect() with generic messages
        if line.contains(".expect(\"") && line.contains("failed") {
            findings.push(
                Finding::new(
                    "code_quality",
                    "generic_expect",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "Generic expect message".to_string(),
                )
                .with_description("Generic expect messages don't provide useful debugging information".to_string())
                .with_suggestion("Use specific, descriptive expect messages".to_string())
            );
        }

        Ok(findings)
    }

    fn check_javascript_quality(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for var usage
        if line.trim_start().starts_with("var ") {
            findings.push(
                Finding::new(
                    "code_quality",
                    "var_usage",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "var keyword usage".to_string(),
                )
                .with_description("var has function scope and can lead to unexpected behavior".to_string())
                .with_suggestion("Use let or const instead of var".to_string())
            );
        }

        // Check for == usage
        if line.contains(" == ") && !line.contains(" === ") {
            findings.push(
                Finding::new(
                    "code_quality",
                    "loose_equality",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "Loose equality operator".to_string(),
                )
                .with_description("== performs type coercion which can be unexpected".to_string())
                .with_suggestion("Use === for strict equality comparison".to_string())
            );
        }

        Ok(findings)
    }

    fn check_python_quality(&self, file_path: &Path, line: &str, line_number: u32) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for bare except
        if line.trim() == "except:" {
            findings.push(
                Finding::new(
                    "code_quality",
                    "bare_except",
                    Severity::Medium,
                    file_path.to_path_buf(),
                    line_number,
                    "Bare except clause".to_string(),
                )
                .with_description("Bare except catches all exceptions, including system exits".to_string())
                .with_suggestion("Catch specific exceptions or use 'except Exception:'".to_string())
            );
        }

        // Check for mutable default arguments
        if line.contains("def ") && (line.contains("=[]") || line.contains("={}")) {
            findings.push(
                Finding::new(
                    "code_quality",
                    "mutable_default_arg",
                    Severity::High,
                    file_path.to_path_buf(),
                    line_number,
                    "Mutable default argument".to_string(),
                )
                .with_description("Mutable default arguments can cause unexpected behavior".to_string())
                .with_suggestion("Use None as default and create the mutable object inside the function".to_string())
            );
        }

        Ok(findings)
    }

    fn check_naming_for_language(&self, file_path: &Path, line: &str, line_number: u32, ext: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for single-letter variables (except common ones like i, j, x, y)
        let single_letter_pattern = Regex::new(r"\b([a-hk-wz])\s*=").unwrap();
        if single_letter_pattern.is_match(line) && !line.contains("for ") {
            findings.push(
                Finding::new(
                    "code_quality",
                    "single_letter_var",
                    Severity::Low,
                    file_path.to_path_buf(),
                    line_number,
                    "Single-letter variable name".to_string(),
                )
                .with_description("Single-letter variables reduce code readability".to_string())
                .with_suggestion("Use descriptive variable names".to_string())
            );
        }

        match ext {
            "rs" => {
                // Check Rust naming conventions
                if line.contains("fn ") {
                    let snake_case_pattern = Regex::new(r"fn\s+([A-Z][a-zA-Z0-9_]*)").unwrap();
                    if snake_case_pattern.is_match(line) {
                        findings.push(
                            Finding::new(
                                "code_quality",
                                "rust_naming_convention",
                                Severity::Low,
                                file_path.to_path_buf(),
                                line_number,
                                "Function name should use snake_case".to_string(),
                            )
                            .with_description("Rust convention is to use snake_case for function names".to_string())
                            .with_suggestion("Convert function name to snake_case".to_string())
                        );
                    }
                }
            }
            "js" | "ts" => {
                // Check JavaScript naming conventions
                if line.contains("function ") {
                    let camel_case_pattern = Regex::new(r"function\s+([a-z][a-zA-Z0-9_]*_[a-zA-Z0-9_]*)").unwrap();
                    if camel_case_pattern.is_match(line) {
                        findings.push(
                            Finding::new(
                                "code_quality",
                                "js_naming_convention",
                                Severity::Low,
                                file_path.to_path_buf(),
                                line_number,
                                "Function name should use camelCase".to_string(),
                            )
                            .with_description("JavaScript convention is to use camelCase for function names".to_string())
                            .with_suggestion("Convert function name to camelCase".to_string())
                        );
                    }
                }
            }
            _ => {}
        }

        Ok(findings)
    }
}

impl Analyzer for CodeQualityAnalyzer {
    fn name(&self) -> &str {
        "code_quality"
    }

    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        self.analyze_code_quality(file_path, content)
    }

    fn supports_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(ext, 
                "rs" | "js" | "ts" | "jsx" | "tsx" | "py" | "java" | 
                "cpp" | "c" | "h" | "hpp" | "go" | "rb" | "php" |
                "cs" | "swift" | "kt" | "scala" | "clj" | "hs"
            )
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_magic_number_detection() {
        let analyzer = CodeQualityAnalyzer::new();
        let code = "let timeout = 5000;";
        
        let findings = analyzer.analyze(&PathBuf::from("test.js"), code.as_bytes()).unwrap();
        assert!(findings.iter().any(|f| f.rule == "magic_number"));
    }

    #[test]
    fn test_complex_condition_detection() {
        let analyzer = CodeQualityAnalyzer::new();
        let code = "if (a && b && c || d && e && f) {";
        
        let findings = analyzer.analyze(&PathBuf::from("test.js"), code.as_bytes()).unwrap();
        assert!(findings.iter().any(|f| f.rule == "complex_condition"));
    }

    #[test]
    fn test_unwrap_detection() {
        let analyzer = CodeQualityAnalyzer::new();
        let code = "let value = result.unwrap();";
        
        let findings = analyzer.analyze(&PathBuf::from("test.rs"), code.as_bytes()).unwrap();
        assert!(findings.iter().any(|f| f.rule == "unwrap_usage"));
    }
}