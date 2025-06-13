# cel-cxx

[![Crates.io](https://img.shields.io/crates/v/cel-cxx.svg)](https://crates.io/crates/cel-cxx)
[![Docs.rs](https://docs.rs/cel-cxx/badge.svg)](https://docs.rs/cel-cxx)
[![CI](https://github.com/xjasonli/cel-cxx/actions/workflows/rust.yml/badge.svg)](https://github.com/xjasonli/cel-cxx/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

A Rust-native interface for [Common Expression Language (CEL)](https://github.com/google/cel-spec),
implemented as a high-level idiomatic Rust wrapper around [google/cel-cpp](https://github.com/google/cel-cpp)
via the [cxx](https://github.com/dtolnay/cxx) crate.

## Overview

This crate provides a modern, ergonomic Rust API for CEL that leverages Rust's type system
and language features to offer a safe, efficient, and developer-friendly experience. It
maintains full compatibility with the CEL specification while providing idiomatic Rust
abstractions and zero-cost FFI bindings.

## Key Features

- **Idiomatic Rust API**: Type-safe interfaces with Rust's type system and error handling
- **Zero-cost FFI**: Efficient C++ interop through the cxx crate
- **Async Support**: First-class async/await support with multiple runtime options
- **Rich Type System**: Full support for CEL's type system with seamless Rust type conversions
- **Custom Types**: Derive macros for creating custom CEL types from Rust structs

## Quick Start

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
cel-cxx = "0.1.0"  # Use the latest version
```

### Basic Usage

```rust
use cel_cxx::*;

// Create an environment and compile an expression
let program = Env::builder()
    .declare_variable::<String>("name")?
    .compile("'Hello, ' + name + '!'")?;

// Create an activation with variable bindings
let activation = Activation::new()
    .bind_variable("name", "World")?;

// Evaluate the expression
let result = program.evaluate(activation)?;
println!("{}", result); // "Hello, World!"
```

### With Custom Functions

```rust
use cel_cxx::*;
use std::convert::Infallible;

// Register a custom function with type-safe Rust closures
let program = Env::builder()
    .register_global_function("double", |x: i64| -> Result<i64, Infallible> { Ok(x * 2) })?
    .build()?
    .compile("double(21)")?;

let result = env.evaluate(())?;
// result == Value::Int(42)
```

## Examples

This repository contains comprehensive examples showcasing various capabilities of the `cel-cxx` crate.

All examples can be executed with:

```bash
cargo run --example <example_name>
```

### Available Examples

- **[`helloworld`](examples/helloworld.rs)** - A comprehensive introduction covering:
  - Basic expression evaluation
  - Variable binding and providers
  - Global function registration
  - Custom types with derive macros
  - Member function registration
  - Type introspection

- **[`tokio`](examples/tokio.rs)** - Demonstrates async support with Tokio runtime:
  - Async member functions
  - Async global functions
  - Async variable providers

- **[`async-std`](examples/async-std.rs)** - Shows async support with async-std runtime:
  - Same async features as Tokio example but using async-std

A good starting point would be the [`helloworld`](examples/helloworld.rs) example, which covers most of the core functionality.

If you have an example you'd like to see, please feel free to open an issue. Contributions of new examples are also welcome via pull requests!

## Feature Flags

- **`async`**: Enables asynchronous evaluation of CEL expressions with support for
  [async-std](https://github.com/async-rs/async-std) or
  [tokio](https://github.com/tokio-rs/tokio) runtimes.
- **`derive`**: Enables derive macros for custom types:
  - `#[derive(Opaque)]` for creating custom CEL types from Rust structs
  - `#[cel_cxx(...)]` attributes for fine-grained type control
- **`tokio`**: Enables Tokio async runtime support (requires `async` feature)
- **`async-std`**: Enables async-std runtime support (requires `async` feature)

## Async Support

When the `async` feature is enabled, you can evaluate expressions asynchronously
with full async/await support:

```rust
# #[cfg(feature = "async")]
# async fn example() {
use std::convert::Infallible;
use cel_cxx::*;

let env = Env::builder()
    .use_tokio()
    .register_global_function("async_double", async |x: i64| -> Result<i64, Infallible> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        Ok(x * 2)
    }))?
    .build()?
    .compile("async_double(21)")?;

let result = env.evaluate(()).await?;
// result == Value::Int(42)
# }
```

## Prerequisites

- **Rust**: Latest stable version (recommended)
- **C++ Toolchain**: C++17 compatible compiler (GCC, Clang, or MSVC)
- **google/cel-cpp**: The C++ implementation of CEL
  - Follow the [installation guide](https://github.com/google/cel-cpp#building) for your platform
  - Ensure the library is discoverable via pkg-config or set appropriate environment variables

## Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/xjasonli/cel-cxx.git
   cd cel-cxx
   ```

2. Build and test:
   ```bash
   cargo build
   cargo test
   ```

## Contributing

Contributions are welcome! Please feel free to submit pull requests, create issues, or suggest improvements.

When contributing:
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please ensure your code follows the project's coding style and includes appropriate tests.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [google/cel-cpp](https://github.com/google/cel-cpp) - The C++ implementation of CEL
- [cxx](https://github.com/dtolnay/cxx) - Safe interop between Rust and C++
- The CEL community for their excellent work on the specification and implementations 
