---
title: "docker-build-optimize"
description: "Optimize Docker builds for CodeGuardian with multi-stage builds and caching"
category: "Docker/Container"
tags: ["docker", "build", "optimization", "multi-stage", "caching"]
---

# docker-build-optimize

Optimize Docker builds for CodeGuardian using advanced techniques including multi-stage builds, layer caching, and build context optimization to reduce build times and image sizes.

## Synopsis

```bash
codeguardian docker-build-optimize [OPTIONS] [COMMAND]
```

## Description

The `docker-build-optimize` command analyzes and optimizes Docker builds for CodeGuardian, implementing multi-stage build patterns, efficient layer caching, and build context minimization. It generates optimized Dockerfiles and provides build performance analytics.

### Key Features

- **Multi-stage optimization**: Automated multi-stage build generation
- **Layer caching**: Optimize layer ordering for better caching
- **Build context analysis**: Minimize build context size
- **Performance analytics**: Build time and size analysis
- **Security integration**: Security scanning in build pipeline

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--dockerfile` | Path to Dockerfile to optimize | string | Dockerfile | No |
| `--output-dir` | Output directory for optimized files | string | ./optimized | No |
| `--target-stages` | Target build stages to optimize | string[] | build,runtime | No |
| `--base-image` | Base image for runtime stage | string | debian:bookworm-slim | No |
| `--enable-caching` | Enable build caching optimizations | boolean | true | No |
| `--security-scan` | Include security scanning in build | boolean | true | No |
| `--analyze-only` | Only analyze, don't generate optimized files | boolean | false | No |
| `--benchmark` | Run build benchmarks | boolean | false | No |

## Commands

### analyze
Analyze current Docker build configuration.

```bash
codeguardian docker-build-optimize analyze [OPTIONS]
```

### optimize
Generate optimized Docker build configuration.

```bash
codeguardian docker-build-optimize optimize [OPTIONS]
```

### benchmark
Benchmark build performance improvements.

```bash
codeguardian docker-build-optimize benchmark [OPTIONS]
```

### cache-analyze
Analyze Docker layer caching efficiency.

```bash
codeguardian docker-build-optimize cache-analyze [OPTIONS]
```

## Examples

### Analyze Current Build

```bash
# Analyze current Dockerfile
codeguardian docker-build-optimize analyze --dockerfile Dockerfile
```

### Generate Optimized Build

```bash
# Generate optimized multi-stage build
codeguardian docker-build-optimize optimize \
  --dockerfile Dockerfile \
  --output-dir ./optimized
```

### Benchmark Build Performance

```bash
# Benchmark build performance improvements
codeguardian docker-build-optimize benchmark \
  --dockerfile Dockerfile \
  --iterations 5
```

### Analyze Caching Efficiency

```bash
# Analyze layer caching efficiency
codeguardian docker-build-optimize cache-analyze \
  --dockerfile Dockerfile
```

### Security-Enhanced Build

```bash
# Generate build with security scanning
codeguardian docker-build-optimize optimize \
  --security-scan \
  --base-image alpine:latest
```

## Multi-Stage Build Optimization

### Optimized Dockerfile Structure

```dockerfile
# ================================
# Build Stage
# ================================
FROM rust:1.70-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create build user
RUN useradd --create-home --shell /bin/bash build
USER build

# Set working directory
WORKDIR /home/build

# Copy dependency files first for better caching
COPY --chown=build:build Cargo.toml Cargo.lock ./

# Create dummy src to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src target/release/deps/codeguardian*

# Copy source code
COPY --chown=build:build src ./src

# Build application
RUN cargo build --release

# ================================
# Security Scan Stage
# ================================
FROM builder as security-scan

