# Testing Components

Comprehensive testing is essential for building reliable WebAssembly components. This guide covers testing strategies, tools, and best practices for ensuring your components work correctly in production.

## Testing Pyramid

Wassette component testing follows a layered approach:

```
    ┌─────────────────────┐
    │   End-to-End Tests  │  ← AI Agent Integration
    │                     │
    ├─────────────────────┤
    │  Integration Tests  │  ← Wassette Runtime
    │                     │
    ├─────────────────────┤
    │    Unit Tests       │  ← Component Logic
    │                     │
    └─────────────────────┘
```

### Test Types

1. **Unit Tests**: Test individual functions and modules
2. **Integration Tests**: Test component interaction with Wassette
3. **End-to-End Tests**: Test full AI agent workflows
4. **Security Tests**: Verify permission enforcement
5. **Performance Tests**: Validate resource usage

## Unit Testing

### JavaScript/TypeScript

Use Jest or your preferred testing framework:

```javascript
// test/calculator.test.js
import { add, subtract, divide } from '../src/calculator.js';

describe('Calculator Component', () => {
  describe('add', () => {
    test('should add two positive numbers', () => {
      expect(add(2, 3)).toBe(5);
    });
    
    test('should handle negative numbers', () => {
      expect(add(-2, 3)).toBe(1);
    });
    
    test('should handle zero', () => {
      expect(add(0, 5)).toBe(5);
    });
  });
  
  describe('divide', () => {
    test('should divide numbers correctly', () => {
      expect(divide(10, 2)).toBe(5);
    });
    
    test('should handle division by zero', () => {
      expect(() => divide(10, 0)).toThrow('Division by zero');
    });
    
    test('should handle floating point precision', () => {
      expect(divide(1, 3)).toBeCloseTo(0.333, 2);
    });
  });
});
```

**Setup (package.json):**
```json
{
  "scripts": {
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage"
  },
  "devDependencies": {
    "jest": "^29.0.0"
  }
}
```

### Rust

Use the built-in test framework:

```rust
// src/calculator.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-2, 3), 1);
    }

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_divide_floating_point() {
        let result = divide(1.0, 3.0).unwrap();
        assert!((result - 0.333).abs() < 0.001);
    }
}
```

### Python

Use pytest for comprehensive testing:

```python
# test_calculator.py
import pytest
from src.calculator import add, subtract, divide

class TestCalculator:
    def test_add_positive_numbers(self):
        assert add(2, 3) == 5
    
    def test_add_negative_numbers(self):
        assert add(-2, 3) == 1
    
    def test_add_zero(self):
        assert add(0, 5) == 5
    
    def test_divide_success(self):
        assert divide(10, 2) == 5
    
    def test_divide_by_zero(self):
        with pytest.raises(ValueError, match="Division by zero"):
            divide(10, 0)
    
    def test_divide_floating_point(self):
        result = divide(1, 3)
        assert abs(result - 0.333) < 0.001

# Fixtures for complex test data
@pytest.fixture
def sample_data():
    return {
        'numbers': [1, 2, 3, 4, 5],
        'expected_sum': 15
    }

def test_with_fixture(sample_data):
    total = sum(sample_data['numbers'])
    assert total == sample_data['expected_sum']
```

### Go

Use the standard testing package:

```go
// calculator_test.go
package main

import (
    "testing"
    "math"
)

func TestAdd(t *testing.T) {
    tests := []struct {
        name     string
        a, b     int
        expected int
    }{
        {"positive numbers", 2, 3, 5},
        {"negative numbers", -2, 3, 1},
        {"with zero", 0, 5, 5},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            result := Add(tt.a, tt.b)
            if result != tt.expected {
                t.Errorf("Add(%d, %d) = %d; want %d", tt.a, tt.b, result, tt.expected)
            }
        })
    }
}

func TestDivide(t *testing.T) {
    t.Run("successful division", func(t *testing.T) {
        result, err := Divide(10, 2)
        if err != nil {
            t.Fatalf("unexpected error: %v", err)
        }
        if result != 5 {
            t.Errorf("Divide(10, 2) = %f; want 5", result)
        }
    })

    t.Run("division by zero", func(t *testing.T) {
        _, err := Divide(10, 0)
        if err == nil {
            t.Fatal("expected error for division by zero")
        }
    })

    t.Run("floating point precision", func(t *testing.T) {
        result, _ := Divide(1, 3)
        expected := 1.0 / 3.0
        if math.Abs(result-expected) > 1e-9 {
            t.Errorf("Divide(1, 3) = %f; want %f", result, expected)
        }
    })
}
```

