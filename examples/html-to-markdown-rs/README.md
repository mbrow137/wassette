# HTML to Markdown Rust Component

A WebAssembly component that converts HTML to Markdown format.

## Overview

This component provides a simple interface to convert HTML strings to Markdown format. It supports:

- Headers (h1-h6)
- Paragraphs
- Links
- Lists (both ordered and unordered)
- Blockquotes
- Code blocks and inline code
- Emphasis (bold and italic)
- Generic text containers

## Interface

The component exports a single function:

```wit
convert: func(html: string) -> result<string, string>
```

## Usage

The component takes an HTML string as input and returns the converted Markdown as a string, or an error message if the conversion fails.

## Building

To build this component, you need:

1. Rust toolchain with `wasm32-wasip2` target
2. `cargo-component` tool

```bash
# Install the target
rustup target add wasm32-wasip2

# Build the component
cargo component build --release
```

## Example

Input HTML:
```html
<h1>Hello World</h1>
<p>This is a <strong>paragraph</strong> with <em>emphasis</em>.</p>
<ul>
  <li>Item 1</li>
  <li>Item 2</li>
</ul>
```

Output Markdown:
```markdown
# Hello World

This is a **paragraph** with *emphasis*.

- Item 1
- Item 2
```