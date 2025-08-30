---
title: "ci-cd azure-devops"
description: "Generate Azure DevOps YAML pipelines for CodeGuardian integration"
category: "CI/CD Platform"
tags: ["azure", "devops", "pipeline", "yaml", "ci-cd"]
---

# ci-cd azure-devops

Generate Azure DevOps YAML pipeline configurations for CodeGuardian integration, including multi-stage pipelines, security scanning, and deployment automation with Azure-specific features.

## Synopsis

```bash
codeguardian ci-cd azure-devops [OPTIONS] [COMMAND]
```

## Description

The `ci-cd azure-devops` command creates optimized Azure DevOps YAML pipeline configurations for CodeGuardian, supporting multi-stage CI/CD workflows with comprehensive security and performance monitoring.

### Key Features

- **YAML pipeline generation**: Create Azure DevOps YAML pipelines
- **Multi-stage support**: Support for complex multi-stage workflows
- **Security integration**: Automated security scanning and compliance
- **Azure features**: Leverage Azure DevOps specific features
- **Performance monitoring**: Build performance tracking and optimization

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--pipeline-type` | Type of pipeline (basic,multi-stage,pr,release) | string | basic | No |
| `--output-file` | Output file path | string | azure-pipelines.yml | No |
| `--include-security` | Include security scanning jobs | boolean | true | No |
| `--include-performance` | Include performance monitoring | boolean | false | No |
| `--stages` | Pipeline stages to include | string[] | build,test,security,deploy | No |
| `--agents` | Agent pool names | string[] | ubuntu-latest | No |
| `--variables` | Pipeline variables | string[] | [] | No |
| `--artifacts` | Artifact names to publish | string[] | [] | No |

## Commands

### generate
Generate Azure DevOps YAML pipeline.

```bash
codeguardian ci-cd azure-devops generate [OPTIONS]
```

### validate
Validate existing Azure DevOps pipeline.

```bash
codeguardian ci-cd azure-devops validate [OPTIONS]
```

### optimize
Optimize existing Azure DevOps pipeline.

```bash
codeguardian ci-cd azure-devops optimize [OPTIONS]
```

### convert
Convert from other CI/CD formats.

```bash
codeguardian ci-cd azure-devops convert [OPTIONS]
```

## Examples

### Generate Basic Pipeline

```bash
# Generate basic Azure DevOps pipeline
codeguardian ci-cd azure-devops generate \
  --pipeline-type basic \
  --include-security \
  --agents ubuntu-latest
```

### Generate Multi-stage Pipeline

```bash
# Generate multi-stage pipeline
codeguardian ci-cd azure-devops generate \
  --pipeline-type multi-stage \
  --include-performance \
  --stages build,test,security,staging,prod
```

### Generate PR Validation Pipeline

```bash
# Generate PR validation pipeline
codeguardian ci-cd azure-devops generate \
  --pipeline-type pr \
  --include-security \
  --agents windows-latest,ubuntu-latest
```

### Validate Pipeline

```bash
# Validate existing azure-pipelines.yml
codeguardian ci-cd azure-devops validate \
  --output-file azure-pipelines.yml
```

### Convert from GitHub Actions

```bash
# Convert GitHub Actions workflow to Azure DevOps
codeguardian ci-cd azure-devops convert \
  --from github-actions \
  --input-file .github/workflows/ci.yml \
  --output-file azure-pipelines.yml
```

## Basic Pipeline

### Simple CI Pipeline

```yaml
# azure-pipelines.yml
trigger:
  branches:
    include:
    - main
    - develop

pool:
  vmImage: 'ubuntu-latest'

variables:
  CODEGUARDIAN_VERSION: 'latest'
  BUILD_CONFIGURATION: 'Release'

steps:
- checkout: self
  fetchDepth: 1

- task: Bash@3
  displayName: 'Setup CodeGuardian'
  inputs:
    targetType: 'inline'
    script: |
      # Download and install CodeGuardian
      curl -L https://github.com/your-org/codeguardian/releases/download/$(CODEGUARDIAN_VERSION)/codeguardian-linux-x64.tar.gz | tar xz
      chmod +x codeguardian
      echo "##vso[task.setvariable variable=PATH]$PWD:$PATH"

