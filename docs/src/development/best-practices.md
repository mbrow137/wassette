# Best Practices

This guide covers production-ready development practices for building secure, maintainable, and performant WebAssembly components for Wassette.

## Development Principles

### 1. Security First

Always design with security as the primary concern:

- **Principle of Least Privilege**: Request only the minimum permissions needed
- **Input Validation**: Validate all inputs from the host environment
- **Output Sanitization**: Ensure outputs don't leak sensitive information
- **Error Handling**: Don't expose internal details in error messages

### 2. Reliability

Build components that work consistently:

- **Graceful Degradation**: Handle resource constraints gracefully
- **Idempotency**: Make operations safe to retry
- **State Management**: Prefer stateless operations when possible
- **Error Recovery**: Implement proper error recovery mechanisms

### 3. Performance

Optimize for efficient resource usage:

- **Memory Efficiency**: Minimize memory allocations and leaks
- **CPU Optimization**: Use efficient algorithms and data structures
- **I/O Optimization**: Batch operations and use appropriate buffer sizes
- **Lazy Loading**: Load resources only when needed

### 4. Maintainability

Write code that's easy to understand and modify:

- **Clear Interfaces**: Design intuitive APIs with comprehensive documentation
- **Consistent Style**: Follow language conventions and team standards
- **Modular Design**: Break complex functionality into smaller, focused components
- **Comprehensive Testing**: Cover all code paths with appropriate tests

## Security Best Practices

### Input Validation

Always validate inputs at component boundaries:

```javascript
// Good: Comprehensive input validation
export function processFile(path, options) {
  // Validate path
  if (!path || typeof path !== 'string') {
    return { tag: 'error', val: 'Invalid path: must be a non-empty string' };
  }
  
  if (path.length > 1000) {
    return { tag: 'error', val: 'Path too long: maximum 1000 characters' };
  }
  
  // Sanitize path
  const sanitizedPath = path.replace(/\.\./g, '').replace(/\/+/g, '/');
  
  // Validate options
  if (!options || typeof options !== 'object') {
    return { tag: 'error', val: 'Invalid options: must be an object' };
  }
  
  const { format, maxSize = 1024 * 1024 } = options;
  
  if (format && !['json', 'xml', 'csv'].includes(format)) {
    return { tag: 'error', val: 'Invalid format: must be json, xml, or csv' };
  }
  
  if (typeof maxSize !== 'number' || maxSize <= 0 || maxSize > 10 * 1024 * 1024) {
    return { tag: 'error', val: 'Invalid maxSize: must be between 1 and 10MB' };
  }
  
  // Process with validated inputs...
}
```

### Permission Management

Request minimal permissions and document why they're needed:

```yaml
# policy.yaml - Well-documented permissions
version: "1.0"
description: "File processing component with minimal permissions"

permissions:
  storage:
    allow:
      # Read access to workspace for input files
      - uri: "fs://workspace/input/**"
        access: ["read"]
        description: "Read input files for processing"
      
      # Write access to output directory only
      - uri: "fs://workspace/output/**"
        access: ["write"]
        description: "Write processed files to output directory"
    
    # Explicitly deny sensitive directories
    deny:
      - uri: "fs:///etc/**"
        description: "Prevent access to system configuration"
      - uri: "fs:///home/**"
        description: "Prevent access to user directories"

  network:
    allow:
      # Only allow specific API endpoints
      - host: "api.processor.com"
        ports: [443]
        description: "Access processing API for data enrichment"
    
    deny:
      - host: "localhost"
        description: "Prevent local network access"
      - host: "127.0.0.1"
        description: "Prevent loopback access"

  # No environment variable access needed
  environment: {}

# Resource limits to prevent abuse
limits:
  memory: "128MB"
  cpu_time: "30s"
  file_handles: 10
  network_requests: 50
```

### Secure Error Handling

Don't leak sensitive information in errors:

