//! Function registration and implementation utilities.
//!
//! The function system provides a flexible way to register and call functions in CEL expressions.
//! Functions can be either compile-time declarations (type signatures only) or runtime
//! implementations (callable code).
//!
//! # Key Components
//!
//! - [`FunctionRegistry`]: Compile-time function registry for declaring function signatures and registering implementations
//! - [`FunctionBindings`]: Runtime function bindings for calling functions during evaluation
//! - [`FunctionOverloads`]: Function overload management supporting multiple implementations with different signatures
//! - **Declarations**: Use [`FunctionDecl`] trait for compile-time type checking
//! - **Implementations**: Use [`IntoFunction`] trait for runtime function calls
//!
//! # Examples
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! // Register a function implementation
//! let mut env = Env::builder()
//!     .register_global_function("greet", |name: String| -> String {
//!         format!("Hello, {}!", name)
//!     })?
//!     .build()?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! # Detailed Documentation
//!
//! Zero-annotation function registration for CEL expression evaluation.
//!
//! This module provides a type-safe, zero-annotation function registration system
//! that allows Rust functions to be called from CEL expressions without manual
//! type annotations or wrapper code.
//!
//! # Two Function Systems
//!
//! This module provides two distinct but complementary function systems:
//!
//! ## 1. Function Registration (Runtime Implementation)
//! - **Purpose**: Register actual callable Rust functions/closures
//! - **Entry point**: [`IntoFunction`] trait and registration methods
//! - **Usage**: `env.register_function("name", function_impl)`
//! - **Provides**: Executable code that can be called during expression evaluation
//!
//! ## 2. Function Declaration (Compile-Time Signatures)
//! - **Purpose**: Declare function signatures for type checking without implementation
//! - **Entry point**: [`FunctionDecl`] trait and declaration methods  
//! - **Usage**: `env.declare_function::<SignatureType>("name")`
//! - **Provides**: Type information for compile-time validation and overload resolution
//!
//! These systems work together: you can declare functions for type checking during
//! development, then provide implementations later, or register complete functions
//! that include both signature and implementation.
//!
//! # Features
//!
//! - **Zero-annotation registration**: Functions can be registered without explicit type annotations
//! - **Lifetime-aware closures**: Support for closures that capture environment variables
//! - **Reference return types**: Safe handling of functions returning borrowed data like `&str`
//! - **Unified error handling**: Automatic conversion of `Result<T, E>` return types
//! - **Async function support**: Optional support for async functions (requires `async` feature)
//! - **Thread safety**: All function implementations are `Send + Sync`
//!
//! # How Zero-Annotation Works
//!
//! The zero-annotation system is built on top of Rust's type system and Generic Associated Types (GATs).
//! When you register a function, the system automatically:
//!
//! 1. **Extracts argument types** from the function signature using [`FunctionDecl`]
//! 2. **Infers return types** using the [`IntoResult`] trait
//! 3. **Generates type-safe converters** that handle lifetime erasure safely
//! 4. **Creates a unified interface** through the [`Function`] struct
//!
//! ## Type Conversion Process
//!
//! For each argument type `T`, the system:
//! - Uses `T: FromValue + TypedValue` to convert from CEL values
//! - Leverages GATs (`FromValue::Output<'a>`) to handle borrowed data like `&str`
//! - Safely handles lifetime relationships through internal conversion mechanisms
//!
//! ## Safety Guarantees
//!
//! The lifetime handling is safe because:
//! - Source CEL values remain valid for the entire function call
//! - Converted arguments are immediately consumed by the target function
//! - No references escape the function call scope
//!
//! # Examples
//!
//! ## Basic function registration
//!
//! ```rust,no_run
//! use cel_cxx::{function::IntoFunction, Error};
//!
//! // Simple function
//! fn add(a: i64, b: i64) -> i64 {
//!     a + b
//! }
//! let func = add.into_function();
//!
//! // Function with error handling
//! fn divide(a: i64, b: i64) -> Result<i64, Error> {
//!     if b == 0 {
//!         Err(Error::invalid_argument("division by zero"))
//!     } else {
//!         Ok(a / b)
//!     }
//! }
//! let func = divide.into_function();
//! ```
//!
//! ## Advanced: Reference return types
//!
//! The system handles functions that return borrowed data:
//!
//! ```rust,no_run
//! use cel_cxx::function::*;
//! // Function returning borrowed data
//! fn get_first(items: Vec<&str>) -> &str {
//!     items.first().map_or("", |s| *s)
//! }
//! let func = get_first.into_function();
//!
//! // The system automatically handles the lifetime relationships
//! ```
//!
//! ## Closure registration
//!
//! ```rust,no_run
//! use cel_cxx::function::*;
//! // Capturing closure
//! let multiplier = 3;
//! let multiply = move |x: i64| -> i64 { x * multiplier };
//! let func = multiply.into_function();
//!
//! // String processing closure
//! let prefix = String::from("Hello, ");
//! let with_prefix = move |name: &str| -> String {
//!     format!("{}{}", prefix, name)
//! };
//! let func = with_prefix.into_function();
//! ```
//!
//! ## Function metadata and invocation
//!
//! ```rust,no_run
//! use cel_cxx::function::*;
//! fn add(a: i64, b: i64) -> i64 { a + b }
//! let func = add.into_function();
//!
//! // Get function metadata
//! let arg_types = func.arguments(); // Vec<ValueType>
//! let return_type = func.result();  // ValueType
//!
//! // Call the function (would need proper Value instances in real code)
//! // let args = vec![Value::from(10i64), Value::from(20i64)];
//! // let result = func.call(args);
//! ```
//!
//! # Async Functions
//!
//! When the `async` feature is enabled, you can register async functions:
//!
//! ```rust,no_run
//! # #[cfg(feature = "async")]
//! # use cel_cxx::function::*;
//! # async fn example() {
//! // Async function
//! async fn fetch_data(url: String) -> String {
//!     // Simulate async work
//!     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
//!     format!("Data from {}", url)
//! }
//!
//! let func = fetch_data.into_function();
//! // func.call() returns a Future that can be awaited
//! # }
//! ```

