use crate::function::{Arguments, FunctionDecl, FunctionRegistry, IntoFunction};
use crate::variable::VariableRegistry;
use crate::{FnMarker, FnMarkerAggr, IntoConstant, RuntimeMarker};
use std::sync::Arc;

mod inner;

use crate::ffi;
use crate::{Error, Program, TypedValue};
pub(crate) use inner::{EnvInner, EnvInnerOptions};

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
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
///     .declare_variable::<String>("name")?
///     .build()?;
///     
/// let program = env.compile("'Hello, ' + name")?;
/// # Ok::<(), cel_cxx::Error>(())
/// ```
///
/// ## With Custom Functions
///
/// ```rust,no_run
/// use cel_cxx::*;
///
/// let env = Env::builder()
///     .register_global_function("add", |x: i64, y: i64| -> i64 { x + y })?
///     .build()?;
///     
/// let program = env.compile("add(10, 20)")?;
/// # Ok::<(), cel_cxx::Error>(())
/// ```
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
        self.inner.clone().compile::<Fm, Rm, _>(source)
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
///     .register_global_function("double", |x: i64| -> i64 { x * 2 })?
///     .declare_variable::<String>("message")?
///     .build()?;
/// # Ok::<(), cel_cxx::Error>(())
/// ```
pub struct EnvBuilder<'f, Fm: FnMarker = (), Rm: RuntimeMarker = ()> {
    function_registry: FunctionRegistry<'f>,
    variable_registry: VariableRegistry,
    options: EnvInnerOptions,
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
            options: EnvInnerOptions::default(),
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        }
    }
}

impl<'f, Fm: FnMarker, Rm: RuntimeMarker> EnvBuilder<'f, Fm, Rm> {
    /// Sets the CEL container for the environment, which acts as a namespace for unqualified names.
    ///
    /// The container influences how unqualified names (like function or variable names) are
    /// resolved during expression compilation.
    ///
    /// # Examples
    ///
    /// If the container is set to `my.app`, an unqualified reference to `MyMessage` will be
    /// resolved as `my.app.MyMessage`.
    ///
    /// ```cel
    /// // With container "my.app", this expression:
    /// MyMessage{field: 123}
    /// // is equivalent to:
    /// my.app.MyMessage{field: 123}
    /// ```
    pub fn with_container(mut self, container: impl Into<String>) -> Self {
        self.options.container = container.into();
        self
    }

    /// Enables or disables the CEL standard library of functions and macros.
    ///
    /// The standard library provides a rich set of common functions for types like `string`,
    /// `list`, `map`, as well as logical and arithmetic operators. It is enabled by default.
    /// Disabling it can reduce the environment's footprint if only custom functions are needed.
    ///
    /// # Examples
    ///
    /// ```cel
    /// // Standard functions like `size()` and operators like `+` are available:
    /// 'hello'.size() + ' world'.size() == 11
    /// ```
    pub fn with_standard(mut self, enable: bool) -> Self {
        self.options.enable_standard = enable;
        self
    }

    /// Enables or disables support for CEL's optional types and related syntax.
    ///
    /// This enables the `optional` type and related features like optional field selection (`.?`),
    /// optional index/key access (`[?_]`), and optional value construction (`{?key: ...}`).
    /// This is disabled by default.
    ///
    /// # Examples
    ///
    /// ```cel
    /// // Optional field selection returns an optional value.
    /// msg.?field.orValue('default')
    ///
    /// // Optional map construction only includes the entry if the value is present.
    /// {'name': 'bob', ?'age': optional.of(25)}
    /// ```
    pub fn with_optional(mut self, enable: bool) -> Self {
        self.options.enable_optional = enable;
        self
    }

    /// Enables or disables the Bindings extension.
    ///
    /// This extension provides the `cel.bind()` macro, which allows for temporary variable
    /// bindings within a CEL expression to improve readability and performance.
    ///
    /// # Examples
    ///
    /// ```cel
    /// // Bind a sub-expression to a variable `sub_list`.
    /// cel.bind(sub_list, [1, 2, 3], sub_list.size() > 2)
    /// ```
    pub fn with_ext_bindings(mut self, enable: bool) -> Self {
        self.options.enable_ext_bindings = enable;
        self
    }

