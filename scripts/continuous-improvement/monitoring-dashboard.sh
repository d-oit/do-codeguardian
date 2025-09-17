#!/bin/bash
# CodeGuardian Monitoring Dashboard
# Real-time monitoring and alerting for CodeGuardian systems

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuration
MONITORING_DIR="$PROJECT_ROOT/monitoring"
ALERTS_DIR="$MONITORING_DIR/alerts"
DASHBOARD_DIR="$MONITORING_DIR/dashboard"
LOGS_DIR="$PROJECT_ROOT/logs"

# Thresholds for alerts
CPU_THRESHOLD=80
MEMORY_THRESHOLD=85
DISK_THRESHOLD=90
ERROR_RATE_THRESHOLD=5

# Ensure directories exist
mkdir -p "$MONITORING_DIR" "$ALERTS_DIR" "$DASHBOARD_DIR"

# Logging functions
log_info() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')] ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] ✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] ⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ❌ $1${NC}"
}

log_alert() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] 🚨 ALERT: $1${NC}"
}

# Function to collect system metrics
collect_system_metrics() {
    METRICS_FILE="$MONITORING_DIR/system_metrics_$(date +%Y%m%d_%H%M%S).json"

    # Get CPU usage
    CPU_USAGE=$(top -bn1 | grep "Cpu(s)" | sed "s/.*, *\([0-9.]*\)%* id.*/\1/" | awk '{print 100 - $1}' 2>/dev/null || echo "0")

    # Get memory usage
    MEMORY_USAGE=$(free | grep Mem | awk '{printf "%.2f", $3/$2 * 100.0}' 2>/dev/null || echo "0")

    # Get disk usage
    DISK_USAGE=$(df / | tail -1 | awk '{print $5}' | sed 's/%//' 2>/dev/null || echo "0")

    # Get process info
    PROCESS_INFO=$(ps aux | grep do-codeguardian | head -1 | awk '{print $2 "," $3 "," $4 "," $6}' 2>/dev/null || echo "0,0,0,0")
    PID=$(echo $PROCESS_INFO | cut -d',' -f1)
    PROCESS_CPU=$(echo $PROCESS_INFO | cut -d',' -f2)
    PROCESS_MEM=$(echo $PROCESS_INFO | cut -d',' -f3)
    PROCESS_RSS=$(echo $PROCESS_INFO | cut -d',' -f4)

    cat > "$METRICS_FILE" << METRICS_EOF
{
    "timestamp": "$(date -Iseconds)",
    "system": {
        "cpu_usage_percent": $CPU_USAGE,
        "memory_usage_percent": $MEMORY_USAGE,
        "disk_usage_percent": $DISK_USAGE
    },
    "process": {
        "pid": "$PID",
        "cpu_percent": "$PROCESS_CPU",
        "memory_percent": "$PROCESS_MEM",
        "rss_kb": "$PROCESS_RSS"
    },
    "thresholds": {
        "cpu_threshold": $CPU_THRESHOLD,
        "memory_threshold": $MEMORY_THRESHOLD,
        "disk_threshold": $DISK_THRESHOLD
    }
}
METRICS_EOF

    echo "$METRICS_FILE"
}

