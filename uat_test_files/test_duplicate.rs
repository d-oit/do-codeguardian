// Test file with duplicate code patterns
use std::collections::HashMap;

fn process_data(data: &HashMap<String, String>) -> Vec<String> {
    let mut result = Vec::new();
    for (key, value) in data {
        if key.len() > 5 {
            result.push(format!("{}: {}", key, value));
        }
    }
    result
}

fn process_data_similar(data: &HashMap<String, String>) -> Vec<String> {
    let mut result = Vec::new();
    for (key, value) in data {
        if key.len() > 5 {
            result.push(format!("{}: {}", key, value));
        }
    }
    result
}

// Memory leak potential
fn create_large_vector() -> Vec<i32> {
    let mut vec = Vec::with_capacity(1000000);
    for i in 0..1000000 {
        vec.push(i);
    }
    vec // Not using the vector, potential memory waste
}

fn main() {
    println!("Test file with duplicate patterns and performance issues");
}