    /// Enables or disables the Encoders extension.
    ///
    /// This extension provides functions for encoding and decoding between common data formats,
    /// such as `base64`.
    ///
    /// # Examples
    ///
    /// ```cel
    /// // Encode a byte sequence to a Base64 string.
    /// base64.encode(b'hello') == 'aGVsbG8='
    /// ```
    pub fn with_ext_encoders(mut self, enable: bool) -> Self {
        self.options.enable_ext_encoders = enable;
        self
    }

    /// Enables or disables the Lists extension.
    ///
    /// This extension provides additional functions for working with lists, such as `indexOf`
    /// and `lastIndexOf`.
    ///
    /// # Examples
    ///
    /// ```cel
    /// // Find the first index of an element in a list.
    /// [1, 2, 3, 2].indexOf(2) == 1
    /// ```
    pub fn with_ext_lists(mut self, enable: bool) -> Self {
        self.options.enable_ext_lists = enable;
        self
    }

    /// Enables or disables the Math extension.
    ///
    /// This extension provides advanced mathematical functions beyond the standard operators,
    /// such as `min`, `max`, and `sqrt`.
    ///
    /// # Examples
    ///
    /// ```cel
    /// // Find the minimum of a set of numbers.
    /// math.min([5, 2, 8]) == 2
    /// ```
    pub fn with_ext_math(mut self, enable: bool) -> Self {
        self.options.enable_ext_math = enable;
        self
    }

    /// Enables or disables the Protocol Buffers (Protobuf) extension.
    ///
    /// This provides the ability to work with Protobuf messages, including accessing fields
    /// and using Protobuf-specific functions.
    ///
    /// # Note
    /// Requires proper setup of Protobuf descriptors in the environment.
    ///
    /// # Examples
    ///
    /// ```cel
    /// // Access a field on a Protobuf message.
    /// my_proto_message.my_field == 'value'
    /// ```
    pub fn with_ext_proto(mut self, enable: bool) -> Self {
        self.options.enable_ext_proto = enable;
        self
    }

    /// Enables or disables the Regular Expression (Regex) extension.
    ///
    /// This extension provides functions for pattern matching on strings using regular expressions,
    /// such as `matches`.
    ///
    /// # Examples
    ///
    /// ```cel
    /// // Check if a string matches a regex pattern.
    /// 'a-123'.matches('^[a-z]-\\d+$')
    /// ```
    pub fn with_ext_regex(mut self, enable: bool) -> Self {
        self.options.enable_ext_regex = enable;
        self
    }

    /// Enables or disables the Regular Expression (Regex) extension.
    ///
    /// This extension provides functions for pattern matching on strings using regular expressions,
    /// such as `matches`.
    pub fn with_ext_re(mut self, enable: bool) -> Self {
        self.options.enable_ext_re = enable;
        self
    }

    /// Enables or disables the Sets extension.
    ///
    /// This extension provides functions for set-based operations on lists, such as `intersects`
    /// and `contains`. Note that CEL does not have a native `set` type; these functions
    /// treat lists as sets.
    ///
    /// # Examples
    ///
    /// ```cel
    /// // Check if two lists (treated as sets) have common elements.
    /// [1, 2, 3].intersects([3, 4, 5])
    /// ```
    pub fn with_ext_sets(mut self, enable: bool) -> Self {
        self.options.enable_ext_sets = enable;
        self
    }

    /// Enables or disables the Strings extension.
    ///
    /// This extension provides additional functions for string manipulation, such as `join`,
    /// `format`, and `replace`.
    ///
    /// # Examples
    ///
    /// ```cel
    /// // Join a list of strings with a separator.
    /// ['a', 'b', 'c'].join(',') == 'a,b,c'
    /// ```
    pub fn with_ext_strings(mut self, enable: bool) -> Self {
        self.options.enable_ext_strings = enable;
        self
    }

