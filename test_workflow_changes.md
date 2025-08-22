# Workflow Testing Changes

This PR contains intentional changes to test the GitHub Actions workflows:

## Files Added/Modified

### 1. `src/test_security_sample.rs`
- Contains intentional security issues for testing
- Should trigger security analysis workflows
- Includes hardcoded secrets, SQL injection patterns, etc.

### 2. `test_workflow_changes.md` (this file)
- Documents the testing approach
- Should be analyzed by workflows

## Expected Workflow Triggers

### PR-Triggered Workflows:
1. **`codeguardian-ci.yml`** - Should run PR analysis
2. **`turbo-pr-analysis.yml`** - Should analyze changed files only

### Expected Findings:
- Hardcoded API key detection
- TODO/FIXME comments
- Performance issues
- Debug code patterns

## Testing Goals

✅ Verify workflows parse and execute without syntax errors
✅ Confirm PR-specific analysis works
✅ Validate security finding detection
✅ Test workflow commenting on PR
✅ Ensure proper artifact upload

## Notes

This is a test PR specifically designed to validate the GitHub Actions workflows after fixing all syntax errors. The security issues are intentional and should be detected by CodeGuardian.