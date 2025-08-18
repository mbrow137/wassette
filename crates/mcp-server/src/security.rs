// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! Security utilities for MCP server operations
//! 
//! This module implements the security requirements from the MCP specification:
//! - Input validation and sanitization
//! - Rate limiting
//! - Output sanitization
//! - Access control validation

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{bail, Result};
use serde_json::Value;
use tracing::{debug, warn};

/// Maximum size for tool input parameters (1MB)
const MAX_INPUT_SIZE: usize = 1024 * 1024;

/// Maximum size for tool output (10MB)
const MAX_OUTPUT_SIZE: usize = 10 * 1024 * 1024;

/// Maximum depth for nested JSON objects
const MAX_JSON_DEPTH: usize = 32;

/// Maximum number of array/object elements
const MAX_COLLECTION_SIZE: usize = 10000;

/// Default rate limit: 100 requests per minute
const DEFAULT_RATE_LIMIT: u32 = 100;

/// Rate limit window duration
const RATE_LIMIT_WINDOW: Duration = Duration::from_secs(60);

/// Input validation configuration
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Maximum input size in bytes
    pub max_input_size: usize,
    /// Maximum JSON nesting depth
    pub max_depth: usize,
    /// Maximum collection size
    pub max_collection_size: usize,
    /// Whether to allow potentially dangerous strings
    pub strict_mode: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_input_size: MAX_INPUT_SIZE,
            max_depth: MAX_JSON_DEPTH,
            max_collection_size: MAX_COLLECTION_SIZE,
            strict_mode: true,
        }
    }
}

/// Rate limiting bucket for token bucket algorithm
#[derive(Debug)]
struct RateLimitBucket {
    tokens: u32,
    last_refill: Instant,
    max_tokens: u32,
    refill_rate: u32, // tokens per second
}

impl RateLimitBucket {
    fn new(max_tokens: u32, refill_rate: u32) -> Self {
        Self {
            tokens: max_tokens,
            last_refill: Instant::now(),
            max_tokens,
            refill_rate,
        }
    }

    fn try_consume(&mut self, tokens: u32) -> bool {
        self.refill();
        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        let tokens_to_add = (elapsed.as_secs_f64() * self.refill_rate as f64) as u32;
        
        if tokens_to_add > 0 {
            self.tokens = (self.tokens + tokens_to_add).min(self.max_tokens);
            self.last_refill = now;
        }
    }
}

/// Rate limiter for tool invocations
#[derive(Debug)]
pub struct RateLimiter {
    buckets: Arc<Mutex<HashMap<String, RateLimitBucket>>>,
    default_limit: u32,
    refill_rate: u32,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(default_limit: u32) -> Self {
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
            default_limit,
            refill_rate: default_limit / 60, // spread over 60 seconds
        }
    }

    /// Check if a request is allowed for the given key
    pub fn allow_request(&self, key: &str, tokens: u32) -> Result<bool> {
        let mut buckets = self.buckets.lock().map_err(|_| anyhow::anyhow!("Rate limiter lock poisoned"))?;
        
        let bucket = buckets.entry(key.to_string()).or_insert_with(|| {
            RateLimitBucket::new(self.default_limit, self.refill_rate)
        });

        Ok(bucket.try_consume(tokens))
    }

    /// Get remaining tokens for a key
    pub fn remaining_tokens(&self, key: &str) -> Result<u32> {
        let mut buckets = self.buckets.lock().map_err(|_| anyhow::anyhow!("Rate limiter lock poisoned"))?;
        
        let bucket = buckets.entry(key.to_string()).or_insert_with(|| {
            RateLimitBucket::new(self.default_limit, self.refill_rate)
        });
        
        bucket.refill();
        Ok(bucket.tokens)
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(DEFAULT_RATE_LIMIT)
    }
}

/// Validates and sanitizes tool input parameters
pub fn validate_tool_input(input: &Value, config: &ValidationConfig) -> Result<()> {
    // Check serialized size
    let serialized = serde_json::to_string(input)?;
    if serialized.len() > config.max_input_size {
        bail!("Input size {} exceeds maximum allowed size {}", serialized.len(), config.max_input_size);
    }

    // Validate JSON structure and content
    validate_json_value(input, config, 0)?;

    debug!("Input validation passed for {} bytes", serialized.len());
    Ok(())
}

/// Recursively validates JSON value structure and content
fn validate_json_value(value: &Value, config: &ValidationConfig, depth: usize) -> Result<()> {
    if depth > config.max_depth {
        bail!("JSON nesting depth {} exceeds maximum {}", depth, config.max_depth);
    }

    match value {
        Value::Object(map) => {
            if map.len() > config.max_collection_size {
                bail!("Object size {} exceeds maximum {}", map.len(), config.max_collection_size);
            }
            
            for (key, val) in map {
                validate_string_content(key, config)?;
                validate_json_value(val, config, depth + 1)?;
            }
        }
        Value::Array(arr) => {
            if arr.len() > config.max_collection_size {
                bail!("Array size {} exceeds maximum {}", arr.len(), config.max_collection_size);
            }
            
            for item in arr {
                validate_json_value(item, config, depth + 1)?;
            }
        }
        Value::String(s) => {
            validate_string_content(s, config)?;
        }
        _ => {} // Numbers, booleans, null are always safe
    }

    Ok(())
}

