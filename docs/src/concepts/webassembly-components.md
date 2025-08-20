# WebAssembly Components

WebAssembly Components are the foundation of how Wassette executes tools securely. This page explains what they are, how they differ from traditional WebAssembly modules, and why they're perfect for MCP tools.

## What are WebAssembly Components?

WebAssembly Components are a higher-level abstraction built on top of WebAssembly (Wasm) that provides:

- **Standardized interfaces** using WebAssembly Interface Types (WIT)
- **Language interoperability** - components can be written in any language that compiles to Wasm
- **Composability** - components can be combined and linked together
- **Type safety** - interfaces are strongly typed and validated

## Components vs. Modules

| Feature | WebAssembly Module | WebAssembly Component |
|---------|-------------------|----------------------|
| **Interface** | Low-level exports/imports | High-level WIT interfaces |
| **Types** | Basic Wasm types (i32, f64, etc.) | Rich types (strings, records, variants) |
| **Interop** | Manual binding code | Automatic binding generation |
| **Composability** | Limited | Native composition support |
| **Tooling** | Assembly-level tools | High-level development tools |

## The WebAssembly Component Model

The Component Model defines how WebAssembly components:

1. **Export interfaces** that other components can use
2. **Import interfaces** that they depend on
3. **Compose together** to build larger applications
4. **Maintain type safety** across component boundaries

### Example Component Interface (WIT)

```wit
package component:example;

interface filesystem {
  record file-info {
    name: string,
    size: u64,
    is-directory: bool,
  }

  read-file: func(path: string) -> result<string, string>;
  list-directory: func(path: string) -> result<list<file-info>, string>;
}

world example-tool {
  export filesystem;
}
```

This WIT interface defines:
- A `file-info` record type with structured data
- Two functions that return rich result types
- A `world` that exports the filesystem interface

## Why Components for MCP Tools?

### 1. Security Isolation

Components run in a memory-safe sandbox:
```
┌─────────────────────────────────────┐
│            Host System              │
│  ┌─────────────────────────────────┐ │
│  │      Wassette MCP Server        │ │
│  │  ┌─────────────────────────────┐ │ │
│  │  │    WebAssembly Runtime      │ │ │
│  │  │  ┌─────────┐  ┌─────────┐   │ │ │
│  │  │  │Component│  │Component│   │ │ │
│  │  │  │    A    │  │    B    │   │ │ │
│  │  │  └─────────┘  └─────────┘   │ │ │
│  │  └─────────────────────────────┘ │ │
│  └─────────────────────────────────┘ │
└─────────────────────────────────────┘
```

### 2. Type-Safe Tool Definitions

Tools are defined with precise type signatures:
```wit
// Instead of loosely-typed JSON
read-file: func(path: string) -> result<string, string>;

// Rich structured types
record api-response {
  status: u32,
  headers: list<tuple<string, string>>,
  body: option<string>,
}
```

### 3. Language Independence

Write components in any language:
- **JavaScript/TypeScript** - Using `jco` and Node.js tooling
- **Python** - Using `componentize-py`
- **Rust** - Using `wit-bindgen` and `cargo component`
- **Go** - Using TinyGo and `wit-bindgen`
- **C/C++** - Using Clang and WASI SDK

### 4. Standard Distribution

Components package as OCI artifacts:
```bash
# Push to registry
wasm-tools component embed wit/world.wit my-tool.wasm -o component.wasm
oci-distribute push ghcr.io/myorg/my-tool:v1.0.0 component.wasm

# Pull and use
wassette component load oci://ghcr.io/myorg/my-tool:v1.0.0
```

## How Wassette Uses Components

### Component Lifecycle

1. **Load**: Component is loaded from file or OCI registry
2. **Validate**: WIT interface is parsed and validated
3. **Register**: Functions are mapped to MCP tools
4. **Execute**: Runtime creates isolated instance per call
5. **Clean up**: Instance is destroyed after execution

### WIT to MCP Tool Mapping

Wassette automatically maps WIT functions to MCP tools:

```wit
// WIT function definition
read-file: func(path: string) -> result<string, string>;
```

Becomes an MCP tool:
```json
{
  "name": "read_file",
  "description": "Read the contents of a file",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": {"type": "string"}
    },
    "required": ["path"]
  }
}
```

### Error Handling

WIT result types map naturally to MCP responses:
```wit
// WIT result type
result<string, string>  // Success: string, Error: string
```

```json
// Success response
{
  "content": [{"type": "text", "text": "file contents"}]
}

// Error response
{
  "isError": true,
  "content": [{"type": "text", "text": "file not found"}]
}
```

## Component Development Workflow

### 1. Define Interface (WIT)

Create a `wit/world.wit` file:
```wit
package my-org:my-tool;

world my-tool {
  export process-data: func(input: string) -> result<string, string>;
}
```

### 2. Implement Logic

Write your tool in your preferred language:

**Rust:**
```rust
use wit_bindgen::generate;

generate!({
    world: "my-tool",
});

export!(MyTool);

struct MyTool;

impl Guest for MyTool {
    fn process_data(input: String) -> Result<String, String> {
        // Your logic here
        Ok(format!("Processed: {}", input))
    }
}
```

**JavaScript:**
```javascript
export function processData(input) {
  // Your logic here
  return `Processed: ${input}`;
}
```

### 3. Build Component

**Rust:**
```bash
cargo component build --release
```

**JavaScript:**
```bash
jco componentize app.js --wit wit/world.wit --out component.wasm
```

### 4. Test with Wassette

```bash
# Load the component
wassette component load file://component.wasm

# Test via MCP client or CLI
wassette tools call process_data '{"input": "test data"}'
```

## Advanced Component Features

### Resource Management

Components can declare resource requirements:
```wit
interface resource-manager {
  resource file-handle;
  
  open-file: func(path: string) -> result<file-handle, string>;
  read-data: func(handle: borrow<file-handle>) -> result<string, string>;
  close-file: func(handle: file-handle);
}
```

### Streaming Interfaces

For handling large data:
```wit
interface streaming {
  resource stream;
  
  create-stream: func() -> stream;
  write-chunk: func(s: borrow<stream>, data: list<u8>);
  finish-stream: func(s: stream) -> result<string, string>;
}
```

### Component Linking

Combine multiple components:
```bash
# Compose components
wasm-tools compose component-a.wasm component-b.wasm -o composed.wasm

# Load composed component
wassette component load file://composed.wasm
```

## Performance Characteristics

### Memory Usage
- **Isolated heaps**: Each component instance has its own memory
- **Copy-free transfers**: Efficient data passing between host and component
- **Garbage collection**: Automatic cleanup after execution

### Execution Speed
- **Near-native performance**: WebAssembly compiles to efficient machine code
- **Fast instantiation**: Component instances start quickly
- **Minimal overhead**: Direct function calls with type checking

### Scalability
- **Parallel execution**: Multiple components can run simultaneously
- **Resource pooling**: Reuse component instances when possible
- **Horizontal scaling**: Components can be distributed across multiple Wassette instances

## Next Steps

- Learn about [Policy & Capabilities](./policy-capabilities.md) to control component permissions
- Explore [Component Development](../developing/getting-started.md) guides for your language
- Check out the [Component Lifecycle](../architecture/component-lifecycle.md) architecture details