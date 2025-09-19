//! Multi-language AST analyzer for enhanced duplicate detection across programming languages
//!
//! This module provides AST-based analysis for multiple programming languages to improve
//! duplicate code detection accuracy by understanding code structure and semantics.

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

/// Language-specific AST features for duplicate detection
#[derive(Debug, Clone, Default)]
pub struct LanguageAstFeatures {
    /// Language identifier
    pub language: String,
    /// Structural complexity metrics
    pub complexity_score: f32,
    /// Function/method definitions
    pub function_count: f32,
    /// Class/struct definitions
    pub type_definition_count: f32,
    /// Control flow statements
    pub control_flow_count: f32,
    /// Variable declarations
    pub variable_count: f32,
    /// Import/include statements
    pub import_count: f32,
    /// Comment density
    pub comment_density: f32,
    /// Nesting depth
    pub max_nesting_depth: f32,
    /// Token-based features for similarity
    pub token_frequencies: HashMap<String, f32>,
    /// Semantic patterns specific to the language
    pub semantic_patterns: Vec<String>,
}

/// Multi-language AST analyzer
pub struct MultiLanguageAstAnalyzer {
    analyzers: HashMap<String, Box<dyn LanguageAstAnalyzer>>,
}

impl MultiLanguageAstAnalyzer {
    pub fn new() -> Self {
        let mut analyzers = HashMap::new();

        // Register language analyzers
        analyzers.insert(
            "rust".to_string(),
            Box::new(RustAstAnalyzer::new()) as Box<dyn LanguageAstAnalyzer>,
        );
        analyzers.insert(
            "python".to_string(),
            Box::new(PythonAstAnalyzer::new()) as Box<dyn LanguageAstAnalyzer>,
        );
        analyzers.insert(
            "javascript".to_string(),
            Box::new(JavaScriptAstAnalyzer::new()) as Box<dyn LanguageAstAnalyzer>,
        );
        analyzers.insert(
            "typescript".to_string(),
            Box::new(TypeScriptAstAnalyzer::new()) as Box<dyn LanguageAstAnalyzer>,
        );
        analyzers.insert(
            "java".to_string(),
            Box::new(JavaAstAnalyzer::new()) as Box<dyn LanguageAstAnalyzer>,
        );
        analyzers.insert(
            "cpp".to_string(),
            Box::new(CppAstAnalyzer::new()) as Box<dyn LanguageAstAnalyzer>,
        );
        analyzers.insert(
            "c".to_string(),
            Box::new(CAstAnalyzer::new()) as Box<dyn LanguageAstAnalyzer>,
        );
        analyzers.insert(
            "go".to_string(),
            Box::new(GoAstAnalyzer::new()) as Box<dyn LanguageAstAnalyzer>,
        );
        analyzers.insert(
            "php".to_string(),
            Box::new(PHPAstAnalyzer::new()) as Box<dyn LanguageAstAnalyzer>,
        );

        Self { analyzers }
    }

    /// Analyze code for a specific language and extract AST features
    pub fn analyze_code(&self, file_path: &Path, content: &str) -> Result<LanguageAstFeatures> {
        let language = self.detect_language(file_path)?;

        if let Some(analyzer) = self.analyzers.get(&language) {
            analyzer.analyze(content)
        } else {
            // Fallback to generic text analysis
            Ok(self.generic_text_analysis(content, &language))
        }
    }

    /// Detect programming language from file extension
    fn detect_language(&self, file_path: &Path) -> Result<String> {
        let extension = file_path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow::anyhow!("No file extension found"))?;

