#!/bin/bash
# Enhanced Build Optimization Script with AST Analysis Support
# Provides fast development builds, optimized releases, and performance monitoring

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_LOG="$PROJECT_ROOT/build-performance.log"
PERFORMANCE_BASELINE="$PROJECT_ROOT/performance-baseline.json"

# Performance tracking
start_time=""
end_time=""
build_duration=""

# Function to measure build time with high precision
measure_build_time() {
    local cmd="$1"
    local label="$2"
    local features="${3:-}"

    echo -e "${YELLOW}🚀 Building: ${label}${NC}"
    if [ -n "$features" ]; then
        echo -e "${CYAN}   Features: ${features}${NC}"
    fi

    start_time=$(date +%s.%3N)

    if eval "$cmd"; then
        end_time=$(date +%s.%3N)
        build_duration=$(echo "$end_time - $start_time" | bc)
        echo -e "${GREEN}✅ ${label} completed in ${build_duration}s${NC}"

        # Log performance data
        log_performance "$label" "$build_duration" "$features"
        return 0
    else
        end_time=$(date +%s.%3N)
        build_duration=$(echo "$end_time - $start_time" | bc)
        echo -e "${RED}❌ ${label} failed after ${build_duration}s${NC}"
        return 1
    fi
}

# Log performance metrics
log_performance() {
    local label="$1"
    local duration="$2"
    local features="$3"

    echo "$(date +%Y-%m-%dT%H:%M:%S),$label,$duration,$features" >> "$BUILD_LOG"
}

# Fast development build with minimal features
fast_dev_build() {
    echo -e "${BLUE}Fast Development Build (minimal features)${NC}"
    echo -e "${CYAN}Optimizations: incremental compilation, minimal features${NC}"

    measure_build_time "cargo build --profile dev-fast --features dev --quiet" \
                      "Fast Dev Build" "dev"
}

# Check build (syntax and type checking only)
check_build() {
    echo -e "${BLUE}Check Build (no codegen)${NC}"
    echo -e "${CYAN}Optimizations: no binary generation, fast feedback${NC}"

    measure_build_time "cargo check --profile dev-fast --features dev --quiet" \
                      "Check Build" "dev"
}

# AST-enhanced development build
ast_dev_build() {
    echo -e "${BLUE}AST-Enhanced Development Build${NC}"
    echo -e "${CYAN}Features: ML + AST analysis capabilities${NC}"

    measure_build_time "cargo build --profile dev-fast --features ml-enhanced --quiet" \
                      "AST Dev Build" "ml-enhanced"
}

# Optimized release build with AST
release_build() {
    echo -e "${BLUE}Optimized Release Build with AST${NC}"
    echo -e "${CYAN}Optimizations: LTO, codegen-units=1, strip, AST features${NC}"

    measure_build_time "cargo build --release --features ml-enhanced --quiet" \
                      "Release Build" "ml-enhanced"
}

# Performance benchmark build
bench_build() {
    echo -e "${BLUE}Benchmark Build${NC}"
    echo -e "${CYAN}Optimizations: Optimized for benchmarking${NC}"

    measure_build_time "cargo build --profile bench --features ml-enhanced --quiet" \
                      "Benchmark Build" "ml-enhanced"
}

# Clean build artifacts
clean_build() {
    echo -e "${YELLOW}🧹 Cleaning build artifacts...${NC}"
    cargo clean
    echo -e "${GREEN}✅ Clean completed${NC}"
}

# Setup build environment
setup_environment() {
    echo -e "${BLUE}🔧 Setting up build environment...${NC}"

    # Enable incremental compilation for development
    export CARGO_INCREMENTAL=1

    # Set backtrace for better error reporting
    export RUST_BACKTRACE=1

    # Clean stale lock files
    rm -f target/debug/.cargo-lock target/release/.cargo-lock

    # Update dependencies
    echo -e "${CYAN}Updating dependencies...${NC}"
    cargo update --quiet

    echo -e "${GREEN}✅ Environment setup complete${NC}"
}

# Test feature combinations
test_features() {
    echo -e "${BLUE}🧪 Testing feature combinations...${NC}"

    local features=(
        ""
        "--features git"
        "--features security"
        "--features logging"
        "--features ml"
        "--features ast"
        "--features ml-enhanced"
        "--features full"
    )

    for feature_flag in "${features[@]}"; do
        echo -e "${YELLOW}Testing: cargo check $feature_flag${NC}"
        if cargo check $feature_flag --quiet 2>/dev/null; then
            echo -e "${GREEN}✅ Features work: $feature_flag${NC}"
        else
            echo -e "${RED}❌ Features failed: $feature_flag${NC}"
            return 1
        fi
    done

    echo -e "${GREEN}✅ All feature combinations tested${NC}"
}