- task: Bash@3
  displayName: 'CodeGuardian Security Scan'
  inputs:
    targetType: 'inline'
    script: |
      ./codeguardian check --fail-on-issues --format json > $(Build.ArtifactStagingDirectory)/codeguardian-results.json
  continueOnError: false

- task: PublishBuildArtifacts@1
  displayName: 'Publish CodeGuardian Results'
  inputs:
    pathToPublish: '$(Build.ArtifactStagingDirectory)/codeguardian-results.json'
    artifactName: 'CodeGuardianResults'

- task: Bash@3
  displayName: 'Build Application'
  inputs:
    targetType: 'inline'
    script: |
      cargo build --release

- task: Bash@3
  displayName: 'Run Tests'
  inputs:
    targetType: 'inline'
    script: |
      cargo test --verbose

- task: PublishTestResults@2
  displayName: 'Publish Test Results'
  inputs:
    testResultsFiles: 'target/debug/deps/*.xml'
    testRunTitle: 'Unit Tests'
```

## Multi-stage Pipeline

### Comprehensive Multi-stage Pipeline

```yaml
# azure-pipelines.yml
trigger:
  branches:
    include:
    - main
  paths:
    exclude:
    - docs/
    - README.md

pr:
  branches:
    include:
    - main
    - develop
  paths:
    exclude:
    - docs/

stages:
- stage: 'Build'
  displayName: 'Build Stage'
  jobs:
  - job: 'Build'
    displayName: 'Build Job'
    pool:
      vmImage: 'ubuntu-latest'
    variables:
      CODEGUARDIAN_VERSION: 'latest'
    steps:
    - checkout: self
      fetchDepth: 0

    - task: Bash@3
      displayName: 'Setup CodeGuardian'
      inputs:
        targetType: 'inline'
        script: |
          curl -L https://github.com/your-org/codeguardian/releases/download/$(CODEGUARDIAN_VERSION)/codeguardian-linux-x64.tar.gz | tar xz
          chmod +x codeguardian
          echo "##vso[task.setvariable variable=PATH]$PWD:$PATH"

    - task: Bash@3
      displayName: 'Security Scan'
      inputs:
        targetType: 'inline'
        script: |
          ./codeguardian check --fail-on-issues --format sarif > $(Build.ArtifactStagingDirectory)/security-results.sarif
      continueOnError: false

    - task: PublishSecurityAnalysisLogs@3
      displayName: 'Publish Security Results'
      inputs:
        sarifFile: '$(Build.ArtifactStagingDirectory)/security-results.sarif'

    - task: Bash@3
      displayName: 'Build Application'
      inputs:
        targetType: 'inline'
        script: |
          cargo build --release

    - task: Bash@3
      displayName: 'Run Unit Tests'
      inputs:
        targetType: 'inline'
        script: |
          cargo test --verbose -- --nocapture

    - task: PublishTestResults@2
      displayName: 'Publish Test Results'
      inputs:
        testResultsFiles: 'target/debug/deps/*.xml'
        testRunTitle: 'Unit Tests'

    - task: CopyFiles@2
      displayName: 'Copy Build Artifacts'
      inputs:
        contents: 'target/release/codeguardian'
        targetFolder: '$(Build.ArtifactStagingDirectory)'

    - task: PublishBuildArtifacts@1
      displayName: 'Publish Build Artifacts'
      inputs:
        pathToPublish: '$(Build.ArtifactStagingDirectory)'
        artifactName: 'build-artifacts'

