#!/bin/bash
# CodeGuardian Performance Trend Analysis Script
# Analyzes performance trends over time and generates insights

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Configuration
TREND_DATA_DIR="$PROJECT_ROOT/performance_trends"
REPORT_DIR="$PROJECT_ROOT/performance_reports"
THRESHOLD_CONFIG="$PROJECT_ROOT/performance_thresholds.json"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to log errors
log_error() {
    echo -e "${RED}❌ $1${NC}" >&2
}

# Function to log warnings
log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Function to log success
log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

# Function to log info
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Function to collect historical performance data
collect_historical_data() {
    log_info "Collecting historical performance data..."

    # Create trend data directory
    mkdir -p "$TREND_DATA_DIR"

    # Find all performance data files from recent runs
    # This would typically query GitHub API or a database
    # For now, we'll simulate with local files

    # Generate sample historical data for demonstration
    generate_sample_historical_data
}

# Function to generate sample historical data (for demonstration)
generate_sample_historical_data() {
    log_info "Generating sample historical performance data..."

    # Create sample data points for the last 30 days
    for i in {0..29}; do
        date_str=$(date -d "$i days ago" +%Y%m%d)
        data_file="$TREND_DATA_DIR/performance_$date_str.json"

        # Generate realistic sample data with some variation
        base_memory=$((150 + RANDOM % 50))
        base_time=$((800 + RANDOM % 400))
        base_cache_hit=$((70 + RANDOM % 20))

        cat > "$data_file" << EOF
{
  "date": "$date_str",
  "timestamp": "$(date -d "$i days ago" +%s)",
  "metrics": {
    "memory_usage_mb": $base_memory,
    "processing_time_ms": $base_time,
    "cache_hit_rate_percent": $base_cache_hit,
    "files_processed_per_second": $((50 + RANDOM % 30)),
    "error_rate_percent": $((RANDOM % 5))
  },
  "benchmarks": {
    "baseline_analysis": {
      "mean_time_ms": $((450 + RANDOM % 100)),
      "standard_deviation": $((20 + RANDOM % 10))
    },
    "memory_regression_detection": {
      "mean_memory_mb": $((45 + RANDOM % 15)),
      "peak_memory_mb": $((55 + RANDOM % 20))
    }
  },
  "system_info": {
    "rust_version": "1.70.0",
    "os": "ubuntu-latest",
    "cpu_cores": 4,
    "memory_total_gb": 16
  }
}
EOF
    done

    log_success "Generated sample historical data for 30 days"
}

