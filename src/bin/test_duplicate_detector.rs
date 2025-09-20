use anyhow::{Context, Result};
use blake3;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use syn::{Expr, ExprCall, ExprMethodCall, ExprPath, File, Item, ItemFn, Stmt};
use walkdir::WalkDir;

/// Test duplication detector for CodeGuardian
#[derive(Parser)]
#[command(name = "test-duplicate-detector")]
#[command(about = "Detect duplicate patterns in test files")]
struct Args {
    /// Path to analyze (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Output format (json, text)
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Minimum similarity threshold (0.0-1.0)
    #[arg(short, long, default_value = "0.7")]
    threshold: f64,

    /// Output file (defaults to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

/// Test function information
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestFunction {
    name: String,
    file_path: String,
    line_start: usize,
    line_end: usize,
    setup_code: Vec<String>,
    assertions: Vec<String>,
    body_hash: String,
}

/// Duplicate detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DuplicateResult {
    test1: TestFunction,
    test2: TestFunction,
    similarity_score: f64,
    duplicate_type: String,
    shared_patterns: Vec<String>,
}

/// Analysis report
#[derive(Debug, Serialize, Deserialize)]
struct AnalysisReport {
    total_tests: usize,
    duplicates_found: usize,
    results: Vec<DuplicateResult>,
    recommendations: Vec<String>,
}

struct TestDuplicateDetector {
    args: Args,
}

impl TestDuplicateDetector {
    fn new(args: Args) -> Self {
        Self { args }
    }

    async fn run(&self) -> Result<AnalysisReport> {
        println!("🔍 Analyzing test files for duplicates...");

        // Find all test files
        let test_files = self.find_test_files()?;
        println!("📁 Found {} test files", test_files.len());

        // Parse test functions
        let mut all_tests = Vec::new();
        for file_path in test_files {
            let tests = self.parse_test_file(&file_path).await?;
            all_tests.extend(tests);
        }
        println!("🧪 Found {} test functions", all_tests.len());

        // Detect duplicates
        let duplicates = self.detect_duplicates(&all_tests);
        println!("🔍 Found {} potential duplicates", duplicates.len());

        // Generate recommendations
        let recommendations = self.generate_recommendations(&duplicates);

        let report = AnalysisReport {
            total_tests: all_tests.len(),
            duplicates_found: duplicates.len(),
            results: duplicates,
            recommendations,
        };

        Ok(report)
    }

    fn find_test_files(&self) -> Result<Vec<PathBuf>> {
        let mut test_files = Vec::new();

        for entry in WalkDir::new(&self.args.path) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && self.is_test_file(path) {
                test_files.push(path.to_path_buf());
            }
        }

