# CEL-CXX Async Examples

This crate contains asynchronous examples for cel-cxx, demonstrating integration with popular async runtimes.

## Features

- **Tokio Integration**: High-performance async runtime examples
- **Async-std Integration**: Alternative async runtime examples  
- **Smol Integration**: Lightweight async runtime examples

## Running Examples

### Tokio Example
```bash
cargo run --bin tokio
```

### Async-std Example
```bash
cargo run --bin async-std
```

### Smol Example
```bash
cargo run --bin smol
```

## Why Separate Crate?

This crate is separate from the main `cel-cxx` examples because:

1. **Platform Compatibility**: Async runtimes often don't support WebAssembly targets
2. **Dependency Isolation**: Avoids pulling in async dependencies for synchronous use cases
3. **Build Performance**: Faster builds when async features aren't needed

## Examples Overview

Each example demonstrates:
- Setting up async environments with CEL-CXX
- Concurrent expression evaluation
- Error handling in async contexts
- Integration patterns for different runtimes
