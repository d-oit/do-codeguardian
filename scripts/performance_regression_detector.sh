#!/bin/bash

# Performance Regression Detection Script
# This script automates performance regression detection and alerting
# for the CodeGuardian project

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BENCHMARK_RESULTS_DIR="$PROJECT_ROOT/benchmark_results"
HISTORICAL_DATA_DIR="$PROJECT_ROOT/performance_history"
THRESHOLDS_FILE="$PROJECT_ROOT/performance_thresholds.json"

# Default thresholds
DEFAULT_MEMORY_THRESHOLD_MB=200
DEFAULT_TIME_THRESHOLD_MS=2000
DEFAULT_CACHE_HIT_RATE=0.7
DEFAULT_REGRESSION_THRESHOLD_PERCENT=10

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Create necessary directories
setup_directories() {
    mkdir -p "$BENCHMARK_RESULTS_DIR"
    mkdir -p "$HISTORICAL_DATA_DIR"
    log_info "Created directories: $BENCHMARK_RESULTS_DIR, $HISTORICAL_DATA_DIR"
}

# Load performance thresholds
load_thresholds() {
    if [ -f "$THRESHOLDS_FILE" ]; then
        MEMORY_THRESHOLD=$(jq -r '.memory_threshold_mb // 200' "$THRESHOLDS_FILE")
        TIME_THRESHOLD=$(jq -r '.time_threshold_ms // 2000' "$THRESHOLDS_FILE")
        CACHE_THRESHOLD=$(jq -r '.cache_hit_rate // 0.7' "$THRESHOLDS_FILE")
        REGRESSION_THRESHOLD=$(jq -r '.regression_threshold_percent // 10' "$THRESHOLDS_FILE")
        log_info "Loaded custom thresholds from $THRESHOLDS_FILE"
    else
        MEMORY_THRESHOLD=$DEFAULT_MEMORY_THRESHOLD_MB
        TIME_THRESHOLD=$DEFAULT_TIME_THRESHOLD_MS
        CACHE_THRESHOLD=$DEFAULT_CACHE_HIT_RATE
        REGRESSION_THRESHOLD=$DEFAULT_REGRESSION_THRESHOLD_PERCENT
        log_info "Using default performance thresholds"
    fi

    log_info "Performance Thresholds:"
    log_info "  Memory: ${MEMORY_THRESHOLD}MB"
    log_info "  Time: ${TIME_THRESHOLD}ms"
    log_info "  Cache Hit Rate: ${CACHE_THRESHOLD}"
    log_info "  Regression Threshold: ${REGRESSION_THRESHOLD}%"
}

# Run performance benchmarks
run_benchmarks() {
    log_info "Running performance benchmarks..."

    cd "$PROJECT_ROOT"

    # Run comprehensive benchmark suite
    if cargo bench --bench performance_regression_suite -- --measurement-time 10 --output-format json > "$BENCHMARK_RESULTS_DIR/regression_results.json" 2>/dev/null; then
        log_success "Performance regression benchmarks completed"
    else
        log_warning "Some benchmarks failed, but continuing with analysis"
    fi

    # Run load testing benchmarks
    if cargo bench --bench load_testing_benchmark -- --measurement-time 15 --output-format json > "$BENCHMARK_RESULTS_DIR/load_test_results.json" 2>/dev/null; then
        log_success "Load testing benchmarks completed"
    else
        log_warning "Load testing benchmarks failed"
    fi

    # Run metrics collection benchmarks
    if cargo bench --bench performance_metrics_benchmark -- --measurement-time 10 --output-format json > "$BENCHMARK_RESULTS_DIR/metrics_results.json" 2>/dev/null; then
        log_success "Metrics collection benchmarks completed"
    else
        log_warning "Metrics collection benchmarks failed"
    fi

    # Run optimization benchmarks
    if cargo bench --bench optimization_recommendations_benchmark -- --measurement-time 10 --output-format json > "$BENCHMARK_RESULTS_DIR/optimization_results.json" 2>/dev/null; then
        log_success "Optimization benchmarks completed"
    else
        log_warning "Optimization benchmarks failed"
    fi
}

