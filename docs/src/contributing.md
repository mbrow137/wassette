# Contributing to Wassette

Thank you for your interest in contributing to Wassette! This guide will help you get started with contributing to the project, whether you're fixing bugs, adding features, improving documentation, or creating components.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contributing Guidelines](#contributing-guidelines)
- [Component Development](#component-development)
- [Documentation](#documentation)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)

## Code of Conduct

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/)
or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

## Getting Started

### Ways to Contribute

- **Bug Reports**: Report issues you encounter
- **Feature Requests**: Suggest new features or improvements
- **Code Contributions**: Fix bugs or implement new features
- **Documentation**: Improve or expand documentation
- **Component Examples**: Create example components
- **Testing**: Help improve test coverage

### Prerequisites

- Git
- Rust (latest stable version)
- Node.js (for documentation and some examples)
- Docker (optional, for testing)

## Development Setup

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR-USERNAME/wassette.git
cd wassette

# Add upstream remote
git remote add upstream https://github.com/microsoft/wassette.git
```

### 2. Install Dependencies

```bash
# Install Rust dependencies
cargo build

# Install additional tools
cargo install cargo-component just mdbook

# Install Node.js dependencies (for docs)
cd docs && npm install && cd ..
```

### 3. Verify Setup

```bash
# Run tests to ensure everything works
cargo test

# Build the project
cargo build

# Run the server
cargo run -- serve --stdio
```

## Contributing Guidelines

### Coding Standards

#### Rust Code

- Follow standard Rust formatting: `cargo fmt`
- Ensure no clippy warnings: `cargo clippy`
- Write comprehensive tests for new functionality
- Document public APIs with rustdoc comments
- Use `anyhow` for error handling
- Follow the project's existing patterns and conventions

Example of well-documented code:

```rust
/// Loads a WebAssembly component from the specified URI
/// 
/// # Arguments
/// 
/// * `uri` - Component URI (file://, oci://, or https://)
/// * `policy` - Security policy to apply to the component
/// 
/// # Returns
/// 
/// * `Ok(ComponentId)` - Successfully loaded component ID
/// * `Err(Error)` - Loading failed with specific error
/// 
/// # Examples
/// 
/// ```rust
/// let component_id = load_component(
///     "oci://ghcr.io/example/tool:latest",
///     &default_policy
/// )?;
/// ```
/// 
/// # Security
/// 
/// Components are loaded with the principle of least privilege.
/// Only explicitly granted permissions are available to the component.
pub fn load_component(uri: &str, policy: &Policy) -> Result<ComponentId, Error> {
    // Implementation
}
```

#### Component Code

- Follow language-specific best practices
- Include comprehensive error handling
- Validate all inputs from the host
- Use minimal required permissions
- Include clear documentation and examples

### Commit Messages

Use clear, descriptive commit messages:

```
feat: Add support for streaming file processing

- Implement chunked file reading for large files
- Add memory-efficient processing pipeline
- Include tests for various file sizes
- Update documentation with streaming examples

Fixes #123
```

Format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `test:` for adding or updating tests
- `refactor:` for code refactoring
- `perf:` for performance improvements

### Pull Request Process

1. **Create a branch** from `main` for your work
2. **Make changes** following the coding standards
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Run all tests** and ensure they pass
6. **Submit a pull request** with a clear description

#### PR Description Template

```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Other (specify)

## Testing
- [ ] Added new tests
- [ ] Updated existing tests
- [ ] All tests pass locally
- [ ] Manual testing completed

## Documentation
- [ ] Updated relevant documentation
- [ ] Added code comments where needed
- [ ] Updated CHANGELOG.md

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Breaking changes are documented
- [ ] Security considerations addressed
```

## Component Development

### Creating Example Components

We welcome new example components that demonstrate different use cases:

#### Component Requirements

- **Clear Purpose**: Solve a specific, useful problem
- **Minimal Permissions**: Request only necessary permissions
- **Comprehensive Documentation**: Include README with usage examples
- **Tests**: Include unit and integration tests
- **Security**: Follow security best practices

#### Directory Structure

```
examples/my-component/
├── src/                    # Source code
├── wit/                    # WIT interface definitions
├── tests/                  # Test files
├── policy.yaml            # Security policy
├── Cargo.toml             # Dependencies (for Rust)
├── package.json           # Dependencies (for JS)
├── Justfile              # Build automation
├── README.md             # Component documentation
└── examples/             # Usage examples
```

#### Example Component Template

```rust
// src/lib.rs
//! My Component
//! 
//! This component demonstrates [specific functionality].
//! 
//! ## Features
//! 
//! - Feature 1
//! - Feature 2
//! 
//! ## Security
//! 
//! This component requires minimal permissions:
//! - Storage: Read access to workspace files
//! - Network: HTTPS access to api.example.com

wit_bindgen::generate!({
    path: "../wit/world.wit",
    world: "my-component",
});

struct Component;

impl Guest for Component {
    /// Main component function
    /// 
    /// # Arguments
    /// 
    /// * `input` - Input data to process
    /// 
    /// # Returns
    /// 
    /// * `Ok(String)` - Processed result
    /// * `Err(String)` - Error message
    fn process(input: String) -> Result<String, String> {
        // Validate input
        if input.is_empty() {
            return Err("Input cannot be empty".to_string());
        }
        
        // Process data
        let result = format!("Processed: {}", input);
        
        Ok(result)
    }
}

export!(Component);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_valid_input() {
        let result = Component::process("test".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Processed: test");
    }

    #[test]
    fn test_process_empty_input() {
        let result = Component::process("".to_string());
        assert!(result.is_err());
    }
}
```

### Component Guidelines

1. **Security First**: Always follow the principle of least privilege
2. **Error Handling**: Handle all error cases gracefully
3. **Input Validation**: Validate all inputs from the host
4. **Documentation**: Provide clear usage examples
5. **Testing**: Include comprehensive tests
6. **Performance**: Consider memory and CPU usage

## Documentation

### Documentation Structure

- **Concepts**: High-level explanations
- **Guides**: Step-by-step tutorials
- **Reference**: Detailed API documentation
- **Examples**: Practical code examples

### Writing Guidelines

- Use clear, concise language
- Include practical examples
- Add code samples that work
- Update related documentation when making changes
- Follow the existing documentation structure

### Building Documentation

```bash
# Build the documentation
just docs-build

# Serve documentation locally
just docs-serve

# Watch for changes
just docs-watch
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_tests
```

### Test Categories

1. **Unit Tests**: Test individual functions and modules
2. **Integration Tests**: Test component integration with Wassette
3. **End-to-End Tests**: Test complete workflows
4. **Security Tests**: Test permission enforcement
5. **Performance Tests**: Test resource usage

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = "test input";
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_output);
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

## Submitting Changes

### Before Submitting

1. **Run all tests**: `cargo test`
2. **Check formatting**: `cargo fmt -- --check`
3. **Check linting**: `cargo clippy`
4. **Update documentation**: Build and review docs
5. **Update CHANGELOG.md**: Add entry for your changes

### Contributor License Agreement

Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit https://cla.microsoft.com.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repositories using our CLA.

### Review Process

1. **Automated Checks**: CI will run tests and checks
2. **Code Review**: Maintainers will review your code
3. **Feedback**: Address any requested changes
4. **Approval**: Once approved, your PR will be merged

### After Your PR is Merged

- **Delete your branch**: Clean up your fork
- **Update your fork**: Sync with upstream changes
- **Celebrate**: Thank you for contributing! 🎉

## Getting Help

### Communication Channels

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Discord**: Join the `#wassette` channel on [Microsoft Open Source Discord](https://discord.gg/microsoft-open-source)

### Asking for Help

When asking for help:

1. **Search existing issues** to see if your question has been answered
2. **Provide context** about what you're trying to do
3. **Include relevant code** and error messages
4. **Describe what you've tried** so far

### Reporting Issues

When reporting bugs:

1. **Use the issue template** provided
2. **Include reproduction steps** 
3. **Provide environment details** (OS, Rust version, etc.)
4. **Include relevant logs** and error messages

## Recognition

Contributors are recognized in several ways:

- **Contributors List**: Listed in the project README
- **Release Notes**: Mentioned in release announcements
- **Community Recognition**: Highlighted in community discussions

Thank you for contributing to Wassette! Your contributions help make the project better for everyone.