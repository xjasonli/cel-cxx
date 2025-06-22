//! Variable management for CEL expressions.
//!
//! This module provides functionality for managing variables and their bindings
//! in CEL expression evaluation contexts. Variables can be bound to values
//! and accessed during expression evaluation.
//!
//! # Key Components
//!
//! - **Variable Registry**: Central registry for managing variable declarations and bindings
//! - **Variable Bindings**: Runtime value bindings for variables during evaluation
//! - **Type Safety**: Compile-time and runtime type checking for variable access
//!
//! # Variable Types
//!
//! Variables in CEL can hold any value type that implements the required traits:
//! - **Primitive types**: `int`, `uint`, `double`, `bool`, `string`, `bytes`
//! - **Complex types**: `list`, `map`, `struct`, `type`
//! - **Custom types**: User-defined types via `#[derive(Opaque)]`
//! - **Optional types**: Nullable values using `Optional<T>`
//!
//! # Examples
//!
//! ## Basic variable binding
//!
//! ```rust,no_run
//! use cel_cxx::{VariableRegistry, VariableBindings, Value};
//!
//! // Create registry and declare variables
//! let mut registry = VariableRegistry::new();
//! registry.declare("user_id", i64::value_type())?;
//! registry.declare("user_name", String::value_type())?;
//!
//! // Create bindings with actual values
//! let mut bindings = VariableBindings::new();
//! bindings.bind("user_id", 12345i64)?;
//! bindings.bind("user_name", "Alice".to_string())?;
//!
//! // Variables are now available for CEL expression evaluation
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! ## Working with custom types
//!
//! ```rust,no_run
//! use cel_cxx::{VariableRegistry, VariableBindings, Opaque};
//!
//! #[derive(Opaque)]
//! struct User {
//!     id: i64,
//!     name: String,
//! }
//!
//! let mut registry = VariableRegistry::new();
//! registry.declare("current_user", User::value_type())?;
//!
//! let mut bindings = VariableBindings::new();
//! let user = User { id: 123, name: "Bob".to_string() };
//! bindings.bind("current_user", user)?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! ## Optional variables
//!
//! ```rust,no_run
//! use cel_cxx::{VariableRegistry, VariableBindings, Optional};
//!
//! let mut registry = VariableRegistry::new();
//! registry.declare("optional_value", Optional::<String>::value_type())?;
//!
//! let mut bindings = VariableBindings::new();
//! bindings.bind("optional_value", Optional::some("Hello".to_string()))?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```

/// Variable registry for managing variable declarations.
///
/// This module provides the [`VariableRegistry`] type for declaring variables
/// and their types in CEL environments. The registry tracks variable names
/// and their expected types for compile-time validation.
///
/// # Features
///
/// - **Type declarations**: Associate variable names with CEL types
/// - **Validation**: Ensure variable bindings match declared types
/// - **Lookup**: Efficient variable type resolution during compilation
/// - **Iteration**: Enumerate all declared variables
mod registry;
pub use registry::*;

/// Variable bindings for runtime value storage.
///
/// This module provides the [`VariableBindings`] type for binding actual
/// values to declared variables during CEL expression evaluation.
///
/// # Features
///
/// - **Value binding**: Associate variable names with runtime values
/// - **Type checking**: Validate that bound values match declared types
/// - **Efficient lookup**: Fast variable resolution during evaluation
/// - **Lifetime management**: Proper handling of borrowed values
mod bindings;
pub use bindings::*;
