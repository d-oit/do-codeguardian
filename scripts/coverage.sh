#!/bin/bash
# CodeGuardian Coverage Script
# Generates comprehensive code coverage reports

set -e

echo "🧪 CodeGuardian Code Coverage Analysis"
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if cargo-tarpaulin is installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo -e "${YELLOW}Installing cargo-tarpaulin...${NC}"
    cargo install cargo-tarpaulin
fi

# Create coverage directory
mkdir -p coverage

echo -e "${BLUE}Running code coverage analysis...${NC}"

# Generate coverage with multiple output formats
cargo tarpaulin \
    --verbose \
    --all-features \
    --workspace \
    --timeout 120 \
    --out Html,Xml,Json \
    --output-dir coverage/ \
    --exclude-files 'target/*' 'tests/fixtures/*' 'benches/*' 'examples/*' 'tmp_*' \
    --line \
    --branch \
    --count

# Check if coverage was generated successfully
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Coverage analysis completed successfully!${NC}"
    
    # Extract coverage percentage from JSON report
    if [ -f "coverage/tarpaulin-report.json" ]; then
        COVERAGE=$(cat coverage/tarpaulin-report.json | grep -o '"coverage":[0-9.]*' | cut -d':' -f2)
        echo -e "${BLUE}📊 Overall Coverage: ${COVERAGE}%${NC}"
        
        # Coverage quality gate
        THRESHOLD=80
        if (( $(echo "$COVERAGE >= $THRESHOLD" | bc -l) )); then
            echo -e "${GREEN}✅ Coverage threshold met (>=${THRESHOLD}%)${NC}"
        else
            echo -e "${RED}❌ Coverage below threshold (${THRESHOLD}%)${NC}"
            echo -e "${YELLOW}Consider adding more tests to improve coverage${NC}"
        fi
    fi
    
    echo -e "${BLUE}📁 Coverage reports generated in: coverage/${NC}"
    echo -e "${BLUE}   - HTML Report: coverage/tarpaulin-report.html${NC}"
    echo -e "${BLUE}   - XML Report: coverage/cobertura.xml${NC}"
    echo -e "${BLUE}   - JSON Report: coverage/tarpaulin-report.json${NC}"
    
    # Open HTML report if on macOS or Linux with display
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo -e "${BLUE}Opening HTML report...${NC}"
        open coverage/tarpaulin-report.html
    elif [[ -n "$DISPLAY" ]]; then
        echo -e "${BLUE}Opening HTML report...${NC}"
        xdg-open coverage/tarpaulin-report.html 2>/dev/null || true
    fi
    
else
    echo -e "${RED}❌ Coverage analysis failed${NC}"
    exit 1
fi

echo -e "${GREEN}🎉 Coverage analysis complete!${NC}"