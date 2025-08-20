# Quickstart Guide

Get up and running with Wassette in under 5 minutes. This guide will walk you through installing Wassette, loading a sample component, and calling a tool from an MCP client.

## Prerequisites

- A system running Linux, macOS, or Windows
- An MCP client (VS Code with Copilot, Cursor, Claude Code, or Gemini CLI)

## Step 1: Install Wassette

### Quick Install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/microsoft/wassette/main/install.sh | bash
```

### Alternative Installation Methods

- **macOS/Linux**: [Homebrew](./homebrew.md)
- **Windows**: [WinGet](./winget.md)
- **Nix users**: [Nix Flakes](./nix.md)
- **Manual**: Download from [GitHub Releases](https://github.com/microsoft/wassette/releases)

### Verify Installation

```bash
wassette --help
```

You should see the Wassette command-line interface help output.

## Step 2: Start Wassette as an MCP Server

Start Wassette in MCP server mode:

```bash
wassette serve --stdio
```

This starts Wassette as an MCP server that communicates over stdin/stdout, which is the standard way MCP clients connect to servers.

## Step 3: Connect from an MCP Client

Choose your preferred MCP client and follow the setup:

### VS Code (GitHub Copilot)

Click to install Wassette as an MCP server in VS Code:

[![Install in VS Code](https://img.shields.io/badge/VS_Code-Install_Server-0098FF?style=flat-square&logo=visualstudiocode&logoColor=white)](https://vscode.dev/redirect?url=vscode:mcp/install?%7B%22name%22%3A%22wassette%22%2C%22gallery%22%3Afalse%2C%22command%22%3A%22wassette%22%2C%22args%22%3A%5B%22serve%22%2C%22--stdio%22%5D%7D)

Or manually add this configuration to your MCP settings:

```json
{
  "name": "wassette",
  "command": "wassette",
  "args": ["serve", "--stdio"]
}
```

### Cursor

Use the one-click installation:

[![Install MCP Server](https://cursor.com/deeplink/mcp-install-light.svg)](https://cursor.com/install-mcp?name=wassette&config=JTdCJTIyY29tbWFuZCUyMiUzQSUyMndhc3NldHRlJTIwc2VydmUlMjAtLXN0ZGlvJTIyJTdE)

### Claude Code

First install Claude Code (requires Node.js 18+):

```bash
npm install -g @anthropic-ai/claude-code
```

Then add Wassette:

```bash
claude mcp add -- wassette wassette serve --stdio
```

### Gemini CLI

Add Wassette to your MCP configuration file (typically `~/.config/gemini/mcp_servers.json`):

```json
{
  "servers": {
    "wassette": {
      "command": "wassette",
      "args": ["serve", "--stdio"]
    }
  }
}
```

## Step 4: Load Your First Component

With Wassette running as an MCP server, it provides built-in tools for managing components. Let's load a sample component.

### Option A: Load from Examples (if you have the repo)

If you have the Wassette repository cloned:

```bash
# Load the filesystem component example
wassette component load file:///path/to/wassette/examples/filesystem-rs/target/wasm32-wasip2/release/filesystem.wasm
```

### Option B: Load from OCI Registry

Load a component from a container registry:

```bash
# Load a time component (example)
wassette component load oci://ghcr.io/yoshuawuyts/time:latest
```

### Option C: Use Built-in Tools Through MCP Client

Since Wassette provides component management as MCP tools, you can also load components directly from your MCP client by asking it to use the `load-component` tool.

## Step 5: Call Your First Tool

Once a component is loaded, its tools automatically become available to your MCP client. The exact steps depend on your client:

### In VS Code/Cursor
1. Open a chat window
2. Type something like: "List the files in my current directory" (if you loaded the filesystem component)
3. The assistant will discover and use the appropriate tool automatically

### In Claude Code
```bash
claude "What tools are available?" # This will show loaded tools
claude "List files in the current directory" # Use a filesystem tool
```

### In Gemini CLI
The loaded tools will be available for the assistant to use automatically when you ask relevant questions.

## Step 6: Explore More

Congratulations! You've successfully:
- ✅ Installed Wassette
- ✅ Started it as an MCP server
- ✅ Connected an MCP client
- ✅ Loaded a WebAssembly component
- ✅ Called a tool through the MCP client

### Next Steps

- **Learn the fundamentals**: Read about [MCP concepts](./concepts/mcp-fundamentals.md) and [WebAssembly components](./concepts/webassembly-components.md)
- **Explore security**: Understand [sandboxing](./security/sandboxing-overview.md) and [policy configuration](./security/policy-schema.md)
- **Build your own tools**: Follow our [development guides](./developing/getting-started.md) for your preferred language
- **Browse examples**: Check out the [cookbook](./cookbook/common-patterns.md) for practical patterns

## Troubleshooting

### Component Won't Load
- Ensure the file path or OCI URL is correct
- Check that the component is compiled for `wasm32-wasip2` target
- Verify the component has a valid WIT interface

### MCP Client Can't Connect
- Make sure `wassette serve --stdio` is running
- Check that your MCP client configuration matches exactly
- Try restarting both Wassette and your MCP client

### Tools Don't Appear
- Use `wassette component list` to verify components are loaded
- Check that components export functions with proper MCP tool signatures
- Review the [troubleshooting guide](./troubleshooting.md) for common issues

## What's Happening Under the Hood?

When you run `wassette serve --stdio`, you're starting an MCP server that:

1. **Listens for MCP requests** from clients over stdin/stdout
2. **Manages WebAssembly components** loaded from files or registries
3. **Applies security policies** to each component's runtime environment
4. **Translates MCP tool calls** to WebAssembly function calls
5. **Returns results** back to the MCP client in standard format

The beauty is that **your MCP client doesn't know the difference** - tools appear and behave exactly like they would with any other MCP server, but they're running in a secure sandbox with explicit permissions.