# Security Model

Wassette implements a comprehensive security model designed to safely execute untrusted WebAssembly components while providing powerful capabilities to AI agents. This multi-layered approach ensures that malicious or buggy components cannot compromise the host system.

## Security Philosophy

Wassette's security model is built on three core principles:

1. **Zero Trust**: Assume all components are potentially malicious
2. **Principle of Least Privilege**: Components only get permissions they explicitly need
3. **Defense in Depth**: Multiple security layers provide redundant protection

## Security Layers

### Layer 1: WebAssembly Sandbox

The foundation of Wassette's security is the WebAssembly sandbox:

**Memory Safety**
- Isolated linear memory space
- No direct access to host memory
- Automatic bounds checking
- Protection against buffer overflows

**Execution Safety**
- Deterministic execution model
- No arbitrary code execution
- Stack overflow protection
- Infinite loop detection

**Interface Control**
- Only pre-defined functions can be called
- Type-safe function calls
- No raw pointer manipulation
- Controlled host imports

### Layer 2: WASI Security

WebAssembly System Interface (WASI) provides controlled access to system resources:

**Capability-Based Access**
- Explicit capability grants required
- No ambient authority
- Unforgeable capability tokens
- Revocable permissions

**Resource Isolation**
- Separate capability spaces per component
- No cross-component resource sharing
- Virtual filesystem views
- Network namespace isolation

### Layer 3: Wassette Policy Engine

Wassette adds an additional policy layer on top of WASI:

**Permission Policies**
- YAML-based policy definitions
- Fine-grained resource controls
- Runtime permission management
- Policy validation and enforcement

**Resource Monitoring**
- Real-time resource usage tracking
- Configurable resource limits
- Automated enforcement actions
- Comprehensive audit logging

## Permission System

### Permission Types

Wassette controls access to three types of resources:

#### Storage Permissions

Control access to the filesystem:

```yaml
permissions:
  storage:
    allow:
      - uri: "fs://workspace/**"
        access: ["read", "write"]
      - uri: "fs://config/app.yaml"
        access: ["read"]
    deny:
      - uri: "fs:///etc/**"
      - uri: "fs:///home/**"
```

**Access Levels:**
- `read`: Read files and directory listings
- `write`: Create, modify, and delete files
- `execute`: Execute files (not currently supported)

**Path Patterns:**
- Exact paths: `/specific/file.txt`
- Directory patterns: `/dir/**` (recursive)
- Glob patterns: `/logs/*.log`

#### Network Permissions

Control access to network resources:

```yaml
permissions:
  network:
    allow:
      - host: "api.openai.com"
        ports: [443]
      - host: "*.github.com"
        ports: [80, 443]
    deny:
      - host: "localhost"
      - host: "127.0.0.1"
```

**Host Patterns:**
- Exact hostnames: `api.example.com`
- Wildcard patterns: `*.example.com`
- IP addresses: `192.168.1.1`
- CIDR ranges: `10.0.0.0/8`

**Port Controls:**
- Specific ports: `[80, 443]`
- Port ranges: `[8000-9000]`
- All ports: `[]` (empty list means all)

#### Environment Permissions

Control access to environment variables and configuration:

```yaml
permissions:
  environment:
    allow:
      - key: "API_KEY"
      - key: "CONFIG_*"
    deny:
      - key: "HOME"
      - key: "PATH"
```

**Variable Patterns:**
- Exact names: `API_KEY`
- Prefix patterns: `CONFIG_*`
- Regex patterns: `/^LOG_LEVEL_.+/`

### Policy Files

Each component has an associated policy file that defines its permissions:

#### Policy Structure

```yaml
version: "1.0"
description: "Weather API component permissions"
metadata:
  component_id: "weather-tool"
  version: "1.0.0"
  author: "example@company.com"

permissions:
  storage:
    allow:
      - uri: "fs://tmp/cache/**"
        access: ["read", "write"]
        description: "Cache API responses"
  
  network:
    allow:
      - host: "api.openweathermap.org"
        ports: [443]
        description: "Weather API access"
  
  environment:
    allow:
      - key: "WEATHER_API_KEY"
        description: "Weather service API key"

limits:
  memory: "64MB"
  cpu_time: "5s"
  network_requests: 10
  file_handles: 5
```

#### Policy Discovery

Policies are discovered in this order:

1. **Embedded Policy**: Policy embedded in component metadata
2. **Sidecar Policy**: `component-name.policy.yaml` in same directory
3. **Registry Policy**: Policy downloaded with component from OCI registry
4. **Default Policy**: Minimal default permissions

### Runtime Permission Management

Permissions can be managed dynamically at runtime:

#### Granting Permissions

```bash
# Grant storage permission
wassette permission grant storage \
  --component weather-tool \
  --uri "fs://data/**" \
  --access read,write

# Grant network permission  
wassette permission grant network \
  --component weather-tool \
  --host "backup.api.com" \
  --ports 443
```

#### Revoking Permissions

```bash
# Revoke specific permission
wassette permission revoke storage \
  --component weather-tool \
  --uri "fs://data/**"

# Reset all permissions
wassette permission reset --component weather-tool
```

#### Auditing Permissions

```bash
# View component permissions
wassette permission list --component weather-tool

# View all component permissions
wassette permission list --all
```

## Resource Limits

Beyond permissions, Wassette enforces resource limits to prevent abuse:

### Memory Limits

