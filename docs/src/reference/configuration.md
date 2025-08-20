# Configuration

This guide covers all configuration options for Wassette, including server settings, security policies, runtime options, and environment variables.

## Configuration Overview

Wassette can be configured through:

1. **Command Line Arguments** - Runtime options and overrides
2. **Environment Variables** - System-level configuration
3. **Configuration Files** - Detailed server and policy settings
4. **Component Policies** - Per-component security settings

## Command Line Arguments

### Server Mode

Start Wassette as an MCP server:

```bash
wassette serve [OPTIONS]
```

**Transport Options:**
- `--stdio` - Use standard input/output (default for MCP clients)
- `--sse [PORT]` - Use Server-Sent Events on specified port (default: 8080)
- `--websocket [PORT]` - Use WebSocket on specified port (default: 8080)

**General Options:**
- `--plugin-dir <PATH>` - Directory for component storage (default: `~/.wassette/plugins`)
- `--config <FILE>` - Configuration file path
- `--log-level <LEVEL>` - Logging level (error, warn, info, debug, trace)
- `--max-components <N>` - Maximum number of loaded components (default: 100)

**Security Options:**
- `--default-policy <FILE>` - Default component security policy
- `--require-signatures` - Require component signatures (default: false)
- `--sandbox-mode <MODE>` - Sandbox strictness (strict, normal, permissive)

**Performance Options:**
- `--worker-threads <N>` - Number of worker threads (default: CPU cores)
- `--memory-limit <SIZE>` - Global memory limit (e.g., "1GB")
- `--execution-timeout <SECONDS>` - Default execution timeout (default: 30)

### CLI Mode

Use Wassette for direct component management:

```bash
# Component management
wassette component load <URI>
wassette component unload <ID>
wassette component list

# Permission management  
wassette permission grant <TYPE> --component <ID> [OPTIONS]
wassette permission revoke <TYPE> --component <ID> [OPTIONS]
wassette policy get <COMPONENT_ID>
```

**Output Options:**
- `--output-format <FORMAT>` - Output format (json, yaml, table)
- `--quiet` - Suppress non-essential output
- `--verbose` - Enable verbose output

## Environment Variables

### Core Configuration

```bash
# Server configuration
WASSETTE_PLUGIN_DIR=/path/to/plugins
WASSETTE_CONFIG_FILE=/path/to/config.yaml
WASSETTE_LOG_LEVEL=info
WASSETTE_BIND_ADDRESS=127.0.0.1
WASSETTE_PORT=8080

# Security settings
WASSETTE_DEFAULT_POLICY=/path/to/default-policy.yaml
WASSETTE_REQUIRE_SIGNATURES=false
WASSETTE_SANDBOX_MODE=normal

# Performance tuning
WASSETTE_MAX_COMPONENTS=100
WASSETTE_WORKER_THREADS=4
WASSETTE_MEMORY_LIMIT=1GB
WASSETTE_EXECUTION_TIMEOUT=30

# Feature flags
WASSETTE_ENABLE_METRICS=true
WASSETTE_ENABLE_TRACING=false
WASSETTE_ENABLE_CACHE=true
```

### Registry Configuration

```bash
# OCI registry settings
WASSETTE_REGISTRY_AUTH_TOKEN=<token>
WASSETTE_REGISTRY_USERNAME=<username>
WASSETTE_REGISTRY_PASSWORD=<password>
WASSETTE_REGISTRY_INSECURE=false

# HTTP client settings
WASSETTE_HTTP_TIMEOUT=30
WASSETTE_HTTP_RETRIES=3
WASSETTE_HTTP_USER_AGENT="Wassette/1.0"
```

### Logging and Monitoring

```bash
# Logging configuration
RUST_LOG=wassette=info,wasmtime=warn
WASSETTE_LOG_FORMAT=json
WASSETTE_LOG_FILE=/var/log/wassette.log

# Metrics and tracing
WASSETTE_METRICS_ENDPOINT=http://prometheus:9090
WASSETTE_JAEGER_ENDPOINT=http://jaeger:14268
WASSETTE_HEALTH_CHECK_PORT=9001
```

## Configuration Files

### Server Configuration

Create a `wassette.yaml` configuration file:

