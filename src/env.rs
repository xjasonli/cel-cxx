use std::sync::Arc;
use crate::{FnMarker, FnMarkerAggr, IntoConstant, RuntimeMarker};
use crate::function::{Arguments, FunctionDecl, FunctionRegistry, IntoFunction};
use crate::variable::VariableRegistry;

mod inner;

pub(crate) use inner::EnvInner;
use crate::{Program, Error, TypedValue};
use crate::ffi;

#[cfg(feature = "async")]
use crate::marker::Async;

/// CEL expression evaluation environment.
/// 
/// The `Env` struct represents a CEL environment that can compile expressions
/// into programs. It encapsulates function registries, variable declarations,
/// and type information needed for expression compilation.
/// 
/// # Type Parameters
/// 
/// - `'f`: Lifetime of functions registered in this environment
/// - `Fm`: Function marker type indicating sync/async function support
/// - `Rm`: Runtime marker type indicating the async runtime (if any)
/// 
/// # Examples
/// 
/// ## Basic Usage
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// let env = Env::builder()
///     .declare_variable::<String>("name")
///     .build()
///     .unwrap();
///     
/// let program = env.compile("'Hello, ' + name").unwrap();
/// ```
/// 
/// ## With Custom Functions
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// let env = Env::builder()
///     .register_global_function("add", |x: i64, y: i64| -> i64 { x + y })
///     .unwrap()
///     .build()
///     .unwrap();
///     
/// let program = env.compile("add(10, 20)").unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct Env<'f, Fm: FnMarker = (), Rm: RuntimeMarker = ()> {
    pub(crate) inner: Arc<EnvInner<'f>>,
    _fn_marker: std::marker::PhantomData<Fm>,
    _rt_marker: std::marker::PhantomData<Rm>,
}

/// Type alias for asynchronous CEL environments.
/// 
/// This is a convenience type alias for environments that support asynchronous
/// function evaluation.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub type AsyncEnv<'f, Rm = ()> = Env<'f, Async, Rm>;

impl<'f> Env<'f> {
    /// Creates a new environment builder.
    /// 
    /// This is the starting point for creating a CEL environment. The builder
    /// allows you to register functions, declare variables, and configure
    /// the environment before building it.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let builder = Env::builder();
    /// ```
    pub fn builder() -> EnvBuilder<'f, ()> {
        EnvBuilder::<(), ()>::new()
    }
}

impl<'f, Fm: FnMarker, Rm: RuntimeMarker> Env<'f, Fm, Rm> {
    /// Compiles a CEL expression into a Program.
    /// 
    /// This method takes a CEL expression as a string or byte slice and compiles
    /// it into a [`Program`] that can be evaluated with different activations.
    /// 
    /// # Arguments
    /// 
    /// * `source` - The CEL expression to compile
    /// 
    /// # Returns
    /// 
    /// Returns a [`Result`] containing the compiled [`Program`] or an [`Error`]
    /// if compilation fails.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder().build().unwrap();
    /// let program = env.compile("1 + 2 * 3").unwrap();
    /// ```
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The expression contains syntax errors
    /// - Referenced functions or variables are not declared
    /// - Type checking fails
    pub fn compile<S: AsRef<[u8]>>(&self, source: S) -> Result<Program<'f, Fm, Rm>, Error> {
        self.inner
            .clone()
            .compile::<Fm, Rm, _>(source)
    }
}

/// Builder for creating CEL environments.
/// 
/// The `EnvBuilder` allows you to configure a CEL environment by registering
/// functions, declaring variables, and setting up runtime options before
/// building the final environment.
/// 
/// # Type Parameters
/// 
/// - `'f`: Lifetime of functions that will be registered
/// - `Fm`: Function marker type indicating sync/async function support
/// - `Rm`: Runtime marker type indicating the async runtime (if any)
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// let env = Env::builder()
///     .register_global_function("double", |x: i64| -> i64 { x * 2 })
///     .unwrap()
///     .declare_variable::<String>("message")
///     .unwrap()
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Default)]
pub struct EnvBuilder<'f, Fm: FnMarker = (), Rm: RuntimeMarker = ()> {
    function_registry: FunctionRegistry<'f>,
    variable_registry: VariableRegistry,
    _fn_marker: std::marker::PhantomData<Fm>,
    _rt_marker: std::marker::PhantomData<Rm>,
}

