#!/bin/bash
# CodeGuardian Continuous Improvement System Demo
# Demonstrates the complete continuous improvement workflow

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

# Demo configuration
DEMO_DURATION=30
QUICK_MODE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --quick)
            QUICK_MODE=true
            DEMO_DURATION=10
            shift
            ;;
        --duration=*)
            DEMO_DURATION="${1#*=}"
            shift
            ;;
        *)
            echo "Usage: $0 [--quick] [--duration=SECONDS]"
            exit 1
            ;;
    esac
done

# Logging functions
log_header() {
    echo -e "\n${CYAN}╔══════════════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║ $1${NC}"
    echo -e "${CYAN}╚══════════════════════════════════════════════════════════════════════════════╝${NC}"
}

log_step() {
    echo -e "\n${MAGENTA}▶️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Function to show system status
show_system_status() {
    log_step "Checking system status..."

    echo "📊 Current System Status:"
    echo "  - CPU Usage: $(top -bn1 | grep "Cpu(s)" | sed "s/.*, *\([0-9.]*\)%* id.*/\1/" | awk '{print 100 - $1}')%"
    echo "  - Memory Usage: $(free | grep Mem | awk '{printf "%.1f", $3/$2 * 100.0}')%"
    echo "  - Disk Usage: $(df / | tail -1 | awk '{print $5}')%"
    echo "  - Active Processes: $(ps aux | wc -l)"

    log_success "System status checked"
}

# Function to demonstrate monitoring
demo_monitoring() {
    log_step "Running monitoring dashboard..."

    if [ "$QUICK_MODE" = true ]; then
        log_info "Quick mode: Skipping full monitoring demo"
        return
    fi

    # Run monitoring dashboard
    if "$SCRIPT_DIR/monitoring-dashboard.sh" > /dev/null 2>&1; then
        log_success "Monitoring dashboard completed"

        # Show latest metrics
        LATEST_METRICS=$(ls -t "$PROJECT_ROOT/monitoring"/system_metrics_*.json 2>/dev/null | head -1)
        if [ -n "$LATEST_METRICS" ]; then
            echo "📈 Latest Metrics: $LATEST_METRICS"
        fi

        # Show dashboard location
        LATEST_DASHBOARD=$(ls -t "$PROJECT_ROOT/monitoring/dashboard"/dashboard_*.html 2>/dev/null | head -1)
        if [ -n "$LATEST_DASHBOARD" ]; then
            echo "🌐 Dashboard: $LATEST_DASHBOARD"
        fi
    else
        log_warning "Monitoring dashboard had some issues"
    fi
}

# Function to demonstrate feedback collection
demo_feedback() {
    log_step "Running feedback collection..."

    if [ "$QUICK_MODE" = true ]; then
        log_info "Quick mode: Skipping full feedback demo"
        return
    fi

    # Run feedback collector
    if "$SCRIPT_DIR/feedback-collector.sh" > /dev/null 2>&1; then
        log_success "Feedback collection completed"

        # Show generated files
        echo "📋 Generated Files:"
        find "$PROJECT_ROOT/feedback" -name "*.md" -o -name "*.json" 2>/dev/null | head -5 | while read -r file; do
            echo "  - $(basename "$file")"
        done
    else
        log_warning "Feedback collection had some issues"
    fi
}

# Function to demonstrate continuous improvement
demo_continuous_improvement() {
    log_step "Running continuous improvement cycle..."

    # Run continuous improvement manager
    if timeout "${DEMO_DURATION}s" "$SCRIPT_DIR/continuous-improvement-manager.sh" > /dev/null 2>&1; then
        log_success "Continuous improvement cycle completed"

        # Show generated reports
        echo "📊 Generated Reports:"
        find "$PROJECT_ROOT/reports" -name "*.md" 2>/dev/null | head -5 | while read -r file; do
            echo "  - $(basename "$file")"
        done

        # Show metrics
        echo "📈 Collected Metrics:"
        find "$PROJECT_ROOT/metrics" -name "*.json" 2>/dev/null | head -3 | while read -r file; do
            echo "  - $(basename "$file")"
        done
    else
        log_warning "Continuous improvement cycle timed out or had issues"
    fi
}

# Function to show results summary
show_results_summary() {
    log_step "Generating results summary..."

    echo ""
    echo "📊 Demo Results Summary"
    echo "======================="

    # Count generated files
    METRICS_COUNT=$(find "$PROJECT_ROOT/metrics" "$PROJECT_ROOT/monitoring" "$PROJECT_ROOT/reports" "$PROJECT_ROOT/feedback" -name "*.json" -o -name "*.md" -o -name "*.html" 2>/dev/null | wc -l)

    echo "📁 Files Generated: $METRICS_COUNT"

    # Show directory structure
    echo ""
    echo "📂 Generated Directory Structure:"
    find "$PROJECT_ROOT/metrics" "$PROJECT_ROOT/monitoring" "$PROJECT_ROOT/reports" "$PROJECT_ROOT/feedback" -type d 2>/dev/null | head -10 | while read -r dir; do
        REL_PATH=${dir#$PROJECT_ROOT/}
        echo "  📁 $REL_PATH/"
    done

    # Show key files
    echo ""
    echo "🔑 Key Generated Files:"
    find "$PROJECT_ROOT" -path "*/metrics/*" -o -path "*/reports/*" -o -path "*/monitoring/*" -o -path "*/feedback/*" \( -name "*.md" -o -name "*.html" \) 2>/dev/null | head -10 | while read -r file; do
        REL_PATH=${file#$PROJECT_ROOT/}
        FILE_TYPE=""
        case "${file##*.}" in
            md) FILE_TYPE="📄" ;;
            html) FILE_TYPE="🌐" ;;
            json) FILE_TYPE="📊" ;;
        esac
        echo "  $FILE_TYPE $REL_PATH"
    done
}