# Run performance analysis
performance_analysis() {
    echo -e "${BLUE}📊 Performance Analysis${NC}"

    if [ ! -f "$BUILD_LOG" ]; then
        echo -e "${YELLOW}No build log found. Run some builds first.${NC}"
        return 0
    fi

    echo -e "${CYAN}Recent build performance:${NC}"
    tail -10 "$BUILD_LOG" | while IFS=',' read -r timestamp label duration features; do
        printf "  %-20s %-8s %-15s %s\n" "$label" "${duration}s" "$features" "$timestamp"
    done

    # Calculate averages
    echo -e "\n${CYAN}Average build times:${NC}"
    awk -F',' '
        $2 != "" {
            sum[$2] += $3;
            count[$2]++;
        }
        END {
            for (build in sum) {
                avg = sum[build] / count[build];
                printf "  %-20s %.3fs (n=%d)\n", build, avg, count[build];
            }
        }
    ' "$BUILD_LOG"
}

# Compare with baseline
compare_baseline() {
    echo -e "${BLUE}📈 Baseline Comparison${NC}"

    if [ ! -f "$PERFORMANCE_BASELINE" ]; then
        echo -e "${YELLOW}No baseline found. Creating one...${NC}"
        create_baseline
        return 0
    fi

    echo -e "${CYAN}Comparing with baseline:${NC}"

    # Read baseline data
    local baseline_data=$(cat "$PERFORMANCE_BASELINE")

    # Get latest build data
    if [ -f "$BUILD_LOG" ]; then
        local latest_build=$(tail -1 "$BUILD_LOG")
        IFS=',' read -r timestamp label duration features <<< "$latest_build"

        local baseline_duration=$(echo "$baseline_data" | jq -r ".builds[\"$label\"] // 0")

        if [ "$baseline_duration" != "0" ]; then
            local ratio=$(echo "scale=3; $duration / $baseline_duration" | bc)
            local change_percent=$(echo "scale=1; ($ratio - 1) * 100" | bc)

            if (( $(echo "$ratio < 1" | bc -l) )); then
                echo -e "${GREEN}✅ Improved: ${label} is ${change_percent}% faster than baseline${NC}"
            elif (( $(echo "$ratio > 1.1" | bc -l) )); then
                echo -e "${RED}⚠️  Regressed: ${label} is ${change_percent}% slower than baseline${NC}"
            else
                echo -e "${YELLOW}➡️  Stable: ${label} performance within 10% of baseline${NC}"
            fi
        fi
    fi
}

# Create performance baseline
create_baseline() {
    echo -e "${BLUE}📝 Creating performance baseline...${NC}"

    if [ ! -f "$BUILD_LOG" ]; then
        echo -e "${YELLOW}No build data available. Run some builds first.${NC}"
        return 1
    fi

    # Calculate averages for baseline
    local baseline_data="{\"created\": \"$(date +%Y-%m-%dT%H:%M:%S)\", \"builds\": {"
    local first=true

    awk -F',' '
        $2 != "" {
            sum[$2] += $3;
            count[$2]++;
        }
        END {
            for (build in sum) {
                if (!first) printf ",";
                printf "\"%s\": %.3f", build, sum[build] / count[build];
                first=0;
            }
        }
    ' "$BUILD_LOG" | while read -r averages; do
        baseline_data+="$averages"
    done

    baseline_data+="}}"

    echo "$baseline_data" > "$PERFORMANCE_BASELINE"
    echo -e "${GREEN}✅ Baseline created at $PERFORMANCE_BASELINE${NC}"
}

# Show usage information
usage() {
    echo "Enhanced CodeGuardian Build Optimization Script"
    echo "==============================================="
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Development Commands:"
    echo "  fast         - Fast development build (minimal features)"
    echo "  check        - Quick syntax check (no codegen)"
    echo "  ast          - AST-enhanced development build"
    echo ""
    echo "Production Commands:"
    echo "  release      - Optimized release build with AST"
    echo "  bench        - Benchmark build"
    echo ""
    echo "Utility Commands:"
    echo "  clean        - Clean build artifacts"
    echo "  setup        - Setup build environment"
    echo "  test         - Test all feature combinations"
    echo "  analyze      - Show performance analysis"
    echo "  baseline     - Create/update performance baseline"
    echo "  compare      - Compare with baseline"
    echo "  all          - Run all builds sequentially"
    echo "  help         - Show this help"
    echo ""
    echo "Examples:"
    echo "  $0 fast      # Fast iterative development"
    echo "  $0 ast       # Development with AST features"
    echo "  $0 release   # Production build"
    echo "  $0 analyze   # View performance metrics"
}

# Main execution
main() {
    cd "$PROJECT_ROOT"

    case "${1:-help}" in
        "fast")
            setup_environment
            fast_dev_build
            ;;
        "check")
            setup_environment
            check_build
            ;;
        "ast")
            setup_environment
            ast_dev_build
            ;;
        "release")
            setup_environment
            release_build
            ;;
        "bench")
            setup_environment
            bench_build
            ;;
        "clean")
            clean_build
            ;;
        "setup")
            setup_environment
            ;;
        "test")
            setup_environment
            test_features
            ;;
        "analyze")
            performance_analysis
            ;;
        "baseline")
            create_baseline
            ;;
        "compare")
            compare_baseline
            ;;
        "all")
            setup_environment
            echo ""
            check_build
            echo ""
            fast_dev_build
            echo ""
            ast_dev_build
            echo ""
            release_build
            echo ""
            performance_analysis
            ;;
        "help"|*)
            usage
            ;;
    esac
}

# Run main function with all arguments
main "$@"