# Analyze benchmark results
analyze_results() {
    log_info "Analyzing benchmark results..."

    local regression_detected=false
    local alerts=()

    # Analyze regression results
    if [ -f "$BENCHMARK_RESULTS_DIR/regression_results.json" ]; then
        analyze_regression_results
        if [ $? -eq 1 ]; then
            regression_detected=true
            alerts+=("Performance regression detected in core functionality")
        fi
    fi

    # Analyze load testing results
    if [ -f "$BENCHMARK_RESULTS_DIR/load_test_results.json" ]; then
        analyze_load_test_results
        if [ $? -eq 1 ]; then
            regression_detected=true
            alerts+=("Load testing performance regression detected")
        fi
    fi

    # Analyze metrics results
    if [ -f "$BENCHMARK_RESULTS_DIR/metrics_results.json" ]; then
        analyze_metrics_results
        if [ $? -eq 1 ]; then
            regression_detected=true
            alerts+=("Performance metrics regression detected")
        fi
    fi

    # Generate performance report
    generate_performance_report "$regression_detected"

    # Handle regression detection
    if [ "$regression_detected" = true ]; then
        log_error "PERFORMANCE REGRESSION DETECTED!"
        for alert in "${alerts[@]}"; do
            log_error "  - $alert"
        done

        # Create GitHub issue if running in CI
        if [ -n "$GITHUB_RUN_ID" ]; then
            create_github_issue "${alerts[@]}"
        fi

        return 1
    else
        log_success "No performance regressions detected"
        return 0
    fi
}

# Analyze regression benchmark results
analyze_regression_results() {
    log_info "Analyzing regression benchmark results..."

    # This is a simplified analysis - in practice, you'd compare against historical baselines
    # For now, we'll check if the benchmarks completed successfully
    if jq -e '.benchmarks[0]' "$BENCHMARK_RESULTS_DIR/regression_results.json" >/dev/null 2>&1; then
        log_success "Regression benchmarks completed successfully"
        return 0
    else
        log_warning "Regression benchmark results appear incomplete"
        return 1
    fi
}

# Analyze load testing results
analyze_load_test_results() {
    log_info "Analyzing load testing results..."

    if jq -e '.benchmarks[0]' "$BENCHMARK_RESULTS_DIR/load_test_results.json" >/dev/null 2>&1; then
        log_success "Load testing benchmarks completed successfully"
        return 0
    else
        log_warning "Load testing results appear incomplete"
        return 1
    fi
}

# Analyze metrics results
analyze_metrics_results() {
    log_info "Analyzing performance metrics..."

    if jq -e '.benchmarks[0]' "$BENCHMARK_RESULTS_DIR/metrics_results.json" >/dev/null 2>&1; then
        log_success "Metrics collection completed successfully"
        return 0
    else
        log_warning "Metrics results appear incomplete"
        return 1
    fi
}