        match extension.to_lowercase().as_str() {
            "rs" => Ok("rust".to_string()),
            "py" => Ok("python".to_string()),
            "js" => Ok("javascript".to_string()),
            "jsx" => Ok("javascript".to_string()),
            "ts" => Ok("typescript".to_string()),
            "tsx" => Ok("typescript".to_string()),
            "java" => Ok("java".to_string()),
            "cpp" | "cc" | "cxx" => Ok("cpp".to_string()),
            "c" | "h" => Ok("c".to_string()),
            "go" => Ok("go".to_string()),
            "php" => Ok("php".to_string()),
            "rb" => Ok("ruby".to_string()),
            "cs" => Ok("csharp".to_string()),
            "swift" => Ok("swift".to_string()),
            "kt" => Ok("kotlin".to_string()),
            "scala" => Ok("scala".to_string()),
            "dart" => Ok("dart".to_string()),
            _ => Err(anyhow::anyhow!("Unsupported file extension: {}", extension)),
        }
    }

    /// Generic text analysis for unsupported languages
    fn generic_text_analysis(&self, content: &str, language: &str) -> LanguageAstFeatures {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len() as f32;

        // Basic tokenization
        let tokens: Vec<&str> = content.split_whitespace().collect();
        let mut token_frequencies = HashMap::new();

        for token in &tokens {
            *token_frequencies.entry(token.to_string()).or_insert(0.0) += 1.0;
        }

        // Normalize frequencies
        for freq in token_frequencies.values_mut() {
            *freq /= tokens.len() as f32;
        }

        // Count comments
        let comment_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("//")
                    || trimmed.starts_with("#")
                    || trimmed.starts_with("/*")
                    || trimmed.contains("*/")
            })
            .count() as f32;

        LanguageAstFeatures {
            language: language.to_string(),
            complexity_score: (tokens.len() as f32 / total_lines.max(1.0)).min(10.0),
            function_count: 0.0, // Can't detect without language-specific parsing
            type_definition_count: 0.0,
            control_flow_count: 0.0,
            variable_count: tokens.iter().filter(|t| t.contains('=')).count() as f32,
            import_count: 0.0,
            comment_density: comment_count / total_lines.max(1.0),
            max_nesting_depth: 0.0,
            token_frequencies,
            semantic_patterns: Vec::new(),
        }
    }

    /// Calculate similarity between two AST feature sets
    pub fn calculate_ast_similarity(
        &self,
        features1: &LanguageAstFeatures,
        features2: &LanguageAstFeatures,
    ) -> f64 {
        if features1.language != features2.language {
            // Different languages - use token-based similarity only
            return self.calculate_token_similarity(
                &features1.token_frequencies,
                &features2.token_frequencies,
            );
        }

        // Same language - use comprehensive similarity
        let mut similarity = 0.0f64;
        let mut weight_total = 0.0f64;

        // Complexity similarity (weighted 0.2)
        let complexity_sim =
            1.0 - (features1.complexity_score - features2.complexity_score).abs() / 10.0;
        similarity += complexity_sim * 0.2;
        weight_total += 0.2;

        // Function count similarity (weighted 0.15)
        let func_sim = 1.0
            - (features1.function_count - features2.function_count).abs()
                / features1
                    .function_count
                    .max(features2.function_count)
                    .max(1.0);
        similarity += func_sim * 0.15;
        weight_total += 0.15;

        // Type definition similarity (weighted 0.15)
        let type_sim = 1.0
            - (features1.type_definition_count - features2.type_definition_count).abs()
                / features1
                    .type_definition_count
                    .max(features2.type_definition_count)
                    .max(1.0);
        similarity += type_sim * 0.15;
        weight_total += 0.15;

        // Control flow similarity (weighted 0.1)
        let control_sim = 1.0
            - (features1.control_flow_count - features2.control_flow_count).abs()
                / features1
                    .control_flow_count
                    .max(features2.control_flow_count)
                    .max(1.0);
        similarity += control_sim * 0.1;
        weight_total += 0.1;

        // Comment density similarity (weighted 0.1)
        let comment_sim = 1.0 - (features1.comment_density - features2.comment_density).abs();
        similarity += comment_sim * 0.1;
        weight_total += 0.1;

        // Nesting depth similarity (weighted 0.1)
        let nesting_sim = 1.0
            - (features1.max_nesting_depth - features2.max_nesting_depth).abs()
                / features1
                    .max_nesting_depth
                    .max(features2.max_nesting_depth)
                    .max(1.0);
        similarity += nesting_sim * 0.1;
        weight_total += 0.1;

        // Token similarity (weighted 0.2)
        let token_sim = self
            .calculate_token_similarity(&features1.token_frequencies, &features2.token_frequencies);
        similarity += token_sim * 0.2;
        weight_total += 0.2;

        if weight_total > 0.0 {
            similarity / weight_total
        } else {
            0.0
        }
    }

    /// Calculate token-based similarity
    fn calculate_token_similarity(
        &self,
        tokens1: &HashMap<String, f32>,
        tokens2: &HashMap<String, f32>,
    ) -> f64 {
        if tokens1.is_empty() && tokens2.is_empty() {
            return 1.0;
        }

        let mut intersection = 0.0;
        let mut union = 0.0;

        // Calculate intersection and union of token frequencies
        for (token, freq1) in tokens1 {
            if let Some(freq2) = tokens2.get(token) {
                intersection += (freq1.min(*freq2)) as f64;
            }
            union += *freq1 as f64;
        }

        for freq2 in tokens2.values() {
            union += *freq2 as f64;
        }

        // Subtract double-counted intersection
        for (token, _) in tokens1 {
            if tokens2.contains_key(token) {
                if let Some(freq2) = tokens2.get(token) {
                    union -= *freq2 as f64;
                }
            }
        }

        if union == 0.0 {
            0.0
        } else {
            intersection / union
        }
    }
}

