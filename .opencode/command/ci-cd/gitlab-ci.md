---
title: "ci-cd gitlab-ci"
description: "Generate optimized GitLab CI/CD pipelines for CodeGuardian integration"
category: "CI/CD Platform"
tags: ["gitlab", "ci-cd", "pipelines", "automation", "containers"]
---

# ci-cd gitlab-ci

Generate comprehensive GitLab CI/CD pipelines for CodeGuardian integration, including automated security scanning, performance monitoring, and deployment automation with GitLab-specific optimizations.

## Synopsis

```bash
codeguardian ci-cd gitlab-ci [OPTIONS] [COMMAND]
```

## Description

The `ci-cd gitlab-ci` command creates optimized GitLab CI/CD pipeline configurations for CodeGuardian, supporting merge request validation, scheduled security scans, and multi-environment deployments with GitLab-specific features.

### Key Features

- **Pipeline optimization**: Generate efficient GitLab CI/CD pipelines
- **Security integration**: Automated security scanning and compliance
- **Multi-environment support**: Development, staging, and production pipelines
- **GitLab features**: Leverage GitLab-specific features and integrations
- **Performance monitoring**: Build performance tracking and optimization

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--pipeline-type` | Type of pipeline (merge,main,scheduled,release) | string | merge | No |
| `--output-file` | Output file path | string | .gitlab-ci.yml | No |
| `--include-security` | Include security scanning jobs | boolean | true | No |
| `--include-performance` | Include performance monitoring | boolean | false | No |
| `--environments` | Target environments | string[] | dev,staging,prod | No |
| `--cache-enabled` | Enable build caching | boolean | true | No |
| `--docker-enabled` | Enable Docker-in-Docker support | boolean | false | No |
| `--kubernetes` | Enable Kubernetes deployment | boolean | false | No |

## Commands

### generate
Generate GitLab CI/CD pipeline configuration.

```bash
codeguardian ci-cd gitlab-ci generate [OPTIONS]
```

### validate
Validate existing GitLab CI/CD configuration.

```bash
codeguardian ci-cd gitlab-ci validate [OPTIONS]
```

### optimize
Optimize existing GitLab CI/CD pipelines.

```bash
codeguardian ci-cd gitlab-ci optimize [OPTIONS]
```

### monitor
Monitor GitLab CI/CD pipeline performance.

```bash
codeguardian ci-cd gitlab-ci monitor [OPTIONS]
```

## Examples

### Generate Merge Request Pipeline

```bash
# Generate merge request validation pipeline
codeguardian ci-cd gitlab-ci generate \
  --pipeline-type merge \
  --include-security \
  --cache-enabled
```

### Generate Main Branch Pipeline

```bash
# Generate main branch deployment pipeline
codeguardian ci-cd gitlab-ci generate \
  --pipeline-type main \
  --environments dev,staging,prod \
  --kubernetes
```

### Generate Scheduled Security Pipeline

```bash
# Generate scheduled security scanning pipeline
codeguardian ci-cd gitlab-ci generate \
  --pipeline-type scheduled \
  --include-performance
```

### Validate Pipeline Configuration

```bash
# Validate existing .gitlab-ci.yml
codeguardian ci-cd gitlab-ci validate \
  --output-file .gitlab-ci.yml
```

### Optimize Pipeline Performance

```bash
# Optimize existing pipeline for performance
codeguardian ci-cd gitlab-ci optimize \
  --output-file .gitlab-ci.yml
```

## Merge Request Pipeline

### Basic Merge Request Validation

```yaml
# .gitlab-ci.yml
stages:
  - validate
  - test
  - security
  - deploy

variables:
  CODEGUARDIAN_VERSION: "latest"
  DOCKER_DRIVER: overlay2

cache:
  key: ${CI_COMMIT_REF_SLUG}
  paths:
    - .cargo/
    - target/

validate:codeguardian:
  stage: validate
  image: rust:latest
  before_script:
    - apt-get update && apt-get install -y curl
    - curl -L https://github.com/your-org/codeguardian/releases/download/${CODEGUARDIAN_VERSION}/codeguardian-linux-x64.tar.gz | tar xz
    - chmod +x codeguardian
  script:
    - ./codeguardian check --fail-on-issues --format json > codeguardian-results.json
  artifacts:
    reports:
      junit: codeguardian-results.json
    paths:
      - codeguardian-results.json
    expire_in: 1 week
  only:
    - merge_requests

test:unit:
  stage: test
  image: rust:latest
  script:
    - cargo test --verbose
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths:
      - .cargo/
      - target/
  only:
    - merge_requests

