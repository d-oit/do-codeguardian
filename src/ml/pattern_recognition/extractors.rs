//! Feature extractors for different pattern types

use super::FeatureExtractor;
use anyhow::Result;
use std::collections::HashMap;

/// Code structure feature extractor
pub struct CodeStructureExtractor {
    feature_names: Vec<String>,
}

impl CodeStructureExtractor {
    pub fn new() -> Self {
        let feature_names = vec![
            "line_count".to_string(),
            "function_count".to_string(),
            "class_count".to_string(),
            "complexity_score".to_string(),
            "nesting_depth".to_string(),
            "comment_ratio".to_string(),
            "import_count".to_string(),
            "variable_count".to_string(),
            "loop_count".to_string(),
            "conditional_count".to_string(),
            "string_literal_count".to_string(),
            "numeric_literal_count".to_string(),
            "method_call_count".to_string(),
            "assignment_count".to_string(),
            "return_statement_count".to_string(),
        ];

        Self { feature_names }
    }

    fn extract_code_metrics(&self, content: &str) -> Vec<f64> {
        let lines: Vec<&str> = content.lines().collect();
        let line_count = lines.len() as f64;

        let mut function_count = 0.0;
        let mut class_count = 0.0;
        let mut nesting_depth = 0.0;
        let mut max_nesting = 0.0;
        let mut comment_lines = 0.0;
        let mut import_count = 0.0;
        let mut loop_count = 0.0;
        let mut conditional_count = 0.0;
        let mut string_literal_count = 0.0;
        let mut numeric_literal_count = 0.0;
        let mut method_call_count = 0.0;
        let mut assignment_count = 0.0;
        let mut return_statement_count = 0.0;

        for line in &lines {
            let trimmed = line.trim();

            // Count comments
            if trimmed.starts_with("//") || trimmed.starts_with("#") || trimmed.starts_with("/*") {
                comment_lines += 1.0;
            }

            // Count functions/methods
            if trimmed.contains("fn ") || trimmed.contains("def ") || trimmed.contains("function ")
            {
                function_count += 1.0;
            }

            // Count classes/structs
            if trimmed.starts_with("class ")
                || trimmed.starts_with("struct ")
                || trimmed.starts_with("impl ")
            {
                class_count += 1.0;
            }

            // Count imports
            if trimmed.starts_with("use ")
                || trimmed.starts_with("import ")
                || trimmed.starts_with("from ")
            {
                import_count += 1.0;
            }

            // Count loops
            if trimmed.contains("for ") || trimmed.contains("while ") || trimmed.contains("loop ") {
                loop_count += 1.0;
            }

            // Count conditionals
            if trimmed.contains("if ") || trimmed.contains("else ") || trimmed.contains("match ") {
                conditional_count += 1.0;
            }

            // Count string literals
            string_literal_count += count_string_literals(line);

            // Count numeric literals
            numeric_literal_count += count_numeric_literals(line);

            // Count method calls
            method_call_count += count_method_calls(line);

            // Count assignments
            if line.contains("=") && !line.contains("==") && !line.contains("!=") {
                assignment_count += 1.0;
            }

            // Count return statements
            if trimmed.starts_with("return ") || trimmed == "return" {
                return_statement_count += 1.0;
            }

            // Calculate nesting depth
            let current_nesting = calculate_nesting_depth(line);
            nesting_depth += current_nesting;
            max_nesting = max_nesting.max(current_nesting);
        }

        let comment_ratio = if line_count > 0.0 {
            comment_lines / line_count
        } else {
            0.0
        };
        let avg_nesting = if line_count > 0.0 {
            nesting_depth / line_count
        } else {
            0.0
        };
        let complexity_score = function_count + loop_count + conditional_count + max_nesting;

        vec![
            line_count,
            function_count,
            class_count,
            complexity_score,
            avg_nesting,
            comment_ratio,
            import_count,
            line_count, // variable_count placeholder
            loop_count,
            conditional_count,
            string_literal_count,
            numeric_literal_count,
            method_call_count,
            assignment_count,
            return_statement_count,
        ]
    }
}

