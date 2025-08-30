---
title: "docker-compose"
description: "Generate and manage Docker Compose configurations for CodeGuardian"
category: "Docker/Container"
tags: ["docker", "compose", "orchestration", "development", "testing"]
---

# docker-compose

Generate optimized Docker Compose configurations for CodeGuardian deployments, supporting development, testing, and production environments with proper service orchestration.

## Synopsis

```bash
codeguardian docker-compose [OPTIONS] [COMMAND]
```

## Description

The `docker-compose` command creates comprehensive Docker Compose configurations for CodeGuardian, including database integration, caching layers, and monitoring services. It supports multiple environments with optimized resource allocation and security configurations.

### Key Features

- **Multi-environment support**: Development, testing, and production configurations
- **Service orchestration**: Database, cache, and monitoring services
- **Security hardening**: Secure service configurations
- **Resource optimization**: Environment-specific resource allocation
- **Monitoring integration**: Built-in monitoring and logging

## Options

| Option | Description | Type | Default | Required |
|--------|-------------|------|---------|----------|
| `--environment` | Target environment (dev,test,prod) | string | dev | No |
| `--services` | Services to include (db,redis,monitoring) | string[] | db,redis | No |
| `--scale` | Scale factor for services | number | 1 | No |
| `--with-monitoring` | Include monitoring stack | boolean | false | No |
| `--with-security` | Apply security hardening | boolean | true | No |
| `--output-file` | Output file path | string | docker-compose.yml | No |
| `--registry` | Container registry URL | string | docker.io | No |
| `--image-tag` | Docker image tag | string | latest | No |

## Commands

### generate
Generate Docker Compose configuration.

```bash
codeguardian docker-compose generate [OPTIONS]
```

### up
Start CodeGuardian services using Docker Compose.

```bash
codeguardian docker-compose up [OPTIONS]
```

### down
Stop and remove CodeGuardian services.

```bash
codeguardian docker-compose down [OPTIONS]
```

### logs
View logs from CodeGuardian services.

```bash
codeguardian docker-compose logs [OPTIONS]
```

## Examples

### Generate Development Configuration

```bash
# Generate development environment configuration
codeguardian docker-compose generate --environment dev
```

### Production Setup with Monitoring

```bash
# Generate production configuration with monitoring
codeguardian docker-compose generate \
  --environment prod \
  --with-monitoring \
  --services db,redis,monitoring
```

### Start Services

```bash
# Start all services
codeguardian docker-compose up
```

### Scale Services

```bash
# Scale CodeGuardian service
codeguardian docker-compose up --scale codeguardian=3
```

### View Logs

```bash
# View logs from all services
codeguardian docker-compose logs
```

## Environment Configurations

### Development Environment

```yaml
version: '3.8'
services:
  codeguardian:
    image: codeguardian:dev
    ports:
      - "8080:8080"
    volumes:
      - ./config:/app/config
      - ./results:/app/results
    environment:
      - RUST_LOG=debug
      - CODEGUARDIAN_ENV=development
    depends_on:
      - postgres
      - redis

  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=codeguardian_dev
      - POSTGRES_USER=dev
      - POSTGRES_PASSWORD=dev_password
    volumes:
      - dev_postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - dev_redis_data:/data

volumes:
  dev_postgres_data:
  dev_redis_data:
```

### Production Environment

```yaml
version: '3.8'
services:
  codeguardian:
    image: codeguardian:latest
    ports:
      - "8080:8080"
    volumes:
      - ./config:/app/config:ro
      - results:/app/results
    environment:
      - RUST_LOG=info
      - CODEGUARDIAN_ENV=production
      - DATABASE_URL=postgresql://user:password@postgres:5432/codeguardian
      - REDIS_URL=redis://redis:6379
    depends_on:
      - postgres
      - redis
    deploy:
      resources:
        limits:
          cpus: '1.0'
          memory: 1G
        reservations:
          cpus: '0.5'
          memory: 512M
      restart_policy:
        condition: on-failure
        delay: 5s
        max_attempts: 3
        window: 120s

  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=codeguardian
      - POSTGRES_USER=${DB_USER}
      - POSTGRES_PASSWORD=${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 512M

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    deploy:
      resources:
        limits:
          cpus: '0.25'
          memory: 256M

volumes:
  postgres_data:
  redis_data:
  results:
```

### Testing Environment

```yaml
version: '3.8'
services:
  codeguardian:
    image: codeguardian:test
    ports:
      - "8080:8080"
    volumes:
      - ./config:/app/config:ro
      - ./test-results:/app/results
    environment:
      - RUST_LOG=debug
      - CODEGUARDIAN_ENV=testing
      - DATABASE_URL=postgresql://test:test@postgres:5432/codeguardian_test
    depends_on:
      - postgres
    command: ["codeguardian", "check", "--config", "/app/config/test-config.toml"]

  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=codeguardian_test
      - POSTGRES_USER=test
      - POSTGRES_PASSWORD=test
    volumes:
      - test_postgres_data:/var/lib/postgresql/data
    tmpfs:
      - /tmp
      - /var/run/postgresql

volumes:
  test_postgres_data:
```

