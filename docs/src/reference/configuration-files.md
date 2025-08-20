# Configuration Files

Wassette uses configuration files to manage component policies and server settings.

## Policy Files

Policy files define permissions for WebAssembly components using YAML format:

```yaml
apiVersion: v1
kind: Policy
metadata:
  name: example-component
spec:
  permissions:
    filesystem:
      - path: "/tmp"
        access: ["read", "write"]
    network:
      - domain: "api.example.com"
        access: ["http", "https"]
```

**Location**: Policy files are typically stored alongside components or in a dedicated policies directory.

**Reference**: See [Policy & Capabilities](../concepts/policy-capabilities.md) for detailed syntax and examples.

## Environment Configuration

Environment variables can be configured through:

### Command Line
```bash
wassette serve --env KEY=value --env ANOTHER_KEY=value
```

### Environment File
```bash
wassette serve --env-file .env
```

Environment file format (`.env`):
```
DATABASE_URL=sqlite:///tmp/app.db
API_KEY=your-api-key-here
LOG_LEVEL=info
```

## Component Directory

Components are loaded from a plugin directory:

**Default Location**: `$XDG_DATA_HOME/wassette/components`

**Override**: Use `--plugin-dir` flag or set via configuration

```bash
wassette serve --plugin-dir /path/to/components
```

**Structure**:
```
components/
├── my-tool.wasm
├── another-tool.wasm
└── policies/
    ├── my-tool.yaml
    └── another-tool.yaml
```
