---
title: "ci-cd jenkins"
description: "Generate Jenkins pipeline configurations for CodeGuardian integration"
category: "CI/CD Platform"
tags: ["jenkins", "pipeline", "ci-cd", "automation", "groovy"]
---

# ci-cd jenkins

Generate Jenkins pipeline configurations for CodeGuardian integration, including declarative and scripted pipelines with security scanning, performance monitoring, and deployment automation.

## Synopsis

```bash
codeguardian ci-cd jenkins [OPTIONS] [COMMAND]
```

## Description

The `ci-cd jenkins` command creates optimized Jenkins pipeline configurations for CodeGuardian, supporting both declarative and scripted pipeline syntax with comprehensive CI/CD workflows.

### Key Features

- **Pipeline generation**: Create declarative and scripted Jenkins pipelines
- **Security integration**: Automated security scanning and compliance
- **Multi-branch support**: Support for multi-branch pipeline projects
- **Performance monitoring**: Build performance tracking and optimization
- **Artifact management**: Comprehensive artifact handling and storage

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--pipeline-type` | Type of pipeline (declarative,scripted,multibranch) | string | declarative | No |
| `--output-file` | Output file path | string | Jenkinsfile | No |
| `--include-security` | Include security scanning stages | boolean | true | No |
| `--include-performance` | Include performance monitoring | boolean | false | No |
| `--agents` | Jenkins agent labels | string[] | linux,docker | No |
| `--stages` | Pipeline stages to include | string[] | build,test,security,deploy | No |
| `--credentials` | Jenkins credential IDs | string[] | [] | No |
| `--artifacts` | Artifact patterns to archive | string[] | "**/target/**" | No |

## Commands

### generate
Generate Jenkins pipeline configuration.

```bash
codeguardian ci-cd jenkins generate [OPTIONS]
```

### validate
Validate existing Jenkins pipeline.

```bash
codeguardian ci-cd jenkins validate [OPTIONS]
```

### optimize
Optimize existing Jenkins pipeline.

```bash
codeguardian ci-cd jenkins optimize [OPTIONS]
```

### convert
Convert between pipeline types.

```bash
codeguardian ci-cd jenkins convert [OPTIONS]
```

## Examples

### Generate Declarative Pipeline

```bash
# Generate declarative Jenkins pipeline
codeguardian ci-cd jenkins generate \
  --pipeline-type declarative \
  --include-security \
  --agents linux,docker
```

### Generate Scripted Pipeline

```bash
# Generate scripted Jenkins pipeline
codeguardian ci-cd jenkins generate \
  --pipeline-type scripted \
  --include-performance \
  --stages build,test,security,deploy
```

### Generate Multi-branch Pipeline

```bash
# Generate multi-branch pipeline
codeguardian ci-cd jenkins generate \
  --pipeline-type multibranch \
  --include-security \
  --include-performance
```

### Validate Pipeline

```bash
# Validate existing Jenkinsfile
codeguardian ci-cd jenkins validate \
  --output-file Jenkinsfile
```

### Convert Pipeline Type

```bash
# Convert declarative to scripted
codeguardian ci-cd jenkins convert \
  --from declarative \
  --to scripted \
  --input-file Jenkinsfile \
  --output-file Jenkinsfile.scripted