```rust
// Good: Generic error messages
fn read_config_file(path: &str) -> Result<Config, String> {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            match serde_json::from_str::<Config>(&content) {
                Ok(config) => Ok(config),
                Err(_) => Err("Invalid configuration format".to_string())
            }
        }
        Err(_) => Err("Configuration file not accessible".to_string())
    }
}

// Bad: Exposes internal details
fn read_config_file_bad(path: &str) -> Result<Config, String> {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            match serde_json::from_str::<Config>(&content) {
                Ok(config) => Ok(config),
                // Don't expose detailed parsing errors
                Err(e) => Err(format!("JSON parse error: {}", e))
            }
        }
        // Don't expose file system details
        Err(e) => Err(format!("Failed to read {}: {}", path, e))
    }
}
```

## Performance Best Practices

### Memory Management

Efficient memory usage is crucial in the WebAssembly environment:

```rust
// Good: Efficient memory usage
fn process_large_dataset(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut result = Vec::with_capacity(data.len() / 2); // Pre-allocate
    
    // Process in chunks to avoid large allocations
    for chunk in data.chunks(1024) {
        let processed = process_chunk(chunk)?;
        result.extend_from_slice(&processed);
    }
    
    result.shrink_to_fit(); // Free unused capacity
    Ok(result)
}

// Good: Use streaming for large data
fn process_stream<R: Read, W: Write>(
    mut reader: R, 
    mut writer: W
) -> Result<(), String> {
    let mut buffer = [0; 4096]; // Fixed-size buffer
    
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                let processed = process_chunk(&buffer[..n])?;
                writer.write_all(&processed)
                    .map_err(|_| "Write error".to_string())?;
            }
            Err(_) => return Err("Read error".to_string()),
        }
    }
    
    Ok(())
}
```

### Algorithm Optimization

Choose appropriate algorithms and data structures:

```javascript
// Good: Efficient algorithms
class DataProcessor {
  constructor() {
    this.cache = new Map(); // Use Map for O(1) lookups
    this.sorted_data = []; // Keep data sorted for binary search
  }
  
  // Use caching for expensive operations
  processData(id, data) {
    if (this.cache.has(id)) {
      return this.cache.get(id);
    }
    
    const result = this.expensiveOperation(data);
    this.cache.set(id, result);
    
    // Limit cache size to prevent memory bloat
    if (this.cache.size > 1000) {
      const firstKey = this.cache.keys().next().value;
      this.cache.delete(firstKey);
    }
    
    return result;
  }
  
  // Use binary search for sorted data
  findData(key) {
    let left = 0;
    let right = this.sorted_data.length - 1;
    
    while (left <= right) {
      const mid = Math.floor((left + right) / 2);
      const midValue = this.sorted_data[mid];
      
      if (midValue.key === key) {
        return midValue;
      } else if (midValue.key < key) {
        left = mid + 1;
      } else {
        right = mid - 1;
      }
    }
    
    return null;
  }
}
```

### Resource Cleanup

Always clean up resources properly:

```python
# Good: Proper resource management
def process_files(input_paths, output_path):
    """Process multiple files with proper resource cleanup"""
    output_file = None
    try:
        output_file = open(output_path, 'w')
        
        for input_path in input_paths:
            # Use context manager for automatic cleanup
            with open(input_path, 'r') as input_file:
                data = input_file.read()
                processed = process_data(data)
                output_file.write(processed)
                
    except Exception as e:
        # Clean up on error
        if output_file:
            output_file.close()
            try:
                os.remove(output_path)  # Remove partial file
            except:
                pass
        raise ProcessingError(f"File processing failed: {str(e)}")
    
    finally:
        if output_file:
            output_file.close()

# Even better: Use context managers
from contextlib import contextmanager

@contextmanager
def batch_processor(output_path):
    """Context manager for batch processing"""
    output_file = None
    try:
        output_file = open(output_path, 'w')
        yield output_file
    except Exception:
        # Clean up on error
        if output_file:
            output_file.close()
            try:
                os.remove(output_path)
            except:
                pass
        raise
    finally:
        if output_file:
            output_file.close()

# Usage
def process_files_better(input_paths, output_path):
    with batch_processor(output_path) as output_file:
        for input_path in input_paths:
            with open(input_path, 'r') as input_file:
                data = input_file.read()
                processed = process_data(data)
                output_file.write(processed)
```

## Interface Design

### WIT Best Practices

