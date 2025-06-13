//! [![github]](https://github.com/xjasonli/cel-cxx)
//! [![crates-io]](https://crates.io/crates/cel-cxx)
//! [![docs-rs]](https://docs.rs/cel-cxx)
//! 
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//! 
//! A Rust-native interface for [Common Expression Language (CEL)](https://github.com/google/cel-spec),
//! implemented as a high-level idiomatic Rust wrapper around [google/cel-cpp](https://github.com/google/cel-cpp)
//! via the [cxx](https://github.com/dtolnay/cxx) crate.
//!
//! This crate provides a modern, ergonomic Rust API for CEL that leverages Rust's type system
//! and language features to offer a safe, efficient, and developer-friendly experience. It
//! maintains full compatibility with the CEL specification while providing idiomatic Rust
//! abstractions and zero-cost FFI bindings.
//!
//! ## Key Features
//!
//! - **Idiomatic Rust API**: Type-safe interfaces with Rust's type system and error handling
//! - **Zero-cost FFI**: Efficient C++ interop through the cxx crate
//! - **Async Support**: First-class async/await support with multiple runtime options
//! - **Rich Type System**: Full support for CEL's type system with seamless Rust type conversions
//! - **Custom Types**: Derive macros for creating custom CEL types from Rust structs
//!
//! ## Quick Start
//!
//! ### Basic Usage
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! // Create an environment and compile an expression
//! let program = Env::builder()
//!     .declare_variable::<String>("name")?
//!     .build()?
//!     .compile("'Hello, ' + name + '!'")?
//!
//! // Create an activation with variable bindings
//! let activation = Activation::new()
//!     .bind_variable("name", "World")?;
//!
//! // Evaluate the expression
//! let result = program.evaluate(activation)?;
//! println!("{}", result); // "Hello, World!"
//! ```
//!
//! ### With Custom Functions
//!
//! ```rust,no_run
//! use cel_cxx::*;
//! use std::convert::Infallible;
//!
//! // Register a custom function with type-safe Rust closures
//! let program = Env::builder()
//!     .register_global_function("double", |x: i64| -> Result<i64, Infallible> { Ok(x * 2) })?
//!     .build()?
//!     .compile("double(21)")?;
//!
//! let result = program.evaluate(())?;
//! // result == Value::Int(42)
//! ```
//!
//! ## Feature Flags
//!
//! - **`async`**: Enables asynchronous evaluation of CEL expressions with support for
//!   [async-std](https://github.com/async-rs/async-std) or
//!   [tokio](https://github.com/tokio-rs/tokio) runtimes.
//! - **`derive`**: Enables derive macros for custom types:
//!   - `#[derive(Opaque)]` for creating custom CEL types from Rust structs
//!   - `#[cel_cxx(...)]` attributes for fine-grained type control
//! - **`tokio`**: Enables Tokio async runtime support (requires `async` feature)
//! - **`async-std`**: Enables async-std runtime support (requires `async` feature)
//!
//! ## Async Support
//!
//! When the `async` feature is enabled, you can evaluate expressions asynchronously
//! with full async/await support:
//!
//! ```rust,no_run
//! # #[cfg(feature = "async")]
//! # async fn example() {
//! use std::convert::Infallible;
//! use cel_cxx::*;
//!
//! let program = Env::builder()
//!     .use_tokio()
//!     .register_global_function("async_square", async |x: i64| -> Result<i64, Infallible> {
//!         tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
//!         x * x
//!     })?
//!     .build()?
//!     .compile("async_square(10)")?;
//!
//! let result = program.evaluate(()).await?;
//! // result == Value::Int(100)
//! # }
//! ```
//!
//! ## Type System
//!
//! The crate provides a rich type system that seamlessly integrates with Rust's type system:
//!
//! - **Primitive Types**: `bool`, `int`, `uint`, `double`, `string`, `bytes`
//! - **Complex Types**: `list`, `map`, `struct` with full generic support
//! - **Protocol Buffer Types**: `Duration`, `Timestamp`, and wrapper types
//! - **Custom Types**: Create custom CEL types from Rust structs using derive macros
//!
//! ## Error Handling
//!
//! Comprehensive error handling with Rust's `Result` type and custom error types:
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! match Env::builder().compile("invalid syntax!") {
//!     Ok(program) => println!("Compiled successfully"),
//!     Err(e) => eprintln!("Compilation error: {}", e),
//! }
//! ```

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

/// Function registration and implementation utilities.
///
/// The function system provides a flexible way to register and call functions in CEL expressions.
/// Functions can be either compile-time declarations (type signatures only) or runtime
/// implementations (callable code).
///
/// # Key Components
///
/// - [`function::FunctionRegistry`]: Compile-time function registry for declaring function signatures and registering implementations
/// - [`function::FunctionBindings`]: Runtime function bindings for calling functions during evaluation
/// - [`function::FunctionOverloads`]: Function overload management supporting multiple implementations with different signatures
/// - **Declarations**: Use [`function::FnDecl`] trait for compile-time type checking
/// - **Implementations**: Use [`function::FnImpl`] trait for runtime function calls
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::*;
///
/// // Register a function implementation
/// let mut env = Env::builder()
///     .register_global_function("greet", |name: String| -> Result<String, Error> {
///         Ok(format!("Hello, {}!", name))
///     })?
///     .build()?;
/// # Ok::<(), cel_cxx::Error>(())
/// ```
pub mod function;

/// Variable declaration and binding utilities.
///
/// The variable system supports both compile-time variable declarations and runtime
/// variable bindings. Variables can be constants, runtime values, or dynamic providers
/// that compute values on demand.
///
/// # Key Components
///
/// - [`variable::VariableRegistry`]: Compile-time variable registry for declaring variable types and defining constants
/// - [`variable::VariableBindings`]: Runtime variable bindings for providing variable values during evaluation
/// - [`variable::Provider`]: Dynamic variable providers supporting lazy evaluation of variable values
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::*;
///
/// // Declare variables and create bindings
/// let mut env = Env::builder()
///     .declare_variable("user", Type::String)?
///     .declare_variable("age", Type::Int)?
///     .build()?;
///
/// let mut activation = Activation::new();
/// activation.bind_variable("user", "Alice")?;
/// activation.bind_variable("age", 30i64)?;
/// # Ok::<(), cel_cxx::Error>(())
/// ```
pub mod variable;

/// Activation context for expression evaluation.
pub mod activation;
pub use activation::*;

/// CEL type system kinds and type classification.
pub mod kind;
pub use kind::*;

/// CEL type system types and type definitions.
pub mod types;
pub use types::*;

/// CEL value types and value operations.
///
/// This module provides the core value types used in CEL expressions, including
/// primitives, collections, and custom opaque types. All values implement
/// conversion traits for seamless integration with Rust types.
///
/// # Key Components
///
/// - [`Value`]: The main CEL value enum supporting all CEL types
/// - [`IntoValue`]: Trait for converting Rust types to CEL values
/// - [`FromValue`]: Trait for converting CEL values to Rust types
/// - [`values::Opaque`]: Opaque value type for storing externally-defined custom types
/// - [`OpaqueValue`]: Trait for implementing custom opaque value types
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::*;
///
/// // Convert Rust values to CEL values
/// let string_val = Value::from("hello");
/// let int_val = Value::from(42i64);
/// let list_val = Value::from(vec![1i64, 2i64, 3i64]);
///
/// // Convert back to Rust types
/// let rust_string: String = string_val.try_into()?;
/// let rust_int: i64 = int_val.try_into()?;
/// # Ok::<(), cel_cxx::Error>(())
/// ```
pub mod values;
pub use values::*;

mod maybe_future;
#[allow(unused)]
use maybe_future::*;

