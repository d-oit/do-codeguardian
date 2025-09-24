#!/bin/bash

# CodeGuardian Slack Notification Script
# Usage: ./send-slack-notification.sh <template> <webhook_url> [channel] [variables...]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMPLATE_DIR="$SCRIPT_DIR/../.github/notification-templates"

# Function to display usage
usage() {
    echo "Usage: $0 <template> <webhook_url> [channel] [key1=value1] [key2=value2] ..."
    echo ""
    echo "Templates:"
    echo "  ci-success          - CI/CD pipeline success notification"
    echo "  ci-failure          - CI/CD pipeline failure notification"
    echo "  deployment-success  - Deployment success notification"
    echo "  deployment-failure  - Deployment failure notification"
    echo ""
    echo "Examples:"
    echo "  $0 ci-success https://hooks.slack.com/... '#devops' workflow_name='CI Pipeline' branch='main'"
    echo "  $0 deployment-success https://hooks.slack.com/... '#deployments' version='v1.2.3' environment='production'"
    exit 1
}

# Check arguments
if [ $# -lt 2 ]; then
    usage
fi

TEMPLATE="$1"
WEBHOOK_URL="$2"
CHANNEL="${3:-#ci-cd}"

# Shift arguments
shift 2
if [ "$CHANNEL" != "#ci-cd" ]; then
    shift 1
fi

# Parse additional variables
declare -A VARIABLES
for arg in "$@"; do
    if [[ "$arg" == *"="* ]]; then
        key="${arg%%=*}"
        value="${arg#*=}"
        VARIABLES["$key"]="$value"
    fi
done

# Set default variables
VARIABLES["channel"]="$CHANNEL"
VARIABLES["timestamp"]="$(date +%s)"

# Check if template exists
TEMPLATE_FILE="$TEMPLATE_DIR/$TEMPLATE.json"
if [ ! -f "$TEMPLATE_FILE" ]; then
    echo "Error: Template '$TEMPLATE' not found in $TEMPLATE_DIR"
    exit 1
fi

# Read template
TEMPLATE_CONTENT=$(cat "$TEMPLATE_FILE")

# Replace variables in template
for key in "${!VARIABLES[@]}"; do
    value="${VARIABLES[$key]}"
    # Escape special characters for JSON
    value=$(echo "$value" | sed 's/\\/\\\\/g' | sed 's/"/\\"/g' | sed 's/\n/\\n/g')
    TEMPLATE_CONTENT=$(echo "$TEMPLATE_CONTENT" | sed "s/{{$key}}/$value/g")
done

# Replace any remaining template variables with empty strings
TEMPLATE_CONTENT=$(echo "$TEMPLATE_CONTENT" | sed 's/{{[^}]*}}//g')

# Validate JSON
if ! echo "$TEMPLATE_CONTENT" | jq . >/dev/null 2>&1; then
    echo "Error: Invalid JSON generated from template"
    echo "Generated JSON:"
    echo "$TEMPLATE_CONTENT"
    exit 1
fi

# Send notification
echo "Sending Slack notification using template: $TEMPLATE"
curl -X POST -H 'Content-type: application/json' --data "$TEMPLATE_CONTENT" "$WEBHOOK_URL"

echo "Notification sent successfully"