/// Validates string content for potentially dangerous patterns
fn validate_string_content(s: &str, config: &ValidationConfig) -> Result<()> {
    if !config.strict_mode {
        return Ok(());
    }

    // Check for null bytes
    if s.contains('\0') {
        bail!("String contains null bytes");
    }

    // Check for excessive control characters
    let control_char_count = s.chars().filter(|c| c.is_control() && *c != '\n' && *c != '\r' && *c != '\t').count();
    if control_char_count > s.len() / 10 {
        bail!("String contains excessive control characters");
    }

    // Check for potential script injection patterns
    let dangerous_patterns = [
        "<script", "</script>", "javascript:", "data:text/html",
        "vbscript:", "onload=", "onerror=", "eval(",
        "__proto__", "constructor", "prototype"
    ];

    let lower_s = s.to_lowercase();
    for pattern in &dangerous_patterns {
        if lower_s.contains(pattern) {
            warn!("Potentially dangerous pattern '{}' found in input", pattern);
            bail!("Input contains potentially dangerous pattern: {}", pattern);
        }
    }

    Ok(())
}

/// Sanitizes tool output before returning to client
pub fn sanitize_tool_output(output: &str) -> Result<String> {
    // Check output size
    if output.len() > MAX_OUTPUT_SIZE {
        bail!("Output size {} exceeds maximum allowed size {}", output.len(), MAX_OUTPUT_SIZE);
    }

    // Remove null bytes
    let sanitized = output.replace('\0', "");

    // Limit excessive control characters
    let sanitized = sanitized.chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\r' || *c == '\t')
        .collect::<String>();

    debug!("Output sanitized: {} -> {} bytes", output.len(), sanitized.len());
    Ok(sanitized)
}

/// Validates that the tool name is safe
pub fn validate_tool_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("Tool name cannot be empty");
    }

    if name.len() > 256 {
        bail!("Tool name too long: {}", name.len());
    }

    // Only allow alphanumeric, hyphens, underscores, and dots
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
        bail!("Tool name contains invalid characters: {}", name);
    }

    // Prevent path traversal
    if name.contains("..") || name.starts_with('.') || name.starts_with('/') {
        bail!("Tool name contains unsafe path elements: {}", name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_input_validation_valid() {
        let config = ValidationConfig::default();
        let input = json!({"key": "value", "number": 42});
        
        assert!(validate_tool_input(&input, &config).is_ok());
    }

    #[test]
    fn test_input_validation_too_deep() {
        let config = ValidationConfig {
            max_depth: 2,
            ..Default::default()
        };
        
        let input = json!({"a": {"b": {"c": {"d": "too deep"}}}});
        
        assert!(validate_tool_input(&input, &config).is_err());
    }

    #[test]
    fn test_input_validation_dangerous_pattern() {
        let config = ValidationConfig::default();
        let input = json!({"script": "<script>alert('xss')</script>"});
        
        assert!(validate_tool_input(&input, &config).is_err());
    }

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(10);
        
        // Should allow initial requests
        assert!(limiter.allow_request("user1", 1).unwrap());
        assert!(limiter.allow_request("user1", 5).unwrap());
        assert!(limiter.allow_request("user1", 4).unwrap());
        
        // Should deny when limit exceeded
        assert!(!limiter.allow_request("user1", 1).unwrap());
        
        // Different user should have separate limit
        assert!(limiter.allow_request("user2", 5).unwrap());
    }

    #[test]
    fn test_output_sanitization() {
        let output = "Hello\0World\x01Test\nNewline\tTab";
        let sanitized = sanitize_tool_output(output).unwrap();
        
        assert_eq!(sanitized, "HelloWorldTest\nNewline\tTab");
    }

    #[test]
    fn test_tool_name_validation() {
        assert!(validate_tool_name("valid-tool_name.ext").is_ok());
        assert!(validate_tool_name("../invalid").is_err());
        assert!(validate_tool_name("/invalid").is_err());
        assert!(validate_tool_name("invalid<script>").is_err());
        assert!(validate_tool_name("").is_err());
    }

    #[test]
    fn test_collection_size_limits() {
        let config = ValidationConfig {
            max_collection_size: 2,
            ..Default::default()
        };
        
        let large_array = json!([1, 2, 3]); // Size 3, limit 2
        assert!(validate_tool_input(&large_array, &config).is_err());
        
        let large_object = json!({"a": 1, "b": 2, "c": 3}); // Size 3, limit 2
        assert!(validate_tool_input(&large_object, &config).is_err());
    }
}