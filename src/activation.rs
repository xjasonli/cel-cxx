use std::marker::PhantomData;
use crate::function::FunctionBindings;
use crate::Error;
use crate::variable::VariableBindings;
use crate::values::{IntoValue, TypedValue, TypedArguments};
use crate::variable::Provider;
use crate::function::FnImpl;
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
    /// * `F` - The provider function type
    /// * `M` - The function marker type (sync/async)
    /// * `E` - The error type that the provider can return
    /// * `T` - The type of value the provider returns
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
    pub fn bind_variable_provider<S, F, M, E, T>(
        mut self, name: S, provider: F
    ) -> Result<Activation<'f, <M as FnMarkerAggr<Fm>>::Output>, Error>
    where
        S: Into<String>,
        F: Provider<M, E, T> + 'f,
        M: FnMarkerAggr<Fm>,
        E: Into<Error> + Send + Sync + 'static,
        T: IntoValue + TypedValue + 'f,
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
    /// * `F` - The function implementation type
    /// * `M` - The function marker type (sync/async)
    /// * `E` - The error type that the function can return
    /// * `R` - The return type of the function
    /// * `A` - The argument types of the function
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let activation = Activation::new()
    ///     .bind_function("double", false, |x: i64| -> i64 { x * 2 })
    ///     .unwrap();
    /// ```
    pub fn bind_function<S, F, M, E, R, A>(
        mut self, name: S, member: bool, f: F
    ) -> Result<Activation<'f, <M as FnMarkerAggr<Fm>>::Output>, Error>
    where
        S: Into<String>,
        F: FnImpl<M, E, R, A> + 'f,
        M: FnMarkerAggr<Fm>,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'f,
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
    /// Member functions are called on values of specific types using the dot notation.
    /// This is a convenience method equivalent to `bind_function(name, true, f)`.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the member function
    /// * `f` - The function implementation
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let activation = Activation::new()
    ///     .bind_member_function("reverse", |s: String| -> String {
    ///         s.chars().rev().collect()
    ///     })
    ///     .unwrap();
    /// ```
    pub fn bind_member_function<S, F, M, E, R, A>(
        self, name: S, f: F
    ) -> Result<Activation<'f, <M as FnMarkerAggr<Fm>>::Output>, Error>
    where
        S: Into<String>,
        F: FnImpl<M, E, R, A> + 'f,
        M: FnMarkerAggr<Fm>,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'f,
    {
        self.bind_function(name, true, f)
    }

    /// Binds a global function.
    /// 
    /// Global functions can be called directly by name from CEL expressions.
    /// This is a convenience method equivalent to `bind_function(name, false, f)`.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the global function
    /// * `f` - The function implementation
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let activation = Activation::new()
    ///     .bind_global_function("pow", |base: f64, exp: f64| -> f64 {
    ///         base.powf(exp)
    ///     })
    ///     .unwrap();
    /// ```
    pub fn bind_global_function<S, F, M, E, R, A>(
        self, name: S, f: F
    ) -> Result<Activation<'f, <M as FnMarkerAggr<Fm>>::Output>, Error>
    where
        S: Into<String>,
        F: FnImpl<M, E, R, A> + 'f,
        M: FnMarkerAggr<Fm>,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'f,
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