- stage: 'Test'
  displayName: 'Test Stage'
  dependsOn: 'Build'
  jobs:
  - job: 'IntegrationTests'
    displayName: 'Integration Tests'
    pool:
      vmImage: 'ubuntu-latest'
    steps:
    - checkout: none

    - task: DownloadBuildArtifacts@0
      displayName: 'Download Build Artifacts'
      inputs:
        downloadType: 'single'
        artifactName: 'build-artifacts'
        downloadPath: '$(System.ArtifactsDirectory)'

    - task: Bash@3
      displayName: 'Run Integration Tests'
      inputs:
        targetType: 'inline'
        script: |
          chmod +x $(System.ArtifactsDirectory)/codeguardian
          # Run integration tests
          echo "Running integration tests..."

    - task: PublishTestResults@2
      displayName: 'Publish Integration Test Results'
      inputs:
        testResultsFiles: 'integration-test-results.xml'
        testRunTitle: 'Integration Tests'

  - job: 'PerformanceTests'
    displayName: 'Performance Tests'
    pool:
      vmImage: 'ubuntu-latest'
    condition: and(succeeded(), eq(variables['RunPerformanceTests'], 'true'))
    steps:
    - checkout: none

    - task: DownloadBuildArtifacts@0
      displayName: 'Download Build Artifacts'
      inputs:
        downloadType: 'single'
        artifactName: 'build-artifacts'
        downloadPath: '$(System.ArtifactsDirectory)'

    - task: Bash@3
      displayName: 'Run Performance Tests'
      inputs:
        targetType: 'inline'
        script: |
          chmod +x $(System.ArtifactsDirectory)/codeguardian
          # Run performance benchmarks
          $(System.ArtifactsDirectory)/codeguardian performance benchmark --output performance-results.json

    - task: PublishBuildArtifacts@1
      displayName: 'Publish Performance Results'
      inputs:
        pathToPublish: 'performance-results.json'
        artifactName: 'performance-results'

- stage: 'DeployStaging'
  displayName: 'Deploy to Staging'
  dependsOn: 'Test'
  condition: and(succeeded(), ne(variables['Build.SourceBranch'], 'refs/heads/main'))
  jobs:
  - deployment: 'DeployStaging'
    displayName: 'Deploy to Staging'
    pool:
      vmImage: 'ubuntu-latest'
    environment: 'staging'
    strategy:
      runOnce:
        deploy:
          steps:
          - task: DownloadBuildArtifacts@0
            displayName: 'Download Build Artifacts'
            inputs:
              downloadType: 'single'
              artifactName: 'build-artifacts'
              downloadPath: '$(Pipeline.Workspace)'

          - task: Bash@3
            displayName: 'Deploy to Staging'
            inputs:
              targetType: 'inline'
              script: |
                echo "Deploying to staging environment"
                # Deployment logic here

- stage: 'DeployProduction'
  displayName: 'Deploy to Production'
  dependsOn: 'Test'
  condition: and(succeeded(), eq(variables['Build.SourceBranch'], 'refs/heads/main'))
  jobs:
  - deployment: 'DeployProduction'
    displayName: 'Deploy to Production'
    pool:
      vmImage: 'ubuntu-latest'
    environment: 'production'
    strategy:
      runOnce:
        deploy:
          steps:
          - task: DownloadBuildArtifacts@0
            displayName: 'Download Build Artifacts'
            inputs:
              downloadType: 'single'
              artifactName: 'build-artifacts'
              downloadPath: '$(Pipeline.Workspace)'

          - task: Bash@3
            displayName: 'Deploy to Production'
            inputs:
              targetType: 'inline'
              script: |
                echo "Deploying to production environment"
                # Production deployment logic here
```

## PR Validation Pipeline

### Pull Request Validation

```yaml
# azure-pipelines.yml
trigger: none

pr:
  branches:
    include:
    - main
    - develop
  paths:
    exclude:
    - docs/
    - README.md

pool:
  vmImage: 'ubuntu-latest'

variables:
  CODEGUARDIAN_VERSION: 'latest'

steps:
- checkout: self
  fetchDepth: 0

- task: Bash@3
  displayName: 'Setup CodeGuardian'
  inputs:
    targetType: 'inline'
    script: |
      curl -L https://github.com/your-org/codeguardian/releases/download/$(CODEGUARDIAN_VERSION)/codeguardian-linux-x64.tar.gz | tar xz
      chmod +x codeguardian
      echo "##vso[task.setvariable variable=PATH]$PWD:$PATH"