/// Trait for language-specific AST analyzers
trait LanguageAstAnalyzer {
    fn analyze(&self, content: &str) -> Result<LanguageAstFeatures>;
}

// Rust AST Analyzer
struct RustAstAnalyzer;

impl RustAstAnalyzer {
    fn new() -> Self {
        Self
    }
}

impl LanguageAstAnalyzer for RustAstAnalyzer {
    fn analyze(&self, content: &str) -> Result<LanguageAstFeatures> {
        // Use existing Rust AST analyzer
        let rust_analyzer = crate::ml::ast_analyzer::AstAnalyzer::new();
        let ast_features = rust_analyzer.extract_ast_features(Path::new("temp.rs"), content)?;

        let mut token_frequencies = HashMap::new();
        // Basic tokenization for Rust
        for token in content.split_whitespace() {
            if token.len() > 2 {
                // Filter out short tokens
                *token_frequencies.entry(token.to_string()).or_insert(0.0) += 1.0;
            }
        }

        // Normalize frequencies
        let total_tokens = token_frequencies.values().sum::<f32>();
        if total_tokens > 0.0 {
            for freq in token_frequencies.values_mut() {
                *freq /= total_tokens;
            }
        }

        Ok(LanguageAstFeatures {
            language: "rust".to_string(),
            complexity_score: ast_features.cyclomatic_complexity,
            function_count: ast_features.function_count,
            type_definition_count: ast_features.struct_count + ast_features.enum_count,
            control_flow_count: ast_features.panic_call_count + ast_features.unwrap_call_count,
            variable_count: 0.0, // Not available in current AST features
            import_count: 0.0,   // Not available in current AST features
            comment_density: ast_features.comment_density,
            max_nesting_depth: ast_features.nesting_depth,
            token_frequencies,
            semantic_patterns: vec![
                "fn ".to_string(),
                "struct ".to_string(),
                "enum ".to_string(),
                "impl ".to_string(),
                "unsafe ".to_string(),
            ],
        })
    }
}

// Python AST Analyzer
struct PythonAstAnalyzer;

impl PythonAstAnalyzer {
    fn new() -> Self {
        Self
    }
}

impl LanguageAstAnalyzer for PythonAstAnalyzer {
    fn analyze(&self, content: &str) -> Result<LanguageAstFeatures> {
        let mut features = LanguageAstFeatures {
            language: "python".to_string(),
            ..Default::default()
        };

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len() as f32;

        // Count functions
        features.function_count = lines
            .iter()
            .filter(|line| line.trim().starts_with("def "))
            .count() as f32;

        // Count classes
        features.type_definition_count = lines
            .iter()
            .filter(|line| line.trim().starts_with("class "))
            .count() as f32;

        // Count control flow
        features.control_flow_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("if ")
                    || trimmed.starts_with("for ")
                    || trimmed.starts_with("while ")
                    || trimmed.starts_with("try:")
                    || trimmed.starts_with("with ")
            })
            .count() as f32;

        // Count imports
        features.import_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("import ") || trimmed.starts_with("from ")
            })
            .count() as f32;

        // Count comments
        let comment_count = lines
            .iter()
            .filter(|line| line.trim().starts_with('#'))
            .count() as f32;
        features.comment_density = comment_count / total_lines.max(1.0);

        // Calculate nesting depth
        let mut max_depth = 0;
        let mut current_depth = 0;
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Count leading spaces (assuming 4 spaces per indent)
            let indent_level = line.len() - trimmed.len();
            current_depth = (indent_level / 4) as u32;
            max_depth = max_depth.max(current_depth);
        }
        features.max_nesting_depth = max_depth as f32;

        // Token frequencies
        let mut token_frequencies = HashMap::new();
        for token in content.split_whitespace() {
            if token.len() > 2 && !token.contains('.') && !token.contains('(') {
                *token_frequencies.entry(token.to_string()).or_insert(0.0) += 1.0;
            }
        }

        let total_tokens = token_frequencies.values().sum::<f32>();
        if total_tokens > 0.0 {
            for freq in token_frequencies.values_mut() {
                *freq /= total_tokens;
            }
        }
        features.token_frequencies = token_frequencies;

        // Complexity score based on various factors
        features.complexity_score = (features.function_count * 2.0
            + features.control_flow_count
            + features.max_nesting_depth)
            .min(10.0);

        features.semantic_patterns = vec![
            "def ".to_string(),
            "class ".to_string(),
            "if ".to_string(),
            "for ".to_string(),
            "import ".to_string(),
        ];

        Ok(features)
    }
}

