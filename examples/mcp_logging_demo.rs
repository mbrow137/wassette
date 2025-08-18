#!/usr/bin/env rust-script

//! # MCP Logging Example
//!
//! This example demonstrates how to use MCP logging in Wassette for structured logging.
//!
//! Run this example with:
//! ```bash
//! cargo run --example mcp_logging_demo
//! ```

use mcp_server::{data, McpLogger};
use rmcp::model::LoggingLevel;
use serde_json::json;
use std::sync::Arc;
use tokio;

/// Simulated MCP peer for demonstration
struct MockPeer;

impl MockPeer {
    async fn notify_logging_message(
        &self,
        message: rmcp::model::LoggingMessageNotificationParam,
    ) -> Result<(), String> {
        // In a real implementation, this would send the message to the MCP client
        // For this demo, we'll just print it
        println!("📊 MCP Log Message:");
        println!("   Level: {:?}", message.level);
        println!("   Logger: {:?}", message.logger);
        println!(
            "   Data: {}",
            serde_json::to_string_pretty(&message.data).unwrap()
        );
        println!();
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 MCP Logging Demonstration\n");

    // In a real Wassette implementation, you would have an actual MCP peer
    // For this demo, we'll use a mock
    let _mock_peer = Arc::new(MockPeer);

    // This would be replaced with:
    // let mcp_logger = McpLogger::new(server_peer, "wassette.lifecycle".to_string());
    println!("💡 Note: This demo shows the data structures that would be sent via MCP logging.\n");

    demonstrate_component_lifecycle().await?;
    demonstrate_tool_execution().await?;
    demonstrate_error_logging().await?;
    demonstrate_security_events().await?;
    demonstrate_performance_monitoring().await?;

    println!("✅ MCP Logging demonstration complete!");
    println!("\n📚 See docs/mcp-logging.md for comprehensive documentation.");

    Ok(())
}

async fn demonstrate_component_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 Component Lifecycle Logging Example:");

    // Component loading started
    let loading_data = data::component_loading("/path/to/component.wasm", Some(2048));
    let message = rmcp::model::LoggingMessageNotificationParam {
        level: LoggingLevel::Info,
        data: json!({
            "message": "Component loading started",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "component_path": "/path/to/component.wasm",
            "file_size_bytes": 2048
        }),
        logger: Some("wassette.lifecycle".to_string()),
    };
    print_mcp_message(&message);

    // Component loaded successfully
    let success_message = rmcp::model::LoggingMessageNotificationParam {
        level: LoggingLevel::Info,
        data: json!({
            "message": "Component loaded successfully",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "component_id": "example-component-123",
            "component_path": "/path/to/component.wasm",
            "execution_time_ms": 150,
            "memory_allocated_bytes": 1024000
        }),
        logger: Some("wassette.lifecycle".to_string()),
    };
    print_mcp_message(&success_message);

    Ok(())
}

async fn demonstrate_tool_execution() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Tool Execution Logging Example:");

    let message = rmcp::model::LoggingMessageNotificationParam {
        level: LoggingLevel::Info,
        data: json!({
            "message": "Tool execution completed",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "tool_name": "fetch-data",
            "component_id": "fetch-component-456",
            "execution_time_ms": 250,
            "memory_peak_mb": 15.2,
            "success": true,
            "output_size_bytes": 4096
        }),
        logger: Some("wassette.execution".to_string()),
    };
    print_mcp_message(&message);

    Ok(())
}

async fn demonstrate_error_logging() -> Result<(), Box<dyn std::error::Error>> {
    println!("❌ Error Logging Example:");

    let error_data =
        data::error_details("WasmRuntimeError", "Stack overflow during execution", false);

    let message = rmcp::model::LoggingMessageNotificationParam {
        level: LoggingLevel::Error,
        data: json!({
            "message": "Component execution failed",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "component_id": "failing-component-789",
            "tool_name": "process-data",
            "error_type": "WasmRuntimeError",
            "error_message": "Stack overflow during execution",
            "recoverable": false,
            "execution_time_ms": 5000,
            "memory_at_failure_mb": 128.5
        }),
        logger: Some("wassette.execution".to_string()),
    };
    print_mcp_message(&message);

    Ok(())
}

async fn demonstrate_security_events() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Security Event Logging Example:");

    let security_data =
        data::security_violation("file_access", "/etc/passwd", "filesystem_access_denied");

    let message = rmcp::model::LoggingMessageNotificationParam {
        level: LoggingLevel::Warning,
        data: json!({
            "message": "Component attempted unauthorized operation",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "component_id": "suspicious-component-101",
            "attempted_operation": "file_access",
            "requested_resource": "/etc/passwd",
            "policy_violation": "filesystem_access_denied",
            "action_taken": "request_blocked",
            "risk_level": "high"
        }),
        logger: Some("wassette.security".to_string()),
    };
    print_mcp_message(&message);

    Ok(())
}

async fn demonstrate_performance_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("📈 Performance Monitoring Example:");

    let perf_data = data::execution_performance(
        Some(15728640), // 15MB
        Some(1500),     // 1.5 seconds
        Some(20971520), // 20MB peak
    );

    let message = rmcp::model::LoggingMessageNotificationParam {
        level: LoggingLevel::Notice,
        data: json!({
            "message": "Performance metrics report",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "component_id": "analytics-component-202",
            "memory_usage_bytes": 15728640,
            "cpu_time_ms": 1500,
            "peak_memory_bytes": 20971520,
            "gc_collections": 3,
            "avg_response_time_ms": 45.6,
            "throughput_ops_per_second": 156.7
        }),
        logger: Some("wassette.metrics".to_string()),
    };
    print_mcp_message(&message);

    Ok(())
}

fn print_mcp_message(message: &rmcp::model::LoggingMessageNotificationParam) {
    println!("  📊 Level: {:?}", message.level);
    println!("  🏷️  Logger: {:?}", message.logger);
    println!(
        "  📄 Data: {}",
        serde_json::to_string_pretty(&message.data).unwrap()
    );
    println!();
}