## Integration Testing

### Testing with Wassette CLI

Create scripts to test component loading and execution:

```bash
#!/bin/bash
# test/integration_test.sh

set -e

echo "Building component..."
just build

echo "Loading component..."
wassette component load ./target/wasm32-wasip2/release/calculator.wasm

echo "Testing add function..."
result=$(wassette component call calculator add --args '{"a": 5, "b": 3}')
expected="8"

if [[ "$result" == "$expected" ]]; then
    echo "✓ Add test passed"
else
    echo "✗ Add test failed: expected $expected, got $result"
    exit 1
fi

echo "Testing divide function..."
result=$(wassette component call calculator divide --args '{"a": 10, "b": 2}')
expected="5"

if [[ "$result" == "$expected" ]]; then
    echo "✓ Divide test passed"
else
    echo "✗ Divide test failed: expected $expected, got $result"
    exit 1
fi

echo "Testing error handling..."
set +e
result=$(wassette component call calculator divide --args '{"a": 10, "b": 0}' 2>&1)
set -e

if [[ "$result" == *"Division by zero"* ]]; then
    echo "✓ Error handling test passed"
else
    echo "✗ Error handling test failed: $result"
    exit 1
fi

echo "All integration tests passed!"
```

### Automated Integration Testing

Use test frameworks for structured integration tests:

```javascript
// test/integration.test.js
import { spawn } from 'child_process';
import { promisify } from 'util';

const exec = promisify(require('child_process').exec);

describe('Integration Tests', () => {
    beforeAll(async () => {
        // Build component
        await exec('just build');
        
        // Load component
        await exec('wassette component load ./target/wasm32-wasip2/release/calculator.wasm');
    });

    afterAll(async () => {
        // Cleanup
        await exec('wassette component unload calculator');
    });

    test('should call add function successfully', async () => {
        const { stdout } = await exec('wassette component call calculator add --args \'{"a": 5, "b": 3}\'');
        expect(stdout.trim()).toBe('8');
    });

    test('should handle division by zero', async () => {
        try {
            await exec('wassette component call calculator divide --args \'{"a": 10, "b": 0}\'');
            fail('Expected error for division by zero');
        } catch (error) {
            expect(error.stderr).toContain('Division by zero');
        }
    });
});
```

## Security Testing

### Permission Testing

Verify that components respect permission boundaries:

```bash
#!/bin/bash
# test/security_test.sh

echo "Testing file access permissions..."

# Load component with minimal permissions
cat > test_policy.yaml << EOF
version: "1.0"
permissions:
  storage:
    allow:
      - uri: "fs://test/allowed/**"
        access: ["read", "write"]
EOF

wassette component load ./file-processor.wasm --policy test_policy.yaml

# Test allowed access
result=$(wassette component call file-processor read-file --args '{"path": "test/allowed/file.txt"}')
if [[ $? -eq 0 ]]; then
    echo "✓ Allowed file access works"
else
    echo "✗ Allowed file access failed"
    exit 1
fi

# Test denied access
set +e
result=$(wassette component call file-processor read-file --args '{"path": "/etc/passwd"}' 2>&1)
set -e

if [[ "$result" == *"permission denied"* ]]; then
    echo "✓ Unauthorized file access properly denied"
else
    echo "✗ Security violation: unauthorized access allowed"
    exit 1
fi
```

### Network Permission Testing

```python
# test_network_security.py
import subprocess
import json
import pytest

def run_component_call(component, function, args):
    """Helper to call component functions"""
    cmd = [
        'wassette', 'component', 'call', component, function, 
        '--args', json.dumps(args)
    ]
    result = subprocess.run(cmd, capture_output=True, text=True)
    return result

def test_allowed_network_access():
    """Test that allowed hosts can be accessed"""
    result = run_component_call(
        'http-client', 
        'fetch', 
        {'url': 'https://api.allowed.com/data'}
    )
    assert result.returncode == 0

def test_denied_network_access():
    """Test that denied hosts are blocked"""
    result = run_component_call(
        'http-client', 
        'fetch', 
        {'url': 'https://malicious.example.com/data'}
    )
    assert result.returncode != 0
    assert 'permission denied' in result.stderr.lower()

def test_localhost_blocked():
    """Test that localhost access is blocked by default"""
    result = run_component_call(
        'http-client', 
        'fetch', 
        {'url': 'http://localhost:8080/admin'}
    )
    assert result.returncode != 0
    assert 'host not allowed' in result.stderr.lower()
```

