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
    /// **Default**: Empty string (root scope)
    ///
    /// The container influences how unqualified names (like function or variable names) are
    /// resolved during expression compilation. This affects the CEL runtime's name resolution
    /// behavior for types, functions, and variables.
    ///
    /// # CEL Syntax Impact
    ///
    /// When a container is set, unqualified names in CEL expressions are automatically prefixed
    /// with the container namespace:
    ///
    /// ```cel
    /// // With container "my.app", this expression:
    /// MyMessage{field: 123}
    /// // is equivalent to:
    /// my.app.MyMessage{field: 123}
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// // Set container for protobuf message resolution
    /// let env = Env::builder()
    ///     .with_container("com.example.proto")
    ///     .build()?;
    /// 
    /// // Now "UserMessage" resolves to "com.example.proto.UserMessage"
    /// let program = env.compile("UserMessage{name: 'Alice', id: 123}")?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn with_container(mut self, container: impl Into<String>) -> Self {
        self.options.container = container.into();
        self
    }

    /// Enables or disables the CEL standard library of functions and macros.
    ///
    /// **Default**: Enabled (`true`)
    ///
    /// The standard library provides a rich set of common functions for types like `string`,
    /// `list`, `map`, as well as logical and arithmetic operators. Disabling it can reduce 
    /// the environment's footprint if only custom functions are needed.
    ///
    /// # CEL Syntax Impact
    ///
    /// When enabled, provides access to all standard CEL operations:
    /// - **Arithmetic**: `+`, `-`, `*`, `/`, `%`
    /// - **Comparison**: `==`, `!=`, `<`, `<=`, `>`, `>=`
    /// - **Logical**: `&&`, `||`, `!`
    /// - **String operations**: `+` (concatenation), `contains()`, `startsWith()`, `endsWith()`, `size()`
    /// - **List operations**: `size()`, `in`, `[]` (indexing), `+` (concatenation)
    /// - **Map operations**: `size()`, `in`, `[]` (key access), `+` (merge)
    /// - **Type conversions**: `int()`, `uint()`, `double()`, `string()`, `bytes()`
    /// - **Conditional**: `? :` (ternary operator)
    /// - **Macros**: `has()`, `all()`, `exists()`, `exists_one()`, `map()`, `filter()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// 
    /// // Standard library enabled (default)
    /// let env = Env::builder()
    ///     .with_standard(true)
    ///     .build()?;
    /// 
    /// // Can use standard functions
    /// let program = env.compile("'hello'.size() + ' world'.size() == 11")?;
    /// let result = program.evaluate(&Activation::new())?;
    /// assert_eq!(result, Value::Bool(true));
    /// 
    /// // Standard library disabled
    /// let env_minimal = Env::builder()
    ///     .with_standard(false)
    ///     .build()?;
    /// 
    /// // Standard functions not available - would cause compilation error
    /// // env_minimal.compile("'hello'.size()")?; // Error!
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn with_standard(mut self, enable: bool) -> Self {
        self.options.enable_standard = enable;
        self
    }

    /// Enables or disables support for CEL's optional types and related syntax.
    ///
    /// **Default**: Disabled (`false`)
    ///
    /// This enables the `optional` type and related features like optional field selection (`.?`),
    /// optional index/key access (`[?_]`), and optional value construction (`{?key: ...}`).
    /// Required for some extensions like regex that return optional values.
    ///
    /// # CEL Syntax Impact
    ///
    /// When enabled, adds support for:
    /// - **Optional type**: `optional<T>` for values that may or may not be present
    /// - **Optional field selection**: `msg.?field` returns `optional<T>` instead of error
    /// - **Optional indexing**: `list[?index]` and `map[?key]` return `optional<T>`
    /// - **Optional map construction**: `{?'key': value}` only includes entry if value is present
    /// - **Optional methods**: `.hasValue()`, `.value()`, `.orValue(default)`
    /// - **Optional literals**: `optional.of(value)`, `optional.none()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .with_optional(true)
    ///     .build()?;
    /// 
    /// // Optional field selection
    /// let program = env.compile("msg.?field.orValue('default')")?;
    /// 
    /// // Optional map construction
    /// let program2 = env.compile("{'name': 'bob', ?'age': optional.of(25)}")?;
    /// 
    /// // Optional indexing
    /// let program3 = env.compile("list[?5].hasValue()")?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn with_optional(mut self, enable: bool) -> Self {
        self.options.enable_optional = enable;
        self
    }

    /// Enables or disables the Bindings extension.
    ///
    /// **Default**: Disabled (`false`)
    ///
    /// This extension provides the `cel.bind()` macro, which allows for temporary variable
    /// bindings within a CEL expression to improve readability and performance by avoiding
    /// repeated calculations.
    ///
    /// # Available Functions
    ///
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `cel.bind(var, init, result)` | Bind variable to initialization expression | `cel.bind(x, 5, x * x)` |
    ///
    /// # CEL Syntax Impact
    ///
    /// When enabled, adds the `cel.bind()` macro that creates local variable scopes:
    /// - Variables are scoped to the result expression
    /// - Supports nested bindings
    /// - Enables performance optimization through value reuse
    /// - Improves readability of complex expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .with_ext_bindings(true)
    ///     .build()?;
    /// 
    /// // Simple binding
    /// let program = env.compile("cel.bind(x, 5, x * x)")?;
    /// let result = program.evaluate(&Activation::new())?;
    /// assert_eq!(result, Value::Int(25));
    /// 
    /// // Nested bindings for complex calculations
    /// let program2 = env.compile(r#"
    ///     cel.bind(a, 'hello',
    ///       cel.bind(b, 'world', 
    ///         a + ' ' + b + '!'))
    /// "#)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn with_ext_bindings(mut self, enable: bool) -> Self {
        self.options.enable_ext_bindings = enable;
        self
    }

    /// Enables or disables the Comprehensions extension.
    ///
    /// **Default**: Disabled (`false`)
    ///
    /// This extension provides support for two-variable comprehensions, which are a way to iterate
    /// over a collection and perform an operation on each element.
    ///
    /// The two-variable form of comprehensions looks similar to the one-variable counterparts.
    /// Where possible, the same macro names were used and additional macro signatures added.
    /// The notable distinction for two-variable comprehensions is the introduction of
    /// `transformList`, `transformMap`, and `transformMapEntry` support for list and map types
    /// rather than the more traditional `map` and `filter` macros.
    ///
    /// # All
    ///
    /// Comprehension which tests whether all elements in the list or map satisfy a given
    /// predicate. The `all` macro evaluates in a manner consistent with logical AND and will
    /// short-circuit when encountering a `false` value.
    ///
    /// ```cel
    /// <list>.all(indexVar, valueVar, <predicate>) -> bool
    /// <map>.all(keyVar, valueVar, <predicate>) -> bool
    /// ```
    ///
    /// Examples:
    ///
    /// ```cel
    /// [1, 2, 3].all(i, j, i < j) // returns true
    /// {'hello': 'world', 'taco': 'taco'}.all(k, v, k != v) // returns false
    ///
    /// // Combines two-variable comprehension with single variable
    /// {'h': ['hello', 'hi'], 'j': ['joke', 'jog']}
    ///     .all(k, vals, vals.all(v, v.startsWith(k))) // returns true
    /// ```
    ///
    /// # Exists
    ///
    /// Comprehension which tests whether any element in a list or map exists which satisfies
    /// a given predicate. The `exists` macro evaluates in a manner consistent with logical OR
    /// and will short-circuit when encountering a `true` value.
    ///
    /// ```cel
    /// <list>.exists(indexVar, valueVar, <predicate>) -> bool
    /// <map>.exists(keyVar, valueVar, <predicate>) -> bool
    /// ```
    ///
    /// Examples:
    ///
    /// ```cel
    /// {'greeting': 'hello', 'farewell': 'goodbye'}
    ///     .exists(k, v, k.startsWith('good') || v.endsWith('bye')) // returns true
    /// [1, 2, 4, 8, 16].exists(i, v, v == 1024 && i == 10) // returns false
    /// ```
    ///
    /// # ExistsOne
    ///
    /// Comprehension which tests whether exactly one element in a list or map exists which
    /// satisfies a given predicate expression. This comprehension does not short-circuit in
    /// keeping with the one-variable exists one macro semantics.
    ///
    /// ```cel
    /// <list>.existsOne(indexVar, valueVar, <predicate>)
    /// <map>.existsOne(keyVar, valueVar, <predicate>)
    /// ```
    ///
    /// This macro may also be used with the `exists_one` function name, for compatibility
    /// with the one-variable macro of the same name.
    ///
    /// Examples:
    ///
    /// ```cel
    /// [1, 2, 1, 3, 1, 4].existsOne(i, v, i == 1 || v == 1) // returns false
    /// [1, 1, 2, 2, 3, 3].existsOne(i, v, i == 2 && v == 2) // returns true
    /// {'i': 0, 'j': 1, 'k': 2}.existsOne(i, v, i == 'l' || v == 1) // returns true
    /// ```
    ///
    /// # TransformList
    ///
    /// Comprehension which converts a map or a list into a list value. The output expression
    /// of the comprehension determines the contents of the output list. Elements in the list
    /// may optionally be filtered according to a predicate expression, where elements that
    /// satisfy the predicate are transformed.
    ///
    /// ```cel
    /// <list>.transformList(indexVar, valueVar, <transform>)
    /// <list>.transformList(indexVar, valueVar, <filter>, <transform>)
    /// <map>.transformList(keyVar, valueVar, <transform>)
    /// <map>.transformList(keyVar, valueVar, <filter>, <transform>)
    /// ```
    ///
    /// Examples:
    ///
    /// ```cel
    /// [1, 2, 3].transformList(indexVar, valueVar,
    ///   (indexVar * valueVar) + valueVar) // returns [1, 4, 9]
    /// [1, 2, 3].transformList(indexVar, valueVar, indexVar % 2 == 0,
    ///   (indexVar * valueVar) + valueVar) // returns [1, 9]
    /// {'greeting': 'hello', 'farewell': 'goodbye'}
    ///   .transformList(k, _, k) // returns ['greeting', 'farewell']
    /// {'greeting': 'hello', 'farewell': 'goodbye'}
    ///   .transformList(_, v, v) // returns ['hello', 'goodbye']
    /// ```
    ///
    /// # TransformMap
    ///
    /// Comprehension which converts a map or a list into a map value. The output expression
    /// of the comprehension determines the value of the output map entry; however, the key
    /// remains fixed. Elements in the map may optionally be filtered according to a predicate
    /// expression, where elements that satisfy the predicate are transformed.
    ///
    /// ```cel
    /// <list>.transformMap(indexVar, valueVar, <transform>)
    /// <list>.transformMap(indexVar, valueVar, <filter>, <transform>)
    /// <map>.transformMap(keyVar, valueVar, <transform>)
    /// <map>.transformMap(keyVar, valueVar, <filter>, <transform>)
    /// ```
    ///
    /// Examples:
    ///
    /// ```cel
    /// [1, 2, 3].transformMap(indexVar, valueVar,
    ///   (indexVar * valueVar) + valueVar) // returns {0: 1, 1: 4, 2: 9}
    /// [1, 2, 3].transformMap(indexVar, valueVar, indexVar % 2 == 0,
    ///   (indexVar * valueVar) + valueVar) // returns {0: 1, 2: 9}
    /// {'greeting': 'hello'}.transformMap(k, v, v + '!') // returns {'greeting': 'hello!'}
    /// ```
    ///
    /// # TransformMapEntry
    ///
    /// Comprehension which converts a map or a list into a map value; however, this transform
    /// expects the entry expression be a map literal. If the tranform produces an entry which
    /// duplicates a key in the target map, the comprehension will error.  Note, that key
    /// equality is determined using CEL equality which asserts that numeric values which are
    /// equal, even if they don't have the same type will cause a key collision.
    ///
    /// Elements in the map may optionally be filtered according to a predicate expression, where
    /// elements that satisfy the predicate are transformed.
    ///
    /// ```cel
    /// <list>.transformMapEntry(indexVar, valueVar, <transform>)
    /// <list>.transformMapEntry(indexVar, valueVar, <filter>, <transform>)
    /// <map>.transformMapEntry(keyVar, valueVar, <transform>)
    /// <map>.transformMapEntry(keyVar, valueVar, <filter>, <transform>)
    /// ```
    ///
    /// Examples:
    ///
    /// ```cel
    /// // returns {'hello': 'greeting'}
    /// {'greeting': 'hello'}.transformMapEntry(keyVar, valueVar, {valueVar: keyVar})
    /// // reverse lookup, require all values in list be unique
    /// [1, 2, 3].transformMapEntry(indexVar, valueVar, {valueVar: indexVar})
    ///
    /// {'greeting': 'aloha', 'farewell': 'aloha'}
    ///   .transformMapEntry(keyVar, valueVar, {valueVar: keyVar}) // error, duplicate key
    /// ```
    pub fn with_ext_comprehensions(mut self, enable: bool) -> Self {
        self.options.enable_ext_comprehensions = enable;
        self
    }

    /// Enables or disables the Encoders extension.
    ///
    /// **Default**: Disabled (`false`)
    ///
    /// This extension provides functions for encoding and decoding between common data formats,
    /// such as Base64. All functions handle edge cases gracefully and maintain CEL's safety guarantees.
    ///
    /// # Available Functions
    ///
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `base64.encode(bytes)` | Encode bytes to Base64 string | `base64.encode(b'hello')` |
    /// | `base64.decode(string)` | Decode Base64 string to bytes | `base64.decode('aGVsbG8=')` |
    ///
    /// # CEL Syntax Impact
    ///
    /// When enabled, adds encoding/decoding functions in the `base64` namespace:
    /// - Supports both standard and raw (unpadded) Base64 encoding
    /// - Automatically handles missing padding in decode operations
    /// - Returns errors for invalid Base64 input
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .with_ext_encoders(true)
    ///     .build()?;
    /// 
    /// // Encode bytes to Base64
    /// let program = env.compile("base64.encode(b'hello')")?;
    /// let result = program.evaluate(&Activation::new())?;
    /// assert_eq!(result, Value::String("aGVsbG8=".into()));
    /// 
    /// // Decode Base64 to bytes
    /// let program2 = env.compile("base64.decode('aGVsbG8=')")?;
    /// let result2 = program2.evaluate(&Activation::new())?;
    /// assert_eq!(result2, Value::Bytes(b"hello".to_vec()));
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn with_ext_encoders(mut self, enable: bool) -> Self {
        self.options.enable_ext_encoders = enable;
        self
    }

    /// Enables or disables the Lists extension.
    ///
    /// **Default**: Disabled (`false`)
    ///
    /// This extension provides additional functions for working with lists, such as slicing,
    /// flattening, sorting, and deduplication. All functions maintain CEL's immutability 
    /// guarantees and return new lists rather than modifying existing ones.
    ///
    /// # Available Functions
    ///
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `list.slice(start, end)` | Extract sub-list | `[1,2,3,4].slice(1,3)` → `[2,3]` |
    /// | `list.flatten()` | Flatten nested lists | `[[1,2],[3,4]].flatten()` → `[1,2,3,4]` |
    /// | `list.flatten(depth)` | Flatten to specified depth | `[1,[2,[3]]].flatten(1)` → `[1,2,[3]]` |
    /// | `list.distinct()` | Remove duplicates | `[1,2,2,3].distinct()` → `[1,2,3]` |
    /// | `list.reverse()` | Reverse list order | `[1,2,3].reverse()` → `[3,2,1]` |
    /// | `list.sort()` | Sort comparable elements | `[3,1,2].sort()` → `[1,2,3]` |
    /// | `list.sortBy(var, expr)` | Sort by key expression | `users.sortBy(u, u.age)` |
    /// | `lists.range(n)` | Generate number sequence | `lists.range(3)` → `[0,1,2]` |
    /// | `lists.range(start, end)` | Generate range | `lists.range(2,5)` → `[2,3,4]` |
    ///
    /// # CEL Syntax Impact
    ///
    /// When enabled, adds advanced list manipulation capabilities:
    /// - Zero-based indexing for all operations
    /// - Type safety for sort operations (comparable types only)
    /// - Efficient deduplication and flattening algorithms
    /// - Lazy evaluation for range generation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .with_ext_lists(true)
    ///     .build()?;
    /// 
    /// // List slicing
    /// let program = env.compile("[1, 2, 3, 4].slice(1, 3)")?;
    /// let result = program.evaluate(&Activation::new())?;
    /// assert_eq!(result, Value::List(vec![Value::Int(2), Value::Int(3)]));
    /// 
    /// // List sorting
    /// let program2 = env.compile("[3, 1, 2].sort()")?;
    /// let result2 = program2.evaluate(&Activation::new())?;
    /// assert_eq!(result2, Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]));
    /// 
    /// // Generate ranges
    /// let program3 = env.compile("lists.range(3)")?;
    /// let result3 = program3.evaluate(&Activation::new())?;
    /// assert_eq!(result3, Value::List(vec![Value::Int(0), Value::Int(1), Value::Int(2)]));
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn with_ext_lists(mut self, enable: bool) -> Self {
        self.options.enable_ext_lists = enable;
        self
    }

    /// Enables or disables the Math extension.
    ///
    /// **Default**: Disabled (`false`)
    ///
    /// This extension provides advanced mathematical functions beyond the standard operators,
    /// including min/max operations, rounding functions, absolute value, sign detection,
    /// bitwise operations, floating point helpers, and square root. All functions are 
    /// deterministic and side-effect free.
    ///
    /// **Note**: All macros use the 'math' namespace; however, at the time of macro
    /// expansion the namespace looks just like any other identifier. If you are
    /// currently using a variable named 'math', the macro will likely work just as
    /// intended; however, there is some chance for collision.
    ///
    /// # Available Functions
    ///
         /// ## Min/Max Operations
     /// | Function | Description | Example |
     /// |----------|-------------|---------|
     /// | `math.greatest(...)` | Greatest value from arguments/list | `math.greatest(1,2,3)` → `3` |
     /// | `math.least(...)` | Least value from arguments/list | `math.least([1,2,3])` → `1` |
    ///
    /// ## Absolute Value and Sign
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `math.abs(number)` | Absolute value | `math.abs(-5)` → `5` |
    /// | `math.sign(number)` | Sign (-1, 0, or 1) | `math.sign(-5)` → `-1` |
    ///
    /// ## Rounding Functions
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `math.ceil(number)` | Round up | `math.ceil(3.14)` → `4.0` |
    /// | `math.floor(number)` | Round down | `math.floor(3.14)` → `3.0` |
    /// | `math.round(number)` | Round to nearest | `math.round(3.14)` → `3.0` |
    /// | `math.trunc(number)` | Truncate decimals | `math.trunc(3.14)` → `3.0` |
    ///
    /// ## Bitwise Operations
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `math.bitAnd(a,b)` | Bitwise AND | `math.bitAnd(5,3)` → `1` |
    /// | `math.bitOr(a,b)` | Bitwise OR | `math.bitOr(5,3)` → `7` |
    /// | `math.bitXor(a,b)` | Bitwise XOR | `math.bitXor(5,3)` → `6` |
    /// | `math.bitNot(n)` | Bitwise NOT | `math.bitNot(5)` → `-6` |
    /// | `math.bitShiftLeft(n,bits)` | Left bit shift | `math.bitShiftLeft(5,1)` → `10` |
    /// | `math.bitShiftRight(n,bits)` | Right bit shift | `math.bitShiftRight(5,1)` → `2` |
    ///
    /// ## Floating Point Helpers
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `math.isInf(number)` | Check if infinite | `math.isInf(1.0/0.0)` → `true` |
    /// | `math.isNaN(number)` | Check if NaN | `math.isNaN(0.0/0.0)` → `true` |
    /// | `math.isFinite(number)` | Check if finite | `math.isFinite(1.2)` → `true` |
    ///
    /// ## Square Root
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `math.sqrt(number)` | Square root | `math.sqrt(81)` → `9.0` |
    ///
    /// # CEL Syntax Impact
    ///
    /// When enabled, adds mathematical functions in the `math` namespace:
    /// - Supports both integer and floating-point operations
    /// - Bitwise operations work on integer types only
    /// - Rounding functions return double type
    /// - Min/max functions preserve input type
    /// - Floating point helpers work with double type
    /// - Square root always returns double type
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .with_ext_math(true)
    ///     .build()?;
    /// 
    /// // Greatest/least operations (macros)
    /// let program = env.compile("math.greatest(5, 2, 8, 1)")?;
    /// let result = program.evaluate(&Activation::new())?;
    /// assert_eq!(result, Value::Int(8));
    /// 
    /// let program2 = env.compile("math.least([-42.0, -21.5, -100.0])")?;
    /// let result2 = program2.evaluate(&Activation::new())?;
    /// assert_eq!(result2, Value::Double(-100.0));
    /// 
         /// // Absolute value
     /// let program3 = env.compile("math.abs(-5)")?;
     /// let result3 = program3.evaluate(&Activation::new())?;
     /// assert_eq!(result3, Value::Int(5));
    /// 
    /// // Rounding functions
    /// let program4 = env.compile("math.ceil(3.14)")?;
    /// let result4 = program4.evaluate(&Activation::new())?;
    /// assert_eq!(result4, Value::Double(4.0));
    /// 
    /// // Bitwise operations
    /// let program5 = env.compile("math.bitAnd(5, 3)")?;
    /// let result5 = program5.evaluate(&Activation::new())?;
    /// assert_eq!(result5, Value::Int(1));
    /// 
    /// // Floating point helpers
    /// let program6 = env.compile("math.isFinite(1.2)")?;
    /// let result6 = program6.evaluate(&Activation::new())?;
    /// assert_eq!(result6, Value::Bool(true));
    /// 
    /// // Square root
    /// let program7 = env.compile("math.sqrt(81)")?;
    /// let result7 = program7.evaluate(&Activation::new())?;
    /// assert_eq!(result7, Value::Double(9.0));
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn with_ext_math(mut self, enable: bool) -> Self {
        self.options.enable_ext_math = enable;
        self
    }

    /// Enables or disables the Protocol Buffers (Protobuf) extension.
    ///
    /// **Default**: Disabled (`false`)
    ///
    /// This provides enhanced support for working with Protocol Buffer messages, particularly
    /// for accessing and testing proto2 extension fields. Requires proper setup of Protobuf 
    /// descriptors in the environment.
    ///
    /// # Available Functions
    ///
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `proto.getExt(msg, ext)` | Get extension field value | `proto.getExt(msg, my.extension)` |
    /// | `proto.hasExt(msg, ext)` | Test extension field presence | `proto.hasExt(msg, my.extension)` |
    ///
    /// # CEL Syntax Impact
    ///
    /// When enabled, adds proto2 extension support:
    /// - `proto.getExt()` returns extension value or default if not set
    /// - `proto.hasExt()` returns boolean indicating if extension is explicitly set
    /// - Extension names must be fully qualified (e.g., `com.example.my_extension`)
    /// - Uses safe-traversal semantics (no errors on missing fields)
    /// - Only works with proto2 syntax messages that support extensions
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .with_ext_proto(true)
    ///     .build()?;
    /// 
    /// // Access extension field
    /// let program = env.compile("proto.getExt(my_message, com.example.priority_ext)")?;
    /// 
    /// // Test extension presence
    /// let program2 = env.compile("proto.hasExt(my_message, com.example.priority_ext)")?;
    /// 
    /// // Conditional processing based on extensions
    /// let program3 = env.compile(r#"
    ///     proto.hasExt(msg, com.example.metadata_ext) ? 
    ///         proto.getExt(msg, com.example.metadata_ext).value : 
    ///         "default"
    /// "#)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn with_ext_proto(mut self, enable: bool) -> Self {
        self.options.enable_ext_proto = enable;
        self
    }

    /// Enables or disables the Regular Expression (Regex) extension.
    ///
    /// **Default**: Disabled (`false`)
    ///
    /// This extension provides functions for pattern matching on strings using regular expressions,
    /// including pattern extraction, replacement, and text processing. Requires optional types
    /// to be enabled for proper operation.
    ///
    /// # Available Functions
    ///
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `regex.extract(text, pattern)` | Extract first match (optional) | `regex.extract('hello', 'h(.*)o')` → `optional('ell')` |
    /// | `regex.extractAll(text, pattern)` | Extract all matches | `regex.extractAll('a1 b2', '\\d+')` → `['1', '2']` |
    /// | `regex.replace(text, pattern, replacement)` | Replace all matches | `regex.replace('hello', 'l', 'x')` → `'hexxo'` |
    /// | `regex.replace(text, pattern, replacement, count)` | Replace up to count | `regex.replace('hello', 'l', 'x', 1)` → `'hexlo'` |
    ///
    /// # CEL Syntax Impact
    ///
    /// When enabled, adds regex functions in the `regex` namespace:
    /// - `regex.extract()` returns `optional<string>` (requires optional types)
    /// - Supports 0 or 1 capture groups only (error for multiple groups)
    /// - Uses standard regex syntax with proper escaping
    /// - Replacement supports capture group references (`\1`, `\2`, etc.)
    /// - Count parameter in replace: 0=no replacement, negative=replace all
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .with_ext_regex(true)
    ///     .with_optional(true)  // Required for regex.extract
    ///     .build()?;
    /// 
    /// // Pattern extraction
    /// let program = env.compile(r#"regex.extract('hello world', 'hello (.*)')"#)?;
    /// 
    /// // Extract all matches
    /// let program2 = env.compile(r#"regex.extractAll('id:123, id:456', 'id:(\\d+)')"#)?;
    /// 
    /// // Pattern replacement with capture groups
    /// let program3 = env.compile(r#"regex.replace('John Doe', '(\\w+) (\\w+)', r'\2, \1')"#)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn with_ext_regex(mut self, enable: bool) -> Self {
        self.options.enable_ext_regex = enable;
        self
    }

    /// Enables or disables the Regular Expression (RE) extension.
    ///
    /// **Default**: Disabled (`false`)
    ///
    /// This extension provides C++ specific regular expression functions built on the RE2 library,
    /// offering additional pattern matching capabilities with different semantics than the standard
    /// regex extension. This is specific to the C++ CEL implementation.
    ///
    /// # Available Functions
    ///
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `re.extract(text, pattern, rewrite)` | Extract and rewrite with pattern | `re.extract('Hello World', r'(\\w+) (\\w+)', r'\\2, \\1')` |
    /// | `re.capture(text, pattern)` | Capture first group | `re.capture('john@example.com', r'([^@]+)@')` |
    /// | `re.captureN(text, pattern)` | Capture all groups as map | `re.captureN('2023-12-25', r'(\\d{4})-(\\d{2})-(\\d{2})')` |
    ///
    /// # CEL Syntax Impact
    ///
    /// When enabled, adds RE2-based regex functions in the `re` namespace:
    /// - `re.extract()` performs extraction and rewriting in one operation
    /// - `re.capture()` returns string of first capture group
    /// - `re.captureN()` returns map with numbered/named capture groups
    /// - Uses RE2 library for consistent performance and safety
    /// - Supports named capture groups in `captureN()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .with_ext_re(true)
    ///     .build()?;
    /// 
    /// // Extract and rewrite
    /// let program = env.compile(r#"re.extract('Hello World', r'(\w+) (\w+)', r'\2, \1')"#)?;
    /// 
    /// // Capture first group
    /// let program2 = env.compile(r#"re.capture('john@example.com', r'([^@]+)@')"#)?;
    /// 
    /// // Capture all groups
    /// let program3 = env.compile(r#"re.captureN('2023-12-25', r'(\d{4})-(\d{2})-(\d{2})')"#)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn with_ext_re(mut self, enable: bool) -> Self {
        self.options.enable_ext_re = enable;
        self
    }

    /// Enables or disables the Sets extension.
    ///
    /// **Default**: Disabled (`false`)
    ///
    /// This extension provides functions for set-based operations on lists, such as containment
    /// checking, equivalence testing, and intersection detection. Note that CEL does not have 
    /// a native `set` type; these functions treat lists as sets.
    ///
    /// # Available Functions
    ///
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `sets.contains(list1, list2)` | Check if list1 contains all elements of list2 | `sets.contains([1,2,3], [2,3])` → `true` |
    /// | `sets.equivalent(list1, list2)` | Check if lists are set equivalent | `sets.equivalent([1,2,3], [3,2,1])` → `true` |
    /// | `sets.intersects(list1, list2)` | Check if lists have common elements | `sets.intersects([1,2], [2,3])` → `true` |
    ///
    /// # CEL Syntax Impact
    ///
    /// When enabled, adds set operations in the `sets` namespace:
    /// - Treats lists as sets (order and duplicates don't matter for equivalence)
    /// - Uses standard CEL equality for element comparison
    /// - Supports type coercion (e.g., `1`, `1.0`, `1u` are considered equal)
    /// - Empty list operations: `contains([], [])` → `true`, `intersects([], [])` → `false`
    /// - Works with any comparable types
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .with_ext_sets(true)
    ///     .build()?;
    /// 
    /// // Set containment
    /// let program = env.compile("sets.contains([1, 2, 3, 4], [2, 3])")?;
    /// let result = program.evaluate(&Activation::new())?;
    /// assert_eq!(result, Value::Bool(true));
    /// 
    /// // Set equivalence
    /// let program2 = env.compile("sets.equivalent([1, 2, 3], [3, 2, 1])")?;
    /// let result2 = program2.evaluate(&Activation::new())?;
    /// assert_eq!(result2, Value::Bool(true));
    /// 
    /// // Set intersection
    /// let program3 = env.compile("sets.intersects([1, 2], [2, 3])")?;
    /// let result3 = program3.evaluate(&Activation::new())?;
    /// assert_eq!(result3, Value::Bool(true));
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn with_ext_sets(mut self, enable: bool) -> Self {
        self.options.enable_ext_sets = enable;
        self
    }

    /// Enables or disables the Strings extension.
    ///
    /// **Default**: Disabled (`false`)
    ///
    /// This extension provides additional functions for string manipulation, including character
    /// access, searching, extraction, case conversion, formatting, and advanced text processing
    /// operations that go beyond the basic string operations in the standard library.
    ///
    /// # Available Functions
    ///
    /// | Function | Description | Example |
    /// |----------|-------------|---------|
    /// | `string.charAt(index)` | Get character at index | `'hello'.charAt(1)` → `'e'` |
    /// | `string.indexOf(substring)` | Find first occurrence | `'hello'.indexOf('l')` → `2` |
    /// | `string.indexOf(substring, start)` | Find from start position | `'hello'.indexOf('l', 3)` → `3` |
    /// | `string.lastIndexOf(substring)` | Find last occurrence | `'hello'.lastIndexOf('l')` → `3` |
    /// | `string.substring(start)` | Extract from start to end | `'hello'.substring(1)` → `'ello'` |
    /// | `string.substring(start, end)` | Extract substring | `'hello'.substring(1, 4)` → `'ell'` |
    /// | `strings.quote(string)` | Quote string for CEL | `strings.quote('hello')` → `'"hello"'` |
    /// | `string.trim()` | Remove whitespace | `' hello '.trim()` → `'hello'` |
    /// | `list.join(separator)` | Join strings | `['a','b'].join(',')` → `'a,b'` |
    /// | `string.split(separator)` | Split string | `'a,b,c'.split(',')` → `['a','b','c']` |
    /// | `string.lowerAscii()` | Convert to lowercase | `'HELLO'.lowerAscii()` → `'hello'` |
    /// | `string.upperAscii()` | Convert to uppercase | `'hello'.upperAscii()` → `'HELLO'` |
    /// | `string.replace(old, new)` | Replace all occurrences | `'hello'.replace('l','x')` → `'hexxo'` |
    /// | `string.replace(old, new, count)` | Replace up to count | `'hello'.replace('l','x',1)` → `'hexlo'` |
    /// | `string.format(args)` | Printf-style formatting | `'Hello %s'.format(['World'])` → `'Hello World'` |
    /// | `string.reverse()` | Reverse string | `'hello'.reverse()` → `'olleh'` |
    ///
    /// # CEL Syntax Impact
    ///
    /// When enabled, adds advanced string manipulation capabilities:
    /// - Zero-based indexing for character access and substring operations
    /// - Safe out-of-bounds handling (empty string for invalid indices)
    /// - ASCII-only case conversion (Unicode characters unchanged)
    /// - Printf-style format placeholders: `%s`, `%d`, `%f`, `%.Nf`
    /// - Efficient string processing with immutable operations
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .with_ext_strings(true)
    ///     .build()?;
    /// 
    /// // String searching and extraction
    /// let program = env.compile("'hello world'.substring('hello world'.indexOf(' ') + 1)")?;
    /// let result = program.evaluate(&Activation::new())?;
    /// assert_eq!(result, Value::String("world".into()));
    /// 
    /// // String formatting
    /// let program2 = env.compile("'Hello, %s!'.format(['Alice'])")?;
    /// let result2 = program2.evaluate(&Activation::new())?;
    /// assert_eq!(result2, Value::String("Hello, Alice!".into()));
    /// 
    /// // String processing pipeline
    /// let program3 = env.compile("'  HELLO WORLD  '.trim().lowerAscii().replace(' ', '_')")?;
    /// let result3 = program3.evaluate(&Activation::new())?;
    /// assert_eq!(result3, Value::String("hello_world".into()));
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn with_ext_strings(mut self, enable: bool) -> Self {
        self.options.enable_ext_strings = enable;
        self
    }

    /// Enables or disables the select optimization extension.
    ///
    /// **Default**: Disabled (`false`)
    ///
    /// This is an optimization that can improve the performance of `select` expressions
    /// (field access) by transforming them at compile time. It does not introduce new
    /// user-visible functions but can change the evaluation cost of field access operations.
    ///
    /// # CEL Syntax Impact
    ///
    /// When enabled, provides compile-time optimizations for:
    /// - Message field access operations
    /// - Map key access patterns
    /// - Nested field selection chains
    /// - No new syntax or functions are added
    /// - Transparent performance improvements
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .with_ext_select_optimization(true)
    ///     .build()?;
    /// 
    /// // Field access operations may be optimized
    /// let program = env.compile("user.profile.settings.theme")?;
    /// 
    /// // Map access patterns may be optimized  
    /// let program2 = env.compile("config['database']['host']")?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
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