```

## Declarative Pipeline

### Basic Declarative Pipeline

```groovy
// Jenkinsfile
pipeline {
    agent {
        label 'linux && docker'
    }

    environment {
        CODEGUARDIAN_VERSION = 'latest'
        DOCKER_IMAGE = 'codeguardian:latest'
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Setup') {
            steps {
                sh '''
                    # Install CodeGuardian
                    curl -L https://github.com/your-org/codeguardian/releases/download/${CODEGUARDIAN_VERSION}/codeguardian-linux-x64.tar.gz | tar xz
                    chmod +x codeguardian
                    export PATH=$PWD:$PATH
                '''
            }
        }

        stage('CodeGuardian Security Scan') {
            steps {
                sh '''
                    ./codeguardian check --fail-on-issues --format json > codeguardian-results.json
                '''
            }
            post {
                always {
                    archiveArtifacts artifacts: 'codeguardian-results.json', fingerprint: true
                    junit 'codeguardian-results.json'
                }
                failure {
                    script {
                        def results = readJSON file: 'codeguardian-results.json'
                        echo "Security scan failed with ${results.issues.size()} issues"
                    }
                }
            }
        }

        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
            post {
                success {
                    archiveArtifacts artifacts: 'target/release/codeguardian', fingerprint: true
                }
            }
        }

        stage('Test') {
            steps {
                sh 'cargo test --verbose'
            }
            post {
                always {
                    junit 'target/debug/deps/*.xml'
                }
            }
        }
    }

    post {
        always {
            sh 'docker system prune -f'
            cleanWs()
        }
        success {
            echo 'Pipeline succeeded!'
        }
        failure {
            echo 'Pipeline failed!'
            // Send notifications
            emailext subject: "Build failed: ${currentBuild.fullDisplayName}",
                     body: "Build failed. Check ${env.BUILD_URL} for details.",
                     recipientProviders: [culprits(), developers()]
        }
    }
}
```

### Advanced Declarative Pipeline

```groovy
// Jenkinsfile
pipeline {
    agent none

    environment {
        CODEGUARDIAN_VERSION = 'latest'
        DOCKER_REGISTRY = 'your-registry.com'
        DEPLOY_ENV = 'staging'
    }

    parameters {
        choice(name: 'ENVIRONMENT', choices: ['dev', 'staging', 'prod'], description: 'Deployment environment')
        booleanParam(name: 'RUN_PERFORMANCE_TESTS', defaultValue: true, description: 'Run performance tests')
        string(name: 'CODEGUARDIAN_CONFIG', defaultValue: 'codeguardian.toml', description: 'CodeGuardian config file')
    }

    stages {
        stage('Parallel Analysis') {
            parallel {
                stage('Security Scan') {
                    agent {
                        label 'security-scanner'
                    }
                    steps {
                        checkout scm
                        script {
                            sh '''
                                curl -L https://github.com/your-org/codeguardian/releases/download/${CODEGUARDIAN_VERSION}/codeguardian-linux-x64.tar.gz | tar xz
                                chmod +x codeguardian
                                ./codeguardian check --fail-on-issues --format sarif > security-results.sarif
                            '''
                            def sarif = readFile file: 'security-results.sarif', encoding: 'UTF-8'
                            recordIssues tools: [sarif(pattern: 'security-results.sarif')]
                        }
                    }
                    post {
                        always {
                            archiveArtifacts artifacts: 'security-results.sarif', fingerprint: true
                        }
                    }
                }

                stage('Code Quality') {
                    agent {
                        label 'code-quality'
                    }
                    steps {
                        checkout scm
                        sh '''
                            cargo clippy -- -D warnings
                            cargo fmt --check
                        '''
                    }
                }

                stage('Unit Tests') {
                    agent {
                        label 'linux && docker'
                    }
                    steps {
                        checkout scm
                        sh '''
                            cargo test --verbose -- --nocapture
                        '''
                    }
                    post {
                        always {
                            junit 'target/debug/deps/*.xml'
                        }
                    }
                }
            }
        }

        stage('Performance Tests') {
            when {
                expression { params.RUN_PERFORMANCE_TESTS }
            }
            agent {
                label 'performance'
            }
            steps {
                checkout scm
                sh '''
                    cargo bench > benchmark-results.txt
                    ./codeguardian performance benchmark --input benchmark-results.txt --output performance-report.json
                '''
            }
            post {
                always {
                    archiveArtifacts artifacts: 'benchmark-results.txt,performance-report.json', fingerprint: true
                    plot csvFileName: 'performance-trend.csv',
                         csvSeries: [[file: 'performance-report.json', displayTableFlag: false, exclusionValues: '', fileType: 'json', inclusionFlag: 'INCLUDE_BY_STRING', url: '']],
                         group: 'Performance Trends',
                         title: 'Build Performance',
                         style: 'line'
                }
            }
        }

        stage('Build & Package') {
            agent {
                label 'docker'
            }
            steps {
                checkout scm
                script {
                    docker.build("${DOCKER_REGISTRY}/codeguardian:${env.BUILD_NUMBER}")
                }
            }
        }

        stage('Deploy') {
            when {
                expression { currentBuild.result == null || currentBuild.result == 'SUCCESS' }
            }
            agent {
                label 'deploy'
            }
            steps {
                script {
                    if (params.ENVIRONMENT == 'prod') {
                        timeout(time: 15, unit: 'MINUTES') {
                            input message: 'Deploy to production?', ok: 'Deploy'
                        }
                    }
                    sh """
                        echo "Deploying to ${params.ENVIRONMENT}"
                        kubectl set image deployment/codeguardian codeguardian=${DOCKER_REGISTRY}/codeguardian:${env.BUILD_NUMBER}
                        kubectl rollout status deployment/codeguardian
                    """
                }
            }
        }
    }

    triggers {
        pollSCM('H/15 * * * *')
        cron('@weekly')
    }

    options {
        buildDiscarder(logRotator(numToKeepStr: '10'))
        timeout(time: 60, unit: 'MINUTES')
        disableConcurrentBuilds()
    }

    post {
        always {
            sh 'docker system prune -f'
            cleanWs()
        }
        success {
            script {
                if (env.BRANCH_NAME == 'main') {
                    build job: 'downstream-deployment', wait: false
                }
            }
        }
        unstable {
            emailext subject: "Build unstable: ${currentBuild.fullDisplayName}",
                     body: "Build unstable. Check ${env.BUILD_URL} for details.",
                     recipientProviders: [culprits(), developers()]
        }
        failure {
            emailext subject: "Build failed: ${currentBuild.fullDisplayName}",
                     body: "Build failed. Check ${env.BUILD_URL} for details.",
                     recipientProviders: [culprits(), developers(), requestor()]
        }
    }
}
```

## Scripted Pipeline

### Basic Scripted Pipeline

```groovy
// Jenkinsfile
node('linux && docker') {
    try {
        stage('Checkout') {
            checkout scm
        }

        stage('Setup') {
            sh '''
                # Install CodeGuardian
                curl -L https://github.com/your-org/codeguardian/releases/download/latest/codeguardian-linux-x64.tar.gz | tar xz
                chmod +x codeguardian
                export PATH=$PWD:$PATH
            '''
        }

        stage('Security Scan') {
            sh '''
                ./codeguardian check --fail-on-issues --format json > codeguardian-results.json
            '''
            archiveArtifacts artifacts: 'codeguardian-results.json', fingerprint: true
        }

        stage('Build') {
            sh 'cargo build --release'
            archiveArtifacts artifacts: 'target/release/codeguardian', fingerprint: true
        }

        stage('Test') {
            sh 'cargo test --verbose'
        }

        currentBuild.result = 'SUCCESS'

    } catch (Exception e) {
        currentBuild.result = 'FAILURE'
        throw e
    } finally {
        sh 'docker system prune -f'
        cleanWs()
    }
}
```

### Advanced Scripted Pipeline

```groovy
// Jenkinsfile
def call(Map config = [:]) {
    node(config.agent ?: 'linux && docker') {
        def codeguardianHome = pwd(tmp: true)

        try {
            stage('Checkout') {
                checkout scm
            }

            stage('Environment Setup') {
                sh """
                    curl -L https://github.com/your-org/codeguardian/releases/download/${config.codeguardianVersion ?: 'latest'}/codeguardian-linux-x64.tar.gz | tar xz
                    chmod +x codeguardian
                    export PATH=\$PWD:\$PATH
                """
            }

            stage('Parallel Analysis') {
                parallel(
                    'Security Scan': {
                        sh """
                            ./codeguardian check --fail-on-issues --format sarif > security-results.sarif
                        """
                        archiveArtifacts artifacts: 'security-results.sarif', fingerprint: true
                        recordIssues tools: [sarif(pattern: 'security-results.sarif')]
                    },
                    'Code Quality': {
                        sh """
                            cargo clippy -- -D warnings
                            cargo fmt --check
                        """
                    },
                    'Unit Tests': {
                        sh """
                            cargo test --verbose -- --nocapture
                        """
                        junit 'target/debug/deps/*.xml'
                    }
                )
            }

            if (config.runPerformanceTests) {
                stage('Performance Tests') {
                    sh """
                        cargo bench > benchmark-results.txt
                        ./codeguardian performance benchmark --input benchmark-results.txt --output performance-report.json
                    """
                    archiveArtifacts artifacts: 'benchmark-results.txt,performance-report.json', fingerprint: true
                }
            }

            stage('Build & Package') {
                sh 'cargo build --release'
                archiveArtifacts artifacts: 'target/release/codeguardian', fingerprint: true
            }

            if (config.deploy) {
                stage('Deploy') {
                    def deployEnv = config.environment ?: 'staging'

                    if (deployEnv == 'prod') {
                        timeout(time: 15, unit: 'MINUTES') {
                            input message: 'Deploy to production?', ok: 'Deploy'
                        }
                    }

                    sh """
                        echo "Deploying to ${deployEnv}"
                        # Deployment logic here
                    """
                }
            }

            currentBuild.result = 'SUCCESS'

        } catch (Exception e) {
            currentBuild.result = 'FAILURE'
            emailext subject: "Build failed: ${currentBuild.fullDisplayName}",
                     body: "Build failed. Check ${env.BUILD_URL} for details.",
                     recipientProviders: [culprits(), developers()]
            throw e
        } finally {
            sh 'docker system prune -f'
            cleanWs()
        }
    }
}

