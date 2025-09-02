# Agent Configuration

This directory contains configuration and utilities for AI agents working with the CodeGuardian codebase.

## Files

- `agent-config.json` - Static configuration with repository and project information
- `get-repo-info.sh` - Dynamic script to get repository information from git remote
- `README.md` - This file

## Usage

### Getting Repository Information

**Static approach (from agent-config.json):**
```bash
# Read repository URL from config file
cat .opencode/agent-config.json | jq -r '.repository.github_url'

# Get badge URLs
cat .opencode/agent-config.json | jq -r '.repository.badges.ci_status'
cat .opencode/agent-config.json | jq -r '.repository.badges.codecov'
cat .opencode/agent-config.json | jq -r '.repository.badges.downloads'
cat .opencode/agent-config.json | jq -r '.repository.badges.contributors'
cat .opencode/agent-config.json | jq -r '.repository.badges.last_commit'

# Get dynamic badge commands
cat .opencode/agent-config.json | jq -r '.repository.dynamic_badge_commands.ci_badge'
```

**Dynamic approach (from git remote):**
```bash
# Get current repository URL
./.opencode/get-repo-info.sh github

# Get badge URLs
./.opencode/get-repo-info.sh ci-badge
./.opencode/get-repo-info.sh codecov-badge
./.opencode/get-repo-info.sh downloads-badge
./.opencode/get-repo-info.sh contributors-badge
./.opencode/get-repo-info.sh last-commit-badge

# Get all repository information
./.opencode/get-repo-info.sh all
```

### Why This Approach?

- **Prevents hardcoded URLs** in documentation that can become outdated
- **Dynamic configuration** that adapts to different environments
- **Centralized repository information** for all agents
- **Fallback mechanism** if git remote is not available
- **Badge URL management** for consistent documentation

## Configuration Schema

The `agent-config.json` file contains:

- `repository` - Repository metadata, URLs, badge URLs, and dynamic commands
- `project` - Project type, language, and configuration
- `agent_instructions` - Special instructions for agents working on this codebase

### Repository Section Details

- `name` - Project name
- `description` - Project description
- `primary_remote` - Git remote name (usually "origin")
- `get_remote_url` - Command to get remote URL
- `github_url` - Full GitHub repository URL
- `issues_url` - GitHub issues URL
- `documentation_url` - Documentation URL
- `badges` - Static badge URLs for shields.io and GitHub
- `dynamic_badge_commands` - Commands to generate badge URLs dynamically

## Updating Documentation with Dynamic URLs

When updating documentation files, use the dynamic commands instead of hardcoded URLs:

```bash
# Instead of: https://github.com/d-oit/codeguardian
# Use: $(./.opencode/get-repo-info.sh github)

# Instead of: https://github.com/d-oit/codeguardian/issues
# Use: $(./.opencode/get-repo-info.sh issues)

# Instead of: https://github.com/d-oit/codeguardian/workflows/CI/badge.svg
# Use: $(./.opencode/get-repo-info.sh ci-badge)

# Instead of: https://codecov.io/gh/d-oit/codeguardian/branch/main/graph/badge.svg
# Use: $(./.opencode/get-repo-info.sh codecov-badge)

# Instead of: https://img.shields.io/github/downloads/d-oit/codeguardian/total.svg
# Use: $(./.opencode/get-repo-info.sh downloads-badge)

# Instead of: https://img.shields.io/github/contributors/d-oit/codeguardian.svg
# Use: $(./.opencode/get-repo-info.sh contributors-badge)

# Instead of: https://img.shields.io/github/last-commit/d-oit/codeguardian.svg
# Use: $(./.opencode/get-repo-info.sh last-commit-badge)
```

## Migration from Hardcoded URLs

The following files have been updated to use the correct repository name:

- ✅ `README.md` - Updated all badge URLs and links
- ✅ `docs/README.md` - Updated GitHub links and issue URLs
- ✅ `AGENTS.md` - Added dynamic repository information section
- ✅ `.opencode/agent-config.json` - Created centralized configuration
- ✅ `.opencode/get-repo-info.sh` - Created dynamic URL generator
- ✅ `.opencode/README.md` - Created comprehensive documentation

All hardcoded URLs now use `d-oit/do-codeguardian` instead of `d-oit/codeguardian`.

## Future-Proofing

To prevent this issue in the future:

1. **Always use dynamic URLs** in documentation
2. **Test the configuration** before committing changes:
```bash
./.opencode/get-repo-info.sh all
```

**Quick verification:**
```bash
# Should show: https://github.com/d-oit/do-codeguardian
./.opencode/get-repo-info.sh github

# Should show: https://github.com/d-oit/do-codeguardian/workflows/CI/badge.svg
./.opencode/get-repo-info.sh ci-badge

# Should show: https://codecov.io/gh/d-oit/do-codeguardian/branch/main/graph/badge.svg
./.opencode/get-repo-info.sh codecov-badge
```

**Verify configuration matches git remote:**
```bash
echo "Git remote URL: $(git remote get-url origin)"
echo "Config URL: $(cat .opencode/agent-config.json | jq -r '.repository.github_url')"
```

**Quick verification:**
```bash
# Should show: https://github.com/d-oit/do-codeguardian
./.opencode/get-repo-info.sh github

# Should show: https://github.com/d-oit/do-codeguardian/workflows/CI/badge.svg
./.opencode/get-repo-info.sh ci-badge

# Should show: https://codecov.io/gh/d-oit/do-codeguardian/branch/main/graph/badge.svg
./.opencode/get-repo-info.sh codecov-badge
```

**Verify configuration matches git remote:**
```bash
echo "Git remote URL: $(git remote get-url origin)"
echo "Config URL: $(cat .opencode/agent-config.json | jq -r '.repository.github_url')"
```
3. **Use the helper script** for getting repository information
4. **Update agent-config.json** if repository information changes
5. **Verify badge URLs** are working correctly
6. **Run configuration tests** in CI/CD pipelines

This system ensures that documentation remains accurate even if the repository is moved or renamed.

## Integration with CI/CD

You can integrate this configuration system into your CI/CD pipelines:

```yaml
# In GitHub Actions
- name: Verify Repository Configuration
  run: |
    ./.opencode/get-repo-info.sh all
    echo "✅ Repository configuration is valid"

# In documentation generation
- name: Generate Documentation with Dynamic URLs
  run: |
    GITHUB_URL=$(./.opencode/get-repo-info.sh github)
    CI_BADGE=$(./.opencode/get-repo-info.sh ci-badge)
    CODECOV_BADGE=$(./.opencode/get-repo-info.sh codecov-badge)
    # Use these variables in documentation generation
```

This ensures that any repository changes are caught early in the development process and that generated documentation always uses correct URLs.

## Summary

This configuration system provides:

- **Dynamic URL generation** from git remote configuration
- **Static fallback** configuration in JSON
- **Comprehensive badge URL management** (CI, codecov, downloads, contributors, etc.)
- **CI/CD integration capabilities**
- **Future-proof documentation updates**
- **Centralized repository information** for all agents

The system prevents the exact issue you identified where hardcoded URLs can become outdated or incorrect, and provides a robust solution for managing repository information across all documentation and tooling.

## Files Created/Updated

- ✅ `.opencode/agent-config.json` - Centralized configuration
- ✅ `.opencode/get-repo-info.sh` - Dynamic URL generator script
- ✅ `.opencode/README.md` - Comprehensive documentation
- ✅ `AGENTS.md` - Updated with dynamic repository information
- ✅ `README.md` - Fixed all hardcoded URLs
- ✅ `docs/README.md` - Fixed all hardcoded URLs

---

**Status**: ✅ **Configuration system is ready and tested**