## Performance Testing

### Resource Usage Testing

Monitor memory and CPU usage during component execution:

```bash
#!/bin/bash
# test/performance_test.sh

echo "Running performance tests..."

# Test memory usage
echo "Testing memory usage..."
result=$(wassette component call large-processor process-data \
  --args '{"size": 1000000}' \
  --monitor-memory)

memory_used=$(echo "$result" | grep "Memory used:" | cut -d: -f2 | tr -d ' ')
memory_limit="64MB"

if [[ "$memory_used" > "$memory_limit" ]]; then
    echo "✗ Memory usage exceeded limit: $memory_used > $memory_limit"
    exit 1
else
    echo "✓ Memory usage within limits: $memory_used"
fi

# Test execution time
echo "Testing execution time..."
start_time=$(date +%s%N)
wassette component call calculator fibonacci --args '{"n": 35}'
end_time=$(date +%s%N)

execution_time=$(( (end_time - start_time) / 1000000 )) # Convert to milliseconds
time_limit=5000 # 5 seconds

if [[ $execution_time -gt $time_limit ]]; then
    echo "✗ Execution time exceeded limit: ${execution_time}ms > ${time_limit}ms"
    exit 1
else
    echo "✓ Execution time within limits: ${execution_time}ms"
fi
```

### Load Testing

Test component behavior under high load:

```javascript
// test/load_test.js
import { performance } from 'perf_hooks';

async function callComponent(component, func, args) {
    // Implementation depends on your test setup
    // This is a simplified example
    const result = await fetch('/mcp', {
        method: 'POST',
        body: JSON.stringify({
            method: 'tools/call',
            params: { name: `${component}/${func}`, arguments: args }
        })
    });
    return result.json();
}

describe('Load Testing', () => {
    test('should handle concurrent requests', async () => {
        const concurrency = 10;
        const iterations = 100;
        
        const promises = [];
        
        for (let i = 0; i < concurrency; i++) {
            const promise = (async () => {
                const results = [];
                for (let j = 0; j < iterations; j++) {
                    const start = performance.now();
                    const result = await callComponent('calculator', 'add', { a: i, b: j });
                    const end = performance.now();
                    
                    results.push({
                        success: result.success,
                        latency: end - start
                    });
                }
                return results;
            })();
            
            promises.push(promise);
        }
        
        const allResults = await Promise.all(promises);
        const flatResults = allResults.flat();
        
        // Analyze results
        const successRate = flatResults.filter(r => r.success).length / flatResults.length;
        const avgLatency = flatResults.reduce((sum, r) => sum + r.latency, 0) / flatResults.length;
        const maxLatency = Math.max(...flatResults.map(r => r.latency));
        
        expect(successRate).toBeGreaterThan(0.99); // 99% success rate
        expect(avgLatency).toBeLessThan(100); // < 100ms average
        expect(maxLatency).toBeLessThan(1000); // < 1s max
    });
});
```

## End-to-End Testing

### AI Agent Integration

Test full workflows with actual AI agents:

```python
# test_e2e.py
import pytest
import json
from mcp_client import MCPClient

@pytest.fixture
async def mcp_client():
    """Set up MCP client connected to Wassette"""
    client = MCPClient()
    await client.connect('wassette', ['serve', '--stdio'])
    yield client
    await client.disconnect()

@pytest.mark.asyncio
async def test_tool_discovery(mcp_client):
    """Test that tools are properly discovered"""
    tools = await mcp_client.list_tools()
    
    # Verify expected tools are available
    tool_names = [tool['name'] for tool in tools['tools']]
    assert 'calculator/add' in tool_names
    assert 'calculator/divide' in tool_names

@pytest.mark.asyncio
async def test_tool_execution(mcp_client):
    """Test tool execution through MCP"""
    result = await mcp_client.call_tool(
        'calculator/add', 
        {'a': 5, 'b': 3}
    )
    
    assert result['content'][0]['text'] == '8'

@pytest.mark.asyncio
async def test_error_handling(mcp_client):
    """Test error handling through MCP"""
    with pytest.raises(Exception) as exc_info:
        await mcp_client.call_tool(
            'calculator/divide',
            {'a': 10, 'b': 0}
        )
    
    assert 'Division by zero' in str(exc_info.value)

@pytest.mark.asyncio
async def test_permission_enforcement(mcp_client):
    """Test that permissions are enforced through MCP"""
    with pytest.raises(Exception) as exc_info:
        await mcp_client.call_tool(
            'file-processor/read-file',
            {'path': '/etc/passwd'}
        )
    
    assert 'permission denied' in str(exc_info.value).lower()
```

