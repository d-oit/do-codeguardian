#!/bin/bash

# Test Duplicate Detector Script
# Analyzes test files for duplicate code patterns, similar test setups, and redundant assertions

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="${PROJECT_ROOT}/target/test-analysis"
THRESHOLD=${THRESHOLD:-0.7}
FORMAT=${FORMAT:-text}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo -e "${BLUE}🔍 Test Duplicate Detector${NC}"
echo "=========================="
echo "Project: $PROJECT_ROOT"
echo "Output: $OUTPUT_DIR"
echo "Threshold: $THRESHOLD"
echo "Format: $FORMAT"
echo

# Find all test files
echo -e "${BLUE}📁 Finding test files...${NC}"
TEST_FILES=()

# Find files in tests/ directory
while IFS= read -r -d '' file; do
    TEST_FILES+=("$file")
done < <(find "$PROJECT_ROOT/tests" -name "*.rs" -type f -print0 2>/dev/null || true)

echo "Found ${#TEST_FILES[@]} test files:"
for file in "${TEST_FILES[@]}"; do
    echo "  - $(basename "$file")"
done
echo

# Analyze test files
echo -e "${BLUE}🔍 Analyzing test files...${NC}"

declare -A TEST_FUNCTIONS
declare -A SETUP_PATTERNS
declare -A ASSERTION_PATTERNS
declare -A FUNCTION_BODIES

for file in "${TEST_FILES[@]}"; do
    echo "Processing: $(basename "$file")"

    # Extract test functions using sed and awk
    # This is a simplified parser - in production you'd want a proper Rust AST parser

    # Find test function boundaries
    if [[ -f "$file" ]]; then
        content=$(cat "$file")
    else
        echo "Warning: File not found: $file"
        continue
    fi

    # Extract function names and bodies (simplified)
    while IFS= read -r line; do
        if [[ $line =~ fn[[:space:]]+([a-zA-Z_][a-zA-Z0-9_]*) ]]; then
            func_name="${BASH_REMATCH[1]}"

            # Check if this is a test function
            if grep -q "#\[test\]" <(echo "$content" | sed -n "/fn $func_name/,/^}/p"); then
                # Extract function body (simplified)
                func_body=$(echo "$content" | sed -n "/fn $func_name/,/^}/p" | tail -n +2 | head -n -1)

                # Store function info
                TEST_FUNCTIONS["$file:$func_name"]="$func_body"

                # Extract setup patterns (simplified)
                setup_patterns=$(echo "$func_body" | grep -E "(let|let mut|=)" | head -5 | tr '\n' ' ')
                if [[ -n "$setup_patterns" ]]; then
                    SETUP_PATTERNS["$file:$func_name"]="$setup_patterns"
                fi

                # Extract assertion patterns
                assertion_patterns=$(echo "$func_body" | grep -E "(assert|expect)" | tr '\n' ' ')
                if [[ -n "$assertion_patterns" ]]; then
                    ASSERTION_PATTERNS["$file:$func_name"]="$assertion_patterns"
                fi
            fi
        fi
    done < <(echo "$content" | grep "^fn ")
done

echo "Found ${#TEST_FUNCTIONS[@]} test functions"
echo

# Detect duplicates
echo -e "${BLUE}🔍 Detecting duplicates...${NC}"

DUPLICATES=()
IDENTICAL_COUNT=0
SIMILAR_COUNT=0

# Simple duplicate detection - just check for identical functions
keys=("${!TEST_FUNCTIONS[@]}")
echo "Comparing ${#keys[@]} test functions for identical duplicates..."

