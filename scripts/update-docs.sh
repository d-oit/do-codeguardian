#!/bin/bash

# CodeGuardian Documentation Update Script
# This script uses opencode to automatically update project documentation before commits

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if opencode is installed
check_opencode() {
    if ! command -v opencode &> /dev/null; then
        print_error "opencode is not installed or not in PATH"
        print_error "Please install opencode first: curl -fsSL https://opencode.ai/install | bash"
        exit 1
    fi
    print_success "opencode found: $(opencode --version)"
}

# Initialize opencode for the project if needed
initialize_opencode() {
    if [ ! -f "AGENTS.md" ]; then
        print_status "Initializing opencode for the project..."

        # Create a temporary opencode session to initialize
        echo "/init" | timeout 30 opencode || true

        if [ -f "AGENTS.md" ]; then
            print_success "opencode initialized successfully"
        else
            print_warning "opencode initialization may have failed, but continuing..."
        fi
    else
        print_status "opencode already initialized (AGENTS.md exists)"
    fi
}

# Update README.md with current project information
update_readme() {
    print_status "Updating README.md with current project information..."

    # Use opencode to analyze and update README
    local prompt="Please analyze the current codebase and update the README.md file with:
1. Current project structure and key components
2. Updated installation and setup instructions
3. Current features and capabilities
4. Any recent changes or improvements
5. Keep the existing format and style but ensure accuracy"

    echo "$prompt" | timeout 60 opencode || print_warning "README update timed out or failed"
}

# Update API documentation
update_api_docs() {
    if [ -d "src" ]; then
        print_status "Updating API documentation..."

        local prompt="Please analyze the source code in the src/ directory and update any API documentation files.
Focus on:
1. Public functions and their parameters
2. Module structure and dependencies
3. Configuration options
4. Usage examples
Update docs/ directory files or create new ones as needed."

        echo "$prompt" | timeout 60 opencode || print_warning "API docs update timed out or failed"
    fi
}

# Update performance and analysis documentation
update_performance_docs() {
    if [ -f "PERFORMANCE_ANALYSIS_SUMMARY.md" ] || [ -d "docs" ]; then
        print_status "Updating performance documentation..."

        local prompt="Please analyze the current performance benchmarks and analysis results.
Update the performance documentation with:
1. Latest benchmark results
2. Performance improvements or regressions
3. Optimization recommendations
4. Current performance metrics"

        echo "$prompt" | timeout 60 opencode || print_warning "Performance docs update timed out or failed"
    fi
}

# Update security analyzer documentation
update_security_docs() {
    if [ -f "docs/SECURITY_ANALYZER.md" ] || [ -d "docs" ]; then
        print_status "Updating security analyzer documentation..."

        local prompt="Please analyze the security analyzer code and update the security documentation.
Include:
1. Current security checks and their implementation
2. Security features and capabilities
3. Configuration options for security analysis
4. Security best practices and recommendations"

        echo "$prompt" | timeout 60 opencode || print_warning "Security docs update timed out or failed"
    fi
}

# Main function
main() {
    print_status "Starting automatic documentation update..."

    # Check prerequisites
    check_opencode

    # Initialize opencode if needed
    initialize_opencode

    # Update different types of documentation
    update_readme
    update_api_docs
    update_performance_docs
    update_security_docs

    print_success "Documentation update completed!"
    print_status "Please review the changes before committing."
}

# Run main function
main "$@"