- task: Bash@3
  displayName: 'CodeGuardian PR Validation'
  inputs:
    targetType: 'inline'
    script: |
      ./codeguardian check --fail-on-issues --diff $(System.PullRequest.SourceBranch) --format json > codeguardian-pr-results.json
  continueOnError: false

- task: PublishBuildArtifacts@1
  displayName: 'Publish PR Validation Results'
  inputs:
    pathToPublish: 'codeguardian-pr-results.json'
    artifactName: 'PRValidationResults'

- task: Bash@3
  displayName: 'Build and Test'
  inputs:
    targetType: 'inline'
    script: |
      cargo build --release
      cargo test --verbose

- task: PublishTestResults@2
  displayName: 'Publish Test Results'
  inputs:
    testResultsFiles: 'target/debug/deps/*.xml'
    testRunTitle: 'PR Tests'

- task: Bash@3
  displayName: 'Comment PR with Results'
  inputs:
    targetType: 'inline'
    script: |
      # Add PR comment with CodeGuardian results
      echo "PR validation completed"
  condition: always()
```

## Release Pipeline

### Release Pipeline

```yaml
# azure-pipelines.yml
trigger: none

resources:
  pipelines:
  - pipeline: 'CI'
    source: 'CI Pipeline'
    trigger:
      branches:
      - main

stages:
- stage: 'Release'
  displayName: 'Release Stage'
  jobs:
  - job: 'CreateRelease'
    displayName: 'Create Release'
    pool:
      vmImage: 'ubuntu-latest'
    steps:
    - download: current
      artifact: 'build-artifacts'

    - task: Bash@3
      displayName: 'Final Security Validation'
      inputs:
        targetType: 'inline'
        script: |
          curl -L https://github.com/your-org/codeguardian/releases/download/latest/codeguardian-linux-x64.tar.gz | tar xz
          chmod +x codeguardian
          ./codeguardian check --fail-on-issues --baseline main-baseline.json --format json > release-validation.json

    - task: PublishBuildArtifacts@1
      displayName: 'Publish Release Validation'
      inputs:
        pathToPublish: 'release-validation.json'
        artifactName: 'ReleaseValidation'

    - task: GitHubRelease@1
      displayName: 'Create GitHub Release'
      inputs:
        gitHubConnection: 'github-connection'
        repositoryName: '$(Build.Repository.Name)'
        title: 'Release $(Build.BuildNumber)'
        releaseNotesFile: 'release-notes.md'
        assets: |
          $(Pipeline.Workspace)/build-artifacts/codeguardian
        addChangeLog: true
```

## Template Usage

### Pipeline Templates

```yaml
# templates/codeguardian-security.yml
parameters:
- name: 'failOnIssues'
  type: boolean
  default: true

- name: 'format'
  type: string
  default: 'json'

steps:
- task: Bash@3
  displayName: 'Setup CodeGuardian'
  inputs:
    targetType: 'inline'
    script: |
      curl -L https://github.com/your-org/codeguardian/releases/download/latest/codeguardian-linux-x64.tar.gz | tar xz
      chmod +x codeguardian
      echo "##vso[task.setvariable variable=PATH]$PWD:$PATH"

- task: Bash@3
  displayName: 'CodeGuardian Security Scan'
  inputs:
    targetType: 'inline'
    script: |
      ./codeguardian check ${{ parameters.failOnIssues }} --format ${{ parameters.format }} > codeguardian-results.${{ parameters.format }}
  continueOnError: ${{ parameters.failOnIssues }}

- task: PublishBuildArtifacts@1
  displayName: 'Publish CodeGuardian Results'
  inputs:
    pathToPublish: 'codeguardian-results.${{ parameters.format }}'
    artifactName: 'CodeGuardianResults'
  condition: always()
