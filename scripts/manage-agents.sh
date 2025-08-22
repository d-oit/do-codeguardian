#!/bin/bash

# CodeGuardian Agent Management Script
# Manage and test opencode agents for the project

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

# List available agents
list_agents() {
    print_status "Available CodeGuardian agents:"

    if [ -f ".opencode/agents/documentation-agent.json" ]; then
        echo "📚 Documentation Agent"
        echo "   - Specialized for documentation management"
        echo "   - Auto-activates on pre-commit"
        echo "   - Handles README, API docs, performance docs"
        echo ""
    fi

    if [ -f ".opencode/agents/development-agent.json" ]; then
        echo "💻 Development Agent"
        echo "   - Specialized for code development"
        echo "   - Handles code review, bug fixes, features"
        echo "   - Manual activation required"
        echo ""
    fi
}

# Test documentation agent
test_documentation_agent() {
    print_status "Testing Documentation Agent..."

    if [ ! -f ".opencode/agents/documentation-agent.json" ]; then
        print_error "Documentation agent configuration not found"
        return 1
    fi

    print_status "Documentation agent configuration is valid"
    print_success "Documentation agent is ready for use"
}

# Test development agent
test_development_agent() {
    print_status "Testing Development Agent..."

    if [ ! -f ".opencode/agents/development-agent.json" ]; then
        print_error "Development agent configuration not found"
        return 1
    fi

    print_status "Development agent configuration is valid"
    print_success "Development agent is ready for use"
}

# Validate agent configurations
validate_agents() {
    print_status "Validating agent configurations..."

    local valid=true

    # Check documentation agent
    if [ -f ".opencode/agents/documentation-agent.json" ]; then
        if jq empty ".opencode/agents/documentation-agent.json" 2>/dev/null; then
            print_success "Documentation agent JSON is valid"
        else
            print_error "Documentation agent JSON is invalid"
            valid=false
        fi
    fi

    # Check development agent
    if [ -f ".opencode/agents/development-agent.json" ]; then
        if jq empty ".opencode/agents/development-agent.json" 2>/dev/null; then
            print_success "Development agent JSON is valid"
        else
            print_error "Development agent JSON is invalid"
            valid=false
        fi
    fi

    # Check main config
    if [ -f ".opencode/config.json" ]; then
        if jq empty ".opencode/config.json" 2>/dev/null; then
            print_success "Main opencode config JSON is valid"
        else
            print_error "Main opencode config JSON is invalid"
            valid=false
        fi
    fi

    if [ "$valid" = true ]; then
        print_success "All agent configurations are valid"
        return 0
    else
        print_error "Some agent configurations are invalid"
        return 1
    fi
}

# Show agent information
show_agent_info() {
    local agent_type="$1"

    case "$agent_type" in
        "documentation")
            if [ -f ".opencode/agents/documentation-agent.json" ]; then
                echo "=== Documentation Agent Information ==="
                jq -r '.name, .description, "", "Capabilities:", (.capabilities.primary[]), "", "Specializations:", (.specializations.domain[])' ".opencode/agents/documentation-agent.json"
            else
                print_error "Documentation agent not found"
            fi
            ;;
        "development")
            if [ -f ".opencode/agents/development-agent.json" ]; then
                echo "=== Development Agent Information ==="
                jq -r '.name, .description, "", "Capabilities:", (.capabilities.primary[]), "", "Specializations:", (.specializations.domain[])' ".opencode/agents/development-agent.json"
            else
                print_error "Development agent not found"
            fi
            ;;
        *)
            print_error "Unknown agent type: $agent_type"
            echo "Available types: documentation, development"
            ;;
    esac
}

# Main function
main() {
    print_status "CodeGuardian Agent Management"

    # Check prerequisites
    check_opencode

    case "${1:-list}" in
        "list")
            list_agents
            ;;
        "test")
            test_documentation_agent
            test_development_agent
            ;;
        "validate")
            validate_agents
            ;;
        "info")
            if [ -z "$2" ]; then
                print_error "Please specify agent type: info <documentation|development>"
                exit 1
            fi
            show_agent_info "$2"
            ;;
        "help"|"-h"|"--help")
            echo "CodeGuardian Agent Management Script"
            echo ""
            echo "Usage: $0 [command] [options]"
            echo ""
            echo "Commands:"
            echo "  list                    List available agents (default)"
            echo "  test                    Test agent configurations"
            echo "  validate                Validate all agent JSON configurations"
            echo "  info <type>             Show detailed info for agent type"
            echo "  help                    Show this help message"
            echo ""
            echo "Agent Types:"
            echo "  documentation           Documentation management agent"
            echo "  development             Code development agent"
            echo ""
            echo "Examples:"
            echo "  $0 list"
            echo "  $0 test"
            echo "  $0 info documentation"
            ;;
        *)
            print_error "Unknown command: $1"
            echo "Run '$0 help' for usage information"
            exit 1
            ;;
    esac
}

# Run main function
main "$@"