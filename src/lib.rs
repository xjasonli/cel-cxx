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
//! 1. **Dual Function Architecture**: Separates compile-time type checking from runtime execution
//! 2. **Smart Reference Management**: Automatic lifetime erasure for safe borrowing patterns
//! 3. **Zero-Cost Abstractions**: No runtime overhead for type conversions and function calls
//! 4. **Memory Safety**: Rust's ownership system prevents common CEL integration bugs
//! 
//! ### System Components
//! 
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    CEL-CXX Architecture                     │
//! ├─────────────────────────────────────────────────────────────┤
//! │  Rust Application Layer                                     │
//! │  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
//! │  │ Environment │ │ Activation  │ │ Custom Types        │   │
//! │  │ Builder     │ │ & Variables │ │ (#[derive(Opaque)]) │   │
//! │  └─────────────┘ └─────────────┘ └─────────────────────┘   │
//! ├─────────────────────────────────────────────────────────────┤
//! │  Type System & Function Registry                            │
//! │  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
//! │  │ Value       │ │ Function    │ │ Variable            │   │
//! │  │ Conversions │ │ Overloads   │ │ Bindings            │   │
//! │  └─────────────┘ └─────────────┘ └─────────────────────┘   │
//! ├─────────────────────────────────────────────────────────────┤
//! │  Zero-Cost FFI Layer (cxx)                                 │
//! │  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
//! │  │ Compiler    │ │ Runtime     │ │ Type Checker        │   │
//! │  │ Bindings    │ │ Evaluation  │ │ Integration         │   │
//! │  └─────────────┘ └─────────────┘ └─────────────────────┘   │
//! ├─────────────────────────────────────────────────────────────┤
//! │  Google CEL-CPP (C++)                                      │
//! │  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
//! │  │ Parser      │ │ Type System │ │ Expression          │   │
//! │  │ & Compiler  │ │ & Runtime   │ │ Evaluator           │   │
//! │  └─────────────┘ └─────────────┘ └─────────────────────┘   │
//! └─────────────────────────────────────────────────────────────┘
//! ```
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
//! ## 🔍 Error Handling
//!
//! Comprehensive error handling with detailed error information:
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! // Compilation errors
//! match Env::builder().build()?.compile("invalid syntax!") {
//!     Ok(program) => println!("Compiled successfully"),
//!     Err(e) => eprintln!("Compilation error: {}", e),
//! }
//! 
//! // Runtime errors
//! match program.evaluate(&activation) {
//!     Ok(result) => println!("Result: {}", result),
//!     Err(e) => eprintln!("Runtime error: {}", e),
//! }
//! # Ok::<(), cel_cxx::Error>(())
//! ```
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
//! ## 🔗 Related Crates
//! 
//! - [`cel-cxx-macros`]: Derive macros for custom types (re-exported when `derive` feature is enabled)
//! - [`cxx`]: Safe interop between Rust and C++ (used internally)
//! 
//! [`cel-cxx-macros`]: https://docs.rs/cel-cxx-macros
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

/// Function registration and implementation utilities.
///
/// The function system provides a flexible way to register and call functions in CEL expressions.
/// Functions can be either compile-time declarations (type signatures only) or runtime
/// implementations (callable code).
///
/// # Key Components
///
/// - [`FunctionRegistry`](function::FunctionRegistry): Compile-time function registry for declaring function signatures and registering implementations
/// - [`FunctionBindings`](function::FunctionBindings): Runtime function bindings for calling functions during evaluation
/// - [`FunctionOverloads`](function::FunctionOverloads): Function overload management supporting multiple implementations with different signatures
/// - **Declarations**: Use [`FunctionDecl`](function::FunctionDecl) trait for compile-time type checking
/// - **Implementations**: Use [`IntoFunction`](function::IntoFunction) trait for runtime function calls
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::*;
///
/// // Register a function implementation
/// let mut env = Env::builder()
///     .register_global_function("greet", |name: String| -> String {
///         format!("Hello, {}!", name)
///     })?
///     .build()?;
/// # Ok::<(), cel_cxx::Error>(())
/// ```
pub mod function;
pub use function::*;