// Usage
call(
    agent: 'linux && docker',
    codeguardianVersion: 'v1.0.0',
    runPerformanceTests: true,
    deploy: true,
    environment: 'staging'
)
```

## Multi-branch Pipeline

### Multi-branch Pipeline Configuration

```groovy
// Jenkinsfile
def call(Map config = [:]) {
    properties([
        buildDiscarder(logRotator(numToKeepStr: '10')),
        parameters([
            choice(name: 'ENVIRONMENT', choices: ['dev', 'staging', 'prod'], description: 'Deployment environment'),
            booleanParam(name: 'RUN_SECURITY_SCAN', defaultValue: true, description: 'Run security scan'),
            booleanParam(name: 'RUN_PERFORMANCE_TESTS', defaultValue: false, description: 'Run performance tests')
        ])
    ])

    def isMainBranch = env.BRANCH_NAME == 'main'
    def isPR = env.CHANGE_ID != null

    node('linux && docker') {
        try {
            stage('Checkout') {
                checkout scm
            }

            stage('Setup') {
                sh """
                    curl -L https://github.com/your-org/codeguardian/releases/download/latest/codeguardian-linux-x64.tar.gz | tar xz
                    chmod +x codeguardian
                    export PATH=\$PWD:\$PATH
                """
            }

            if (params.RUN_SECURITY_SCAN) {
                stage('Security Scan') {
                    sh """
                        ./codeguardian check --fail-on-issues --format json > codeguardian-results.json
                    """
                    archiveArtifacts artifacts: 'codeguardian-results.json', fingerprint: true
                    junit 'codeguardian-results.json'
                }
            }

            stage('Build') {
                sh 'cargo build --release'
                archiveArtifacts artifacts: 'target/release/codeguardian', fingerprint: true
            }

            stage('Test') {
                sh 'cargo test --verbose'
                junit 'target/debug/deps/*.xml'
            }

            if (params.RUN_PERFORMANCE_TESTS && isMainBranch) {
                stage('Performance Tests') {
                    sh """
                        cargo bench > benchmark-results.txt
                        ./codeguardian performance benchmark --input benchmark-results.txt --output performance-report.json
                    """
                    archiveArtifacts artifacts: 'benchmark-results.txt,performance-report.json', fingerprint: true
                }
            }

            if (isMainBranch) {
                stage('Deploy') {
                    def deployEnv = params.ENVIRONMENT ?: 'staging'

                    if (deployEnv == 'prod') {
                        timeout(time: 15, unit: 'MINUTES') {
                            input message: 'Deploy to production?', ok: 'Deploy'
                        }
                    }

                    sh """
                        echo "Deploying to ${deployEnv}"
                        # Deployment logic here
                    """
                }
            }

            currentBuild.result = 'SUCCESS'

        } catch (Exception e) {
            currentBuild.result = 'FAILURE'
            throw e
        } finally {
            sh 'docker system prune -f'
            cleanWs()
        }
    }
}
```

## Shared Library Integration

### Jenkins Shared Library

```groovy
// vars/codeguardian.groovy
def call(Map config = [:]) {
    def defaultConfig = [
        version: 'latest',
        failOnIssues: true,
        format: 'json',
        output: 'codeguardian-results.json'
    ]

    def finalConfig = defaultConfig + config

    sh """
        curl -L https://github.com/your-org/codeguardian/releases/download/${finalConfig.version}/codeguardian-linux-x64.tar.gz | tar xz
        chmod +x codeguardian
        ./codeguardian check \
            ${finalConfig.failOnIssues ? '--fail-on-issues' : ''} \
            --format ${finalConfig.format} \
            > ${finalConfig.output}
    """

    if (finalConfig.archive) {
        archiveArtifacts artifacts: finalConfig.output, fingerprint: true
    }

    if (finalConfig.junit) {
        junit finalConfig.output
    }

    return finalConfig.output
}