impl<'f, Fm: FnMarker, Rm: RuntimeMarker> EnvBuilder<'f, Fm, Rm> {
    /// Creates a new environment builder.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let builder = EnvBuilder::<()>::new();
    /// ```
    pub fn new() -> Self {
        EnvBuilder {
            function_registry: FunctionRegistry::new(),
            variable_registry: VariableRegistry::new(),
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        }
    }
}

impl<'f, Fm: FnMarker, Rm: RuntimeMarker> EnvBuilder<'f, Fm, Rm> {
    /// Registers a function (either global or member).
    /// 
    /// This method allows you to register custom functions that can be called
    /// from CEL expressions. The function can be either a global function or
    /// a member function of a type.
    ///
    /// # Function Registration Process
    ///
    /// When you register a function, the system:
    /// 1. Extracts type information from the function signature
    /// 2. Creates type-safe conversion wrappers
    /// 3. Stores both the type signature and implementation
    /// 4. Updates the function marker type to track sync/async status
    ///
    /// # Zero-Annotation Benefits
    ///
    /// Functions are registered without explicit type annotations:
    /// - Argument types are automatically inferred
    /// - Return types are automatically determined
    /// - Error handling is automatically supported for `Result<T, E>` returns
    /// - Reference parameters like `&str` are handled safely
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the function as it will appear in CEL expressions
    /// * `member` - Whether this is a member function (`true`) or global function (`false`)
    /// * `f` - The function implementation (function pointer, closure, etc.)
    /// 
    /// # Type Parameters
    /// 
    /// * `F` - The function implementation type
    /// * `Ffm` - The function marker type (sync/async) inferred from the function
    /// * `Args` - The argument tuple type (automatically inferred)
    /// 
    /// # Returns
    ///
    /// A new `EnvBuilder` with updated function marker type. If this is the first
    /// async function registered, the marker changes from `()` to `Async`.
    ///
    /// # Member vs Global Functions
    ///
    /// ## Global Functions
    /// Called as `function_name(args...)`:
    /// ```text
    /// max(a, b)           // max function with two arguments
    /// calculate(x, y, z)  // calculate function with three arguments
    /// ```
    ///
    /// ## Member Functions  
    /// Called as `object.method(args...)`:
    /// ```text
    /// text.contains(substring)    // contains method on string
    /// list.size()                // size method on list
    /// ```
    ///
    /// # Function Signature Support
    ///
    /// Supports various function signatures:
    /// - **Simple functions**: `fn(T) -> U`
    /// - **Functions with errors**: `fn(T) -> Result<U, E>`
    /// - **Reference parameters**: `fn(&str, i64) -> String`
    /// - **Multiple parameters**: Up to 10 parameters supported
    /// - **Closures**: Move closures that capture environment
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if:
    /// - Function name conflicts with existing registration
    /// - Function signature is invalid or unsupported
    /// - Type inference fails
    /// 
    /// # Examples
    /// 
    /// ## Basic Functions
    /// 
    /// ```rust
    /// use cel_cxx::*;
    /// 
    /// let builder = Env::builder()
    ///     .register_function("add", false, |a: i64, b: i64| a + b)?
    ///     .register_function("greet", false, |name: &str| format!("Hello, {}!", name))?;
    /// ```
    ///
    /// ## Member Functions
    ///
    /// ```rust
    /// use cel_cxx::*;
    ///
    /// let builder = Env::builder()
    ///     .register_function("contains", true, |text: &str, substr: &str| text.contains(substr))?
    ///     .register_function("length", true, |text: &str| text.len() as i64)?;
    /// 
    /// // Usage in expressions:
    /// // text.contains("hello")
    /// // text.length()
    /// ```
    ///
    /// ## Functions with Error Handling
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// use std::convert::Infallible;
    ///
    /// let builder = Env::builder()
    ///     .register_function("divide", false, |a: f64, b: f64| -> Result<f64, Infallible> {
    ///         if b == 0.0 {
    ///             Err(Infallible)  // This won't actually be reached, just for example
    ///         } else {
    ///             Ok(a / b)
    ///         }
    ///     })?;
    /// ```
    ///
    /// ## Closures with Captured Data
    ///
    /// ```rust
    /// use cel_cxx::*;
    ///
    /// let multiplier = 5;
    /// let threshold = 100.0;
    ///
    /// let builder = Env::builder()
    ///     .register_function("scale", false, move |x: i64| x * multiplier)?
    ///     .register_function("check_limit", false, move |value: f64| value < threshold)?;
    /// ```
    pub fn register_function<F, Ffm, Args>(
        mut self, name: impl Into<String>, member: bool, f: F
    ) -> Result<EnvBuilder<'f, <Ffm as FnMarkerAggr<Fm>>::Output, Rm>, Error>
    where
        F: IntoFunction<'f, Ffm, Args>,
        Ffm: FnMarker + FnMarkerAggr<Fm>,
        Args: Arguments,
    {
        self.function_registry
            .register(name, member, f)?;

        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        })
    }

    /// Registers a member function.
    ///
    /// This is a convenience method for registering member functions, equivalent to
    /// calling `register_function(name, true, f)`. Member functions are called using
    /// dot notation in CEL expressions: `object.method(args...)`.
    ///
    /// # Arguments
    ///
    /// * `name` - The method name as it will appear in CEL expressions
    /// * `f` - The function implementation
    ///
    /// # Member Function Semantics
    ///
    /// Member functions in CEL follow these patterns:
    /// - First parameter is the "receiver" (the object before the dot)
    /// - Additional parameters become method arguments
    /// - Called as `receiver.method(arg1, arg2, ...)`
    ///
    /// # Examples
    ///
    /// ## String Methods
    ///
    /// ```rust
    /// use cel_cxx::*;
    ///
    /// let builder = Env::builder()
    ///     .register_member_function("upper", |s: &str| s.to_uppercase())?
    ///     .register_member_function("contains", |s: &str, substr: &str| s.contains(substr))?
    ///     .register_member_function("repeat", |s: &str, n: i64| s.repeat(n as usize))?;
    ///
    /// // Usage in expressions:
    /// // "hello".upper()           -> "HELLO"
    /// // "hello world".contains("world") -> true
    /// // "abc".repeat(3)           -> "abcabcabc"
    /// ```
    ///
    /// ## Numeric Methods
    ///
    /// ```rust
    /// use cel_cxx::*;
    ///
    /// let builder = Env::builder()
    ///     .register_member_function("abs", |x: f64| x.abs())?
    ///     .register_member_function("pow", |x: f64, exp: f64| x.powf(exp))?;
    ///
    /// // Usage in expressions:
    /// // (-5.5).abs()     -> 5.5
    /// // (2.0).pow(3.0)   -> 8.0
    /// ```
    ///
    /// ## Collection Methods
    ///
    /// ```rust
    /// use cel_cxx::*;
    ///
    /// let builder = Env::builder()
    ///     .register_member_function("len", |list: &Vec<i64>| list.len() as i64)?
    ///     .register_member_function("contains", |list: &Vec<i64>, item: &i64| list.contains(item))?;
    ///
    /// // Usage in expressions (assuming list variable):
    /// // my_list.len()         -> list length
    /// // my_list.contains(42)  -> true/false
    /// ```
    pub fn register_member_function<F, Ffm, Args>(
        mut self, name: impl Into<String>, f: F
    ) -> Result<EnvBuilder<'f, <Ffm as FnMarkerAggr<Fm>>::Output, Rm>, Error>
    where
        F: IntoFunction<'f, Ffm, Args>,
        Ffm: FnMarker + FnMarkerAggr<Fm>,
        Args: Arguments,
    {
        self.function_registry
            .register_member(name, f)?;

        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        })
    }

    /// Registers a global function.
    ///
    /// This is a convenience method for registering global functions, equivalent to
    /// calling `register_function(name, false, f)`. Global functions are called directly
    /// by name in CEL expressions: `function_name(args...)`.
    ///
    /// # Arguments
    ///
    /// * `name` - The function name as it will appear in CEL expressions
    /// * `f` - The function implementation
    ///
    /// # Global Function Characteristics
    ///
    /// Global functions:
    /// - Are called directly by name without a receiver object
    /// - Can have 0 to 10 parameters
    /// - Support all CEL-compatible parameter and return types
    /// - Can capture environment variables (for closures)
    ///
    /// # Function Naming Guidelines
    ///
    /// - Use clear, descriptive names: `calculate_tax`, `format_date`
    /// - Follow CEL naming conventions (snake_case is recommended)
    /// - Avoid conflicts with built-in CEL functions
    /// - Consider namespacing for domain-specific functions: `math_sqrt`, `string_trim`
    ///
    /// # Examples
    ///
    /// ## Mathematical Functions
    ///
    /// ```rust
    /// use cel_cxx::*;
    ///
    /// let builder = Env::builder()
    ///     .register_global_function("add", |a: i64, b: i64| a + b)?
    ///     .register_global_function("multiply", |a: f64, b: f64| a * b)?
    ///     .register_global_function("max", |a: i64, b: i64| if a > b { a } else { b })?;
    ///
    /// // Usage in expressions:
    /// // add(10, 20)          -> 30
    /// // multiply(2.5, 4.0)   -> 10.0
    /// // max(15, 8)           -> 15
    /// ```
    ///
    /// ## String Processing Functions
    ///
    /// ```rust
    /// use cel_cxx::*;
    ///
    /// let builder = Env::builder()
    ///     .register_global_function("concat", |a: &str, b: &str| format!("{}{}", a, b))?
    ///     .register_global_function("trim_prefix", |s: &str, prefix: &str| {
    ///         s.strip_prefix(prefix).unwrap_or(s).to_string()
    ///     })?;
    ///
    /// // Usage in expressions:
    /// // concat("Hello, ", "World!")     -> "Hello, World!"
    /// // trim_prefix("prefixed_text", "prefixed_")  -> "text"
    /// ```
    ///
    /// ## Business Logic Functions
    ///
    /// ```rust
    /// use cel_cxx::*;
    ///
    /// let builder = Env::builder()
    ///     .register_global_function("calculate_discount", |price: f64, rate: f64| {
    ///         price * (1.0 - rate.min(1.0).max(0.0))
    ///     })?
    ///     .register_global_function("is_valid_email", |email: &str| {
    ///         email.contains('@') && email.contains('.')
    ///     })?;
    ///
    /// // Usage in expressions:
    /// // calculate_discount(100.0, 0.15)     -> 85.0
    /// // is_valid_email("user@domain.com")   -> true
    /// ```
    ///
    /// ## Functions with Complex Logic
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// use std::collections::HashMap;
    ///
    /// // Function that processes collections
    /// let builder = Env::builder()
    ///     .register_global_function("sum_positive", |numbers: &Vec<i64>| {
    ///         numbers.iter().filter(|&&x| x > 0).sum::<i64>()
    ///     })?;
    ///
    /// // Usage in expressions:
    /// // sum_positive([1, -2, 3, -4, 5])  -> 9
    /// ```
    pub fn register_global_function<F, Ffm, Args>(
        mut self, name: impl Into<String>, f: F
    ) -> Result<EnvBuilder<'f, <Ffm as FnMarkerAggr<Fm>>::Output, Rm>, Error>
    where
        F: IntoFunction<'f, Ffm, Args>,
        Ffm: FnMarker + FnMarkerAggr<Fm>,
        Args: Arguments,
    {
        self.function_registry
            .register_global(name, f)?;

        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        })
    }

    /// Declares a function signature without providing an implementation.
    /// 
    /// This is useful when you want to declare that a function exists for
    /// type checking purposes, but will provide the implementation later
    /// via activation bindings.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the function
    /// * `member` - Whether this is a member function (`true`) or global function (`false`)
    /// 
    /// # Type Parameters
    /// 
    /// * `D` - The function declaration type that specifies the signature
    pub fn declare_function<D>(
        mut self, name: impl Into<String>, member: bool
    ) -> Result<Self, Error>
    where
        D: FunctionDecl,
    {
        self.function_registry.declare::<D>(name, member)?;
        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        })
    }

    /// Declares a member function signature without providing an implementation.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the member function
    /// 
    /// # Type Parameters
    /// 
    /// * `D` - The function declaration type that specifies the signature
    pub fn declare_member_function<D>(
        mut self, name: impl Into<String>
    ) -> Result<Self, Error>
    where
        D: FunctionDecl,
    {
        self.function_registry.declare_member::<D>(name)?;
        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        })
    }

    /// Declares a global function signature without providing an implementation.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the global function
    /// 
    /// # Type Parameters
    /// 
    /// * `D` - The function declaration type that specifies the signature
    pub fn declare_global_function<D>(
        mut self, name: impl Into<String>
    ) -> Result<Self, Error>
    where
        D: FunctionDecl,
    {
        self.function_registry.declare_global::<D>(name)?;
        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        })
    }

    /// Defines a constant value that can be referenced in expressions.
    /// 
    /// Constants are immutable values that are resolved at compile time.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the constant
    /// * `value` - The constant value
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let builder = Env::builder()
    ///     .define_constant("PI", 3.14159)
    ///     .unwrap();
    /// ```
    pub fn define_constant<T>(
        mut self, name: impl Into<String>, value: T
    ) -> Result<Self, Error>
    where
        T: IntoConstant,
    {
        self.variable_registry.define_constant(name, value)?;
        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        })
    }

    /// Declares a variable of a specific type.
    /// 
    /// This declares that a variable of the given name and type may be
    /// provided during evaluation. The actual value must be bound in
    /// the activation when evaluating expressions.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the variable
    /// 
    /// # Type Parameters
    /// 
    /// * `T` - The type of the variable
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let builder = Env::builder()
    ///     .declare_variable::<String>("user_name")
    ///     .unwrap()
    ///     .declare_variable::<i64>("age")
    ///     .unwrap();
    /// ```
    pub fn declare_variable<T>(mut self, name: impl Into<String>) -> Result<Self, Error>
    where
        T: TypedValue,
    {
        self.variable_registry.declare_variable::<T>(name)?;
        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        })
    }

    /// Builds the environment from the configured builder.
    /// 
    /// This method consumes the builder and creates the final [`Env`] instance
    /// that can be used to compile CEL expressions.
    /// 
    /// # Returns
    /// 
    /// Returns a [`Result`] containing the built [`Env`] or an [`Error`] if
    /// the environment could not be created.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .declare_variable::<String>("name")
    ///     .unwrap()
    ///     .build()
    ///     .unwrap();
    /// ```
    /// 
    /// # Errors
    /// 
    /// Returns an error if the environment configuration is invalid or
    /// if the underlying CEL environment cannot be created.
    pub fn build(self) -> Result<Env<'f, Fm, Rm>, Error> {
        let inner = EnvInner::new_with_registries(
            self.function_registry,
            self.variable_registry,
        ).map_err(|ffi_status| ffi::error_to_rust(&ffi_status))?;
        let env = Env {
            inner: Arc::new(inner),
            _fn_marker: self._fn_marker,
            _rt_marker: self._rt_marker,
        };
        Ok(env)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
