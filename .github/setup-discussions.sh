#!/bin/bash

# CodeGuardian GitHub Discussions Setup Script
# This script sets up discussion categories for the CodeGuardian repository

set -e

# Repository details
OWNER="d-oit"
REPO="do-codeguardian"

echo "Setting up GitHub Discussions categories for $OWNER/$REPO..."

# Function to create discussion category
create_category() {
    local name="$1"
    local description="$2"

    echo "Creating category: $name"
    gh api repos/$OWNER/$REPO/discussion-categories \
        --method POST \
        -f name="$name" \
        -f description="$description"
}

# Enable discussions if not already enabled
echo "Ensuring discussions are enabled..."
gh repo edit --enable-discussions

# Create categories
create_category "General" "General discussions about CodeGuardian, its development, and related topics"
create_category "Feature Requests" "Suggest new features, enhancements, or improvements to CodeGuardian"
create_category "Performance" "Discuss performance optimizations, benchmarks, and related topics"
create_category "Q&A" "Ask questions and get answers from the community and maintainers"
create_category "Show and Tell" "Share your CodeGuardian projects, integrations, or achievements"
create_category "Help" "Get help with using or configuring CodeGuardian"

echo "Discussion categories setup complete!"
echo "You can now use the discussion templates in .github/DISCUSSION_TEMPLATE/"
