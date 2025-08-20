# Getting Started with Development

This guide will help you start building WebAssembly components for Wassette. Whether you're new to WebAssembly or experienced with other platforms, this guide provides the foundation you need to create secure, efficient tools for AI agents.

## Overview

Building components for Wassette involves:

1. **Choosing a Language** - Pick from JavaScript, Rust, Python, or Go
2. **Defining Interfaces** - Create WIT files that describe your component's API
3. **Implementing Logic** - Write the actual component functionality
4. **Building Components** - Compile to WebAssembly Components
5. **Testing** - Verify your component works correctly
6. **Deploying** - Distribute your component via registries

## Prerequisites

### Required Tools

- **Git** - For version control and examples
- **Text Editor** - VS Code, vim, or your preferred editor
- **WebAssembly Tools** - Language-specific tooling (see language guides)

### Recommended Knowledge

- Basic understanding of your chosen programming language
- Familiarity with command-line tools
- Understanding of AI agents and the Model Context Protocol (helpful but not required)

## Quick Start

### 1. Choose Your Language

Pick the language you're most comfortable with:

| Language | Maturity | Performance | Learning Curve | Best For |
|----------|----------|-------------|----------------|----------|
| **JavaScript** | High | Good | Low | Web developers, rapid prototyping |
| **Rust** | High | Excellent | Moderate | Systems programming, performance |
| **Python** | Medium | Good | Low | Data science, scripting |
| **Go** | Medium | Good | Low | Microservices, cloud tools |

### 2. Set Up Development Environment

Follow the setup guide for your chosen language:

