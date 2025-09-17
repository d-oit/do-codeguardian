#!/bin/bash

echo "🧪 CodeGuardian Improvements - Comprehensive Edge Case Testing"
echo "=============================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

# Function to run a test and track results
run_test() {
    local test_name="$1"
    local test_command="$2"
    local description="$3"

    echo -e "\n${BLUE}🔍 Running: $test_name${NC}"
    echo "   Description: $description"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    if eval "$test_command" > /tmp/test_output_$$.log 2>&1; then
        echo -e "   ${GREEN}✅ PASSED${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "   ${RED}❌ FAILED${NC}"
        echo "   Error output:"
        sed 's/^/     /' /tmp/test_output_$$.log
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi

    rm -f /tmp/test_output_$$.log
}

# Function to run a test that might be skipped
run_optional_test() {
    local test_name="$1"
    local test_command="$2"
    local description="$3"
    local skip_reason="$4"

    echo -e "\n${BLUE}🔍 Running: $test_name${NC}"
    echo "   Description: $description"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    # Check if test should be skipped
    if [ -n "$skip_reason" ]; then
        echo -e "   ${YELLOW}⏭️  SKIPPED: $skip_reason${NC}"
        SKIPPED_TESTS=$((SKIPPED_TESTS + 1))
        return
    fi

    if eval "$test_command" > /tmp/test_output_$$.log 2>&1; then
        echo -e "   ${GREEN}✅ PASSED${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "   ${RED}❌ FAILED${NC}"
        echo "   Error output:"
        sed 's/^/     /' /tmp/test_output_$$.log
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi

    rm -f /tmp/test_output_$$.log
}

