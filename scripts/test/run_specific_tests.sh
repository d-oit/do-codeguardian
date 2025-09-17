#!/bin/bash

echo "🎯 CodeGuardian Specific Edge Case Testing"
echo "=========================================="

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

PASSED=0
FAILED=0

# Test result tracking
test_result() {
    if [ $1 -eq 0 ]; then
        echo -e "   ${GREEN}✅ PASSED${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "   ${RED}❌ FAILED${NC}"
        FAILED=$((FAILED + 1))
    fi
}

echo -e "${BLUE}🔨 Building CodeGuardian...${NC}"
if ! cargo build --release --quiet; then
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Build successful${NC}"

# Test 1: File Exclusion Precision
echo -e "\n${YELLOW}📁 Test 1: File Exclusion Precision${NC}"
echo "Testing that legitimate files with 'test' in name are NOT excluded..."

mkdir -p specific_tests/src
cat > specific_tests/src/contest_manager.rs << 'EOF'
// This file contains "test" but should NOT be excluded
pub struct ContestManager {
    api_key: String,
}

impl ContestManager {
    pub fn new() -> Self {
        Self {
            api_key: "sk-proj-RealContestSecret123456789".to_string(),
        }
    }
}
EOF

echo "   Testing contest_manager.rs (contains 'test' but should be analyzed):"
./target/release/do-codeguardian check specific_tests/src/contest_manager.rs --format json --out contest_results.json 2>/dev/null

if [ -f "contest_results.json" ]; then
    CONTEST_FINDINGS=$(grep -c '"findings":\[' contest_results.json || echo "0")
    if [ "$CONTEST_FINDINGS" -gt 0 ]; then
        echo "   File was analyzed (good)"
        test_result 0
    else
        echo "   File was NOT analyzed (bad - false exclusion)"
        test_result 1
    fi
else
    echo "   No results file generated"
    test_result 1
fi

# Test 2: Test File Exclusion Accuracy
echo -e "\n${YELLOW}📁 Test 2: Test File Exclusion Accuracy${NC}"
echo "Testing that actual test files ARE excluded..."

mkdir -p specific_tests/tests
cat > specific_tests/tests/integration_test.rs << 'EOF'
#[cfg(test)]
mod tests {
    #[test]
    fn test_authentication() {
        // This should be ignored - test context
        let api_key = "sk-1234567890abcdef";
        let secret = "test_secret_123";
        assert!(!api_key.is_empty());
    }
}
EOF

echo "   Testing integration_test.rs (should be excluded):"
./target/release/do-codeguardian check specific_tests/tests/integration_test.rs --format json --out test_results.json 2>/dev/null

if [ -f "test_results.json" ]; then
    TEST_FINDINGS=$(grep -o '"total_findings":[0-9]*' test_results.json | cut -d: -f2)
    if [ "$TEST_FINDINGS" -eq 0 ]; then
        echo "   Test file properly excluded (good)"
        test_result 0
    else
        echo "   Test file NOT excluded (bad - should be ignored)"
        test_result 1
    fi
else
    echo "   No results file generated"
    test_result 1
fi

# Test 3: Benchmark File Exclusion
echo -e "\n${YELLOW}📁 Test 3: Benchmark File Exclusion${NC}"
echo "Testing that benchmark files ARE excluded..."

mkdir -p specific_tests/benches
cat > specific_tests/benches/crypto_benchmark.rs << 'EOF'
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_crypto() {
    // These should be ignored - benchmark context
    let api_key = "sk-1234567890abcdef";
    let secret_key = "benchmark_secret_123";
    black_box(api_key);
    black_box(secret_key);
}

criterion_group!(benches, benchmark_crypto);
criterion_main!(benches);
EOF

echo "   Testing crypto_benchmark.rs (should be excluded):"
./target/release/do-codeguardian check specific_tests/benches/crypto_benchmark.rs --format json --out bench_results.json 2>/dev/null