// JavaScript/TypeScript AST Analyzer
struct JavaScriptAstAnalyzer;

impl JavaScriptAstAnalyzer {
    fn new() -> Self {
        Self
    }
}

impl LanguageAstAnalyzer for JavaScriptAstAnalyzer {
    fn analyze(&self, content: &str) -> Result<LanguageAstFeatures> {
        let mut features = LanguageAstFeatures {
            language: "javascript".to_string(),
            ..Default::default()
        };

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len() as f32;

        // Count functions
        features.function_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.contains("function ")
                    || trimmed.contains("=>")
                    || trimmed.contains("const ") && trimmed.contains("=") && trimmed.contains("=>")
            })
            .count() as f32;

        // Count classes/types
        features.type_definition_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("class ")
                    || trimmed.starts_with("interface ")
                    || trimmed.starts_with("type ")
            })
            .count() as f32;

        // Count control flow
        features.control_flow_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("if ")
                    || trimmed.starts_with("for ")
                    || trimmed.starts_with("while ")
                    || trimmed.starts_with("try ")
                    || trimmed.starts_with("catch ")
            })
            .count() as f32;

        // Count imports
        features.import_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("import ") || trimmed.starts_with("require(")
            })
            .count() as f32;

        // Count comments
        let comment_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("//") || trimmed.starts_with("/*")
            })
            .count() as f32;
        features.comment_density = comment_count / total_lines.max(1.0);

        // Calculate nesting depth
        let mut max_depth = 0;
        let mut current_depth = 0;
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Count braces for nesting
            let open_braces = line.matches('{').count() as i32;
            let close_braces = line.matches('}').count() as i32;
            current_depth += open_braces - close_braces;
            max_depth = max_depth.max(current_depth.max(0) as u32);
        }
        features.max_nesting_depth = max_depth as f32;

        // Token frequencies
        let mut token_frequencies = HashMap::new();
        for token in content.split_whitespace() {
            if token.len() > 2
                && !token.contains('.')
                && !token.contains('(')
                && !token.contains('{')
            {
                *token_frequencies.entry(token.to_string()).or_insert(0.0) += 1.0;
            }
        }

        let total_tokens = token_frequencies.values().sum::<f32>();
        if total_tokens > 0.0 {
            for freq in token_frequencies.values_mut() {
                *freq /= total_tokens;
            }
        }
        features.token_frequencies = token_frequencies;

        // Complexity score
        features.complexity_score = (features.function_count * 1.5
            + features.control_flow_count * 0.8
            + features.max_nesting_depth * 0.5)
            .min(10.0);

        features.semantic_patterns = vec![
            "function ".to_string(),
            "class ".to_string(),
            "const ".to_string(),
            "let ".to_string(),
            "var ".to_string(),
            "if ".to_string(),
            "for ".to_string(),
        ];

        Ok(features)
    }
}

// TypeScript AST Analyzer (extends JavaScript)
struct TypeScriptAstAnalyzer;

impl TypeScriptAstAnalyzer {
    fn new() -> Self {
        Self
    }
}

