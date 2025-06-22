//! Activation context for CEL expression evaluation.
//!
//! This module provides the [`Activation`] type and related traits for managing
//! variable and function bindings during CEL expression evaluation. Activations
//! serve as the runtime context that provides values for variables and functions
//! declared in the CEL environment.
//!
//! # Key Concepts
//!
//! ## Activation Interface
//!
//! The [`ActivationInterface`] trait defines the contract for providing variable
//! and function bindings to the CEL evaluator. It provides access to:
//!
//! - **Variable bindings**: Map variable names to runtime values
//! - **Function bindings**: Provide runtime function implementations
//!
//! ## Activation Types
//!
//! The module provides several activation types:
//!
//! - **`Activation<'f>`**: Standard activation for synchronous evaluation
//! - **`AsyncActivation<'f>`**: Activation with async function support
//! - **`()`**: Empty activation for expressions without variables/functions
//!
//! # Variable Binding
//!
//! Variables can be bound to values that match the types declared in the environment:
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! let activation = Activation::new()
//!     .bind_variable("user_name", "Alice".to_string())?
//!     .bind_variable("user_age", 30i64)?
//!     .bind_variable("is_admin", true)?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! # Function Binding
//!
//! Functions can be bound at runtime to override or supplement environment functions:
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! let activation = Activation::new()
//!     .bind_global_function("custom_add", |a: i64, b: i64| a + b)?
//!     .bind_member_function("to_upper", |s: String| s.to_uppercase())?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! # Variable Providers
//!
//! For dynamic or computed values, you can bind variable providers:
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! let activation = Activation::new()
//!     .bind_variable_provider("current_time", || {
//!         std::time::SystemTime::now()
//!             .duration_since(std::time::UNIX_EPOCH)
//!             .unwrap()
//!             .as_secs() as i64
//!     })?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! # Empty Activations
//!
//! For expressions that don't require any bindings, you can use the unit type:
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! let env = Env::builder().build()?;
//! let program = env.compile("1 + 2 * 3")?;
//! let result = program.evaluate(())?; // No activation needed
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! # Async Support
//!
//! When the `async` feature is enabled, activations can contain async functions:
//!
//! ```rust,no_run
//! # #[cfg(feature = "async")]
//! # async fn example() -> Result<(), cel_cxx::Error> {
//! use cel_cxx::*;
//!
//! let activation = AsyncActivation::new_async()
//!     .bind_global_function("fetch_data", |url: String| async move {
//!         // Simulate async work
//!         format!("Data from {}", url)
//!     })?;
//! # Ok(())
//! # }
//! ```

use std::marker::PhantomData;
use crate::function::FunctionBindings;
use crate::Error;
use crate::variable::VariableBindings;
use crate::values::{IntoValue, TypedValue};
use crate::function::{IntoFunction, Arguments};
use crate::marker::*;

/// Interface for providing variable and function bindings during evaluation.
/// 
/// This trait defines the interface that activation types must implement
/// to provide variable and function bindings to the CEL evaluator.
/// 
/// # Type Parameters
/// 
/// * `'f` - Lifetime of the functions in the bindings
/// * `Fm` - Function marker type indicating sync/async function support
pub trait ActivationInterface<'f, Fm: FnMarker = ()> {
    /// Returns a reference to the variable bindings.
    /// 
    /// Variable bindings map variable names to their values during evaluation.
    fn variables(&self) -> &VariableBindings<'f>;
    
    /// Returns a reference to the function bindings.
    /// 
    /// Function bindings provide runtime function implementations that can
    /// override or supplement the functions registered in the environment.
    fn functions(&self) -> &FunctionBindings<'f>;
}