    /// Enables or disables the select optimization extension.
    ///
    /// This is an optimization that can improve the performance of `select` expressions
    /// (field access) by transforming them at compile time. It does not introduce new
    /// user-visible functions but can change the evaluation cost.
    pub fn with_ext_select_optimization(mut self, enable: bool) -> Self {
        self.options.enable_ext_select_optimization = enable;
        self
    }

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
    /// # Ok::<(), cel_cxx::Error>(())
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
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    ///
    /// ## Functions with Error Handling
    ///
    /// ```rust
    /// use cel_cxx::*;
    ///
    /// let builder = Env::builder()
    ///     .register_function("divide", false, |a: f64, b: f64| -> Result<f64, Error> {
    ///         if b == 0.0 {
    ///             Err(Error::invalid_argument("division by zero"))
    ///         } else {
    ///             Ok(a / b)
    ///         }
    ///     })?;
    /// # Ok::<(), cel_cxx::Error>(())
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
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn register_function<F, Ffm, Args>(
        mut self,
        name: impl Into<String>,
        member: bool,
        f: F,
    ) -> Result<EnvBuilder<'f, <Ffm as FnMarkerAggr<Fm>>::Output, Rm>, Error>
    where
        F: IntoFunction<'f, Ffm, Args>,
        Ffm: FnMarker + FnMarkerAggr<Fm>,
        Args: Arguments,
    {
        self.function_registry.register(name, member, f)?;

        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            options: self.options,
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
    /// # Ok::<(), cel_cxx::Error>(())
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
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn register_member_function<F, Ffm, Args>(
        mut self,
        name: impl Into<String>,
        f: F,
    ) -> Result<EnvBuilder<'f, <Ffm as FnMarkerAggr<Fm>>::Output, Rm>, Error>
    where
        F: IntoFunction<'f, Ffm, Args>,
        Ffm: FnMarker + FnMarkerAggr<Fm>,
        Args: Arguments,
    {
        self.function_registry.register_member(name, f)?;

        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            options: self.options,
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
    /// # Ok::<(), cel_cxx::Error>(())
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
    /// # Ok::<(), cel_cxx::Error>(())
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
    /// # Ok::<(), cel_cxx::Error>(())
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
    ///     .register_global_function("sum_positive", |numbers: Vec<i64>| {
    ///         numbers.iter().filter(|&x| *x > 0).sum::<i64>()
    ///     })?;
    ///
    /// // Usage in expressions:
    /// // sum_positive([1, -2, 3, -4, 5])  -> 9
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn register_global_function<F, Ffm, Args>(
        mut self,
        name: impl Into<String>,
        f: F,
    ) -> Result<EnvBuilder<'f, <Ffm as FnMarkerAggr<Fm>>::Output, Rm>, Error>
    where
        F: IntoFunction<'f, Ffm, Args>,
        Ffm: FnMarker + FnMarkerAggr<Fm>,
        Args: Arguments,
    {
        self.function_registry.register_global(name, f)?;

        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            options: self.options,
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
        mut self,
        name: impl Into<String>,
        member: bool,
    ) -> Result<Self, Error>
    where
        D: FunctionDecl,
    {
        self.function_registry.declare::<D>(name, member)?;
        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            options: self.options,
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
    pub fn declare_member_function<D>(mut self, name: impl Into<String>) -> Result<Self, Error>
    where
        D: FunctionDecl,
    {
        self.function_registry.declare_member::<D>(name)?;
        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            options: self.options,
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
    pub fn declare_global_function<D>(mut self, name: impl Into<String>) -> Result<Self, Error>
    where
        D: FunctionDecl,
    {
        self.function_registry.declare_global::<D>(name)?;
        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            options: self.options,
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
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn define_constant<T>(mut self, name: impl Into<String>, value: T) -> Result<Self, Error>
    where
        T: IntoConstant,
    {
        self.variable_registry.define_constant(name, value)?;
        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            options: self.options,
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
    ///     .declare_variable::<String>("user_name")?
    ///     .declare_variable::<i64>("age")?;
    ///
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn declare_variable<T>(mut self, name: impl Into<String>) -> Result<Self, Error>
    where
        T: TypedValue,
    {
        self.variable_registry.declare::<T>(name)?;
        Ok(EnvBuilder {
            function_registry: self.function_registry,
            variable_registry: self.variable_registry,
            options: self.options,
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
    ///     .declare_variable::<String>("name")?
    ///     .build()?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the environment configuration is invalid or
    /// if the underlying CEL environment cannot be created.
    pub fn build(self) -> Result<Env<'f, Fm, Rm>, Error> {
        let inner = EnvInner::new_with_registries(self.function_registry, self.variable_registry, self.options)
            .map_err(|ffi_status| ffi::error_to_rust(&ffi_status))?;
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
        /// let sync_env = Env::builder().build()?;
        /// let async_env = sync_env.force_async();
        /// # }
        /// # Ok::<(), cel_cxx::Error>(())
        /// ```
        pub fn force_async(self) -> Env<'f, Async, Rm> {
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
                options: self.options,
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
        ///     .build()?
        ///     .use_runtime::<Tokio>();
        /// # }
        /// # Ok::<(), cel_cxx::Error>(())
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
        ///     .build()?
        ///     .use_tokio();
        /// # }
        /// # Ok::<(), cel_cxx::Error>(())
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
        ///     .build()?
        ///     .use_async_std();
        /// # }
        /// # Ok::<(), cel_cxx::Error>(())
        /// ```
        #[cfg(feature = "async-std")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async-std")))]
        pub fn use_async_std(self) -> Env<'f, Fm, AsyncStd> {
            self.use_runtime::<AsyncStd>()
        }

        /// Configures the environment to use the smol runtime.
        ///
        /// This is a convenience method for setting the runtime to smol.
        /// Requires the `smol` feature to be enabled.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # {
        /// use cel_cxx::*;
        ///
        /// let env = Env::builder().use_smol();
        /// # }
        /// # Ok::<(), cel_cxx::Error>(())
        /// ```
        #[cfg(feature = "smol")]
        #[cfg_attr(docsrs, doc(cfg(feature = "smol")))]
        pub fn use_smol(self) -> Env<'f, Fm, Smol> {
            self.use_runtime::<Smol>()
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
                options: self.options,
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

        /// Configures the builder to use the smol runtime.
        ///
        /// This is a convenience method for setting the runtime to smol.
        /// Requires the `smol` feature to be enabled.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # {
        /// use cel_cxx::*;
        ///
        /// let builder = Env::builder().use_smol();
        /// # }
        /// ```
        #[cfg(feature = "smol")]
        #[cfg_attr(docsrs, doc(cfg(feature = "smol")))]
        pub fn use_smol(self) -> EnvBuilder<'f, Fm, Smol> {
            self.use_runtime::<Smol>()
        }
    }
};

impl<'f, Fm: FnMarker, Rm: RuntimeMarker> std::fmt::Debug for Env<'f, Fm, Rm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Env").field("inner", &self.inner).finish()
    }
}

impl<'f, Fm: FnMarker, Rm: RuntimeMarker> Clone for Env<'f, Fm, Rm> {
    fn clone(&self) -> Self {
        Env {
            inner: self.inner.clone(),
            _fn_marker: self._fn_marker,
            _rt_marker: self._rt_marker,
        }
    }
}

impl<'f, Fm: FnMarker, Rm: RuntimeMarker> std::fmt::Debug for EnvBuilder<'f, Fm, Rm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnvBuilder")
            .field("function_registry", &self.function_registry)
            .field("variable_registry", &self.variable_registry)
            .finish()
    }
}

impl<'f, Fm: FnMarker, Rm: RuntimeMarker> Default for EnvBuilder<'f, Fm, Rm> {
    fn default() -> Self {
        EnvBuilder {
            function_registry: FunctionRegistry::new(),
            variable_registry: VariableRegistry::new(),
            options: EnvInnerOptions::default(),
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        }
    }
}
