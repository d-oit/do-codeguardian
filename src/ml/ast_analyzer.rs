use anyhow::Result;
use std::path::Path;

#[cfg(feature = "ast")]
use syn::{visit::Visit, Block, Expr, Item, ItemFn, Stmt};

/// AST-based code analysis for enhanced feature extraction
pub struct AstAnalyzer {
    enabled: bool,
}

impl AstAnalyzer {
    pub fn new() -> Self {
        Self {
            enabled: cfg!(feature = "ast"),
        }
    }

    /// Extract AST-based features from Rust source code
    pub fn extract_ast_features(&self, file_path: &Path, content: &str) -> Result<AstFeatures> {
        if !self.enabled {
            return Ok(AstFeatures::default());
        }

        // Only analyze Rust files
        if file_path.extension().is_none_or(|ext| ext != "rs") {
            return Ok(AstFeatures::default());
        }

        #[cfg(feature = "ast")]
        {
            self.analyze_rust_ast(content)
        }

        #[cfg(not(feature = "ast"))]
        Ok(AstFeatures::default())
    }

    #[cfg(feature = "ast")]
    fn analyze_rust_ast(&self, content: &str) -> Result<AstFeatures> {
        // Parse the Rust source code into an AST
        let syntax_tree = syn::parse_file(content)
            .map_err(|e| anyhow::anyhow!("Failed to parse Rust code: {}", e))?;

        let mut visitor = AstVisitor::new();
        visitor.visit_file(&syntax_tree);

        Ok(visitor.into_features())
    }
}

/// AST-derived features for enhanced ML classification
#[derive(Debug, Clone, Default)]
pub struct AstFeatures {
    /// Code complexity metrics
    pub cyclomatic_complexity: f32,
    pub nesting_depth: f32,
    pub function_count: f32,
    pub struct_count: f32,
    pub enum_count: f32,
    pub impl_block_count: f32,

    /// Security-relevant patterns
    pub unsafe_block_count: f32,
    pub panic_call_count: f32,
    pub unwrap_call_count: f32,
    pub expect_call_count: f32,

    /// Code quality indicators
    pub comment_density: f32,
    pub documentation_coverage: f32,
    pub test_function_ratio: f32,

    /// Semantic patterns
    pub string_literal_count: f32,
    pub numeric_literal_count: f32,
    pub macro_usage_count: f32,
}

impl AstFeatures {
    /// Convert to normalized feature vector for ML
    pub fn to_feature_vector(&self) -> Vec<f32> {
        vec![
            // Complexity features (0-1 normalized)
            (self.cyclomatic_complexity / 20.0).min(1.0),
            (self.nesting_depth / 10.0).min(1.0),
            (self.function_count / 50.0).min(1.0),
            (self.struct_count / 20.0).min(1.0),
            (self.enum_count / 10.0).min(1.0),
            (self.impl_block_count / 20.0).min(1.0),
            // Security risk features (0-1 normalized)
            (self.unsafe_block_count / 5.0).min(1.0),
            (self.panic_call_count / 10.0).min(1.0),
            (self.unwrap_call_count / 20.0).min(1.0),
            (self.expect_call_count / 10.0).min(1.0),
            // Quality features (0-1 normalized)
            self.comment_density.min(1.0),
            self.documentation_coverage.min(1.0),
            self.test_function_ratio.min(1.0),
            // Pattern features (0-1 normalized)
            (self.string_literal_count / 50.0).min(1.0),
            (self.numeric_literal_count / 30.0).min(1.0),
            (self.macro_usage_count / 20.0).min(1.0),
        ]
    }

    /// Get feature names for debugging and analysis
    pub fn feature_names() -> Vec<&'static str> {
        vec![
            "cyclomatic_complexity",
            "nesting_depth",
            "function_count",
            "struct_count",
            "enum_count",
            "impl_block_count",
            "unsafe_block_count",
            "panic_call_count",
            "unwrap_call_count",
            "expect_call_count",
            "comment_density",
            "documentation_coverage",
            "test_function_ratio",
            "string_literal_count",
            "numeric_literal_count",
            "macro_usage_count",
        ]
    }
}

#[cfg(feature = "ast")]
struct AstVisitor {
    features: AstFeatures,
    current_nesting_depth: u32,
    max_nesting_depth: u32,
    total_lines: u32,
    comment_lines: u32,
    documented_items: u32,
    total_items: u32,
    test_functions: u32,
}

#[cfg(feature = "ast")]
impl AstVisitor {
    fn new() -> Self {
        Self {
            features: AstFeatures::default(),
            current_nesting_depth: 0,
            max_nesting_depth: 0,
            total_lines: 0,
            comment_lines: 0,
            documented_items: 0,
            total_items: 0,
            test_functions: 0,
        }
    }

    fn into_features(mut self) -> AstFeatures {
        // Calculate derived metrics
        if self.total_lines > 0 {
            self.features.comment_density = self.comment_lines as f32 / self.total_lines as f32;
        }

        if self.total_items > 0 {
            self.features.documentation_coverage =
                self.documented_items as f32 / self.total_items as f32;
        }

        if self.features.function_count > 0.0 {
            self.features.test_function_ratio =
                self.test_functions as f32 / self.features.function_count;
        }

        self.features.nesting_depth = self.max_nesting_depth as f32;

        self.features
    }

    fn enter_scope(&mut self) {
        self.current_nesting_depth += 1;
        self.max_nesting_depth = self.max_nesting_depth.max(self.current_nesting_depth);
    }

    fn exit_scope(&mut self) {
        if self.current_nesting_depth > 0 {
            self.current_nesting_depth -= 1;
        }
    }

