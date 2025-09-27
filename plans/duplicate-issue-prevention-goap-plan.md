# GOAP Plan: Prevent Duplicate GitHub Issues for Performance Regressions

## Goal Summary
**Goal**: Prevent the creation of duplicate GitHub issues for performance regressions in the CodeGuardian project, specifically addressing issues like #113 and #114 where multiple workflow runs detect the same regression and create redundant issues.

**Initial World State**:
- Performance benchmark suite workflow runs daily and on PRs/pushes
- Workflow creates GitHub issues with title "🚨 Performance Regression Detected" whenever regression_detected = true
- No duplicate checking exists; multiple runs detecting the same regression create separate issues
- Existing github-issue-utils.sh script provides duplicate detection functionality but is not used in the workflow
- Issue creation uses actions/github-script@v7 with hardcoded logic

**Constraints**:
- Must maintain existing workflow triggers and job dependencies
- Cannot break existing performance regression detection logic
- Must use GitHub API permissions already granted (issues: write)
- Should minimize changes to reduce risk of introducing new bugs
- Must handle edge cases like concurrent workflow runs

## Actions Defined

### Action 1: Analyze Current Issue Creation Process
**Preconditions**:
- Access to .github/workflows/performance-benchmark-suite.yml
- Access to scripts/github-issue-utils.sh
- Understanding of GitHub Actions and GitHub API

**Effects**:
- Identified that issue creation happens in "performance-regression-analysis" job
- Confirmed duplicate detection script exists but unused
- Determined issue title is static: "🚨 Performance Regression Detected"

**Cost**: 2 (low effort analysis)

### Action 2: Create Enhanced Issue Creation Script
**Preconditions**:
- github-issue-utils.sh exists and functions correctly
- Workflow can execute bash scripts
- GitHub CLI (gh) is available in runner environment

**Effects**:
- New script that wraps duplicate detection and issue creation
- Uses detect_duplicate_issue function from github-issue-utils.sh
- Generates unique issue titles including commit hash or run ID
- Provides detailed logging for debugging

**Cost**: 4 (moderate scripting effort)

### Action 3: Modify Workflow to Use New Script
**Preconditions**:
- New issue creation script exists and tested
- Workflow file is editable
- Permissions allow workflow modifications

**Effects**:
- Replace actions/github-script step with bash script execution
- Pass regression details as environment variables or files
- Maintain same labels and issue structure

**Cost**: 3 (workflow modification)

### Action 4: Add Comprehensive Logging
**Preconditions**:
- Script execution allows stdout/stderr capture
- Workflow supports step outputs

**Effects**:
- Log duplicate detection results
- Log issue creation/update actions
- Include run IDs and commit hashes in logs
- Enable debugging of duplicate prevention logic

**Cost**: 2 (logging additions)

### Action 5: Implement Error Handling
**Preconditions**:
- Script can handle API failures gracefully
- Workflow supports conditional steps

**Effects**:
- Handle GitHub API rate limits or failures
- Fallback to creating issues if duplicate check fails
- Prevent workflow failures from duplicate detection issues

**Cost**: 3 (error handling logic)

### Action 6: Test Duplicate Prevention Logic
**Preconditions**:
- Test environment with GitHub repository access
- Ability to trigger workflow manually
- Existing issues for testing duplicate detection

**Effects**:
- Verified duplicate detection works correctly
- Confirmed no false positives or negatives
- Validated issue updates vs. creation logic
- Tested concurrent workflow runs

**Cost**: 5 (testing and validation)

### Action 7: Update Documentation
**Preconditions**:
- Access to workflow documentation
- Understanding of changes made

**Effects**:
- Updated workflow comments explaining duplicate prevention
- Added troubleshooting section for duplicate issues
- Documented new script usage

**Cost**: 2 (documentation updates)

### Action 8: Implement Automated Testing for github-issue-utils.sh
**Preconditions**:
- github-issue-utils.sh exists and functions are defined
- Bats testing framework available in CI environment
- Test environment with mock GitHub API access

