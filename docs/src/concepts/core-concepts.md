# Core Concepts

Understanding these core concepts will help you work effectively with Wassette and build secure WebAssembly components for AI agents.

## Wassette

**Wassette** is a security-oriented runtime that executes WebAssembly Components via the Model Context Protocol (MCP). It acts as a bridge between AI agents and WebAssembly tools, providing a secure sandboxed environment for code execution.

Key characteristics:
- **Security-first**: Every component runs in a WebAssembly sandbox with explicit permissions
- **Language-agnostic**: Supports components written in JavaScript, Rust, Python, Go, and more
- **MCP-native**: Built specifically for the Model Context Protocol
- **Developer-friendly**: Focus on business logic, not infrastructure complexity

> The name "Wassette" is a portmanteau of "Wasm" and "Cassette" (referring to magnetic tape storage), and is pronounced "Wass-ette".

## Model Context Protocol (MCP)

The **Model Context Protocol** is a standardized way for AI language models to access and interact with external tools and data sources. MCP defines:

- **Tools**: Functions that can be called by AI agents
- **Resources**: Data sources that can be read or updated
- **Prompts**: Templates for generating responses

Wassette implements the MCP server specification, allowing AI agents to securely call WebAssembly component functions as tools.

### MCP vs Traditional APIs

| Traditional APIs | MCP Tools |
|------------------|-----------|
| Always-on servers | On-demand execution |
| Network-based | Local or remote |
| Language-specific | WebAssembly universal |
| Complex deployment | Simple component loading |

## WebAssembly Components

**WebAssembly Components** are a higher-level abstraction built on top of WebAssembly modules. They provide:

### Key Features

- **Interface Definition**: Components define clear interfaces using WIT (WebAssembly Interface Types)
- **Composition**: Components can import functionality from other components
- **Security**: Components run in a sandboxed environment with explicit permissions
- **Portability**: Components run across different hosts and platforms

### Component vs Module

| WebAssembly Module | WebAssembly Component |
|-------------------|----------------------|
| Low-level bytecode | High-level abstraction |
| Manual memory management | Managed interfaces |
| Pointer-based APIs | Typed interfaces |
| Limited composition | Rich composition model |

## WebAssembly Interface Types (WIT)

**WIT** is an Interface Definition Language (IDL) for defining component interfaces. It allows you to:

- Define function signatures with rich types
- Specify imports and exports
- Create composable interfaces
- Generate language bindings automatically

Example WIT definition:

```wit
package my-component:tools;

interface time {
  get-current-time: func() -> string;
  format-timestamp: func(timestamp: u64, format: string) -> string;
}

world my-tool {
  export time;
}
```

## Security Model

Wassette implements a **capability-based security model**:

### Sandboxing

- **WebAssembly Sandbox**: All components run in WebAssembly's memory-safe environment
- **WASI (WebAssembly System Interface)**: Controlled access to system resources
- **No Direct System Access**: Components cannot directly access files, network, or processes

### Permissions

- **Explicit Permissions**: Components must declare what resources they need
- **Policy Files**: YAML files define allowed resources and operations
- **Runtime Granting**: Permissions can be granted dynamically
- **Principle of Least Privilege**: Components only get permissions they need

Example policy:

```yaml
version: "1.0"
permissions:
  storage:
    allow:
      - uri: "fs://workspace/**"
        access: ["read", "write"]
  network:
    allow:
      - host: "api.openai.com"
```

## Component Lifecycle

Understanding how components work in Wassette:

### 1. Loading

Components are loaded from various sources:
- **OCI Registries**: `oci://ghcr.io/owner/component:tag`
- **Local Files**: `file:///path/to/component.wasm`
- **HTTP URLs**: `https://example.com/component.wasm`

### 2. Registration

When loaded, Wassette:
- Extracts the WIT interface
- Generates JSON Schema for MCP
- Maps component functions to MCP tools
- Applies default security policies

### 3. Execution

When an AI agent calls a tool:
- Wassette creates a new WebAssembly instance
- Applies security policies and permissions
- Calls the component function with provided arguments
- Returns results to the AI agent

### 4. Management

Components can be:
- **Listed**: See all loaded components
- **Unloaded**: Remove components from memory
- **Updated**: Reload with new versions
- **Configured**: Update permissions and policies

## Resource Types

Wassette components can access three types of resources:

### Storage

File system access with path-based permissions:
- **Read/Write**: Different access levels
- **Path Patterns**: Glob-style path matching
- **URI Scheme**: `fs://` prefix for file paths

### Network

HTTP/HTTPS access with host-based permissions:
- **Host Allowlists**: Specific domains and IPs
- **Protocol Support**: HTTP and HTTPS
- **Port Restrictions**: Control allowed ports

### Environment

Environment variable access:
- **Variable Allowlists**: Specific environment variables
- **Configuration Store**: Secure storage for API keys
- **Runtime Values**: Access to runtime configuration

## Tool Discovery

When components are loaded, Wassette automatically:

1. **Extracts Interface**: Reads the WIT interface from the component
2. **Generates Schema**: Creates JSON Schema for each exported function
3. **Registers Tools**: Makes functions available as MCP tools
4. **Provides Metadata**: Includes descriptions and parameter information

This means AI agents can automatically discover and use component functions without manual configuration.

## Next Steps

- Learn about [WebAssembly Components](./wasm-components.md) in detail
- Understand the [Security Model](../security/security-model.md)
- Start [developing components](../development/getting-started.md)
- Explore [practical examples](../cookbook/common-patterns.md)