/// Activation context for CEL expression evaluation.
/// 
/// An `Activation` provides variable and function bindings that are used
/// during the evaluation of a CEL expression. It allows you to bind runtime
/// values to variables and functions that were declared in the environment.
/// 
/// # Type Parameters
/// 
/// * `'f` - Lifetime of the functions in the activation
/// * `Fm` - Function marker type indicating sync/async function support
/// 
/// # Examples
/// 
/// ## Basic Variable Binding
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// let activation = Activation::new()
///     .bind_variable("name", "Alice")
///     .unwrap()
///     .bind_variable("age", 30i64)
///     .unwrap();
/// ```
/// 
/// ## Function Binding
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// let activation = Activation::new()
///     .bind_global_function("custom_fn", |x: i64| -> i64 { x * 2 })
///     .unwrap();
/// ```
/// 
/// ## Empty Activation
/// 
/// For expressions that don't need any bindings, you can use `()`:
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// let env = Env::builder().build().unwrap();
/// let program = env.compile("1 + 2").unwrap();
/// let result = program.evaluate(()).unwrap();
/// ```
#[derive(Debug)]
pub struct Activation<'f, Fm: FnMarker = ()> {
    variables: VariableBindings<'f>,
    functions: FunctionBindings<'f>,
    _fn_marker: PhantomData<Fm>,
}

/// Activation with async support.
/// 
/// This is a convenience type that can be used to create activations with
/// async support. It is equivalent to `Activation<'f, Async>`.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// let activation = AsyncActivation::new_async();
/// ```
#[cfg(feature = "async")]
pub type AsyncActivation<'f> = Activation<'f, Async>;


impl<'f> Activation<'f, ()> {
    /// Creates a new empty activation.
    /// 
    /// This creates an activation with no variable or function bindings.
    /// You can then use the builder methods to add bindings.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let activation = Activation::new();
    /// ```
    pub fn new() -> Self {
        Self {
            variables: VariableBindings::new(),
            functions: FunctionBindings::new(),
            _fn_marker: PhantomData,
        }
    }

    /// Force the activation to be async.
    /// 
    /// This method is only available when the `async` feature is enabled.
    /// It converts the activation to an `AsyncActivation`.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let activation = Activation::new().force_async();
    /// ```
    #[cfg(feature = "async")]
    pub fn force_async(self) -> Activation<'f, Async> {
        Activation {
            variables: self.variables,
            functions: self.functions,
            _fn_marker: PhantomData,
        }
    }
}

#[cfg(feature = "async")]
impl<'f> Activation<'f, Async> {
    /// Creates a new empty activation with async support.
    /// 
    /// This creates an activation with no variable or function bindings.
    /// You can then use the builder methods to add bindings.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let activation = Activation::new_async();
    /// ```
    pub fn new_async() -> Self {
        Self {
            variables: VariableBindings::new(),
            functions: FunctionBindings::new(),
            _fn_marker: PhantomData,
        }
    }
}

impl<'f, Fm: FnMarker> Activation<'f, Fm> {
    /// Binds a variable to a value.
    /// 
    /// This method adds a variable binding to the activation. The variable
    /// name must match a variable declared in the environment, and the value
    /// must be compatible with the declared type.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the variable to bind
    /// * `value` - The value to bind to the variable
    /// 
    /// # Type Parameters
    /// 
    /// * `S` - The type of the variable name (must convert to `String`)
    /// * `T` - The type of the value (must implement `IntoValue` and `TypedValue`)
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing the updated activation or an error if
    /// the binding failed.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let activation = Activation::new()
    ///     .bind_variable("name", "World")
    ///     .unwrap()
    ///     .bind_variable("count", 42i64)
    ///     .unwrap();
    /// ```
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The variable name is not declared in the environment
    /// - The value type doesn't match the declared variable type
    /// - The value cannot be converted to a CEL value
    pub fn bind_variable<S, T>(
        mut self, name: S, value: T
    ) -> Result<Self, Error>
    where
        S: Into<String>,
        T: IntoValue + TypedValue,
    {
        self.variables.bind(name, value)?;
        Ok(Activation {
            variables: self.variables,
            functions: self.functions,
            _fn_marker: PhantomData,
        })
    }

