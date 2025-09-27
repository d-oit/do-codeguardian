# Duplicate Issue Prevention System

## Overview

The CodeGuardian project implements a comprehensive duplicate issue prevention system for performance regression detection. This system prevents the creation of redundant GitHub issues when multiple workflow runs detect the same performance regression.

## Features

### 🎯 Multi-Strategy Duplicate Detection
- **Exact Title Matching**: Identifies issues with identical titles
- **Commit Hash Matching**: Detects issues related to the same commit
- **Semantic Keyword Matching**: Uses intelligent keyword extraction to find semantically similar issues

### ⚡ Smart Caching System
- **Configurable TTL**: Cache entries expire after configurable time periods
- **Smart Invalidation**: Automatic cache invalidation when issues are updated
- **Performance Optimized**: Reduces GitHub API calls while maintaining accuracy

### 📊 Performance Monitoring
- **Metrics Collection**: Tracks duplicate prevention effectiveness
- **Dashboard Generation**: Creates visual reports for monitoring system performance
- **Trend Analysis**: Historical data for optimization decisions

### 🛡️ Robust Error Handling
- **Fallback Mechanisms**: Creates issues if duplicate detection fails
- **Input Validation**: Comprehensive validation of all inputs
- **Retry Logic**: Automatic retry for transient API failures

## Configuration

The system supports the following environment variables for customization:

```bash
# API Configuration
GITHUB_API_MAX_RETRIES=3          # Maximum retry attempts for API calls
GITHUB_API_RETRY_DELAY=2          # Delay between retries (seconds)

# Cache Configuration
GITHUB_ISSUE_CACHE_DIR="~/.cache/codeguardian/github-issues"  # Cache directory
GITHUB_ISSUE_CACHE_TTL=3600       # Cache TTL in seconds (1 hour)
GITHUB_CACHE_INVALIDATION=true    # Enable smart cache invalidation

# Detection Configuration
GITHUB_SIMILARITY_THRESHOLD=0.8   # Semantic matching threshold
GITHUB_MAX_KEYWORDS=5             # Maximum keywords for semantic matching
```

## Usage

### Automatic Integration

The system is automatically integrated into the performance benchmark workflow:

```yaml
- name: 🚨 Create Performance Regression Issue with Duplicate Prevention
  if: steps.regression_analysis.outputs.regression_detected == 'true'
  run: |
    ./scripts/create_performance_regression_issue.sh "${{ steps.regression_analysis.outputs.regression_details }}"
```

### Manual Usage

You can also use the scripts manually:

```bash
# Check for duplicate issues
./scripts/github-issue-utils.sh detect-duplicate "owner/repo" "Issue Title" "Issue Body" "commit-hash"

# Create or update issue with duplicate prevention
./scripts/create_performance_regression_issue.sh "Regression details here"

# Generate performance metrics report
./scripts/performance_monitoring_metrics.sh analyze

# Create monitoring dashboard
./scripts/duplicate_detection_dashboard.sh generate dashboard.html
```

## Architecture

### Core Components

1. **github-issue-utils.sh**: Core duplicate detection logic
2. **create_performance_regression_issue.sh**: Enhanced issue creation with duplicate prevention
3. **performance_monitoring_metrics.sh**: Metrics collection and analysis
4. **duplicate_detection_dashboard.sh**: Dashboard generation for monitoring

### Detection Strategies

#### 1. Exact Title Matching
```bash
# Searches for issues with identical titles
find_existing_issues "$repo" "\"$title\" in:title"
```

#### 2. Commit Hash Matching
```bash
# Searches for issues containing the commit hash
find_existing_issues "$repo" "$commit_hash in:title,body"
```

#### 3. Semantic Keyword Matching
```bash
# Extracts keywords and searches for semantically similar issues
keywords=$(extract_keywords "$title" "$body")
find_existing_issues "$repo" "$keywords in:title,body"
```

### Cache Management

The system implements intelligent caching with the following features:

- **File-based Cache**: Local file system cache for GitHub API responses
- **TTL-based Expiration**: Automatic expiration of stale cache entries
- **Smart Invalidation**: Targeted cache invalidation on issue updates
- **Cleanup Automation**: Automatic cleanup of expired cache entries

## Metrics and Monitoring

### Key Metrics

- **Total Events**: Number of duplicate detection attempts
- **New Issues Created**: Count of new issues created
- **Duplicates Prevented**: Count of duplicate issues prevented
- **Prevention Rate**: Percentage of duplicates successfully prevented

### Dashboard Features

- **Real-time Metrics**: Current system performance indicators
- **Historical Trends**: Performance trends over time
- **Configuration Display**: Current system configuration
- **Recommendations**: Automated optimization suggestions

### Accessing Reports

```bash
# Generate metrics report
./scripts/performance_monitoring_metrics.sh analyze

# Generate HTML dashboard
./scripts/duplicate_detection_dashboard.sh generate dashboard.html

# Clean up old metrics
./scripts/performance_monitoring_metrics.sh cleanup
```

## Testing

### Automated Tests

The system includes comprehensive test coverage using Bats:

```bash
cd tests
bats test_github_issue_utils.bats
```

### Test Coverage

- ✅ Keyword extraction functionality
- ✅ Commit hash handling
- ✅ Issue title generation
- ✅ Cache management
- ✅ Input validation
- ✅ Duplicate detection strategies
- ✅ Error handling scenarios
- ✅ API retry logic

## Troubleshooting

### Common Issues

#### High False Positive Rate
```bash
# Adjust similarity threshold
export GITHUB_SIMILARITY_THRESHOLD=0.9

# Reduce keyword count
export GITHUB_MAX_KEYWORDS=3
```

#### Low Duplicate Detection Rate
```bash
# Lower similarity threshold
export GITHUB_SIMILARITY_THRESHOLD=0.6

# Increase keyword count
export GITHUB_MAX_KEYWORDS=7

# Shorten cache TTL for fresher data
export GITHUB_ISSUE_CACHE_TTL=1800
```

#### API Rate Limiting
```bash
# Increase retry delay
export GITHUB_API_RETRY_DELAY=5

# Reduce retry attempts
export GITHUB_API_MAX_RETRIES=2

# Increase cache TTL to reduce API calls
export GITHUB_ISSUE_CACHE_TTL=7200
```

### Debug Mode

Enable verbose logging for troubleshooting:

```bash
# Enable debug output
set -x

# Run with verbose GitHub CLI output
gh --debug <command>
```

## Performance Optimization

### Best Practices

1. **Cache Management**
   - Set appropriate TTL based on issue activity
   - Enable smart cache invalidation
   - Regular cache cleanup

2. **API Usage**
   - Monitor API rate limits
   - Use appropriate retry delays
   - Batch operations when possible

3. **Detection Tuning**
   - Adjust similarity thresholds based on false positive/negative rates
   - Optimize keyword extraction patterns
   - Monitor detection effectiveness

### Monitoring Recommendations

- Review dashboard weekly for performance trends
- Analyze metrics monthly for optimization opportunities
- Adjust configuration based on observed patterns
- Monitor GitHub API usage to stay within limits

## Security Considerations

- All GitHub API calls use authenticated tokens
- Input validation prevents injection attacks
- File operations use safe path handling
- No sensitive data is logged or cached

## Future Enhancements

### Planned Features

- Machine learning-based similarity detection
- Cross-repository duplicate detection
- Integration with other issue management systems
- Advanced analytics and reporting
- Webhook-based real-time cache invalidation

### Contributing

To contribute to the duplicate prevention system:

1. Review the existing test suite
2. Add tests for new functionality
3. Ensure all validations pass
4. Update documentation
5. Submit pull request with comprehensive description

## Support

For issues or questions about the duplicate prevention system:

1. Check the troubleshooting section
2. Review test failures for debugging info
3. Generate metrics report for system status
4. Create issue in the repository with detailed logs