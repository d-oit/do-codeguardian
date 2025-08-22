# Automatic Documentation Updates

CodeGuardian now includes automatic documentation updates using [opencode](https://opencode.ai), an AI coding agent that helps maintain up-to-date project documentation.

## Overview

The automatic documentation system uses a pre-commit git hook to update project documentation before each commit. This ensures that documentation stays current with the latest code changes.

## Features

- **Automatic README Updates**: Keeps the main README.md current with project structure and features
- **API Documentation**: Updates API documentation based on source code analysis
- **Performance Documentation**: Maintains performance analysis and benchmark documentation
- **Security Documentation**: Updates security analyzer documentation
- **Pre-commit Integration**: Runs automatically before each git commit

## Setup

### Prerequisites

1. Install opencode:
   ```bash
   curl -fsSL https://opencode.ai/install | bash
   ```

2. Configure opencode authentication:
   ```bash
   opencode auth login
   ```
   Select your preferred LLM provider (Anthropic recommended).

3. Run the setup script:
   ```bash
   bash scripts/setup-opencode.sh
   ```

### Quick Setup

For a complete setup, run:
```bash
bash scripts/setup-opencode.sh
```

This will:
- Configure opencode authentication
- Initialize the project with opencode
- Test the documentation update process

## How It Works

### Pre-commit Hook

The system uses a git pre-commit hook (`.git/hooks/pre-commit`) that:

1. Runs before each commit
2. Executes the documentation update script
3. Updates relevant documentation files
4. Shows which files were modified
5. Allows you to review changes before committing

### Documentation Update Script

The `scripts/update-docs.sh` script:

1. Analyzes the current codebase using opencode
2. Updates README.md with current project information
3. Updates API documentation in the `docs/` directory
4. Updates performance and security documentation
5. Handles timeouts and errors gracefully

## Usage

### Automatic Mode (Recommended)

Once set up, documentation updates happen automatically before each commit. The pre-commit hook will:

- Update documentation files
- Show you what changed
- Allow you to commit the documentation changes along with your code

### Manual Mode

You can also run documentation updates manually:

```bash
# Update all documentation
bash scripts/update-docs.sh

# Setup or reconfigure opencode
bash scripts/setup-opencode.sh
```

## Configuration

### Opencode Configuration

The `.opencode/config.json` file contains project-specific settings:

```json
{
  "project": {
    "name": "CodeGuardian",
    "description": "A security-focused static analysis tool for Rust codebases",
    "type": "rust",
    "mainLanguage": "rust"
  },
  "documentation": {
    "autoUpdate": true,
    "includeApiDocs": true,
    "includePerformanceDocs": true,
    "includeSecurityDocs": true,
    "outputFormat": "markdown"
  }
}
```

### Customization

You can modify the documentation update script to:

- Add new types of documentation
- Change the analysis focus
- Adjust timeout values
- Customize the prompts sent to opencode

## Troubleshooting

### Common Issues

1. **Opencode not found**
   - Ensure opencode is installed and in your PATH
   - Run: `curl -fsSL https://opencode.ai/install | bash`

2. **Authentication required**
   - Run: `opencode auth login`
   - Select your preferred LLM provider

3. **Initialization failed**
   - Check that you're in the project root directory
   - Ensure AGENTS.md doesn't exist or delete it to re-initialize

4. **Documentation updates timeout**
   - The script has built-in timeouts to prevent hanging
   - You can adjust timeout values in `scripts/update-docs.sh`

5. **Pre-commit hook not working**
   - Ensure the hook is executable: `chmod +x .git/hooks/pre-commit`
   - Check that you're committing from the project root

### Disabling Automatic Updates

If you need to disable automatic documentation updates temporarily:

```bash
# Remove the pre-commit hook
rm .git/hooks/pre-commit

# Or make it non-executable
chmod -x .git/hooks/pre-commit
```

### Re-enabling Updates

To re-enable automatic updates:

```bash
# Make the hook executable again
chmod +x .git/hooks/pre-commit
```

## Integration with CI/CD

The automatic documentation system works well with CI/CD pipelines:

- **GitHub Actions**: The pre-commit hook runs locally before pushes
- **CI Checks**: Documentation updates can be validated in CI
- **Release Process**: Ensure documentation is up-to-date before releases

## Best Practices

1. **Review Changes**: Always review documentation changes before committing
2. **Test Updates**: Run manual updates to test the system
3. **Backup Configuration**: Keep your opencode configuration backed up
4. **Monitor Performance**: Watch for timeout issues and adjust as needed
5. **Customize Prompts**: Tailor the documentation prompts to your project's needs

## Contributing

When contributing to the documentation system:

1. Test your changes with the setup script
2. Update this documentation if you modify the system
3. Ensure backward compatibility
4. Follow the existing code style and patterns

## Support

For issues with the automatic documentation system:

1. Check the troubleshooting section above
2. Review opencode documentation: https://opencode.ai/docs
3. Check the project issues for similar problems
4. Create a new issue if you encounter a bug

## Related Files

- `scripts/update-docs.sh` - Main documentation update script
- `scripts/setup-opencode.sh` - Setup and configuration script
- `.git/hooks/pre-commit` - Git pre-commit hook
- `.opencode/config.json` - Opencode project configuration