    /// Binds a variable to a value provider.
    /// 
    /// This method allows you to bind a variable to a provider function that
    /// will be called to get the value when the variable is accessed during
    /// evaluation. This can be useful for lazy evaluation or for variables
    /// whose values are expensive to compute.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the variable to bind
    /// * `provider` - The provider function that will supply the value
    /// 
    /// # Type Parameters
    /// 
    /// * `S` - The type of the variable name (must convert to `String`)
    /// * `F` - The provider function type (must implement `IntoFunction`)
    /// * `Ffm` - The function marker type (sync/async)
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing the updated activation with the appropriate
    /// function marker type, or an error if the binding failed.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let activation = Activation::new()
    ///     .bind_variable_provider("timestamp", || -> i64 {
    ///         std::time::SystemTime::now()
    ///             .duration_since(std::time::UNIX_EPOCH)
    ///             .unwrap()
    ///             .as_secs() as i64
    ///     })
    ///     .unwrap();
    /// ```
    pub fn bind_variable_provider<S, F, Ffm>(
        mut self, name: S, provider: F
    ) -> Result<Activation<'f, <Ffm as FnMarkerAggr<Fm>>::Output>, Error>
    where
        S: Into<String>,
        F: IntoFunction<'f, Ffm>,
        Ffm: FnMarkerAggr<Fm>,
    {
        self.variables.bind_provider(name, provider)?;
        Ok(Activation {
            variables: self.variables,
            functions: self.functions,
            _fn_marker: PhantomData,
        })
    }

    /// Binds a function (either global or member).
    /// 
    /// This method allows you to bind a function implementation that can be
    /// called during expression evaluation. The function can be either a
    /// global function or a member function.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the function to bind
    /// * `member` - Whether this is a member function (`true`) or global function (`false`)
    /// * `f` - The function implementation
    /// 
    /// # Type Parameters
    /// 
    /// * `S` - The type of the function name (must convert to `String`)
    /// * `F` - The function implementation type (must implement `IntoFunction`)
    /// * `Ffm` - The function marker type (sync/async)
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing the updated activation with the appropriate
    /// function marker type, or an error if the binding failed.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// // Bind a global function
    /// let activation = Activation::new()
    ///     .bind_function("double", false, |x: i64| -> i64 { x * 2 })
    ///     .unwrap();
    /// 
    /// // Bind a member function
    /// let activation = Activation::new()
    ///     .bind_function("to_upper", true, |s: String| -> String { s.to_uppercase() })
    ///     .unwrap();
    /// ```
    pub fn bind_function<S, F, Ffm, Args>(
        mut self, name: S, member: bool, f: F
    ) -> Result<Activation<'f, <Ffm as FnMarkerAggr<Fm>>::Output>, Error>
    where
        S: Into<String>,
        F: IntoFunction<'f, Ffm, Args>,
        Ffm: FnMarkerAggr<Fm>,
        Args: Arguments,
    {
        self.functions.bind(name, member, f)?;
        Ok(Activation {
            variables: self.variables,
            functions: self.functions,
            _fn_marker: PhantomData,
        })
    }

    /// Binds a member function.
    /// 
    /// This is a convenience method for binding member functions (functions that
    /// are called as methods on values, like `value.method()`).
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the function to bind
    /// * `f` - The function implementation
    /// 
    /// # Type Parameters
    /// 
    /// * `S` - The type of the function name (must convert to `String`)
    /// * `F` - The function implementation type (must implement `IntoFunction`)
    /// * `Ffm` - The function marker type (sync/async)
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing the updated activation with the appropriate
    /// function marker type, or an error if the binding failed.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let activation = Activation::new()
    ///     .bind_member_function("to_upper", |s: String| -> String { s.to_uppercase() })
    ///     .unwrap();
    /// ```
    pub fn bind_member_function<S, F, Ffm, Args>(
        self, name: S, f: F
    ) -> Result<Activation<'f, <Ffm as FnMarkerAggr<Fm>>::Output>, Error>
    where
        S: Into<String>,
        F: IntoFunction<'f, Ffm, Args>,
        Ffm: FnMarkerAggr<Fm>,
        Args: Arguments,
    {
        self.bind_function(name, true, f)
    }

