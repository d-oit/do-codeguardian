#!/bin/bash
# Test build script for CodeGuardian with GitHub Discussions Manager

echo "🔨 Building CodeGuardian with GitHub Discussions Manager..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "🚀 You can now use the GitHub Discussions Manager with:"
    echo "codeguardian gh-discussions --help"
    echo ""
    echo "📖 Available commands:"
    echo "• Monitor discussions: codeguardian gh-discussions <repo> monitor"
    echo "• Create discussion: codeguardian gh-discussions <repo> create --title '...' --body '...' --category '...' "
    echo "• Respond to discussion: codeguardian gh-discussions <repo> respond --discussion-number <num> --response '...' "
    echo "• Moderate discussion: codeguardian gh-discussions <repo> moderate --discussion-number <num> <action>"
    echo "• Analyze discussions: codeguardian gh-discussions <repo> analyze --days 30"
else
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi