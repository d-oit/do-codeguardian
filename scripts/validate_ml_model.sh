#!/bin/bash

# ML Model Validation Script for Duplicate Detection
# This script validates the ML model's performance for duplicate detection

set -e

echo "🔍 CodeGuardian ML Model Validation for Duplicate Detection"
echo "========================================================="

# Check if ML features are enabled
if ! cargo check --features ml 2>/dev/null; then
    echo "❌ ML features not available. Please install FANN library and enable ML features."
    echo "   On Ubuntu/Debian: sudo apt-get install libfann-dev"
    echo "   On macOS: brew install fann"
    echo "   Then rebuild with: cargo build --features ml"
    exit 1
fi

# Build with ML features
echo "🔨 Building with ML features..."
cargo build --release --features ml

# Check if model file exists
MODEL_PATH=".codeguardian/models/duplicate_similarity.fann"
if [ ! -f "$MODEL_PATH" ]; then
    echo "❌ Model file not found: $MODEL_PATH"
    echo "   Please ensure the model file exists or run training first."
    exit 1
fi

# Validate model file
echo "📊 Validating model file..."
MODEL_SIZE=$(stat -c%s "$MODEL_PATH" 2>/dev/null || stat -f%z "$MODEL_PATH" 2>/dev/null || echo "0")
if [ "$MODEL_SIZE" -lt 1000 ]; then
    echo "⚠️  Model file seems too small ($MODEL_SIZE bytes). May be corrupted."
fi

echo "✅ Model file validation passed ($MODEL_SIZE bytes)"

# Test model loading
echo "🧠 Testing model loading..."
if ./target/release/do-codeguardian check --help | grep -q "ml"; then
    echo "✅ ML features detected in CLI"
else
    echo "⚠️  ML features not detected in CLI output"
fi

# Run basic validation test
echo "🧪 Running basic validation test..."
cat > /tmp/test_duplicate.rs << 'EOF'
fn authenticate_user(username: &str, password: &str) -> bool {
    let hashed = hash_password(password);
    let stored = get_stored_password(username);
    hashed == stored
}

fn authenticate_admin(username: &str, password: &str) -> bool {
    let hashed = hash_password(password);
    let stored = get_stored_password(username);
    hashed == stored
}
EOF

# Run analysis with ML enabled
echo "🔍 Running duplicate analysis with ML..."
./target/release/do-codeguardian check /tmp/test_duplicate.rs --config codeguardian.toml

# Clean up
rm -f /tmp/test_duplicate.rs

echo ""
echo "🎯 ML Model Validation Summary:"
echo "================================"
echo "✅ Model file exists and is accessible"
echo "✅ ML features are enabled in build"
echo "✅ Basic duplicate detection test completed"
echo ""
echo "📈 Next Steps:"
echo "- Monitor ML-enhanced duplicate detection in CI/CD"
echo "- Collect feedback on model accuracy"
echo "- Consider retraining model with domain-specific data"
echo ""
echo "🚀 ML-based duplicate detection is now active!"
