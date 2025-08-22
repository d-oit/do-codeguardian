#!/bin/bash

# CodeGuardian Opencode Setup Script
# This script helps configure opencode for the project

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
        print_error "opencode is not installed"
        print_error "Please install it first: curl -fsSL https://opencode.ai/install | bash"
        exit 1
    fi
    print_success "opencode found: $(opencode --version)"
}

# Setup opencode authentication
setup_auth() {
    print_status "Setting up opencode authentication..."
    print_warning "You may need to configure your API keys for LLM providers"
    print_status "Run: opencode auth login"
    print_status "Then select your preferred provider (Anthropic recommended)"

    read -p "Have you configured opencode authentication? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "Please run 'opencode auth login' manually before using documentation updates"
        return 1
    fi
    return 0
}

# Initialize opencode for the project
initialize_project() {
    print_status "Initializing opencode for CodeGuardian project..."

    if [ -f "AGENTS.md" ]; then
        print_warning "Project already initialized (AGENTS.md exists)"
        read -p "Re-initialize? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            return 0
        fi
    fi

    print_status "Starting opencode initialization..."
    print_status "This will analyze your codebase and create AGENTS.md"
    print_warning "This process may take a few minutes..."

    # Create a temporary script for opencode initialization
    cat > /tmp/opencode_init.txt << 'EOF'
/init
Please analyze this Rust security analysis project and set up appropriate agents for:
1. Security analysis and vulnerability detection
2. Performance optimization and monitoring
3. Code quality and best practices
4. Documentation generation and updates
5. Dependency analysis and security

Focus on the Rust ecosystem, security-first principles, and the existing project structure.
EOF

    # Run opencode with the initialization script
    if timeout 120 opencode < /tmp/opencode_init.txt; then
        print_success "opencode initialization completed"
        rm -f /tmp/opencode_init.txt
    else
        print_error "opencode initialization failed or timed out"
        rm -f /tmp/opencode_init.txt
        return 1
    fi
}

# Test the documentation update process
test_docs_update() {
    print_status "Testing documentation update process..."

    if bash scripts/update-docs.sh; then
        print_success "Documentation update test completed successfully"
    else
        print_error "Documentation update test failed"
        return 1
    fi
}

# Main setup function
main() {
    print_status "Setting up opencode for CodeGuardian..."

    # Check prerequisites
    check_opencode

    # Setup authentication
    if ! setup_auth; then
        print_warning "Authentication setup skipped"
    fi

    # Initialize project
    if initialize_project; then
        print_success "Project initialization completed"
    else
        print_error "Project initialization failed"
        exit 1
    fi

    # Test documentation update
    if test_docs_update; then
        print_success "Setup completed successfully!"
        print_status "You can now use automatic documentation updates with git commits"
        print_status "The pre-commit hook will automatically update documentation before each commit"
    else
        print_warning "Setup completed with warnings"
        print_warning "You may need to run 'bash scripts/update-docs.sh' manually for testing"
    fi
}

# Show usage if requested
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "CodeGuardian Opencode Setup Script"
    echo ""
    echo "This script helps configure opencode for automatic documentation updates."
    echo ""
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  --help, -h    Show this help message"
    echo "  --test        Only run the test phase"
    echo ""
    echo "The setup process includes:"
    echo "1. Authentication configuration"
    echo "2. Project initialization with opencode"
    echo "3. Testing the documentation update process"
    exit 0
fi

# Run main function
main "$@"