**Effects**:
- Unit tests created for detect_duplicate_issue and related functions
- Tests cover various scenarios: exact matches, semantic matches, no matches
- CI pipeline includes automated test execution
- Test failures prevent deployment of changes

**Cost**: 4 (testing framework setup and test writing)

### Action 9: Add Input Validation to Scripts
**Preconditions**:
- Enhanced issue creation script exists
- Understanding of expected input formats for regression details

**Effects**:
- Explicit validation for all inputs passed to scripts (e.g., commit hash, regression metrics)
- Error messages for invalid inputs with guidance
- Prevents script execution with malformed data
- Logs validation failures for debugging

**Cost**: 2 (validation logic addition)

### Action 10: Create Monitoring Dashboard for Duplicate Detection Effectiveness
**Preconditions**:
- Metrics collection system in place (e.g., GitHub Actions artifacts or external monitoring)
- Access to dashboard creation tools (e.g., Grafana, GitHub Issues, or simple markdown reports)

**Effects**:
- Dashboard displaying duplicate detection accuracy metrics
- Tracks false positives, false negatives, and successful preventions
- Visual indicators for detection effectiveness over time
- Alerts for significant drops in accuracy

**Cost**: 3 (dashboard setup and metric integration)

### Action 11: Implement Cache Optimization with Smart Invalidation
**Preconditions**:
- Existing caching mechanism in github-issue-utils.sh or workflow
- Access to issue update events or webhooks

**Effects**:
- Cache invalidation triggered by issue updates, closures, or new comments
- Reduces stale cache hits while maintaining performance
- Configurable cache TTL based on issue activity
- Logs cache hit/miss ratios for optimization tuning

**Cost**: 3 (cache logic enhancement)

### Action 12: Make Duplicate Detection Parameters Configurable
**Preconditions**:
- Configuration management system (e.g., config files or environment variables)
- Understanding of tunable parameters (e.g., similarity thresholds, API timeouts)

**Effects**:
- Duplicate detection settings moved to configuration
- Parameters like title similarity threshold, commit hash inclusion, API retry limits configurable
- Environment-specific configurations (dev vs. prod)
- Documentation of all configurable parameters

**Cost**: 2 (configuration abstraction)

### Action 13: Add Performance Monitoring Metrics
**Preconditions**:
- Performance monitoring system (e.g., integrated with existing performance benchmarks)
- Access to API usage data and detection outcomes

**Effects**:
- Metrics collected for GitHub API call counts and response times
- Detection accuracy metrics (precision, recall) calculated and tracked
- Performance impact of duplicate checking measured
- Historical trends for optimization decisions

**Cost**: 3 (metrics collection and integration)

## Generated Plan

### Step 1: Analyze Current Implementation (Cost: 2)
Execute analysis of the performance-benchmark-suite.yml workflow and github-issue-utils.sh script to understand current issue creation process and available duplicate detection capabilities.

**Rationale**: Understanding the current state is essential before making changes to ensure compatibility and identify integration points.

### Step 2: Create Enhanced Issue Creation Script (Cost: 4)
Create a new bash script (e.g., `scripts/create_performance_regression_issue.sh`) that:
- Uses `detect_duplicate_issue` from github-issue-utils.sh
- Generates unique titles with commit hash/run ID
- Creates or updates issues appropriately
- Includes comprehensive logging

**Rationale**: This provides a clean separation of concerns and leverages existing duplicate detection logic.

### Step 3: Implement Automated Testing for Scripts (Cost: 4)
Set up unit tests for github-issue-utils.sh functions using Bats, covering duplicate detection logic, input handling, and edge cases. Integrate tests into CI pipeline.

**Rationale**: High priority recommendation to ensure reliability and prevent regressions in duplicate detection functionality.

### Step 4: Add Input Validation to Enhanced Script (Cost: 2)
Enhance the issue creation script with explicit validation for regression details inputs, including format checks and required field validation.

**Rationale**: High priority to prevent script failures from invalid data and improve robustness of the duplicate prevention system.