impl FeatureExtractor for CodeStructureExtractor {
    fn extract_features(
        &self,
        content: &str,
        _metadata: &HashMap<String, String>,
    ) -> Result<Vec<f64>> {
        Ok(self.extract_code_metrics(content))
    }

    fn get_feature_names(&self) -> Vec<String> {
        self.feature_names.clone()
    }

    fn get_feature_dimension(&self) -> usize {
        self.feature_names.len()
    }
}

/// Documentation content feature extractor
pub struct DocumentationExtractor {
    feature_names: Vec<String>,
}

impl DocumentationExtractor {
    pub fn new() -> Self {
        let feature_names = vec![
            "word_count".to_string(),
            "sentence_count".to_string(),
            "paragraph_count".to_string(),
            "heading_count".to_string(),
            "link_count".to_string(),
            "code_block_count".to_string(),
            "list_count".to_string(),
            "table_count".to_string(),
            "image_count".to_string(),
            "readability_score".to_string(),
            "technical_term_density".to_string(),
            "average_sentence_length".to_string(),
        ];

        Self { feature_names }
    }
}

impl FeatureExtractor for DocumentationExtractor {
    fn extract_features(
        &self,
        content: &str,
        _metadata: &HashMap<String, String>,
    ) -> Result<Vec<f64>> {
        let word_count = content.split_whitespace().count() as f64;
        let sentence_count = content.matches('.').count() as f64;
        let paragraph_count = content.split("\n\n").count() as f64;
        let heading_count = content.matches('#').count() as f64;
        let link_count = content.matches("http").count() as f64;
        let code_block_count = content.matches("```").count() as f64 / 2.0;
        let list_count = content.matches("- ").count() as f64;
        let table_count = content.matches('|').count() as f64;
        let image_count = content.matches("![").count() as f64;

        let readability_score = calculate_readability_score(content);
        let technical_term_density = calculate_technical_density(content);
        let avg_sentence_length = if sentence_count > 0.0 {
            word_count / sentence_count
        } else {
            0.0
        };

        Ok(vec![
            word_count,
            sentence_count,
            paragraph_count,
            heading_count,
            link_count,
            code_block_count,
            list_count,
            table_count,
            image_count,
            readability_score,
            technical_term_density,
            avg_sentence_length,
        ])
    }

    fn get_feature_names(&self) -> Vec<String> {
        self.feature_names.clone()
    }

    fn get_feature_dimension(&self) -> usize {
        self.feature_names.len()
    }
}

/// Configuration values feature extractor
pub struct ConfigurationExtractor {
    feature_names: Vec<String>,
}

impl ConfigurationExtractor {
    pub fn new() -> Self {
        let feature_names = vec![
            "key_value_pairs".to_string(),
            "nested_levels".to_string(),
            "array_count".to_string(),
            "boolean_count".to_string(),
            "numeric_values".to_string(),
            "string_values".to_string(),
            "null_values".to_string(),
            "comment_lines".to_string(),
            "section_count".to_string(),
            "environment_variables".to_string(),
        ];

        Self { feature_names }
    }
}

impl FeatureExtractor for ConfigurationExtractor {
    fn extract_features(
        &self,
        content: &str,
        metadata: &HashMap<String, String>,
    ) -> Result<Vec<f64>> {
        let file_extension = metadata
            .get("file_extension")
            .map(|s| s.as_str())
            .unwrap_or("");

        match file_extension {
            "toml" => self.extract_toml_features(content),
            "json" => self.extract_json_features(content),
            "yaml" | "yml" => self.extract_yaml_features(content),
            _ => self.extract_generic_config_features(content),
        }
    }

    fn get_feature_names(&self) -> Vec<String> {
        self.feature_names.clone()
    }

    fn get_feature_dimension(&self) -> usize {
        self.feature_names.len()
    }
}

