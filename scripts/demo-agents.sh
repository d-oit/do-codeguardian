#!/bin/bash

# CodeGuardian Agent System Demo
# Demonstrates how to use the agent system in practice

set -e

# Colors for output
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[DEMO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

echo "🚀 CodeGuardian Agent System Demo"
echo "=================================="
echo

print_status "This demo shows how to use your 23 specialized agents"
echo

echo "📋 Available Agent Categories:"
echo "=============================="
echo
echo "🔧 Development & Code Quality:"
echo "   • codeguardian-main (primary agent)"
echo "   • clean-code-developer"
echo "   • code-consolidator"
echo "   • code-research"
echo

echo "🔒 Security & Performance:"
echo "   • security-reviewer"
echo "   • performance-optimizer"
echo "   • benchmark-agent"
echo

echo "📚 Documentation & Testing:"
echo "   • documentation-agent"
echo "   • testing-agent"
echo

echo "🚀 DevOps & GitHub:"
echo "   • github-pr-manager"
echo "   • github-issue-manager"
echo "   • github-workflow-manager"
echo "   • release-agent"
echo

echo "🤖 ML & Advanced:"
echo "   • ml-training-agent"
echo "   • orchestrator"
echo "   • swarm-orchestrator"
echo

echo "💡 Usage Examples:"
echo "=================="
echo

print_status "1. Start opencode and switch to main agent:"
echo "   opencode"
echo "   /agent codeguardian-main"
echo

print_status "2. Use specialized agents with @ mentions:"
echo "   @documentation-agent Generate API docs for this function"
echo "   @security-reviewer Review this code for vulnerabilities"
echo "   @performance-optimizer Optimize this slow function"
echo "   @testing-agent Generate tests for this module"
echo

print_status "3. List all available agents:"
echo "   /agents"
echo

print_status "4. Get help with agents:"
echo "   /help agents"
echo

echo "🎯 Real-World Scenarios:"
echo "======================="
echo

print_status "Scenario 1: Code Review"
echo "   1. Switch to main agent: /agent codeguardian-main"
echo "   2. Ask for review: 'Review this Rust function for security issues'"
echo "   3. Use security agent: @security-reviewer Deep dive on this code"
echo

print_status "Scenario 2: Documentation"
echo "   1. Use doc agent: @documentation-agent Update README with new features"
echo "   2. Generate API docs: @documentation-agent Create docs for this module"
echo

print_status "Scenario 3: Performance Optimization"
echo "   1. Run benchmarks: @benchmark-agent Benchmark this function"
echo "   2. Optimize: @performance-optimizer Improve this slow code"
echo

print_status "Scenario 4: Testing"
echo "   1. Generate tests: @testing-agent Create comprehensive tests"
echo "   2. Review coverage: @testing-agent Analyze test coverage gaps"
echo

echo "⚙️  Agent Configuration:"
echo "======================"
echo
echo "Each agent is configured in: .opencode/agent/[agent-name].md"
echo "Configuration includes:"
echo "   • Description and purpose"
echo "   • Mode (primary/subagent)"
echo "   • Model and temperature settings"
echo "   • Available tools and permissions"
echo "   • Specialized prompts and knowledge"
echo

print_success "🎉 Your agent system is ready for production use!"
echo
print_status "Next steps:"
echo "   1. Run: opencode"
echo "   2. Try: /agents (to see all agents)"
echo "   3. Try: @documentation-agent Hello! (test a specialized agent)"
echo "   4. Explore different agents for your specific needs"
echo

print_warning "💡 Pro Tips:"
echo "   • Use the main agent (codeguardian-main) for general development"
echo "   • Switch to specialized agents for specific tasks"
echo "   • Combine agents: Use one agent to analyze, another to implement"
echo "   • The system learns from your interactions over time"