# Check prerequisites
echo -e "\n${YELLOW}📋 Checking Prerequisites${NC}"
echo "================================"

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo not found. Please install Rust.${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Cargo found${NC}"

# Check if git is available
if ! command -v git &> /dev/null; then
    echo -e "${RED}❌ Git not found. Please install Git.${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Git found${NC}"

# Check if gh CLI is available (optional)
if command -v gh &> /dev/null; then
    echo -e "${GREEN}✅ GitHub CLI found${NC}"
    GH_AVAILABLE=true
else
    echo -e "${YELLOW}⚠️  GitHub CLI not found (some tests will be skipped)${NC}"
    GH_AVAILABLE=false
fi

# Check if jq is available (optional)
if command -v jq &> /dev/null; then
    echo -e "${GREEN}✅ jq found${NC}"
    JQ_AVAILABLE=true
else
    echo -e "${YELLOW}⚠️  jq not found (some tests will be skipped)${NC}"
    JQ_AVAILABLE=false
fi

# Build the project first
echo -e "\n${YELLOW}🔨 Building CodeGuardian${NC}"
echo "========================="
if cargo build --release --quiet; then
    echo -e "${GREEN}✅ Build successful${NC}"
else
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

# Run core unit tests
echo -e "\n${YELLOW}🧪 Core Unit Tests${NC}"
echo "=================="

run_test "Security Analyzer Tests" \
    "cargo test codeguardian_improvements_tests::edge_case_tests::test_file_exclusion_edge_cases" \
    "Test file exclusion patterns with edge cases"

run_test "Secret Pattern Tests" \
    "cargo test codeguardian_improvements_tests::edge_case_tests::test_secret_pattern_edge_cases" \
    "Test secret detection with false positive edge cases"

run_test "Mixed Content Tests" \
    "cargo test codeguardian_improvements_tests::edge_case_tests::test_mixed_content_scenarios" \
    "Test files with both test patterns and real secrets"

run_test "File Path Variations" \
    "cargo test codeguardian_improvements_tests::edge_case_tests::test_file_path_variations" \
    "Test different file path formats and naming conventions"

run_test "Tricky Content Patterns" \
    "cargo test codeguardian_improvements_tests::edge_case_tests::test_tricky_content_patterns" \
    "Test multiline and complex content scenarios"

run_test "Analyzer File Exclusions" \
    "cargo test codeguardian_improvements_tests::edge_case_tests::test_analyzer_file_exclusions" \
    "Test that analyzer files themselves are excluded"

run_test "Integration with Temp Files" \
    "cargo test codeguardian_improvements_tests::edge_case_tests::test_integration_with_temp_files" \
    "Test with actual temporary file system"

# Run GitHub integration tests
echo -e "\n${YELLOW}🐙 GitHub Integration Tests${NC}"
echo "============================"

run_test "Commit Hash Edge Cases" \
    "cargo test github_deduplication_tests::github_deduplication_edge_cases::test_commit_hash_edge_cases" \
    "Test different commit hash formats and lengths"

run_test "Issue Title Generation" \
    "cargo test github_deduplication_tests::github_deduplication_edge_cases::test_issue_title_generation_edge_cases" \
    "Test issue title generation in various scenarios"

run_test "Duplicate Detection Logic" \
    "cargo test github_deduplication_tests::github_deduplication_edge_cases::test_duplicate_detection_edge_cases" \
    "Test duplicate issue detection with similar hashes"

run_optional_test "GitHub CLI Commands" \
    "cargo test github_deduplication_tests::github_deduplication_edge_cases::test_github_cli_edge_cases" \
    "Test GitHub CLI command construction" \
    "$( [ "$GH_AVAILABLE" = false ] && echo "GitHub CLI not available" )"

run_test "CI Workflow Scenarios" \
    "cargo test github_deduplication_tests::github_deduplication_edge_cases::test_ci_workflow_edge_cases" \
    "Test different CI workflow event scenarios"

run_test "Concurrent Issue Scenarios" \
    "cargo test github_deduplication_tests::github_deduplication_edge_cases::test_concurrent_issue_scenarios" \
    "Test concurrent CI runs and race conditions"

run_test "Issue Body Size Limits" \
    "cargo test github_deduplication_tests::github_deduplication_edge_cases::test_issue_body_size_edge_cases" \
    "Test handling of large issue bodies"

run_test "Network Failure Handling" \
    "cargo test github_deduplication_tests::github_deduplication_edge_cases::test_network_failure_edge_cases" \
    "Test retry logic for network failures"

# Run CI workflow tests
echo -e "\n${YELLOW}⚙️  CI Workflow Tests${NC}"
echo "===================="

run_test "CI Deduplication Scripts" \
    "cargo test ci_workflow_edge_cases::ci_workflow_edge_cases::test_ci_deduplication_script_edge_cases" \
    "Test CI deduplication script logic"

run_optional_test "GitHub CLI Commands in CI" \
    "cargo test ci_workflow_edge_cases::ci_workflow_edge_cases::test_github_cli_command_edge_cases" \
    "Test GitHub CLI command variations" \
    "$( [ "$GH_AVAILABLE" = false ] && echo "GitHub CLI not available" )"

run_test "CI Environment Variables" \
    "cargo test ci_workflow_edge_cases::ci_workflow_edge_cases::test_ci_environment_edge_cases" \
    "Test CI environment variable handling"

run_test "Concurrent CI Runs" \
    "cargo test ci_workflow_edge_cases::ci_workflow_edge_cases::test_concurrent_ci_runs_edge_cases" \
    "Test concurrent CI run scenarios"

run_test "Workflow File Parsing" \
    "cargo test ci_workflow_edge_cases::ci_workflow_edge_cases::test_workflow_file_edge_cases" \
    "Test YAML workflow file parsing"

run_test "Rate Limiting" \
    "cargo test ci_workflow_edge_cases::ci_workflow_edge_cases::test_rate_limiting_edge_cases" \
    "Test rate limiting logic"

run_test "Workflow Step Dependencies" \
    "cargo test ci_workflow_edge_cases::ci_workflow_edge_cases::test_workflow_step_dependencies_edge_cases" \
    "Test workflow step dependency logic"

run_test "Artifact Handling" \
    "cargo test ci_workflow_edge_cases::ci_workflow_edge_cases::test_artifact_handling_edge_cases" \
    "Test CI artifact upload and compression"

# Run integration tests with real CodeGuardian binary
echo -e "\n${YELLOW}🔗 Integration Tests${NC}"
echo "==================="

# Create test scenarios
echo "Creating test scenarios..."
mkdir -p tmp_edge_test/{tests,benches,src,examples}

# Test scenario 1: Complex mixed content
cat > tmp_edge_test/src/complex_mixed.rs << 'EOF'
//! Complex file with mixed legitimate and test content

use std::env;

/// Production configuration
pub struct Config {
    pub api_key: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            // This should be detected - real secret pattern
            api_key: "sk-proj-RealProductionKeyThatShouldBeDetected123456".to_string(),
            database_url: env::var("DATABASE_URL").unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        // These should be ignored - test patterns
        let test_key = "sk-1234567890abcdef";
        let dummy_url = "postgres://test:test@localhost/test";

        let config = Config {
            api_key: test_key.to_string(),
            database_url: dummy_url.to_string(),
        };

        assert!(!config.api_key.is_empty());
    }
}

// This should also be detected - another real secret
const BACKUP_KEY: &str = "ghp-RealGitHubTokenThatShouldBeDetected123456789";
EOF

# Test scenario 2: Benchmark file with test secrets
cat > tmp_edge_test/benches/complex_benchmark.rs << 'EOF'
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_auth_performance() {
    // These should be ignored - benchmark test patterns
    let test_api_key = "sk-1234567890abcdef";
    let dummy_secret = "dummy_secret_for_benchmarking";
    let fake_token = "fake_token_123456789";

    black_box(test_api_key);
    black_box(dummy_secret);
    black_box(fake_token);
}

criterion_group!(benches, benchmark_auth_performance);
criterion_main!(benches);
EOF