impl ConfigurationExtractor {
    fn extract_toml_features(&self, content: &str) -> Result<Vec<f64>> {
        let lines: Vec<&str> = content.lines().collect();
        let mut key_value_pairs = 0.0;
        let mut nested_levels = 0.0;
        let mut array_count = 0.0;
        let mut boolean_count = 0.0;
        let mut numeric_values = 0.0;
        let mut string_values = 0.0;
        let mut null_values = 0.0;
        let mut comment_lines = 0.0;
        let mut section_count = 0.0;
        let mut environment_variables = 0.0;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.starts_with('#') {
                comment_lines += 1.0;
            } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
                section_count += 1.0;
                nested_levels += count_dots(trimmed);
            } else if trimmed.contains('=') {
                key_value_pairs += 1.0;

                if trimmed.contains("true") || trimmed.contains("false") {
                    boolean_count += 1.0;
                } else if trimmed.contains('[') {
                    array_count += 1.0;
                } else if trimmed.contains('"') {
                    string_values += 1.0;
                } else if trimmed.chars().any(|c| c.is_ascii_digit()) {
                    numeric_values += 1.0;
                }

                if trimmed.contains("${") || trimmed.contains("$ENV") {
                    environment_variables += 1.0;
                }
            }
        }

        Ok(vec![
            key_value_pairs,
            nested_levels,
            array_count,
            boolean_count,
            numeric_values,
            string_values,
            null_values,
            comment_lines,
            section_count,
            environment_variables,
        ])
    }

    fn extract_json_features(&self, content: &str) -> Result<Vec<f64>> {
        // Simplified JSON feature extraction
        let key_value_pairs = content.matches(':').count() as f64;
        let nested_levels = content.matches('{').count() as f64;
        let array_count = content.matches('[').count() as f64;
        let boolean_count =
            content.matches("true").count() as f64 + content.matches("false").count() as f64;
        let null_values = content.matches("null").count() as f64;
        let string_values = content.matches('"').count() as f64 / 2.0;
        let numeric_values = count_json_numbers(content);

        Ok(vec![
            key_value_pairs,
            nested_levels,
            array_count,
            boolean_count,
            numeric_values,
            string_values,
            null_values,
            0.0, // comment_lines (JSON doesn't have comments)
            0.0, // section_count
            0.0, // environment_variables
        ])
    }

    fn extract_yaml_features(&self, content: &str) -> Result<Vec<f64>> {
        let lines: Vec<&str> = content.lines().collect();
        let mut key_value_pairs = 0.0;
        let mut nested_levels = 0.0;
        let mut array_count = 0.0;
        let mut boolean_count = 0.0;
        let mut numeric_values = 0.0;
        let mut string_values = 0.0;
        let mut null_values = 0.0;
        let mut comment_lines = 0.0;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.starts_with('#') {
                comment_lines += 1.0;
            } else if trimmed.contains(':') {
                key_value_pairs += 1.0;
                nested_levels += count_leading_spaces(line) as f64 / 2.0;

                if trimmed.contains("true") || trimmed.contains("false") {
                    boolean_count += 1.0;
                } else if trimmed.contains("null") {
                    null_values += 1.0;
                } else if trimmed.contains('-') && !trimmed.contains(':') {
                    array_count += 1.0;
                } else if trimmed.chars().any(|c| c.is_ascii_digit()) {
                    numeric_values += 1.0;
                } else {
                    string_values += 1.0;
                }
            }
        }

        Ok(vec![
            key_value_pairs,
            nested_levels,
            array_count,
            boolean_count,
            numeric_values,
            string_values,
            null_values,
            comment_lines,
            0.0, // section_count
            0.0, // environment_variables
        ])
    }

    fn extract_generic_config_features(&self, content: &str) -> Result<Vec<f64>> {
        let lines = content.lines().count() as f64;
        let equals_count = content.matches('=').count() as f64;
        let comment_lines = content
            .lines()
            .filter(|line| line.trim().starts_with('#'))
            .count() as f64;

        Ok(vec![
            equals_count,
            0.0, // nested_levels
            0.0, // array_count
            0.0, // boolean_count
            0.0, // numeric_values
            0.0, // string_values
            0.0, // null_values
            comment_lines,
            0.0, // section_count
            0.0, // environment_variables
        ])
    }
}

// Additional feature extractors with simplified implementations
pub struct SecurityPatternExtractor {
    feature_names: Vec<String>,
}

impl SecurityPatternExtractor {
    pub fn new() -> Self {
        let feature_names = vec![
            "sql_injection_patterns".to_string(),
            "xss_patterns".to_string(),
            "hardcoded_secrets".to_string(),
            "unsafe_functions".to_string(),
            "crypto_usage".to_string(),
            "input_validation".to_string(),
            "authentication_patterns".to_string(),
            "authorization_checks".to_string(),
            "logging_security".to_string(),
            "error_handling".to_string(),
        ];

        Self { feature_names }
    }
}