for ((i=0; i<${#keys[@]}; i++)); do
    for ((j=i+1; j<${#keys[@]}; j++)); do
        key1="${keys[$i]}"
        key2="${keys[$j]}"

        body1="${TEST_FUNCTIONS[$key1]}"
        body2="${TEST_FUNCTIONS[$key2]}"

        # Check for identical functions
        if [[ "$body1" == "$body2" ]]; then
            similarity=1.0
            duplicate_type="identical"
            ((IDENTICAL_COUNT++))

            # Find shared patterns
            shared_setup=""
            if [[ -n "${SETUP_PATTERNS[$key1]}" && -n "${SETUP_PATTERNS[$key2]}" ]]; then
                # Simple intersection check
                setup1_words=(${SETUP_PATTERNS[$key1]})
                setup2_words=(${SETUP_PATTERNS[$key2]})
                for word1 in "${setup1_words[@]}"; do
                    for word2 in "${setup2_words[@]}"; do
                        if [[ "$word1" == "$word2" ]]; then
                            shared_setup="$shared_setup$word1, "
                        fi
                    done
                done
                shared_setup="${shared_setup%, }"
            fi

            shared_assertions=""
            if [[ -n "${ASSERTION_PATTERNS[$key1]}" && -n "${ASSERTION_PATTERNS[$key2]}" ]]; then
                # Simple intersection check
                assert1_words=(${ASSERTION_PATTERNS[$key1]})
                assert2_words=(${ASSERTION_PATTERNS[$key2]})
                for word1 in "${assert1_words[@]}"; do
                    for word2 in "${assert2_words[@]}"; do
                        if [[ "$word1" == "$word2" ]]; then
                            shared_assertions="$shared_assertions$word1, "
                        fi
                    done
                done
                shared_assertions="${shared_assertions%, }"
            fi

            DUPLICATES+=("$key1|$key2|$similarity|$duplicate_type|$shared_setup|$shared_assertions")
        fi
    done
done

echo "Found $IDENTICAL_COUNT identical duplicates"
echo

# Generate report
echo -e "${BLUE}📊 Generating report...${NC}"
echo "Format: $FORMAT"

if [[ "$FORMAT" == "json" ]]; then
    echo "Creating JSON output..."
    # JSON output
    json_output="{\"total_tests\": ${#TEST_FUNCTIONS[@]}, \"duplicates_found\": ${#DUPLICATES[@]}, \"identical_count\": $IDENTICAL_COUNT, \"similar_count\": $SIMILAR_COUNT, \"duplicates\": ["

    first=true
    for dup in "${DUPLICATES[@]}"; do
        IFS='|' read -r key1 key2 similarity dup_type shared_setup shared_assertions <<< "$dup"

        if [[ "$first" == "true" ]]; then
            first=false
        else
            json_output+=","
        fi

        json_output+="{\"test1\": \"$key1\", \"test2\": \"$key2\", \"similarity\": $similarity, \"type\": \"$dup_type\", \"shared_setup\": \"$shared_setup\", \"shared_assertions\": \"$shared_assertions\"}"
    done

    json_output+="], \"recommendations\": ["

    recommendations=()
    if ((IDENTICAL_COUNT > 0)); then
        recommendations+=("\"Found $IDENTICAL_COUNT identical test functions. Consider extracting common functionality into helper functions.\"")
    fi

    if ((SIMILAR_COUNT > 0)); then
        recommendations+=("\"Found $SIMILAR_COUNT similar test functions. Review for potential consolidation opportunities.\"")
    fi

    if ((IDENTICAL_COUNT == 0 && SIMILAR_COUNT == 0)); then
        recommendations+=("\"No significant test duplicates found. Good job maintaining test quality!\"")
    fi

    # Join recommendations with commas
    first=true
    for rec in "${recommendations[@]}"; do
        if [[ "$first" == "true" ]]; then
            first=false
        else
            json_output+=","
        fi
        json_output+="$rec"
    done

    json_output+="]}"

    echo "$json_output" > "$OUTPUT_DIR/test_duplicates.json"
    echo "Report saved to: $OUTPUT_DIR/test_duplicates.json"

else
    # Text output
    {
        echo "Test Duplication Analysis Report"
        echo "================================"
        echo "Generated: $(date)"
        echo "Project: $PROJECT_ROOT"
        echo
        echo "Summary:"
        echo "--------"
        echo "Total test functions: ${#TEST_FUNCTIONS[@]}"
        echo "Duplicates found: ${#DUPLICATES[@]}"
        echo "Identical functions: $IDENTICAL_COUNT"
        echo "Similar functions: $SIMILAR_COUNT"
        echo

        if (( ${#DUPLICATES[@]} > 0 )); then
            echo "Duplicate Details:"
            echo "------------------"

            count=1
            for dup in "${DUPLICATES[@]}"; do
                IFS='|' read -r key1 key2 similarity dup_type shared_setup shared_assertions <<< "$dup"

                echo "$count. $key1 vs $key2"
                echo "   Type: $dup_type"
                echo "   Similarity: $(awk "BEGIN {printf \"%.1f\", $similarity * 100}")%"

                if [[ -n "$shared_setup" ]]; then
                    echo "   Shared setup: $shared_setup"
                fi

                if [[ -n "$shared_assertions" ]]; then
                    echo "   Shared assertions: $shared_assertions"
                fi

                echo
                ((count++))
            done
        fi

        echo "Recommendations:"
        echo "---------------"

        if ((IDENTICAL_COUNT > 0)); then
            echo -e "${RED}• Found $IDENTICAL_COUNT identical test functions${NC}"
            echo "  → Consider extracting common functionality into helper functions"
            echo "  → Use parameterized tests where appropriate"
        fi

        if ((SIMILAR_COUNT > 0)); then
            echo -e "${YELLOW}• Found $SIMILAR_COUNT similar test functions${NC}"
            echo "  → Review for potential consolidation opportunities"
            echo "  → Extract common setup/teardown code"
        fi

        if ((IDENTICAL_COUNT == 0 && SIMILAR_COUNT == 0)); then
            echo -e "${GREEN}• No significant test duplicates found${NC}"
            echo "  → Good job maintaining test quality!"
        fi

        echo
        echo "💡 General Recommendations:"
        echo "• Use test fixtures for common setup code"
        echo "• Create helper functions for repeated assertions"
        echo "• Consider using table-driven tests for similar test cases"
        echo "• Regular refactoring of test code improves maintainability"

    } > "$OUTPUT_DIR/test_duplicates.txt"

    echo "Report saved to: $OUTPUT_DIR/test_duplicates.txt"
fi

echo
echo -e "${GREEN}✅ Analysis complete!${NC}"

# Display summary
if [[ "$FORMAT" == "text" ]]; then
    echo
    echo "Summary:"
    echo "- Total test functions: ${#TEST_FUNCTIONS[@]}"
    echo "- Duplicates found: ${#DUPLICATES[@]}"
    echo "- Identical: $IDENTICAL_COUNT"
    echo "- Similar: $SIMILAR_COUNT"
fi

echo "Script completed successfully"
