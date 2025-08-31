// Simple test to verify AST functionality
fn main() {
    #[cfg(all(feature = "ml", feature = "ast"))]
    {
        println!("✅ AST features are enabled!");
        
        // Test basic AST parsing
        let test_code = r#"
            fn test() {
                let x = "hello";
                if x.len() > 0 {
                    println!("Not empty");
                }
            }
        "#;
        
        match syn::parse_file(test_code) {
            Ok(_) => println!("✅ AST parsing works!"),
            Err(e) => println!("❌ AST parsing failed: {}", e),
        }
    }
    
    #[cfg(not(all(feature = "ml", feature = "ast")))]
    {
        println!("❌ AST features not enabled");
        println!("Build with: cargo run --features ml,ast");
    }
}