    /// Binds a global function.
    /// 
    /// This is a convenience method for binding global functions (functions that
    /// can be called from any context).
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the function to bind
    /// * `f` - The function implementation
    /// 
    /// # Type Parameters
    /// 
    /// * `S` - The type of the function name (must convert to `String`)
    /// * `F` - The function implementation type (must implement `IntoFunction`)
    /// * `Ffm` - The function marker type (sync/async)
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing the updated activation with the appropriate
    /// function marker type, or an error if the binding failed.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let activation = Activation::new()
    ///     .bind_global_function("double", |x: i64| -> i64 { x * 2 })
    ///     .unwrap();
    /// ```
    pub fn bind_global_function<S, F, Ffm, Args>(
        self, name: S, f: F
    ) -> Result<Activation<'f, <Ffm as FnMarkerAggr<Fm>>::Output>, Error>
    where
        S: Into<String>,
        F: IntoFunction<'f, Ffm, Args>,
        Ffm: FnMarkerAggr<Fm>,
        Args: Arguments,
    {
        self.bind_function(name, false, f)
    }
}

impl<'f, Fm: FnMarker> ActivationInterface<'f, Fm> for Activation<'f, Fm> {
    fn variables(&self) -> &VariableBindings<'f> {
        &self.variables
    }

    fn functions(&self) -> &FunctionBindings<'f> {
        &self.functions
    }
}

impl<'a, 'f, Fm: FnMarker> ActivationInterface<'f, Fm> for &'a Activation<'f, Fm> {
    fn variables(&self) -> &VariableBindings<'f> {
        (*self).variables()
    }

    fn functions(&self) -> &FunctionBindings<'f> {
        (*self).functions()
    }
}

impl<'f, Fm: FnMarker> ActivationInterface<'f, Fm> for std::sync::Arc<Activation<'f, Fm>> {
    fn variables(&self) -> &VariableBindings<'f> {
        (&**self).variables()
    }

    fn functions(&self) -> &FunctionBindings<'f> {
        (&**self).functions()
    }
}

impl<'f, Fm: FnMarker> ActivationInterface<'f, Fm> for Box<Activation<'f, Fm>> {
    fn variables(&self) -> &VariableBindings<'f> {
        (&**self).variables()
    }

    fn functions(&self) -> &FunctionBindings<'f> {
        (&**self).functions()
    }
}

static EMPTY_VARIABLES: std::sync::LazyLock<VariableBindings<'static>> = std::sync::LazyLock::new(||
    VariableBindings::new()
);
static EMPTY_FUNCTIONS: std::sync::LazyLock<FunctionBindings<'static>> = std::sync::LazyLock::new(||
    FunctionBindings::new()
);

/// Empty activation implementation for the unit type.
/// 
/// This allows you to use `()` as an activation when no variable or function
/// bindings are needed.
impl ActivationInterface<'static> for () {
    fn variables(&self) -> &VariableBindings<'static> {
        &EMPTY_VARIABLES
    }

    fn functions(&self) -> &FunctionBindings<'static> {
        &EMPTY_FUNCTIONS
    }
}

/// Empty activation implementation for references to the unit type.
impl ActivationInterface<'static> for &() {
    fn variables(&self) -> &VariableBindings<'static> {
        &EMPTY_VARIABLES
    }

    fn functions(&self) -> &FunctionBindings<'static> {
        &EMPTY_FUNCTIONS
    }
}
