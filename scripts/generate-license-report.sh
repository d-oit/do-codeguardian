#!/bin/bash
# Generate comprehensive license compliance report

set -e

echo "# License Compliance Report"
echo "Generated on: $(date)"
echo "Repository: $GITHUB_REPOSITORY"
echo "Commit: $GITHUB_SHA"
echo ""

# Check if cargo-deny is available
if ! command -v cargo-deny &> /dev/null; then
    echo "Installing cargo-deny..."
    cargo install cargo-deny --locked
fi

echo "## License Summary"
echo ""

# Generate license list
echo "### Dependencies by License"
cargo deny list | head -30
echo ""

echo "## Compliance Check Results"
echo ""

# Run license check
if cargo deny check licenses --format json > license-results.json 2>&1; then
    echo "✅ **Status**: All dependencies are license compliant"
    echo ""

    # Parse JSON for summary
    if command -v jq &> /dev/null; then
        echo "### License Distribution"
        jq -r '.licenses | group_by(.license) | map({license: .[0].license, count: length}) | sort_by(.count) | reverse | .[] | "- \(.license): \(.count) dependencies"' license-results.json 2>/dev/null || echo "Could not parse license distribution"
        echo ""
    fi
else
    echo "❌ **Status**: License compliance violations detected"
    echo ""
    echo "### Violations"
    cat license-results.json | grep -o '"message":"[^"]*"' | sed 's/"message":"//g' | sed 's/"//g' | head -10
    echo ""
fi

echo "## Configuration"
echo ""
echo "License policy configured in \`deny.toml\`:"
echo "- **Allowed Licenses**: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, and others"
echo "- **Denied Licenses**: GPL-3.0, LGPL-3.0, AGPL-3.0, MS-PL, and others"
echo "- **Unlicensed crates**: Denied"
echo ""

echo "## Recommendations"
echo ""
if [ -f "license-results.json" ] && grep -q "error" license-results.json; then
    echo "⚠️  **Action Required**: Review and resolve license compliance issues"
    echo "- Check deny.toml configuration"
    echo "- Consider alternative dependencies with compliant licenses"
    echo "- Contact maintainers for license clarification if needed"
else
    echo "✅ **No Action Required**: All dependencies are compliant"
fi
