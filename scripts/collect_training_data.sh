#!/bin/bash
# Training Data Collection Pipeline Script
# This script demonstrates how to collect comprehensive training data for CodeGuardian ML models

set -euo pipefail

echo "🏷️  CodeGuardian Training Data Collection Pipeline"
echo "================================================="

# Configuration
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_DIR="${PROJECT_ROOT}/data/training"
CONFIG_FILE="${PROJECT_ROOT}/config/training_data_collection.json"
FINDINGS_FILE="${PROJECT_ROOT}/tmp_findings.json"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if CodeGuardian is available
check_codeguardian() {
    if ! command -v codeguardian &> /dev/null; then
        error "CodeGuardian not found in PATH"
        echo "Please build and install CodeGuardian first:"
        echo "  cargo build --release --features ml"
        echo "  cargo install --path . --features ml"
        exit 1
    fi
    
    info "CodeGuardian found: $(which codeguardian)"
}

# Function to run analysis and collect findings
collect_findings() {
    local source_path="${1:-$PROJECT_ROOT}"
    
    info "Running analysis on: $source_path"
    
    # Run comprehensive analysis
    codeguardian check "$source_path" \
        --format json \
        --out "$FINDINGS_FILE" \
        --detect-duplicates \
        --detect-conflicts \
        --detect-placeholders \
        --detect-broken-files \
        --parallel 0 \
        --quiet
    
    if [[ -f "$FINDINGS_FILE" ]]; then
        local finding_count=$(jq '.findings | length' "$FINDINGS_FILE" 2>/dev/null || echo "0")
        success "Collected $finding_count findings"
    else
        error "Failed to generate findings file"
        exit 1
    fi
}

# Function to run automated labeling
run_automated_labeling() {
    info "Running automated training data collection..."
    
    # Create output directory
    mkdir -p "$OUTPUT_DIR"
    
    # Run training data collection with configuration
    codeguardian training-data \
        --input-file "$FINDINGS_FILE" \
        --output-dir "$OUTPUT_DIR" \
        --config-file "$CONFIG_FILE" \
        --export-formats "json,csv,tfrecord" \
        --labeling-strategies "heuristic,severity_based,file_type_based,analyzer_based"
    
    success "Automated labeling completed"
}

# Function to run interactive labeling
run_interactive_labeling() {
    info "Starting interactive labeling session..."
    warning "This will open an interactive terminal interface"
    
    read -p "Continue with interactive labeling? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        codeguardian training-data \
            --input-file "$FINDINGS_FILE" \
            --output-dir "$OUTPUT_DIR" \
            --interactive \
            --min-examples 100 \
            --skip-manual-review
        
        success "Interactive labeling completed"
    else
        info "Skipping interactive labeling"
    fi
}

# Function to validate training data quality
validate_training_data() {
    info "Validating training data quality..."
    
    if [[ -f "$OUTPUT_DIR/training_data.json" ]]; then
        # Use jq to analyze the training data
        local total_examples=$(jq '.examples | length' "$OUTPUT_DIR/training_data.json")
        local true_positives=$(jq '[.examples[] | select(.is_true_positive == true)] | length' "$OUTPUT_DIR/training_data.json")
        local false_positives=$(jq '[.examples[] | select(.is_true_positive == false)] | length' "$OUTPUT_DIR/training_data.json")
        
        echo "📊 Training Data Statistics:"
        echo "  • Total examples: $total_examples"
        echo "  • True positives: $true_positives"
        echo "  • False positives: $false_positives"
        
        if [[ $total_examples -gt 0 ]]; then
            local balance_ratio=$(echo "scale=2; $true_positives / $false_positives" | bc -l 2>/dev/null || echo "N/A")
            echo "  • Balance ratio: $balance_ratio"
            
            # Quality assessment
            if [[ $total_examples -ge 100 ]]; then
                success "Sufficient training data collected"
            else
                warning "Consider collecting more training examples (minimum 100 recommended)"
            fi
        fi
    else
        error "Training data file not found"
        exit 1
    fi
}

# Function to demonstrate training usage
demonstrate_training() {
    info "Training the ML model with collected data..."
    
    # Train model with collected data
    codeguardian train \
        --training-data "$OUTPUT_DIR/training_data.json" \
        --model-path "$OUTPUT_DIR/trained_model.fann" \
        --epochs 500 \
        --validate \
        --cross-validate \
        --bootstrap
    
    success "Model training completed"
    
    # Display model metrics
    info "Model performance metrics:"
    codeguardian metrics \
        --model-path "$OUTPUT_DIR/trained_model.fann" \
        summary
}

# Function to clean up temporary files
cleanup() {
    info "Cleaning up temporary files..."
    rm -f "$FINDINGS_FILE"
    success "Cleanup completed"
}

# Main execution flow
main() {
    local mode="${1:-automated}"
    local source_path="${2:-$PROJECT_ROOT}"
    
    case "$mode" in
        "automated")
            info "Running automated training data collection"
            check_codeguardian
            collect_findings "$source_path"
            run_automated_labeling
            validate_training_data
            demonstrate_training
            ;;
        "interactive")
            info "Running interactive training data collection"
            check_codeguardian
            collect_findings "$source_path"
            run_automated_labeling
            run_interactive_labeling
            validate_training_data
            demonstrate_training
            ;;
        "validate")
            info "Validating existing training data"
            validate_training_data
            ;;
        "help"|"-h"|"--help")
            echo "Usage: $0 [mode] [source_path]"
            echo "Modes:"
            echo "  automated   - Run automated labeling only (default)"
            echo "  interactive - Run automated + interactive labeling"
            echo "  validate    - Validate existing training data"
            echo "  help        - Show this help"
            echo ""
            echo "Examples:"
            echo "  $0 automated ."
            echo "  $0 interactive /path/to/project"
            echo "  $0 validate"
            exit 0
            ;;
        *)
            error "Unknown mode: $mode"
            echo "Use '$0 help' for usage information"
            exit 1
            ;;
    esac
    
    cleanup
    success "Training data collection pipeline completed!"
    
    echo ""
    echo "📁 Output files:"
    echo "  • Training data: $OUTPUT_DIR/training_data.json"
    echo "  • CSV export: $OUTPUT_DIR/training_data.csv"
    echo "  • TensorFlow format: $OUTPUT_DIR/training_data.tfrecord"
    echo "  • Collection report: $OUTPUT_DIR/collection_report.json"
    echo "  • Trained model: $OUTPUT_DIR/trained_model.fann"
    
    echo ""
    echo "🚀 Next steps:"
    echo "  1. Review the collection report for quality metrics"
    echo "  2. Use the trained model in analysis:"
    echo "     codeguardian check . --ml-model $OUTPUT_DIR/trained_model.fann"
    echo "  3. Collect more data from different projects to improve the model"
}

# Handle script interruption
trap cleanup EXIT

# Run main function with all arguments
main "$@"