use crate::values::*;
use crate::types::*;
use crate::error::*;
use crate::maybe_future::*;
use crate::marker::*;
use std::sync::Arc;

pub mod decl;
pub use decl::*;

pub mod overload;
pub use overload::*;

mod registry;
pub use registry::*;

mod bindings;
pub use bindings::*;

/// Marker trait for function argument tuples.
///
/// This trait is automatically implemented for tuples of types that implement
/// [`FromValue`] and [`TypedValue`]. It serves as a constraint to ensure
/// type safety in function registration.
///
/// The trait supports function signatures with 0 to 10 parameters. Each parameter
/// type must be convertible from CEL values and have a known CEL type.
///
/// # Implementation Details
///
/// This trait is sealed and cannot be implemented outside this crate. It is
/// automatically implemented for valid argument tuple types through procedural
/// macros.
///
/// # Supported Argument Types
///
/// Any type that implements both [`FromValue`] and [`TypedValue`] can be used
/// as a function argument. This includes:
///
/// - **Primitive types**: `bool`, `i64`, `u64`, `f64`, `String`, `Vec<u8>`
/// - **Reference types**: `&str`, `&[u8]` (with proper lifetime handling)
/// - **Collection types**: `Vec<T>`, `HashMap<K, V>` where `T`, `K`, `V` are valid CEL types
/// - **Custom types**: Types that implement the required traits
///
/// # Note
///
/// This trait is sealed and cannot be implemented outside this crate.
/// It supports function signatures with 0 to 10 parameters.
pub trait Arguments: Sized + private::Sealed {}

/// Trait for types that can be converted into function implementations.
///
/// This is the main entry point for function registration. Any Rust function
/// or closure that meets the constraints can be converted into a [`Function`].
///
/// # Type System Integration
///
/// The trait uses Rust's type system to automatically:
/// - Extract function signatures using [`FunctionDecl`]
/// - Handle argument conversion using [`FromValue`] with GATs
/// - Manage return type conversion using [`IntoResult`]
/// - Support both synchronous and asynchronous functions
///
/// # Generic Associated Types (GATs)
///
/// This trait leverages GATs to handle complex lifetime relationships:
/// - Functions returning `&str` can borrow from input parameters
/// - Closures can capture environment variables with appropriate lifetimes
/// - The system maintains memory safety through controlled lifetime erasure
///
/// # Note
///
/// This trait is sealed and cannot be implemented outside this crate.
///
/// # Type Parameters
///
/// - `'f`: Lifetime parameter for captured data in closures
/// - `Fm`: Function marker (sync/async)
/// - `Args`: Argument tuple type
///
/// # Examples
///
/// ## Simple Functions
///
/// ```rust
/// # use cel_cxx::function::IntoFunction;
/// # use std::convert::Infallible;
/// fn add(a: i64, b: i64) -> i64 { a + b }
/// fn divide(a: i64, b: i64) -> Result<f64, Infallible> { 
///     Ok(a as f64 / b as f64) 
/// }
///
/// let add_func = add.into_function();
/// let div_func = divide.into_function();
/// ```
///
/// ## Closures with Captured Variables
///
/// ```rust
/// # use cel_cxx::function::IntoFunction;
/// let factor = 2.5;
/// let scale = move |x: f64| -> f64 { x * factor };
/// let scale_func = scale.into_function();
/// ```
///
/// ## Functions with Reference Parameters
///
/// ```rust
/// # use cel_cxx::function::IntoFunction;
/// fn process_text(text: &str, uppercase: bool) -> String {
///     if uppercase { text.to_uppercase() } else { text.to_lowercase() }
/// }
/// let process_func = process_text.into_function();
/// ```
pub trait IntoFunction<'f, Fm: FnMarker, Args = ()>: private::Sealed<Fm, Args> {
    /// Convert this function into a type-erased implementation.
    ///
    /// This method performs the conversion from a strongly-typed Rust function
    /// to a type-erased [`Function`] that can be called from CEL expressions.
    ///
    /// # Returns
    ///
    /// A [`Function`] that encapsulates the original function with:
    /// - Type-safe argument conversion
    /// - Return value conversion
    /// - Error handling
    /// - Async support (if applicable)
    ///
    /// # Performance
    ///
    /// The conversion is zero-cost at runtime. All type checking and conversion
    /// logic is generated at compile time.
    fn into_function(self) -> Function<'f>;
}