security:scan:
  stage: security
  image: rust:latest
  script:
    - ./codeguardian check --fail-on-issues --format sarif > security-results.sarif
  artifacts:
    reports:
      sast: security-results.sarif
    paths:
      - security-results.sarif
    expire_in: 1 week
  only:
    - merge_requests

deploy:review:
  stage: deploy
  script:
    - echo "Deploying to review environment"
    - ./codeguardian check --format markdown > review-report.md
  environment:
    name: review/$CI_COMMIT_REF_NAME
    url: https://review-${CI_COMMIT_REF_SLUG}.example.com
  only:
    - merge_requests
  dependencies:
    - validate:codeguardian
```

### Advanced Merge Request Pipeline

```yaml
# .gitlab-ci.yml
stages:
  - validate
  - test
  - security
  - performance
  - deploy

variables:
  CODEGUARDIAN_VERSION: "latest"
  DOCKER_TLS_CERTDIR: "/certs"

cache:
  key: ${CI_COMMIT_REF_SLUG}
  paths:
    - .cargo/
    - target/
    - node_modules/

validate:codeguardian:
  stage: validate
  image: rust:latest
  services:
    - docker:dind
  variables:
    DOCKER_HOST: tcp://docker:2376
    DOCKER_TLS_CERTDIR: "/certs"
  before_script:
    - docker info
    - apt-get update && apt-get install -y curl
    - curl -L https://github.com/your-org/codeguardian/releases/download/${CODEGUARDIAN_VERSION}/codeguardian-linux-x64.tar.gz | tar xz
    - chmod +x codeguardian
  script:
    - ./codeguardian check --fail-on-issues --parallel 4 --format json > codeguardian-results.json
  artifacts:
    reports:
      junit: codeguardian-results.json
    paths:
      - codeguardian-results.json
    expire_in: 1 week
  coverage: '/TOTAL.*\s+(\d+%)$/'
  only:
    - merge_requests

test:integration:
  stage: test
  image: rust:latest
  services:
    - postgres:15-alpine
    - redis:7-alpine
  variables:
    POSTGRES_DB: test_db
    POSTGRES_USER: test_user
    POSTGRES_PASSWORD: test_password
    REDIS_URL: redis://redis:6379
  script:
    - cargo test --test integration --verbose
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths:
      - .cargo/
      - target/
  only:
    - merge_requests

security:comprehensive:
  stage: security
  image: rust:latest
  script:
    - ./codeguardian check --fail-on-issues --include "src/**" --exclude "tests/**" --format sarif > security-results.sarif
    - ./codeguardian check --baseline baseline.json --drift-analysis --format json > drift-results.json
  artifacts:
    reports:
      sast: security-results.sarif
    paths:
      - security-results.sarif
      - drift-results.json
    expire_in: 1 week
  only:
    - merge_requests

performance:benchmark:
  stage: performance
  image: rust:latest
  script:
    - cargo bench > benchmark-results.txt
    - ./codeguardian performance benchmark --input benchmark-results.txt --output performance-report.json
  artifacts:
    paths:
      - benchmark-results.txt
      - performance-report.json
    expire_in: 1 week
  only:
    - merge_requests

deploy:staging:
  stage: deploy
  script:
    - echo "Deploying to staging environment"
    - ./codeguardian check --format markdown > staging-report.md
  environment:
    name: staging
    url: https://staging.example.com
  only:
    refs:
      - merge_requests
    variables:
      - $DEPLOY_STAGING == "true"
  dependencies:
    - validate:codeguardian
    - test:integration
    - security:comprehensive
```

## Main Branch Pipeline

### Production Deployment Pipeline

```yaml
# .gitlab-ci.yml
stages:
  - build
  - test
  - security
  - deploy

variables:
  CODEGUARDIAN_VERSION: "latest"

build:application:
  stage: build
  image: rust:latest
  script:
    - cargo build --release
    - cargo test --release
  artifacts:
    paths:
      - target/release/codeguardian
    expire_in: 1 hour
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths:
      - .cargo/
      - target/
  only:
    - main

test:comprehensive:
  stage: test
  image: rust:latest
  services:
    - postgres:15-alpine
    - redis:7-alpine
  variables:
    POSTGRES_DB: test_db
    POSTGRES_USER: test_user
    POSTGRES_PASSWORD: test_password
    REDIS_URL: redis://redis:6379
  script:
    - cargo test --verbose
    - cargo test --test integration --verbose
    - cargo test --doc
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths:
      - .cargo/
      - target/
  artifacts:
    reports:
      junit: test-results.xml
    paths:
      - test-results.xml
    expire_in: 1 week
  coverage: '/TOTAL.*\s+(\d+%)$/'
  only:
    - main