Design clear and extensible interfaces:

```wit
// Good: Well-designed interface
package my-component:api@1.0.0;

/// File processing interface with comprehensive types
interface file-processor {
  /// Processing options with sensible defaults
  record processing-options {
    /// Output format (json, xml, csv)
    format: string,
    /// Enable compression
    compression: bool,
    /// Maximum file size in bytes
    max-size: u64,
    /// Processing timeout in seconds
    timeout: option<u32>,
  }
  
  /// Processing result with detailed information
  record processing-result {
    /// Success indicator
    success: bool,
    /// Result data or error message
    message: string,
    /// Number of records processed
    records-processed: u32,
    /// Processing time in milliseconds
    processing-time: u64,
  }
  
  /// File processing errors
  variant file-error {
    /// File not found
    not-found,
    /// File too large
    too-large(u64),
    /// Invalid format
    invalid-format(string),
    /// Permission denied
    permission-denied,
    /// Processing timeout
    timeout,
  }
  
  /// Process a single file
  /// 
  /// # Parameters
  /// - `path`: Input file path
  /// - `options`: Processing options
  /// 
  /// # Returns
  /// Result containing processing information or error details
  process-file: func(
    path: string, 
    options: processing-options
  ) -> result<processing-result, file-error>;
  
  /// Process multiple files in batch
  /// 
  /// # Parameters
  /// - `paths`: List of input file paths
  /// - `output-path`: Output file path
  /// - `options`: Processing options
  /// 
  /// # Returns
  /// Result containing batch processing information
  process-batch: func(
    paths: list<string>,
    output-path: string,
    options: processing-options
  ) -> result<processing-result, file-error>;
  
  /// Get supported file formats
  /// 
  /// # Returns
  /// List of supported format strings
  get-supported-formats: func() -> list<string>;
}

/// Main component world
world file-processor-tool {
  export file-processor;
}
```

### API Evolution

Design for backward compatibility:

```wit
// Version 1.0.0
package my-component:api@1.0.0;

interface processor {
  record options {
    format: string,
    compression: bool,
  }
  
  process: func(data: string, options: options) -> string;
}

// Version 1.1.0 - Add new optional fields
package my-component:api@1.1.0;

interface processor {
  record options {
    format: string,
    compression: bool,
    // New optional fields maintain compatibility
    timeout: option<u32>,
    max-size: option<u64>,
  }
  
  // Original function unchanged
  process: func(data: string, options: options) -> string;
  
  // New function for enhanced capabilities
  process-enhanced: func(
    data: string, 
    options: options
  ) -> result<string, string>;
}
```

## Error Handling

### Comprehensive Error Types

Define clear error types for different failure modes:

```rust
// Good: Comprehensive error handling
#[derive(Debug)]
pub enum ProcessingError {
    InputValidation(String),
    FileSystem(String),
    Network(String),
    Parsing(String),
    ResourceExhausted(String),
    Timeout,
    PermissionDenied,
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProcessingError::InputValidation(msg) => write!(f, "Input validation error: {}", msg),
            ProcessingError::FileSystem(_) => write!(f, "File system error"),
            ProcessingError::Network(_) => write!(f, "Network error"),
            ProcessingError::Parsing(_) => write!(f, "Data parsing error"),
            ProcessingError::ResourceExhausted(_) => write!(f, "Resource limit exceeded"),
            ProcessingError::Timeout => write!(f, "Operation timed out"),
            ProcessingError::PermissionDenied => write!(f, "Permission denied"),
        }
    }
}

// Convert to WIT-compatible error messages
impl Into<String> for ProcessingError {
    fn into(self) -> String {
        self.to_string()
    }
}
```

### Graceful Degradation

Handle resource constraints gracefully:

