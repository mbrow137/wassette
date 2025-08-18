# MCP Logging in Wassette

This document provides a comprehensive guide on leveraging Model Context Protocol (MCP) logging for structured logging in Wassette.

## Overview

The Model Context Protocol provides a standardized way for servers to send structured log messages to clients. This enables better debugging, monitoring, and observability for MCP interactions.

## MCP Logging Specification

According to the [MCP specification](https://modelcontextprotocol.io/specification/2025-06-18/server/utilities/logging), MCP logging provides:

- **Structured log messages**: Log data is sent as JSON objects with required fields
- **Standardized levels**: Eight severity levels from Debug to Emergency 
- **Client-controlled filtering**: Clients can set minimum log levels
- **Real-time delivery**: Log messages are sent as notifications

## Logging Levels

MCP defines 8 standard logging levels in order of increasing severity:

| Level | Description | Use Case |
|-------|-------------|----------|
| `debug` | Detailed diagnostic information | Development and troubleshooting |
| `info` | General informational messages | Normal operation status |
| `notice` | Significant conditions | Important events |
| `warning` | Warning conditions | Potential issues |
| `error` | Error conditions | Recoverable errors |
| `critical` | Critical conditions | Critical system issues |
| `alert` | Action must be taken immediately | Immediate attention required |
| `emergency` | System is unusable | System failure |

## Message Structure

Each MCP log message contains:

```json
{
  "level": "info",
  "data": {
    "message": "Component loaded successfully",
    "timestamp": "2025-01-15T10:30:00Z",
    "component_id": "example-component",
    "execution_time_ms": 150
  },
  "logger": "wassette.lifecycle"
}
```

### Required Fields

- **`level`**: The logging level as a string
- **`data`**: A JSON object containing the actual log data

### Optional Fields

- **`logger`**: Name of the logger that generated the message

### Data Object Conventions

The `data` object should include:

- **`message`**: Human-readable description of the event
- **`timestamp`**: RFC3339 timestamp when the event occurred
- Additional structured fields relevant to the event

## Integration with Wassette

### Current Logging

Wassette currently uses the `tracing` crate for internal logging:

```rust
use tracing::{info, error, debug, instrument};

#[instrument(skip(lifecycle_manager))]
pub async fn handle_load_component(req: &CallToolRequestParam) -> Result<()> {
    info!(component_id = %id, "Loading component");
    // Component loading logic
    info!(component_id = %id, "Component loaded successfully");
}
```

### MCP Logging Integration

To leverage MCP logging, Wassette can send structured log messages to MCP clients:

```rust
use rmcp::{LoggingLevel, LoggingMessageNotificationParam};
use serde_json::json;
use chrono::Utc;

// Send structured log message to MCP client
let log_message = LoggingMessageNotificationParam {
    level: LoggingLevel::Info,
    data: json!({
        "message": "Component loaded successfully",
        "timestamp": Utc::now().to_rfc3339(),
        "component_id": id,
        "execution_time_ms": execution_time,
        "memory_usage_bytes": memory_usage
    }),
    logger: Some("wassette.lifecycle".to_string()),
};

peer.notify_logging_message(log_message).await?;
```

## Benefits of MCP Logging

### 1. **Structured Data**
Unlike plain text logs, MCP logging provides structured JSON data that clients can parse and filter programmatically.

### 2. **Real-time Monitoring**
Log messages are sent immediately as notifications, enabling real-time monitoring of MCP server operations.

### 3. **Client-controlled Filtering**
Clients can set minimum log levels, reducing noise and focusing on relevant information.

### 4. **Standardized Format**
All MCP servers use the same logging format, making it easier to build tools and dashboards.

### 5. **Rich Context**
Structured data allows including relevant context like component IDs, execution times, memory usage, etc.

## Use Cases

### Component Lifecycle Monitoring

```rust
// Component loading
peer.notify_logging_message(LoggingMessageNotificationParam {
    level: LoggingLevel::Info,
    data: json!({
        "message": "Loading WebAssembly component",
        "timestamp": Utc::now().to_rfc3339(),
        "component_id": component_id,
        "file_path": component_path,
        "file_size_bytes": file_size
    }),
    logger: Some("wassette.lifecycle".to_string()),
}).await?;
```

### Performance Metrics

```rust
// Tool execution performance
peer.notify_logging_message(LoggingMessageNotificationParam {
    level: LoggingLevel::Info,
    data: json!({
        "message": "Tool execution completed",
        "timestamp": Utc::now().to_rfc3339(),
        "tool_name": tool_name,
        "execution_time_ms": execution_time,
        "memory_peak_mb": memory_peak,
        "success": true
    }),
    logger: Some("wassette.execution".to_string()),
}).await?;
```

### Error Reporting

```rust
// Detailed error information
peer.notify_logging_message(LoggingMessageNotificationParam {
    level: LoggingLevel::Error,
    data: json!({
        "message": "Component execution failed",
        "timestamp": Utc::now().to_rfc3339(),
        "component_id": component_id,
        "error_type": "WasmRuntimeError",
        "error_details": error.to_string(),
        "stack_trace": error.backtrace(),
        "recoverable": false
    }),
    logger: Some("wassette.execution".to_string()),
}).await?;
```

### Security Events

```rust
// Permission violations
peer.notify_logging_message(LoggingMessageNotificationParam {
    level: LoggingLevel::Warning,
    data: json!({
        "message": "Component attempted unauthorized operation",
        "timestamp": Utc::now().to_rfc3339(),
        "component_id": component_id,
        "attempted_operation": "file_system_access",
        "requested_path": "/etc/passwd",
        "policy_violation": "filesystem_access_denied"
    }),
    logger: Some("wassette.security".to_string()),
}).await?;
```

## Implementation Guidelines

### 1. **Enable Logging Capability**

Update server capabilities to advertise logging support:

```rust
ServerCapabilities {
    logging: Some(serde_json::json!({})),
    tools: Some(ToolsCapability {
        list_changed: Some(true),
    }),
    // ... other capabilities
}
```

### 2. **Create Logging Utility**

Create a utility module for consistent MCP logging:

```rust
pub struct McpLogger {
    peer: Peer<RoleServer>,
    logger_name: String,
}

impl McpLogger {
    pub async fn info(&self, message: &str, data: serde_json::Value) -> Result<()> {
        self.log(LoggingLevel::Info, message, data).await
    }
    
    pub async fn error(&self, message: &str, data: serde_json::Value) -> Result<()> {
        self.log(LoggingLevel::Error, message, data).await
    }
    
    async fn log(&self, level: LoggingLevel, message: &str, mut data: serde_json::Value) -> Result<()> {
        // Ensure required fields are present
        if let Some(obj) = data.as_object_mut() {
            obj.insert("message".to_string(), json!(message));
            obj.insert("timestamp".to_string(), json!(Utc::now().to_rfc3339()));
        }
        
        let log_message = LoggingMessageNotificationParam {
            level,
            data,
            logger: Some(self.logger_name.clone()),
        };
        
        self.peer.notify_logging_message(log_message).await?;
        Ok(())
    }
}
```

### 3. **Bridge with Tracing**

Create a tracing subscriber that forwards to MCP logging:

```rust
pub struct McpTracingSubscriber {
    peer: Arc<Peer<RoleServer>>,
}

impl<S> Layer<S> for McpTracingSubscriber 
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        // Convert tracing events to MCP log messages
        let level = match *event.metadata().level() {
            tracing::Level::TRACE => LoggingLevel::Debug,
            tracing::Level::DEBUG => LoggingLevel::Debug,
            tracing::Level::INFO => LoggingLevel::Info,
            tracing::Level::WARN => LoggingLevel::Warning,
            tracing::Level::ERROR => LoggingLevel::Error,
        };
        
        // Extract structured data from tracing fields
        let mut visitor = JsonFieldVisitor::new();
        event.record(&mut visitor);
        
        let log_message = LoggingMessageNotificationParam {
            level,
            data: visitor.into_json(),
            logger: Some(event.metadata().target().to_string()),
        };
        
        // Send asynchronously (in practice, use a channel to avoid blocking)
        tokio::spawn({
            let peer = self.peer.clone();
            async move {
                let _ = peer.notify_logging_message(log_message).await;
            }
        });
    }
}
```

## Best Practices

### 1. **Use Appropriate Levels**
- **Debug**: Verbose diagnostic information
- **Info**: Normal operational events
- **Warning**: Potentially problematic situations
- **Error**: Error events that might still allow the application to continue

### 2. **Include Relevant Context**
Always include relevant identifiers and metrics:
- Component IDs
- Tool names  
- Execution times
- Memory usage
- Request IDs

### 3. **Structure Data Consistently**
Use consistent field names across your application:
- `component_id` not `componentId` or `comp_id`
- `timestamp` in RFC3339 format
- `execution_time_ms` for timing data

### 4. **Avoid Sensitive Information**
Never log sensitive data like:
- Passwords or API keys
- User personal information
- File contents (unless specifically intended)

### 5. **Rate Limiting**
Implement rate limiting for high-frequency events to avoid overwhelming clients:

```rust
// Use a token bucket or similar approach
if self.should_log_rate_limited_event() {
    peer.notify_logging_message(log_message).await?;
}
```

## Client Integration

MCP clients can control logging behavior by:

### Setting Log Level

```typescript
// Set minimum log level to Warning
await client.request("logging/setLevel", { level: "warning" });
```

### Handling Log Messages

```typescript
client.onNotification("notifications/message", (params) => {
    console.log(`[${params.level}] ${params.data.message}`, params.data);
    
    // Store in database, send to monitoring system, etc.
    if (params.level === "error") {
        alertingSystem.sendAlert(params.data);
    }
});
```

## Conclusion

MCP logging provides a powerful way to create observable, debuggable MCP servers. By leveraging structured logging with the MCP protocol, Wassette can provide clients with rich, real-time insights into component execution, performance, and errors.

The key is to balance informativeness with performance, providing structured data that helps with debugging and monitoring while not overwhelming clients with too much information.