impl LanguageAstAnalyzer for TypeScriptAstAnalyzer {
    fn analyze(&self, content: &str) -> Result<LanguageAstFeatures> {
        // Use JavaScript analyzer as base
        let mut features = JavaScriptAstAnalyzer::new().analyze(content)?;
        features.language = "typescript".to_string();

        // Add TypeScript-specific patterns
        features.semantic_patterns.extend(vec![
            "interface ".to_string(),
            "type ".to_string(),
            "enum ".to_string(),
            ": ".to_string(), // Type annotations
            "<".to_string(),  // Generics
        ]);

        // TypeScript typically has higher type definition count
        let lines: Vec<&str> = content.lines().collect();
        features.type_definition_count += lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.contains(": ") || // Type annotations
                trimmed.contains("<") && trimmed.contains(">") // Generics
            })
            .count() as f32
            * 0.5; // Weight type annotations less

        Ok(features)
    }
}

// Java AST Analyzer
struct JavaAstAnalyzer;

impl JavaAstAnalyzer {
    fn new() -> Self {
        Self
    }
}

impl LanguageAstAnalyzer for JavaAstAnalyzer {
    fn analyze(&self, content: &str) -> Result<LanguageAstFeatures> {
        let mut features = LanguageAstFeatures {
            language: "java".to_string(),
            ..Default::default()
        };

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len() as f32;

        // Count methods
        features.function_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                (trimmed.contains("public ")
                    || trimmed.contains("private ")
                    || trimmed.contains("protected "))
                    && trimmed.contains("(")
                    && trimmed.contains(")")
            })
            .count() as f32;

        // Count classes/interfaces
        features.type_definition_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("class ")
                    || trimmed.starts_with("interface ")
                    || trimmed.starts_with("enum ")
            })
            .count() as f32;

        // Count control flow
        features.control_flow_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("if ")
                    || trimmed.starts_with("for ")
                    || trimmed.starts_with("while ")
                    || trimmed.starts_with("try ")
                    || trimmed.starts_with("catch ")
                    || trimmed.starts_with("switch ")
            })
            .count() as f32;

        // Count imports
        features.import_count = lines
            .iter()
            .filter(|line| line.trim().starts_with("import "))
            .count() as f32;

        // Count comments
        let comment_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("*")
            })
            .count() as f32;
        features.comment_density = comment_count / total_lines.max(1.0);

        // Calculate nesting depth
        let mut max_depth = 0;
        let mut current_depth = 0;
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let open_braces = line.matches('{').count() as i32;
            let close_braces = line.matches('}').count() as i32;
            current_depth += open_braces - close_braces;
            max_depth = max_depth.max(current_depth.max(0) as u32);
        }
        features.max_nesting_depth = max_depth as f32;

        // Token frequencies
        let mut token_frequencies = HashMap::new();
        for token in content.split_whitespace() {
            if token.len() > 2 && !token.contains('.') && !token.contains('(') {
                *token_frequencies.entry(token.to_string()).or_insert(0.0) += 1.0;
            }
        }

        let total_tokens = token_frequencies.values().sum::<f32>();
        if total_tokens > 0.0 {
            for freq in token_frequencies.values_mut() {
                *freq /= total_tokens;
            }
        }
        features.token_frequencies = token_frequencies;

        features.complexity_score = (features.function_count * 2.0
            + features.control_flow_count
            + features.max_nesting_depth)
            .min(10.0);

        features.semantic_patterns = vec![
            "public ".to_string(),
            "private ".to_string(),
            "class ".to_string(),
            "interface ".to_string(),
            "void ".to_string(),
            "String ".to_string(),
        ];

        Ok(features)
    }
}

// C++ AST Analyzer
struct CppAstAnalyzer;

impl CppAstAnalyzer {
    fn new() -> Self {
        Self
    }
}