# Function to check for alerts
check_alerts() {
    METRICS_FILE="$1"

    if [ ! -f "$METRICS_FILE" ]; then
        log_error "Metrics file not found: $METRICS_FILE"
        return 1
    fi

    # Extract values using grep and awk
    CPU_USAGE=$(grep '"cpu_usage_percent"' "$METRICS_FILE" | grep -o '[0-9.]*' | head -1)
    MEMORY_USAGE=$(grep '"memory_usage_percent"' "$METRICS_FILE" | grep -o '[0-9.]*' | head -1)
    DISK_USAGE=$(grep '"disk_usage_percent"' "$METRICS_FILE" | grep -o '[0-9]*' | head -1)

    ALERTS_TRIGGERED=0

    # Check CPU threshold
    if (( $(echo "$CPU_USAGE > $CPU_THRESHOLD" | bc -l 2>/dev/null || echo "0") )); then
        ALERT_FILE="$ALERTS_DIR/cpu_alert_$(date +%Y%m%d_%H%M%S).json"
        cat > "$ALERT_FILE" << ALERT_EOF
{
    "timestamp": "$(date -Iseconds)",
    "alert_type": "cpu_usage_high",
    "severity": "warning",
    "message": "CPU usage is ${CPU_USAGE}% (threshold: ${CPU_THRESHOLD}%)",
    "value": $CPU_USAGE,
    "threshold": $CPU_THRESHOLD
}
ALERT_EOF
        log_alert "High CPU usage detected: ${CPU_USAGE}%"
        ALERTS_TRIGGERED=$((ALERTS_TRIGGERED + 1))
    fi

    # Check memory threshold
    if (( $(echo "$MEMORY_USAGE > $MEMORY_THRESHOLD" | bc -l 2>/dev/null || echo "0") )); then
        ALERT_FILE="$ALERTS_DIR/memory_alert_$(date +%Y%m%d_%H%M%S).json"
        cat > "$ALERT_FILE" << ALERT_EOF
{
    "timestamp": "$(date -Iseconds)",
    "alert_type": "memory_usage_high",
    "severity": "warning",
    "message": "Memory usage is ${MEMORY_USAGE}% (threshold: ${MEMORY_THRESHOLD}%)",
    "value": $MEMORY_USAGE,
    "threshold": $MEMORY_THRESHOLD
}
ALERT_EOF
        log_alert "High memory usage detected: ${MEMORY_USAGE}%"
        ALERTS_TRIGGERED=$((ALERTS_TRIGGERED + 1))
    fi

    # Check disk threshold
    if [ "$DISK_USAGE" -gt "$DISK_THRESHOLD" ] 2>/dev/null; then
        ALERT_FILE="$ALERTS_DIR/disk_alert_$(date +%Y%m%d_%H%M%S).json"
        cat > "$ALERT_FILE" << ALERT_EOF
{
    "timestamp": "$(date -Iseconds)",
    "alert_type": "disk_usage_high",
    "severity": "critical",
    "message": "Disk usage is ${DISK_USAGE}% (threshold: ${DISK_THRESHOLD}%)",
    "value": $DISK_USAGE,
    "threshold": $DISK_THRESHOLD
}
ALERT_EOF
        log_alert "High disk usage detected: ${DISK_USAGE}%"
        ALERTS_TRIGGERED=$((ALERTS_TRIGGERED + 1))
    fi

    # Check for error logs
    if ls "$LOGS_DIR/"*.log >/dev/null 2>&1; then
        ERROR_COUNT=$(grep -i "error\|failed\|exception" "$LOGS_DIR/"*.log 2>/dev/null | wc -l)
        TOTAL_LOG_LINES=$(wc -l "$LOGS_DIR/"*.log 2>/dev/null | tail -1 | awk '{print $1}' || echo "1")

        if [ "$TOTAL_LOG_LINES" -gt 0 ]; then
            ERROR_RATE=$((ERROR_COUNT * 100 / TOTAL_LOG_LINES))

            if [ "$ERROR_RATE" -gt "$ERROR_RATE_THRESHOLD" ]; then
                ALERT_FILE="$ALERTS_DIR/error_rate_alert_$(date +%Y%m%d_%H%M%S).json"
                cat > "$ALERT_FILE" << ALERT_EOF
{
    "timestamp": "$(date -Iseconds)",
    "alert_type": "error_rate_high",
    "severity": "critical",
    "message": "Error rate is ${ERROR_RATE}% (threshold: ${ERROR_RATE_THRESHOLD}%)",
    "value": $ERROR_RATE,
    "threshold": $ERROR_RATE_THRESHOLD
}
ALERT_EOF
                log_alert "High error rate detected: ${ERROR_RATE}%"
                ALERTS_TRIGGERED=$((ALERTS_TRIGGERED + 1))
            fi
        fi
    fi

    echo $ALERTS_TRIGGERED
}

