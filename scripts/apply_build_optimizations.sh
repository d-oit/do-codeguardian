#!/bin/bash
# Apply CodeGuardian Build Optimizations
# This script applies all the build performance optimizations

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🚀 Applying CodeGuardian Build Optimizations${NC}"
echo "=============================================="

# Backup original files
echo -e "${YELLOW}Creating backups...${NC}"
cp build.rs build_original.rs
cp Cargo.toml Cargo_original.toml

# Apply optimized build.rs
echo -e "${YELLOW}Applying optimized build.rs...${NC}"
cp build_fixed.rs build.rs
echo -e "${GREEN}✅ build.rs optimized${NC}"

# Apply optimized Cargo.toml
echo -e "${YELLOW}Applying optimized Cargo.toml...${NC}"
cp Cargo_optimized.toml Cargo.toml
echo -e "${GREEN}✅ Cargo.toml optimized${NC}"

# Make scripts executable
echo -e "${YELLOW}Setting up build scripts...${NC}"
chmod +x scripts/build_optimization.sh
chmod +x scripts/performance_analysis.sh
echo -e "${GREEN}✅ Scripts configured${NC}"

# Test the optimizations
echo -e "${YELLOW}Testing optimizations...${NC}"
echo -e "${BLUE}Running fast check build...${NC}"
if cargo check --profile dev-fast --features dev --quiet; then
    echo -e "${GREEN}✅ Fast check build successful${NC}"
else
    echo -e "${RED}❌ Fast check build failed${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}🎉 Build optimizations applied successfully!${NC}"
echo ""
echo -e "${BLUE}Next steps:${NC}"
echo "1. Use './scripts/build_optimization.sh fast' for development"
echo "2. Use './scripts/build_optimization.sh check' for quick checks"
echo "3. Read BUILD_OPTIMIZATION_README.md for detailed guide"
echo "4. Update your CI to use .github/workflows/optimized-ci.yml"
echo ""
echo -e "${YELLOW}Expected improvements:${NC}"
echo "- Development builds: 60-70% faster"
echo "- Check builds: 70-75% faster"
echo "- Release builds: 25-35% faster"