```javascript
// Good: Graceful degradation
class DataProcessor {
  constructor() {
    this.memoryLimit = 64 * 1024 * 1024; // 64MB
    this.timeLimit = 30000; // 30 seconds
  }
  
  async processData(data) {
    const startTime = Date.now();
    
    try {
      // Check if we can process all at once
      if (data.length * 4 < this.memoryLimit) {
        return await this.processFull(data);
      }
      
      // Fall back to streaming processing
      console.log('Large dataset detected, using streaming mode');
      return await this.processStreaming(data);
      
    } catch (error) {
      // Check if we're running out of time
      if (Date.now() - startTime > this.timeLimit * 0.9) {
        return this.processPartial(data);
      }
      
      throw error;
    }
  }
  
  async processPartial(data) {
    // Process subset of data when resources are constrained
    const sampleSize = Math.min(1000, data.length);
    const sample = data.slice(0, sampleSize);
    
    const result = await this.processFull(sample);
    result.partial = true;
    result.sampleSize = sampleSize;
    result.totalSize = data.length;
    
    return result;
  }
}
```

## Testing Best Practices

### Test Coverage

Ensure comprehensive test coverage:

```python
# Good: Comprehensive test coverage
import pytest
from unittest.mock import patch, mock_open

class TestFileProcessor:
    
    def test_valid_input(self):
        """Test normal operation with valid input"""
        result = process_file("test.txt", {"format": "json"})
        assert result["success"] is True
    
    def test_invalid_path(self):
        """Test error handling for invalid paths"""
        with pytest.raises(ValueError, match="Invalid path"):
            process_file("", {"format": "json"})
    
    def test_unsupported_format(self):
        """Test error handling for unsupported formats"""
        with pytest.raises(ValueError, match="Unsupported format"):
            process_file("test.txt", {"format": "unknown"})
    
    def test_file_not_found(self):
        """Test error handling for missing files"""
        with pytest.raises(FileNotFoundError):
            process_file("nonexistent.txt", {"format": "json"})
    
    @patch("builtins.open", side_effect=PermissionError())
    def test_permission_denied(self, mock_file):
        """Test error handling for permission errors"""
        with pytest.raises(PermissionError):
            process_file("restricted.txt", {"format": "json"})
    
    def test_large_file_handling(self):
        """Test handling of large files"""
        # Create large mock data
        large_data = "x" * (10 * 1024 * 1024)  # 10MB
        
        with patch("builtins.open", mock_open(read_data=large_data)):
            result = process_file("large.txt", {"format": "json"})
            # Should handle gracefully or return partial result
            assert result["success"] is True or result.get("partial") is True
    
    def test_resource_limits(self):
        """Test behavior under resource constraints"""
        # Test memory limit
        with patch("psutil.virtual_memory") as mock_memory:
            mock_memory.return_value.available = 1024 * 1024  # 1MB available
            result = process_file("test.txt", {"format": "json"})
            # Should degrade gracefully
            assert "warning" in result or result.get("partial") is True
```

### Property-Based Testing

Use property-based testing for robust validation:

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};
    
    // Property: encoding then decoding should return original data
    fn prop_encode_decode_roundtrip(data: Vec<u8>) -> bool {
        if let Ok(encoded) = encode_data(&data) {
            if let Ok(decoded) = decode_data(&encoded) {
                return decoded == data;
            }
        }
        true // Allow encoding/decoding to fail, but not to corrupt data
    }
    
    // Property: processing should not increase data size beyond expected bounds
    fn prop_compression_ratio(data: Vec<u8>) -> TestResult {
        if data.is_empty() {
            return TestResult::discard();
        }
        
        match compress_data(&data) {
            Ok(compressed) => {
                // Compressed data should not be more than 2x original size
                // (accounting for small data overhead)
                let max_size = if data.len() < 100 {
                    data.len() * 3
                } else {
                    data.len() * 2
                };
                TestResult::from_bool(compressed.len() <= max_size)
            }
            Err(_) => TestResult::passed() // Allow compression to fail
        }
    }
    
    quickcheck! {
        fn test_encode_decode_roundtrip(data: Vec<u8>) -> bool {
            prop_encode_decode_roundtrip(data)
        }
        
        fn test_compression_ratio(data: Vec<u8>) -> TestResult {
            prop_compression_ratio(data)
        }
    }
}
```

## Documentation

### Code Documentation

Document all public interfaces thoroughly:

```rust
/// File processing component with support for multiple formats
/// 
/// This component provides secure file processing capabilities with
/// built-in format validation, size limits, and error handling.
/// 
/// # Security Considerations
/// 
/// - Only processes files within allowed directories
/// - Enforces maximum file size limits
/// - Validates all input formats
/// - Does not expose internal file paths in errors
/// 
/// # Performance Characteristics
/// 
/// - Memory usage: O(file_size) for small files, O(1) for streaming
/// - Time complexity: O(n) where n is file size
/// - Maximum file size: 10MB
/// - Processing timeout: 30 seconds
pub struct FileProcessor {
    /// Maximum file size in bytes
    max_file_size: usize,
    /// Processing timeout in milliseconds
    timeout_ms: u64,
}