```yaml
# Server settings
server:
  bind_address: "127.0.0.1"
  port: 8080
  transport: "sse"  # stdio, sse, websocket
  
# Component settings
components:
  plugin_dir: "~/.wassette/plugins"
  max_components: 100
  default_timeout: 30
  cache_enabled: true
  cache_size: "256MB"
  
# Security settings
security:
  default_policy: "default-policy.yaml"
  require_signatures: false
  sandbox_mode: "normal"  # strict, normal, permissive
  allowed_registries:
    - "ghcr.io"
    - "docker.io"
  blocked_hosts:
    - "localhost"
    - "127.0.0.1"
    - "metadata.google.internal"

# Performance settings
performance:
  worker_threads: 4
  memory_limit: "1GB"
  execution_timeout: 30
  max_memory_per_component: "64MB"
  max_cpu_time_per_component: "10s"

# Logging settings
logging:
  level: "info"
  format: "json"
  file: "/var/log/wassette.log"
  max_size: "100MB"
  max_files: 10

# Monitoring settings
monitoring:
  enable_metrics: true
  metrics_port: 9090
  enable_tracing: false
  health_check_port: 9001
  
# Registry settings
registries:
  oci:
    timeout: 30
    retries: 3
    auth:
      # Configure per-registry authentication
      "ghcr.io":
        username: "${GITHUB_USERNAME}"
        token: "${GITHUB_TOKEN}"
```

### Load Configuration

```bash
# Use configuration file
wassette serve --config /path/to/wassette.yaml

# Override with environment variables
WASSETTE_LOG_LEVEL=debug wassette serve --config wassette.yaml

# Override with command line arguments
wassette serve --config wassette.yaml --port 9090 --log-level trace
```

## Security Policies

### Default Policy

Create a default security policy for all components:

```yaml
# default-policy.yaml
version: "1.0"
description: "Default security policy for all components"

# Default permissions (very restrictive)
permissions:
  storage:
    allow:
      - uri: "fs://workspace/**"
        access: ["read"]
        description: "Read workspace files"
    deny:
      - uri: "fs:///etc/**"
        description: "Block system configuration"
      - uri: "fs:///home/**"
        description: "Block user directories"
      - uri: "fs:///root/**"
        description: "Block root directory"

  network:
    allow: []  # No network access by default
    deny:
      - host: "localhost"
        description: "Block localhost access"
      - host: "127.0.0.1"
        description: "Block loopback access"
      - host: "*.internal"
        description: "Block internal domains"

  environment:
    allow: []  # No environment variables by default
    deny:
      - key: "*"
        description: "Block all environment variables by default"

# Resource limits
limits:
  memory: "64MB"
  cpu_time: "30s"
  file_handles: 10
  network_connections: 5
  execution_timeout: 30

# Monitoring settings
monitoring:
  log_access_attempts: true
  alert_on_violations: true
  collect_metrics: true
```

### Component-Specific Policies

Components can have their own policies:

```yaml
# weather-tool-policy.yaml
version: "1.0"
description: "Policy for weather API tool"

metadata:
  component_id: "weather-tool"
  version: "1.0.0"
  author: "weather-team@company.com"

permissions:
  storage:
    allow:
      - uri: "fs://cache/weather/**"
        access: ["read", "write"]
        description: "Cache weather data"
    
  network:
    allow:
      - host: "api.openweathermap.org"
        ports: [443]
        description: "Weather API access"
      - host: "backup.weather-api.com"
        ports: [443]
        description: "Backup weather API"
        
  environment:
    allow:
      - key: "WEATHER_API_KEY"
        description: "Weather service API key"

limits:
  memory: "32MB"
  cpu_time: "10s"
  file_handles: 5
  network_connections: 2
  network_requests_per_minute: 60
```

## Runtime Configuration

### Memory Management

```yaml
memory:
  # Global memory settings
  global_limit: "2GB"
  component_default_limit: "64MB"
  component_max_limit: "256MB"
  
  # Garbage collection
  gc_enabled: true
  gc_threshold: 0.8  # Trigger GC at 80% memory usage
  gc_interval: "5m"  # Force GC every 5 minutes
  
  # Memory monitoring
  track_allocations: true
  log_high_usage: true
  alert_threshold: 0.9  # Alert at 90% usage
```

### CPU Management

```yaml
cpu:
  # Execution limits
  default_timeout: 30
  max_timeout: 300
  fuel_limit: 1000000  # WebAssembly instruction limit
  
  # Scheduling
  worker_threads: 4
  max_concurrent_executions: 10
  priority_scheduling: true
  
  # Monitoring
  track_cpu_usage: true
  log_slow_executions: true
  slow_execution_threshold: "5s"
```

### I/O Configuration

```yaml
io:
  # File system
  max_open_files_per_component: 10
  file_operation_timeout: "10s"
  enable_file_caching: true
  
  # Network
  max_connections_per_component: 5
  connection_timeout: "30s"
  read_timeout: "10s"
  write_timeout: "10s"
  max_request_size: "10MB"
  max_response_size: "50MB"
  
  # Rate limiting
  enable_rate_limiting: true
  default_rate_limit: 100  # requests per minute
  burst_limit: 10
```

## Monitoring Configuration

### Metrics Collection

