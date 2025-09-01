#!/bin/bash
# CodeGuardian Performance Analysis Script
# Runs comprehensive performance analysis and benchmarks

set -e

echo "⚡ CodeGuardian Performance Analysis"
echo "===================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Run performance analysis test
echo -e "${BLUE}Running performance bottleneck analysis...${NC}"
cargo test tmp_rovodev_performance_analysis::performance_analysis::analyze_current_performance_bottlenecks -- --nocapture

# Run benchmarks if available
echo -e "${BLUE}Running performance benchmarks...${NC}"
if cargo bench --list >/dev/null 2>&1; then
    echo -e "${GREEN}Running comprehensive benchmarks...${NC}"
    cargo bench --bench comprehensive_performance_benchmark
    
    echo -e "${GREEN}Running hashing benchmarks...${NC}"
    cargo bench --bench hashing_benchmark
    
    echo -e "${GREEN}Running existing performance benchmarks...${NC}"
    cargo bench --bench performance_benchmark
else
    echo -e "${YELLOW}Benchmarks not available or criterion not configured${NC}"
fi

# Run performance regression tests
echo -e "${BLUE}Running performance regression tests...${NC}"
cargo test performance_regression_tests --release -- --nocapture

echo -e "${GREEN}✅ Performance analysis complete!${NC}"
echo -e "${BLUE}Check the output above for performance insights and recommendations.${NC}"