security:final:
  stage: security
  image: rust:latest
  script:
    - ./codeguardian check --fail-on-issues --format sarif > security-final.sarif
    - ./codeguardian check --baseline main-baseline.json --format json > baseline-comparison.json
  artifacts:
    reports:
      sast: security-final.sarif
    paths:
      - security-final.sarif
      - baseline-comparison.json
    expire_in: 1 week
  only:
    - main

deploy:production:
  stage: deploy
  image: alpine:latest
  before_script:
    - apk add --no-cache openssh-client
    - eval $(ssh-agent -s)
    - echo "$SSH_PRIVATE_KEY" | tr -d '\r' | ssh-add -
    - mkdir -p ~/.ssh
    - chmod 700 ~/.ssh
    - ssh-keyscan -H $DEPLOY_HOST >> ~/.ssh/known_hosts
  script:
    - echo "Deploying to production"
    - scp target/release/codeguardian user@$DEPLOY_HOST:/opt/codeguardian/
    - ssh user@$DEPLOY_HOST "sudo systemctl restart codeguardian"
  environment:
    name: production
    url: https://codeguardian.example.com
  only:
    - main
  when: manual
  dependencies:
    - build:application
    - test:comprehensive
    - security:final
```

## Scheduled Security Pipeline

### Weekly Security Audit

```yaml
# .gitlab-ci.yml
stages:
  - audit
  - report

variables:
  CODEGUARDIAN_VERSION: "latest"

audit:weekly:
  stage: audit
  image: rust:latest
  only:
    - schedules
  script:
    - apt-get update && apt-get install -y curl
    - curl -L https://github.com/your-org/codeguardian/releases/download/${CODEGUARDIAN_VERSION}/codeguardian-linux-x64.tar.gz | tar xz
    - chmod +x codeguardian
    - ./codeguardian check --fail-on-issues --include "src/**" --exclude "tests/**" --format sarif > weekly-audit.sarif
    - ./codeguardian check --format json --output weekly-results.json
  artifacts:
    reports:
      sast: weekly-audit.sarif
    paths:
      - weekly-audit.sarif
      - weekly-results.json
    expire_in: 1 month

report:weekly:
  stage: report
  image: alpine:latest
  only:
    - schedules
  before_script:
    - apk add --no-cache curl
  script:
    - ./codeguardian report --from weekly-results.json --format markdown --output weekly-report.md
    - |
      curl -X POST -H "Content-Type: application/json" \
        -d "{\"text\":\"Weekly Security Audit Complete\",\"attachments\":[{\"text\":\"$(cat weekly-report.md)\"}]}" \
        $SLACK_WEBHOOK_URL
  dependencies:
    - audit:weekly
```

## Release Pipeline

### Automated Release

```yaml
# .gitlab-ci.yml
stages:
  - build
  - test
  - release

variables:
  CODEGUARDIAN_VERSION: "latest"

build:release:
  stage: build
  image: rust:latest
  script:
    - cargo build --release
    - cargo test --release
  artifacts:
    paths:
      - target/release/codeguardian
    expire_in: 1 hour
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths:
      - .cargo/
      - target/
  only:
    - tags

test:release:
  stage: test
  image: rust:latest
  script:
    - cargo test --release --verbose
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths:
      - .cargo/
      - target/
  only:
    - tags

release:create:
  stage: release
  image: registry.gitlab.com/gitlab-org/release-cli:latest
  script:
    - ./codeguardian check --format markdown > release-notes.md
    - |
      release-cli create --name "Release $CI_COMMIT_TAG" \
        --description "Automated release of CodeGuardian $CI_COMMIT_TAG" \
        --tag-name $CI_COMMIT_TAG \
        --assets-link "{\"name\":\"codeguardian-linux-x64\",\"url\":\"https://gitlab.com/your-org/codeguardian/-/jobs/$CI_JOB_ID/artifacts/download?file_type=archive\"}"
  only:
    - tags
  dependencies:
    - build:release
    - test:release
```

## Performance Monitoring Pipeline

### Build Performance Tracking

```yaml
# .gitlab-ci.yml
stages:
  - build
  - performance
  - report

variables:
  CODEGUARDIAN_VERSION: "latest"

build:with-timing:
  stage: build
  image: rust:latest
  script:
    - echo "BUILD_START=$(date +%s)" > build-timing.env
    - cargo build --release --timings
    - echo "BUILD_END=$(date +%s)" >> build-timing.env
  artifacts:
    paths:
      - target/release/codeguardian
      - cargo-timing.html
      - build-timing.env
    expire_in: 1 hour
  cache:
    key: ${CI_COMMIT_REF_SLUG}
    paths:
      - .cargo/
      - target/
  only:
    - main
    - merge_requests