impl LanguageAstAnalyzer for CppAstAnalyzer {
    fn analyze(&self, content: &str) -> Result<LanguageAstFeatures> {
        let mut features = LanguageAstFeatures {
            language: "cpp".to_string(),
            ..Default::default()
        };

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len() as f32;

        // Count functions
        features.function_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.starts_with('#') && // Not preprocessor
                trimmed.contains("(") &&
                trimmed.contains(")") &&
                (trimmed.contains("int ") || trimmed.contains("void ") ||
                 trimmed.contains("bool ") || trimmed.contains("char ") ||
                 trimmed.contains("float ") || trimmed.contains("double "))
            })
            .count() as f32;

        // Count classes/structs
        features.type_definition_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("class ")
                    || trimmed.starts_with("struct ")
                    || trimmed.starts_with("enum ")
            })
            .count() as f32;

        // Count control flow
        features.control_flow_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("if ")
                    || trimmed.starts_with("for ")
                    || trimmed.starts_with("while ")
                    || trimmed.starts_with("try ")
                    || trimmed.starts_with("catch ")
                    || trimmed.starts_with("switch ")
            })
            .count() as f32;

        // Count includes
        features.import_count = lines
            .iter()
            .filter(|line| line.trim().starts_with("#include"))
            .count() as f32;

        // Count comments
        let comment_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("//") || trimmed.starts_with("/*")
            })
            .count() as f32;
        features.comment_density = comment_count / total_lines.max(1.0);

        // Calculate nesting depth
        let mut max_depth = 0;
        let mut current_depth = 0;
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let open_braces = line.matches('{').count() as i32;
            let close_braces = line.matches('}').count() as i32;
            current_depth += open_braces - close_braces;
            max_depth = max_depth.max(current_depth.max(0) as u32);
        }
        features.max_nesting_depth = max_depth as f32;

        // Token frequencies
        let mut token_frequencies = HashMap::new();
        for token in content.split_whitespace() {
            if token.len() > 2 && !token.starts_with('#') && !token.contains('(') {
                *token_frequencies.entry(token.to_string()).or_insert(0.0) += 1.0;
            }
        }

        let total_tokens = token_frequencies.values().sum::<f32>();
        if total_tokens > 0.0 {
            for freq in token_frequencies.values_mut() {
                *freq /= total_tokens;
            }
        }
        features.token_frequencies = token_frequencies;

        features.complexity_score = (features.function_count * 2.0
            + features.control_flow_count
            + features.max_nesting_depth)
            .min(10.0);

        features.semantic_patterns = vec![
            "int ".to_string(),
            "void ".to_string(),
            "class ".to_string(),
            "struct ".to_string(),
            "#include ".to_string(),
            "const ".to_string(),
        ];

        Ok(features)
    }
}

// C AST Analyzer
struct CAstAnalyzer;

impl CAstAnalyzer {
    fn new() -> Self {
        Self
    }
}

impl LanguageAstAnalyzer for CAstAnalyzer {
    fn analyze(&self, content: &str) -> Result<LanguageAstFeatures> {
        // Similar to C++ but simpler
        let mut features = CppAstAnalyzer::new().analyze(content)?;
        features.language = "c".to_string();

        // C has fewer features than C++
        features.type_definition_count *= 0.7; // Reduce struct count weight
        features.complexity_score *= 0.9; // Generally simpler than C++

        features.semantic_patterns = vec![
            "int ".to_string(),
            "void ".to_string(),
            "char ".to_string(),
            "float ".to_string(),
            "#include ".to_string(),
            "const ".to_string(),
        ];

        Ok(features)
    }
}

// Go AST Analyzer
struct GoAstAnalyzer;

impl GoAstAnalyzer {
    fn new() -> Self {
        Self
    }
}

