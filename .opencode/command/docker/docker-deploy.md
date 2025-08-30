---
title: "docker-deploy"
description: "Deploy CodeGuardian in containerized environments"
category: "Docker/Container"
tags: ["docker", "container", "deployment", "kubernetes", "orchestration"]
---

# docker-deploy

Deploy CodeGuardian in containerized environments with optimized Docker configurations, supporting both standalone containers and orchestrated deployments.

## Synopsis

```bash
codeguardian docker-deploy [OPTIONS] [COMMAND]
```

## Description

The `docker-deploy` command provides comprehensive containerization support for CodeGuardian, including optimized Docker images, multi-stage builds, and orchestration configurations for Kubernetes and Docker Compose.

### Key Features

- **Optimized images**: Minimal, secure container images
- **Multi-stage builds**: Efficient build process with separate stages
- **Orchestration support**: Kubernetes and Docker Compose configurations
- **Security hardening**: Container security best practices
- **Performance optimization**: Resource-efficient container configurations

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--image-tag` | Docker image tag to use | string | latest | No |
| `--registry` | Container registry URL | string | docker.io | No |
| `--namespace` | Kubernetes namespace | string | codeguardian | No |
| `--replicas` | Number of replicas for deployment | number | 1 | No |
| `--cpu-limit` | CPU limit for containers | string | 1000m | No |
| `--memory-limit` | Memory limit for containers | string | 1Gi | No |
| `--storage-class` | Kubernetes storage class | string | standard | No |
| `--ingress-enabled` | Enable ingress for web interface | boolean | false | No |
| `--tls-enabled` | Enable TLS/SSL for secure communication | boolean | true | No |

## Commands

### build
Build optimized Docker images for CodeGuardian.

```bash
codeguardian docker-deploy build [OPTIONS]
```

### deploy
Deploy CodeGuardian to container orchestration platform.

```bash
codeguardian docker-deploy deploy [OPTIONS]
```

### config
Generate configuration files for container deployment.

```bash
codeguardian docker-deploy config [OPTIONS]
```

### status
Check status of deployed CodeGuardian containers.

```bash
codeguardian docker-deploy status [OPTIONS]
```

## Examples

### Build Optimized Docker Image

```bash
# Build production-ready Docker image
codeguardian docker-deploy build --image-tag v1.0.0
```

### Deploy to Kubernetes

```bash
# Deploy to Kubernetes with custom configuration
codeguardian docker-deploy deploy \
  --namespace security-tools \
  --replicas 3 \
  --cpu-limit 2000m \
  --memory-limit 2Gi
```

### Generate Docker Compose Configuration

```bash
# Generate Docker Compose configuration
codeguardian docker-deploy config --format compose > docker-compose.yml
```

### Check Deployment Status

```bash
# Check status of deployed containers
codeguardian docker-deploy status --namespace security-tools
```

### Secure Deployment with TLS

```bash
# Deploy with TLS enabled and ingress
codeguardian docker-deploy deploy \
  --tls-enabled \
  --ingress-enabled \
  --registry my-registry.com
```

## Docker Image Optimization

### Multi-Stage Build

```dockerfile
# Build stage
FROM rust:1.70-slim as builder
WORKDIR /usr/src/codeguardian
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/codeguardian/target/release/codeguardian /usr/local/bin/
USER codeguardian
CMD ["codeguardian"]
```

### Security Hardening

```dockerfile
# Security best practices
FROM debian:bookworm-slim

# Create non-root user
RUN groupadd -r codeguardian && useradd -r -g codeguardian codeguardian

# Install minimal dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Set secure permissions
RUN mkdir -p /app && chown -R codeguardian:codeguardian /app
WORKDIR /app

# Copy binary with minimal permissions
COPY --chown=codeguardian:codeguardian target/release/codeguardian /usr/local/bin/

# Switch to non-root user
USER codeguardian

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD codeguardian --version || exit 1

CMD ["codeguardian"]
```

## Kubernetes Deployment

### Deployment Manifest

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: codeguardian
  namespace: security-tools
spec:
  replicas: 3
  selector:
    matchLabels:
      app: codeguardian
  template:
    metadata:
      labels:
        app: codeguardian
    spec:
      securityContext:
        runAsNonRoot: true
        runAsUser: 1000
      containers:
      - name: codeguardian
        image: codeguardian:latest
        ports:
        - containerPort: 8080
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
        securityContext:
          allowPrivilegeEscalation: false
          readOnlyRootFilesystem: true
          capabilities:
            drop:
            - ALL
        volumeMounts:
        - name: tmp
          mountPath: /tmp
        - name: config
          mountPath: /app/config
      volumes:
      - name: tmp
        emptyDir: {}
      - name: config
        configMap:
          name: codeguardian-config
```

