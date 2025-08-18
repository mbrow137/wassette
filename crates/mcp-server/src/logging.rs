// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! MCP logging utilities for structured logging via the Model Context Protocol.

use anyhow::Result;
use chrono::Utc;
use rmcp::model::{LoggingLevel, LoggingMessageNotificationParam};
use rmcp::{Peer, RoleServer};
use serde_json::{json, Value};
use std::sync::Arc;

/// A utility for sending structured log messages via MCP logging notifications.
///
/// This struct provides a convenient interface for sending log messages to MCP clients
/// with consistent formatting and structure.
#[derive(Clone)]
pub struct McpLogger {
    peer: Arc<Peer<RoleServer>>,
    logger_name: String,
}

impl McpLogger {
    /// Creates a new MCP logger with the specified peer and logger name.
    ///
    /// # Arguments
    /// * `peer` - The MCP server peer for sending notifications
    /// * `logger_name` - Name of the logger (e.g., "wassette.lifecycle")
    pub fn new(peer: Arc<Peer<RoleServer>>, logger_name: String) -> Self {
        Self { peer, logger_name }
    }

    /// Logs a debug message with structured data.
    pub async fn debug(&self, message: &str, data: Value) -> Result<()> {
        self.log(LoggingLevel::Debug, message, data).await
    }

    /// Logs an info message with structured data.
    pub async fn info(&self, message: &str, data: Value) -> Result<()> {
        self.log(LoggingLevel::Info, message, data).await
    }

    /// Logs a notice message with structured data.
    pub async fn notice(&self, message: &str, data: Value) -> Result<()> {
        self.log(LoggingLevel::Notice, message, data).await
    }

    /// Logs a warning message with structured data.
    pub async fn warning(&self, message: &str, data: Value) -> Result<()> {
        self.log(LoggingLevel::Warning, message, data).await
    }

    /// Logs an error message with structured data.
    pub async fn error(&self, message: &str, data: Value) -> Result<()> {
        self.log(LoggingLevel::Error, message, data).await
    }

    /// Logs a critical message with structured data.
    pub async fn critical(&self, message: &str, data: Value) -> Result<()> {
        self.log(LoggingLevel::Critical, message, data).await
    }

    /// Logs an alert message with structured data.
    pub async fn alert(&self, message: &str, data: Value) -> Result<()> {
        self.log(LoggingLevel::Alert, message, data).await
    }

    /// Logs an emergency message with structured data.
    pub async fn emergency(&self, message: &str, data: Value) -> Result<()> {
        self.log(LoggingLevel::Emergency, message, data).await
    }

    /// Logs a component lifecycle event (loading, unloading, etc.).
    pub async fn log_component_event(
        &self,
        event: &str,
        component_id: &str,
        additional_data: Value,
    ) -> Result<()> {
        let mut data = json!({
            "event": event,
            "component_id": component_id,
        });

        if let (Some(data_obj), Some(additional_obj)) =
            (data.as_object_mut(), additional_data.as_object())
        {
            for (key, value) in additional_obj {
                data_obj.insert(key.clone(), value.clone());
            }
        }

        self.info(&format!("Component {}: {}", event, component_id), data)
            .await
    }

    /// Logs a tool execution event with performance metrics.
    pub async fn log_tool_execution(
        &self,
        tool_name: &str,
        component_id: &str,
        execution_time_ms: u64,
        success: bool,
        error: Option<&str>,
    ) -> Result<()> {
        let level = if success {
            LoggingLevel::Info
        } else {
            LoggingLevel::Error
        };
        let message = if success {
            format!("Tool execution completed: {}", tool_name)
        } else {
            format!("Tool execution failed: {}", tool_name)
        };

        let mut data = json!({
            "tool_name": tool_name,
            "component_id": component_id,
            "execution_time_ms": execution_time_ms,
            "success": success,
        });

        if let Some(error_msg) = error {
            data["error"] = json!(error_msg);
        }

        self.log(level, &message, data).await
    }

    /// Logs a security event (permission violations, policy enforcement, etc.).
    pub async fn log_security_event(
        &self,
        event_type: &str,
        component_id: &str,
        details: Value,
    ) -> Result<()> {
        let mut data = json!({
            "event_type": event_type,
            "component_id": component_id,
        });

        if let (Some(data_obj), Some(details_obj)) = (data.as_object_mut(), details.as_object()) {
            for (key, value) in details_obj {
                data_obj.insert(key.clone(), value.clone());
            }
        }

        self.warning(&format!("Security event: {}", event_type), data)
            .await
    }

    /// Core logging method that formats and sends the log message.
    async fn log(&self, level: LoggingLevel, message: &str, mut data: Value) -> Result<()> {
        // Ensure required fields are present in the data object
        if let Some(obj) = data.as_object_mut() {
            // Always include message and timestamp
            obj.insert("message".to_string(), json!(message));
            obj.insert("timestamp".to_string(), json!(Utc::now().to_rfc3339()));
        } else {
            // If data is not an object, create a new object with the required fields
            data = json!({
                "message": message,
                "timestamp": Utc::now().to_rfc3339(),
                "original_data": data
            });
        }

        let log_message = LoggingMessageNotificationParam {
            level,
            data,
            logger: Some(self.logger_name.clone()),
        };

        // Send the notification - ignore errors to avoid breaking execution flow
        if let Err(e) = self.peer.notify_logging_message(log_message).await {
            tracing::warn!("Failed to send MCP log message: {}", e);
        }

        Ok(())
    }
}

/// Convenience functions for creating common structured data objects.
pub mod data {
    use serde_json::{json, Value};

    /// Creates data for component loading events.
    pub fn component_loading(component_path: &str, file_size: Option<u64>) -> Value {
        let mut data = json!({
            "component_path": component_path,
        });

        if let Some(size) = file_size {
            data["file_size_bytes"] = json!(size);
        }

        data
    }

    /// Creates data for component execution performance.
    pub fn execution_performance(
        memory_usage: Option<u64>,
        cpu_time_ms: Option<u64>,
        peak_memory: Option<u64>,
    ) -> Value {
        let mut data = json!({});

        if let Some(memory) = memory_usage {
            data["memory_usage_bytes"] = json!(memory);
        }
        if let Some(cpu) = cpu_time_ms {
            data["cpu_time_ms"] = json!(cpu);
        }
        if let Some(peak) = peak_memory {
            data["peak_memory_bytes"] = json!(peak);
        }

        data
    }

    /// Creates data for error events.
    pub fn error_details(error_type: &str, error_message: &str, recoverable: bool) -> Value {
        json!({
            "error_type": error_type,
            "error_message": error_message,
            "recoverable": recoverable,
        })
    }

    /// Creates data for permission/security events.
    pub fn security_violation(
        operation: &str,
        requested_resource: &str,
        policy_rule: &str,
    ) -> Value {
        json!({
            "attempted_operation": operation,
            "requested_resource": requested_resource,
            "policy_violation": policy_rule,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_helpers() {
        let loading_data = data::component_loading("/path/to/component.wasm", Some(1024));
        assert_eq!(loading_data["component_path"], "/path/to/component.wasm");
        assert_eq!(loading_data["file_size_bytes"], 1024);

        let error_data = data::error_details("RuntimeError", "Division by zero", false);
        assert_eq!(error_data["error_type"], "RuntimeError");
        assert_eq!(error_data["recoverable"], false);

        let security_data =
            data::security_violation("file_access", "/etc/passwd", "filesystem_access_denied");
        assert_eq!(security_data["attempted_operation"], "file_access");
        assert_eq!(
            security_data["policy_violation"],
            "filesystem_access_denied"
        );
    }
}
