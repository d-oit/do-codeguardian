// Complex Rust code to demonstrate AST analysis capabilities
use std::collections::HashMap;

/// A function with high cyclomatic complexity
pub fn complex_security_function(data: &[u8], mode: ProcessingMode) -> Result<ProcessedData, SecurityError> {
    let mut result = ProcessedData::new();
    
    // Decision point 1: Input validation
    if data.is_empty() {
        return Err(SecurityError::EmptyInput);
    }
    
    // Decision point 2: Mode selection
    match mode {
        ProcessingMode::Safe => {
            for (index, &byte) in data.iter().enumerate() {
                // Decision point 3: Byte validation
                if byte > 127 {
                    return Err(SecurityError::InvalidByte(index));
                }
                
                // Decision point 4: Special handling
                match byte {
                    0x00..=0x1F => {
                        // Control characters - risky!
                        result.add_warning(format!("Control character at {}", index));
                    }
                    0x20..=0x7E => {
                        // Printable ASCII
                        result.add_byte(byte);
                    }
                    _ => {
                        // Extended ASCII - potential issue
                        result.add_warning(format!("Extended ASCII at {}", index));
                        result.add_byte(byte);
                    }
                }
            }
        }
        ProcessingMode::Fast => {
            // SECURITY RISK: Unsafe block for performance
            unsafe {
                let ptr = data.as_ptr();
                let len = data.len();
                
                // Decision point 5: Length check
                if len > 1024 {
                    return Err(SecurityError::TooLarge);
                }
                
                // Direct memory access - very risky!
                for i in 0..len {
                    let byte = *ptr.add(i);
                    result.add_byte(byte);
                }
            }
        }
        ProcessingMode::Debug => {
            // Debug mode with lots of unwraps - risky!
            let string_data = String::from_utf8(data.to_vec()).unwrap(); // Panic risk!
            let lines: Vec<&str> = string_data.lines().collect();
            
            // Decision point 6: Line processing
            for (line_num, line) in lines.iter().enumerate() {
                if line.is_empty() {
                    continue;
                }
                
                // More unwraps - compounding risk!
                let parsed: i32 = line.parse().unwrap(); // Another panic risk!
                result.add_number(parsed);
                
                // Decision point 7: Range validation
                if parsed < 0 {
                    panic!("Negative numbers not allowed!"); // Direct panic!
                }
            }
        }
    }
    
    Ok(result)
}

#[derive(Debug)]
pub enum ProcessingMode {
    Safe,
    Fast,
    Debug,
}

#[derive(Debug)]
pub enum SecurityError {
    EmptyInput,
    InvalidByte(usize),
    TooLarge,
    ParseError(String),
}

#[derive(Debug)]
pub struct ProcessedData {
    bytes: Vec<u8>,
    numbers: Vec<i32>,
    warnings: Vec<String>,
}

impl ProcessedData {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            numbers: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn add_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }
    
    pub fn add_number(&mut self, num: i32) {
        self.numbers.push(num);
    }
    
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

// TODO: Add proper error handling
// FIXME: Remove unsafe code before production
// DEBUG: This function needs optimization

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_mode() {
        let data = b"hello world";
        let result = complex_security_function(data, ProcessingMode::Safe);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_empty_input() {
        let data = b"";
        let result = complex_security_function(data, ProcessingMode::Safe);
        assert!(result.is_err());
    }
}