impl LanguageAstAnalyzer for GoAstAnalyzer {
    fn analyze(&self, content: &str) -> Result<LanguageAstFeatures> {
        let mut features = LanguageAstFeatures {
            language: "go".to_string(),
            ..Default::default()
        };

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len() as f32;

        // Count functions
        features.function_count = lines
            .iter()
            .filter(|line| line.trim().starts_with("func "))
            .count() as f32;

        // Count types/structs
        features.type_definition_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("type ") || trimmed.starts_with("struct ")
            })
            .count() as f32;

        // Count control flow
        features.control_flow_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("if ")
                    || trimmed.starts_with("for ")
                    || trimmed.starts_with("switch ")
                    || trimmed.starts_with("select ")
            })
            .count() as f32;

        // Count imports
        features.import_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("import ")
                    || trimmed.starts_with("\t\"")
                    || trimmed.starts_with(" \"")
            })
            .count() as f32;

        // Count comments
        let comment_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("//") || trimmed.starts_with("/*")
            })
            .count() as f32;
        features.comment_density = comment_count / total_lines.max(1.0);

        // Calculate nesting depth
        let mut max_depth = 0;
        let mut current_depth = 0;
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let open_braces = line.matches('{').count() as i32;
            let close_braces = line.matches('}').count() as i32;
            current_depth += open_braces - close_braces;
            max_depth = max_depth.max(current_depth.max(0) as u32);
        }
        features.max_nesting_depth = max_depth as f32;

        // Token frequencies
        let mut token_frequencies = HashMap::new();
        for token in content.split_whitespace() {
            if token.len() > 2 && !token.contains('(') && !token.contains('{') {
                *token_frequencies.entry(token.to_string()).or_insert(0.0) += 1.0;
            }
        }

        let total_tokens = token_frequencies.values().sum::<f32>();
        if total_tokens > 0.0 {
            for freq in token_frequencies.values_mut() {
                *freq /= total_tokens;
            }
        }
        features.token_frequencies = token_frequencies;

        features.complexity_score = (features.function_count * 1.8
            + features.control_flow_count
            + features.max_nesting_depth)
            .min(10.0);

        features.semantic_patterns = vec![
            "func ".to_string(),
            "type ".to_string(),
            "struct ".to_string(),
            "interface ".to_string(),
            "package ".to_string(),
            "import ".to_string(),
        ];

        Ok(features)
    }
}

// PHP AST Analyzer
struct PHPAstAnalyzer;

impl PHPAstAnalyzer {
    fn new() -> Self {
        Self
    }
}

impl LanguageAstAnalyzer for PHPAstAnalyzer {
    fn analyze(&self, content: &str) -> Result<LanguageAstFeatures> {
        let mut features = LanguageAstFeatures {
            language: "php".to_string(),
            ..Default::default()
        };

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len() as f32;

        // Count functions
        features.function_count = lines
            .iter()
            .filter(|line| line.trim().starts_with("function "))
            .count() as f32;

        // Count classes
        features.type_definition_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("class ")
                    || trimmed.starts_with("interface ")
                    || trimmed.starts_with("trait ")
            })
            .count() as f32;

        // Count control flow
        features.control_flow_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("if ")
                    || trimmed.starts_with("for ")
                    || trimmed.starts_with("foreach ")
                    || trimmed.starts_with("while ")
                    || trimmed.starts_with("try ")
                    || trimmed.starts_with("catch ")
            })
            .count() as f32;

        // Count includes/requires
        features.import_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("include ")
                    || trimmed.starts_with("require ")
                    || trimmed.starts_with("include_once ")
                    || trimmed.starts_with("require_once ")
            })
            .count() as f32;

        // Count comments
        let comment_count = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("//") || trimmed.starts_with("#") || trimmed.starts_with("/*")
            })
            .count() as f32;
        features.comment_density = comment_count / total_lines.max(1.0);

        // Calculate nesting depth
        let mut max_depth = 0;
        let mut current_depth = 0;
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let open_braces = line.matches('{').count() as i32;
            let close_braces = line.matches('}').count() as i32;
            current_depth += open_braces - close_braces;
            max_depth = max_depth.max(current_depth.max(0) as u32);
        }
        features.max_nesting_depth = max_depth as f32;

        // Token frequencies
        let mut token_frequencies = HashMap::new();
        for token in content.split_whitespace() {
            if token.len() > 2 && !token.starts_with('$') && !token.contains('(') {
                *token_frequencies.entry(token.to_string()).or_insert(0.0) += 1.0;
            }
        }

        let total_tokens = token_frequencies.values().sum::<f32>();
        if total_tokens > 0.0 {
            for freq in token_frequencies.values_mut() {
                *freq /= total_tokens;
            }
        }
        features.token_frequencies = token_frequencies;

        features.complexity_score = (features.function_count * 1.5
            + features.control_flow_count
            + features.max_nesting_depth)
            .min(10.0);

        features.semantic_patterns = vec![
            "function ".to_string(),
            "class ".to_string(),
            "$".to_string(),
            "public ".to_string(),
            "private ".to_string(),
            "if ".to_string(),
        ];

        Ok(features)
    }
}