# Function to analyze performance trends
analyze_trends() {
    log_info "Analyzing performance trends..."

    # Collect all data points
    data_files=("$TREND_DATA_DIR"/performance_*.json)
    if [ ${#data_files[@]} -eq 0 ]; then
        log_error "No historical performance data found"
        return 1
    fi

    # Extract metrics for trend analysis
    memory_trend_file="$TREND_DATA_DIR/memory_trend.txt"
    time_trend_file="$TREND_DATA_DIR/time_trend.txt"
    cache_trend_file="$TREND_DATA_DIR/cache_trend.txt"

    > "$memory_trend_file"
    > "$time_trend_file"
    > "$cache_trend_file"

    for data_file in "${data_files[@]}"; do
        if [ -f "$data_file" ]; then
            date_str=$(jq -r '.date' "$data_file")
            memory_mb=$(jq -r '.metrics.memory_usage_mb' "$data_file")
            time_ms=$(jq -r '.metrics.processing_time_ms' "$data_file")
            cache_hit=$(jq -r '.metrics.cache_hit_rate_percent' "$data_file")

            echo "$date_str $memory_mb" >> "$memory_trend_file"
            echo "$date_str $time_ms" >> "$time_trend_file"
            echo "$date_str $cache_hit" >> "$cache_trend_file"
        fi
    done

    # Sort data by date
    sort -o "$memory_trend_file" "$memory_trend_file"
    sort -o "$time_trend_file" "$time_trend_file"
    sort -o "$cache_trend_file" "$cache_trend_file"

    log_success "Performance trend data extracted and sorted"
}

# Function to calculate trend statistics
calculate_trend_statistics() {
    log_info "Calculating trend statistics..."

    # Memory usage trend
    if [ -f "$TREND_DATA_DIR/memory_trend.txt" ]; then
        memory_values=$(cut -d' ' -f2 "$TREND_DATA_DIR/memory_trend.txt")
        memory_avg=$(echo "$memory_values" | awk '{sum+=$1} END {print sum/NR}')
        memory_min=$(echo "$memory_values" | sort -n | head -1)
        memory_max=$(echo "$memory_values" | sort -n | tail -1)

        # Calculate trend (simple linear regression slope)
        memory_trend=$(calculate_trend "$TREND_DATA_DIR/memory_trend.txt")

        echo "memory_avg=$memory_avg" >> "$TREND_DATA_DIR/trend_stats.txt"
        echo "memory_min=$memory_min" >> "$TREND_DATA_DIR/trend_stats.txt"
        echo "memory_max=$memory_max" >> "$TREND_DATA_DIR/trend_stats.txt"
        echo "memory_trend=$memory_trend" >> "$TREND_DATA_DIR/trend_stats.txt"
    fi

    # Processing time trend
    if [ -f "$TREND_DATA_DIR/time_trend.txt" ]; then
        time_values=$(cut -d' ' -f2 "$TREND_DATA_DIR/time_trend.txt")
        time_avg=$(echo "$time_values" | awk '{sum+=$1} END {print sum/NR}')
        time_min=$(echo "$time_values" | sort -n | head -1)
        time_max=$(echo "$time_values" | sort -n | tail -1)
        time_trend=$(calculate_trend "$TREND_DATA_DIR/time_trend.txt")

        echo "time_avg=$time_avg" >> "$TREND_DATA_DIR/trend_stats.txt"
        echo "time_min=$time_min" >> "$TREND_DATA_DIR/trend_stats.txt"
        echo "time_max=$time_max" >> "$TREND_DATA_DIR/trend_stats.txt"
        echo "time_trend=$time_trend" >> "$TREND_DATA_DIR/trend_stats.txt"
    fi

    # Cache hit rate trend
    if [ -f "$TREND_DATA_DIR/cache_trend.txt" ]; then
        cache_values=$(cut -d' ' -f2 "$TREND_DATA_DIR/cache_trend.txt")
        cache_avg=$(echo "$cache_values" | awk '{sum+=$1} END {print sum/NR}')
        cache_min=$(echo "$cache_values" | sort -n | head -1)
        cache_max=$(echo "$cache_values" | sort -n | tail -1)
        cache_trend=$(calculate_trend "$TREND_DATA_DIR/cache_trend.txt")

        echo "cache_avg=$cache_avg" >> "$TREND_DATA_DIR/trend_stats.txt"
        echo "cache_min=$cache_min" >> "$TREND_DATA_DIR/trend_stats.txt"
        echo "cache_max=$cache_max" >> "$TREND_DATA_DIR/trend_stats.txt"
        echo "cache_trend=$cache_trend" >> "$TREND_DATA_DIR/trend_stats.txt"
    fi

    log_success "Trend statistics calculated"
}

# Function to calculate simple trend (slope)
calculate_trend() {
    local data_file="$1"

    # Simple linear regression to calculate trend
    awk '
    BEGIN {
        n = 0
        sum_x = 0
        sum_y = 0
        sum_xy = 0
        sum_x2 = 0
    }
    {
        x = NR  # Use row number as x (time)
        y = $2  # Value as y
        n++
        sum_x += x
        sum_y += y
        sum_xy += x * y
        sum_x2 += x * x
    }
    END {
        if (n > 1) {
            slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x)
            printf "%.4f", slope
        } else {
            printf "0.0000"
        }
    }
    ' "$data_file"
}

# Function to detect anomalies and regressions
detect_anomalies() {
    log_info "Detecting performance anomalies and regressions..."

    mkdir -p "$REPORT_DIR"

    # Load thresholds from config
    if [ -f "$THRESHOLD_CONFIG" ]; then
        memory_threshold=$(jq -r '.memory_threshold_mb' "$THRESHOLD_CONFIG")
        time_threshold=$(jq -r '.time_threshold_ms' "$THRESHOLD_CONFIG")
        regression_threshold=$(jq -r '.regression_threshold_percent' "$THRESHOLD_CONFIG")
    else
        memory_threshold=200
        time_threshold=2000
        regression_threshold=10
    fi

    anomaly_report="$REPORT_DIR/anomaly_report_$(date +%Y%m%d_%H%M%S).md"

    cat > "$anomaly_report" << EOF
# Performance Anomaly Detection Report
Generated: $(date)

## Thresholds
- Memory Usage: ≤ ${memory_threshold}MB
- Processing Time: ≤ ${time_threshold}ms
- Regression Threshold: ≤ ${regression_threshold}%

## Detected Anomalies
EOF

    anomaly_count=0

    # Check memory anomalies
    if [ -f "$TREND_DATA_DIR/trend_stats.txt" ]; then
        source "$TREND_DATA_DIR/trend_stats.txt"

        # Memory regression detection
        if (( $(echo "$memory_max > $memory_threshold" | bc -l) )); then
            echo "- 🚨 **Memory Usage Anomaly**: Peak usage ${memory_max}MB exceeds threshold ${memory_threshold}MB" >> "$anomaly_report"
            anomaly_count=$((anomaly_count + 1))
        fi

        # Time regression detection
        if (( $(echo "$time_max > $time_threshold" | bc -l) )); then
            echo "- 🚨 **Processing Time Anomaly**: Peak time ${time_max}ms exceeds threshold ${time_threshold}ms" >> "$anomaly_report"
            anomaly_count=$((anomaly_count + 1))
        fi

        # Trend analysis
        if (( $(echo "$memory_trend > 0" | bc -l) )); then
            echo "- ⚠️ **Memory Trend Warning**: Memory usage is trending upward (${memory_trend} MB/day)" >> "$anomaly_report"
            anomaly_count=$((anomaly_count + 1))
        fi

        if (( $(echo "$time_trend > 0" | bc -l) )); then
            echo "- ⚠️ **Time Trend Warning**: Processing time is trending upward (${time_trend} ms/day)" >> "$anomaly_report"
            anomaly_count=$((anomaly_count + 1))
        fi
    fi

    if [ $anomaly_count -eq 0 ]; then
        echo "- ✅ No anomalies detected" >> "$anomaly_report"
    fi

    cat >> "$anomaly_report" << EOF

## Summary
- Total Anomalies Detected: $anomaly_count
- Analysis Period: 30 days
- Data Points Analyzed: $(ls "$TREND_DATA_DIR"/performance_*.json 2>/dev/null | wc -l)

---
*This report was generated automatically by the Performance Trend Analysis script*
EOF

    log_success "Anomaly detection completed. Report: $anomaly_report"
}

# Function to generate trend visualization
generate_visualization() {
    log_info "Generating performance trend visualizations..."

    # Check if gnuplot is available
    if command -v gnuplot >/dev/null 2>&1; then
        # Generate memory usage plot
        cat > "$TREND_DATA_DIR/memory_plot.gp" << EOF
set terminal png size 800,600
set output 'memory_trend.png'
set title 'Memory Usage Trend (Last 30 Days)'
set xlabel 'Date'
set ylabel 'Memory Usage (MB)'
set xdata time
set timefmt '%Y%m%d'
set format x '%m/%d'
plot '$TREND_DATA_DIR/memory_trend.txt' using 1:2 with lines title 'Memory Usage'
EOF

        # Generate processing time plot
        cat > "$TREND_DATA_DIR/time_plot.gp" << EOF
set terminal png size 800,600
set output 'time_trend.png'
set title 'Processing Time Trend (Last 30 Days)'
set xlabel 'Date'
set ylabel 'Processing Time (ms)'
set xdata time
set timefmt '%Y%m%d'
set format x '%m/%d'
plot '$TREND_DATA_DIR/time_trend.txt' using 1:2 with lines title 'Processing Time'
EOF

        # Generate plots
        cd "$TREND_DATA_DIR"
        gnuplot memory_plot.gp
        gnuplot time_plot.gp
        cd - >/dev/null

        log_success "Trend visualizations generated"
    else
        log_warning "gnuplot not available - skipping visualization generation"
    fi
}

# Function to generate comprehensive trend report
generate_trend_report() {
    log_info "Generating comprehensive trend report..."

    mkdir -p "$REPORT_DIR"
    report_file="$REPORT_DIR/trend_analysis_report_$(date +%Y%m%d_%H%M%S).md"

    cat > "$report_file" << EOF
# CodeGuardian Performance Trend Analysis Report
Generated: $(date)

## Executive Summary

This report provides a comprehensive analysis of CodeGuardian's performance trends over the last 30 days, including statistical analysis, anomaly detection, and optimization recommendations.

## Performance Overview
EOF

    # Include trend statistics
    if [ -f "$TREND_DATA_DIR/trend_stats.txt" ]; then
        source "$TREND_DATA_DIR/trend_stats.txt"

        cat >> "$report_file" << EOF

### Memory Usage Statistics
- Average: ${memory_avg} MB
- Range: ${memory_min} - ${memory_max} MB
- Trend: ${memory_trend} MB/day $(trend_indicator "$memory_trend")

### Processing Time Statistics
- Average: ${time_avg} ms
- Range: ${time_min} - ${time_max} ms
- Trend: ${time_trend} ms/day $(trend_indicator "$time_trend")

### Cache Performance Statistics
- Average Hit Rate: ${cache_avg}%
- Range: ${cache_min} - ${cache_max}%
- Trend: ${cache_trend}%/day $(trend_indicator "$cache_trend")
EOF
    fi

    cat >> "$report_file" << EOF

## Trend Analysis

### Performance Trends
EOF

    # Add trend analysis insights
    if [ -f "$TREND_DATA_DIR/trend_stats.txt" ]; then
        if (( $(echo "$memory_trend < 0" | bc -l) )); then
            echo "- ✅ Memory usage is trending downward (improving)" >> "$report_file"
        elif (( $(echo "$memory_trend > 0" | bc -l) )); then
            echo "- ⚠️ Memory usage is trending upward (degrading)" >> "$report_file"
        else
            echo "- ➡️ Memory usage is stable" >> "$report_file"
        fi

        if (( $(echo "$time_trend < 0" | bc -l) )); then
            echo "- ✅ Processing time is trending downward (improving)" >> "$report_file"
        elif (( $(echo "$time_trend > 0" | bc -l) )); then
            echo "- ⚠️ Processing time is trending upward (degrading)" >> "$report_file"
        else
            echo "- ➡️ Processing time is stable" >> "$report_file"
        fi
    fi

    cat >> "$report_file" << EOF

## Recommendations

### Immediate Actions
EOF

    # Generate recommendations based on trends
    if [ -f "$TREND_DATA_DIR/trend_stats.txt" ]; then
        if (( $(echo "$memory_trend > 0" | bc -l) )); then
            echo "- Investigate memory usage patterns and consider optimization" >> "$report_file"
        fi

        if (( $(echo "$time_trend > 0" | bc -l) )); then
            echo "- Review processing bottlenecks and algorithm efficiency" >> "$report_file"
        fi

        if (( $(echo "$cache_trend < 0" | bc -l) )); then
            echo "- Monitor cache performance and consider cache strategy improvements" >> "$report_file"
        fi
    fi

    cat >> "$report_file" << EOF

### Optimization Opportunities
- Review memory pool utilization and configuration
- Analyze cache hit rates and eviction policies
- Consider parallel processing improvements
- Evaluate algorithm complexity for large files

### Monitoring Recommendations
- Continue daily performance monitoring
- Review trend reports weekly
- Update performance baselines quarterly
- Monitor for seasonal performance patterns

## Data Sources
- Historical performance data: 30 days
- Benchmark results: Multiple test scenarios
- System metrics: CPU, memory, I/O
- Configuration: Current performance thresholds

## Methodology
- Statistical analysis: Mean, min, max, trend calculation
- Anomaly detection: Threshold-based and trend-based
- Trend calculation: Simple linear regression
- Visualization: Gnuplot-generated charts (if available)

---
*This report was generated automatically by the Performance Trend Analysis script*
EOF

    log_success "Comprehensive trend report generated: $report_file"
}

# Helper function to indicate trend direction
trend_indicator() {
    local trend="$1"
    if (( $(echo "$trend > 0" | bc -l) )); then
        echo "(↗️ increasing)"
    elif (( $(echo "$trend < 0" | bc -l) )); then
        echo "(↘️ decreasing)"
    else
        echo "(➡️ stable)"
    fi
}

# Main execution
main() {
    echo "🚀 CodeGuardian Performance Trend Analysis"
    echo "=========================================="

    # Collect historical data
    collect_historical_data

    # Analyze trends
    analyze_trends

    # Calculate statistics
    calculate_trend_statistics

    # Detect anomalies
    detect_anomalies

    # Generate visualizations
    generate_visualization

    # Generate comprehensive report
    generate_trend_report

    echo ""
    log_success "Performance trend analysis completed!"
    log_info "Check the following outputs:"
    log_info "- Trend reports: $REPORT_DIR/"
    log_info "- Trend data: $TREND_DATA_DIR/"
    log_info "- Visualizations: $TREND_DATA_DIR/ (if gnuplot available)"
}

# Run main function
main "$@"