// Usage in Jenkinsfile
stage('Security Scan') {
    def results = codeguardian(
        version: 'v1.0.0',
        format: 'sarif',
        output: 'security-results.sarif',
        archive: true
    )
    recordIssues tools: [sarif(pattern: results)]
}
```

## Best Practices

### Pipeline Optimization

- **Parallel execution**: Run independent stages in parallel
- **Caching**: Use Jenkins caching for dependencies
- **Resource management**: Optimize agent usage and resource allocation
- **Artifact management**: Clean up old artifacts regularly
- **Pipeline as code**: Keep pipeline configuration in version control

### Security Considerations

- **Credential management**: Use Jenkins credentials for secrets
- **Agent security**: Secure Jenkins agents and workspaces
- **Access control**: Implement proper role-based access control
- **Audit logging**: Enable comprehensive audit logging
- **Vulnerability scanning**: Regular scanning of Jenkins and plugins

### Performance Monitoring

- **Build metrics**: Track build times and resource usage
- **Trend analysis**: Monitor build trends over time
- **Bottleneck identification**: Identify and resolve performance bottlenecks
- **Resource optimization**: Optimize resource allocation
- **Caching strategies**: Implement effective caching strategies

## Error Handling

### Common Issues

- **Agent availability**: Ensure required agents are available
  ```groovy
  agent {
      label 'linux && docker'
  }
  ```

- **Plugin dependencies**: Ensure required plugins are installed
  ```groovy
  // Check for required plugins
  def requiredPlugins = ['git', 'docker', 'junit']
  ```

- **Resource constraints**: Monitor and manage resource usage
  ```groovy
  options {
      timeout(time: 60, unit: 'MINUTES')
  }
  ```

### Troubleshooting

1. **Check build logs**:
   ```bash
   # View build logs in Jenkins UI
   ```

2. **Validate pipeline syntax**:
   ```bash
   codeguardian ci-cd jenkins validate
   ```

3. **Test pipeline locally**:
   ```bash
   # Use Jenkins Pipeline Unit for local testing
   ```

4. **Monitor agent status**:
   ```bash
   # Check agent status in Jenkins UI
   ```

## Integration with CodeGuardian

### Automated Issue Creation

```groovy
// Jenkinsfile
stage('Create GitHub Issues') {
    when {
        expression { currentBuild.result == 'FAILURE' }
    }
    steps {
        script {
            withCredentials([string(credentialsId: 'github-token', variable: 'GITHUB_TOKEN')]) {
                sh """
                    ./codeguardian gh-issue \
                        --from codeguardian-results.json \
                        --repo your-org/your-repo \
                        --mode checklist \
                        --title "Jenkins Build Failed: Security Issues Found"
                """
            }
        }
    }
}
```

### Performance Regression Detection

```groovy
// Jenkinsfile
stage('Performance Regression Check') {
    steps {
        script {
            sh """
                ./codeguardian performance benchmark --output current-performance.json
                ./codeguardian performance compare \
                    --baseline baseline-performance.json \
                    --current current-performance.json \
                    --threshold 10 \
                    --output regression-report.json
            """

            def regression = readJSON file: 'regression-report.json'
            if (regression.regression_detected) {
                unstable('Performance regression detected')
            }
        }
    }
    post {
        always {
            archiveArtifacts artifacts: 'regression-report.json', fingerprint: true
        }
    }
}
```

## See Also

- [`codeguardian ci-cd github-actions`](github-actions.md) - GitHub Actions integration
- [`codeguardian ci-cd gitlab-ci`](gitlab-ci.md) - GitLab CI/CD integration
- [`codeguardian check`](../../../commands/check.md) - Primary analysis command
- [Jenkins Pipeline Documentation](https://www.jenkins.io/doc/book/pipeline/)