# Documentation

Welcome to the cel-cxx documentation! This guide covers everything you need to know to use cel-cxx effectively.

## Table of Contents

- [Documentation](#documentation)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
  - [Architecture](#architecture)
    - [Core Design Principles](#core-design-principles)
    - [Integration Architecture](#integration-architecture)
  - [Type System](#type-system)
    - [Type Conversion Examples](#type-conversion-examples)
  - [Feature Flags](#feature-flags)
  - [Performance](#performance)
  - [Examples](#examples)
  - [Guides](#guides)
    - [Function Registration](#function-registration)
    - [Advanced Features](#advanced-features)
  - [Reference](#reference)
  - [Language Documentation](#language-documentation)

## Getting Started

### Prerequisites

**System Requirements:**
- **Rust**: 1.80+
- **C++ Toolchain**: C++17 compatible compiler
  - Linux: GCC 9+ or Clang 15+
  - macOS: Xcode 10+ or Clang 15+
  - Windows: MSVC 2022+

**Installation Verification:**
```bash
# Clone and test
git clone https://github.com/xjasonli/cel-cxx.git
cd cel-cxx
cargo test --all-targets

# Run examples
cargo run --example comprehensive
cargo run --example tokio --features="tokio"
```

For a quick start guide, see the [main README](../README.md#quick-start).

## Architecture

### Core Design Principles

- **Type Safety**: Compile-time verification of CEL expressions and function signatures
- **Zero-Cost Abstractions**: Direct FFI calls to CEL-CPP with minimal overhead
- **Memory Safety**: Rust ownership system prevents common integration bugs
- **Ergonomic API**: Builder patterns and automatic type inference reduce boilerplate
- **Extensibility**: Support for custom types and async operations

### Integration Architecture

The library provides a layered architecture that bridges Rust and CEL-CPP:

- **Application Layer**: High-level APIs for environment building and expression evaluation
- **Type System Layer**: Automatic conversions between Rust and CEL types
- **FFI Layer**: Zero-cost bindings to CEL-CPP via the `cxx` crate
- **CEL-CPP Layer**: Google's reference implementation for parsing and evaluation

## Type System

The crate provides comprehensive type support with automatic conversions between CEL and Rust types.
All types support the three core traits for seamless integration:

| CEL Type | | Rust Type | | |
|----------|---|-----------|---|---|
| | | **Declare** | **To CEL** | **From CEL** |
| | | `TypedValue` | `IntoValue` | `FromValue` |
| `null` | | `()` | ✅ | ✅ | ✅ |
| `bool` | | `bool` | ✅ | ✅ | ✅ |
| `int` | | `i64`, `i32`, `i16`, `isize` | ✅ | ✅ | ✅ |
| `uint` | | `u64`, `u32`, `u16`, `usize` | ✅ | ✅ | ✅ |
| `double` | | `f64`, `f32` | ✅ | ✅ | ✅ |
| `string` | | `String`, `ArcStr`, `Box<str>`, `str` | ✅ | ✅ | ✅ |
| `bytes` | | `Vec<u8>`, `ArcBytes`, `Box<[u8]>`, `[u8]` | ✅ | ✅ | ✅ |
| `duration` | | `chrono::Duration` | ✅ | ✅ | ✅ |
| `timestamp` | | `chrono::DateTime<Utc>`, `SystemTime` | ✅ | ✅ | ✅ |
| `list<T>` | | `Vec<T>`, `VecDeque<T>`, `LinkedList<T>`, `[T]` | ✅ | ✅ | ✅ |
| `map<K,V>` | | `HashMap<K,V>`, `BTreeMap<K,V>`, `Vec<(K,V)>` | ✅ | ✅ | ✅ |
| `optional<T>` | | `Option<T>`, `Optional<T>` | ✅ | ✅ | ✅ |
| `type` | | `ValueType` | ✅ | ✅ | ✅ |
| `error` | | `Error` | ✅ | ✅ | ✅ |
| `opaque` | | `#[derive(Opaque)] struct` | ✅ | ✅ | ✅ |

**Special Reference Support**: All `&T` types support **Declare** and **To CEL** operations,
enabling zero-copy function arguments like `&str`, `&[u8]`, `&MyStruct`, etc.

### Type Conversion Examples

```rust,no_run
use cel_cxx::*;
use std::collections::VecDeque;

// Automatic conversions work seamlessly
let env = Env::builder()
    // Different integer types all map to CEL int
    .register_global_function("process_i32", |x: i32| x * 2)?
    .register_global_function("process_i64", |x: i64| x * 2)?
    
    // String types are interchangeable
    .register_global_function("process_string", |s: String| s.to_uppercase())?
    .register_global_function("process_str", |s: &str| s.len() as i64)?
    
    // Container types work with any compatible Rust collection
    .register_global_function("sum_vec", |nums: Vec<i64>| nums.iter().sum::<i64>())?
    .register_global_function("sum_deque", |nums: VecDeque<i64>| nums.iter().sum::<i64>())?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `derive` | Derive macros for custom types (`#[derive(Opaque)]`) | ✅ |
| `async` | Async/await support for expressions and functions | ❌ |
| `tokio` | Tokio async runtime integration (requires `async`) | ❌ |
| `smol` | smol runtime integration (requires `async`) | ❌ |
| `async-std` | async-std runtime integration (requires `async`) | ❌ |

## Performance

- **Zero-cost FFI**: Direct C++ function calls with no marshaling overhead
- **Compile-time optimization**: Function signatures resolved at compile time  
- **Memory efficient**: Minimal allocations through smart reference handling
- **Async overhead**: Only when async features are explicitly used
- **Type safety**: Compile-time prevention of common integration errors

## Examples

The crate includes comprehensive examples demonstrating various features:

- **Basic Usage**: Variable binding, function registration, expression evaluation
- **Custom Types**: Derive macros, member functions, type integration  
- **Async Support**: Tokio/smol/async-std integration examples
- **Advanced Features**: Function overloads, error handling, complex type conversions

Run examples with:
```bash
cargo run --example comprehensive
cargo run --example tokio --features="tokio"
```

## Guides

### [Function Registration](function-registration.md)

Comprehensive guide to zero-annotation function registration using Generic Associated Types (GATs).
Learn how to register functions with owned types, reference types, async functions, and more.

### [Advanced Features](advanced-features.md)

Deep dive into advanced features including async support architecture, function overloads,
and smart reference handling.

## Reference

- [CEL Reference](cel-reference/) - CEL language features and standard library documentation
- [API Documentation](https://docs.rs/cel-cxx) - Complete API reference on docs.rs
- [GitHub Repository](https://github.com/xjasonli/cel-cxx) - Source code and issue tracker
- [Crates.io](https://crates.io/crates/cel-cxx) - Package registry

## Language Documentation

- [English Documentation](../README.md) (Main)
- [中文文档](../docs-cn/) (Chinese)