/// A type-erased function implementation that can be called from CEL expressions.
///
/// This is the main type used to store and invoke registered functions.
/// It provides a uniform interface for calling functions regardless of
/// their original signature.
///
/// # Design
///
/// The `Function` struct uses dynamic dispatch through trait objects to provide
/// a uniform interface while maintaining type safety. The original function's
/// type information is preserved through internal trait implementations.
///
/// # Memory Safety
///
/// Despite using type erasure, the system maintains complete memory safety:
/// - All conversions are checked at runtime
/// - Lifetime relationships are preserved where possible
/// - Reference parameters are handled safely through controlled lifetime management
///
/// # Performance
///
/// - Function calls involve minimal overhead (one virtual call + conversions)
/// - Argument validation is performed once per call
/// - Type conversions use zero-copy where possible
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// # use cel_cxx::function::*;
/// fn greet(name: &str) -> String {
///     format!("Hello, {}!", name)
/// }
///
/// let func = greet.into_function();
/// let args = vec!["World".into()];
/// let result = func.call(args);
/// ```
///
/// ## Metadata Inspection
///
/// ```rust
/// # use cel_cxx::function::*;
/// # fn greet(name: &str) -> String { format!("Hello, {}!", name) }
/// let func = greet.into_function();
///
/// // Inspect function signature
/// println!("Arguments: {:?}", func.arguments());
/// println!("Return type: {:?}", func.result());
/// println!("Function type: {:?}", func.function_type());
/// ```
#[derive(Debug, Clone)]
pub struct Function<'f> {
    inner: Arc<dyn ErasedFn + 'f>,
}

impl<'f> Function<'f> {
    /// Create a new function implementation from an `ErasedFn`.
    fn new(inner: impl ErasedFn + 'f) -> Self {
        Self { inner: Arc::new(inner) }
    }