```yaml
metrics:
  enabled: true
  port: 9090
  path: "/metrics"
  
  # Metric types
  collect_system_metrics: true
  collect_component_metrics: true
  collect_security_metrics: true
  collect_performance_metrics: true
  
  # Retention
  retention_period: "7d"
  scrape_interval: "15s"
  
  # Labels
  additional_labels:
    environment: "production"
    datacenter: "us-west-2"
```

### Health Checks

```yaml
health:
  enabled: true
  port: 9001
  path: "/health"
  
  # Check intervals
  component_check_interval: "30s"
  resource_check_interval: "10s"
  
  # Thresholds
  memory_threshold: 0.9
  cpu_threshold: 0.8
  error_rate_threshold: 0.1
  
  # Endpoints
  readiness_endpoint: "/ready"
  liveness_endpoint: "/live"
```

### Distributed Tracing

```yaml
tracing:
  enabled: false
  endpoint: "http://jaeger:14268/api/traces"
  
  # Sampling
  sampling_rate: 0.1  # Sample 10% of traces
  max_trace_size: "1MB"
  
  # Components
  trace_component_loading: true
  trace_function_calls: true
  trace_permission_checks: true
```

## Development Configuration

### Development Mode

```yaml
development:
  enabled: false
  
  # Relaxed security for development
  allow_unsigned_components: true
  disable_sandbox: false  # Never fully disable
  verbose_errors: true
  
  # Hot reloading
  watch_components: true
  auto_reload_on_change: true
  
  # Debug features
  enable_component_inspector: true
  inspector_port: 9002
  enable_debug_endpoints: true
```

### Testing Configuration

```yaml
testing:
  # Test isolation
  isolated_mode: true
  clean_state_per_test: true
  
  # Mock services
  mock_network_calls: false
  mock_file_system: false
  
  # Test data
  test_data_dir: "./test-data"
  preserve_test_data: false
```

## Production Configuration

### Production Hardening

```yaml
production:
  # Security hardening
  strict_mode: true
  require_component_signatures: true
  disable_debug_endpoints: true
  
  # Resource limits
  enforce_strict_limits: true
  kill_on_limit_exceeded: true
  
  # Monitoring
  detailed_audit_logging: true
  security_event_alerting: true
  
  # Performance
  optimize_for_production: true
  enable_jit_compilation: true
```

### Scaling Configuration

```yaml
scaling:
  # Horizontal scaling
  max_instances: 10
  auto_scaling_enabled: true
  scale_up_threshold: 0.8
  scale_down_threshold: 0.3
  
  # Load balancing
  load_balancer_type: "round_robin"
  health_check_enabled: true
  
  # Clustering
  cluster_mode: false
  cluster_discovery: "dns"
  cluster_nodes: []
```

## Configuration Validation

### Validation Rules

Wassette validates configuration on startup:

```bash
# Validate configuration
wassette validate-config --config wassette.yaml

# Check policy syntax
wassette validate-policy --policy default-policy.yaml

# Test component policy
wassette test-policy --policy weather-policy.yaml --component weather-tool
```

### Common Configuration Errors

1. **Invalid YAML syntax**
2. **Missing required fields**
3. **Conflicting settings**
4. **Invalid resource limits**
5. **Malformed permission patterns**
6. **Unreachable network endpoints**

## Configuration Management

### Environment-Specific Configs

```bash
# Use different configs per environment
wassette serve --config configs/development.yaml  # Development
wassette serve --config configs/staging.yaml     # Staging  
wassette serve --config configs/production.yaml  # Production
```

### Configuration Templates

```yaml
# base-config.yaml (template)
server:
  transport: "${TRANSPORT:-sse}"
  port: ${PORT:-8080}

security:
  sandbox_mode: "${SANDBOX_MODE:-normal}"
  
logging:
  level: "${LOG_LEVEL:-info}"
```

### Secret Management

```yaml
# Use environment variable substitution for secrets
registries:
  oci:
    auth:
      "ghcr.io":
        token: "${GITHUB_TOKEN}"  # From environment
        
database:
  password: "${DB_PASSWORD}"  # From secret manager
```

## Troubleshooting Configuration

### Common Issues

1. **Configuration not found**: Check file paths and permissions
2. **Invalid format**: Validate YAML syntax
3. **Permission denied**: Check file system permissions
4. **Environment variables not expanded**: Verify variable names
5. **Resource limits too restrictive**: Adjust limits based on usage

### Debug Configuration

```bash
# Enable configuration debugging
WASSETTE_DEBUG_CONFIG=true wassette serve --config wassette.yaml

# Dump effective configuration
wassette config dump --config wassette.yaml

# Validate configuration
wassette config validate --config wassette.yaml
```

## Next Steps

- Review [CLI Reference](./cli.md) for command-line usage
- Learn about [Built-in Tools](./built-in-tools.md) for system management
- Check [FAQ](./faq.md) for common configuration questions
- Explore [Security Model](../security/security-model.md) for policy details