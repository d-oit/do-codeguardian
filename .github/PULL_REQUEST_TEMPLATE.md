# Pull Request Template for CodeGuardian

## Description
Provide a clear and concise description of the changes made in this pull request. Explain the problem this PR solves and the approach taken to address it. Include any relevant context or background information.

## Type of Change
Select the type(s) of change this PR introduces:
- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Refactoring (no functional changes)
- [ ] Security enhancement
- [ ] Performance improvement
- [ ] CI/CD or build system change
- [ ] Other (please specify): __________________

## Checklist
Before submitting this PR, please ensure the following:
- [ ] **Tests**: Added or updated unit tests, integration tests, and/or end-to-end tests as appropriate
- [ ] **Documentation**: Updated relevant documentation (README, API docs, user guides, etc.)
- [ ] **Security Review**: Conducted a security review of the changes, ensuring no new vulnerabilities are introduced
- [ ] **Performance**: Assessed performance impact and optimized where necessary
- [ ] **Code Quality**: Code follows Rust best practices and passes all linting checks (`cargo clippy`)
- [ ] **Dependencies**: No unnecessary dependencies added; all dependencies are secure and up-to-date
- [ ] **Analyzers**: If adding/modifying security analyzers, ensured they handle edge cases and false positives appropriately
- [ ] **Configuration**: Updated configuration files or examples if needed
- [ ] **License Compliance**: All new code complies with the project's license (MIT)

## Testing Details
Describe the testing approach used for this PR:
- **Test Coverage**: What areas of the codebase are covered by tests?
- **Test Results**: Provide summary of test results (e.g., "All tests pass", specific test outputs)
- **Manual Testing**: Any manual testing performed? Describe scenarios tested.
- **Edge Cases**: How were edge cases and error conditions handled?
- **Performance Testing**: If applicable, describe performance benchmarks or load testing conducted.

## Breaking Changes
If this PR introduces breaking changes, please list them here:
- [List any breaking changes, including API changes, configuration changes, or behavioral changes]
- [Explain migration steps for users if applicable]

## Additional Notes
Add any additional information, context, or considerations for reviewers:
- [Any known limitations or future improvements]
- [Related issues or PRs]
- [Screenshots or examples if UI changes are involved]

---

**By submitting this pull request, I confirm that:**
- My changes do not introduce new security vulnerabilities
- I have tested my changes thoroughly
- I have updated documentation as needed
- My code follows the project's coding standards and security guidelines