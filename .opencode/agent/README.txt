# OpenCode Agents for CodeGuardian

This directory contains specialized OpenCode agents designed specifically for the CodeGuardian security analysis CLI project. Each agent is configured to handle specific aspects of CodeGuardian development, security analysis, and optimization.

## Available Agents

### 🌐 General Agent (`general.md`)
- **Purpose**: General-purpose agent with swarm orchestration capabilities
- **Specialization**: Research, analysis, coordination, and multi-agent orchestration
- **Key Features**:
  - Complex question research and reasoning
  - Multi-step task coordination
  - Swarm orchestration for parallel agent execution
  - Codebase-wide analysis and synthesis
  - Cross-cutting concerns and system-level insights

### 🔒 Security Auditor (`security-auditor.md`)
- **Purpose**: Performs security audits and identifies vulnerabilities
- **Specialization**: Rust security patterns, vulnerability detection, secure coding practices
- **Key Features**:
  - Memory safety analysis
  - Cryptographic security review
  - Input validation assessment
  - Authentication/authorization review
  - Dependency security analysis

### ⚡ Performance Optimizer (`performance-optimizer.md`)
- **Purpose**: Optimizes performance, memory usage, and resource efficiency
- **Specialization**: Rust performance patterns, algorithm optimization, memory management
- **Key Features**:
  - Algorithm complexity analysis
  - Memory optimization
  - I/O performance optimization
  - Concurrency and parallelism
  - CI/CD performance optimization

### 🏗️ Code Quality Reviewer (`code-quality-reviewer.md`)
- **Purpose**: Reviews code for quality, maintainability, and best practices
- **Specialization**: Rust best practices, code organization, maintainability analysis
- **Key Features**:
  - Code structure analysis
  - Documentation review
  - Testing coverage assessment
  - Technical debt identification
  - Refactoring recommendations

### 🤖 ML Training Specialist (`ml-training-specialist.md`)
- **Purpose**: Manages ML training, data preparation, and model optimization
- **Specialization**: RUV-FANN neural networks, ML pipeline optimization, model training
- **Key Features**:
  - Training data preparation
  - Model architecture optimization
  - Performance metrics analysis
  - Online learning implementation
  - Model deployment optimization

### 🧪 Testing Engineer (`testing-engineer.md`)
- **Purpose**: Manages testing, generates tests, and ensures code quality
- **Specialization**: Rust testing frameworks, test coverage, automated testing
- **Key Features**:
  - Unit test generation
  - Integration test creation
  - Property-based testing
  - Performance testing
  - CI/CD testing integration

### 📚 Documentation Specialist (`documentation-specialist.md`)
- **Purpose**: Generates and maintains comprehensive documentation
- **Specialization**: Technical writing, API documentation, user guides
- **Key Features**:
  - API documentation generation
  - User guide creation
  - Architecture documentation
  - Troubleshooting guides
  - Documentation maintenance

### 🔨 Build/CI Optimizer (`build-ci-optimizer.md`)
- **Purpose**: Optimizes build processes, CI/CD pipelines, and deployment
- **Specialization**: Cargo optimization, GitHub Actions, deployment automation
- **Key Features**:
  - Build time optimization
  - CI/CD pipeline optimization
  - Release automation
  - Security scanning integration
  - Cost optimization

### ⚙️ GitHub Workflow Optimizer (`github-workflow-optimizer.md`)
- **Purpose**: Optimizes GitHub Actions workflows for performance and efficiency
- **Specialization**: Workflow optimization, caching strategies, cost reduction
- **Key Features**:
  - Workflow performance analysis
  - Caching strategy implementation
  - Security integration optimization
  - Cost reduction techniques
  - Resource utilization optimization

### 📦 Cargo Dependency Manager (`cargo-dependency-manager.md`)
- **Purpose**: Manages Cargo dependencies with security and performance focus
- **Specialization**: Dependency analysis, security audits, optimization
- **Key Features**:
  - Dependency vulnerability scanning
  - Security audit automation
  - Performance optimization
  - License compliance checking
  - Dependency tree optimization

### 🐝 Swarm Orchestrator (`swarm-orchestrator.md`)
- **Purpose**: Orchestrates multiple specialized agents in parallel for complex tasks
- **Specialization**: Dynamic agent coordination, conflict resolution, result synthesis
- **Key Features**:
  - Adaptive scaling based on task complexity
  - Parallel execution of multiple agents
  - Conflict resolution and result synthesis
  - Dynamic workload distribution
  - Holistic outcome generation

### 📊 GitHub Projects Manager (`github-projects-manager.md`)
- **Purpose**: Manages GitHub Projects for CodeGuardian development roadmap and project tracking
- **Specialization**: Project creation, organization, roadmap management, and progress tracking
- **Key Features**:
  - Security analysis project management
  - Performance optimization tracking
  - ML integration roadmap planning
  - CI/CD integration project organization
  - Development roadmap creation and management
  - Automated project insights and reporting

## Agent Configuration

All agents are configured in the main `opencode.json` file with the following settings:

- **Mode**: All agents are configured as `subagent` mode for specialized tasks
- **Temperature**: Set appropriately for each agent's task (0.1-0.3 range)
- **Tools**: Configured with read-only access for analysis, selective write/bash access
- **No Model Configuration**: Model selection is handled at runtime by the OpenCode system

## Usage

### Invoking Agents

You can invoke these agents in several ways:

1. **Direct Invocation**: Mention the agent by name in your request
   ```
   @security-auditor please review this code for security vulnerabilities
   ```

2. **Task Delegation**: Use the Task tool to launch specific agents
   ```
   Use the Task tool to launch the performance-optimizer agent to analyze this function
   ```

3. **Automatic Activation**: Agents can be automatically triggered based on context and content

### Best Practices