impl FeatureExtractor for SecurityPatternExtractor {
    fn extract_features(
        &self,
        content: &str,
        _metadata: &HashMap<String, String>,
    ) -> Result<Vec<f64>> {
        let sql_patterns = count_sql_injection_patterns(content);
        let xss_patterns = count_xss_patterns(content);
        let secrets = count_hardcoded_secrets(content);
        let unsafe_funcs = count_unsafe_functions(content);
        let crypto_usage = count_crypto_usage(content);
        let input_validation = count_input_validation(content);
        let auth_patterns = count_auth_patterns(content);
        let authz_checks = count_authz_checks(content);
        let logging_security = count_logging_security(content);
        let error_handling = count_error_handling(content);

        Ok(vec![
            sql_patterns,
            xss_patterns,
            secrets,
            unsafe_funcs,
            crypto_usage,
            input_validation,
            auth_patterns,
            authz_checks,
            logging_security,
            error_handling,
        ])
    }

    fn get_feature_names(&self) -> Vec<String> {
        self.feature_names.clone()
    }

    fn get_feature_dimension(&self) -> usize {
        self.feature_names.len()
    }
}

// Implement remaining extractors with similar patterns
macro_rules! impl_simple_extractor {
    ($name:ident, $features:expr) => {
        pub struct $name {
            feature_names: Vec<String>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    feature_names: $features.iter().map(|s| s.to_string()).collect(),
                }
            }
        }

        impl FeatureExtractor for $name {
            fn extract_features(
                &self,
                content: &str,
                _metadata: &HashMap<String, String>,
            ) -> Result<Vec<f64>> {
                // Simplified implementation - return zeros for now
                Ok(vec![0.0; self.feature_names.len()])
            }

            fn get_feature_names(&self) -> Vec<String> {
                self.feature_names.clone()
            }

            fn get_feature_dimension(&self) -> usize {
                self.feature_names.len()
            }
        }
    };
}

impl_simple_extractor!(
    PerformancePatternExtractor,
    &[
        "loop_complexity",
        "memory_allocations",
        "io_operations",
        "database_queries",
        "network_calls",
        "caching_usage",
        "algorithm_complexity",
        "resource_usage"
    ]
);

impl_simple_extractor!(
    NamingPatternExtractor,
    &[
        "camel_case_usage",
        "snake_case_usage",
        "pascal_case_usage",
        "kebab_case_usage",
        "naming_consistency",
        "abbreviation_usage",
        "length_distribution",
        "convention_adherence"
    ]
);

impl_simple_extractor!(
    ApiUsageExtractor,
    &[
        "api_call_count",
        "error_handling_patterns",
        "retry_mechanisms",
        "timeout_configurations",
        "rate_limiting",
        "authentication_usage",
        "response_handling",
        "async_patterns"
    ]
);

impl_simple_extractor!(
    DataFlowExtractor,
    &[
        "data_transformations",
        "pipeline_stages",
        "data_validation",
        "serialization_usage",
        "data_persistence",
        "streaming_patterns",
        "batch_processing",
        "data_quality_checks"
    ]
);

// Helper functions
fn count_string_literals(line: &str) -> f64 {
    let mut count = 0.0;
    let mut in_string = false;
    let mut escape_next = false;

    for ch in line.chars() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '\\' => escape_next = true,
            '"' | '\'' => {
                if !in_string {
                    in_string = true;
                    count += 1.0;
                } else {
                    in_string = false;
                }
            }
            _ => {}
        }
    }

    count
}

fn count_numeric_literals(line: &str) -> f64 {
    line.split_whitespace()
        .filter(|word| word.chars().any(|c| c.is_ascii_digit()))
        .count() as f64
}

fn count_method_calls(line: &str) -> f64 {
    line.matches('(').count() as f64
}

fn calculate_nesting_depth(line: &str) -> f64 {
    let open_braces = line.matches('{').count();
    let close_braces = line.matches('}').count();
    (open_braces as f64) - (close_braces as f64)
}