    fn analyze_function_complexity(&mut self, func: &ItemFn) {
        // Simple cyclomatic complexity calculation
        let mut complexity = 1; // Base complexity

        if let Some(block) = func.block.stmts.iter().find_map(|stmt| {
            if let Stmt::Expr(expr, _) = stmt {
                Some(expr)
            } else {
                None
            }
        }) {
            complexity += self.count_decision_points(block);
        }

        self.features.cyclomatic_complexity += complexity as f32;
    }

    fn count_decision_points(&self, expr: &Expr) -> u32 {
        match expr {
            Expr::If(_) => 1,
            Expr::Match(match_expr) => match_expr.arms.len() as u32,
            Expr::While(_) => 1,
            Expr::ForLoop(_) => 1,
            Expr::Loop(_) => 1,
            _ => 0,
        }
    }

    fn is_test_function(&self, func: &ItemFn) -> bool {
        func.attrs.iter().any(|attr| {
            attr.path().is_ident("test")
                || attr.path().segments.iter().any(|seg| seg.ident == "test")
        })
    }

    fn has_documentation(&self, attrs: &[syn::Attribute]) -> bool {
        attrs.iter().any(|attr| attr.path().is_ident("doc"))
    }

    fn count_method_calls(&mut self, expr: &Expr, method_name: &str) {
        match expr {
            Expr::MethodCall(method_call) => {
                if method_call.method == method_name {
                    match method_name {
                        "unwrap" => self.features.unwrap_call_count += 1.0,
                        "expect" => self.features.expect_call_count += 1.0,
                        "panic" => self.features.panic_call_count += 1.0,
                        _ => {}
                    }
                }
            }
            Expr::Macro(macro_expr) => {
                if let Some(ident) = macro_expr.mac.path.get_ident() {
                    if ident == "panic" {
                        self.features.panic_call_count += 1.0;
                    }
                    self.features.macro_usage_count += 1.0;
                }
            }
            Expr::Lit(lit_expr) => match &lit_expr.lit {
                syn::Lit::Str(_) => self.features.string_literal_count += 1.0,
                syn::Lit::Int(_) | syn::Lit::Float(_) => self.features.numeric_literal_count += 1.0,
                _ => {}
            },
            _ => {}
        }
    }
}

#[cfg(feature = "ast")]
impl<'ast> Visit<'ast> for AstVisitor {
    fn visit_item(&mut self, item: &'ast Item) {
        self.total_items += 1;

        match item {
            Item::Fn(func) => {
                self.features.function_count += 1.0;
                if self.has_documentation(&func.attrs) {
                    self.documented_items += 1;
                }
                if self.is_test_function(func) {
                    self.test_functions += 1;
                }
                self.analyze_function_complexity(func);
            }
            Item::Struct(struct_item) => {
                self.features.struct_count += 1.0;
                if self.has_documentation(&struct_item.attrs) {
                    self.documented_items += 1;
                }
            }
            Item::Enum(enum_item) => {
                self.features.enum_count += 1.0;
                if self.has_documentation(&enum_item.attrs) {
                    self.documented_items += 1;
                }
            }
            Item::Impl(_impl_item) => {
                self.features.impl_block_count += 1.0;
            }
            _ => {}
        }

        syn::visit::visit_item(self, item);
    }

    fn visit_block(&mut self, block: &'ast Block) {
        self.enter_scope();
        syn::visit::visit_block(self, block);
        self.exit_scope();
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        // Count various method calls and patterns
        self.count_method_calls(expr, "unwrap");
        self.count_method_calls(expr, "expect");
        self.count_method_calls(expr, "panic");

        // Count unsafe blocks
        if let Expr::Unsafe(_) = expr {
            self.features.unsafe_block_count += 1.0;
        }

        syn::visit::visit_expr(self, expr);
    }
}

impl Default for AstAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_analyzer_creation() -> Result<(), Box<dyn std::error::Error>> {
        let analyzer = AstAnalyzer::new();
        assert_eq!(analyzer.enabled, cfg!(feature = "ast"));
    }

    #[cfg(feature = "ast")]
    #[test]
    fn test_rust_ast_analysis() -> Result<(), Box<dyn std::error::Error>> {
        let analyzer = AstAnalyzer::new();
        let code = r#"
            /// A test function
            #[test]
            fn test_example() -> Result<(), Box<dyn std::error::Error>> {
                let x = "hello".to_string();
                let y = x?;
                if y.len() > 0 {
                    println!("Not empty");
                }
            }

            struct MyStruct {
                field: i32,
            }

            unsafe fn dangerous_function() {
                // Unsafe code here
            }
        "#;

        let features = analyzer.extract_ast_features(Path::new("test.rs"), code)?;

        assert!(features.function_count > 0.0);
        assert!(features.struct_count > 0.0);
        assert!(features.unsafe_block_count > 0.0);
        assert!(features.string_literal_count > 0.0);

        let feature_vector = features.to_feature_vector();
        assert_eq!(feature_vector.len(), 16);
    }

    #[test]
    fn test_feature_vector_normalization() -> Result<(), Box<dyn std::error::Error>> {
        let features = AstFeatures {
            cyclomatic_complexity: 25.0, // Should be clamped to 1.0
            function_count: 100.0,       // Should be clamped to 1.0
            comment_density: 0.5,        // Should remain 0.5
            ..Default::default()
        };

        let vector = features.to_feature_vector();
        assert_eq!(vector[0], 1.0); // Clamped complexity
        assert_eq!(vector[2], 1.0); // Clamped function count
        assert_eq!(vector[10], 0.5); // Unchanged comment density
    }
}