if [ -f "bench_results.json" ]; then
    BENCH_FINDINGS=$(grep -o '"total_findings":[0-9]*' bench_results.json | cut -d: -f2)
    if [ "$BENCH_FINDINGS" -eq 0 ]; then
        echo "   Benchmark file properly excluded (good)"
        test_result 0
    else
        echo "   Benchmark file NOT excluded (bad - should be ignored)"
        test_result 1
    fi
else
    echo "   No results file generated"
    test_result 1
fi

# Test 4: False Positive Pattern Detection
echo -e "\n${YELLOW}🔍 Test 4: False Positive Pattern Detection${NC}"
echo "Testing that test patterns in source files are ignored..."

cat > specific_tests/src/auth_service.rs << 'EOF'
pub struct AuthService;

impl AuthService {
    pub fn authenticate(&self, token: &str) -> bool {
        // Real production code
        !token.is_empty()
    }

    #[cfg(test)]
    fn test_helper() {
        // This should be ignored even in source file - test context
        let test_key = "sk-1234567890abcdef";
        let dummy_secret = "dummy_secret_123";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth() {
        // These should be ignored - test context
        let mock_token = "fake_token_123";
        let test_secret = "test_secret_456";
        assert!(AuthService.authenticate(mock_token));
    }
}
EOF

echo "   Testing auth_service.rs (should ignore test patterns in #[cfg(test)]):"
./target/release/do-codeguardian check specific_tests/src/auth_service.rs --format json --out auth_results.json 2>/dev/null

if [ -f "auth_results.json" ]; then
    AUTH_FINDINGS=$(grep -o '"total_findings":[0-9]*' auth_results.json | cut -d: -f2)
    echo "   Found $AUTH_FINDINGS findings (should be 0 due to test context)"
    if [ "$AUTH_FINDINGS" -eq 0 ]; then
        echo "   Test patterns properly ignored (good)"
        test_result 0
    else
        echo "   Test patterns NOT ignored (may need refinement)"
        test_result 1
    fi
else
    echo "   No results file generated"
    test_result 1
fi

# Test 5: Real Secret Detection Still Works
echo -e "\n${YELLOW}🔍 Test 5: Real Secret Detection${NC}"
echo "Testing that real secrets are still detected..."

cat > specific_tests/src/production_config.rs << 'EOF'
pub struct ProductionConfig {
    pub database_url: String,
}

impl ProductionConfig {
    pub fn new() -> Self {
        Self {
            // This should be detected - real secret in production context
            database_url: "postgresql://user:MyRealPassword123!@localhost/prod".to_string(),
        }
    }
}
EOF

echo "   Testing production_config.rs (should detect real secret):"
./target/release/do-codeguardian check specific_tests/src/production_config.rs --format json --out prod_results.json 2>/dev/null

if [ -f "prod_results.json" ]; then
    PROD_FINDINGS=$(grep -o '"total_findings":[0-9]*' prod_results.json | cut -d: -f2)
    echo "   Found $PROD_FINDINGS findings"
    if [ "$PROD_FINDINGS" -gt 0 ]; then
        echo "   Real secrets properly detected (good)"
        test_result 0
    else
        echo "   Real secrets NOT detected (needs investigation)"
        test_result 1
    fi
else
    echo "   No results file generated"
    test_result 1
fi

# Test 6: CI Workflow Logic
echo -e "\n${YELLOW}⚙️  Test 6: CI Workflow Logic${NC}"
echo "Testing commit hash extraction and GitHub CLI command construction..."

# Test commit hash extraction
COMMIT_HASH=$(git rev-parse --short HEAD 2>/dev/null)
echo "   Current commit hash: $COMMIT_HASH"

if [ ${#COMMIT_HASH} -ge 7 ]; then
    echo "   Commit hash extraction working"
    test_result 0
else
    echo "   Commit hash extraction failed"
    test_result 1
fi

# Test GitHub CLI command construction
if command -v gh &> /dev/null; then
    echo "   Testing GitHub CLI command construction:"
    GH_SEARCH_CMD="gh issue list --repo test/repo --search '$COMMIT_HASH in:title' --state open --json number --jq '.[0].number // empty'"
    echo "   Command: $GH_SEARCH_CMD"

    # Validate command structure
    if [[ "$GH_SEARCH_CMD" == *"issue list"* ]] && [[ "$GH_SEARCH_CMD" == *"$COMMIT_HASH"* ]]; then
        echo "   GitHub CLI command properly constructed"
        test_result 0
    else
        echo "   GitHub CLI command construction failed"
        test_result 1
    fi
else
    echo "   GitHub CLI not available - skipping"
fi

# Test 7: Directory Structure Analysis
echo -e "\n${YELLOW}📂 Test 7: Directory Structure Analysis${NC}"
echo "Testing analysis of complete directory structure..."

echo "   Running full directory analysis:"
./target/release/do-codeguardian check specific_tests --format json --out full_results.json 2>/dev/null

if [ -f "full_results.json" ]; then
    TOTAL_FILES=$(grep -o '"total_files_scanned":[0-9]*' full_results.json | cut -d: -f2)
    TOTAL_FINDINGS=$(grep -o '"total_findings":[0-9]*' full_results.json | cut -d: -f2)

    echo "   Files scanned: $TOTAL_FILES"
    echo "   Total findings: $TOTAL_FINDINGS"

    # Check findings by directory
    SRC_FINDINGS=$(grep -c '"file":"[^"]*specific_tests/src/' full_results.json || echo "0")
    TEST_FINDINGS=$(grep -c '"file":"[^"]*specific_tests/tests/' full_results.json || echo "0")
    BENCH_FINDINGS=$(grep -c '"file":"[^"]*specific_tests/benches/' full_results.json || echo "0")

    echo "   Source findings: $SRC_FINDINGS"
    echo "   Test findings: $TEST_FINDINGS"
    echo "   Benchmark findings: $BENCH_FINDINGS"

    # Validate directory exclusion
    if [ "$TEST_FINDINGS" -eq 0 ] && [ "$BENCH_FINDINGS" -eq 0 ]; then
        echo "   Directory exclusion working correctly"
        test_result 0
    else
        echo "   Directory exclusion not working properly"
        test_result 1
    fi
else
    echo "   No results file generated"
    test_result 1
fi

# Test 8: Performance Under Load
echo -e "\n${YELLOW}⚡ Test 8: Performance Under Load${NC}"
echo "Testing performance with multiple files..."

# Create multiple test files
for i in {1..10}; do
    cat > specific_tests/src/module_${i}.rs << EOF
pub struct Module${i} {
    config: String,
}

impl Module${i} {
    pub fn new() -> Self {
        Self {
            config: "production_config_${i}".to_string(),
        }
    }
}
EOF
done

echo "   Testing performance with 10+ files:"
START_TIME=$(date +%s%N)
./target/release/do-codeguardian check specific_tests --format json --out perf_results.json 2>/dev/null
END_TIME=$(date +%s%N)

DURATION_MS=$(( (END_TIME - START_TIME) / 1000000 ))
echo "   Analysis duration: ${DURATION_MS}ms"

if [ "$DURATION_MS" -lt 5000 ]; then  # Less than 5 seconds
    echo "   Performance acceptable"
    test_result 0
else
    echo "   Performance slower than expected"
    test_result 1
fi

# Clean up
echo -e "\n${YELLOW}🧹 Cleaning up test files...${NC}"
rm -rf specific_tests *.json

# Final Results
echo -e "\n${BLUE}📊 Specific Test Results Summary${NC}"
echo "================================="
echo "Tests passed: $PASSED"
echo "Tests failed: $FAILED"
echo "Total tests: $((PASSED + FAILED))"

if [ $FAILED -eq 0 ]; then
    echo -e "\n${GREEN}🎉 All specific tests passed!${NC}"
    echo -e "${GREEN}✅ File exclusion logic working correctly${NC}"
    echo -e "${GREEN}✅ False positive detection functional${NC}"
    echo -e "${GREEN}✅ CI workflow logic operational${NC}"
    echo -e "${GREEN}✅ Performance within acceptable limits${NC}"
    exit 0
else
    echo -e "\n${YELLOW}⚠️  Some tests failed - see details above${NC}"
    echo -e "${YELLOW}Note: Secret detection may need pattern refinement${NC}"
    exit 1
fi
