//! [![github]](https://github.com/xjasonli/cel-cxx)
//! [![crates-io]](https://crates.io/crates/cel-cxx)
//! [![docs-rs]](https://docs.rs/cel-cxx)
//! 
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//! 
//! # cel-cxx: Modern Rust Interface for CEL
//! 
//! A high-performance, type-safe Rust interface for [Common Expression Language (CEL)](https://github.com/google/cel-spec),
//! built on top of [google/cel-cpp](https://github.com/google/cel-cpp) with zero-cost FFI bindings via [cxx](https://github.com/dtolnay/cxx).
//!
//! ## 🎯 Key Innovation: Zero-Annotation Function System
//! 
//! The library's flagship feature uses **Generic Associated Types (GATs)** to automatically infer function signatures,
//! eliminating the need for manual type annotations:
//! 
//! ```rust,no_run
//! use cel_cxx::*;
//! 
//! let env = Env::builder()
//!     // ✨ Function signatures automatically inferred from Rust types!
//!     .register_global_function("add", |a: i64, b: i64| a + b)?
//!     .register_global_function("concat", |a: String, b: &str| a + b)?
//!     .register_global_function("length", |s: &str| s.len() as i64)?
//!     .register_global_function("parse", |s: &str| s.parse::<i64>())?  // Result<i64, _> auto-handled
//!     .build()?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//! 
//! ## 🏗️ Architecture Overview
//! 
//! ### Core Design Principles
//! 
//! - **Type Safety**: Compile-time verification of CEL expressions and function signatures
//! - **Zero-Cost Abstractions**: Direct FFI calls to CEL-CPP with minimal overhead
//! - **Memory Safety**: Rust ownership system prevents common integration bugs
//! - **Ergonomic API**: Builder patterns and automatic type inference reduce boilerplate
//! - **Extensibility**: Support for custom types and async operations
//! 
//! ### Integration Architecture
//! 
//! The library provides a layered architecture that bridges Rust and CEL-CPP:
//! 
//! - **Application Layer**: High-level APIs for environment building and expression evaluation
//! - **Type System Layer**: Automatic conversions between Rust and CEL types
//! - **FFI Layer**: Zero-cost bindings to CEL-CPP via the `cxx` crate
//! - **CEL-CPP Layer**: Google's reference implementation for parsing and evaluation
//! 
//! ## 🚀 Quick Start
//!
//! ### Basic Expression Evaluation
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! // 1. Build environment with variables and functions
//! let env = Env::builder()
//!     .declare_variable::<String>("name")?
//!     .declare_variable::<i64>("age")?
//!     .register_global_function("adult", |age: i64| age >= 18)?
//!     .build()?;
//!
//! // 2. Compile expression
//! let program = env.compile("'Hello ' + name + '! You are ' + (adult(age) ? 'an adult' : 'a minor')")?;
//!
//! // 3. Create activation with variable bindings
//! let activation = Activation::new()
//!     .bind_variable("name", "Alice")?
//!     .bind_variable("age", 25i64)?;
//!
//! // 4. Evaluate
//! let result = program.evaluate(&activation)?;
//! println!("{}", result); // "Hello Alice! You are an adult"
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! ### Custom Types with Derive Macros
//!
//! ```rust,no_run
//! use cel_cxx::*;
//! 
//! #[derive(Opaque, Debug, Clone, PartialEq)]
//! #[cel_cxx(type = "myapp.User")]
//! struct User {
//!     name: String,
//!     age: i32,
//!     roles: Vec<String>,
//! }
//! 
//! impl User {
//!     // Struct methods can be registered directly as CEL member functions
//!     fn has_role(&self, role: &str) -> bool {
//!         self.roles.contains(&role.to_string())
//!     }
//!     
//!     fn is_adult(&self) -> bool {
//!         self.age >= 18
//!     }
//!     
//!     fn get_role_count(&self) -> i64 {
//!         self.roles.len() as i64
//!     }
//! }
//! 
//! impl std::fmt::Display for User {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "User({})", self.name)
//!     }
//! }
//! 
//! let env = Env::builder()
//!     .declare_variable::<User>("user")?
//!     // ✨ Register struct methods directly - &self becomes CEL receiver
//!     .register_member_function("has_role", User::has_role)?
//!     .register_member_function("is_adult", User::is_adult)?
//!     .register_member_function("get_role_count", User::get_role_count)?
//!     .build()?;
//! 
//! let program = env.compile("user.has_role('admin') && user.is_adult()")?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! ## 🔧 Advanced Features
//! 
//! ### Async Support
//!
//! When the `async` feature is enabled, you can evaluate expressions asynchronously:
//!
//! ```rust,no_run
//! # #[cfg(feature = "async")]
//! # async fn example() -> Result<(), cel_cxx::Error> {
//! use cel_cxx::*;
//!
//! let env = Env::builder()
//!     .use_tokio()
//!     .register_global_function("async_fetch", async |id: i64| -> Result<String, Error> {
//!         // Simulate async database call
//!         tokio::time::sleep(std::time::Duration::from_millis(100)).await;
//!         Ok(format!("Data for ID: {}", id))
//!     })?
//!     .build()?;
//!
//! let program = env.compile("async_fetch(42)")?;
//! let result = program.evaluate(()).await?;
//! # Ok(())
//! # }
//! ```
//! 
//! #### Async Architecture Design
//! 
//! Supporting Rust async functions in CEL presents unique challenges since CEL-CPP doesn't
//! natively support asynchronous or callback-based user-defined functions and variable providers.
//! When a Rust async function returns a `Future`, it has already exited the current stack frame,
//! and the C++ CEL evaluation engine cannot schedule or await Rust futures.
//! 
//! **cel-cxx** solves this through an innovative dual-threading architecture:
//! 
//! 1. **Async-to-Blocking Bridge**: When async functions or variable providers are registered,
//!    the entire program evaluation is moved to a blocking thread using `Runtime::spawn_blocking()`.
//!    The main async context receives a future that resolves when evaluation completes.
//! 
//! 2. **Blocking-to-Async Bridge**: When async callbacks are invoked within the blocking thread,
//!    the returned futures are dispatched back to the async runtime for execution, while the
//!    blocking thread waits for completion using `Runtime::block_on()`.
//! 
//! #### Implementation Details
//! 
//! - **Lifetime Management**: Since user-provided functions and variable providers can be capturing
//!   closures with complex lifetimes, cel-cxx uses the [`async-scoped`](https://crates.io/crates/async-scoped) 
//!   crate to safely manage these lifetimes across thread boundaries.
//! 
//! - **Multi-threaded Runtime Requirement**: When using Tokio, the runtime must be multi-threaded
//!   because the implementation relies on [`tokio::task::block_in_place()`](https://docs.rs/tokio/latest/tokio/task/fn.block_in_place.html),
//!   which panics in single-threaded runtimes.
//! 
//! This design enables seamless integration of async Rust code with the synchronous CEL-CPP
//! evaluation engine, maintaining both performance and correctness across runtime boundaries.
//! 
//! ### Function Overloads
//! 
//! The library supports function overloading with automatic type resolution:
//! 
//! ```rust,no_run
//! use cel_cxx::*;
//! 
//! let env = Env::builder()
//!     // Multiple functions with same name, different signatures
//!     .register_global_function("process", |x: i64| x * 2)?
//!     .register_global_function("process", |x: f64| x * 2.0)?
//!     .register_global_function("process", |x: String| x.to_uppercase())?
//!     .build()?;
//! 
//! // CEL will automatically choose the right overload based on argument types
//! let program1 = env.compile("process(42)")?;      // Calls i64 version
//! let program2 = env.compile("process(3.14)")?;    // Calls f64 version  
//! let program3 = env.compile("process('hello')")?; // Calls String version
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//! 
//! ### Smart Reference Handling
//! 
//! The library automatically manages reference types with safe lifetime handling:
//! 
//! ```rust,no_run
//! use cel_cxx::*;
//! use std::collections::HashMap;
//! 
//! // ✅ These reference patterns work automatically:
//! let env = Env::builder()
//!     .declare_variable::<Vec<&str>>("string_refs")?        // Borrowed strings
//!     .declare_variable::<HashMap<i64, &str>>("lookup")?    // Borrowed values
//!     .declare_variable::<Option<&str>>("maybe_str")?       // Optional borrows
//!     .build()?;
//! 
//! // The library prevents unsafe patterns at compile time:
//! // ❌ .declare_variable::<&Vec<String>>("invalid")?  // Compiler error
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//! 
//! ## 📊 Type System
//!
//! The crate provides comprehensive type support with automatic conversions between CEL and Rust types.
//! All types support the three core traits for seamless integration:
//!
//! | CEL Type | | Rust Type | | |
//! |----------|---|-----------|---|---|
//! | | | **Declare** | **To CEL** | **From CEL** |
//! | | | `TypedValue` | `IntoValue` | `FromValue` |
//! | `null` | | `()` | ✅ | ✅ | ✅ |
//! | `bool` | | `bool` | ✅ | ✅ | ✅ |
//! | `int` | | `i64`, `i32`, `i16`, `isize` | ✅ | ✅ | ✅ |
//! | `uint` | | `u64`, `u32`, `u16`, `usize` | ✅ | ✅ | ✅ |
//! | `double` | | `f64`, `f32` | ✅ | ✅ | ✅ |
//! | `string` | | `String`, `ArcStr`, `Box<str>`, `str` | ✅ | ✅ | ✅ |
//! | `bytes` | | `Vec<u8>`, `ArcBytes`, `Box<[u8]>`, `[u8]` | ✅ | ✅ | ✅ |
//! | `duration` | | `chrono::Duration` | ✅ | ✅ | ✅ |
//! | `timestamp` | | `chrono::DateTime<Utc>`, `SystemTime` | ✅ | ✅ | ✅ |
//! | `list<T>` | | `Vec<T>`, `VecDeque<T>`, `LinkedList<T>`, `[T]` | ✅ | ✅ | ✅ |
//! | `map<K,V>` | | `HashMap<K,V>`, `BTreeMap<K,V>`, `Vec<(K,V)>` | ✅ | ✅ | ✅ |
//! | `optional<T>` | | `Option<T>`, `Optional<T>` | ✅ | ✅ | ✅ |
//! | `type` | | `ValueType` | ✅ | ✅ | ✅ |
//! | `error` | | `Error` | ✅ | ✅ | ✅ |
//! | `opaque` | | `#[derive(Opaque)] struct` | ✅ | ✅ | ✅ |
//!
//! **Special Reference Support**: All `&T` types support **Declare** and **To CEL** operations,
//! enabling zero-copy function arguments like `&str`, `&[u8]`, `&MyStruct`, etc.
//!
//! ### Type Conversion Examples
//!
//! ```rust,no_run
//! use cel_cxx::*;
//! use std::collections::VecDeque;
//!
//! // Automatic conversions work seamlessly
//! let env = Env::builder()
//!     // Different integer types all map to CEL int
//!     .register_global_function("process_i32", |x: i32| x * 2)?
//!     .register_global_function("process_i64", |x: i64| x * 2)?
//!     
//!     // String types are interchangeable
//!     .register_global_function("process_string", |s: String| s.to_uppercase())?
//!     .register_global_function("process_str", |s: &str| s.len() as i64)?
//!     
//!     // Container types work with any compatible Rust collection
//!     .register_global_function("sum_vec", |nums: Vec<i64>| nums.iter().sum::<i64>())?
//!     .register_global_function("sum_deque", |nums: VecDeque<i64>| nums.iter().sum::<i64>())?
//!     .build()?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//! 
//! ## 🛠️ Feature Flags
//!
//! | Feature | Description | Default |
//! |---------|-------------|---------|
//! | `async` | Async/await support for expressions and functions | ❌ |
//! | `derive` | Derive macros for custom types (`#[derive(Opaque)]`) | ✅ |
//! | `tokio` | Tokio async runtime integration (requires `async`) | ❌ |
//! | `async-std` | async-std runtime integration (requires `async`) | ❌ |
//! 
//! ## 🎯 Performance Characteristics
//! 
//! - **Zero-cost FFI**: Direct C++ function calls with no marshaling overhead
//! - **Compile-time optimization**: Function signatures resolved at compile time  
//! - **Memory efficient**: Minimal allocations through smart reference handling
//! - **Async overhead**: Only when async features are explicitly used
//! - **Type safety**: Compile-time prevention of common integration errors
//! 
//! ## 📚 Examples
//! 
//! The crate includes comprehensive examples demonstrating various features:
//! 
//! - **Basic Usage**: Variable binding, function registration, expression evaluation
//! - **Custom Types**: Derive macros, member functions, type integration  
//! - **Async Support**: Tokio and async-std integration examples
//! - **Advanced Features**: Function overloads, error handling, complex type conversions
//! 
//! Run examples with:
//! ```bash
//! cargo run --example comprehensive
//! cargo run --example tokio --features="async,tokio"
//! ```
//! 
//! ## 🖥️ Platform Support
//! 
//! | Platform | Status | Notes |
//! |----------|--------|-------|
//! | **Linux** | ✅ Supported | Fully tested and supported |
//! | **macOS** | ⚠️ Untested | Should work but not regularly tested |
//! | **Windows** | ❌ Not Supported | CEL-CPP Bazel build scripts don't support Windows |
//! 
//! ## 📋 CEL Feature Support
//! 
//! ### ✅ Supported Features
//! 
//! | Feature | Status | Description |
//! |---------|--------|-------------|
//! | **Basic Types** | ✅ | `null`, `bool`, `int`, `uint`, `double`, `string`, `bytes` |
//! | **Collections** | ✅ | `list<T>`, `map<K,V>` with full indexing and comprehensions |
//! | **Time Types** | ✅ | `duration`, `timestamp` with full arithmetic support |
//! | **Operators** | ✅ | Arithmetic, logical, comparison, and membership operators |
//! | **Functions** | ✅ | Built-in functions and custom function registration |
//! | **Variables** | ✅ | Variable binding and scoping |
//! | **Conditionals** | ✅ | Ternary operator and logical short-circuiting |
//! | **Comprehensions** | ✅ | List and map comprehensions with filtering |
//! | **Optional Types** | ✅ | `optional<T>` with safe navigation |
//! | **Custom Types** | ✅ | Opaque types via `#[derive(Opaque)]` |
//! | **Extensions** | ✅ | CEL language extensions and custom operators |
//! | **Macros** | ✅ | CEL macro expansion support |
//! | **Async Support** | ✅ | Async function calls and evaluation |
//! | **Function Overloads** | ✅ | Multiple function signatures with automatic resolution |
//! | **Type Checking** | ✅ | Compile-time type validation |
//! 
//! ### 🚧 Planned Features
//! 
//! | Feature | Status | Description |
//! |---------|--------|-------------|
//! | **Protocol Buffer Integration** | 🚧 Planned | Direct support for protobuf messages and enums as native CEL types |
//! | **Windows Support** | 🚧 Planned | Requires CEL-CPP Windows build support |
//! 
//! ## 🔗 Related Crates
//! 
//! - [`async-scoped`]: Scoped async execution for safe lifetime management across thread boundaries
//! - [`cxx`]: Safe interop between Rust and C++ (used internally)
//! 
//! [`async-scoped`]: https://crates.io/crates/async-scoped
//! [`cxx`]: https://docs.rs/cxx

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    missing_docs,
    unreachable_pub,
)]


#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use cel_cxx_macros::*;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod r#async;
#[cfg(feature = "tokio")]
pub use r#async::Tokio;
#[cfg(feature = "async-std")]
pub use r#async::AsyncStd;

/// Marker types and traits for function and runtime polymorphism.
pub mod marker;
pub use marker::*;

/// Environment for compiling CEL expressions.
pub mod env;
pub use env::*;

/// Compiled CEL programs ready for evaluation.
pub mod program;
pub use program::*;

mod ffi;

/// Error types and error handling utilities.
pub mod error;
pub use error::*;

pub mod function;
pub use function::*;

pub mod variable;
pub use variable::*;

pub mod activation;
pub use activation::*;

pub mod kind;
pub use kind::*;

pub mod types;
pub use types::*;

pub mod values;
pub use values::*;

pub mod maybe_future;
pub use maybe_future::*;
