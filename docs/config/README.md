# Configuration

CodeGuardian uses TOML configuration files for customization.

## Default Configuration

```toml
[security]
fail_on_issues = true
max_file_size = "10MB"

[analysis]
parallel_workers = 0
streaming = false

[output]
format = "json"
verbose = false
```

## Configuration Options

### Security Settings

- `fail_on_issues`: Exit with error if security issues are found
- `max_file_size`: Maximum file size to analyze

### Analysis Settings

- `parallel_workers`: Number of parallel analysis workers (0 = auto)
- `streaming`: Enable streaming analysis for large files

### Output Settings

- `format`: Output format (json, human, sarif)
- `verbose`: Enable verbose output