        Ok(test_files)
    }

    fn is_test_file(&self, path: &Path) -> bool {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Test files in tests/ directory
        if path.components().any(|c| c.as_os_str() == "tests") {
            return file_name.ends_with(".rs");
        }

        // Files with test in src/ that contain #[test] or #[cfg(test)]
        if file_name.ends_with(".rs") && path.components().any(|c| c.as_os_str() == "src") {
            if let Ok(content) = fs::read_to_string(path) {
                return content.contains("#[test]") || content.contains("#[cfg(test)]");
            }
        }

        false
    }

    async fn parse_test_file(&self, path: &Path) -> Result<Vec<TestFunction>> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        let syntax = syn::parse_file(&content)
            .with_context(|| format!("Failed to parse Rust file: {}", path.display()))?;

        let mut tests = Vec::new();

        for item in &syntax.items {
            if let Item::Fn(func) = item {
                if self.is_test_function(func) {
                    let test = self.extract_test_info(func, path, &content)?;
                    tests.push(test);
                }
            } else if let Item::Mod(module) = item {
                if let Some((_, items)) = &module.content {
                    for item in items {
                        if let Item::Fn(func) = item {
                            if self.is_test_function(func) {
                                let test = self.extract_test_info(func, path, &content)?;
                                tests.push(test);
                            }
                        }
                    }
                }
            }
        }

        Ok(tests)
    }

    fn is_test_function(&self, func: &ItemFn) -> bool {
        func.attrs.iter().any(|attr| {
            attr.path().is_ident("test")
                || (attr.path().is_ident("cfg")
                    && attr
                        .meta
                        .require_list()
                        .is_ok_and(|list| list.tokens.to_string().contains("test")))
        })
    }

    fn extract_test_info(&self, func: &ItemFn, path: &Path, content: &str) -> Result<TestFunction> {
        let name = func.sig.ident.to_string();
        let file_path = path.to_string_lossy().to_string();

        // Find line numbers (approximate)
        let func_start = content.find(&format!("fn {}", name)).unwrap_or(0);
        let lines: Vec<&str> = content[..func_start].split('\n').collect();
        let line_start = lines.len();

        // Extract setup code and assertions
        let mut setup_code = Vec::new();
        let mut assertions = Vec::new();

        self.analyze_function_body(&func.block.stmts, &mut setup_code, &mut assertions);

        // Create a simple hash of the function body for comparison
        let body_text = format!("{:?}", func.block);
        let body_hash = format!("{:x}", blake3::hash(body_text.as_bytes()));

        Ok(TestFunction {
            name,
            file_path,
            line_start,
            line_end: line_start + func.block.stmts.len(),
            setup_code,
            assertions,
            body_hash,
        })
    }

    fn analyze_function_body(
        &self,
        stmts: &[Stmt],
        setup: &mut Vec<String>,
        assertions: &mut Vec<String>,
    ) {
        for stmt in stmts {
            match stmt {
                Stmt::Expr(expr, _) => self.analyze_expression(expr, setup, assertions),
                Stmt::Semi(expr, _) => self.analyze_expression(expr, setup, assertions),
                Stmt::Item(_) => {} // Skip item statements
                Stmt::Local(local) => {
                    if let Some(init) = &local.init {
                        self.analyze_expression(&init.expr, setup, assertions);
                    }
                }
            }
        }
    }

    fn analyze_expression(
        &self,
        expr: &Expr,
        setup: &mut Vec<String>,
        assertions: &mut Vec<String>,
    ) {
        match expr {
            Expr::Call(call) => self.analyze_call(call, setup, assertions),
            Expr::MethodCall(call) => self.analyze_method_call(call, setup, assertions),
            Expr::Macro(macro_call) => {
                let macro_name = macro_call
                    .mac
                    .path
                    .segments
                    .last()
                    .map(|s| s.ident.to_string())
                    .unwrap_or_default();

                if macro_name == "assert"
                    || macro_name == "assert_eq"
                    || macro_name == "assert_ne"
                    || macro_name == "assert_matches"
                {
                    assertions.push(format!("{}!({})", macro_name, macro_call.mac.tokens));
                }
            }
            _ => {}
        }
    }

    fn analyze_call(&self, call: &ExprCall, setup: &mut Vec<String>, assertions: &mut Vec<String>) {
        if let Expr::Path(path) = &*call.func {
            let func_name = path
                .path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default();

            if func_name.starts_with("assert") {
                assertions.push(format!("{}({})", func_name, "..."));
            } else {
                setup.push(format!("{}({})", func_name, "..."));
            }
        }
    }

    fn analyze_method_call(
        &self,
        call: &ExprMethodCall,
        setup: &mut Vec<String>,
        assertions: &mut Vec<String>,
    ) {
        let method_name = call.method.to_string();

        if method_name.starts_with("assert") {
            assertions.push(format!(".{}({})", method_name, "..."));
        } else {
            setup.push(format!(".{}({})", method_name, "..."));
        }
    }

    fn detect_duplicates(&self, tests: &[TestFunction]) -> Vec<DuplicateResult> {
        let mut duplicates = Vec::new();
        let mut seen_pairs = HashSet::new();

        for i in 0..tests.len() {
            for j in (i + 1)..tests.len() {
                let test1 = &tests[i];
                let test2 = &tests[j];

                // Skip if already processed
                let pair_key = format!("{}:{}", test1.body_hash, test2.body_hash);
                if seen_pairs.contains(&pair_key) {
                    continue;
                }
                seen_pairs.insert(pair_key);

                // Calculate similarity
                let similarity = self.calculate_similarity(test1, test2);

                if similarity >= self.args.threshold {
                    let duplicate_type = self.classify_duplicate(test1, test2);
                    let shared_patterns = self.find_shared_patterns(test1, test2);

                    duplicates.push(DuplicateResult {
                        test1: test1.clone(),
                        test2: test2.clone(),
                        similarity_score: similarity,
                        duplicate_type,
                        shared_patterns,
                    });
                }
            }
        }

        duplicates
    }

    fn calculate_similarity(&self, test1: &TestFunction, test2: &TestFunction) -> f64 {
        // Simple similarity based on shared setup and assertions
        let setup_similarity = self.jaccard_similarity(&test1.setup_code, &test2.setup_code);
        let assertion_similarity = self.jaccard_similarity(&test1.assertions, &test2.assertions);

        // Weight setup more heavily
        (setup_similarity * 0.6) + (assertion_similarity * 0.4)
    }

    fn jaccard_similarity(&self, set1: &[String], set2: &[String]) -> f64 {
        let set1: HashSet<_> = set1.iter().collect();
        let set2: HashSet<_> = set2.iter().collect();

        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    fn classify_duplicate(&self, test1: &TestFunction, test2: &TestFunction) -> String {
        if test1.setup_code == test2.setup_code && test1.assertions == test2.assertions {
            "identical".to_string()
        } else if test1.setup_code == test2.setup_code {
            "same_setup".to_string()
        } else if test1.assertions == test2.assertions {
            "same_assertions".to_string()
        } else {
            "similar_patterns".to_string()
        }
    }

    fn find_shared_patterns(&self, test1: &TestFunction, test2: &TestFunction) -> Vec<String> {
        let mut shared = Vec::new();

        // Shared setup patterns
        let setup1: HashSet<_> = test1.setup_code.iter().collect();
        let setup2: HashSet<_> = test2.setup_code.iter().collect();
        for pattern in setup1.intersection(&setup2) {
            shared.push(format!("Setup: {}", pattern));
        }

        // Shared assertion patterns
        let assert1: HashSet<_> = test1.assertions.iter().collect();
        let assert2: HashSet<_> = test2.assertions.iter().collect();
        for pattern in assert1.intersection(&assert2) {
            shared.push(format!("Assertion: {}", pattern));
        }

        shared
    }

    fn generate_recommendations(&self, duplicates: &[DuplicateResult]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if duplicates.is_empty() {
            recommendations.push("✅ No significant test duplicates found.".to_string());
            return recommendations;
        }

        // Group by type
        let mut type_counts = HashMap::new();
        for dup in duplicates {
            *type_counts.entry(dup.duplicate_type.clone()).or_insert(0) += 1;
        }

        for (dup_type, count) in type_counts {
            match dup_type.as_str() {
                "identical" => {
                    recommendations.push(format!(
                        "🔴 {} identical test functions found. Consider extracting common functionality into helper functions.",
                        count
                    ));
                }
                "same_setup" => {
                    recommendations.push(format!(
                        "🟡 {} test pairs with identical setup code. Consider using setup fixtures or helper functions.",
                        count
                    ));
                }
                "same_assertions" => {
                    recommendations.push(format!(
                        "🟡 {} test pairs with identical assertions. Consider creating assertion helper functions.",
                        count
                    ));
                }
                "similar_patterns" => {
                    recommendations.push(format!(
                        "🟢 {} test pairs with similar patterns. Review for potential consolidation opportunities.",
                        count
                    ));
                }
                _ => {}
            }
        }

        recommendations.push("💡 Consider using test fixtures, parameterized tests, or helper functions to reduce duplication.".to_string());
        recommendations
            .push("📚 Review the duplicate pairs above and refactor accordingly.".to_string());

        recommendations
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let detector = TestDuplicateDetector::new(args);

    let report = detector.run().await?;

    // Output the report
    match detector.args.format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&report)?;
            if let Some(output_path) = &detector.args.output {
                fs::write(output_path, json)?;
                println!("📄 Report saved to: {}", output_path.display());
            } else {
                println!("{}", json);
            }
        }
        "text" => {
            print_text_report(&report);
        }
        _ => {
            eprintln!("❌ Unsupported format: {}", detector.args.format);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn print_text_report(report: &AnalysisReport) {
    println!("\n📊 Test Duplication Analysis Report");
    println!("==================================");
    println!("Total test functions: {}", report.total_tests);
    println!("Duplicates found: {}", report.duplicates_found);
    println!();

    if !report.results.is_empty() {
        println!("🔍 Duplicate Details:");
        println!("---------------------");

        for (i, dup) in report.results.iter().enumerate() {
            println!("{}. {} vs {}", i + 1, dup.test1.name, dup.test2.name);
            println!("   Type: {}", dup.duplicate_type);
            println!("   Similarity: {:.2}%", dup.similarity_score * 100.0);
            println!(
                "   Files: {} vs {}",
                dup.test1.file_path, dup.test2.file_path
            );

            if !dup.shared_patterns.is_empty() {
                println!("   Shared patterns:");
                for pattern in &dup.shared_patterns {
                    println!("     - {}", pattern);
                }
            }
            println!();
        }
    }

    println!("💡 Recommendations:");
    println!("------------------");
    for rec in &report.recommendations {
        println!("• {}", rec);
    }
}
