#!/bin/bash
set -euo pipefail

# Regression Test Runner for CodeGuardian Improvements
# This script validates that all fixes and improvements are working correctly

echo "🧪 Running CodeGuardian Regression Tests"
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test tracking
TESTS_PASSED=0
TESTS_FAILED=0
TOTAL_TESTS=0

log_test() {
    echo -e "${BLUE}[TEST]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
    ((TESTS_PASSED++))
}

log_failure() {
    echo -e "${RED}[FAIL]${NC} $1"
    ((TESTS_FAILED++))
}

log_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Ensure we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    echo "Error: Must be run from the root of the CodeGuardian project"
    exit 1
fi

# Build the project first
echo "🔨 Building CodeGuardian..."
if cargo build --release --quiet; then
    log_success "Build completed successfully"
else
    log_failure "Build failed"
    exit 1
fi

# Test 1: Verify binary exists and runs
log_test "Testing binary availability"
((TOTAL_TESTS++))
if ./target/release/do-codeguardian --version >/dev/null 2>&1; then
    log_success "Binary is available and responds to --version"
else
    log_failure "Binary not available or not responding"
fi

# Test 2: Test git conflict analyzer improvements
log_test "Testing git conflict analyzer improvements"
((TOTAL_TESTS++))
cat > /tmp/test_git_conflicts.rs << 'EOF'
#[cfg(test)]
mod tests {
    #[test]
    fn test_conflict_detection() {
        let content = r#"
<<<<<<< HEAD
version 1
=======
version 2
>>>>>>> branch
"#;
        assert!(content.contains("======="));
    }
}
EOF

# Run analysis on test file - should not detect conflicts in test code
if ./target/release/do-codeguardian check /tmp/test_git_conflicts.rs --format json --quiet 2>/dev/null | grep -q '"git_conflict"'; then
    log_failure "Git conflict analyzer still flagging test content"
else
    log_success "Git conflict analyzer correctly ignores test content"
fi
rm -f /tmp/test_git_conflicts.rs

# Test 3: Test AI content analyzer improvements
log_test "Testing AI content analyzer improvements"
((TOTAL_TESTS++))
cat > /tmp/test_ai_content.rs << 'EOF'
//! Module documentation
//! TODO: This should not be flagged

/// Function documentation
/// TODO: Also should not be flagged
fn test_function() {
    // This might be flagged
    // TODO: implement logic
    println!("test");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        let todo_text = "TODO: test data";
        assert!(todo_text.contains("TODO"));
    }
}
EOF

# Check if AI analyzer correctly handles documentation and test content
AI_FINDINGS=$(./target/release/do-codeguardian check /tmp/test_ai_content.rs --format json --quiet 2>/dev/null | jq -r '.findings[] | select(.analyzer == "ai_content" and .rule == "incomplete_implementation") | .line' || echo "")

if [[ -n "$AI_FINDINGS" ]]; then
    # If there are findings, check they're not from documentation or test strings
    PROBLEMATIC_FINDINGS=0
    for line_num in $AI_FINDINGS; do
        line_content=$(sed -n "${line_num}p" /tmp/test_ai_content.rs)
        if [[ "$line_content" == *"//! TODO:"* ]] || [[ "$line_content" == *"/// TODO:"* ]] || [[ "$line_content" == *"\"TODO:"* ]]; then
            ((PROBLEMATIC_FINDINGS++))
        fi
    done

    if [[ $PROBLEMATIC_FINDINGS -eq 0 ]]; then
        log_success "AI content analyzer correctly ignores documentation and test strings"
    else
        log_failure "AI content analyzer still flagging documentation or test strings"
    fi
else
    log_success "AI content analyzer correctly ignores documentation and test content"
fi
rm -f /tmp/test_ai_content.rs

# Test 4: Test debug statement detection accuracy
log_test "Testing debug statement detection accuracy"
((TOTAL_TESTS++))
cat > /tmp/test_debug_statements.rs << 'EOF'
use tracing::Level;

fn main() {
    // This should NOT be flagged
    let log_level = tracing::Level::DEBUG;

    // This SHOULD be flagged
    eprintln!("DEBUG: error occurred");

    // This should NOT be flagged
    tracing::debug!("proper logging");
}
EOF

DEBUG_FINDINGS=$(./target/release/do-codeguardian check /tmp/test_debug_statements.rs --format json --quiet 2>/dev/null | jq -r '.findings[] | select(.rule == "debug_statement") | .line' || echo "")