### Service Configuration

```yaml
apiVersion: v1
kind: Service
metadata:
  name: codeguardian-service
  namespace: security-tools
spec:
  selector:
    app: codeguardian
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP
```

## Docker Compose Configuration

### Basic Setup

```yaml
version: '3.8'
services:
  codeguardian:
    image: codeguardian:latest
    ports:
      - "8080:8080"
    volumes:
      - ./config:/app/config:ro
      - ./results:/app/results
    environment:
      - CODEGUARDIAN_CONFIG=/app/config/codeguardian.toml
    restart: unless-stopped
    security_opt:
      - no-new-privileges:true
```

### Advanced Setup with Database

```yaml
version: '3.8'
services:
  codeguardian:
    image: codeguardian:latest
    depends_on:
      - postgres
      - redis
    ports:
      - "8080:8080"
    volumes:
      - ./config:/app/config:ro
      - results:/app/results
    environment:
      - DATABASE_URL=postgresql://user:password@postgres:5432/codeguardian
      - REDIS_URL=redis://redis:6379
    restart: unless-stopped

  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=codeguardian
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    restart: unless-stopped

volumes:
  postgres_data:
  redis_data:
  results:
```

## Best Practices

### Security Considerations

- **Minimal base images**: Use slim or distroless base images
- **Non-root execution**: Run containers as non-root user
- **Read-only filesystem**: Use read-only root filesystem where possible
- **Capability dropping**: Drop unnecessary Linux capabilities
- **Image scanning**: Scan images for vulnerabilities regularly

### Performance Optimization

- **Resource limits**: Set appropriate CPU and memory limits
- **Multi-stage builds**: Use multi-stage builds to reduce image size
- **Layer caching**: Optimize Docker layer caching for faster builds
- **Volume mounting**: Use volumes for persistent data
- **Health checks**: Implement proper health checks

### Orchestration

- **Rolling updates**: Use rolling update strategies for zero downtime
- **Resource quotas**: Set resource quotas for namespaces
- **Network policies**: Implement network segmentation
- **Monitoring**: Enable comprehensive monitoring and logging

## Error Handling

### Common Issues

- **Image pull failures**: Check registry access and credentials
  ```bash
  docker login my-registry.com
  ```

- **Resource constraints**: Adjust CPU and memory limits
  ```bash
  kubectl describe pod codeguardian-xyz
  ```

- **Network issues**: Verify service networking
  ```bash
  kubectl get services -n security-tools
  ```

### Troubleshooting

1. **Check container logs**:
   ```bash
   kubectl logs -f deployment/codeguardian -n security-tools
   ```

2. **Verify image**:
   ```bash
   docker inspect codeguardian:latest
   ```

3. **Test deployment**:
   ```bash
   kubectl exec -it deployment/codeguardian -- codeguardian --version
   ```

4. **Check resources**:
   ```bash
   kubectl top pods -n security-tools
   ```

## Integration with CI/CD

### GitHub Actions Example

```yaml
# .github/workflows/docker-deploy.yml
name: Docker Deploy
on:
  push:
    tags:
      - 'v*'

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build Docker Image
        run: codeguardian docker-deploy build --image-tag ${{ github.ref_name }}
      - name: Deploy to Kubernetes
        run: |
          codeguardian docker-deploy deploy \
            --image-tag ${{ github.ref_name }} \
            --namespace production
```

### GitLab CI Example

```yaml
# .gitlab-ci.yml
stages:
  - build
  - deploy

build_image:
  stage: build
  script:
    - codeguardian docker-deploy build --image-tag $CI_COMMIT_TAG
  only:
    - tags

deploy_k8s:
  stage: deploy
  script:
    - codeguardian docker-deploy deploy --image-tag $CI_COMMIT_TAG
  only:
    - tags
  dependencies:
    - build_image
```

## See Also

- [`codeguardian docker-compose`](docker-compose.md) - Docker Compose configurations
- [`codeguardian docker-build-optimize`](docker-build-optimize.md) - Build optimizations
- [`codeguardian ci-cd github-actions`](../../../ci-cd/github-actions.md) - CI/CD integration
- [Docker Best Practices](https://docs.docker.com/develop/dev-best-practices/)