## Test Automation

### CI/CD Integration

Set up automated testing in GitHub Actions:

```yaml
# .github/workflows/test.yml
name: Test Components

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    
    - name: Install Wassette
      run: cargo install wassette
    
    - name: Install component tools
      run: cargo install cargo-component
    
    - name: Run unit tests
      run: cargo test
    
    - name: Build component
      run: cargo component build --release
    
    - name: Run integration tests
      run: ./test/integration_test.sh
    
    - name: Run security tests
      run: ./test/security_test.sh
    
    - name: Run performance tests
      run: ./test/performance_test.sh
```

### Local Test Automation

Use Justfile for local test automation:

```make
# Justfile
test-all: test-unit test-integration test-security test-performance

test-unit:
    @echo "Running unit tests..."
    cargo test

test-integration: build
    @echo "Running integration tests..."
    ./test/integration_test.sh

test-security: build
    @echo "Running security tests..."
    ./test/security_test.sh

test-performance: build
    @echo "Running performance tests..."
    ./test/performance_test.sh

test-e2e: build
    @echo "Running end-to-end tests..."
    python -m pytest test/test_e2e.py

test-watch:
    @echo "Running tests in watch mode..."
    cargo watch -x test

build:
    cargo component build --release

clean:
    cargo clean
    rm -rf target/
```

## Testing Best Practices

### Test Organization

1. **Separate Test Types**: Keep unit, integration, and e2e tests in separate directories
2. **Descriptive Names**: Use clear, descriptive test names
3. **Test Data**: Use fixtures and test data files
4. **Cleanup**: Always clean up resources after tests

### Test Coverage

```bash
# Generate coverage reports
cargo install cargo-tarpaulin

# Run with coverage
cargo tarpaulin --out Html

# View coverage report
open tarpaulin-report.html
```

### Mock and Stub

For testing complex interactions:

```javascript
// Mock Wassette runtime for unit tests
jest.mock('../src/host-interface', () => ({
  readFile: jest.fn(),
  writeFile: jest.fn(),
  httpRequest: jest.fn(),
}));
```

### Property-Based Testing

Use property-based testing for robust validation:

```rust
// Use quickcheck for property-based testing
use quickcheck::{quickcheck, TestResult};

fn prop_add_commutative(a: i32, b: i32) -> bool {
    add(a, b) == add(b, a)
}

fn prop_divide_multiply_inverse(a: f64, b: f64) -> TestResult {
    if b == 0.0 {
        TestResult::discard()
    } else {
        let result = divide(a, b).unwrap();
        let back = result * b;
        TestResult::from_bool((back - a).abs() < 1e-10)
    }
}

quickcheck! {
    fn test_add_commutative(a: i32, b: i32) -> bool {
        prop_add_commutative(a, b)
    }
    
    fn test_divide_multiply_inverse(a: f64, b: f64) -> TestResult {
        prop_divide_multiply_inverse(a, b)
    }
}
```

## Debugging Tests

### Common Issues

1. **Component Loading Failures**: Check WIT syntax and build output
2. **Permission Errors**: Verify policy configuration
3. **Timeout Issues**: Increase timeouts for slow operations
4. **Flaky Tests**: Use proper wait conditions and cleanup

### Debug Tools

```bash
# Run tests with debug logging
RUST_LOG=debug cargo test

# Run single test with output
cargo test test_name -- --nocapture

# Debug integration tests
RUST_LOG=wassette=debug ./test/integration_test.sh
```

## Next Steps

- Learn about [Best Practices](./best-practices.md) for production-ready components
- Explore [Cookbook](../cookbook/common-patterns.md) for testing examples
- Review [Security Model](../security/security-model.md) for security testing guidance
- Check out [Contributing](../contributing.md) for testing standards