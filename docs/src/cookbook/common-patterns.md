# Common Patterns

This cookbook provides practical examples and patterns for building WebAssembly components with Wassette. Each pattern includes complete code examples, security considerations, and best practices.

## Pattern Categories

### Data Processing Patterns
- [File Processing](#file-processing)
- [Data Transformation](#data-transformation)
- [Streaming Processing](#streaming-processing)

### Integration Patterns
- [API Integration](#api-integration)
- [Database Access](#database-access)
- [Service Communication](#service-communication)

### Utility Patterns
- [Configuration Management](#configuration-management)
- [Error Handling](#error-handling)
- [Resource Management](#resource-management)

### Performance Patterns
- [Caching](#caching)
- [Batch Processing](#batch-processing)
- [Memory Optimization](#memory-optimization)

## File Processing

### Basic File Reader

Read and process files with proper error handling and security:

```wit
// wit/file-reader.wit
package file-processor:reader@1.0.0;

interface file-reader {
  record file-info {
    path: string,
    size: u64,
    mime-type: option<string>,
  }
  
  variant file-error {
    not-found,
    permission-denied,
    too-large(u64),
    invalid-format,
  }
  
  read-file: func(path: string) -> result<string, file-error>;
  get-file-info: func(path: string) -> result<file-info, file-error>;
  list-files: func(directory: string) -> result<list<string>, file-error>;
}

world file-reader-tool {
  export file-reader;
}
```

**JavaScript Implementation:**

```javascript
// main.js
import { readFile, stat, readdir } from 'node:fs/promises';
import { join, extname } from 'node:path';

const MAX_FILE_SIZE = 10 * 1024 * 1024; // 10MB

export async function readFile(path) {
  try {
    // Validate input
    if (!path || typeof path !== 'string') {
      return { tag: 'err', val: { tag: 'invalid-format' } };
    }
    
    // Get file info first to check size
    const fileInfo = await getFileInfo(path);
    if (fileInfo.tag === 'err') {
      return fileInfo;
    }
    
    // Check file size
    if (fileInfo.val.size > MAX_FILE_SIZE) {
      return { 
        tag: 'err', 
        val: { tag: 'too-large', val: fileInfo.val.size } 
      };
    }
    
    // Read file content
    const content = await readFile(path, 'utf8');
    return { tag: 'ok', val: content };
    
  } catch (error) {
    if (error.code === 'ENOENT') {
      return { tag: 'err', val: { tag: 'not-found' } };
    } else if (error.code === 'EACCES') {
      return { tag: 'err', val: { tag: 'permission-denied' } };
    } else {
      return { tag: 'err', val: { tag: 'invalid-format' } };
    }
  }
}

export async function getFileInfo(path) {
  try {
    const stats = await stat(path);
    
    if (!stats.isFile()) {
      return { tag: 'err', val: { tag: 'invalid-format' } };
    }
    
    const mimeType = getMimeType(path);
    
    return {
      tag: 'ok',
      val: {
        path,
        size: stats.size,
        mimeType: mimeType ? { tag: 'some', val: mimeType } : { tag: 'none' }
      }
    };
  } catch (error) {
    if (error.code === 'ENOENT') {
      return { tag: 'err', val: { tag: 'not-found' } };
    } else if (error.code === 'EACCES') {
      return { tag: 'err', val: { tag: 'permission-denied' } };
    } else {
      return { tag: 'err', val: { tag: 'invalid-format' } };
    }
  }
}

export async function listFiles(directory) {
  try {
    const entries = await readdir(directory);
    const files = [];
    
    for (const entry of entries) {
      const fullPath = join(directory, entry);
      try {
        const stats = await stat(fullPath);
        if (stats.isFile()) {
          files.push(fullPath);
        }
      } catch {
        // Skip files we can't stat
        continue;
      }
    }
    
    return { tag: 'ok', val: files };
  } catch (error) {
    if (error.code === 'ENOENT') {
      return { tag: 'err', val: { tag: 'not-found' } };
    } else if (error.code === 'EACCES') {
      return { tag: 'err', val: { tag: 'permission-denied' } };
    } else {
      return { tag: 'err', val: { tag: 'invalid-format' } };
    }
  }
}

function getMimeType(path) {
  const ext = extname(path).toLowerCase();
  const mimeTypes = {
    '.txt': 'text/plain',
    '.json': 'application/json',
    '.xml': 'application/xml',
    '.csv': 'text/csv',
    '.md': 'text/markdown',
    '.html': 'text/html',
    '.pdf': 'application/pdf',
  };
  return mimeTypes[ext];
}
```

**Security Policy:**

```yaml
# policy.yaml
version: "1.0"
description: "File reader with minimal permissions"

permissions:
  storage:
    allow:
      - uri: "fs://workspace/**"
        access: ["read"]
        description: "Read files from workspace"
      - uri: "fs://input/**"
        access: ["read"]
        description: "Read input files"
    deny:
      - uri: "fs:///etc/**"
        description: "Block system configuration"
      - uri: "fs:///home/**"
        description: "Block user directories"

limits:
  memory: "64MB"
  cpu_time: "10s"
  file_handles: 10
```

### CSV Data Processor

Process CSV files with validation and transformation:

```rust
// src/lib.rs
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

wit_bindgen::generate!({
    path: "../wit/csv-processor.wit",
    world: "csv-processor",
});

#[derive(Deserialize, Serialize)]
struct Record {
    #[serde(flatten)]
    fields: HashMap<String, String>,
}

struct Component;

impl Guest for Component {
    fn process_csv(input_path: String, output_path: String, options: ProcessingOptions) -> Result<ProcessingResult, CsvError> {
        let start_time = std::time::Instant::now();
        let mut processed_count = 0;
        
        // Validate inputs
        if input_path.is_empty() || output_path.is_empty() {
            return Err(CsvError::InvalidInput);
        }
        
        // Open input file
        let input_file = std::fs::File::open(&input_path)
            .map_err(|_| CsvError::FileNotFound)?;
        
        let mut reader = Reader::from_reader(input_file);
        
        // Create output file
        let output_file = std::fs::File::create(&output_path)
            .map_err(|_| CsvError::PermissionDenied)?;
        
        let mut writer = Writer::from_writer(output_file);
        
        // Process headers
        let headers = reader.headers()
            .map_err(|_| CsvError::InvalidFormat)?
            .clone();
        
        // Filter headers if needed
        let filtered_headers = if let Some(ref columns) = options.select_columns {
            headers.iter()
                .filter(|h| columns.contains(&h.to_string()))
                .map(|h| h.to_string())
                .collect::<Vec<_>>()
        } else {
            headers.iter().map(|h| h.to_string()).collect()
        };
        
        writer.write_record(&filtered_headers)
            .map_err(|_| CsvError::WriteError)?;
        
        // Process records
        for result in reader.records() {
            let record = result.map_err(|_| CsvError::InvalidFormat)?;
            
            // Convert to HashMap for easy manipulation
            let mut record_map = HashMap::new();
            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    record_map.insert(header.to_string(), field.to_string());
                }
            }
            
            // Apply transformations
            if let Some(ref transforms) = options.transformations {
                apply_transformations(&mut record_map, transforms)?;
            }
            
            // Filter record if needed
            if let Some(ref filter) = options.filter {
                if !apply_filter(&record_map, filter) {
                    continue;
                }
            }
            
            // Write filtered fields
            let output_record: Vec<String> = filtered_headers.iter()
                .map(|h| record_map.get(h).cloned().unwrap_or_default())
                .collect();
            
            writer.write_record(&output_record)
                .map_err(|_| CsvError::WriteError)?;
            
            processed_count += 1;
            
            // Check limits
            if processed_count >= options.max_records.unwrap_or(100000) {
                break;
            }
        }
        
        writer.flush().map_err(|_| CsvError::WriteError)?;
        
        Ok(ProcessingResult {
            records_processed: processed_count,
            processing_time: start_time.elapsed().as_millis() as u64,
            output_path,
        })
    }
}

fn apply_transformations(record: &mut HashMap<String, String>, transforms: &[Transformation]) -> Result<(), CsvError> {
    for transform in transforms {
        match transform {
            Transformation::Uppercase(field) => {
                if let Some(value) = record.get_mut(field) {
                    *value = value.to_uppercase();
                }
            }
            Transformation::Lowercase(field) => {
                if let Some(value) = record.get_mut(field) {
                    *value = value.to_lowercase();
                }
            }
            Transformation::Trim(field) => {
                if let Some(value) = record.get_mut(field) {
                    *value = value.trim().to_string();
                }
            }
            Transformation::Replace { field, from, to } => {
                if let Some(value) = record.get_mut(field) {
                    *value = value.replace(from, to);
                }
            }
        }
    }
    Ok(())
}

fn apply_filter(record: &HashMap<String, String>, filter: &FilterCondition) -> bool {
    match filter {
        FilterCondition::Equals { field, value } => {
            record.get(field).map_or(false, |v| v == value)
        }
        FilterCondition::Contains { field, value } => {
            record.get(field).map_or(false, |v| v.contains(value))
        }
        FilterCondition::NotEmpty(field) => {
            record.get(field).map_or(false, |v| !v.trim().is_empty())
        }
    }
}

export!(Component);
```

## API Integration

### HTTP Client with Retry Logic

Robust HTTP client with retry, timeout, and error handling:

```python
# main.py
import asyncio
import json
import time
from dataclasses import dataclass
from typing import Optional, Dict, Any
from urllib.parse import urljoin, urlparse

@dataclass
class HttpRequest:
    url: str
    method: str
    headers: Optional[Dict[str, str]] = None
    body: Optional[str] = None
    timeout: Optional[int] = None

@dataclass
class HttpResponse:
    status: int
    headers: Dict[str, str]
    body: str
    duration: int

@dataclass
class RetryConfig:
    max_attempts: int = 3
    base_delay: float = 1.0
    max_delay: float = 60.0
    exponential_base: float = 2.0

class HttpError(Exception):
    def __init__(self, message: str, status_code: Optional[int] = None):
        self.message = message
        self.status_code = status_code
        super().__init__(message)

async def http_request(request: HttpRequest, retry_config: Optional[RetryConfig] = None) -> HttpResponse:
    """Make HTTP request with retry logic."""
    if not retry_config:
        retry_config = RetryConfig()
    
    # Validate URL
    parsed_url = urlparse(request.url)
    if not parsed_url.scheme or not parsed_url.netloc:
        raise HttpError("Invalid URL format")
    
    # Ensure HTTPS for security
    if parsed_url.scheme != "https":
        raise HttpError("Only HTTPS URLs are allowed")
    
    last_error = None
    
    for attempt in range(retry_config.max_attempts):
        try:
            start_time = time.time()
            
            # Make the actual HTTP request
            response = await _make_request(request)
            
            duration = int((time.time() - start_time) * 1000)
            
            # Check for success status codes
            if 200 <= response.status < 300:
                return HttpResponse(
                    status=response.status,
                    headers=dict(response.headers),
                    body=response.body,
                    duration=duration
                )
            elif response.status >= 500 and attempt < retry_config.max_attempts - 1:
                # Retry on server errors
                last_error = HttpError(f"Server error: {response.status}", response.status)
                await _wait_for_retry(attempt, retry_config)
                continue
            else:
                # Don't retry on client errors
                raise HttpError(f"HTTP error: {response.status}", response.status)
                
        except asyncio.TimeoutError:
            last_error = HttpError("Request timeout")
            if attempt < retry_config.max_attempts - 1:
                await _wait_for_retry(attempt, retry_config)
                continue
        except Exception as e:
            last_error = HttpError(f"Request failed: {str(e)}")
            if attempt < retry_config.max_attempts - 1:
                await _wait_for_retry(attempt, retry_config)
                continue
    
    # All retries exhausted
    raise last_error or HttpError("Request failed after all retries")

async def _make_request(request: HttpRequest) -> HttpResponse:
    """Make the actual HTTP request (implementation depends on runtime)."""
    # This would use the WASI HTTP interface in a real implementation
    # For now, this is a placeholder
    
    # Set default timeout
    timeout = request.timeout or 30
    
    # Prepare headers
    headers = request.headers or {}
    if 'User-Agent' not in headers:
        headers['User-Agent'] = 'Wassette-Component/1.0'
    
    # Add content-length for POST/PUT requests
    if request.body and request.method.upper() in ['POST', 'PUT', 'PATCH']:
        headers['Content-Length'] = str(len(request.body.encode('utf-8')))
        if 'Content-Type' not in headers:
            headers['Content-Type'] = 'application/json'
    
    # Simulate HTTP request (replace with actual WASI HTTP call)
    await asyncio.sleep(0.1)  # Simulate network delay
    
    return HttpResponse(
        status=200,
        headers={"Content-Type": "application/json"},
        body='{"success": true}',
        duration=100
    )

async def _wait_for_retry(attempt: int, config: RetryConfig) -> None:
    """Calculate and wait for retry delay."""
    delay = min(
        config.base_delay * (config.exponential_base ** attempt),
        config.max_delay
    )
    await asyncio.sleep(delay)

# Component interface functions
def make_http_request(url: str, method: str, headers: dict, body: str, timeout: int) -> dict:
    """Main component function for making HTTP requests."""
    try:
        request = HttpRequest(
            url=url,
            method=method,
            headers=headers if headers else None,
            body=body if body else None,
            timeout=timeout if timeout > 0 else None
        )
        
        # Run async request
        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        try:
            response = loop.run_until_complete(http_request(request))
            return {
                "success": True,
                "status": response.status,
                "headers": response.headers,
                "body": response.body,
                "duration": response.duration
            }
        finally:
            loop.close()
            
    except HttpError as e:
        return {
            "success": False,
            "error": e.message,
            "status_code": e.status_code
        }
    except Exception as e:
        return {
            "success": False,
            "error": f"Unexpected error: {str(e)}"
        }

def fetch_json(url: str, headers: dict) -> dict:
    """Convenience function for fetching JSON data."""
    response = make_http_request(url, "GET", headers, "", 30)
    
    if not response["success"]:
        return response
    
    try:
        parsed_body = json.loads(response["body"])
        response["json"] = parsed_body
        return response
    except json.JSONDecodeError:
        return {
            "success": False,
            "error": "Response is not valid JSON"
        }

def post_json(url: str, data: dict, headers: dict) -> dict:
    """Convenience function for posting JSON data."""
    json_headers = headers.copy() if headers else {}
    json_headers["Content-Type"] = "application/json"
    
    try:
        json_body = json.dumps(data)
        return make_http_request(url, "POST", json_headers, json_body, 30)
    except json.JSONEncodeError:
        return {
            "success": False,
            "error": "Failed to encode data as JSON"
        }
```

## Data Transformation

### JSON to XML Converter

Convert between data formats with validation:

```go
// main.go
package main

import (
    "encoding/json"
    "encoding/xml"
    "fmt"
    "strings"
)

//go:generate wit-bindgen tiny-go wit/ --out-dir=gen
import "github.com/bytecodealliance/wasm-tools-go/cm"

// Component represents our component implementation
type Component struct{}

// ConvertJsonToXml converts JSON to XML with options
func (c Component) ConvertJsonToXml(jsonData string, options ConversionOptions) ConvertResult {
    startTime := getCurrentTimestamp()
    
    // Validate input
    if len(jsonData) == 0 {
        return ConvertResult{
            Success: false,
            Error: cm.Some("Input JSON data is empty"),
        }
    }
    
    if len(jsonData) > int(options.MaxInputSize) {
        return ConvertResult{
            Success: false,
            Error: cm.Some(fmt.Sprintf("Input too large: %d bytes (max: %d)", len(jsonData), options.MaxInputSize)),
        }
    }
    
    // Parse JSON
    var data interface{}
    if err := json.Unmarshal([]byte(jsonData), &data); err != nil {
        return ConvertResult{
            Success: false,
            Error: cm.Some(fmt.Sprintf("Invalid JSON: %v", err)),
        }
    }
    
    // Convert to XML structure
    xmlData, err := c.convertToXML(data, options)
    if err != nil {
        return ConvertResult{
            Success: false,
            Error: cm.Some(fmt.Sprintf("Conversion failed: %v", err)),
        }
    }
    
    processingTime := getCurrentTimestamp() - startTime
    
    return ConvertResult{
        Success: true,
        Data: cm.Some(xmlData),
        ProcessingTime: processingTime,
        Error: cm.None[string](),
    }
}

// ConvertXmlToJson converts XML to JSON with options
func (c Component) ConvertXmlToJson(xmlData string, options ConversionOptions) ConvertResult {
    startTime := getCurrentTimestamp()
    
    // Validate input
    if len(xmlData) == 0 {
        return ConvertResult{
            Success: false,
            Error: cm.Some("Input XML data is empty"),
        }
    }
    
    if len(xmlData) > int(options.MaxInputSize) {
        return ConvertResult{
            Success: false,
            Error: cm.Some(fmt.Sprintf("Input too large: %d bytes (max: %d)", len(xmlData), options.MaxInputSize)),
        }
    }
    
    // Parse XML
    var data interface{}
    if err := xml.Unmarshal([]byte(xmlData), &data); err != nil {
        return ConvertResult{
            Success: false,
            Error: cm.Some(fmt.Sprintf("Invalid XML: %v", err)),
        }
    }
    
    // Convert to JSON
    jsonBytes, err := json.MarshalIndent(data, "", getIndentation(options.PrettyPrint))
    if err != nil {
        return ConvertResult{
            Success: false,
            Error: cm.Some(fmt.Sprintf("JSON encoding failed: %v", err)),
        }
    }
    
    processingTime := getCurrentTimestamp() - startTime
    
    return ConvertResult{
        Success: true,
        Data: cm.Some(string(jsonBytes)),
        ProcessingTime: processingTime,
        Error: cm.None[string](),
    }
}

func (c Component) convertToXML(data interface{}, options ConversionOptions) (string, error) {
    var xmlElements []XMLElement
    
    switch v := data.(type) {
    case map[string]interface{}:
        xmlElements = c.mapToXMLElements(v, options.RootElement)
    case []interface{}:
        xmlElements = c.arrayToXMLElements(v, options.RootElement)
    default:
        return "", fmt.Errorf("unsupported data type: %T", v)
    }
    
    xmlString := c.buildXMLString(xmlElements, options.PrettyPrint)
    return xmlString, nil
}

func (c Component) mapToXMLElements(data map[string]interface{}, rootElement string) []XMLElement {
    var elements []XMLElement
    
    for key, value := range data {
        element := XMLElement{Name: key}
        
        switch v := value.(type) {
        case string:
            element.Content = v
        case float64, int:
            element.Content = fmt.Sprintf("%v", v)
        case bool:
            element.Content = fmt.Sprintf("%t", v)
        case map[string]interface{}:
            element.Children = c.mapToXMLElements(v, "")
        case []interface{}:
            element.Children = c.arrayToXMLElements(v, key)
        case nil:
            element.Content = ""
        default:
            element.Content = fmt.Sprintf("%v", v)
        }
        
        elements = append(elements, element)
    }
    
    if rootElement != "" {
        return []XMLElement{{
            Name:     rootElement,
            Children: elements,
        }}
    }
    
    return elements
}

func (c Component) arrayToXMLElements(data []interface{}, elementName string) []XMLElement {
    var elements []XMLElement
    
    itemName := "item"
    if elementName != "" {
        // Singularize element name for array items
        if strings.HasSuffix(elementName, "s") {
            itemName = elementName[:len(elementName)-1]
        } else {
            itemName = elementName + "Item"
        }
    }
    
    for _, item := range data {
        element := XMLElement{Name: itemName}
        
        switch v := item.(type) {
        case string:
            element.Content = v
        case float64, int:
            element.Content = fmt.Sprintf("%v", v)
        case bool:
            element.Content = fmt.Sprintf("%t", v)
        case map[string]interface{}:
            element.Children = c.mapToXMLElements(v, "")
        case []interface{}:
            element.Children = c.arrayToXMLElements(v, itemName)
        case nil:
            element.Content = ""
        default:
            element.Content = fmt.Sprintf("%v", v)
        }
        
        elements = append(elements, element)
    }
    
    return elements
}

func (c Component) buildXMLString(elements []XMLElement, prettyPrint bool) string {
    var sb strings.Builder
    
    sb.WriteString(`<?xml version="1.0" encoding="UTF-8"?>`)
    if prettyPrint {
        sb.WriteString("\n")
    }
    
    for _, element := range elements {
        c.writeXMLElement(&sb, element, 0, prettyPrint)
    }
    
    return sb.String()
}

func (c Component) writeXMLElement(sb *strings.Builder, element XMLElement, depth int, prettyPrint bool) {
    indent := ""
    if prettyPrint {
        indent = strings.Repeat("  ", depth)
    }
    
    if prettyPrint && depth > 0 {
        sb.WriteString("\n")
    }
    sb.WriteString(indent)
    sb.WriteString("<")
    sb.WriteString(element.Name)
    
    // Add attributes if any
    for key, value := range element.Attributes {
        sb.WriteString(fmt.Sprintf(` %s="%s"`, key, value))
    }
    
    if len(element.Children) == 0 && element.Content == "" {
        sb.WriteString("/>")
        return
    }
    
    sb.WriteString(">")
    
    if element.Content != "" {
        sb.WriteString(element.Content)
    }
    
    for _, child := range element.Children {
        c.writeXMLElement(sb, child, depth+1, prettyPrint)
    }
    
    if len(element.Children) > 0 && prettyPrint {
        sb.WriteString("\n")
        sb.WriteString(indent)
    }
    
    sb.WriteString("</")
    sb.WriteString(element.Name)
    sb.WriteString(">")
}

type XMLElement struct {
    Name       string
    Content    string
    Attributes map[string]string
    Children   []XMLElement
}

func getCurrentTimestamp() uint64 {
    // This would use WASI clock interface in real implementation
    return 0
}

func getIndentation(prettyPrint bool) string {
    if prettyPrint {
        return "  "
    }
    return ""
}

func main() {
    // This is required for TinyGo
}
```

## Configuration Management

### Environment Configuration Loader

Secure configuration management with validation:

```rust
// src/config.rs
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

wit_bindgen::generate!({
    path: "../wit/config.wit",
    world: "config-manager",
});

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub api: ApiConfig,
    pub logging: LoggingConfig,
    pub features: FeatureFlags,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub pool_size: u32,
    pub timeout_seconds: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub timeout_seconds: u32,
    pub retry_attempts: u32,
    pub rate_limit: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeatureFlags {
    pub enable_caching: bool,
    pub enable_metrics: bool,
    pub enable_debug: bool,
}

struct Component;

impl Guest for Component {
    fn load_config(config_path: String, environment: String) -> Result<String, ConfigError> {
        // Validate inputs
        if config_path.is_empty() {
            return Err(ConfigError::InvalidInput("Config path cannot be empty".to_string()));
        }
        
        if environment.is_empty() {
            return Err(ConfigError::InvalidInput("Environment cannot be empty".to_string()));
        }
        
        // Load base configuration
        let mut config = load_base_config(&config_path)?;
        
        // Apply environment-specific overrides
        apply_environment_overrides(&mut config, &environment)?;
        
        // Load secrets from environment variables
        load_secrets(&mut config)?;
        
        // Validate final configuration
        validate_config(&config)?;
        
        // Serialize to JSON
        let config_json = serde_json::to_string_pretty(&config)
            .map_err(|e| ConfigError::SerializationError(e.to_string()))?;
        
        Ok(config_json)
    }
    
    fn validate_config_file(config_path: String) -> Result<ConfigValidation, ConfigError> {
        let start_time = std::time::Instant::now();
        let mut validation = ConfigValidation {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        
        // Load and parse configuration
        match load_base_config(&config_path) {
            Ok(config) => {
                // Validate individual sections
                validate_database_config(&config.database, &mut validation);
                validate_api_config(&config.api, &mut validation);
                validate_logging_config(&config.logging, &mut validation);
                
                // Check for missing required fields
                check_required_fields(&config, &mut validation);
                
                // Check for deprecated settings
                check_deprecated_settings(&config, &mut validation);
            }
            Err(e) => {
                validation.is_valid = false;
                validation.errors.push(format!("Failed to load config: {:?}", e));
            }
        }
        
        Ok(validation)
    }
    
    fn get_environment_variables(prefix: String) -> Result<HashMap<String, String>, ConfigError> {
        let mut env_vars = HashMap::new();
        
        // This would use WASI environment interface in real implementation
        // For now, simulate with some common variables
        let mock_env = [
            ("APP_DATABASE_HOST", "localhost"),
            ("APP_DATABASE_PORT", "5432"),
            ("APP_API_BASE_URL", "https://api.example.com"),
            ("APP_LOG_LEVEL", "info"),
        ];
        
        for (key, value) in mock_env.iter() {
            if key.starts_with(&prefix) {
                env_vars.insert(key.to_string(), value.to_string());
            }
        }
        
        Ok(env_vars)
    }
}

fn load_base_config(config_path: &str) -> Result<AppConfig, ConfigError> {
    // Read configuration file
    let config_content = std::fs::read_to_string(config_path)
        .map_err(|_| ConfigError::FileNotFound(config_path.to_string()))?;
    
    // Parse based on file extension
    if config_path.ends_with(".yaml") || config_path.ends_with(".yml") {
        serde_yaml::from_str(&config_content)
            .map_err(|e| ConfigError::ParseError(format!("YAML parse error: {}", e)))
    } else if config_path.ends_with(".json") {
        serde_json::from_str(&config_content)
            .map_err(|e| ConfigError::ParseError(format!("JSON parse error: {}", e)))
    } else {
        Err(ConfigError::UnsupportedFormat(config_path.to_string()))
    }
}

fn apply_environment_overrides(config: &mut AppConfig, environment: &str) -> Result<(), ConfigError> {
    match environment {
        "development" => {
            config.logging.level = "debug".to_string();
            config.features.enable_debug = true;
            config.database.pool_size = 5;
        }
        "staging" => {
            config.logging.level = "info".to_string();
            config.features.enable_debug = false;
            config.database.pool_size = 10;
        }
        "production" => {
            config.logging.level = "warn".to_string();
            config.features.enable_debug = false;
            config.database.pool_size = 20;
        }
        _ => {
            return Err(ConfigError::InvalidEnvironment(environment.to_string()));
        }
    }
    
    Ok(())
}

fn load_secrets(config: &mut AppConfig) -> Result<(), ConfigError> {
    // Load sensitive configuration from environment variables
    // This prevents secrets from being stored in config files
    
    if let Ok(db_password) = std::env::var("DATABASE_PASSWORD") {
        config.database.password = db_password;
    }
    
    // Validate that required secrets are present
    if config.database.password.is_empty() {
        return Err(ConfigError::MissingSecret("DATABASE_PASSWORD".to_string()));
    }
    
    Ok(())
}

fn validate_config(config: &AppConfig) -> Result<(), ConfigError> {
    // Validate database configuration
    if config.database.host.is_empty() {
        return Err(ConfigError::ValidationError("Database host cannot be empty".to_string()));
    }
    
    if config.database.port == 0 || config.database.port > 65535 {
        return Err(ConfigError::ValidationError("Database port must be between 1 and 65535".to_string()));
    }
    
    // Validate API configuration
    if !config.api.base_url.starts_with("https://") {
        return Err(ConfigError::ValidationError("API base URL must use HTTPS".to_string()));
    }
    
    // Validate logging configuration
    let valid_log_levels = ["error", "warn", "info", "debug", "trace"];
    if !valid_log_levels.contains(&config.logging.level.as_str()) {
        return Err(ConfigError::ValidationError(
            format!("Invalid log level: {}. Must be one of: {:?}", config.logging.level, valid_log_levels)
        ));
    }
    
    Ok(())
}

fn validate_database_config(db_config: &DatabaseConfig, validation: &mut ConfigValidation) {
    if db_config.pool_size > 100 {
        validation.warnings.push("Database pool size is very large (>100)".to_string());
    }
    
    if db_config.timeout_seconds > 300 {
        validation.warnings.push("Database timeout is very long (>5 minutes)".to_string());
    }
}

fn validate_api_config(api_config: &ApiConfig, validation: &mut ConfigValidation) {
    if api_config.retry_attempts > 10 {
        validation.warnings.push("API retry attempts is very high (>10)".to_string());
    }
    
    if api_config.rate_limit > 1000 {
        validation.warnings.push("API rate limit is very high (>1000)".to_string());
    }
}

fn validate_logging_config(log_config: &LoggingConfig, validation: &mut ConfigValidation) {
    if log_config.level == "trace" {
        validation.warnings.push("Trace logging can impact performance".to_string());
    }
}

fn check_required_fields(config: &AppConfig, validation: &mut ConfigValidation) {
    if config.database.host.is_empty() {
        validation.is_valid = false;
        validation.errors.push("Database host is required".to_string());
    }
    
    if config.api.base_url.is_empty() {
        validation.is_valid = false;
        validation.errors.push("API base URL is required".to_string());
    }
}

fn check_deprecated_settings(_config: &AppConfig, validation: &mut ConfigValidation) {
    // Check for deprecated configuration options
    validation.warnings.push("Consider updating to the latest configuration format".to_string());
}

export!(Component);
```

This cookbook provides practical patterns that developers can adapt for their own components. Each pattern includes complete, working code examples with proper error handling, security considerations, and best practices.

## Next Steps

- Explore [File Operations](./file-operations.md) for file handling patterns
- Learn [Network Requests](./network-requests.md) for API integration patterns  
- Review [Data Processing](./data-processing.md) for data transformation examples
- Check [Working with APIs](./apis.md) for API client patterns