### Step 5: Modify Workflow Issue Creation Step (Cost: 3)
Replace the `actions/github-script` step in the workflow with a bash script execution that calls the new enhanced script, passing regression details via environment variables or temporary files.

**Rationale**: Integrates duplicate prevention into the workflow with minimal disruption to existing logic.

### Step 6: Add Logging and Error Handling (Cost: 5)
Enhance the script with:
- Detailed logging of duplicate detection results
- Error handling for API failures
- Fallback mechanisms
- Workflow step outputs for visibility

**Rationale**: Ensures reliability and debuggability of the duplicate prevention system.

### Step 7: Implement Cache Optimization (Cost: 3)
Add smart cache invalidation logic to the duplicate detection process, triggering invalidation on issue updates or closures to maintain accuracy.

**Rationale**: Medium priority to improve performance and reduce false positives from stale cached data.

### Step 8: Make Detection Parameters Configurable (Cost: 2)
Abstract duplicate detection settings (e.g., similarity thresholds, timeouts) into configurable parameters via environment variables or config files.

**Rationale**: Medium priority for flexibility and easier tuning without code changes.

### Step 9: Add Performance Monitoring Metrics (Cost: 3)
Integrate metrics collection for API usage, detection accuracy, and performance impact into the existing monitoring system.

**Rationale**: Medium priority to track effectiveness and optimize resource usage.

### Step 10: Test Implementation (Cost: 5)
- Test duplicate detection with existing issues
- Trigger multiple workflow runs to verify no duplicates created
- Test edge cases like API failures and concurrent runs
- Validate issue updates work correctly
- Include validation of new automated tests and input validation

**Rationale**: Comprehensive testing ensures the solution works in production and doesn't introduce regressions, now including the new enhancements.

### Step 11: Create Monitoring Dashboard (Cost: 3)
Develop a dashboard to visualize duplicate detection metrics, effectiveness rates, and trends for ongoing monitoring.

**Rationale**: High priority to provide visibility into system performance and enable proactive improvements.

### Step 12: Update Documentation (Cost: 2)
Update workflow file comments and any relevant documentation to explain the duplicate prevention mechanism, new configurations, monitoring, and troubleshooting steps.

**Rationale**: Maintains code maintainability and helps future developers understand the enhanced system.

**Total Estimated Cost**: 38

## Analysis

### Pros of This Plan:
- **Leverages Existing Code**: Uses the already-implemented github-issue-utils.sh, reducing development effort
- **Minimal Workflow Changes**: Only modifies the issue creation step, preserving existing benchmark logic
- **Robust Duplicate Detection**: Multiple strategies (exact title, commit hash, semantic matching) prevent various types of duplicates
- **Maintains Functionality**: Issues are still created when regressions are detected; duplicates are just prevented
- **Scalable**: Can be extended to other issue types in the future

### Cons of This Plan:
- **Dependency on External Script**: Requires github-issue-utils.sh to be maintained and functional
- **GitHub CLI Dependency**: Assumes `gh` CLI is available and authenticated in the runner
- **Potential API Limits**: Duplicate checking adds API calls that could hit rate limits
- **Testing Complexity**: Requires careful testing of concurrent scenarios

### Contingency Plans:
- **API Failure Fallback**: If duplicate check fails, proceed with issue creation (current behavior)
- **Script Failure**: Workflow continues but logs the failure for manual review
- **False Positives**: If duplicate detection incorrectly identifies matches, manual issue management can override
- **Performance Impact**: If API calls become too slow, implement caching or async processing

### Alternative Plans Considered:
1. **Inline JavaScript Modification**: Modify the existing actions/github-script to include duplicate checking using GitHub API directly. Rejected due to complexity of JavaScript API calls and maintenance burden.
2. **Database Tracking**: Maintain a database of created issues. Rejected due to added infrastructure complexity and potential for data inconsistencies.
3. **Workflow-Level Deduplication**: Use concurrency controls to prevent parallel runs. Rejected as it wouldn't prevent duplicates from different triggers (schedule vs. push).

This plan provides an efficient, maintainable solution that directly addresses the duplicate issue problem while minimizing risk to the existing performance monitoring system.