performance:analyze:
  stage: performance
  image: rust:latest
  script:
    - source build-timing.env
    - BUILD_TIME=$((BUILD_END - BUILD_START))
    - echo "Build time: ${BUILD_TIME}s"
    - cargo bench > benchmark-results.txt
    - ./codeguardian performance benchmark --input benchmark-results.txt --output performance-report.json
  artifacts:
    paths:
      - benchmark-results.txt
      - performance-report.json
    expire_in: 1 week
  dependencies:
    - build:with-timing
  only:
    - main
    - merge_requests

report:performance:
  stage: report
  image: alpine:latest
  script:
    - apk add --no-cache curl
    - |
      curl -X POST -H "Content-Type: application/json" \
        -d "{\"text\":\"Performance Report\",\"attachments\":[{\"text\":\"Build completed in $(source build-timing.env && echo $((BUILD_END - BUILD_START)))s\"}]}" \
        $SLACK_WEBHOOK_URL
  dependencies:
    - performance:analyze
  only:
    - main
  when: always
```

## Best Practices

### Security Considerations

- **Secret management**: Use GitLab CI/CD variables for secrets
- **Container security**: Scan container images for vulnerabilities
- **Access control**: Implement proper role-based access control
- **Audit logging**: Enable comprehensive audit logging
- **Compliance**: Ensure compliance with security standards

### Performance Optimization

- **Caching strategies**: Use GitLab cache for dependencies
- **Parallel execution**: Run jobs in parallel when possible
- **Resource optimization**: Use appropriate runner sizes
- **Artifact management**: Clean up old artifacts regularly
- **Pipeline efficiency**: Optimize pipeline structure

### GitLab Features

- **Environments**: Use GitLab environments for deployment
- **Review apps**: Implement review apps for merge requests
- **Auto DevOps**: Leverage GitLab Auto DevOps features
- **Security dashboard**: Integrate with GitLab security dashboard
- **Compliance pipelines**: Implement compliance pipeline features

## Error Handling

### Common Issues

- **Runner configuration**: Ensure proper runner setup
  ```yaml
  # Use specific runners
  tags:
    - docker
    - linux
  ```

- **Service dependencies**: Configure service dependencies correctly
  ```yaml
  services:
    - postgres:15-alpine
  variables:
    POSTGRES_DB: test_db
  ```

- **Artifact expiration**: Set appropriate artifact expiration
  ```yaml
  artifacts:
    expire_in: 1 week
  ```

### Troubleshooting

1. **Check pipeline status**:
   ```bash
   # View pipeline status in GitLab UI
   ```

2. **Validate pipeline syntax**:
   ```bash
   codeguardian ci-cd gitlab-ci validate
   ```

3. **Test pipeline locally**:
   ```bash
   # Use gitlab-ci-pipelines-exporter for local testing
   ```

4. **Monitor resource usage**:
   ```bash
   # Check runner resource usage in pipeline logs
   ```

## Integration with CodeGuardian

### Automated Issue Creation

```yaml
# .gitlab-ci.yml
stages:
  - security
  - report

security:scan:
  stage: security
  script:
    - ./codeguardian check --emit-gh --repo $CI_PROJECT_PATH --format json > security-results.json
  artifacts:
    paths:
      - security-results.json
    expire_in: 1 week
  only:
    - main

report:create-issue:
  stage: report
  image: alpine:latest
  before_script:
    - apk add --no-cache curl
  script:
    - ./codeguardian gh-issue --from security-results.json --repo $CI_PROJECT_PATH --mode checklist
  dependencies:
    - security:scan
  only:
    - main
```

### Performance Regression Detection

```yaml
# .gitlab-ci.yml
stages:
  - performance
  - compare

performance:benchmark:
  stage: performance
  script:
    - ./codeguardian performance benchmark --output current-performance.json
  artifacts:
    paths:
      - current-performance.json
    expire_in: 1 week

compare:performance:
  stage: compare
  script:
    - ./codeguardian performance compare --baseline baseline-performance.json --current current-performance.json --threshold 10
  dependencies:
    - performance:benchmark
  only:
    - main
```

## See Also

- [`codeguardian ci-cd github-actions`](github-actions.md) - GitHub Actions integration
- [`codeguardian ci-cd jenkins`](jenkins.md) - Jenkins pipeline integration
- [`codeguardian check`](../../../commands/check.md) - Primary analysis command
- [GitLab CI/CD Documentation](https://docs.gitlab.com/ee/ci/)