# Function to generate dashboard
generate_dashboard() {
    DASHBOARD_FILE="$DASHBOARD_DIR/dashboard_$(date +%Y%m%d_%H%M%S).html"

    # Get latest metrics
    LATEST_METRICS=$(ls -t "$MONITORING_DIR"/system_metrics_*.json 2>/dev/null | head -1)

    if [ -n "$LATEST_METRICS" ]; then
        CPU_USAGE=$(grep '"cpu_usage_percent"' "$LATEST_METRICS" | grep -o '[0-9.]*' | head -1)
        MEMORY_USAGE=$(grep '"memory_usage_percent"' "$LATEST_METRICS" | grep -o '[0-9.]*' | head -1)
        DISK_USAGE=$(grep '"disk_usage_percent"' "$LATEST_METRICS" | grep -o '[0-9]*' | head -1)
    else
        CPU_USAGE="0"
        MEMORY_USAGE="0"
        DISK_USAGE="0"
    fi

    # Count recent alerts
    ALERT_COUNT=$(find "$ALERTS_DIR" -name "*.json" -mtime -1 2>/dev/null | wc -l)

    cat > "$DASHBOARD_FILE" << DASHBOARD_EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CodeGuardian Monitoring Dashboard</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #f5f5f5; color: #333; }
        .header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 1rem 2rem; }
        .header h1 { font-size: 2rem; font-weight: 600; }
        .container { max-width: 1200px; margin: 0 auto; padding: 2rem; }
        .metrics-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; margin-bottom: 2rem; }
        .metric-card { background: white; border-radius: 8px; padding: 1.5rem; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }
        .metric-card h3 { color: #667eea; margin-bottom: 1rem; }
        .metric-value { font-size: 2rem; font-weight: bold; margin-bottom: 0.5rem; }
        .metric-label { color: #666; font-size: 0.9rem; }
        .status-indicator { display: inline-block; width: 12px; height: 12px; border-radius: 50%; margin-right: 0.5rem; }
        .status-healthy { background-color: #10b981; }
        .status-warning { background-color: #f59e0b; }
        .status-critical { background-color: #ef4444; }
        .alerts-section { background: white; border-radius: 8px; padding: 1.5rem; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }
        .alert-item { padding: 0.5rem 0; border-bottom: 1px solid #eee; }
        .alert-item:last-child { border-bottom: none; }
        .timestamp { color: #666; font-size: 0.8rem; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🔍 CodeGuardian Monitoring Dashboard</h1>
        <p>Real-time system monitoring and alerting</p>
    </div>

    <div class="container">
        <div class="metrics-grid">
            <div class="metric-card">
                <h3>CPU Usage</h3>
                <div class="metric-value">${CPU_USAGE}%</div>
                <div class="metric-label">
                    <span class="status-indicator $( [ "${CPU_USAGE%.*}" -gt "$CPU_THRESHOLD" ] && echo "status-critical" || echo "status-healthy" )"></span>
                    $( [ "${CPU_USAGE%.*}" -gt "$CPU_THRESHOLD" ] && echo "High" || echo "Normal" )
                </div>
            </div>

            <div class="metric-card">
                <h3>Memory Usage</h3>
                <div class="metric-value">${MEMORY_USAGE}%</div>
                <div class="metric-label">
                    <span class="status-indicator $( [ "${MEMORY_USAGE%.*}" -gt "$MEMORY_THRESHOLD" ] && echo "status-warning" || echo "status-healthy" )"></span>
                    $( [ "${MEMORY_USAGE%.*}" -gt "$MEMORY_THRESHOLD" ] && echo "High" || echo "Normal" )
                </div>
            </div>

            <div class="metric-card">
                <h3>Disk Usage</h3>
                <div class="metric-value">${DISK_USAGE}%</div>
                <div class="metric-label">
                    <span class="status-indicator $( [ "$DISK_USAGE" -gt "$DISK_THRESHOLD" ] && echo "status-critical" || echo "status-healthy" )"></span>
                    $( [ "$DISK_USAGE" -gt "$DISK_THRESHOLD" ] && echo "High" || echo "Normal" )
                </div>
            </div>

            <div class="metric-card">
                <h3>Active Alerts</h3>
                <div class="metric-value">${ALERT_COUNT}</div>
                <div class="metric-label">
                    <span class="status-indicator $( [ "$ALERT_COUNT" -gt 0 ] && echo "status-warning" || echo "status-healthy" )"></span>
                    Last 24 hours
                </div>
            </div>
        </div>

        <div class="alerts-section">
            <h3>Recent Alerts</h3>
            <div id="alerts-list">
DASHBOARD_EOF

    # Add recent alerts to dashboard
    find "$ALERTS_DIR" -name "*.json" -mtime -1 2>/dev/null | head -5 | while read -r alert_file; do
        if [ -f "$alert_file" ]; then
            TIMESTAMP=$(grep '"timestamp"' "$alert_file" | sed 's/.*"timestamp": "\([^"]*\)".*/\1/' | cut -d'T' -f1)
            ALERT_TYPE=$(grep '"alert_type"' "$alert_file" | sed 's/.*"alert_type": "\([^"]*\)".*/\1/')
            MESSAGE=$(grep '"message"' "$alert_file" | sed 's/.*"message": "\([^"]*\)".*/\1/')
            SEVERITY=$(grep '"severity"' "$alert_file" | sed 's/.*"severity": "\([^"]*\)".*/\1/')

            echo "                <div class=\"alert-item\">" >> "$DASHBOARD_FILE"
            echo "                    <strong>$ALERT_TYPE</strong> ($SEVERITY)<br>" >> "$DASHBOARD_FILE"
            echo "                    <span class=\"timestamp\">$TIMESTAMP</span><br>" >> "$DASHBOARD_FILE"
            echo "                    $MESSAGE" >> "$DASHBOARD_FILE"
            echo "                </div>" >> "$DASHBOARD_FILE"
        fi
    done

    cat >> "$DASHBOARD_EOF" << DASHBOARD_EOF
            </div>
        </div>
    </div>

    <script>
        // Auto-refresh dashboard every 30 seconds
        setTimeout(function() {
            if (window.location.reload) {
                window.location.reload();
            }
        }, 30000);
    </script>
</body>
</html>
DASHBOARD_EOF

    echo "$DASHBOARD_FILE"
}

# Function to display dashboard in terminal
display_terminal_dashboard() {
    echo ""
    echo "╔══════════════════════════════════════════════════════════════════════════════╗"
    echo "║                         CodeGuardian Monitoring Dashboard                    ║"
    echo "╠══════════════════════════════════════════════════════════════════════════════╣"

    # Get latest metrics
    LATEST_METRICS=$(ls -t "$MONITORING_DIR"/system_metrics_*.json 2>/dev/null | head -1)

    if [ -n "$LATEST_METRICS" ]; then
        CPU_USAGE=$(grep '"cpu_usage_percent"' "$LATEST_METRICS" | grep -o '[0-9.]*' | head -1)
        MEMORY_USAGE=$(grep '"memory_usage_percent"' "$LATEST_METRICS" | grep -o '[0-9.]*' | head -1)
        DISK_USAGE=$(grep '"disk_usage_percent"' "$LATEST_METRICS" | grep -o '[0-9]*' | head -1)

        echo "║ CPU Usage:    $CPU_USAGE% $( [ "${CPU_USAGE%.*}" -gt "$CPU_THRESHOLD" ] && echo "🔴 HIGH" || echo "🟢 OK" )"
        echo "║ Memory Usage: $MEMORY_USAGE% $( [ "${MEMORY_USAGE%.*}" -gt "$MEMORY_THRESHOLD" ] && echo "🟡 HIGH" || echo "🟢 OK" )"
        echo "║ Disk Usage:   $DISK_USAGE% $( [ "$DISK_USAGE" -gt "$DISK_THRESHOLD" ] && echo "🔴 CRITICAL" || echo "🟢 OK" )"
    else
        echo "║ No metrics data available"
    fi

    # Count alerts
    ALERT_COUNT=$(find "$ALERTS_DIR" -name "*.json" -mtime -1 2>/dev/null | wc -l)
    echo "║ Active Alerts: $ALERT_COUNT $( [ "$ALERT_COUNT" -gt 0 ] && echo "⚠️" || echo "✅" )"

    echo "╠══════════════════════════════════════════════════════════════════════════════╣"
    echo "║ Timestamp: $(date)                                                        ║"
    echo "╚══════════════════════════════════════════════════════════════════════════════╝"
    echo ""
}

# Main execution
main() {
    echo ""
    log_info "CodeGuardian Monitoring Dashboard"
    echo "=================================="

    # Collect system metrics
    log_info "Collecting system metrics..."
    METRICS_FILE=$(collect_system_metrics)
    log_success "Metrics collected: $METRICS_FILE"

    # Check for alerts
    log_info "Checking for alerts..."
    ALERTS_COUNT=$(check_alerts "$METRICS_FILE")

    if [ "$ALERTS_COUNT" -gt 0 ]; then
        log_warning "$ALERTS_COUNT alert(s) triggered"
    else
        log_success "No alerts triggered"
    fi

    # Generate HTML dashboard
    log_info "Generating HTML dashboard..."
    DASHBOARD_FILE=$(generate_dashboard)
    log_success "Dashboard generated: $DASHBOARD_FILE"

    # Display terminal dashboard
    display_terminal_dashboard

    # Summary
    echo "📊 Monitoring Summary:"
    echo "  - System metrics collected: ✅"
    echo "  - Alerts checked: ✅ ($ALERTS_COUNT triggered)"
    echo "  - Dashboard generated: ✅"
    echo ""
    echo "📁 Files created:"
    echo "  - Metrics: $METRICS_FILE"
    echo "  - Dashboard: $DASHBOARD_FILE"
    if [ "$ALERTS_COUNT" -gt 0 ]; then
        echo "  - Alerts: $ALERTS_DIR/"
    fi
    echo ""
    echo "🔄 Dashboard auto-refreshes every 30 seconds when opened in browser"
    echo "💡 To view alerts: cat $ALERTS_DIR/*.json"
}

# Run main function
main "$@"
