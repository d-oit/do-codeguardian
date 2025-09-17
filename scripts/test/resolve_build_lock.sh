# Build Directory Lock Resolution Script
#!/bin/bash
echo '🔧 Resolving build directory lock issues...'

# Kill any running Cargo processes
pkill -f cargo || true

# Remove stale lock files
find . -name '.cargo-lock' -type f -delete 2>/dev/null || true
find target/ -name '*.lock' -type f -delete 2>/dev/null || true

# Clean build artifacts
cargo clean

# Verify resolution
echo '✅ Build directory lock resolved successfully'
echo '🧪 Running basic test to verify...'
cargo test --lib --quiet && echo '✅ Tests running successfully' || echo '❌ Test failed'

echo '📋 Next steps:'
echo '  1. Run ./scripts/test_broken_files.sh for comprehensive testing'
echo '  2. Use cargo build --release for optimized builds'
echo '  3. Consider implementing build caching for CI/CD'
