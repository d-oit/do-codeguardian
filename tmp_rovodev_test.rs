// Simple test file for CodeGuardian analysis
fn main() {
    // TODO: This is a security-related comment
    let password = "hardcoded_password"; // This should be flagged
    println!("Hello, world!");
    
    // FIXME: Implement proper error handling
    let _result = std::fs::read_to_string("/etc/passwd").unwrap();
}