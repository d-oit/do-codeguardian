#!/bin/bash

# Test script for the CodeGuardian agent system
# Demonstrates agent functionality and configuration

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

echo "🧪 CodeGuardian Agent System Test"
echo "================================="
echo

# Test 1: Check opencode installation
print_status "Test 1: Checking opencode installation..."
if command -v opencode &> /dev/null; then
    VERSION=$(opencode --version)
    print_success "✅ opencode $VERSION is installed"
else
    print_error "❌ opencode not found"
    exit 1
fi

# Test 2: Check agent directory structure
print_status "Test 2: Checking agent directory structure..."
AGENT_COUNT=$(find .opencode/agent/ -name "*.md" | wc -l)
if [ "$AGENT_COUNT" -eq 23 ]; then
    print_success "✅ Found $AGENT_COUNT agent files"
else
    print_error "❌ Expected 23 agents, found $AGENT_COUNT"
fi

# Test 3: Check main agent configuration
print_status "Test 3: Checking main agent configuration..."
if [ -f ".opencode/agent/codeguardian-main.md" ]; then
    if grep -q "mode: primary" .opencode/agent/codeguardian-main.md; then
        print_success "✅ Main agent is properly configured as primary"
    else
        print_error "❌ Main agent missing primary mode"
    fi
else
    print_error "❌ Main agent file not found"
fi

# Test 4: Check specialized agents
print_status "Test 4: Checking specialized agents..."
SPECIALIZED_AGENTS=(
    "documentation-agent.md"
    "security-reviewer.md"
    "performance-optimizer.md"
    "testing-agent.md"
)

for agent in "${SPECIALIZED_AGENTS[@]}"; do
    if [ -f ".opencode/agent/$agent" ]; then
        print_success "✅ $agent found"
    else
        print_error "❌ $agent missing"
    fi
done

# Test 5: Check agent file format
print_status "Test 5: Checking agent file formats..."
INVALID_COUNT=0
for agent_file in .opencode/agent/*.md; do
    if ! head -1 "$agent_file" | grep -q "^---$"; then
        print_error "❌ $agent_file missing YAML frontmatter"
        INVALID_COUNT=$((INVALID_COUNT + 1))
    fi
done

if [ "$INVALID_COUNT" -eq 0 ]; then
    print_success "✅ All agent files have proper YAML frontmatter"
else
    print_error "❌ $INVALID_COUNT agent files have format issues"
fi

# Test 6: Check opencode configuration
print_status "Test 6: Checking opencode configuration..."
if [ -f ".opencode/config.json" ]; then
    if jq empty .opencode/config.json 2>/dev/null; then
        print_success "✅ Main configuration is valid JSON"
    else
        print_error "❌ Main configuration has JSON syntax errors"
    fi
else
    print_error "❌ Main configuration file not found"
fi

echo
echo "📊 Test Results Summary:"
echo "======================"

# Count successful tests
SUCCESS_COUNT=0
if command -v opencode &> /dev/null; then ((SUCCESS_COUNT++)); fi
if [ "$AGENT_COUNT" -eq 23 ]; then ((SUCCESS_COUNT++)); fi
if [ -f ".opencode/agent/codeguardian-main.md" ] && grep -q "mode: primary" .opencode/agent/codeguardian-main.md; then ((SUCCESS_COUNT++)); fi

SPECIALIZED_FOUND=0
for agent in "${SPECIALIZED_AGENTS[@]}"; do
    if [ -f ".opencode/agent/$agent" ]; then ((SPECIALIZED_FOUND++)); fi
done
if [ "$SPECIALIZED_FOUND" -eq 4 ]; then ((SUCCESS_COUNT++)); fi

if [ "$INVALID_COUNT" -eq 0 ]; then ((SUCCESS_COUNT++)); fi
if [ -f ".opencode/config.json" ] && jq empty .opencode/config.json 2>/dev/null; then ((SUCCESS_COUNT++)); fi

TOTAL_TESTS=6

if [ "$SUCCESS_COUNT" -eq "$TOTAL_TESTS" ]; then
    print_success "🎉 All $TOTAL_TESTS tests passed! Agent system is working correctly."
    echo
    print_status "You can now use the agent system with commands like:"
    echo "  • /agent codeguardian-main (switch to main agent)"
    echo "  • @documentation-agent Generate docs for this function"
    echo "  • @security-reviewer Review this code"
    echo "  • /agents (list all available agents)"
else
    print_warning "⚠️  $SUCCESS_COUNT/$TOTAL_TESTS tests passed"
    print_status "Some issues found - check the output above"
fi

echo
print_status "To use the agent system interactively:"
echo "  1. Run: opencode"
echo "  2. Try: /agents (to see all agents)"
echo "  3. Try: @documentation-agent Hello! (to test a specialized agent)"