- **Be Specific**: Clearly describe what you want the agent to analyze or optimize
- **Provide Context**: Include relevant code, configuration, or requirements
- **Set Expectations**: Specify the scope and depth of analysis needed
- **Follow Recommendations**: Implement agent suggestions incrementally and test thoroughly

### Swarm Orchestration

The **Swarm Orchestrator Agent** is the dedicated agent for coordinating multiple specialized agents in parallel for complex tasks:

- **When to Use**: Complex analysis requiring multiple perspectives (security + performance + quality)
- **Benefits**: Parallel execution, comprehensive coverage, unified synthesis of results
- **Available Agents**: All 8 specialized agents can be orchestrated together
- **Coordination**: Handles conflicts and synthesizes findings into unified reports

**Example Usage:**
```
@swarm-orchestrator perform a comprehensive security and performance review of the new authentication module
```
This will automatically coordinate the security-auditor, performance-optimizer, and code-quality-reviewer agents in parallel.

**Advanced Usage:**
```
@swarm-orchestrator swarm-optimize and secure the entire ML pipeline
```
This will dynamically scale and coordinate relevant agents based on task complexity.

## Enhanced CI/CD Workflow

The agent system now provides comprehensive CI/CD workflow optimization through specialized agents:

### Build & Dependency Management
- **@build-ci-optimizer**: Overall CI/CD strategy and optimization
- **@github-workflow-optimizer**: GitHub Actions workflow optimization
- **@cargo-dependency-manager**: Cargo dependency security and performance

### Git & GitHub Integration
- **@git-best-practices**: Git workflow and commit hygiene
- **@github-pr-manager**: Pull request automation
- **@github-issue-manager**: Issue tracking and management
- **@github-label-manager**: Automated labeling and categorization
- **@github-workflow-manager**: Workflow management and debugging
- **@github-projects-manager**: Project and roadmap management

### Quality & Security Gates
- **@security-auditor**: Security vulnerability scanning
- **@testing-engineer**: Comprehensive test automation
- **@code-quality-reviewer**: Code quality assessment
- **@performance-optimizer**: Performance validation

### Orchestration & Coordination
- **@swarm-orchestrator**: Parallel agent coordination for complex tasks
- **@general**: Research and analysis coordination

This creates a complete CI/CD ecosystem that can handle everything from dependency management to deployment automation with security and performance built-in at every stage.

## Integration with CodeGuardian

These consolidated agents are specifically designed to work with CodeGuardian's:

- **Security-first architecture**: All agents prioritize security considerations
- **Performance requirements**: Optimized for CI/CD environments with <30s analysis times
- **Memory constraints**: Designed for <100MB peak memory usage
- **Rust ecosystem**: Deep understanding of Rust patterns and best practices
- **ML integration**: Specialized knowledge of RUV-FANN and ML pipeline
- **GitHub integration**: Understanding of GitHub API and workflow patterns
- **Swarm orchestration**: Dedicated swarm-orchestrator agent for complex multi-agent tasks

## Agent Status

- **Active Agents**: 12 consolidated, well-maintained agents
- **Duplicate Agents**: 4 pairs consolidated (marked for archival)
- **Configuration**: Standardized without model specifications
- **Maintenance**: All agents follow consistent structure and formatting
- **Swarm Capability**: Dedicated swarm-orchestrator agent for parallel execution
- **CI/CD Integration**: Comprehensive GitHub Actions and Cargo optimization suite

## Customization

To customize agent behavior:

1. **Modify Agent Files**: Edit the `.md` files in this directory to adjust agent prompts and behavior
2. **Update Configuration**: Modify `opencode.json` to change model, temperature, or tool access
3. **Add New Agents**: Create new `.md` files following the established format
4. **Adjust Permissions**: Configure tool access based on agent requirements

### Enterprise Support Configuration

By default, agents are configured to focus on community and open-source usage. To control enterprise support output:

- Set `"enterprise_support": false` in `opencode.json` to disable default enterprise support messaging
- Set `"enterprise_support": true` to enable enterprise-focused responses when needed
- Individual agents can be configured with custom enterprise support behavior in their `.md` files

## GitHub Projects Manager Usage Examples

The new GitHub Projects Manager agent provides comprehensive project management for CodeGuardian:

### Security Analysis Projects
```
@github-projects-manager create a security analysis enhancement project for Q4
@github-projects-manager add the latest security findings to the vulnerability tracking project
@github-projects-manager generate progress report for security analyzer improvements
```

### Performance Optimization Projects
```
@github-projects-manager create a performance optimization roadmap for the next release
@github-projects-manager organize performance benchmark results in the optimization project
@github-projects-manager track resource usage improvements across the codebase
```

### ML Integration Projects
```
@github-projects-manager set up ML model accuracy improvement tracking
@github-projects-manager monitor false positive reduction progress
@github-projects-manager create roadmap for ML pipeline enhancements
```

### CI/CD Integration Projects
```
@github-projects-manager organize build optimization tasks for the upcoming sprint
@github-projects-manager track GitHub Actions workflow improvements
@github-projects-manager manage deployment automation project
```

### Development Roadmap Projects
```
@github-projects-manager create comprehensive development roadmap for next quarter
@github-projects-manager add new security analyzer features to the roadmap
@github-projects-manager generate insights from development project data
```

## Contributing

When adding new agents:

1. Follow the established markdown format with frontmatter configuration
2. Include comprehensive role definition and specialization areas
3. Provide clear usage guidelines and examples
4. Test agent effectiveness with CodeGuardian-specific scenarios
5. Update this README with new agent documentation

## Support

For issues with these agents or requests for new agents, please:

1. Check existing agent capabilities first
2. Provide specific use cases and requirements
3. Include examples of desired agent behavior
4. Consider contributing improvements to existing agents