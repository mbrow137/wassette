# Built-in Tools

Wassette provides several built-in tools for managing components, permissions, and system configuration. These tools are automatically available when you start Wassette and provide essential functionality for component lifecycle management.

## Tool Categories

### Component Management
- [load-component](#load-component) - Load WebAssembly components
- [unload-component](#unload-component) - Unload components  
- [list-components](#list-components) - List loaded components

### Permission Management
- [get-policy](#get-policy) - Get component policy information
- [grant-storage-permission](#grant-storage-permission) - Grant storage access
- [grant-network-permission](#grant-network-permission) - Grant network access
- [grant-environment-variable-permission](#grant-environment-variable-permission) - Grant environment variable access
- [revoke-storage-permission](#revoke-storage-permission) - Revoke storage permissions
- [revoke-network-permission](#revoke-network-permission) - Revoke network permissions
- [revoke-environment-variable-permission](#revoke-environment-variable-permission) - Revoke environment permissions
- [reset-permission](#reset-permission) - Reset all component permissions

## Component Management Tools

### load-component

Load a WebAssembly component from various sources.

**Parameters:**
- `uri` (string, required): Component URI (file://, oci://, or https://)

**Supported URI Formats:**
- Local files: `file:///path/to/component.wasm`
- OCI registries: `oci://ghcr.io/owner/component:tag`
- HTTP URLs: `https://example.com/component.wasm`

**Returns:**
```json
{
  "component_id": "unique-component-id",
  "status": "loaded",
  "tools": ["tool1", "tool2"],
  "load_time": "2024-01-15T10:30:00Z"
}
```

**Example Usage:**
```
Please load the component from oci://ghcr.io/microsoft/time-server-js:latest
```

**Error Handling:**
- Invalid URI format
- Component not found
- Compilation errors
- Permission denied
- Network connectivity issues

### unload-component

Unload a previously loaded component and free its resources.

**Parameters:**
- `component_id` (string, required): ID of the component to unload

**Returns:**
```json
{
  "component_id": "unique-component-id",
  "status": "unloaded",
  "unload_time": "2024-01-15T10:35:00Z"
}
```

**Example Usage:**
```
Please unload the weather-tool component
```

### list-components

List all currently loaded components with their status and metadata.

**Parameters:**
None

**Returns:**
```json
{
  "components": [
    {
      "component_id": "weather-tool",
      "status": "active",
      "tools": ["get-weather", "get-forecast"],
      "load_time": "2024-01-15T10:30:00Z",
      "memory_usage": "12MB",
      "call_count": 45
    }
  ],
  "total_components": 1
}
```

**Example Usage:**
```
What components are currently loaded?
```

## Permission Management Tools

### get-policy

Retrieve policy information for a specific component.

**Parameters:**
- `component_id` (string, required): ID of the component

**Returns:**
```json
{
  "component_id": "weather-tool",
  "policy": {
    "version": "1.0",
    "permissions": {
      "storage": [
        {
          "uri": "fs://workspace/**",
          "access": ["read", "write"]
        }
      ],
      "network": [
        {
          "host": "api.openweathermap.org",
          "ports": [443]
        }
      ],
      "environment": [
        {
          "key": "WEATHER_API_KEY"
        }
      ]
    },
    "limits": {
      "memory": "64MB",
      "cpu_time": "30s"
    }
  },
  "policy_source": "embedded"
}
```

**Example Usage:**
```
What permissions does the weather-tool component have?
```

### grant-storage-permission

Grant file system access to a component.

**Parameters:**
- `component_id` (string, required): ID of the component
- `path` (string, required): File system path or pattern
- `access` (array, required): Access types ["read", "write"]

**Returns:**
```json
{
  "component_id": "weather-tool",
  "permission_type": "storage",
  "granted": {
    "uri": "fs://data/**",
    "access": ["read", "write"]
  },
  "status": "granted"
}
```

**Example Usage:**
```
Grant the file-processor component read and write access to fs://workspace/output/**
```

### grant-network-permission

Grant network access to a component.

**Parameters:**
- `component_id` (string, required): ID of the component
- `host` (string, required): Hostname or IP address
- `ports` (array, optional): Allowed ports (empty = all ports)

**Returns:**
```json
{
  "component_id": "weather-tool",
  "permission_type": "network",
  "granted": {
    "host": "backup.api.com",
    "ports": [443]
  },
  "status": "granted"
}
```

**Example Usage:**
```
Allow the http-client component to access api.github.com on port 443
```

### grant-environment-variable-permission

Grant access to environment variables.

**Parameters:**
- `component_id` (string, required): ID of the component
- `variable_name` (string, required): Environment variable name

**Returns:**
```json
{
  "component_id": "config-loader",
  "permission_type": "environment",
  "granted": {
    "key": "DATABASE_URL"
  },
  "status": "granted"
}
```

**Example Usage:**
```
Allow the database-client component to read the DATABASE_URL environment variable
```

### revoke-storage-permission

Revoke previously granted file system access.

**Parameters:**
- `component_id` (string, required): ID of the component
- `path` (string, required): File system path to revoke

**Returns:**
```json
{
  "component_id": "weather-tool",
  "permission_type": "storage",
  "revoked": {
    "uri": "fs://temp/**"
  },
  "status": "revoked"
}
```

### revoke-network-permission

Revoke previously granted network access.

**Parameters:**
- `component_id` (string, required): ID of the component
- `host` (string, required): Hostname to revoke

**Returns:**
```json
{
  "component_id": "weather-tool",
  "permission_type": "network",
  "revoked": {
    "host": "old.api.com"
  },
  "status": "revoked"
}
```

### revoke-environment-variable-permission

Revoke access to environment variables.

**Parameters:**
- `component_id` (string, required): ID of the component
- `variable_name` (string, required): Environment variable name

**Returns:**
```json
{
  "component_id": "config-loader",
  "permission_type": "environment",
  "revoked": {
    "key": "OLD_API_KEY"
  },
  "status": "revoked"
}
```

### reset-permission

Reset all permissions for a component to default/minimal permissions.

**Parameters:**
- `component_id` (string, required): ID of the component

**Returns:**
```json
{
  "component_id": "weather-tool",
  "status": "reset",
  "previous_permissions": {
    "storage": 2,
    "network": 1,
    "environment": 3
  },
  "reset_time": "2024-01-15T10:45:00Z"
}
```

**Example Usage:**
```
Reset all permissions for the file-processor component
```

## Tool Usage Patterns

### Component Loading Workflow

1. **Load Component**: Use `load-component` to load from registry or file
2. **Check Status**: Use `list-components` to verify successful loading  
3. **Review Permissions**: Use `get-policy` to understand current permissions
4. **Grant Additional Permissions**: Use permission grant tools as needed
5. **Use Component**: Call component functions via AI agent
6. **Monitor Usage**: Check component status and resource usage
7. **Cleanup**: Use `unload-component` when no longer needed

### Permission Management Workflow

1. **Audit Current Permissions**: Use `get-policy` to review current state
2. **Grant Minimal Permissions**: Use grant tools for only required access
3. **Test Functionality**: Verify component works with granted permissions
4. **Adjust as Needed**: Grant additional permissions if required
5. **Regular Review**: Periodically audit and revoke unnecessary permissions
6. **Emergency Response**: Use `reset-permission` to quickly remove all access

### Security Best Practices

1. **Principle of Least Privilege**: Grant only necessary permissions
2. **Regular Audits**: Periodically review component permissions
3. **Time-bound Access**: Consider temporary permission grants
4. **Monitor Usage**: Track how components use granted permissions
5. **Incident Response**: Have procedures for revoking compromised components

## Error Handling

### Common Error Scenarios

**Component Not Found**
```json
{
  "error": "Component not found",
  "component_id": "non-existent-component",
  "suggestion": "Use list-components to see available components"
}
```

**Permission Already Granted**
```json
{
  "error": "Permission already exists",
  "component_id": "weather-tool",
  "permission": "fs://workspace/**",
  "suggestion": "Use get-policy to see current permissions"
}
```

**Invalid Permission**
```json
{
  "error": "Invalid permission",
  "details": "Path must start with fs://",
  "suggestion": "Use format: fs://path/pattern"
}
```

### Error Recovery

1. **Check Component Status**: Use `list-components` to verify component state
2. **Review Current Permissions**: Use `get-policy` to understand current state
3. **Validate Parameters**: Ensure all parameters are correctly formatted
4. **Check Logs**: Review Wassette logs for detailed error information
5. **Retry with Corrections**: Fix issues and retry the operation

## Automation and Scripting

### Batch Operations

You can script common operations:

```bash
#!/bin/bash
# Load and configure a component

# Load component
wassette component load oci://ghcr.io/example/tool:latest

# Get component ID from response
COMPONENT_ID="example-tool"

# Grant necessary permissions
wassette permission grant storage --component $COMPONENT_ID --path "fs://workspace/**" --access read,write
wassette permission grant network --component $COMPONENT_ID --host "api.example.com" --ports 443

# Verify configuration
wassette policy get $COMPONENT_ID
```

### Monitoring Scripts

```python
#!/usr/bin/env python3
import subprocess
import json
import time

def check_component_health():
    """Monitor component health and resource usage"""
    result = subprocess.run(['wassette', 'component', 'list'], 
                          capture_output=True, text=True)
    
    if result.returncode == 0:
        components = json.loads(result.stdout)
        for component in components['components']:
            memory_mb = int(component['memory_usage'].replace('MB', ''))
            if memory_mb > 100:  # Alert if memory usage > 100MB
                print(f"High memory usage: {component['component_id']} using {component['memory_usage']}")

if __name__ == "__main__":
    while True:
        check_component_health()
        time.sleep(60)  # Check every minute
```

## Integration with AI Agents

### Natural Language Interface

All built-in tools can be called using natural language:

**Instead of technical commands:**
```json
{
  "method": "tools/call",
  "params": {
    "name": "load-component",
    "arguments": {
      "uri": "oci://ghcr.io/microsoft/weather-tool:latest"
    }
  }
}
```

**Use natural language:**
```
Please load the weather tool from oci://ghcr.io/microsoft/weather-tool:latest
```

### Common Phrases

- "Load the [component] from [URI]"
- "What components are loaded?"
- "Grant [component] access to [resource]"
- "What permissions does [component] have?"
- "Remove [component] access to [resource]"
- "Unload the [component] component"

## Troubleshooting

### Component Loading Issues

1. **Check URI format**: Ensure proper scheme (file://, oci://, https://)
2. **Verify connectivity**: Test network access to registries/URLs
3. **Check permissions**: Ensure file system access for local files
4. **Review logs**: Check detailed error messages in logs

### Permission Issues

1. **Use get-policy**: Review current permissions
2. **Check path format**: Ensure proper fs:// URI format
3. **Verify access types**: Use correct access values (read, write)
4. **Test incrementally**: Grant minimal permissions first

### Performance Issues

1. **Monitor resource usage**: Check memory and CPU usage
2. **Review component count**: Too many loaded components can impact performance
3. **Check permission complexity**: Complex permission patterns can slow access
4. **Optimize policies**: Simplify permission patterns where possible

## Next Steps

- Review [CLI Reference](./cli.md) for command-line usage
- Learn about [Configuration](./configuration.md) for system setup
- Check [FAQ](./faq.md) for common questions
- Explore [Security Model](../security/security-model.md) for permission details