    /// Call the function with the provided arguments.
    ///
    /// This method invokes the function with type-safe argument conversion
    /// and return value handling. It supports both synchronous and asynchronous
    /// functions through the [`MaybeFuture`] return type.
    ///
    /// # Arguments
    ///
    /// * `args` - Vector of [`Value`] arguments to pass to the function
    ///
    /// # Returns
    ///
    /// Returns a [`MaybeFuture`] that represents either an immediate result or a future:
    /// 
    /// - **Without `async` feature**: [`MaybeFuture`] is `Result<Value, Error>` - returns immediately
    /// - **With `async` feature**: [`MaybeFuture`] can be either:
    ///   - `MaybeFuture::Result(Result<Value, Error>)` for synchronous functions
    ///   - `MaybeFuture::Future(BoxFuture<Result<Value, Error>>)` for async functions
    ///
    /// # Type Safety
    ///
    /// The method performs runtime type checking to ensure:
    /// - Correct number of arguments is provided
    /// - Each argument can be converted to the expected parameter type
    /// - Return value conversion succeeds
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if:
    /// - The number of arguments doesn't match the function signature
    /// - Argument types cannot be converted to the expected types
    /// - The function itself returns an error
    /// - Return value conversion fails
    ///
    /// # Examples
    ///
    /// ## Synchronous function call
    ///
    /// ```rust
    /// # use cel_cxx::function::*;
    /// # use cel_cxx::values::Value;
    /// fn add(a: i64, b: i64) -> i64 { a + b }
    /// let func = add.into_function();
    ///
    /// let args = vec![Value::Int(10), Value::Int(20)];
    /// let maybe_result = func.call(args);
    /// 
    /// // In sync mode, extract the result directly
    /// # #[cfg(not(feature = "async"))]
    /// let result = maybe_result.unwrap();
    /// 
    /// // In async mode, check if it's an immediate result
    /// # #[cfg(feature = "async")]
    /// let result = maybe_result.expect_result("shoud be result")?;
    /// 
    /// assert_eq!(result, Value::Int(30));
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    ///
    /// ## Async function call (when `async` feature is enabled)
    ///
    /// ```rust
    /// # #[cfg(feature = "async")]
    /// # async fn example() {
    /// # use cel_cxx::function::*;
    /// # use cel_cxx::values::Value;
    /// async fn async_multiply(a: i64, b: i64) -> i64 { a * b }
    /// let func = async_multiply.into_function();
    ///
    /// let args = vec![Value::Int(6), Value::Int(7)];
    /// let maybe_result = func.call(args);
    /// 
    /// // For async functions, extract and await the future
    /// let result = maybe_result.unwrap_future().await.unwrap();
    /// assert_eq!(result, Value::Int(42));
    /// # }
    /// ```
    pub fn call<'this, 'future>(
        &'this self,
        args: Vec<Value>
    ) -> MaybeFuture<'future, Value, Error>
    where 'this: 'future, Self: 'future {
        self.inner.call(args)
    }

    /// Get the expected argument types for this function.
    ///
    /// Returns the function signature information that can be used for:
    /// - Compile-time type checking
    /// - Runtime argument validation
    /// - Documentation generation
    /// - IDE support and auto-completion
    ///
    /// # Returns
    ///
    /// A vector of [`ValueType`] representing the expected argument types
    /// in the order they should be provided to [`call`](Self::call).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cel_cxx::function::*;
    /// # use cel_cxx::types::ValueType;
    /// fn process(name: &str, count: i64, active: bool) -> String {
    ///     format!("{}: {} ({})", name, count, active)
    /// }
    /// let func = process.into_function();
    ///
    /// let arg_types = func.arguments();
    /// assert_eq!(arg_types, vec![
    ///     ValueType::String,
    ///     ValueType::Int,
    ///     ValueType::Bool
    /// ]);
    /// ```
    pub fn arguments(&self) -> Vec<ValueType> {
        self.inner.arguments()
    }

    /// Get the return type of this function.
    ///
    /// Returns type information that can be used for:
    /// - Compile-time type checking of expressions
    /// - Runtime result validation
    /// - Type inference in complex expressions
    ///
    /// # Returns
    ///
    /// The [`ValueType`] that this function returns when called successfully.
    /// For functions returning `Result<T, E>`, this returns the success type `T`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cel_cxx::function::*;
    /// # use cel_cxx::types::ValueType;
    /// fn calculate(x: f64, y: f64) -> f64 { x * y + 1.0 }
    /// let func = calculate.into_function();
    ///
    /// assert_eq!(func.result(), ValueType::Double);
    /// ```
    ///
    /// ```rust
    /// # use cel_cxx::function::*;
    /// # use cel_cxx::types::ValueType;
    /// # use std::convert::Infallible;
    /// fn get_message() -> Result<String, Infallible> { 
    ///     Ok("Hello".to_string()) 
    /// }
    /// let func = get_message.into_function();
    ///
    /// // Returns the success type, not Result<String, Infallible>
    /// assert_eq!(func.result(), ValueType::String);
    /// ```
    pub fn result(&self) -> ValueType {
        self.inner.result()
    }

    /// Get complete function type information.
    ///
    /// Returns a [`FunctionType`] that combines argument and return type information.
    /// This is useful for:
    /// - Function signature matching
    /// - Overload resolution
    /// - Type checking in complex expressions
    ///
    /// # Returns
    ///
    /// A [`FunctionType`] containing complete function signature information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cel_cxx::function::*;
    /// # use cel_cxx::types::{ValueType, FunctionType};
    /// fn multiply(a: i64, b: i64) -> i64 { a * b }
    /// let func = multiply.into_function();
    ///
    /// let func_type = func.function_type();
    /// assert_eq!(func_type.arguments(), &[ValueType::Int, ValueType::Int]);
    /// assert_eq!(func_type.result(), &ValueType::Int);
    /// ```
    pub fn function_type(&self) -> FunctionType {
        FunctionType::new(self.result(), self.arguments())
    }

    /// Get the number of arguments this function expects.
    ///
    /// This is a convenience method equivalent to `self.arguments().len()`.
    /// Useful for quick arity checking without allocating the full argument
    /// type vector.
    ///
    /// # Returns
    ///
    /// The number of parameters this function expects.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cel_cxx::function::*;
    /// fn no_args() -> i64 { 42 }
    /// fn one_arg(x: i64) -> i64 { x }
    /// fn three_args(a: i64, b: i64, c: i64) -> i64 { a + b + c }
    ///
    /// assert_eq!(no_args.into_function().arguments_len(), 0);
    /// assert_eq!(one_arg.into_function().arguments_len(), 1);
    /// assert_eq!(three_args.into_function().arguments_len(), 3);
    /// ```
    pub fn arguments_len(&self) -> usize {
        self.inner.arguments_len()
    }
}

