#!/bin/bash
# Performance Monitoring Metrics for Duplicate Detection System
# This script collects and reports metrics on the effectiveness of duplicate detection

set -eo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
METRICS_LOG="${GITHUB_ISSUE_CACHE_DIR:-$HOME/.cache/codeguardian/github-issues}/metrics.log"
REPORTS_DIR="$PROJECT_ROOT/reports/duplicate-detection"
mkdir -p "$REPORTS_DIR"
REPORT_FILE="${1:-$REPORTS_DIR/duplicate_detection_metrics_$(date +%Y%m%d_%H%M%S).md}"

# Function to analyze metrics
analyze_metrics() {
    if [ ! -f "$METRICS_LOG" ]; then
        echo "No metrics log found at $METRICS_LOG"
        echo "Creating empty metrics report for first run..."
    fi

    local total_events=0
    local new_issues=0
    local duplicates_prevented=0
    
    if [ -f "$METRICS_LOG" ]; then
        total_events=$(awk 'END{print NR}' "$METRICS_LOG" 2>/dev/null || echo "0")
        new_issues=$(awk '/new_issue/{count++}END{print count+0}' "$METRICS_LOG" 2>/dev/null || echo "0")
        duplicates_prevented=$(awk '/duplicate_update/{count++}END{print count+0}' "$METRICS_LOG" 2>/dev/null || echo "0")
    fi
    
    # Calculate effectiveness metrics
    local prevention_rate=0
    if [ "$total_events" -gt 0 ]; then
        prevention_rate=$(awk "BEGIN{print int($duplicates_prevented * 100 / $total_events)}")
    fi

    # Generate report
    cat > "$REPORT_FILE" << EOF
# Duplicate Detection Effectiveness Report

Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
Metrics Period: Last 30 days

## Summary Metrics

- **Total Events**: $total_events
- **New Issues Created**: $new_issues
- **Duplicates Prevented**: $duplicates_prevented
- **Prevention Rate**: ${prevention_rate}%

## Effectiveness Analysis

EOF

    if [ "$prevention_rate" -ge 20 ]; then
        echo "✅ **Status**: Good - Duplicate prevention is working effectively" >> "$REPORT_FILE"
    elif [ "$prevention_rate" -ge 10 ]; then
        echo "⚠️ **Status**: Moderate - Some duplicates are being prevented" >> "$REPORT_FILE"
    else
        echo "❌ **Status**: Poor - Low duplicate prevention rate" >> "$REPORT_FILE"
    fi

    cat >> "$REPORT_FILE" << EOF

## Detailed Breakdown

### Recent Activity (Last 10 events)
\`\`\`
$(tail -10 "$METRICS_LOG" 2>/dev/null || echo "No recent activity")
\`\`\`

### Recommendations

EOF

    if [ "$prevention_rate" -lt 10 ]; then
        cat >> "$REPORT_FILE" << EOF
- Consider tuning similarity thresholds (current: ${GITHUB_SIMILARITY_THRESHOLD:-0.8})
- Review keyword extraction effectiveness
- Investigate false negative cases
EOF
    elif [ "$prevention_rate" -gt 50 ]; then
        cat >> "$REPORT_FILE" << EOF
- System is performing well
- Monitor for false positives
- Consider expanding to other issue types
EOF
    else
        cat >> "$REPORT_FILE" << EOF
- Performance is acceptable
- Monitor trends over time
- Fine-tune as needed based on feedback
EOF
    fi

    echo "📊 Metrics report generated: $REPORT_FILE"
}

# Function to clean old metrics
cleanup_old_metrics() {
    if [ -f "$METRICS_LOG" ]; then
        # Keep only last 30 days of metrics
        local cutoff_date=$(date -d '30 days ago' +%Y-%m-%d 2>/dev/null || date -v-30d +%Y-%m-%d 2>/dev/null || echo "2024-01-01")
        
        # Create temporary file with recent metrics
        local temp_file=$(mktemp)
        awk -F',' -v cutoff="$cutoff_date" '$1 >= cutoff' "$METRICS_LOG" > "$temp_file" 2>/dev/null || true
        
        # Replace original with filtered metrics
        if [ -s "$temp_file" ]; then
            mv "$temp_file" "$METRICS_LOG"
            echo "🧹 Cleaned old metrics, keeping entries from $cutoff_date onwards"
        else
            rm -f "$temp_file"
        fi
    fi
}

# Main execution
case "${1:-analyze}" in
    "analyze")
        analyze_metrics
        ;;
    "cleanup")
        cleanup_old_metrics
        ;;
    "report")
        analyze_metrics "$2"
        ;;
    *)
        echo "Usage: $0 [analyze|cleanup|report <filename>]"
        exit 1
        ;;
esac