```yaml
limits:
  memory:
    initial: "32MB"    # Initial memory allocation
    maximum: "128MB"   # Maximum memory allowed
    growth_rate: "1MB/s"  # Maximum growth rate
```

### CPU Limits

```yaml
limits:
  cpu:
    time_limit: "10s"     # Maximum execution time
    instruction_limit: 1000000  # Maximum instructions
    fuel_limit: 100000    # WebAssembly fuel limit
```

### I/O Limits

```yaml
limits:
  io:
    file_handles: 10      # Maximum open files
    network_connections: 5  # Maximum network connections
    read_bandwidth: "1MB/s"   # Read bandwidth limit
    write_bandwidth: "500KB/s"  # Write bandwidth limit
```

## Threat Model

Wassette's security model protects against several classes of attacks:

### Malicious Components

**Resource Exhaustion**
- Memory bombs: Prevented by memory limits
- CPU exhaustion: Prevented by execution time limits
- Disk space attacks: Prevented by storage quotas

**Data Exfiltration**
- Unauthorized file access: Prevented by filesystem permissions
- Network reconnaissance: Prevented by network allowlists
- Environment snooping: Prevented by environment variable controls

**System Compromise**
- Privilege escalation: Prevented by WebAssembly sandbox
- Code injection: Prevented by memory safety
- System calls: Prevented by WASI capability model

### Supply Chain Attacks

**Malicious Dependencies**
- Component signing: Cryptographic component verification
- Registry scanning: Automated vulnerability scanning
- Policy validation: Mandatory security policies

**Compromised Registries**
- Content verification: Hash-based integrity checking
- Multiple sources: Support for multiple registries
- Local caching: Reduce dependency on external sources

### Configuration Attacks

**Policy Bypass**
- Policy validation: Schemas and constraints
- Immutable policies: Once loaded, policies cannot be modified
- Audit trails: All policy changes are logged

**Privilege Escalation**
- Least privilege: Components start with minimal permissions
- Explicit grants: All permissions must be explicitly granted
- Time-limited: Permissions can have expiration times

## Security Best Practices

### For Component Developers

1. **Minimal Permissions**: Only request permissions you actually need
2. **Input Validation**: Validate all inputs from the host environment
3. **Secure Secrets**: Use the configuration store for API keys
4. **Error Handling**: Don't leak sensitive information in errors
5. **Dependencies**: Audit all component dependencies

### For Platform Operators

1. **Policy Review**: Review all component policies before deployment
2. **Monitor Usage**: Track resource usage and access patterns
3. **Regular Updates**: Keep components and policies updated
4. **Incident Response**: Have procedures for security incidents
5. **Backup Policies**: Maintain secure backups of policies and components

### For AI Agent Developers

1. **Validate Components**: Verify component signatures and policies
2. **Limit Exposure**: Don't expose sensitive data to components
3. **Monitor Calls**: Track component calls and results
4. **Error Handling**: Handle component errors gracefully
5. **User Consent**: Get user consent for sensitive operations

## Security Monitoring

### Audit Logging

Wassette provides comprehensive audit logging:

```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "event_type": "permission_check",
  "component_id": "weather-tool",
  "operation": "network_access",
  "resource": "api.openweathermap.org:443",
  "result": "allowed",
  "policy_id": "weather-policy-v1",
  "user_agent": "claude-desktop"
}
```

### Security Metrics

Key security metrics to monitor:

- **Permission Denials**: Rate of denied access attempts
- **Resource Usage**: Memory, CPU, and I/O consumption
- **Error Rates**: Component execution errors
- **Policy Violations**: Attempted policy bypasses
- **Network Activity**: Outbound network connections

### Alerting

Configure alerts for security events:

```yaml
alerts:
  - name: "High Permission Denials"
    condition: "permission_denials > 10 per minute"
    action: "disable_component"
  
  - name: "Resource Exhaustion"
    condition: "memory_usage > 90%"
    action: "restart_component"
  
  - name: "Suspicious Network Activity"
    condition: "new_hosts > 5 per hour"
    action: "alert_admin"
```

## Compliance and Certifications

Wassette's security model supports various compliance requirements:

### Industry Standards

- **NIST Cybersecurity Framework**: Comprehensive security controls
- **ISO 27001**: Information security management
- **SOC 2**: Security, availability, and confidentiality
- **GDPR**: Data protection and privacy

### Government Standards

- **FISMA**: Federal information security requirements
- **Common Criteria**: International security evaluation
- **FIPS 140-2**: Cryptographic module validation

## Future Security Enhancements

### Planned Features

1. **Component Signing**: Cryptographic verification of component integrity
2. **Policy Encryption**: Encrypted policy storage and transmission
3. **Hardware Security**: Integration with hardware security modules
4. **Formal Verification**: Mathematical proofs of security properties
5. **Zero-Knowledge Proofs**: Privacy-preserving component execution

### Research Areas

- **Homomorphic Encryption**: Computation on encrypted data
- **Secure Multi-Party Computation**: Collaborative computation without data sharing
- **Confidential Computing**: TEE-based component execution
- **Quantum-Resistant Cryptography**: Post-quantum security

## Next Steps

- Learn about the [Permission System](../design/permission-system.md) in detail
- Understand [Sandboxing](./sandboxing.md) mechanisms
- Review [Security Best Practices](../development/best-practices.md)
- Explore [Cookbook examples](../cookbook/common-patterns.md) with security considerations