const _: () = {
    use crate::r#async::*;

    impl<'f, Rm: RuntimeMarker> Env<'f, (), Rm> {
        /// Forces conversion to an async environment.
        /// 
        /// This method converts a synchronous environment to an asynchronous one,
        /// allowing it to work with async functions and evaluation.
        /// 
        /// # Type Parameters
        /// 
        /// * `Rt` - The async runtime type to use
        /// 
        /// # Examples
        /// 
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # {
        /// use cel_cxx::*;
        /// 
        /// let sync_env = Env::builder().build().unwrap();
        /// let async_env = sync_env.force_async::<Tokio>();
        /// # }
        /// ```
        pub fn force_async<Rt: Runtime>(self) -> Env<'f, Async, Rt> {
            Env {
                inner: self.inner,
                _fn_marker: std::marker::PhantomData,
                _rt_marker: std::marker::PhantomData,
            }
        }
    }

    impl<'f, Rm: RuntimeMarker> EnvBuilder<'f, (), Rm> {
        /// Forces conversion to an async environment builder.
        /// 
        /// This method converts a synchronous environment builder to an asynchronous one,
        /// allowing it to register async functions and build async environments.
        /// 
        /// # Examples
        /// 
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # {
        /// use cel_cxx::*;
        /// 
        /// let async_builder = Env::builder().force_async();
        /// # }
        /// ```
        pub fn force_async(self) -> EnvBuilder<'f, Async, Rm> {
            EnvBuilder {
                function_registry: self.function_registry,
                variable_registry: self.variable_registry,
                _fn_marker: std::marker::PhantomData,
                _rt_marker: std::marker::PhantomData,
            }
        }
    }

    impl<'f, Fm: FnMarker> Env<'f, Fm, ()> {
        /// Sets the async runtime for this environment.
        /// 
        /// This method specifies which async runtime should be used for
        /// asynchronous evaluation of expressions.
        /// 
        /// # Type Parameters
        /// 
        /// * `Rt` - The runtime type to use (must implement [`Runtime`])
        /// 
        /// # Examples
        /// 
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # {
        /// use cel_cxx::*;
        /// 
        /// let env = Env::builder()
        ///     .build()
        ///     .unwrap()
        ///     .use_runtime::<Tokio>();
        /// # }
        /// ```
        pub fn use_runtime<Rt: Runtime>(self) -> Env<'f, Fm, Rt> {
            let inner = self.inner.clone();
            Env {
                inner,
                _fn_marker: self._fn_marker,
                _rt_marker: std::marker::PhantomData,
            }
        }

        /// Configures the environment to use the Tokio async runtime.
        /// 
        /// This is a convenience method for setting the runtime to Tokio.
        /// Requires the `tokio` feature to be enabled.
        /// 
        /// # Examples
        /// 
        /// ```rust,no_run
        /// # #[cfg(all(feature = "async", feature = "tokio"))]
        /// # {
        /// use cel_cxx::*;
        /// 
        /// let env = Env::builder()
        ///     .build()
        ///     .unwrap()
        ///     .use_tokio();
        /// # }
        /// ```
        #[cfg(feature = "tokio")]
        #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
        pub fn use_tokio(self) -> Env<'f, Fm, Tokio> {
            self.use_runtime::<Tokio>()
        }

        /// Configures the environment to use the async-std runtime.
        /// 
        /// This is a convenience method for setting the runtime to async-std.
        /// Requires the `async-std` feature to be enabled.
        /// 
        /// # Examples
        /// 
        /// ```rust,no_run
        /// # #[cfg(all(feature = "async", feature = "async-std"))]
        /// # {
        /// use cel_cxx::*;
        /// 
        /// let env = Env::builder()
        ///     .build()
        ///     .unwrap()
        ///     .use_async_std();
        /// # }
        /// ```
        #[cfg(feature = "async-std")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async-std")))]
        pub fn use_async_std(self) -> Env<'f, Fm, AsyncStd> {
            self.use_runtime::<AsyncStd>()
        }
    }

    impl<'f, Fm: FnMarker> EnvBuilder<'f, Fm, ()> {
        /// Sets the async runtime for the environment builder.
        /// 
        /// This method specifies which async runtime should be used by
        /// environments built from this builder.
        /// 
        /// # Type Parameters
        /// 
        /// * `Rt` - The runtime type to use (must implement [`Runtime`])
        /// 
        /// # Examples
        /// 
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # {
        /// use cel_cxx::*;
        /// 
        /// let builder = Env::builder().use_runtime::<Tokio>();
        /// # }
        /// ```
        pub fn use_runtime<Rt: Runtime>(self) -> EnvBuilder<'f, Fm, Rt> {
            EnvBuilder {
                function_registry: self.function_registry,
                variable_registry: self.variable_registry,
                _fn_marker: self._fn_marker,
                _rt_marker: std::marker::PhantomData,
            }
        }

        /// Configures the builder to use the Tokio async runtime.
        /// 
        /// This is a convenience method for setting the runtime to Tokio.
        /// Requires the `tokio` feature to be enabled.
        /// 
        /// # Examples
        /// 
        /// ```rust,no_run
        /// # #[cfg(all(feature = "async", feature = "tokio"))]
        /// # {
        /// use cel_cxx::*;
        /// 
        /// let builder = Env::builder().use_tokio();
        /// # }
        /// ```
        #[cfg(feature = "tokio")]
        #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
        pub fn use_tokio(self) -> EnvBuilder<'f, Fm, Tokio> {
            self.use_runtime::<Tokio>()
        }

        /// Configures the builder to use the async-std runtime.
        /// 
        /// This is a convenience method for setting the runtime to async-std.
        /// Requires the `async-std` feature to be enabled.
        /// 
        /// # Examples
        /// 
        /// ```rust,no_run
        /// # #[cfg(all(feature = "async", feature = "async-std"))]
        /// # {
        /// use cel_cxx::*;
        /// 
        /// let builder = Env::builder().use_async_std();
        /// # }
        /// ```
        #[cfg(feature = "async-std")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async-std")))]
        pub fn use_async_std(self) -> EnvBuilder<'f, Fm, AsyncStd> {
            self.use_runtime::<AsyncStd>()
        }
    }
};
