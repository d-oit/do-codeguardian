# Security Policy

## Supported Versions

We actively support the following versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| 0.x.x   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability in CodeGuardian, please follow these steps:

### For Public Issues
- Open an issue in our [GitHub Issues](../../../issues)
- Use the "Security" label
- Provide detailed information about the vulnerability

### For Private/Sensitive Issues
- Use GitHub's private vulnerability reporting feature
- Go to the Security tab and click "Report a vulnerability"
- Provide detailed information about the vulnerability

### What to Include
- Description of the vulnerability
- Steps to reproduce
- Potential impact on code analysis and security scanning
- Affected components (CLI, analyzers, ML models, etc.)
- Suggested fix (if you have one)

### Response Timeline
- **Initial Response**: Within 24 hours
- **Status Updates**: Every 48 hours until resolved
- **Resolution**: We aim to resolve critical vulnerabilities within 5 days

### Security Best Practices

#### For Users
- Always download CodeGuardian from official releases
- Keep your installation updated with latest security patches
- Use `--memory-limit` and `--max-file-size` to prevent resource exhaustion
- Enable `--security-enhanced` mode for additional security checks
- Review analysis results carefully, especially in automated CI/CD pipelines

#### For CI/CD Integration
- Set appropriate resource limits (`--memory-limit`, `--max-parallel`)
- Use `--fail-on-issues` to block builds with security findings
- Implement proper secret management for GitHub tokens
- Regularly update your CodeGuardian version
- Monitor for unusual analysis patterns or performance degradation

#### For Development
- Follow secure coding practices when extending analyzers
- Validate all file paths and inputs to prevent path traversal
- Implement proper error handling without exposing sensitive information
- Use the provided security utilities for path validation and secret redaction

### Security Features

CodeGuardian includes several built-in security features:

- **Path Validation**: Prevents directory traversal attacks
- **File Size Limits**: Configurable limits to prevent resource exhaustion
- **Secret Redaction**: Automatic removal of sensitive patterns from logs
- **Sandboxed Execution**: No symlink following, resource limits
- **Memory Safety**: Streaming analysis for large files to prevent OOM
- **Input Validation**: Comprehensive validation of all user inputs

### Responsible Disclosure

We follow a responsible disclosure process:
1. Vulnerability is reported privately
2. CodeGuardian team investigates and validates the issue
3. Fix is developed and tested
4. Security advisory is prepared
5. Fix is deployed to production
6. Public disclosure after a reasonable period (typically 90 days)

Thank you for helping keep CodeGuardian and the security community secure!
