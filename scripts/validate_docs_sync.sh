#!/bin/bash

# validate_docs_sync.sh
# Validates documentation synchronization with codebase
# Checks CLI options, config examples, and cross-references

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Validate TOML syntax
validate_toml() {
    local file="$1"
    if command_exists python3; then
        if ! python3 -c "import toml; toml.load('$file')" 2>/dev/null; then
            log_error "Invalid TOML syntax in $file"
            return 1
        fi
    else
        log_warn "python3 not found, skipping TOML validation for $file"
    fi
    return 0
}

# Validate config examples
validate_config_examples() {
    log_info "Validating configuration examples..."

    local config_dir="$PROJECT_ROOT/examples/config"
    local errors=0

    if [ ! -d "$config_dir" ]; then
        log_error "Configuration examples directory not found: $config_dir"
        return 1
    fi

    for toml_file in "$config_dir"/*.toml; do
        if [ -f "$toml_file" ]; then
            log_info "Validating $toml_file"
            if ! validate_toml "$toml_file"; then
                ((errors++))
            fi

            # Try to load with codeguardian if available
            if [ -f "$PROJECT_ROOT/target/debug/codeguardian" ]; then
                if ! "$PROJECT_ROOT/target/debug/codeguardian" check --config "$toml_file" --dry-run --quiet 2>/dev/null; then
                    log_error "Configuration validation failed for $toml_file"
                    ((errors++))
                fi
            fi
        fi
    done

    return $errors
}

# Extract CLI help
extract_cli_help() {
    local help_file="$1"
    if [ -f "$PROJECT_ROOT/target/debug/codeguardian" ]; then
        "$PROJECT_ROOT/target/debug/codeguardian" --help > "$help_file" 2>&1
        return $?
    else
        log_error "codeguardian binary not found. Run 'cargo build' first."
        return 1
    fi
}

# Extract command help
extract_command_help() {
    local command="$1"
    local help_file="$2"
    if [ -f "$PROJECT_ROOT/target/debug/codeguardian" ]; then
        "$PROJECT_ROOT/target/debug/codeguardian" "$command" --help > "$help_file" 2>&1
        return $?
    else
        log_error "codeguardian binary not found. Run 'cargo build' first."
        return 1
    fi
}

# Check CLI documentation sync
check_cli_docs_sync() {
    log_info "Checking CLI documentation synchronization..."

    local temp_dir=$(mktemp -d)
    local errors=0

    # Extract current CLI help
    local main_help="$temp_dir/main_help.txt"
    if ! extract_cli_help "$main_help"; then
        log_error "Failed to extract main CLI help"
        rm -rf "$temp_dir"
        return 1
    fi

    # Check main CLI docs
    local cli_ref="$PROJECT_ROOT/docs/cli-reference.md"
    if [ -f "$cli_ref" ]; then
        # Extract commands from help
        local commands_in_help=$(grep -E "^\s*[a-zA-Z-]+ " "$main_help" | sed 's/^\s*//' | cut -d' ' -f1 | sort | uniq)

        # Check if commands are documented
        for cmd in $commands_in_help; do
            if ! grep -q "$cmd" "$cli_ref"; then
                log_warn "Command '$cmd' not found in CLI reference documentation"
                ((errors++))
            fi
        done
    else
        log_error "CLI reference documentation not found: $cli_ref"
        ((errors++))
    fi

    # Check command-specific help
    local commands=("check" "report" "gh-issue" "init" "git-commit" "turbo")
    for cmd in "${commands[@]}"; do
        local cmd_help="$temp_dir/${cmd}_help.txt"
        if extract_command_help "$cmd" "$cmd_help" 2>/dev/null; then
            # Check if command has user guide documentation
            local user_guide="$PROJECT_ROOT/docs/user-guide/$cmd.md"
            if [ ! -f "$user_guide" ]; then
                log_warn "User guide not found for command: $cmd"
                ((errors++))
            fi
        fi
    done

    rm -rf "$temp_dir"
    return $errors
}

# Check config documentation sync
check_config_docs_sync() {
    log_info "Checking configuration documentation synchronization..."

    local errors=0

    # Check if config docs exist
    local config_basics="$PROJECT_ROOT/docs/configuration-basics.md"
    local config_advanced="$PROJECT_ROOT/docs/configuration-advanced.md"

    if [ ! -f "$config_basics" ]; then
        log_error "Configuration basics documentation not found: $config_basics"
        ((errors++))
    fi

    if [ ! -f "$config_advanced" ]; then
        log_error "Configuration advanced documentation not found: $config_advanced"
        ((errors++))
    fi

    # Check for outdated defaults (this would require parsing the code, simplified check)
    if [ -f "$config_basics" ]; then
        # Check if common defaults are mentioned
        local common_defaults=("max_file_size_bytes" "timeout_seconds" "max_workers")
        for default in "${common_defaults[@]}"; do
            if ! grep -q "$default" "$config_basics"; then
                log_warn "Default value for '$default' not documented in configuration basics"
                ((errors++))
            fi
        done
    fi

    return $errors
}

# Check cross-references
check_cross_references() {
    log_info "Checking documentation cross-references..."

    local errors=0
    local docs_dir="$PROJECT_ROOT/docs"

    # Find all markdown files
    local md_files=$(find "$docs_dir" -name "*.md" -type f)

    for md_file in $md_files; do
        # Check for broken relative links
        local broken_links=$(grep -n '\[.*\](\.\./.*\.md)' "$md_file" | while read -r line; do
            local link=$(echo "$line" | grep -o '\.\./.*\.md')
            local target="$docs_dir/$(echo "$link" | sed 's|\.\./||')"
            if [ ! -f "$target" ]; then
                echo "$md_file:$line - $link"
            fi
        done)

        if [ -n "$broken_links" ]; then
            log_error "Broken cross-references in $md_file:"
            echo "$broken_links"
            ((errors++))
        fi
    done

    return $errors
}

# Main validation function
main() {
    log_info "Starting documentation synchronization validation..."

    local total_errors=0

    # Build the project first
    log_info "Building project..."
    if ! (cd "$PROJECT_ROOT" && cargo build --quiet); then
        log_error "Failed to build project"
        exit 1
    fi

    # Run validations
    validate_config_examples
    ((total_errors += $?))

    check_cli_docs_sync
    ((total_errors += $?))

    check_config_docs_sync
    ((total_errors += $?))

    check_cross_references
    ((total_errors += $?))

    # Summary
    if [ $total_errors -eq 0 ]; then
        log_info "All documentation synchronization checks passed!"
        exit 0
    else
        log_error "Found $total_errors documentation synchronization issues"
        exit 1
    fi
}

# Run main if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi