// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! Integration tests for MCP security features

use anyhow::Result;
use mcp_server::security::{RateLimiter, ValidationConfig, validate_tool_input, validate_tool_name};
use rmcp::model::CallToolRequestParam;
use serde_json::{json, Map};
use tempfile::TempDir;
use mcp_server::LifecycleManager;

#[tokio::test]
async fn test_security_input_validation_integration() -> Result<()> {
    let config = ValidationConfig::default();
    
    // Test valid input
    let valid_input = json!({
        "component_id": "test-component",
        "details": {"host": "api.example.com"}
    });
    assert!(validate_tool_input(&valid_input, &config).is_ok());
    
    // Test input with dangerous patterns
    let dangerous_input = json!({
        "component_id": "test-component",
        "malicious": "<script>alert('xss')</script>"
    });
    assert!(validate_tool_input(&dangerous_input, &config).is_err());
    
    // Test oversized input
    let large_string = "x".repeat(2_000_000); // 2MB string
    let oversized_input = json!({
        "component_id": "test-component",
        "large_data": large_string
    });
    assert!(validate_tool_input(&oversized_input, &config).is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_security_rate_limiting_integration() -> Result<()> {
    let rate_limiter = RateLimiter::new(5); // 5 requests limit
    
    // Should allow first 5 requests
    for i in 0..5 {
        assert!(
            rate_limiter.allow_request("test-user", 1)?,
            "Request {} should be allowed",
            i + 1
        );
    }
    
    // 6th request should be denied
    assert!(
        !rate_limiter.allow_request("test-user", 1)?,
        "6th request should be denied"
    );
    
    // Different user should have separate limit
    assert!(
        rate_limiter.allow_request("other-user", 1)?,
        "Different user should be allowed"
    );
    
    Ok(())
}

#[tokio::test]
async fn test_security_tool_name_validation_integration() -> Result<()> {
    // Valid tool names
    let valid_names = vec![
        "load-component",
        "grant-storage-permission",
        "my_tool.v1",
        "tool123",
    ];
    
    for name in valid_names {
        assert!(
            validate_tool_name(name).is_ok(),
            "Tool name '{}' should be valid",
            name
        );
    }
    
    // Invalid tool names
    let invalid_names = vec![
        "../malicious",
        "/absolute/path",
        "tool<script>",
        "tool with spaces",
        "",
        ".hidden",
        "tool\0null",
    ];
    
    for name in invalid_names {
        assert!(
            validate_tool_name(name).is_err(),
            "Tool name '{}' should be invalid",
            name
        );
    }
    
    Ok(())
}

#[tokio::test]
async fn test_security_comprehensive_tool_call_validation() -> Result<()> {
    // Test that a complete tool call request would pass validation
    let mut args = Map::new();
    args.insert("component_id".to_string(), json!("test-component"));
    args.insert("details".to_string(), json!({"host": "api.example.com"}));
    
    let req = CallToolRequestParam {
        name: "grant-network-permission".into(),
        arguments: Some(args),
    };
    
    // Validate tool name
    assert!(validate_tool_name(&req.name).is_ok());
    
    // Validate input
    if let Some(ref arguments) = req.arguments {
        let args_value = serde_json::to_value(arguments)?;
        let config = ValidationConfig::default();
        assert!(validate_tool_input(&args_value, &config).is_ok());
    }
    
    Ok(())
}

#[tokio::test]
async fn test_security_edge_cases() -> Result<()> {
    let config = ValidationConfig::default();
    
    // Test empty objects and arrays
    assert!(validate_tool_input(&json!({}), &config).is_ok());
    assert!(validate_tool_input(&json!([]), &config).is_ok());
    
    // Test null values
    assert!(validate_tool_input(&json!(null), &config).is_ok());
    assert!(validate_tool_input(&json!({"key": null}), &config).is_ok());
    
    // Test deeply nested but within limits
    let nested = json!({
        "level1": {
            "level2": {
                "level3": {
                    "level4": "deep but ok"
                }
            }
        }
    });
    assert!(validate_tool_input(&nested, &config).is_ok());
    
    // Test numbers and booleans (should always be safe)
    assert!(validate_tool_input(&json!(42), &config).is_ok());
    assert!(validate_tool_input(&json!(true), &config).is_ok());
    assert!(validate_tool_input(&json!(3.14), &config).is_ok());
    
    Ok(())
}

#[tokio::test] 
async fn test_security_with_lifecycle_manager_context() -> Result<()> {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new()?;
    let lifecycle_manager = LifecycleManager::new(&temp_dir).await?;
    
    // Test that security validation works in the context where it would be used
    // This simulates what happens in handle_tools_call
    
    let valid_request = CallToolRequestParam {
        name: "list-components".into(),
        arguments: None,
    };
    
    // Tool name validation
    assert!(validate_tool_name(&valid_request.name).is_ok());
    
    // Rate limiting
    let rate_limiter = RateLimiter::new(10);
    assert!(rate_limiter.allow_request("test-client", 1)?);
    
    // The actual tool call would happen here, but we're just testing security layers
    
    Ok(())
}

#[tokio::test]
async fn test_security_strict_mode_vs_permissive() -> Result<()> {
    let strict_config = ValidationConfig {
        strict_mode: true,
        ..Default::default()
    };
    
    let permissive_config = ValidationConfig {
        strict_mode: false,
        ..Default::default()
    };
    
    let potentially_dangerous = json!({
        "script": "eval('some code')",
        "proto": "__proto__"
    });
    
    // Strict mode should reject
    assert!(validate_tool_input(&potentially_dangerous, &strict_config).is_err());
    
    // Permissive mode should allow (though real usage should prefer strict)
    assert!(validate_tool_input(&potentially_dangerous, &permissive_config).is_ok());
    
    Ok(())
}