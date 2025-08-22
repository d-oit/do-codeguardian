// Test file to trigger security analysis workflows
use std::collections::HashMap;

fn main() {
    // This file contains intentional security issues for testing
    
    // 1. Hardcoded API key (should be detected)
    let api_key = "sk-test-1234567890abcdef"; // TODO: Move to environment variable
    
    // 2. Potential SQL injection vulnerability
    let user_input = "'; DROP TABLE users; --";
    let query = format!("SELECT * FROM users WHERE name = '{}'", user_input);
    
    // 3. Insecure random number generation
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    "seed".hash(&mut hasher);
    let weak_random = hasher.finish();
    
    // 4. Debug information leak
    #[cfg(debug_assertions)]
    println!("Debug: API key is {}", api_key); // FIXME: Remove debug output
    
    // 5. Inefficient algorithm (performance issue)
    let mut data = HashMap::new();
    for i in 0..10000 {
        for j in 0..1000 {
            if i * j > 50000 {
                data.insert(format!("key_{}_{}", i, j), i + j);
            }
        }
    }
    
    println!("Test completed with {} entries", data.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_security_sample() {
        // Test that should trigger security analysis
        main();
    }
}