# Function to show next steps
show_next_steps() {
    log_step "Next steps and recommendations..."

    echo ""
    echo "🚀 Next Steps"
    echo "============="
    echo ""
    echo "1. 📊 Review Generated Reports"
    echo "   - Check reports/ for optimization recommendations"
    echo "   - Review feedback/ for user insights"
    echo "   - Analyze monitoring/ for system health"
    echo ""
    echo "2. 🔧 Implement Recommendations"
    echo "   - Prioritize high-impact optimizations"
    echo "   - Schedule implementation in development sprints"
    echo "   - Track progress against success metrics"
    echo ""
    echo "3. 📈 Monitor Progress"
    echo "   - Run monitoring dashboard regularly"
    echo "   - Track performance trends over time"
    echo "   - Collect ongoing user feedback"
    echo ""
    echo "4. 🔄 Automate with GitHub Actions"
    echo "   - Enable the continuous-improvement.yml workflow"
    echo "   - Schedule regular improvement cycles"
    echo "   - Monitor automated issue creation"
    echo ""
    echo "5. 📚 Explore Advanced Features"
    echo "   - Customize alert thresholds"
    echo "   - Integrate with external monitoring systems"
    echo "   - Extend feedback collection mechanisms"
}

# Main demo execution
main() {
    echo ""
    log_header "CodeGuardian Continuous Improvement System Demo"
    echo ""
    log_info "This demo will run the complete continuous improvement workflow"
    log_info "Duration: ${DEMO_DURATION} seconds per major component"
    if [ "$QUICK_MODE" = true ]; then
        log_info "Running in quick mode (reduced functionality)"
    fi
    echo ""

    # Pre-demo checks
    log_step "Performing pre-demo checks..."

    # Check if scripts exist and are executable
    for script in "monitoring-dashboard.sh" "feedback-collector.sh" "continuous-improvement-manager.sh"; do
        if [ ! -x "$SCRIPT_DIR/$script" ]; then
            log_warning "Script $script not found or not executable"
            log_info "Run: chmod +x $SCRIPT_DIR/$script"
        fi
    done

    # Check dependencies
    MISSING_DEPS=""
    for dep in jq bc curl; do
        if ! command -v "$dep" >/dev/null 2>&1; then
            MISSING_DEPS="$MISSING_DEPS $dep"
        fi
    done

    if [ -n "$MISSING_DEPS" ]; then
        log_warning "Missing dependencies:$MISSING_DEPS"
        log_info "Install with: sudo apt-get install$MISSSING_DEPS"
    fi

    log_success "Pre-demo checks completed"

    # Show initial system status
    show_system_status

    # Run monitoring demo
    demo_monitoring

    # Run feedback demo
    demo_feedback

    # Run continuous improvement demo
    demo_continuous_improvement

    # Show results
    show_results_summary

    # Show next steps
    show_next_steps

    # Final summary
    echo ""
    log_header "Demo Completed Successfully! 🎉"
    echo ""
    log_success "The continuous improvement system is now set up and running"
    log_info "Regular automated cycles will help maintain CodeGuardian's performance and user satisfaction"
    echo ""
    echo "📚 For more information, see: $SCRIPT_DIR/README.md"
    echo "🔧 To customize the system, edit the scripts in: $SCRIPT_DIR/"
    echo "🤝 Contribute improvements at: https://github.com/your-org/codeguardian"
}

# Handle interrupts gracefully
trap 'echo -e "\n${YELLOW}Demo interrupted by user${NC}"; exit 1' INT TERM

# Run main demo
main "$@"