fn calculate_readability_score(content: &str) -> f64 {
    // Simplified readability score
    let words = content.split_whitespace().count() as f64;
    let sentences = content.matches('.').count() as f64;
    let syllables = words * 1.5; // Rough estimate

    if sentences > 0.0 && words > 0.0 {
        206.835 - (1.015 * (words / sentences)) - (84.6 * (syllables / words))
    } else {
        0.0
    }
}

fn calculate_technical_density(content: &str) -> f64 {
    let technical_terms = [
        "algorithm",
        "implementation",
        "configuration",
        "optimization",
        "performance",
        "security",
        "authentication",
        "authorization",
        "encryption",
        "decryption",
        "serialization",
        "deserialization",
    ];

    let total_words = content.split_whitespace().count() as f64;
    let technical_count = technical_terms
        .iter()
        .map(|term| content.matches(term).count())
        .sum::<usize>() as f64;

    if total_words > 0.0 {
        technical_count / total_words
    } else {
        0.0
    }
}

fn count_dots(text: &str) -> f64 {
    text.matches('.').count() as f64
}

fn count_leading_spaces(line: &str) -> usize {
    line.chars().take_while(|&c| c == ' ').count()
}

fn count_json_numbers(content: &str) -> f64 {
    // Simplified number counting for JSON
    content
        .split(&[',', ':', '{', '}', '[', ']', ' ', '\n', '\t'][..])
        .filter(|s| {
            s.chars()
                .all(|c| c.is_ascii_digit() || c == '.' || c == '-')
        })
        .filter(|s| !s.is_empty())
        .count() as f64
}

// Security pattern detection functions
fn count_sql_injection_patterns(content: &str) -> f64 {
    let patterns = ["SELECT", "INSERT", "UPDATE", "DELETE", "DROP", "UNION"];
    patterns
        .iter()
        .map(|p| content.matches(p).count())
        .sum::<usize>() as f64
}

fn count_xss_patterns(content: &str) -> f64 {
    let patterns = ["<script", "javascript:", "onclick", "onerror", "innerHTML"];
    patterns
        .iter()
        .map(|p| content.matches(p).count())
        .sum::<usize>() as f64
}

fn count_hardcoded_secrets(content: &str) -> f64 {
    let patterns = ["password", "secret", "api_key", "token", "private_key"];
    patterns
        .iter()
        .map(|p| content.matches(p).count())
        .sum::<usize>() as f64
}

fn count_unsafe_functions(content: &str) -> f64 {
    let patterns = ["unsafe", "eval", "exec", "system", "shell_exec"];
    patterns
        .iter()
        .map(|p| content.matches(p).count())
        .sum::<usize>() as f64
}

fn count_crypto_usage(content: &str) -> f64 {
    let patterns = ["encrypt", "decrypt", "hash", "crypto", "cipher"];
    patterns
        .iter()
        .map(|p| content.matches(p).count())
        .sum::<usize>() as f64
}

fn count_input_validation(content: &str) -> f64 {
    let patterns = ["validate", "sanitize", "escape", "filter", "check"];
    patterns
        .iter()
        .map(|p| content.matches(p).count())
        .sum::<usize>() as f64
}

fn count_auth_patterns(content: &str) -> f64 {
    let patterns = ["authenticate", "login", "signin", "auth", "credential"];
    patterns
        .iter()
        .map(|p| content.matches(p).count())
        .sum::<usize>() as f64
}

fn count_authz_checks(content: &str) -> f64 {
    let patterns = ["authorize", "permission", "role", "access", "privilege"];
    patterns
        .iter()
        .map(|p| content.matches(p).count())
        .sum::<usize>() as f64
}

fn count_logging_security(content: &str) -> f64 {
    let patterns = ["log", "audit", "trace", "debug", "monitor"];
    patterns
        .iter()
        .map(|p| content.matches(p).count())
        .sum::<usize>() as f64
}

fn count_error_handling(content: &str) -> f64 {
    let patterns = [
        "try",
        "catch",
        "except",
        "error",
        "exception",
        "Result",
        "Option",
    ];
    patterns
        .iter()
        .map(|p| content.matches(p).count())
        .sum::<usize>() as f64
}
