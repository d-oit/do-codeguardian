#!/bin/bash

# CodeGuardian Agent Information Script
# Display information about available opencode agents

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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

echo "🤖 CodeGuardian Agent System Overview"
echo "====================================="
echo

print_status "Agent Directory Structure:"
echo "📁 .opencode/agent/ (contains $(ls .opencode/agent/*.md | wc -l) specialized agents)"
echo

print_status "Available Agents:"
echo

# List all agents with descriptions
for agent_file in .opencode/agent/*.md; do
    agent_name=$(basename "$agent_file" .md)
    description=$(grep -A 2 "description:" "$agent_file" | head -3 | sed 's/description: //' | sed 's/^  //' | tr '\n' ' ' | sed 's/>-//g' | cut -c1-80)
    echo "  🔹 $agent_name"
    echo "     $description..."
    echo
done

echo "📋 Agent Usage Examples:"
echo "======================="
echo
echo "1. Switch to main development agent:"
echo "   /agent codeguardian-main"
echo
echo "2. Use specialized agents:"
echo "   @documentation-agent Generate API docs for this function"
echo "   @security-reviewer Review this code for vulnerabilities"
echo "   @performance-optimizer Optimize this slow function"
echo
echo "3. List all available agents:"
echo "   /agents"
echo
echo "4. Get help with agents:"
echo "   /help agents"
echo

print_status "Current Configuration:"
echo "📄 .opencode/config.json - Main agent configuration"
echo "📄 .agent.md - Legacy comprehensive agent guide"
echo "📄 AGENTS.md - Opencode-generated project guide"
echo

print_success "Agent system is properly configured and ready to use!"
echo
print_warning "Note: The .opencode/agents/ directory was removed to avoid conflicts with the existing .opencode/agent/ structure."