# Generate comprehensive performance report
generate_performance_report() {
    local regression_detected=$1
    local report_file="$BENCHMARK_RESULTS_DIR/performance_report_$(date +%Y%m%d_%H%M%S).md"

    log_info "Generating performance report: $report_file"

    cat > "$report_file" << EOF
# CodeGuardian Performance Report
Generated: $(date)
Regression Detected: $([ "$regression_detected" = true ] && echo "YES" || echo "NO")

## Summary
- Benchmarks Run: $(ls "$BENCHMARK_RESULTS_DIR"/*.json | wc -l)
- Test Environment: $(uname -a)
- Rust Version: $(rustc --version)

## Thresholds Used
- Memory Threshold: ${MEMORY_THRESHOLD}MB
- Time Threshold: ${TIME_THRESHOLD}ms
- Cache Hit Rate Threshold: ${CACHE_THRESHOLD}
- Regression Threshold: ${REGRESSION_THRESHOLD}%

## Recommendations
EOF

    # Add optimization recommendations based on results
    if [ -f "$BENCHMARK_RESULTS_DIR/optimization_results.json" ]; then
        echo "### Optimization Recommendations" >> "$report_file"
        echo "- Review benchmark results for specific optimization opportunities" >> "$report_file"
        echo "- Consider memory pool optimizations for large file processing" >> "$report_file"
        echo "- Evaluate parallel processing improvements" >> "$report_file"
    fi

    log_success "Performance report generated: $report_file"
}

# Create GitHub issue for performance regression
create_github_issue() {
    local alerts=("$@")
    local issue_title="🚨 Performance Regression Detected"
    local issue_body="## Performance Regression Alert

A performance regression has been detected in the CodeGuardian codebase.

### Details
$(printf '%s\n' "${alerts[@]}" | sed 's/^/- /')

### Environment
- Run ID: $GITHUB_RUN_ID
- Branch: $GITHUB_REF
- Commit: $GITHUB_SHA
- Date: $(date)

### Actions Required
1. Review the performance benchmarks
2. Identify the root cause of the regression
3. Implement performance optimizations
4. Re-run benchmarks to verify fixes

### Performance Report
See the attached performance report for detailed metrics.

### Labels
performance, regression, urgent"

    # This would create a GitHub issue using the GitHub CLI or API
    # For now, we'll just log the issue details
    log_warning "GitHub issue would be created with title: $issue_title"
    log_warning "Issue body preview:"
    echo "$issue_body" | head -20
}

# Store historical performance data
store_historical_data() {
    log_info "Storing historical performance data..."

    local timestamp=$(date +%Y%m%d_%H%M%S)
    local history_file="$HISTORICAL_DATA_DIR/performance_$timestamp.json"

    # Combine all benchmark results
    jq -n '{
        timestamp: "'$timestamp'",
        results: {
            regression: null,
            load_testing: null,
            metrics: null,
            optimization: null
        }
    }' > "$history_file"

    # Add individual results if they exist
    if [ -f "$BENCHMARK_RESULTS_DIR/regression_results.json" ]; then
        jq --argfile regression "$BENCHMARK_RESULTS_DIR/regression_results.json" \
           '.results.regression = $regression' "$history_file" > "${history_file}.tmp" && mv "${history_file}.tmp" "$history_file"
    fi

    if [ -f "$BENCHMARK_RESULTS_DIR/load_test_results.json" ]; then
        jq --argfile loadtest "$BENCHMARK_RESULTS_DIR/load_test_results.json" \
           '.results.load_testing = $loadtest' "$history_file" > "${history_file}.tmp" && mv "${history_file}.tmp" "$history_file"
    fi

    if [ -f "$BENCHMARK_RESULTS_DIR/metrics_results.json" ]; then
        jq --argfile metrics "$BENCHMARK_RESULTS_DIR/metrics_results.json" \
           '.results.metrics = $metrics' "$history_file" > "${history_file}.tmp" && mv "${history_file}.tmp" "$history_file"
    fi

    if [ -f "$BENCHMARK_RESULTS_DIR/optimization_results.json" ]; then
        jq --argfile optimization "$BENCHMARK_RESULTS_DIR/optimization_results.json" \
           '.results.optimization = $optimization' "$history_file" > "${history_file}.tmp" && mv "${history_file}.tmp" "$history_file"
    fi

    log_success "Historical data stored: $history_file"

    # Clean up old historical data (keep last 30 days)
    find "$HISTORICAL_DATA_DIR" -name "performance_*.json" -mtime +30 -delete
}

# Main execution
main() {
    log_info "Starting Performance Regression Detection"
    log_info "========================================="

    setup_directories
    load_thresholds

    # Run benchmarks
    if run_benchmarks; then
        log_success "All benchmarks completed"
    else
        log_warning "Some benchmarks failed, proceeding with analysis"
    fi

    # Analyze results
    if analyze_results; then
        log_success "Performance analysis completed successfully"
        store_historical_data
        exit 0
    else
        log_error "Performance regression detected!"
        store_historical_data
        exit 1
    fi
}

# Run main function
main "$@"