# Test scenario 3: Test file with mock data
cat > tmp_edge_test/tests/complex_integration_test.rs << 'EOF'
#[cfg(test)]
mod integration_tests {
    #[test]
    fn test_authentication_flow() {
        // These should all be ignored - test context
        let mock_password = "fake_password_123";
        let test_secret = "test_secret_value";
        let example_key = "example_api_key_789";
        let placeholder_token = "placeholder_token_000";

        assert!(authenticate_with_mock_data(mock_password));
    }

    fn authenticate_with_mock_data(password: &str) -> bool {
        // Mock implementation
        !password.is_empty()
    }
}
EOF

# Test scenario 4: Example file
cat > tmp_edge_test/examples/usage_example.rs << 'EOF'
//! Example usage of the authentication system

fn main() {
    // These should be ignored - example context
    let example_api_key = "your-api-key-here";
    let placeholder_secret = "your-secret-here";

    println!("Example usage with key: {}", example_api_key);
}
EOF

run_test "Complex Mixed Content Analysis" \
    "./target/release/do-codeguardian check tmp_edge_test --format json --out tmp_edge_results.json" \
    "Test real CodeGuardian binary with complex mixed content"

# Analyze results
if [ -f "tmp_edge_results.json" ]; then
    echo -e "\n${BLUE}📊 Integration Test Results Analysis${NC}"
    echo "====================================="

    if command -v jq &> /dev/null; then
        TOTAL_FINDINGS=$(jq '.summary.total_findings' tmp_edge_results.json 2>/dev/null || echo "0")
        BENCHMARK_FINDINGS=$(jq '[.findings[] | select(.file | contains("benches/"))] | length' tmp_edge_results.json 2>/dev/null || echo "0")
        TEST_FINDINGS=$(jq '[.findings[] | select(.file | contains("tests/"))] | length' tmp_edge_results.json 2>/dev/null || echo "0")
        EXAMPLE_FINDINGS=$(jq '[.findings[] | select(.file | contains("examples/"))] | length' tmp_edge_results.json 2>/dev/null || echo "0")
        SRC_FINDINGS=$(jq '[.findings[] | select(.file | contains("src/"))] | length' tmp_edge_results.json 2>/dev/null || echo "0")

        echo "📈 Analysis Results:"
        echo "   - Total findings: $TOTAL_FINDINGS"
        echo "   - Benchmark findings: $BENCHMARK_FINDINGS (should be 0)"
        echo "   - Test findings: $TEST_FINDINGS (should be 0)"
        echo "   - Example findings: $EXAMPLE_FINDINGS (should be 0)"
        echo "   - Source findings: $SRC_FINDINGS (should be > 0)"

        echo -e "\n📋 Detailed findings:"
        jq -r '.findings[] | "   - \(.analyzer):\(.rule) in \(.file):\(.line) - \(.message)"' tmp_edge_results.json 2>/dev/null || echo "   No findings or jq error"

        # Validate results
        if [ "$BENCHMARK_FINDINGS" -eq 0 ] && [ "$TEST_FINDINGS" -eq 0 ] && [ "$EXAMPLE_FINDINGS" -eq 0 ] && [ "$SRC_FINDINGS" -gt 0 ]; then
            echo -e "\n${GREEN}✅ Integration test PASSED: All improvements working correctly!${NC}"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        else
            echo -e "\n${RED}❌ Integration test FAILED: Issues detected${NC}"
            [ "$BENCHMARK_FINDINGS" -gt 0 ] && echo "   - Benchmark files not properly ignored"
            [ "$TEST_FINDINGS" -gt 0 ] && echo "   - Test files not properly ignored"
            [ "$EXAMPLE_FINDINGS" -gt 0 ] && echo "   - Example files not properly ignored"
            [ "$SRC_FINDINGS" -eq 0 ] && echo "   - Real secrets not detected in source files"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        fi
    else
        echo "jq not available - skipping detailed analysis"
        SKIPPED_TESTS=$((SKIPPED_TESTS + 1))
    fi
else
    echo -e "${RED}❌ Integration test results file not found${NC}"
    FAILED_TESTS=$((FAILED_TESTS + 1))
fi

# Clean up test files
echo -e "\n${YELLOW}🧹 Cleaning up test files${NC}"
rm -rf tmp_edge_test tmp_edge_results.json

# Final summary
echo -e "\n${YELLOW}📊 Final Test Summary${NC}"
echo "===================="
echo "Total tests run: $TOTAL_TESTS"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"
echo -e "Skipped: ${YELLOW}$SKIPPED_TESTS${NC}"

# Calculate success rate
if [ $TOTAL_TESTS -gt 0 ]; then
    SUCCESS_RATE=$(( (PASSED_TESTS * 100) / TOTAL_TESTS ))
    echo "Success rate: $SUCCESS_RATE%"

    if [ $FAILED_TESTS -eq 0 ]; then
        echo -e "\n${GREEN}🎉 All tests passed! CodeGuardian improvements are working correctly.${NC}"
        exit 0
    else
        echo -e "\n${RED}⚠️  Some tests failed. Please review the failures above.${NC}"
        exit 1
    fi
else
    echo -e "\n${RED}❌ No tests were run.${NC}"
    exit 1
fi