impl FileProcessor {
    /// Create a new file processor with default limits
    /// 
    /// # Default Limits
    /// 
    /// - Maximum file size: 10MB
    /// - Processing timeout: 30 seconds
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let processor = FileProcessor::new();
    /// let result = processor.process_file("data.json", &options)?;
    /// ```
    pub fn new() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024,
            timeout_ms: 30_000,
        }
    }
    
    /// Process a file with the specified options
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the input file (must be within allowed directories)
    /// * `options` - Processing options including format and compression settings
    /// 
    /// # Returns
    /// 
    /// * `Ok(ProcessingResult)` - Processing completed successfully
    /// * `Err(ProcessingError)` - Processing failed with specific error
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// 
    /// - The file path is invalid or outside allowed directories
    /// - The file size exceeds the maximum limit
    /// - The file format is not supported
    /// - Processing times out
    /// - Insufficient permissions to read the file
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let processor = FileProcessor::new();
    /// let options = ProcessingOptions {
    ///     format: "json".to_string(),
    ///     compression: true,
    ///     max_size: 1024 * 1024, // 1MB
    ///     timeout: Some(10), // 10 seconds
    /// };
    /// 
    /// match processor.process_file("data.json", &options) {
    ///     Ok(result) => println!("Processed {} records", result.records_processed),
    ///     Err(e) => eprintln!("Processing failed: {}", e),
    /// }
    /// ```
    pub fn process_file(
        &self,
        path: &str,
        options: &ProcessingOptions,
    ) -> Result<ProcessingResult, ProcessingError> {
        // Implementation...
    }
}
```

### User Documentation

Provide comprehensive user guides:

```markdown
# File Processor Component

## Overview

The File Processor component provides secure file processing capabilities for AI agents. It supports multiple formats, automatic compression, and built-in safety limits.

## Usage

### Basic Processing

Process a single file:

```
Process the file data.json with JSON format
```

### Batch Processing

Process multiple files:

```
Process all CSV files in the input directory and save to output.json
```

### Configuration Options

Available options:

- **format**: Output format (json, xml, csv)
- **compression**: Enable compression (true/false)
- **max-size**: Maximum file size in bytes
- **timeout**: Processing timeout in seconds

## Security

### Permissions Required

- **Storage**: Read access to input directories, write access to output directories
- **Network**: None (processes files locally)

### Safety Features

- File size limits (10MB default)
- Path validation (prevents directory traversal)
- Format validation
- Processing timeouts

## Error Handling

Common errors and solutions:

| Error | Cause | Solution |
|-------|-------|----------|
| Permission denied | Insufficient file permissions | Grant storage permissions |
| File too large | File exceeds size limit | Use smaller files or increase limit |
| Invalid format | Unsupported file format | Use supported format (json, xml, csv) |
| Timeout | Processing took too long | Increase timeout or use smaller files |

## Examples

### Process JSON file with compression

```
Please process data.json with compression enabled and save as output.xml
```

### Batch process with custom limits

```
Process all files in /workspace/input with max size 5MB and 60 second timeout
```
```

## Monitoring and Observability

### Logging

Implement structured logging for debugging and monitoring:

```rust
use log::{info, warn, error, debug};