## Monitoring Stack Integration

### With Prometheus and Grafana

```yaml
version: '3.8'
services:
  codeguardian:
    image: codeguardian:latest
    ports:
      - "8080:8080"
    environment:
      - METRICS_ENABLED=true
      - METRICS_PORT=9090
    depends_on:
      - postgres
      - redis

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml:ro
      - prometheus_data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/etc/prometheus/console_libraries'
      - '--web.console.templates=/etc/prometheus/consoles'
      - '--storage.tsdb.retention.time=200h'
      - '--web.enable-lifecycle'

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    volumes:
      - grafana_data:/var/lib/grafana
      - ./monitoring/grafana/provisioning:/etc/grafana/provisioning:ro
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
      - GF_USERS_ALLOW_SIGN_UP=false

  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=codeguardian
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
  prometheus_data:
  grafana_data:
```

## Security Configurations

### Network Security

```yaml
version: '3.8'
services:
  codeguardian:
    image: codeguardian:latest
    networks:
      - codeguardian_network
    security_opt:
      - no-new-privileges:true
    read_only: true
    tmpfs:
      - /tmp
    cap_drop:
      - ALL
    cap_add:
      - NET_BIND_SERVICE

  postgres:
    image: postgres:15-alpine
    networks:
      - codeguardian_network
    security_opt:
      - no-new-privileges:true

networks:
  codeguardian_network:
    driver: bridge
    internal: true
```

### Secret Management

```yaml
version: '3.8'
services:
  codeguardian:
    image: codeguardian:latest
    secrets:
      - db_password
      - api_key
    environment:
      - DB_PASSWORD_FILE=/run/secrets/db_password
      - API_KEY_FILE=/run/secrets/api_key

secrets:
  db_password:
    file: ./secrets/db_password.txt
  api_key:
    file: ./secrets/api_key.txt
```

## Best Practices

### Development Workflow

- **Hot reloading**: Use volume mounts for development
- **Debug logging**: Enable detailed logging for troubleshooting
- **Local databases**: Use local database instances for development
- **Service isolation**: Keep services isolated for testing

### Production Deployment

- **Resource limits**: Set appropriate resource constraints
- **Health checks**: Implement comprehensive health checks
- **Backup strategies**: Configure automated backups
- **Monitoring**: Enable monitoring and alerting

### Security Considerations

- **Network isolation**: Use internal networks for sensitive services
- **Secret management**: Use Docker secrets for sensitive data
- **Image scanning**: Scan images for vulnerabilities
- **Access control**: Implement proper access controls

## Error Handling

### Common Issues

- **Port conflicts**: Check for port availability
  ```bash
  netstat -tlnp | grep :8080
  ```

- **Volume permissions**: Ensure proper volume permissions
  ```bash
  ls -la ./results
  ```

- **Service dependencies**: Verify service startup order
  ```bash
  docker-compose logs postgres
  ```

### Troubleshooting

1. **Check service status**:
   ```bash
   docker-compose ps
   ```

2. **View service logs**:
   ```bash
   docker-compose logs codeguardian
   ```

3. **Test service connectivity**:
   ```bash
   docker-compose exec codeguardian curl http://localhost:8080/health
   ```

4. **Restart services**:
   ```bash
   docker-compose restart codeguardian
   ```

## Integration with CI/CD

### GitHub Actions Example

```yaml
# .github/workflows/compose-deploy.yml
name: Compose Deploy
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Generate Compose Config
        run: codeguardian docker-compose generate --environment prod
      - name: Deploy Services
        run: codeguardian docker-compose up -d
      - name: Run Tests
        run: docker-compose exec -T codeguardian codeguardian check
```

### GitLab CI Example

```yaml
# .gitlab-ci.yml
stages:
  - build
  - test
  - deploy

generate_config:
  stage: build
  script:
    - codeguardian docker-compose generate --environment test
  artifacts:
    paths:
      - docker-compose.yml

test_services:
  stage: test
  script:
    - docker-compose up -d
    - docker-compose exec -T codeguardian codeguardian check
    - docker-compose down

deploy_prod:
  stage: deploy
  script:
    - codeguardian docker-compose generate --environment prod
    - docker-compose up -d
  only:
    - main
```

## See Also

- [`codeguardian docker-deploy`](docker-deploy.md) - Container deployment
- [`codeguardian docker-build-optimize`](docker-build-optimize.md) - Build optimizations
- [`codeguardian ci-cd github-actions`](../../../ci-cd/github-actions.md) - CI/CD integration
- [Docker Compose Documentation](https://docs.docker.com/compose/)