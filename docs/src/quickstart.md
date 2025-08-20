# Quickstart Guide

Get up and running with Wassette in minutes! This guide will walk you through installing Wassette, loading your first WebAssembly component, and calling it from an AI agent.

## Prerequisites

- An AI agent that supports the Model Context Protocol (MCP) such as:
  - [GitHub Copilot in VS Code](./mcp-clients.md#github-copilot)
  - [Claude Desktop](./mcp-clients.md#claude-desktop)
  - [Cursor](./mcp-clients.md#cursor)

## Step 1: Install Wassette

### Using Homebrew (macOS/Linux)

```bash
brew install microsoft/wassette/wassette
```

### Using Cargo (All platforms)

```bash
cargo install wassette
```

### Download Binary

Download the latest release from [GitHub Releases](https://github.com/microsoft/wassette/releases).

## Step 2: Verify Installation

```bash
wassette --version
```

## Step 3: Configure Your AI Agent

Add Wassette as an MCP server to your AI agent. The exact configuration varies by agent:

### GitHub Copilot in VS Code

1. Install the [GitHub Copilot extension](https://marketplace.visualstudio.com/items?itemName=GitHub.copilot)
2. Open VS Code settings (Cmd/Ctrl + ,)
3. Search for "copilot mcp"
4. Add Wassette server configuration:

```json
{
  "name": "wassette",
  "command": "wassette",
  "args": ["serve", "--stdio"]
}
```

### Claude Desktop

Add to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "wassette": {
      "command": "wassette",
      "args": ["serve", "--stdio"]
    }
  }
}
```

For detailed setup instructions for your specific agent, see [MCP Clients](./mcp-clients.md).

## Step 4: Load Your First Component

Once Wassette is configured with your AI agent, you can load WebAssembly components by asking your agent to do it:

```
Please load the time component from oci://ghcr.io/microsoft/time-server-js:latest
```

This loads a simple time server component that provides time-related functions.

## Step 5: Use the Component

Now you can call the loaded component's functions:

```
What's the current time in UTC?
```

The AI agent will use the loaded time component to provide you with the current time.

## Step 6: Explore More Components

Try loading and using other example components:

```
Load the weather component from oci://ghcr.io/microsoft/get-weather-js:latest
```

```
What's the weather like in Seattle?
```

## Next Steps

- **Learn about [Core Concepts](./concepts/core-concepts.md)** - Understand WebAssembly components and MCP
- **Explore [Security](./security/security-model.md)** - Learn about Wassette's security model
- **Build Components** - Create your own components in [JavaScript](./development/javascript.md), [Rust](./development/rust.md), [Python](./development/python.md), or [Go](./development/go.md)
- **Check Examples** - Browse the [Cookbook](./cookbook/common-patterns.md) for practical examples

## Troubleshooting

### Component Won't Load

If you encounter issues loading a component:

1. Check that the URI is correct
2. Verify network connectivity if loading from a registry
3. Check component permissions with `wassette policy get <component-id>`

### Agent Can't Connect

If your AI agent can't connect to Wassette:

1. Verify Wassette is installed: `wassette --version`
2. Test the server manually: `wassette serve --stdio`
3. Check your agent's MCP configuration
4. Review the [MCP Clients](./mcp-clients.md) setup guide

For more troubleshooting help, see the [FAQ](./reference/faq.md) or [open an issue](https://github.com/microsoft/wassette/issues).