impl FileProcessor {
    pub fn process_file(&self, path: &str, options: &ProcessingOptions) -> Result<ProcessingResult, ProcessingError> {
        let start_time = std::time::Instant::now();
        
        info!(
            "Starting file processing";
            "path" => path,
            "format" => &options.format,
            "compression" => options.compression,
            "max_size" => options.max_size
        );
        
        // Validate input
        if let Err(e) = self.validate_input(path, options) {
            warn!(
                "Input validation failed";
                "path" => path,
                "error" => %e
            );
            return Err(e);
        }
        
        // Process file
        match self.process_internal(path, options) {
            Ok(result) => {
                let duration = start_time.elapsed();
                info!(
                    "File processing completed";
                    "path" => path,
                    "records_processed" => result.records_processed,
                    "duration_ms" => duration.as_millis(),
                    "success" => true
                );
                Ok(result)
            }
            Err(e) => {
                error!(
                    "File processing failed";
                    "path" => path,
                    "error" => %e,
                    "duration_ms" => start_time.elapsed().as_millis()
                );
                Err(e)
            }
        }
    }
}
```

### Metrics

Track key performance indicators:

```javascript
class MetricsCollector {
  constructor() {
    this.metrics = {
      filesProcessed: 0,
      totalProcessingTime: 0,
      errorCount: 0,
      memoryUsage: [],
    };
  }
  
  recordProcessing(result) {
    this.metrics.filesProcessed++;
    this.metrics.totalProcessingTime += result.processingTime;
    
    // Track memory usage
    if (typeof performance !== 'undefined' && performance.memory) {
      this.metrics.memoryUsage.push({
        timestamp: Date.now(),
        used: performance.memory.usedJSHeapSize,
        total: performance.memory.totalJSHeapSize,
      });
      
      // Keep only last 100 measurements
      if (this.metrics.memoryUsage.length > 100) {
        this.metrics.memoryUsage.shift();
      }
    }
  }
  
  recordError(error) {
    this.metrics.errorCount++;
    console.error('Processing error:', error);
  }
  
  getMetrics() {
    const avgProcessingTime = this.metrics.filesProcessed > 0 
      ? this.metrics.totalProcessingTime / this.metrics.filesProcessed 
      : 0;
    
    return {
      filesProcessed: this.metrics.filesProcessed,
      averageProcessingTime: avgProcessingTime,
      errorRate: this.metrics.errorCount / Math.max(1, this.metrics.filesProcessed),
      currentMemoryUsage: this.getCurrentMemoryUsage(),
    };
  }
  
  getCurrentMemoryUsage() {
    if (this.metrics.memoryUsage.length === 0) return null;
    
    const latest = this.metrics.memoryUsage[this.metrics.memoryUsage.length - 1];
    return {
      used: latest.used,
      total: latest.total,
      percentage: (latest.used / latest.total) * 100,
    };
  }
}
```

## Production Deployment

### Component Packaging

Create complete component packages:

```yaml
# component-manifest.yaml
apiVersion: v1
kind: WasmComponent
metadata:
  name: file-processor
  version: "1.2.0"
  description: "Secure file processing component"
  author: "team@company.com"
  
spec:
  component: "./file-processor.wasm"
  policy: "./policy.yaml"
  
  interfaces:
    - name: "file-processor"
      version: "1.2.0"
      description: "Main file processing interface"
  
  dependencies:
    - name: "wasi:filesystem"
      version: "0.2.0"
    - name: "wasi:io"
      version: "0.2.0"
  
  metadata:
    documentation: "./docs/"
    examples: "./examples/"
    tests: "./tests/"
  
  build:
    language: "rust"
    toolchain: "1.75.0"
    target: "wasm32-wasip2"
    optimizations: true
```

### Release Checklist

Before releasing a component:

- [ ] All tests pass (unit, integration, security, performance)
- [ ] Documentation is complete and up-to-date
- [ ] Security policy is minimal and well-documented
- [ ] Performance benchmarks meet requirements
- [ ] Version follows semantic versioning
- [ ] CHANGELOG.md is updated
- [ ] Component is signed and verified
- [ ] Examples and tutorials are tested
- [ ] Breaking changes are documented
- [ ] Migration guide is provided (if needed)

## Next Steps

- Review [Testing Components](./testing.md) for comprehensive testing strategies
- Explore [Cookbook](../cookbook/common-patterns.md) for practical examples
- Learn about [Security Model](../security/security-model.md) for advanced security practices
- Check [Contributing Guidelines](../contributing.md) for project standards