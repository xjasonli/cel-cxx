[![github]](https://github.com/xjasonli/cel-cxx)
[![crates-io]](https://crates.io/crates/cel-cxx)
[![docs-rs]](https://docs.rs/cel-cxx)
[![deepwiki]](https://deepwiki.com/xjasonli/cel-cxx)

[github]: https://img.shields.io/badge/GitHub-Repository-blue?style=flat&logo=github
[crates-io]: https://img.shields.io/crates/v/cel-cxx.svg
[docs-rs]: https://docs.rs/cel-cxx/badge.svg
[deepwiki]: https://deepwiki.com/badge.svg

- [CEL-CXX: Modern Rust Interface for CEL](#cel-cxx-modern-rust-interface-for-cel)
  - [Architecture Overview](#architecture-overview)
    - [Core Design Principles](#core-design-principles)
    - [Integration Architecture](#integration-architecture)
  - [Quick Start](#quick-start)
    - [Installation](#installation)
    - [Basic Expression Evaluation](#basic-expression-evaluation)
    - [Custom Types with Derive Macros](#custom-types-with-derive-macros)
  - [Zero-Annotation Function Registration](#zero-annotation-function-registration)
    - [Owned Type Parameters](#owned-type-parameters)
    - [Reference Type Parameters](#reference-type-parameters)
    - [Reference Type Return Values](#reference-type-return-values)
    - [Direct Return Values vs Result Types](#direct-return-values-vs-result-types)
    - [Synchronous vs Asynchronous Functions](#synchronous-vs-asynchronous-functions)
    - [Function Signature Examples](#function-signature-examples)
  - [Advanced Features](#advanced-features)
    - [Async Support](#async-support)
      - [Async Architecture Design](#async-architecture-design)
      - [Implementation Details](#implementation-details)
    - [Function Overloads](#function-overloads)
    - [Smart Reference Handling](#smart-reference-handling)
  - [Type System](#type-system)
    - [Type Conversion Examples](#type-conversion-examples)
  - [Feature Flags](#feature-flags)
  - [Performance Characteristics](#performance-characteristics)
  - [Examples](#examples)
  - [Platform Support](#platform-support)
    - [Cross-Compilation Support](#cross-compilation-support)
    - [Android Build Instructions](#android-build-instructions)
  - [CEL Feature Support](#cel-feature-support)
    - [Supported Features](#supported-features)
    - [Planned Features](#planned-features)
  - [Prerequisites](#prerequisites)
    - [System Requirements](#system-requirements)
    - [Installation Verification](#installation-verification)
  - [License](#license)
  - [Acknowledgements](#acknowledgements)


# CEL-CXX: Modern Rust Interface for CEL

A high-performance, type-safe Rust interface for [Common Expression Language (CEL)](https://github.com/google/cel-spec),
built on top of [cel-cpp](https://github.com/google/cel-cpp) with zero-cost FFI bindings via [cxx](https://github.com/dtolnay/cxx).

## Architecture Overview

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

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cel-cxx = "0.1.0"

# Optional features
cel-cxx = { version = "0.1.0", features = ["tokio"] }
```

### Basic Expression Evaluation

```rust,no_run
use cel_cxx::*;

// 1. Build environment with variables and functions
let env = Env::builder()
    .declare_variable::<String>("name")?
    .declare_variable::<i64>("age")?
    .register_global_function("adult", |age: i64| age >= 18)?
    .build()?;

// 2. Compile expression
let program = env.compile("'Hello ' + name + '! You are ' + (adult(age) ? 'an adult' : 'a minor')")?;

// 3. Create activation with variable bindings
let activation = Activation::new()
    .bind_variable("name", "Alice")?
    .bind_variable("age", 25i64)?;

// 4. Evaluate
let result = program.evaluate(&activation)?;
println!("{}", result); // "Hello Alice! You are an adult"
# Ok::<(), cel_cxx::Error>(())
```

### Custom Types with Derive Macros

```rust,no_run
use cel_cxx::*;

#[derive(Opaque, Debug, Clone, PartialEq)]
#[cel_cxx(type = "myapp.User")]
struct User {
    name: String,
    age: i32,
    roles: Vec<String>,
}

impl User {
    // Struct methods can be registered directly as CEL member functions
    fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }
    
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
    
    fn get_role_count(&self) -> i64 {
        self.roles.len() as i64
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User({})", self.name)
    }
}

let env = Env::builder()
    .declare_variable::<User>("user")?
    // ✨ Register struct methods directly - &self becomes CEL receiver
    .register_member_function("has_role", User::has_role)?
    .register_member_function("is_adult", User::is_adult)?
    .register_member_function("get_role_count", User::get_role_count)?
    .build()?;

let program = env.compile("user.has_role('admin') && user.is_adult()")?;
# Ok::<(), cel_cxx::Error>(())
```

## Zero-Annotation Function Registration

The library's flagship feature uses **Generic Associated Types (GATs)** to automatically infer function signatures,
eliminating the need for manual type annotations:

```rust,no_run
use cel_cxx::*;

let env = Env::builder()
    // ✨ Function signatures automatically inferred from Rust types!
    .register_global_function("add", |a: i64, b: i64| a + b)?
    .register_global_function("concat", |a: String, b: &str| a + b)?
    .register_global_function("length", |s: &str| s.len() as i64)?
    .register_global_function("parse", |s: &str| s.parse::<i64>())?  // Result<i64, _> auto-handled
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

This system supports a wide variety of function patterns:

### Owned Type Parameters

Functions can accept owned values, which are automatically converted from CEL types:

```rust,no_run
use cel_cxx::*;

let env = Env::builder()
    // Basic owned types
    .register_global_function("add", |a: i64, b: i64| a + b)?
    .register_global_function("concat", |a: String, b: String| a + &b)?
    .register_global_function("sum_list", |nums: Vec<i64>| nums.iter().sum::<i64>())?
    
    // Complex owned types
    .register_global_function("process_map", |data: std::collections::HashMap<String, i64>| {
        data.values().sum::<i64>()
    })?
    .register_global_function("handle_optional", |maybe_val: Option<String>| {
        maybe_val.unwrap_or_else(|| "default".to_string())
    })?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

### Reference Type Parameters

Reference parameters enable zero-copy operations for performance-critical code:

```rust,no_run
use cel_cxx::*;

let env = Env::builder()
    // String references - no copying required
    .register_global_function("length", |s: &str| s.len() as i64)?
    .register_global_function("starts_with", |text: &str, prefix: &str| text.starts_with(prefix))?
    
    // Collection element references - containers hold owned values
    .register_global_function("first", |items: Vec<i64>| items.first().copied().unwrap_or(0))?
    .register_global_function("contains", |haystack: Vec<&str>, needle: &str| {
        haystack.iter().any(|&s| s == needle)
    })?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

### Reference Type Return Values

Functions can return references to data within their parameters, enabling efficient data access:

```rust,no_run
use cel_cxx::*;

// Define functions that return references with proper lifetime annotations
fn get_domain(email: &str) -> &str {
    email.split('@').nth(1).unwrap_or("")
}

fn get_substring(text: &str, start: i64) -> &str {
    let start = start as usize;
    if start < text.len() { &text[start..] } else { "" }
}

let env = Env::builder()
    // Return string slices from borrowed parameters using named functions
    .register_global_function("get_domain", get_domain)?
    
    // Return owned values from owned containers using closures
    .register_global_function("get_first", |items: Vec<String>| {
        items.into_iter().next().unwrap_or_default()
    })?
    
    // Return references to parameter data using named functions
    .register_global_function("get_substring", get_substring)?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

### Direct Return Values vs Result Types

The system supports both direct return values and `Result<T, E>` for error handling:

```rust,no_run
use cel_cxx::*;
use std::num::ParseIntError;
use std::io;

let env = Env::builder()
    // Direct return values - always succeed
    .register_global_function("double", |x: i64| x * 2)?
    .register_global_function("format_name", |first: &str, last: &str| {
        format!("{}, {}", last, first)
    })?
    
    // Result return values - can fail gracefully with standard library errors
    .register_global_function("parse_int", |s: &str| -> Result<i64, ParseIntError> {
        s.parse()
    })?
    .register_global_function("divide", |a: f64, b: f64| -> Result<f64, io::Error> {
        if b == 0.0 {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Division by zero"))
        } else {
            Ok(a / b)
        }
    })?
    
    // Result with owned return values and concrete error types
    .register_global_function("safe_index", |items: Vec<String>, idx: i64| -> Result<String, io::Error> {
        let index = idx as usize;
        items.get(index)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Index out of bounds"))
    })?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

### Synchronous vs Asynchronous Functions

Both sync and async functions are supported seamlessly:

```rust,no_run
# #[cfg(feature = "async")]
# async fn example() -> Result<(), cel_cxx::Error> {
use cel_cxx::*;

let env = Env::builder()
    .use_tokio()
    
    // Synchronous functions - execute immediately
    .register_global_function("sync_add", |a: i64, b: i64| a + b)?
    .register_global_function("sync_format", |name: &str| format!("Hello, {}", name))?
    
    // Asynchronous functions - return futures
    .register_global_function("async_fetch", async |id: i64| -> Result<String, std::io::Error> {
        // Simulate async database call
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(format!("Data for ID: {}", id))
    })?
    .register_global_function("async_validate", async |email: &str| -> Result<bool, std::fmt::Error> {
        // Simulate async validation service
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        Ok(email.contains('@'))
    })?
    
    // Mixed sync and async in same environment
    .register_global_function("process", |data: String| data.to_uppercase())?
    .register_global_function("async_process", async |data: String| {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        data.to_lowercase()
    })?
    .build()?;

// Async evaluation when any async functions are used
let program = env.compile("async_fetch(42) + ' - ' + async_validate('user@example.com')")?;
let result = program.evaluate(()).await?;
# Ok(())
# }
```

### Function Signature Examples

Here's a comprehensive overview of supported function signatures:

```rust,no_run
use cel_cxx::*;

// Define function that returns reference with proper lifetime annotation
fn substring_fn(s: &str, start: i64, len: i64) -> &str {
    let start = start as usize;
    let end = (start + len as usize).min(s.len());
    &s[start..end]
}

// All of these function signatures are automatically inferred:
let env = Env::builder()
    // No parameters
    .register_global_function("now", || std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64)?
    .register_global_function("pi", || std::f64::consts::PI)?
    
    // Single parameter - various types
    .register_global_function("abs", |x: i64| x.abs())?
    .register_global_function("uppercase", |s: String| s.to_uppercase())?
    .register_global_function("len", |s: &str| s.len() as i64)?
    
    // Multiple parameters - mixed types (using named function for lifetime)
    .register_global_function("substring", substring_fn)?
    
    // Generic collections - owned containers
    .register_global_function("join", |items: Vec<String>, sep: &str| items.join(sep))?
    .register_global_function("filter_positive", |nums: Vec<i64>| {
        nums.into_iter().filter(|&x| x > 0).collect::<Vec<_>>()
    })?
    
    // Optional types
    .register_global_function("unwrap_or", |opt: Option<String>, default: String| {
        opt.unwrap_or(default)
    })?
    
    // Result types for error handling with standard library errors
    .register_global_function("safe_divide", |a: f64, b: f64| -> Result<f64, std::io::Error> {
        if b == 0.0 { 
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Division by zero")) 
        } else { 
            Ok(a / b) 
        }
    })?
    .register_global_function("parse_float", |s: &str| -> Result<f64, std::num::ParseFloatError> {
        s.parse()
    })?
    .build()?;
# Ok::<(), cel_cxx::Error>(())
```

## Advanced Features

### Async Support

When the `async` feature is enabled, you can evaluate expressions asynchronously:

```rust,no_run
# #[cfg(feature = "async")]
# async fn example() -> Result<(), cel_cxx::Error> {
use cel_cxx::*;

let env = Env::builder()
    .use_tokio()
    .register_global_function("async_fetch", async |id: i64| -> Result<String, Error> {
        // Simulate async database call
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(format!("Data for ID: {}", id))
    })?
    .build()?;

let program = env.compile("async_fetch(42)")?;
let result = program.evaluate(()).await?;
# Ok(())
# }
```

#### Async Architecture Design

Supporting Rust async functions in CEL presents unique challenges since CEL-CPP doesn't
natively support asynchronous or callback-based user-defined functions and variable providers.
When a Rust async function returns a `Future`, it has already exited the current stack frame,
and the C++ CEL evaluation engine cannot schedule or await Rust futures.

**cel-cxx** solves this through an innovative dual-threading architecture:

1. **Async-to-Blocking Bridge**: When async functions or variable providers are registered,
   the entire program evaluation is moved to a blocking thread using `Runtime::spawn_blocking()`.
   The main async context receives a future that resolves when evaluation completes.

2. **Blocking-to-Async Bridge**: When async callbacks are invoked within the blocking thread,
   the returned futures are dispatched back to the async runtime for execution, while the
   blocking thread waits for completion using `Runtime::block_on()`.

#### Implementation Details

- **Lifetime Management**: Since user-provided functions and variable providers can be capturing
  closures with complex lifetimes, cel-cxx uses the [`async-scoped`](https://crates.io/crates/async-scoped)
  crate to safely manage these lifetimes across thread boundaries.

- **Multi-threaded Runtime Requirement**: When using Tokio, the runtime must be multi-threaded
  because the implementation relies on [`tokio::task::block_in_place()`](https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html),
  which panics in single-threaded runtimes.

This design enables seamless integration of async Rust code with the synchronous CEL-CPP
evaluation engine, maintaining both performance and correctness across runtime boundaries.

### Function Overloads

The library supports function overloading with automatic type resolution:

```rust,no_run
use cel_cxx::*;

let env = Env::builder()
    // Multiple functions with same name, different signatures
    .register_global_function("process", |x: i64| x * 2)?
    .register_global_function("process", |x: f64| x * 2.0)?
    .register_global_function("process", |x: String| x.to_uppercase())?
    .build()?;

// CEL will automatically choose the right overload based on argument types
let program1 = env.compile("process(42)")?;      // Calls i64 version
let program2 = env.compile("process(3.14)")?;    // Calls f64 version  
let program3 = env.compile("process('hello')")?; // Calls String version
# Ok::<(), cel_cxx::Error>(())
```

### Smart Reference Handling

The library automatically manages reference types with safe lifetime handling:

```rust,no_run
use cel_cxx::*;
use std::collections::HashMap;

// ✅ These reference patterns work automatically:
let env = Env::builder()
    .declare_variable::<Vec<&str>>("string_refs")?        // Borrowed strings
    .declare_variable::<HashMap<i64, &str>>("lookup")?    // Borrowed values
    .declare_variable::<Option<&str>>("maybe_str")?       // Optional borrows
    .build()?;

// The library prevents unsafe patterns at compile time:
// ❌ .declare_variable::<&Vec<String>>("invalid")?  // Compiler error
# Ok::<(), cel_cxx::Error>(())
```

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

## Performance Characteristics

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

## Platform Support

<table>
<thead>
<tr>
<th>Platform</th>
<th>Target Triple</th>
<th>Status</th>
<th>Notes</th>
</tr>
</thead>
<tbody>
<tr>
<td rowspan="4"><strong>Linux</strong></td>
<td><code>x86_64-unknown-linux-gnu</code></td>
<td>✅</td>
<td>Tested</td>
</tr>
<tr>
<td><code>aarch64-unknown-linux-gnu</code></td>
<td>✅</td>
<td>Tested</td>
</tr>
<tr>
<td><code>armv7-unknown-linux-gnueabi</code></td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td><code>i686-unknown-linux-gnu</code></td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
<tr>
<td><strong>Windows</strong></td>
<td><code>x86_64-pc-windows-msvc</code></td>
<td>✅</td>
<td>Tested (Visual Studio 2022+)</td>
</tr>
<tr>
<td rowspan="3"><strong>macOS</strong></td>
<td><code>x86_64-apple-darwin</code></td>
<td>✅</td>
<td>Tested</td>
</tr>
<tr>
<td><code>aarch64-apple-darwin</code></td>
<td>✅</td>
<td>Tested</td>
</tr>
<tr>
<td><code>arm64e-apple-darwin</code></td>
<td>✅</td>
<td>Tested</td>
</tr>
<tr>
<td rowspan="4"><strong>Android</strong></td>
<td><code>aarch64-linux-android</code></td>
<td>🟡</td>
<td>Should work, use cargo-ndk</td>
</tr>
<tr>
<td><code>armv7-linux-androideabi</code></td>
<td>🟡</td>
<td>Should work, use cargo-ndk</td>
</tr>
<tr>
<td><code>x86_64-linux-android</code></td>
<td>🟡</td>
<td>Should work, use cargo-ndk</td>
</tr>
<tr>
<td><code>i686-linux-android</code></td>
<td>🟡</td>
<td>Should work, use cargo-ndk</td>
</tr>
<tr>
<td rowspan="4"><strong>iOS</strong></td>
<td><code>aarch64-apple-ios</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>aarch64-apple-ios-sim</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>x86_64-apple-ios</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>arm64e-apple-ios</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td rowspan="3"><strong>tvOS</strong></td>
<td><code>aarch64-apple-tvos</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>aarch64-apple-tvos-sim</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>x86_64-apple-tvos</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td rowspan="5"><strong>watchOS</strong></td>
<td><code>aarch64-apple-watchos</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>aarch64-apple-watchos-sim</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>x86_64-apple-watchos-sim</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>arm64_32-apple-watchos</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>armv7k-apple-watchos</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td rowspan="2"><strong>visionOS</strong></td>
<td><code>aarch64-apple-visionos</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><code>aarch64-apple-visionos-sim</code></td>
<td>🟡</td>
<td>Should work, untested</td>
</tr>
<tr>
<td><strong>WebAssembly</strong></td>
<td><code>wasm32-unknown-emscripten</code></td>
<td>✅</td>
<td>Tested via cross-rs</td>
</tr>
</tbody>
</table>

**Legend:**
- ✅ **Tested**: Confirmed working with automated tests
- 🟡 **Should work**: Build configuration exists but not tested in CI

### Cross-Compilation Support

cel-cxx includes built-in support for cross-compilation via [cross-rs](https://github.com/cross-rs/cross). The build system automatically detects cross-compilation environments and configures the appropriate toolchains.

**Usage with cross-rs:**
```bash
# Install cross-rs
cargo install cross --git https://github.com/cross-rs/cross

# Build for aarch64
cross build --target aarch64-unknown-linux-gnu
```

**Note**: Not all cross-rs targets are supported due to CEL-CPP's build requirements. musl targets and some embedded targets may not work due to missing C++ standard library support or incompatible toolchains.

### Android Build Instructions

Android builds require additional setup beyond the standard Rust toolchain:

**Prerequisites:**
1. Install Android NDK and set `ANDROID_NDK_HOME`
2. Install `cargo-ndk` for simplified Android builds

```bash
# Install cargo-ndk
cargo install cargo-ndk

# Add Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android
```

**Building for Android:**
```bash
# Build for ARM64 (recommended)
cargo ndk --target aarch64-linux-android build

# Build for ARMv7
cargo ndk --target armv7-linux-androideabi build

# Build for x86_64 (emulator)
cargo ndk --target x86_64-linux-android build

# Build for i686 (emulator)
cargo ndk --target i686-linux-android build
```

**Why cargo-ndk is required:**
- `ANDROID_NDK_HOME` configures Bazel for CEL-CPP compilation
- `cargo-ndk` automatically sets up `CC_{target}` and `AR_{target}` environment variables needed for the Rust FFI layer
- This ensures both the C++ (CEL-CPP) and Rust (cel-cxx-ffi) components use compatible toolchains



## CEL Feature Support

### Supported Features

| Feature | Status | Description |
|---------|--------|-------------|
| **Basic Types** | ✅ | `null`, `bool`, `int`, `uint`, `double`, `string`, `bytes` |
| **Collections** | ✅ | `list<T>`, `map<K,V>` with full indexing and comprehensions |
| **Time Types** | ✅ | `duration`, `timestamp` with full arithmetic support |
| **Operators** | ✅ | Arithmetic, logical, comparison, and membership operators |
| **Functions** | ✅ | Built-in functions and custom function registration |
| **Variables** | ✅ | Variable binding and scoping |
| **Conditionals** | ✅ | Ternary operator and logical short-circuiting |
| **Comprehensions** | ✅ | List and map comprehensions with filtering |
| **Optional Types** | ✅ | `optional<T>` with safe navigation |
| **Custom Types** | ✅ | Opaque types via `#[derive(Opaque)]` |
| **Extensions** | ✅ | CEL language extensions and custom operators |
| **Macros** | ✅ | CEL macro expansion support |
| **Async Support** | ✅ | Async function calls and evaluation |
| **Function Overloads** | ✅ | Multiple function signatures with automatic resolution |
| **Type Checking** | ✅ | Compile-time type validation |

### Planned Features

| Feature | Status | Description |
|---------|--------|-------------|
| **Protocol Buffer Integration** | 🚧 Planned | Direct support for protobuf messages and enums as native CEL types |

## Prerequisites

### System Requirements

- **Rust**: 1.80+
- **C++ Toolchain**: C++17 compatible compiler
  - Linux: GCC 9+ or Clang 15+
  - macOS: Xcode 10+ or Clang 15+
  - Windows: MSVC 2022+

### Installation Verification

```bash
# Clone and test
git clone https://github.com/xjasonli/cel-cxx.git
cd cel-cxx
cargo test --all-targets

# Run examples
cargo run --example comprehensive
cargo run --example tokio --features="tokio"
```

## License

Licensed under the Apache License 2.0. See [LICENSE](https://github.com/xjasonli/cel-cxx/blob/master/LICENSE) for details.

## Acknowledgements

- [`google/cel-cpp`](https://github.com/google/cel-cpp) - The foundational C++ CEL implementation
- [`dtolnay/cxx`](https://github.com/dtolnay/cxx) - Safe and efficient Rust-C++ interop
- [`rmanoka/async-scoped`](https://github.com/rmanoka/async-scoped) - Scoped async execution for safe lifetime management
- The CEL community and other Rust CEL implementations for inspiration and ecosystem growth 
