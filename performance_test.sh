#!/bin/bash

# CodeGuardian Performance Test Script
# This script runs comprehensive performance tests to measure improvements

set -e

echo "🚀 CodeGuardian Performance Test Suite"
echo "====================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Create test files for benchmarking
create_test_files() {
    print_status "Creating test files for benchmarking..."

    # Create a directory with test files
    mkdir -p test_performance

    # Create various file types for testing
    for i in {1..100}; do
        # Create Rust files with different patterns
        cat > "test_performance/test_$i.rs" << EOF
use std::collections::HashMap;

fn example_function() {
    let mut map = HashMap::new();

    // Some nested loops to test performance detection
    for i in 0..100 {
        for j in 0..50 {
            map.insert(i, j);
        }
    }

    // String concatenation in loop
    let mut result = String::new();
    for i in 0..100 {
        result = result + &i.to_string(); // Inefficient
    }

    println!("Result: {}", result);
}

fn main() {
    example_function();
}
EOF
    done

    # Create some large files
    for i in {1..5}; do
        cat > "test_performance/large_$i.rs" << EOF
// Large file for streaming tests
$(for j in {1..10000}; do echo "// Line $j: This is a comment with some code patterns"; done)

fn large_function() {
$(for j in {1..5000}; do echo "    println!(\"Line $j\");"; done)
}
EOF
    done

    print_status "Test files created successfully"
}

# Run benchmarks
run_benchmarks() {
    print_status "Running Criterion benchmarks..."

    if command -v cargo &> /dev/null; then
        cargo bench
    else
        print_warning "Cargo not found, skipping benchmarks"
    fi
}

# Test different configurations
test_configurations() {
    print_status "Testing different performance configurations..."

    # Test default configuration
    print_status "Testing default configuration..."
    time cargo run --release . --format json > /dev/null 2>&1

    # Test CI optimized configuration
    print_status "Testing CI optimized configuration..."
    time cargo run --release . --config ci_optimized --format json > /dev/null 2>&1

    # Test maximum performance configuration
    print_status "Testing maximum performance configuration..."
    time cargo run --release . --config maximum_performance --format json > /dev/null 2>&1
}

# Memory usage test
test_memory_usage() {
    print_status "Testing memory usage..."

    if command -v /usr/bin/time &> /dev/null; then
        print_status "Memory usage with default config:"
        /usr/bin/time -v cargo run --release . --format json > /dev/null 2>&1

        print_status "Memory usage with maximum performance config:"
        /usr/bin/time -v cargo run --release . --config maximum_performance --format json > /dev/null 2>&1
    else
        print_warning "GNU time not found, skipping memory tests"
    fi
}

# Cleanup function
cleanup() {
    print_status "Cleaning up test files..."
    rm -rf test_performance
}

# Main execution
main() {
    print_status "Starting performance tests..."

    # Set up
    create_test_files

    # Run tests
    run_benchmarks
    test_configurations
    test_memory_usage

    # Cleanup
    cleanup

    print_status "Performance tests completed!"
    print_status "Check the results above and in target/criterion/ for detailed benchmarks"
}

# Trap to ensure cleanup on exit
trap cleanup EXIT

# Run main function
main "$@"