// =============================================================================
// Implementation details
// =============================================================================

/// Internal trait for type-erased function implementations.
///
/// This trait provides a uniform interface for calling functions regardless
/// of their original signature. It is not exposed publicly as users should
/// interact with [`Function`] instead.
trait ErasedFn: Send + Sync {
    /// Call the function with the provided arguments.
    fn call<'this, 'future>(
        &'this self,
        args: Vec<Value>
    ) -> MaybeFuture<'future, Value, Error>
    where 'this: 'future, Self: 'future;

    /// Get the expected argument types.
    fn arguments(&self) -> Vec<ValueType>;

    /// Get the return type.
    fn result(&self) -> ValueType;

    /// Get the number of expected arguments.
    fn arguments_len(&self) -> usize;
}

macro_rules! impl_arguments {
    ($(
        $($ty:ident),+
    )?) => {
        impl<$($($ty: FromValue + TypedValue),+)?> Arguments for ($($($ty,)+)?) {}
        impl<$($($ty: FromValue + TypedValue),+)?> private::Sealed for ($($($ty,)+)?) {}
    }
}

impl_arguments!();
impl_arguments!(A1);
impl_arguments!(A1, A2);
impl_arguments!(A1, A2, A3);
impl_arguments!(A1, A2, A3, A4);
impl_arguments!(A1, A2, A3, A4, A5);
impl_arguments!(A1, A2, A3, A4, A5, A6);
impl_arguments!(A1, A2, A3, A4, A5, A6, A7);
impl_arguments!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_arguments!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_arguments!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);