```

### Using Templates

```yaml
# azure-pipelines.yml
stages:
- stage: 'Security'
  displayName: 'Security Stage'
  jobs:
  - job: 'SecurityScan'
    displayName: 'Security Scan'
    pool:
      vmImage: 'ubuntu-latest'
    steps:
    - template: 'templates/codeguardian-security.yml'
      parameters:
        failOnIssues: true
        format: 'sarif'

    - task: PublishSecurityAnalysisLogs@3
      displayName: 'Publish Security Results'
      inputs:
        sarifFile: 'codeguardian-results.sarif'
```

## Best Practices

### Pipeline Optimization

- **Stage dependencies**: Use appropriate stage dependencies
- **Parallel jobs**: Run independent jobs in parallel
- **Caching**: Use Azure DevOps caching for dependencies
- **Artifact management**: Optimize artifact storage and retrieval
- **Resource management**: Use appropriate agent pools and sizing

### Security Considerations

- **Secret management**: Use Azure Key Vault for secrets
- **Access control**: Implement proper role-based access control
- **Audit logging**: Enable comprehensive audit logging
- **Vulnerability scanning**: Regular scanning of pipeline dependencies
- **Compliance**: Ensure compliance with security standards

### Performance Monitoring

- **Build metrics**: Track build times and resource usage
- **Trend analysis**: Monitor pipeline trends over time
- **Bottleneck identification**: Identify and resolve performance bottlenecks
- **Resource optimization**: Optimize resource allocation
- **Caching strategies**: Implement effective caching strategies

## Error Handling

### Common Issues

- **Agent availability**: Ensure required agent pools are available
  ```yaml
  pool:
    name: 'my-agent-pool'
  ```

- **Resource authorization**: Check pipeline permissions
  ```yaml
  # Ensure proper permissions for environments
  environment: 'production'
  ```

- **Artifact limits**: Monitor artifact size limits
  ```yaml
  # Use artifact filters to reduce size
  - task: PublishBuildArtifacts@1
    inputs:
      pathToPublish: '$(Build.ArtifactStagingDirectory)'
      artifactName: 'filtered-artifacts'
  ```

### Troubleshooting

1. **Check pipeline logs**:
   ```bash
   # View pipeline logs in Azure DevOps UI
   ```

2. **Validate pipeline syntax**:
   ```bash
   codeguardian ci-cd azure-devops validate
   ```

3. **Test pipeline locally**:
   ```bash
   # Use Azure DevOps CLI for local testing
   ```

4. **Monitor agent status**:
   ```bash
   # Check agent pool status in Azure DevOps
   ```

## Integration with CodeGuardian

### Automated Issue Creation

```yaml
# azure-pipelines.yml
steps:
- task: Bash@3
  displayName: 'Create Azure DevOps Work Items'
  inputs:
    targetType: 'inline'
    script: |
      ./codeguardian gh-issue \
        --from codeguardian-results.json \
        --repo $(System.TeamProject)/$(Build.Repository.Name) \
        --mode checklist \
        --title "Pipeline Build: Security Issues Found"
  condition: failed()
```

### Performance Regression Detection

```yaml
# azure-pipelines.yml
steps:
- task: Bash@3
  displayName: 'Performance Regression Check'
  inputs:
    targetType: 'inline'
    script: |
      ./codeguardian performance benchmark --output current-performance.json
      ./codeguardian performance compare \
        --baseline baseline-performance.json \
        --current current-performance.json \
        --threshold 10 \
        --output regression-report.json
  condition: always()

- task: PublishBuildArtifacts@1
  displayName: 'Publish Performance Report'
  inputs:
    pathToPublish: 'regression-report.json'
    artifactName: 'PerformanceReport'
  condition: always()
```

## See Also

- [`codeguardian ci-cd github-actions`](github-actions.md) - GitHub Actions integration
- [`codeguardian ci-cd gitlab-ci`](gitlab-ci.md) - GitLab CI/CD integration
- [`codeguardian check`](../../../commands/check.md) - Primary analysis command
- [Azure DevOps Pipelines Documentation](https://docs.microsoft.com/en-us/azure/devops/pipelines/)