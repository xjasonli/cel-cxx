//! Variable declaration and binding utilities.
//!
//! The variable system supports both compile-time variable declarations and runtime
//! variable bindings. Variables can be constants, runtime values, or dynamic providers
//! that compute values on demand.
//!
//! # Key Components
//!
//! - [`VariableRegistry`]: Compile-time variable registry for declaring variable types and defining constants
//! - [`VariableBindings`]: Runtime variable bindings for providing variable values during evaluation
//! - **Dynamic providers**: Functions that compute variable values at runtime
//!
//! # Examples
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! // Declare variables and create bindings
//! let mut env = Env::builder()
//!     .declare_variable::<String>("user")?
//!     .declare_variable::<i64>("age")?
//!     .build()?;
//!
//! let mut activation = Activation::new()
//!     .bind_variable("user", "Alice".to_string())?
//!     .bind_variable("age", 30i64)?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! # Detailed Documentation
//!
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
//! registry.declare::<i64>("user_id")?;
//! registry.declare::<String>("user_name")?;
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
//! #[derive(Opaque, Debug, Clone, PartialEq)]
//! struct User {
//!     id: i64,
//!     name: String,
//! }
//! 
//! impl std::fmt::Display for User {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "User({})", self.name)
//!     }
//! }
//!
//! let mut registry = VariableRegistry::new();
//! registry.declare::<User>("current_user")?;
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
//! registry.declare::<Optional<String>>("optional_value")?;
//!
//! let mut bindings = VariableBindings::new();
//! bindings.bind("optional_value", Optional::new("Hello"))?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```

mod registry;
pub use registry::*;

mod bindings;
pub use bindings::*;