/// Internal trait for safely converting `FromValue::Output` to function parameter types.
///
/// This trait contains unsafe code for lifetime erasure and should only be used
/// within the controlled context of function dispatch. It is not exposed publicly
/// to ensure memory safety.
trait Argument: FromValue + TypedValue {
    /// Convert a CEL value to the function parameter type.
    ///
    /// This method safely converts a [`Value`] reference to the target type
    /// by first using [`FromValue::from_value`] and then performing controlled
    /// lifetime erasure via [`Self::from_output`].
    fn make_argument<'a>(value: &'a Value) -> Result<Self, FromValueError> {
        let output = <Self as FromValue>::from_value(value)?;
        Ok(unsafe { Self::from_output(output) })
    }

    /// Convert `FromValue::Output<'a>` to `Self` by erasing lifetime information.
    /// 
    /// # Safety
    /// 
    /// This method performs an unsafe lifetime erasure operation that is only safe
    /// under specific controlled conditions:
    /// 
    /// 1. **Memory Layout Guarantee**: The method assumes that `Self` and 
    ///    `Self::Output<'a>` have identical memory layouts. This is verified by
    ///    a debug assertion checking `size_of` equality.
    /// 
    /// 2. **Lifetime Erasure Safety**: The lifetime parameter 'a is erased through
    ///    unsafe pointer casting. This is safe because:
    ///    - The input `output` is consumed (moved) into this function
    ///    - The returned `Self` will be immediately consumed by the function call
    ///    - No references escape the function call scope
    /// 
    /// 3. **Controlled Usage Context**: This method is only called within the
    ///    function dispatch mechanism where:
    ///    - The source `&[Value]` array remains valid for the entire function call
    ///    - The converted arguments are immediately passed to the target function
    ///    - The function result is immediately converted via `IntoResult`
    /// 
    /// 4. **Type System Cooperation**: For reference types like `&str`:
    ///    - `Self::Output<'a>` is `&'a str` (borrowed from Value)
    ///    - `Self` is `&str` (with erased lifetime)
    ///    - The underlying string data in Value remains valid throughout the call
    /// 
    /// The safety of this operation relies on the fact that the lifetime erasure
    /// is temporary and scoped - the converted values never outlive the original
    /// Value array that owns the underlying data.
    /// 
    /// # Implementation Details
    /// 
    /// The conversion process:
    /// 1. Cast the reference to `output` as a pointer to `Self`
    /// 2. Forget the original `output` to prevent double-drop
    /// 3. Read the value from the pointer, effectively transferring ownership
    /// 
    /// This is essentially a controlled `transmute` operation that preserves
    /// the bit representation while changing the type signature.
    unsafe fn from_output<'a>(output: <Self as FromValue>::Output<'a>) -> Self {
        debug_assert!(std::mem::size_of::<<Self as FromValue>::Output<'a>>() == std::mem::size_of::<Self>());

        let ptr: *const Self = (&output as *const <Self as FromValue>::Output<'a>).cast();
        std::mem::forget(output);
        std::ptr::read(ptr)
    }
}

// Blanket implementation for all types that implement FromValue and TypedValue
impl<T: FromValue + TypedValue> Argument for T {}

/// Internal wrapper for synchronous functions that implements [`ErasedFn`].
///
/// This struct wraps a function along with phantom data for its signature,
/// allowing type-erased storage and invocation.
struct FnWrapper<F, R, Args> {
    func: F,
    _phantom: std::marker::PhantomData<(R, Args)>,
}

impl<F, R, Args> FnWrapper<F, R, Args> {
    /// Create a new function wrapper.
    fn new(func: F) -> Self {
        Self {
            func,
            _phantom: std::marker::PhantomData,
        }
    }
}

// =============================================================================
// Helper macros and implementations
// =============================================================================

/// Compile-time macro to count the number of function arguments.
macro_rules! count_args {
    () => { 0 };
    ($head:ident $(, $tail:ident)*) => { 1 + count_args!($($tail),*) };
}
use count_args;

/// Macro to generate [`ErasedFn`] implementations for synchronous functions
/// with different arities (0 to 10 parameters).
macro_rules! impl_fn_wrapper {
    ($($ty:ident),*) => {
        paste::paste! {
            impl<F, R, $($ty,)*> ErasedFn for FnWrapper<F, R, ($($ty,)*)>
            where
                F: Fn($($ty,)*) -> R + Send + Sync,
                R: IntoResult + Send + Sync,
                $($ty: FromValue + TypedValue + Send + Sync,)*
            {
                fn call<'this, 'future>(
                    &'this self,
                    args: Vec<Value>
                ) -> MaybeFuture<'future, Value, Error>
                where 'this: 'future, Self: 'future {
                    let f = || {
                        // Compile-time constant: number of expected arguments
                        const EXPECTED_LEN: usize = count_args!($($ty),*);
                        
                        if args.len() != EXPECTED_LEN {
                            return Err(Error::invalid_argument(
                                format!("expected {} arguments, got {}", EXPECTED_LEN, args.len())
                            ));
                        }
                        
                        #[allow(unused_mut, unused_variables)]
                        let mut iter = args.iter();
                        $(
                            let [< $ty:lower >] = $ty::make_argument(
                                iter.next().expect("argument count already validated")
                            ).map_err(|e| Error::invalid_argument(format!("argument error: {}", e)))?;
                        )*
                        
                        let result = (self.func)($([< $ty:lower >],)*);
                        result.into_result()
                    };
                    f().into()
                }

                fn arguments(&self) -> Vec<ValueType> {
                    vec![$($ty::value_type()),*]
                }

                fn result(&self) -> ValueType {
                    R::value_type()
                }

                fn arguments_len(&self) -> usize {
                    count_args!($($ty),*)
                }
            }
            
            // Implementation of IntoFunction for synchronous functions
            impl<'f, F, R, $($ty,)*> IntoFunction<'f, (), ($($ty,)*)> for F
            where
                F: Fn($($ty,)*) -> R + Send + Sync + 'f,
                R: IntoResult + Send + Sync + 'f,
                ($($ty,)*): Arguments,
                $($ty: FromValue + TypedValue + Send + Sync + 'f,)*
            {
                fn into_function(self) -> Function<'f> {
                    Function::new(FnWrapper::<F, R, ($($ty,)*)>::new(self))
                }
            }
            
            // Sealed implementation for synchronous functions
            impl<'f, F, R, $($ty,)*> private::Sealed<(), ($($ty,)*)> for F
            where
                F: Fn($($ty,)*) -> R + Send + Sync + 'f,
                R: IntoResult + Send + Sync + 'f,
                ($($ty,)*): Arguments,
                $($ty: FromValue + TypedValue + Send + Sync + 'f,)*
            {}
        }
    };
}

// Generate implementations for functions with 0-10 parameters
impl_fn_wrapper!();
impl_fn_wrapper!(A1);
impl_fn_wrapper!(A1, A2);
impl_fn_wrapper!(A1, A2, A3);
impl_fn_wrapper!(A1, A2, A3, A4);
impl_fn_wrapper!(A1, A2, A3, A4, A5);
impl_fn_wrapper!(A1, A2, A3, A4, A5, A6);
impl_fn_wrapper!(A1, A2, A3, A4, A5, A6, A7);
impl_fn_wrapper!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_fn_wrapper!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_fn_wrapper!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

// =============================================================================
// Async function support (optional feature)
// =============================================================================

/// Internal wrapper for asynchronous functions that implements [`ErasedFn`].
///
/// This struct is similar to [`FnWrapper`] but designed for async functions
/// that return futures. It's only available when the `async` feature is enabled.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
struct FnWrapperAsync<F, R, Args> {
    func: F,
    _phantom: std::marker::PhantomData<(R, Args)>,
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl<F, R, Args> FnWrapperAsync<F, R, Args> {
    /// Create a new async function wrapper.
    fn new(func: F) -> Self {
        Self {
            func,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
mod async_impls {
    use super::*;

    /// Macro to generate [`ErasedFn`] implementations for asynchronous functions
    /// with different arities (0 to 10 parameters).
    macro_rules! impl_fn_wrapper_async {
        ($($ty:ident),*) => {
            paste::paste! {
                impl<F, Fut, R, $($ty,)*> ErasedFn for FnWrapperAsync<F, R, ($($ty,)*)>
                where
                    F: Fn($($ty,)*) -> Fut + Send + Sync,
                    Fut: std::future::Future<Output = R> + Send,
                    R: IntoResult + Send + Sync,
                    $($ty: FromValue + TypedValue + Send + Sync,)*
                {
                    fn call<'this, 'future>(
                        &'this self,
                        args: Vec<Value>
                    ) -> MaybeFuture<'future, Value, Error>
                    where 'this: 'future, Self: 'future {
                        let f = || async move {
                            // Compile-time constant: number of expected arguments
                            const EXPECTED_LEN: usize = count_args!($($ty),*);
                            
                            if args.len() != EXPECTED_LEN {
                                return Err(Error::invalid_argument(
                                    format!("expected {} arguments, got {}", EXPECTED_LEN, args.len())
                                ));
                            }
                            
                            #[allow(unused_mut, unused_variables)]
                            let mut iter = args.iter();
                            $(
                                let [< $ty:lower >] = $ty::make_argument(
                                    iter.next().expect("argument count already validated")
                                ).map_err(|e| Error::invalid_argument(format!("argument error: {}", e)))?;
                            )*
                            
                            let future = (self.func)($([< $ty:lower >],)*);
                            let result = future.await;
                            result.into_result()
                        };
                        Box::pin(f()).into()
                    }

                    fn arguments(&self) -> Vec<ValueType> {
                        vec![$($ty::value_type()),*]
                    }

                    fn result(&self) -> ValueType {
                        R::value_type()
                    }

                    fn arguments_len(&self) -> usize {
                        count_args!($($ty),*)
                    }
                }
                
                // Implementation of IntoFunction for asynchronous functions
                impl<'f, F, Fut, R, $($ty,)*> IntoFunction<'f, Async, ($($ty,)*)> for F
                where
                    F: Fn($($ty,)*) -> Fut + Send + Sync + 'f,
                    Fut: std::future::Future<Output = R> + Send + 'f,
                    R: IntoResult + Send + Sync + 'f,
                    $($ty: FromValue + TypedValue + Send + Sync + 'f,)*
                {
                    fn into_function(self) -> Function<'f> {
                        Function::new(FnWrapperAsync::<F, R, ($($ty,)*)>::new(self))
                    }
                }
                
                // Sealed implementation for asynchronous functions
                impl<'f, F, Fut, R, $($ty,)*> private::Sealed<Async, ($($ty,)*)> for F
                where
                    F: Fn($($ty,)*) -> Fut + Send + Sync + 'f,
                    Fut: std::future::Future<Output = R> + Send + 'f,
                    R: IntoResult + Send + Sync + 'f,
                    $($ty: FromValue + TypedValue + Send + Sync + 'f,)*
                {}
            }
        };
    }

    
    impl_fn_wrapper_async!();
    impl_fn_wrapper_async!(A1);
    impl_fn_wrapper_async!(A1, A2);
    impl_fn_wrapper_async!(A1, A2, A3);
    impl_fn_wrapper_async!(A1, A2, A3, A4);
    impl_fn_wrapper_async!(A1, A2, A3, A4, A5);
    impl_fn_wrapper_async!(A1, A2, A3, A4, A5, A6);
    impl_fn_wrapper_async!(A1, A2, A3, A4, A5, A6, A7);
    impl_fn_wrapper_async!(A1, A2, A3, A4, A5, A6, A7, A8);
    impl_fn_wrapper_async!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
    impl_fn_wrapper_async!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
}

// =============================================================================
// Additional implementations
// =============================================================================

/// Debug implementation for type-erased functions.
impl std::fmt::Debug for dyn ErasedFn + '_ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<function>")
    }
}

/// Private module for sealed traits to prevent external implementations.
mod private {
    #[derive(Debug)]
    pub enum Placeholder {}

    pub trait Sealed<T = Placeholder, U = Placeholder> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_function_registration() {
        // Test basic function registration and metadata extraction
        fn add(a: i64, b: i64) -> i64 { a + b }
        let func = add.into_function();
        
        assert_eq!(func.arguments(), vec![ValueType::Int, ValueType::Int]);
        assert_eq!(func.result(), ValueType::Int);
        
        // Test closure registration
        let multiplier = 3;
        let multiply = move |x: i64| -> i64 { x * multiplier };
        let func2 = multiply.into_function();
        
        assert_eq!(func2.arguments(), vec![ValueType::Int]);
        assert_eq!(func2.result(), ValueType::Int);
    }

    #[test]
    fn test_reference_return_functions() {
        // Test functions that return borrowed data
        fn return_arg<'a>(a: &'a str) -> &'a str { a }
        let func = return_arg.into_function();
        
        assert_eq!(func.arguments(), vec![ValueType::String]);
        assert_eq!(func.result(), ValueType::String);
        
        // Test function invocation
        let result = func.call(vec!["hello".into()]);
        let result = result.expect_result("test_reference_return_functions");
        assert_eq!(result.unwrap(), "hello".into());
    }

    #[test]
    fn test_closure_with_captured_data() {
        // Test closures that capture environment variables
        let prefix = String::from("Hello, ");
        let with_prefix = move |name: &str| -> String { 
            format!("{}{}", prefix, name) 
        };
        
        let func = with_prefix.into_function();
        
        assert_eq!(func.arguments(), vec![ValueType::String]);
        assert_eq!(func.result(), ValueType::String);
        
        // Test function invocation
        let result = func.call(vec!["world".into()]);
        let result = result.expect_result("test_closure_with_captured_data");
        assert_eq!(result.unwrap(), "Hello, world".into());
    }

    #[test]
    fn test_zero_parameter_function() {
        // Test functions with no parameters
        fn get_answer() -> i64 { 42 }
        let func = get_answer.into_function();
        
        assert_eq!(func.arguments(), vec![]);
        assert_eq!(func.result(), ValueType::Int);
        
        // Test function invocation
        let result = func.call(vec![]);
        let result = result.expect_result("test_zero_parameter_function");
        assert_eq!(result.unwrap(), 42i64.into());
    }

    #[test]
    fn test_multiple_parameter_function() {
        // Test functions with multiple parameters
        fn add_three(a: i64, b: i64, c: i64) -> i64 { a + b + c }
        let func = add_three.into_function();
        
        assert_eq!(func.arguments(), vec![ValueType::Int, ValueType::Int, ValueType::Int]);
        assert_eq!(func.result(), ValueType::Int);
        
        // Test function invocation
        let result = func.call(vec![1i64.into(), 2i64.into(), 3i64.into()]);
        let result = result.expect_result("test_multiple_parameter_function");
        assert_eq!(result.unwrap(), 6i64.into());
    }

    #[test]
    fn test_result_error_handling() {
        // Test functions that return Result for error handling
        fn divide(a: i64, b: i64) -> Result<i64, std::io::Error> {
            if b == 0 {
                Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "division by zero"))
            } else {
                Ok(a / b)
            }
        }
        
        let func = divide.into_function();
        
        assert_eq!(func.arguments(), vec![ValueType::Int, ValueType::Int]);
        assert_eq!(func.result(), ValueType::Int);
        
        // Test successful case
        let result = func.call(vec![10i64.into(), 2i64.into()]);
        let result = result.expect_result("test_result_error_handling_success");
        assert_eq!(result.unwrap(), 5i64.into());
        
        // Test error case
        let result = func.call(vec![10i64.into(), 0i64.into()]);
        let result = result.expect_result("test_result_error_handling_error");
        assert!(result.is_err());
    }
} 
