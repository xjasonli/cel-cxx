use std::sync::Arc;
use crate::{FnMarker, FnMarkerAggr, RuntimeMarker};
use crate::function::{FunctionRegistry, FnImpl, FnDecl};
use crate::variable::{VariableRegistry, Constant};

mod inner;

pub(crate) use inner::EnvInner;
use crate::{Program, Error, IntoValue, TypedValue, TypedArguments};
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
    /// # Arguments
    /// 
    /// * `name` - The name of the function as it will appear in CEL expressions
    /// * `member` - Whether this is a member function (`true`) or global function (`false`)
    /// * `f` - The function implementation
    /// 
    /// # Type Parameters
    /// 
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
    /// let builder = Env::builder()
    ///     .register_function("add", false, |x: i64, y: i64| -> i64 { x + y })
    ///     .unwrap();
    /// ```
    pub fn register_function<F, M, E, R, A>(
        mut self, name: impl Into<String>, member: bool, f: F
    ) -> Result<EnvBuilder<'f, <M as FnMarkerAggr<Fm>>::Output, Rm>, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: FnMarker + FnMarkerAggr<Fm>,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'a,
    {
        self.function_registry.register(name, member, f)?;
        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        })
    }

    /// Registers a member function.
    /// 
    /// Member functions are called on values of specific types using the dot notation.
    /// For example, if you register a member function "length" for strings,
    /// it can be called as `"hello".length()`.
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
    /// let builder = Env::builder()
    ///     .register_member_function("double", |s: String| -> String { s + &s })
    ///     .unwrap();
    /// ```
    pub fn register_member_function<F, M, E, R, A>(
        mut self, name: impl Into<String>, f: F
    ) -> Result<EnvBuilder<'f, <M as FnMarkerAggr<Fm>>::Output, Rm>, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: FnMarkerAggr<Fm>,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'a,
    {
        self.function_registry.register_member(name, f)?;
        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        })
    }

    /// Registers a global function.
    /// 
    /// Global functions can be called directly by name from CEL expressions.
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
    /// let builder = Env::builder()
    ///     .register_global_function("max", |x: i64, y: i64| -> i64 { x.max(y) })
    ///     .unwrap();
    /// ```
    pub fn register_global_function<F, M, E, R, A>(
        mut self, name: impl Into<String>, f: F
    ) -> Result<EnvBuilder<'f, <M as FnMarkerAggr<Fm>>::Output, Rm>, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: FnMarkerAggr<Fm>,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'a,
    {
        self.function_registry.register_global(name, f)?;
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
        D: FnDecl,
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
        D: FnDecl,
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
        D: FnDecl,
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
        T: Into<Constant>,
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
