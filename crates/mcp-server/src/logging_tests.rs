// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! Tests for MCP logging functionality.

use crate::logging::data;
use rmcp::model::{LoggingLevel, LoggingMessageNotificationParam};
use serde_json::json;

#[tokio::test]
async fn test_mcp_logger_basic_functionality() {
    // This test demonstrates how to use the MCP logger
    // Note: In a real test, you would need to set up a proper MCP peer
    // For now, this just tests the data structure creation

    let test_data = data::component_loading("/path/to/component.wasm", Some(1024));
    assert_eq!(test_data["component_path"], "/path/to/component.wasm");
    assert_eq!(test_data["file_size_bytes"], 1024);

    let error_data = data::error_details("RuntimeError", "Division by zero", false);
    assert_eq!(error_data["error_type"], "RuntimeError");
    assert_eq!(error_data["error_message"], "Division by zero");
    assert_eq!(error_data["recoverable"], false);

    let security_data =
        data::security_violation("file_access", "/etc/passwd", "filesystem_access_denied");
    assert_eq!(security_data["attempted_operation"], "file_access");
    assert_eq!(security_data["requested_resource"], "/etc/passwd");
    assert_eq!(
        security_data["policy_violation"],
        "filesystem_access_denied"
    );
}

#[test]
fn test_logging_level_serialization() {
    // Test that LoggingLevel values serialize correctly according to MCP spec
    let levels = [
        (LoggingLevel::Debug, "debug"),
        (LoggingLevel::Info, "info"),
        (LoggingLevel::Notice, "notice"),
        (LoggingLevel::Warning, "warning"),
        (LoggingLevel::Error, "error"),
        (LoggingLevel::Critical, "critical"),
        (LoggingLevel::Alert, "alert"),
        (LoggingLevel::Emergency, "emergency"),
    ];

    for (level, expected) in levels {
        let serialized = serde_json::to_string(&level).unwrap();
        assert_eq!(serialized, format!("\"{}\"", expected));
    }
}

#[test]
fn test_logging_message_structure() {
    // Test that LoggingMessageNotificationParam has the correct structure
    let message = LoggingMessageNotificationParam {
        level: LoggingLevel::Info,
        data: json!({
            "message": "Test message",
            "timestamp": "2025-01-15T10:30:00Z",
            "component_id": "test-component"
        }),
        logger: Some("wassette.test".to_string()),
    };

    // Serialize to JSON to verify structure
    let serialized = serde_json::to_value(&message).unwrap();

    assert_eq!(serialized["level"], "info");
    assert_eq!(serialized["data"]["message"], "Test message");
    assert_eq!(serialized["data"]["component_id"], "test-component");
    assert_eq!(serialized["logger"], "wassette.test");
}

#[test]
fn test_data_helpers_comprehensive() {
    // Test component loading data helper
    let loading_data = data::component_loading("/path/to/component.wasm", Some(2048));
    assert!(loading_data.is_object());
    assert_eq!(loading_data["component_path"], "/path/to/component.wasm");
    assert_eq!(loading_data["file_size_bytes"], 2048);

    // Test execution performance data helper
    let perf_data = data::execution_performance(Some(1024), Some(150), Some(2048));
    assert_eq!(perf_data["memory_usage_bytes"], 1024);
    assert_eq!(perf_data["cpu_time_ms"], 150);
    assert_eq!(perf_data["peak_memory_bytes"], 2048);

    // Test with None values
    let perf_data_partial = data::execution_performance(None, Some(100), None);
    assert!(perf_data_partial["cpu_time_ms"] == 100);
    assert!(perf_data_partial.get("memory_usage_bytes").is_none());
    assert!(perf_data_partial.get("peak_memory_bytes").is_none());

    // Test error details data helper
    let error_data = data::error_details("ValidationError", "Invalid input", true);
    assert_eq!(error_data["error_type"], "ValidationError");
    assert_eq!(error_data["error_message"], "Invalid input");
    assert_eq!(error_data["recoverable"], true);

    // Test security violation data helper
    let security_data = data::security_violation(
        "network_access",
        "https://evil.com",
        "unauthorized_network_access",
    );
    assert_eq!(security_data["attempted_operation"], "network_access");
    assert_eq!(security_data["requested_resource"], "https://evil.com");
    assert_eq!(
        security_data["policy_violation"],
        "unauthorized_network_access"
    );
}

/// This test demonstrates the expected JSON structure for MCP logging messages
#[test]
fn test_mcp_logging_json_structure() {
    let message = LoggingMessageNotificationParam {
        level: LoggingLevel::Info,
        data: json!({
            "message": "Component loaded successfully",
            "timestamp": "2025-01-15T10:30:00Z",
            "component_id": "example-component",
            "execution_time_ms": 150,
            "path": "/components/example.wasm",
            "file_size_bytes": 2048
        }),
        logger: Some("wassette.lifecycle".to_string()),
    };

    let json_output = serde_json::to_string_pretty(&message).unwrap();

    // Verify the JSON contains all expected fields
    assert!(json_output.contains("\"level\": \"info\""));
    assert!(json_output.contains("\"message\": \"Component loaded successfully\""));
    assert!(json_output.contains("\"component_id\": \"example-component\""));
    assert!(json_output.contains("\"logger\": \"wassette.lifecycle\""));

    // The structure should match the MCP specification
    let parsed: serde_json::Value = serde_json::from_str(&json_output).unwrap();
    assert!(parsed["level"].is_string());
    assert!(parsed["data"].is_object());
    assert!(parsed["logger"].is_string());
}