impl Default for MultiLanguageAstAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_language_analyzer_creation() {
        let analyzer = MultiLanguageAstAnalyzer::new();
        assert!(analyzer.analyzers.contains_key("rust"));
        assert!(analyzer.analyzers.contains_key("python"));
        assert!(analyzer.analyzers.contains_key("javascript"));
    }

    #[test]
    fn test_language_detection() {
        let analyzer = MultiLanguageAstAnalyzer::new();

        assert_eq!(
            analyzer
                .detect_language(Path::new("test.rs"))
                .expect("Failed to detect Rust"),
            "rust"
        );
        assert_eq!(
            analyzer
                .detect_language(Path::new("test.py"))
                .expect("Failed to detect Python"),
            "python"
        );
        assert_eq!(
            analyzer
                .detect_language(Path::new("test.js"))
                .expect("Failed to detect JavaScript"),
            "javascript"
        );
        assert_eq!(
            analyzer
                .detect_language(Path::new("test.java"))
                .expect("Failed to detect Java"),
            "java"
        );
        assert!(analyzer.detect_language(Path::new("test.unknown")).is_err());
    }

    #[test]
    fn test_python_ast_analysis() {
        let analyzer = MultiLanguageAstAnalyzer::new();
        let code = r#"
def authenticate_user(username, password):
    """Authenticate a user"""
    if username and password:
        return True
    return False

class UserManager:
    def __init__(self):
        self.users = []

    def add_user(self, user):
        self.users.append(user)
"#;

        let features = analyzer
            .analyze_code(Path::new("test.py"), code)
            .expect("Failed to analyze Python code");
        assert_eq!(features.language, "python");
        assert!(features.function_count >= 2.0); // authenticate_user and add_user
        assert!(features.type_definition_count >= 1.0); // UserManager class
        assert!(features.control_flow_count >= 1.0); // if statement
    }

    #[test]
    fn test_javascript_ast_analysis() {
        let analyzer = MultiLanguageAstAnalyzer::new();
        let code = r#"
// User authentication module
function authenticateUser(username, password) {
    if (username && password) {
        return true;
    }
    return false;
}

class UserManager {
    constructor() {
        this.users = [];
    }

    addUser(user) {
        this.users.push(user);
    }
}
"#;

        let features = analyzer
            .analyze_code(Path::new("test.js"), code)
            .expect("Failed to analyze JavaScript code");
        assert_eq!(features.language, "javascript");
        assert!(features.function_count >= 2.0); // authenticateUser and addUser
        assert!(features.type_definition_count >= 1.0); // UserManager class
        assert!(features.control_flow_count >= 1.0); // if statement
    }

    #[test]
    fn test_similarity_calculation() {
        let analyzer = MultiLanguageAstAnalyzer::new();

        let features1 = LanguageAstFeatures {
            language: "python".to_string(),
            complexity_score: 5.0,
            function_count: 2.0,
            type_definition_count: 1.0,
            control_flow_count: 1.0,
            comment_density: 0.2,
            max_nesting_depth: 2.0,
            token_frequencies: {
                let mut map = HashMap::new();
                map.insert("def".to_string(), 0.3);
                map.insert("class".to_string(), 0.2);
                map
            },
            ..Default::default()
        };

        let features2 = LanguageAstFeatures {
            language: "python".to_string(),
            complexity_score: 5.5,
            function_count: 2.0,
            type_definition_count: 1.0,
            control_flow_count: 1.0,
            comment_density: 0.25,
            max_nesting_depth: 2.0,
            token_frequencies: {
                let mut map = HashMap::new();
                map.insert("def".to_string(), 0.3);
                map.insert("class".to_string(), 0.2);
                map
            },
            ..Default::default()
        };

        let similarity = analyzer.calculate_ast_similarity(&features1, &features2);
        assert!(similarity > 0.8); // Should be very similar
    }

    #[test]
    fn test_different_languages_similarity() {
        let analyzer = MultiLanguageAstAnalyzer::new();

        let python_features = LanguageAstFeatures {
            language: "python".to_string(),
            token_frequencies: {
                let mut map = HashMap::new();
                map.insert("def".to_string(), 0.5);
                map
            },
            ..Default::default()
        };

        let js_features = LanguageAstFeatures {
            language: "javascript".to_string(),
            token_frequencies: {
                let mut map = HashMap::new();
                map.insert("function".to_string(), 0.5);
                map
            },
            ..Default::default()
        };

        let similarity = analyzer.calculate_ast_similarity(&python_features, &js_features);
        assert!(similarity < 0.5); // Should be low similarity for different languages
    }
}