/// Variable declaration and binding utilities.
///
/// The variable system supports both compile-time variable declarations and runtime
/// variable bindings. Variables can be constants, runtime values, or dynamic providers
/// that compute values on demand.
///
/// # Key Components
///
/// - [`VariableRegistry`](variable::VariableRegistry): Compile-time variable registry for declaring variable types and defining constants
/// - [`VariableBindings`](variable::VariableBindings): Runtime variable bindings for providing variable values during evaluation
/// - **Dynamic providers**: Functions that compute variable values at runtime
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::*;
///
/// // Declare variables and create bindings
/// let mut env = Env::builder()
///     .declare_variable::<String>("user")?
///     .declare_variable::<i64>("age")?
///     .build()?;
///
/// let mut activation = Activation::new()
///     .bind_variable("user", "Alice".to_string())?
///     .bind_variable("age", 30i64)?;
/// # Ok::<(), cel_cxx::Error>(())
/// ```
pub mod variable;
pub use variable::*;

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
/// - [`Opaque`](values::Opaque): Opaque value type for storing externally-defined custom types
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

/// Conditional future types for async/sync compatibility.
///
/// This module provides the [`MaybeFuture`] type, which has **different definitions**
/// depending on whether the `async` feature is enabled. This allows the same API to work
/// seamlessly in both synchronous and asynchronous contexts.
///
/// # ⚠️ Feature-Dependent Type Definition
///
/// **Critical**: [`MaybeFuture`] is implemented differently based on feature flags:
///
/// | Feature State | Type Definition | Behavior |
/// |---------------|-----------------|----------|
/// | **No `async`** | `type MaybeFuture<'a, T, E> = Result<T, E>` | Simple type alias, zero overhead |
/// | **With `async`** | `enum MaybeFuture<'a, T, E> { Result(..), Future(..) }` | Can hold immediate results or futures |
///
/// # Documentation Generation
///
/// When building documentation with `--all-features`, both variants are documented:
/// - The actual definition shown depends on which features are enabled during doc generation
/// - The [`maybe_future::doc_examples`] module shows both variants for reference
/// - Each definition is marked with appropriate `#[cfg(...)]` attributes
///
/// # Usage Guidelines
///
/// ## For Library Authors
///
/// When returning [`MaybeFuture`] from your APIs:
/// ```rust,no_run
/// use cel_cxx::MaybeFuture;
///
/// // This signature works regardless of async feature
/// fn your_function() -> MaybeFuture<'_, YourType, YourError> {
///     // Implementation varies by feature
///     # unimplemented!()
/// }
/// ```
///
/// ## For Library Users
///
/// When consuming [`MaybeFuture`] values:
/// ```rust,no_run
/// # use cel_cxx::MaybeFuture;
/// # fn example_usage(maybe_future: MaybeFuture<'_, i32>) {
/// // Sync mode (no async feature)
/// #[cfg(not(feature = "async"))]
/// {
///     let result = maybe_future?; // It's just Result<T, E>
/// }
///
/// // Async mode (with async feature) 
/// #[cfg(feature = "async")]
/// {
///     match maybe_future {
///         MaybeFuture::Result(result) => {
///             let value = result?; // Immediate result
///         }
///         MaybeFuture::Future(future) => {
///             let result = future.await?; // Await the future
///         }
///     }
/// }
/// # Ok::<(), Error>(())
/// # }
/// ```
///
/// # Examples
///
/// ## Synchronous mode (without `async` feature)
///
/// ```rust
/// # #[cfg(not(feature = "async"))]
/// # {
/// use cel_cxx::MaybeFuture;
///
/// // MaybeFuture is just Result<T, E> in sync mode
/// let result: MaybeFuture<'_, i32> = Ok(42);
/// assert_eq!(result.unwrap(), 42);
/// # }
/// ```
///
/// ## Asynchronous mode (with `async` feature)
///
/// ```rust
/// # #[cfg(feature = "async")]
/// # async fn example() {
/// use cel_cxx::MaybeFuture;
///
/// // Can hold either immediate results or futures
/// let immediate: MaybeFuture<'_, i32, &str> = MaybeFuture::Result(Ok(42));
/// let future_result: MaybeFuture<'_, i32, &str> = MaybeFuture::Future(
///     Box::pin(async { Ok(100) })
/// );
///
/// assert!(immediate.is_result());
/// assert!(future_result.is_future());
/// # }
/// ```
pub mod maybe_future;
pub use maybe_future::*;