# Install security scanning tools
USER root
RUN apt-get update && apt-get install -y \
    clamav \
    trivy \
    && rm -rf /var/lib/apt/lists/*

# Scan for vulnerabilities
RUN trivy filesystem --exit-code 1 --no-progress /home/build/target/release/

# ================================
# Runtime Stage
# ================================
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user
RUN groupadd --gid 1000 codeguardian && \
    useradd --uid 1000 --gid codeguardian --shell /bin/bash --create-home codeguardian

# Set working directory
WORKDIR /home/codeguardian

# Copy binary from builder stage
COPY --from=builder --chown=codeguardian:codeguardian \
    /home/build/target/release/codeguardian /usr/local/bin/

# Create necessary directories
RUN mkdir -p /home/codeguardian/results /home/codeguardian/config && \
    chown -R codeguardian:codeguardian /home/codeguardian

# Switch to non-root user
USER codeguardian

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD codeguardian --version || exit 1

# Set default command
CMD ["codeguardian"]
```

## Build Context Optimization

### .dockerignore Optimization

```dockerignore
# Version control
.git
.gitignore

# Documentation
*.md
docs/
README*

# Development files
.env
.env.local
.env.*.local

# IDE files
.vscode/
.idea/
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db

# Build artifacts
target/
build/
dist/

# Test files
tests/
*.test
*_test.go

# Logs
*.log
logs/

# Temporary files
tmp/
temp/
*.tmp

# CI/CD files
.github/
.gitlab-ci.yml
Jenkinsfile

# Docker files (avoid recursion)
Dockerfile*
docker-compose*.yml
.dockerignore
```

## Layer Caching Optimization

### Dependency Layer Separation

```dockerfile
# Bad: Dependencies mixed with source
COPY . .
RUN cargo build --release

# Good: Dependencies cached separately
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --locked
COPY src ./src
RUN cargo build --release
```

### Package Manager Caching

```dockerfile
# For apt-based systems
RUN apt-get update && apt-get install -y \
    package1 \
    package2 \
    && rm -rf /var/lib/apt/lists/*

# For apk-based systems (Alpine)
RUN apk add --no-cache \
    package1 \
    package2
```

## Performance Analytics

### Build Time Analysis

```json
{
  "build_metrics": {
    "total_build_time": "45.2s",
    "stage_breakdown": {
      "build": "32.1s",
      "security_scan": "8.9s",
      "runtime": "4.2s"
    },
    "layer_efficiency": {
      "cache_hit_ratio": 0.85,
      "largest_layer": "128MB",
      "total_layers": 12
    }
  },
  "optimization_suggestions": [
    {
      "type": "layer_ordering",
      "impact": "high",
      "description": "Reorder COPY commands to improve cache efficiency",
      "estimated_savings": "15.3s"
    },
    {
      "type": "base_image",
      "impact": "medium",
      "description": "Use Alpine Linux for smaller image size",
      "estimated_savings": "45MB"
    }
  ]
}
```

## Security Integration

### Build-Time Security Scanning

```dockerfile
# Include security scanning in build
FROM builder as security-scan

# Install Trivy for vulnerability scanning
RUN apt-get update && apt-get install -y \
    wget \
    && wget -qO - https://aquasecurity.github.io/trivy-repo/deb/public.key | apt-key add - \
    && echo "deb https://aquasecurity.github.io/trivy-repo/deb generic main" | tee /etc/apt/sources.list.d/trivy.list \
    && apt-get update && apt-get install -y trivy \
    && rm -rf /var/lib/apt/lists/*

# Scan for vulnerabilities
RUN trivy filesystem --exit-code 1 --no-progress --format json \
    --output /tmp/trivy-results.json /home/build/target/release/

# Scan for secrets
RUN trivy filesystem --exit-code 1 --security-checks secret \
    /home/build/
```

## Best Practices

### Build Optimization

- **Layer ordering**: Place frequently changing layers at bottom
- **Minimal base images**: Use slim or distroless images
- **Multi-stage builds**: Separate build and runtime stages
- **Dependency caching**: Cache dependencies in separate layers
- **Build context**: Minimize files sent to Docker daemon

### Security Considerations

- **Base image security**: Use trusted, minimal base images
- **Vulnerability scanning**: Scan images during build process
- **Secret detection**: Check for secrets in build context
- **User permissions**: Run as non-root user in runtime
- **Capability dropping**: Drop unnecessary Linux capabilities

### Performance Optimization

- **Parallel builds**: Use BuildKit for parallel layer building
- **Cache mounting**: Mount caches for faster rebuilds
- **Incremental builds**: Only rebuild changed components
- **Registry caching**: Use registry mirrors for faster pulls
- **BuildKit**: Enable BuildKit for advanced features

## Error Handling

### Common Issues

- **Build context too large**: Optimize .dockerignore file
  ```bash
  du -sh . # Check current directory size
  codeguardian docker-build-optimize analyze --dockerfile Dockerfile
  ```

- **Layer caching ineffective**: Reorder Dockerfile commands
  ```bash
  codeguardian docker-build-optimize cache-analyze --dockerfile Dockerfile
  ```

- **Security scan failures**: Update base images or suppress false positives
  ```bash
  trivy image --exit-code 0 --no-progress my-image:latest
  ```

### Troubleshooting

1. **Analyze build performance**:
   ```bash
   codeguardian docker-build-optimize benchmark --iterations 3
   ```

2. **Check layer efficiency**:
   ```bash
   docker history my-image:latest
   ```

3. **Validate security**:
   ```bash
   trivy image my-image:latest
   ```

4. **Test optimized build**:
   ```bash
   docker build -f optimized/Dockerfile .
   ```

## Integration with CI/CD

### GitHub Actions Example

```yaml
# .github/workflows/optimized-build.yml
name: Optimized Docker Build
on:
  push:
    branches: [main]

jobs:
  optimize-and-build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Optimize Docker Build
        run: |
          codeguardian docker-build-optimize optimize \
            --output-dir ./optimized \
            --security-scan
      - name: Build Optimized Image
        uses: docker/build-push-action@v4
        with:
          context: .
          file: ./optimized/Dockerfile
          push: true
          tags: my-registry/codeguardian:latest
```

### GitLab CI Example

```yaml
# .gitlab-ci.yml
stages:
  - optimize
  - build
  - security

optimize_build:
  stage: optimize
  script:
    - codeguardian docker-build-optimize optimize --output-dir optimized
  artifacts:
    paths:
      - optimized/

build_image:
  stage: build
  script:
    - docker build -f optimized/Dockerfile -t codeguardian:$CI_COMMIT_SHA .
  dependencies:
    - optimize_build

security_scan:
  stage: security
  script:
    - trivy image codeguardian:$CI_COMMIT_SHA
  dependencies:
    - build_image
```

## See Also

- [`codeguardian docker-deploy`](docker-deploy.md) - Container deployment
- [`codeguardian docker-compose`](docker-compose.md) - Compose configurations
- [`codeguardian ci-cd github-actions`](../../../ci-cd/github-actions.md) - CI/CD integration
- [Docker Build Best Practices](https://docs.docker.com/develop/dev-best-practices/)