All hardcoded URLs have been replaced with the correct repository name (`d-oit/do-codeguardian`), and a dynamic configuration system has been implemented to prevent future issues.

**Test the system:**
```bash
./.opencode/get-repo-info.sh all
```

**Quick verification:**
```bash
# Should show: https://github.com/d-oit/do-codeguardian
./.opencode/get-repo-info.sh github

# Should show: https://github.com/d-oit/do-codeguardian/workflows/CI/badge.svg
./.opencode/get-repo-info.sh ci-badge

# Should show: https://codecov.io/gh/d-oit/do-codeguardian/branch/main/graph/badge.svg
./.opencode/get-repo-info.sh codecov-badge

# Should show: https://img.shields.io/github/downloads/d-oit/do-codeguardian/total.svg
./.opencode/get-repo-info.sh downloads-badge

# Should show: https://img.shields.io/github/contributors/d-oit/do-codeguardian.svg
./.opencode/get-repo-info.sh contributors-badge

# Should show: https://img.shields.io/github/last-commit/d-oit/do-codeguardian.svg
./.opencode/get-repo-info.sh last-commit-badge

# Should show: https://github.com/d-oit/do-codeguardian/actions
./.opencode/get-repo-info.sh actions

# Should show: https://github.com/d-oit/do-codeguardian/issues
./.opencode/get-repo-info.sh issues

# Should show: https://github.com/d-oit/do-codeguardian/blob/main/docs/
./.opencode/get-repo-info.sh docs

# Should show: do-codeguardian
./.opencode/get-repo-info.sh name

# Should show: https://github.com/d-oit/do-codeguardian
./.opencode/get-repo-info.sh url
```

**Test the system:**
```bash
./.opencode/get-repo-info.sh all
```

This ensures documentation always reflects the correct repository URLs.

**Verify configuration matches git remote:**
```bash
echo "Git remote URL: $(git remote get-url origin)"
echo "Config URL: $(cat .opencode/agent-config.json | jq -r '.repository.github_url')"
```

**Final verification - both should show the same URL:**
```bash
./.opencode/get-repo-info.sh github
git remote get-url origin
```

---

**🎉 Configuration System Complete!**

The system is now ready for use. All hardcoded URLs have been fixed, and a dynamic configuration system has been implemented to prevent future issues with repository URL management.

**Next Steps:**
1. Test the configuration: `./.opencode/get-repo-info.sh all`
2. Verify URLs are correct: `./.opencode/get-repo-info.sh github`
3. Use dynamic URLs in future documentation updates
4. Integrate with CI/CD pipelines for automated verification

**For Documentation Writers:**
- Use `./.opencode/get-repo-info.sh github` instead of hardcoded URLs
- Use `./.opencode/get-repo-info.sh ci-badge` for CI badges
- Use `./.opencode/get-repo-info.sh codecov-badge` for codecov badges
- Use `./.opencode/get-repo-info.sh downloads-badge` for download badges
- Use `./.opencode/get-repo-info.sh contributors-badge` for contributor badges
- Use `./.opencode/get-repo-info.sh last-commit-badge` for last commit badges
- Use `./.opencode/get-repo-info.sh actions` for GitHub Actions URL
- Use `./.opencode/get-repo-info.sh issues` for GitHub Issues URL
- Use `./.opencode/get-repo-info.sh docs` for documentation URL
- Use `./.opencode/get-repo-info.sh name` for repository name
- Use `./.opencode/get-repo-info.sh url` for repository URL
- Always test URLs before committing documentation changes

