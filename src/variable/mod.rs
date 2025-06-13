//! Variable system for CEL environments.
//!
//! This module provides functionality for declaring, defining, and binding variables in CEL environments.
//! The variable system consists of:
//!
//! - [`variable::VariableRegistry`]: Compile-time variable registry for declaring variable types and defining constants
//! - [`variable::VariableBindings`]: Runtime variable bindings for providing variable values during evaluation
//! - [`variable::Provider`]: Dynamic variable providers supporting lazy evaluation of variable values
//!
//! # Compile-time vs Runtime
//!
//! - **Compile-time**: Use [`variable::VariableRegistry`] to declare variable types when compiling CEL expressions
//! - **Runtime**: Use [`variable::VariableBindings`] to provide actual variable values during evaluation
//!
//! # Examples
//!
//! ```rust,no_run
//! use cel_cxx::{Env, Activation, Type};
//!
//! // Compile-time variable declaration
//! let env = Env::builder()
//!     .declare_variable("name", Type::String)?
//!     .declare_variable("age", Type::Int)?
//!     .build()?;
//!
//! // Runtime variable binding
//! let activation = Activation::new()
//!     .bind("name", "Alice")?
//!     .bind("age", 30i64)?;
//!
//! let program = env.compile("name + ' is ' + string(age)")?;
//! let result = program.eval(&activation)?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```

mod registry;
mod bindings;
mod provider;

pub use registry::*;
pub use bindings::*;
pub use provider::*;