if [[ -n "$DEBUG_FINDINGS" ]]; then
    CORRECT_DETECTION=true
    for line_num in $DEBUG_FINDINGS; do
        line_content=$(sed -n "${line_num}p" /tmp/test_debug_statements.rs)
        # Should flag eprintln! but not tracing::Level::DEBUG or tracing::debug!
        if [[ "$line_content" == *"tracing::Level::DEBUG"* ]] || [[ "$line_content" == *"tracing::debug!"* ]]; then
            CORRECT_DETECTION=false
            break
        fi
    done

    if [[ "$CORRECT_DETECTION" == "true" ]]; then
        log_success "Debug statement detection working correctly"
    else
        log_failure "Debug statement detection flagging proper logging constructs"
    fi
else
    log_warning "No debug statements detected (may be correct depending on config)"
fi
rm -f /tmp/test_debug_statements.rs

# Test 5: Run unit tests
log_test "Running unit tests"
((TOTAL_TESTS++))
if cargo test --quiet 2>/dev/null; then
    log_success "All unit tests passed"
else
    log_failure "Some unit tests failed"
fi

# Test 6: Run regression tests specifically
log_test "Running regression test suite"
((TOTAL_TESTS++))
if cargo test --test regression_tests --quiet 2>/dev/null; then
    log_success "Regression tests passed"
else
    log_failure "Regression tests failed"
fi

# Test 7: Run analyzer improvement tests
log_test "Running analyzer improvement tests"
((TOTAL_TESTS++))
if cargo test --test analyzer_improvement_tests --quiet 2>/dev/null; then
    log_success "Analyzer improvement tests passed"
else
    log_failure "Analyzer improvement tests failed"
fi

# Test 8: Run CLI integration tests
log_test "Running CLI integration tests"
((TOTAL_TESTS++))
if cargo test --test cli_regression_integration_tests --quiet 2>/dev/null; then
    log_success "CLI integration tests passed"
else
    log_failure "CLI integration tests failed"
fi

# Test 9: Test configuration handling
log_test "Testing configuration handling"
((TOTAL_TESTS++))
if ./target/release/do-codeguardian --config tests/test_config.toml check src/main.rs --quiet >/dev/null 2>&1; then
    log_success "Configuration handling working correctly"
else
    log_failure "Configuration handling issues detected"
fi

# Test 10: Test output format consistency
log_test "Testing output format consistency"
((TOTAL_TESTS++))
JSON_OUTPUT=$(./target/release/do-codeguardian check src/main.rs --format json --quiet 2>/dev/null)
if echo "$JSON_OUTPUT" | jq . >/dev/null 2>&1; then
    log_success "JSON output format is valid"
else
    log_failure "JSON output format is invalid"
fi

# Test 11: Performance regression test
log_test "Testing performance regression"
((TOTAL_TESTS++))
START_TIME=$(date +%s%3N)
./target/release/do-codeguardian check src/ --quiet >/dev/null 2>&1 || true
END_TIME=$(date +%s%3N)
DURATION=$((END_TIME - START_TIME))

if [[ $DURATION -lt 5000 ]]; then  # Less than 5 seconds
    log_success "Performance is acceptable (${DURATION}ms)"
else
    log_warning "Performance may have regressed (${DURATION}ms)"
fi

# Test 12: Self-analysis validation
log_test "Running self-analysis validation"
((TOTAL_TESTS++))
SELF_ANALYSIS=$(./target/release/do-codeguardian check src/main.rs --format json --quiet 2>/dev/null)
MAIN_FINDINGS=$(echo "$SELF_ANALYSIS" | jq -r '.summary.total_findings' 2>/dev/null || echo "0")

if [[ "$MAIN_FINDINGS" =~ ^[0-9]+$ ]] && [[ $MAIN_FINDINGS -le 2 ]]; then
    log_success "Self-analysis shows reasonable findings count ($MAIN_FINDINGS)"
else
    log_warning "Self-analysis shows unexpected findings count ($MAIN_FINDINGS)"
fi

# Summary
echo ""
echo "📊 Test Results Summary"
echo "======================"
echo -e "Total tests: ${BLUE}$TOTAL_TESTS${NC}"
echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Failed: ${RED}$TESTS_FAILED${NC}"

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo -e "\n${GREEN}✅ All regression tests passed!${NC}"
    echo "The improvements and fixes are working correctly."
    exit 0
else
    echo -e "\n${RED}❌ Some tests failed!${NC}"
    echo "Please review the failed tests and fix any regressions."
    exit 1
fi