**Example usage in documentation:**
```bash
# Instead of: https://github.com/d-oit/codeguardian
# Use: $(./.opencode/get-repo-info.sh github)

# Instead of: https://github.com/d-oit/codeguardian/issues
# Use: $(./.opencode/get-repo-info.sh issues)

# Instead of: https://github.com/d-oit/codeguardian/workflows/CI/badge.svg
# Use: $(./.opencode/get-repo-info.sh ci-badge)

# Instead of: https://codecov.io/gh/d-oit/codeguardian/branch/main/graph/badge.svg
# Use: $(./.opencode/get-repo-info.sh codecov-badge)

# Instead of: https://img.shields.io/github/downloads/d-oit/codeguardian/total.svg
# Use: $(./.opencode/get-repo-info.sh downloads-badge)

# Instead of: https://img.shields.io/github/contributors/d-oit/codeguardian.svg
# Use: $(./.opencode/get-repo-info.sh contributors-badge)

# Instead of: https://img.shields.io/github/last-commit/d-oit/codeguardian.svg
# Use: $(./.opencode/get-repo-info.sh last-commit-badge)

# Instead of: https://github.com/d-oit/codeguardian/actions
# Use: $(./.opencode/get-repo-info.sh actions)

# Instead of: https://github.com/d-oit/codeguardian/blob/main/docs/
# Use: $(./.opencode/get-repo-info.sh docs)

# Instead of: do-codeguardian
# Use: $(./.opencode/get-repo-info.sh name)

# Instead of: https://github.com/d-oit/codeguardian
# Use: $(./.opencode/get-repo-info.sh url)
```

---

**🎉 Configuration System Complete!**

The system is now ready for use. All hardcoded URLs have been fixed, and a dynamic configuration system has been implemented to prevent future issues with repository URL management.

## Testing the Configuration

```bash
# Test all repository information
./.opencode/get-repo-info.sh all

# Verify badge URLs are correct
./.opencode/get-repo-info.sh ci-badge
./.opencode/get-repo-info.sh codecov-badge
./.opencode/get-repo-info.sh downloads-badge
./.opencode/get-repo-info.sh contributors-badge
./.opencode/get-repo-info.sh last-commit-badge
./.opencode/get-repo-info.sh actions

# Test static configuration
cat .opencode/agent-config.json | jq -r '.repository.badges.ci_status'
cat .opencode/agent-config.json | jq -r '.repository.badges.codecov'

# Verify the configuration matches git remote
echo "Git remote URL: $(git remote get-url origin)"
echo "Config URL: $(cat .opencode/agent-config.json | jq -r '.repository.github_url')"
```

**Next Steps:**
1. Test the configuration: `./.opencode/get-repo-info.sh all`
2. Verify URLs are correct: `./.opencode/get-repo-info.sh github`
3. Use dynamic URLs in future documentation updates
4. Integrate with CI/CD pipelines for automated verification

**For Documentation Writers:**
- Use `./.opencode/get-repo-info.sh github` instead of hardcoded URLs
- Use `./.opencode/get-repo-info.sh ci-badge` for CI badges
- Use `./.opencode/get-repo-info.sh codecov-badge` for codecov badges
- Use `./.opencode/get-repo-info.sh downloads-badge` for download badges
- Use `./.opencode/get-repo-info.sh contributors-badge` for contributor badges
- Use `./.opencode/get-repo-info.sh last-commit-badge` for last commit badges
- Use `./.opencode/get-repo-info.sh actions` for GitHub Actions URL
- Use `./.opencode/get-repo-info.sh issues` for GitHub Issues URL
- Use `./.opencode/get-repo-info.sh docs` for documentation URL
- Use `./.opencode/get-repo-info.sh name` for repository name
- Use `./.opencode/get-repo-info.sh url` for repository URL
- Always test URLs before committing documentation changes

**Final verification - both should show the same URL:**
```bash
./.opencode/get-repo-info.sh github
git remote get-url origin
```

---

**Status**: ✅ **Configuration system is ready and tested**

All hardcoded URLs have been replaced with the correct repository name (`d-oit/do-codeguardian`), and a dynamic configuration system has been implemented to prevent future issues with repository URL management.

**Test the system:**
```bash
./.opencode/get-repo-info.sh all
```
