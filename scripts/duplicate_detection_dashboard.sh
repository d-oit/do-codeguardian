#!/bin/bash
# Duplicate Detection Monitoring Dashboard
# Creates a comprehensive dashboard for monitoring duplicate detection effectiveness

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CACHE_DIR="${GITHUB_ISSUE_CACHE_DIR:-$HOME/.cache/codeguardian/github-issues}"
METRICS_LOG="$CACHE_DIR/metrics.log"
DASHBOARDS_DIR="$PROJECT_ROOT/reports/dashboards"
mkdir -p "$DASHBOARDS_DIR"
DASHBOARD_FILE="${1:-$DASHBOARDS_DIR/duplicate_detection_dashboard_$(date +%Y%m%d_%H%M%S).html}"

# Function to generate HTML dashboard
generate_dashboard() {
    local total_events=0
    local new_issues=0
    local duplicates_prevented=0
    local prevention_rate=0
    
    # Calculate metrics if log exists
    if [ -f "$METRICS_LOG" ]; then
        total_events=$(awk 'END{print NR}' "$METRICS_LOG" 2>/dev/null || echo "0")
        new_issues=$(awk '/new_issue/{count++}END{print count+0}' "$METRICS_LOG" 2>/dev/null || echo "0")
        duplicates_prevented=$(awk '/duplicate_update/{count++}END{print count+0}' "$METRICS_LOG" 2>/dev/null || echo "0")
        
        if [ "$total_events" -gt 0 ]; then
            prevention_rate=$(awk "BEGIN{print int($duplicates_prevented * 100 / $total_events)}")
        fi
    fi

    # Generate HTML dashboard
    cat > "$DASHBOARD_FILE" << EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Duplicate Detection Dashboard - CodeGuardian</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f5f7fa;
            color: #333;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            padding: 30px;
        }
        .header {
            text-align: center;
            margin-bottom: 40px;
            border-bottom: 2px solid #e1e8ed;
            padding-bottom: 20px;
        }
        .metrics-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 40px;
        }
        .metric-card {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 25px;
            border-radius: 8px;
            text-align: center;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
        }
        .metric-value {
            font-size: 2.5em;
            font-weight: bold;
            margin-bottom: 10px;
        }
        .metric-label {
            font-size: 1.1em;
            opacity: 0.9;
        }
        .status-indicator {
            padding: 10px 20px;
            border-radius: 20px;
            font-weight: bold;
            text-align: center;
            margin: 20px 0;
        }
        .status-good { background-color: #d4edda; color: #155724; }
        .status-moderate { background-color: #fff3cd; color: #856404; }
        .status-poor { background-color: #f8d7da; color: #721c24; }
        .section {
            margin-bottom: 30px;
            padding: 20px;
            background: #f8f9fa;
            border-radius: 6px;
        }
        .section h3 {
            margin-top: 0;
            color: #495057;
        }
        .progress-bar {
            width: 100%;
            height: 20px;
            background-color: #e9ecef;
            border-radius: 10px;
            overflow: hidden;
            margin: 10px 0;
        }
        .progress-fill {
            height: 100%;
            transition: width 0.3s ease;
        }
        .progress-good { background-color: #28a745; }
        .progress-moderate { background-color: #ffc107; }
        .progress-poor { background-color: #dc3545; }
        .recent-activity {
            max-height: 300px;
            overflow-y: auto;
            background: white;
            border: 1px solid #dee2e6;
            border-radius: 4px;
            padding: 15px;
        }
        .activity-item {
            padding: 8px 0;
            border-bottom: 1px solid #f1f3f4;
        }
        .activity-item:last-child {
            border-bottom: none;
        }
        .timestamp {
            color: #6c757d;
            font-size: 0.9em;
        }
        .footer {
            margin-top: 40px;
            text-align: center;
            color: #6c757d;
            font-size: 0.9em;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🛡️ Duplicate Detection Dashboard</h1>
            <p>CodeGuardian Performance Regression Issue Management</p>
            <p><em>Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")</em></p>
        </div>

        <div class="metrics-grid">
            <div class="metric-card">
                <div class="metric-value">$total_events</div>
                <div class="metric-label">Total Events</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">$new_issues</div>
                <div class="metric-label">New Issues Created</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">$duplicates_prevented</div>
                <div class="metric-label">Duplicates Prevented</div>
            </div>
            <div class="metric-card">
                <div class="metric-value">${prevention_rate}%</div>
                <div class="metric-label">Prevention Rate</div>
            </div>
        </div>

        <div class="section">
            <h3>🎯 System Effectiveness</h3>
EOF

    # Add status indicator based on prevention rate
    if [ "$prevention_rate" -ge 20 ]; then
        echo '            <div class="status-indicator status-good">✅ Excellent - Duplicate prevention is highly effective</div>' >> "$DASHBOARD_FILE"
        echo '            <div class="progress-bar"><div class="progress-fill progress-good" style="width: '"$prevention_rate"'%;"></div></div>' >> "$DASHBOARD_FILE"
    elif [ "$prevention_rate" -ge 10 ]; then
        echo '            <div class="status-indicator status-moderate">⚠️ Good - Moderate duplicate prevention effectiveness</div>' >> "$DASHBOARD_FILE"
        echo '            <div class="progress-bar"><div class="progress-fill progress-moderate" style="width: '"$prevention_rate"'%;"></div></div>' >> "$DASHBOARD_FILE"
    else
        echo '            <div class="status-indicator status-poor">❌ Needs Improvement - Low duplicate prevention rate</div>' >> "$DASHBOARD_FILE"
        echo '            <div class="progress-bar"><div class="progress-fill progress-poor" style="width: '"$prevention_rate"'%;"></div></div>' >> "$DASHBOARD_FILE"
    fi

    cat >> "$DASHBOARD_FILE" << EOF
        </div>

        <div class="section">
            <h3>📊 Recent Activity</h3>
            <div class="recent-activity">
EOF

    # Add recent activity if metrics exist
    if [ -f "$METRICS_LOG" ] && [ -s "$METRICS_LOG" ]; then
        echo "                <h4>Last 20 Events:</h4>" >> "$DASHBOARD_FILE"
        tail -20 "$METRICS_LOG" | while IFS=',' read -r timestamp action repo issue_number; do
            local action_emoji="🆕"
            local action_text="New Issue Created"
            if [[ "$action" == "duplicate_update" ]]; then
                action_emoji="🔄"
                action_text="Duplicate Prevented - Updated Existing Issue"
            fi
            
            echo "                <div class=\"activity-item\">" >> "$DASHBOARD_FILE"
            echo "                    <strong>$action_emoji $action_text</strong><br>" >> "$DASHBOARD_FILE"
            echo "                    <span class=\"timestamp\">$timestamp</span> - Repository: $repo - Issue: #$issue_number" >> "$DASHBOARD_FILE"
            echo "                </div>" >> "$DASHBOARD_FILE"
        done
    else
        echo "                <p><em>No recent activity recorded</em></p>" >> "$DASHBOARD_FILE"
    fi

    cat >> "$DASHBOARD_FILE" << EOF
            </div>
        </div>

        <div class="section">
            <h3>⚙️ Configuration</h3>
            <table style="width: 100%; border-collapse: collapse;">
                <tr style="background-color: #f8f9fa;">
                    <th style="padding: 10px; text-align: left; border: 1px solid #dee2e6;">Parameter</th>
                    <th style="padding: 10px; text-align: left; border: 1px solid #dee2e6;">Value</th>
                </tr>
                <tr>
                    <td style="padding: 10px; border: 1px solid #dee2e6;">Cache TTL</td>
                    <td style="padding: 10px; border: 1px solid #dee2e6;">${GITHUB_ISSUE_CACHE_TTL:-3600} seconds</td>
                </tr>
                <tr>
                    <td style="padding: 10px; border: 1px solid #dee2e6;">Max Keywords</td>
                    <td style="padding: 10px; border: 1px solid #dee2e6;">${GITHUB_MAX_KEYWORDS:-5}</td>
                </tr>
                <tr>
                    <td style="padding: 10px; border: 1px solid #dee2e6;">Similarity Threshold</td>
                    <td style="padding: 10px; border: 1px solid #dee2e6;">${GITHUB_SIMILARITY_THRESHOLD:-0.8}</td>
                </tr>
                <tr>
                    <td style="padding: 10px; border: 1px solid #dee2e6;">Cache Invalidation</td>
                    <td style="padding: 10px; border: 1px solid #dee2e6;">${GITHUB_CACHE_INVALIDATION:-true}</td>
                </tr>
            </table>
        </div>

        <div class="section">
            <h3>📈 Recommendations</h3>
EOF

    # Add recommendations based on performance
    if [ "$prevention_rate" -lt 10 ]; then
        cat >> "$DASHBOARD_FILE" << EOF
            <ul>
                <li>🔧 Consider lowering the similarity threshold to catch more duplicates</li>
                <li>📝 Review and expand the keyword extraction patterns</li>
                <li>🕵️ Investigate false negative cases manually</li>
                <li>⏰ Consider shortening cache TTL for more up-to-date results</li>
                <li>📊 Enable verbose logging to debug detection issues</li>
            </ul>
EOF
    elif [ "$prevention_rate" -gt 50 ]; then
        cat >> "$DASHBOARD_FILE" << EOF
            <ul>
                <li>✅ System is performing excellently</li>
                <li>👀 Monitor for potential false positives</li>
                <li>🚀 Consider expanding duplicate detection to other issue types</li>
                <li>📊 Maintain current configuration settings</li>
                <li>🔄 Regular monitoring and maintenance is sufficient</li>
            </ul>
EOF
    else
        cat >> "$DASHBOARD_FILE" << EOF
            <ul>
                <li>📊 Performance is acceptable - continue monitoring</li>
                <li>🔍 Fine-tune parameters based on observed patterns</li>
                <li>📝 Consider feedback from development team</li>
                <li>⚡ Optimize for better performance if needed</li>
                <li>📈 Track trends over longer periods</li>
            </ul>
EOF
    fi

    cat >> "$DASHBOARD_FILE" << EOF
        </div>

        <div class="footer">
            <p>🛡️ CodeGuardian Duplicate Detection System | Generated by performance monitoring dashboard</p>
            <p>For technical support, refer to the project documentation or create an issue in the repository.</p>
        </div>
    </div>
</body>
</html>
EOF

    echo "📊 Dashboard generated: $DASHBOARD_FILE"
}

# Main execution
case "${1:-generate}" in
    "generate")
        generate_dashboard "${2:-}"
        ;;
    *)
        echo "Usage: $0 [generate] [output_file]"
        exit 1
        ;;
esac