- [JavaScript/TypeScript Setup](./javascript.md#prerequisites)
- [Rust Setup](./rust.md#prerequisites)
- [Python Setup](./python.md#prerequisites)
- [Go Setup](./go.md#prerequisites)

### 3. Create Your First Component

Let's create a simple "Hello World" component:

#### Define the Interface (WIT)

Create `wit/world.wit`:

```wit
package hello:world@1.0.0;

interface greeter {
  /// Say hello to someone
  greet: func(name: string) -> string;
}

world hello-world {
  export greeter;
}
```

#### Implement in Your Language

=== JavaScript
```javascript
// main.js
export function greet(name) {
  return `Hello, ${name}! Welcome to Wassette.`;
}
```

=== Rust
```rust
// src/lib.rs
wit_bindgen::generate!({
    path: "../wit/world.wit",
    world: "hello-world",
});

struct Component;

impl Guest for Component {
    fn greet(name: String) -> String {
        format!("Hello, {}! Welcome to Wassette.", name)
    }
}

export!(Component);
```

=== Python
```python
# main.py
def greet(name: str) -> str:
    return f"Hello, {name}! Welcome to Wassette."
```

#### Build the Component

Follow your language-specific build instructions to compile the component to WebAssembly.

### 4. Test Your Component

```bash
# Load the component
wassette component load ./hello-world.wasm

# Test it via CLI
wassette component call hello-world greet --args '{"name": "Developer"}'

# Or ask an AI agent:
# "Please load the component from ./hello-world.wasm and greet me"
```

## Component Architecture

### Interface-First Design

Start by designing your component's interface:

```wit
package my-tool:api@1.0.0;

interface file-processor {
  record processing-options {
    format: string,
    compression: bool,
    max-size: u64,
  }
  
  variant processing-result {
    success(string),
    error(string),
  }
  
  process-file: func(
    path: string, 
    options: processing-options
  ) -> processing-result;
}
```

### Implementation Patterns

**Input Validation**
```javascript
export function processFile(path, options) {
  // Validate inputs
  if (!path || typeof path !== 'string') {
    return { tag: 'error', val: 'Invalid path provided' };
  }
  
  if (!options.format) {
    return { tag: 'error', val: 'Format is required' };
  }
  
  // Process file...
  return { tag: 'success', val: 'File processed successfully' };
}
```

**Error Handling**
```rust
fn process_file(path: String, options: ProcessingOptions) -> ProcessingResult {
    match std::fs::read(&path) {
        Ok(data) => {
            // Process data
            ProcessingResult::Success("File processed".to_string())
        }
        Err(e) => ProcessingResult::Error(format!("Failed to read file: {}", e))
    }
}
```

**Resource Management**
```python
def process_file(path: str, options: ProcessingOptions) -> ProcessingResult:
    try:
        with open(path, 'r') as file:
            data = file.read(options.max_size)
            # Process data
            return ProcessingResult(tag='success', value='File processed')
    except Exception as e:
        return ProcessingResult(tag='error', value=str(e))
```

## Development Workflow

### 1. Design Phase

- **Define Use Cases**: What problems will your component solve?
- **Design Interface**: Create WIT definitions for your API
- **Plan Security**: What permissions will you need?
- **Consider Performance**: What are the resource requirements?

### 2. Implementation Phase

- **Set Up Project**: Create directory structure and build files
- **Implement Core Logic**: Write the main functionality
- **Add Error Handling**: Handle all error conditions gracefully
- **Write Tests**: Create unit tests for your logic

### 3. Integration Phase

- **Build Component**: Compile to WebAssembly
- **Test with Wassette**: Load and test with the runtime
- **Validate Security**: Ensure minimal permissions work
- **Performance Testing**: Check resource usage

### 4. Distribution Phase

- **Create Policy**: Define security permissions
- **Package Component**: Bundle component with policy
- **Publish to Registry**: Make available to others
- **Document Usage**: Provide clear documentation

## Project Structure

### Recommended Layout

```
my-component/
├── wit/                    # Interface definitions
│   └── world.wit
├── src/                    # Source code
│   ├── main.{js,rs,py,go}
│   └── lib.{js,rs,py,go}
├── policy.yaml            # Security policy
├── Cargo.toml             # Rust dependencies
├── package.json           # JS dependencies  
├── pyproject.toml         # Python dependencies
├── go.mod                 # Go dependencies
├── Justfile              # Build automation
├── tests/                 # Test files
├── docs/                  # Documentation
└── README.md             # Project documentation
```

### Build Automation

Use a `Justfile` for consistent builds:

```make
# Default build
build:
    @echo "Building component..."
    cargo component build --release

# Run tests
test:
    @echo "Running tests..."
    cargo test
    
# Load component for testing
load: build
    wassette component load ./target/wasm32-wasip2/release/my_component.wasm

# Clean build artifacts
clean:
    cargo clean
    rm -rf target/
```

## Security Considerations

### Principle of Least Privilege

Only request permissions you actually need:

```yaml
# policy.yaml
version: "1.0"
description: "Minimal permissions for file processor"
permissions:
  storage:
    allow:
      - uri: "fs://input/**"
        access: ["read"]
      - uri: "fs://output/**"  
        access: ["write"]
  # No network permissions needed
```

### Input Validation

Always validate inputs from the host:

```javascript
function validateInput(input) {
  if (typeof input !== 'string') {
    throw new Error('Input must be a string');
  }
  
  if (input.length > MAX_INPUT_SIZE) {
    throw new Error('Input too large');
  }
  
  // Sanitize input
  return input.trim();
}
```

### Error Messages

Don't leak sensitive information:

```rust
// Bad: exposes internal paths
Err(format!("Failed to read /etc/secrets/key.txt: {}", e))

// Good: generic error message
Err("Failed to read configuration file".to_string())
```

## Testing Strategies

### Unit Testing

Test your core logic independently:

```javascript
// test/greet.test.js
import { greet } from '../src/main.js';

describe('greet function', () => {
  test('should greet with name', () => {
    const result = greet('Alice');
    expect(result).toBe('Hello, Alice! Welcome to Wassette.');
  });
  
  test('should handle empty name', () => {
    const result = greet('');
    expect(result).toBe('Hello, ! Welcome to Wassette.');
  });
});
```

### Integration Testing

Test with Wassette runtime:

```bash
#!/bin/bash
# test/integration.sh

# Build component
just build

# Load component
wassette component load ./target/wasm32-wasip2/release/my_component.wasm

# Test function calls
result=$(wassette component call my-component greet --args '{"name": "Test"}')
echo "Result: $result"

# Verify output
if [[ $result == *"Hello, Test"* ]]; then
    echo "✓ Integration test passed"
else
    echo "✗ Integration test failed"
    exit 1
fi
```

### Performance Testing

Monitor resource usage:

```bash
# Monitor component execution
wassette component call my-component process-large-file \
  --args '{"path": "large-file.txt"}' \
  --monitor
```

## Debugging

### Common Issues

**Build Failures**
- Check language-specific toolchain versions
- Verify WIT syntax with `wit-deps check`
- Review dependency versions

**Runtime Errors**
- Check Wassette logs: `RUST_LOG=debug wassette serve`
- Verify component permissions
- Test with minimal input first

**Performance Issues**
- Profile memory usage
- Check for resource leaks
- Optimize algorithms and data structures

### Debugging Tools

```bash
# Check component metadata
wassette component inspect my-component.wasm

# View component logs
wassette logs --component my-component --follow

# Analyze resource usage
wassette metrics --component my-component
```

## Advanced Topics

### Multi-Language Components

Components can be built from multiple languages:

```wit
world polyglot-tool {
  import rust-crypto: crypto;        // Rust implementation
  import python-ml: machine-learning; // Python implementation
  export combined-tool;              // Combined interface
}
```

### Streaming Data

Handle large datasets efficiently:

```wit
interface data-processor {
  resource stream-handle {
    read-chunk: func() -> option<list<u8>>;
    write-chunk: func(data: list<u8>);
  }
  
  create-stream: func(path: string) -> stream-handle;
}
```

### Component Composition

Build complex tools from simple components:

```wit
world file-analyzer {
  import text-processor: text;
  import image-processor: image;
  import audio-processor: audio;
  
  export unified-analyzer;
}
```

## Next Steps

Choose your language and dive deeper:

- **JavaScript Developers**: [JavaScript/TypeScript Guide](./javascript.md)
- **Rust Developers**: [Rust Guide](./rust.md)
- **Python Developers**: [Python Guide](./python.md)
- **Go Developers**: [Go Guide](./go.md)

Or explore other topics:

- [Testing Components](./testing.md) - Comprehensive testing strategies
- [Best Practices](./best-practices.md) - Production-ready development
- [Cookbook](../cookbook/common-patterns.md) - Practical examples
- [Security Model](../security/security-model.md) - Understanding security

## Community

- **GitHub**: [microsoft/wassette](https://github.com/microsoft/wassette)
- **Discord**: [Microsoft Open Source](https://discord.gg/microsoft-open-source)
- **Examples**: [Component Examples](https://github.com/microsoft/wassette/tree/main/examples)
- **Contributing**: [Contributing Guide](../contributing.md)