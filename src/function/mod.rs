//! CEL function system.
//!
//! This module provides functionality for declaring, registering, and calling custom functions in CEL environments.
//! The function system consists of:
//!
//! - [`function::FunctionRegistry`]: Compile-time function registry for declaring function signatures and registering implementations
//! - [`function::FunctionBindings`]: Runtime function bindings for calling functions during evaluation
//! - [`function::FunctionOverloads`]: Function overload management supporting multiple implementations with different signatures
//!
//! # Function Types
//!
//! The system supports both **function declarations** (type signatures only) and **function implementations** (callable code):
//!
//! - **Declarations**: Use [`function::FnDecl`] trait for compile-time type checking
//! - **Implementations**: Use [`function::FnImpl`] trait for runtime function calls
//!
//! Both traits are sealed and automatically implemented for compatible function types.
//!
//! # Examples
//!
//! ```rust,no_run
//! use cel_cxx::{Env, Error};
//!
//! // Define custom functions
//! fn add_numbers(a: i64, b: i64) -> Result<i64, Error> {
//!     Ok(a + b)
//! }
//!
//! fn format_greeting(name: String) -> Result<String, Error> {
//!     Ok(format!("Hello, {}!", name))
//! }
//!
//! // Register functions in environment
//! let env = Env::builder()
//!     .register_global("add", add_numbers)?
//!     .register_global("greet", format_greeting)?
//!     .build()?;
//!
//! // Use in CEL expressions
//! let program = env.compile("greet('World') + ' Result: ' + string(add(2, 3))")?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```

//pub mod registry;

use std::sync::Arc;
use crate::Kind;
use crate::values::*;
use crate::types::*;
use crate::marker::*;
use crate::maybe_future::*;
use crate::error::Error;

mod registry;
mod overload;
mod bindings;

pub use registry::*;
pub use overload::*;
pub use bindings::*;

/// Function declaration trait.
///
/// `FnDecl` provides static type information for function signatures. This trait is sealed and
/// cannot be implemented directly by external crates. Instead, it is automatically implemented
/// for function pointer types that meet specific criteria.
///
/// # Automatically Implemented For
///
/// The `FnDecl` trait is automatically implemented for function pointer types:
///
/// - `fn() -> R`
/// - `fn(A1) -> R`
/// - `fn(A1, A2) -> R`
/// - `fn(A1, A2, A3) -> R`
/// - ... (up to 10 arguments)
///
/// Where:
/// - `R: IntoValue + TypedValue` (return type)
/// - `A1, A2, ...: FromValue + TypedValue` (argument types)
///
/// # Examples of Valid Function Types
///
/// ```rust,no_run
/// use cel_cxx::{FunctionRegistry, Error};
///
/// let mut registry = FunctionRegistry::new();
/// registry
///     .declare_global::<fn() -> f64>("pi")?
///     .declare_global::<fn(i64) -> i64>("square")?
///     .declare_global::<fn(i64, i64, i64) -> i64>("add_three")?
///     .declare_global::<fn(String, i64) -> String>("format_number")?;
/// # Ok::<(), cel_cxx::Error>(())
/// ```
pub trait FnDecl {
    /// Returns the argument types of the function.
    fn arguments() -> Vec<Type>;
    /// Returns the result type of the function.
    fn result() -> Type;
}

const _: () = {
    macro_rules! impl_fn_decl {
        ($($ty:ident),*) => {
            paste::paste! {
                impl<R, $($ty,)*> FnDecl for fn($($ty,)*) -> R
                where
                    R: IntoValue + TypedValue,
                    $(
                        $ty: FromValue + TypedValue,
                    )*
                {
                    fn arguments() -> Vec<Type> {
                        <($($ty,)*) as TypedArguments>::arguments_type()
                    }

                    fn result() -> Type {
                        R::value_type()
                    }
                }
            }
        }
    }

    impl_fn_decl!();
    impl_fn_decl!(A1);
    impl_fn_decl!(A1, A2);
    impl_fn_decl!(A1, A2, A3);
    impl_fn_decl!(A1, A2, A3, A4);
    impl_fn_decl!(A1, A2, A3, A4, A5);
    impl_fn_decl!(A1, A2, A3, A4, A5, A6);
    impl_fn_decl!(A1, A2, A3, A4, A5, A6, A7);
    impl_fn_decl!(A1, A2, A3, A4, A5, A6, A7, A8);
    impl_fn_decl!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
    impl_fn_decl!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
};


/// Function implementation trait.
///
/// `FnImpl` defines the interface for callable CEL functions. This trait is sealed and cannot be
/// implemented directly by external crates. Instead, it is automatically implemented for function
/// types that meet specific criteria.
///
/// # Automatically Implemented For
///
/// The `FnImpl` trait is automatically implemented for:
///
/// - **Synchronous functions and closures**: `Fn(A1, A2, ...) -> Result<R, E>` where:
///   - `A1, A2, ...: FromValue + TypedValue` (argument types)
///   - `R: IntoValue + TypedValue` (return type)
///   - `E: Into<Error> + Send + Sync + 'static` (error type)
///   - The function/closure is `Send + Sync`
///
/// - **Asynchronous functions and closures** (when `async` feature is enabled): `Fn(A1, A2, ...) -> Fut` where:
///   - `Fut: Future<Output = Result<R, E>> + Send` (future type)
///   - `A1, A2, ...: FromValue + TypedValue` (argument types)
///   - `R: IntoValue + TypedValue` (return type)
///   - `E: Into<Error> + Send + Sync + 'static` (error type)
///   - The function/closure is `Send + Sync`
///
/// # Type Parameters
///
/// - `M`: Function marker indicating sync (`()`) or async (`Async`) mode
/// - `E`: Error type, must be convertible to [`Error`]
/// - `R`: Return type, must implement [`IntoValue`] and [`TypedValue`]
/// - `A`: Argument tuple type
///
/// # Examples of Valid Function Types
///
/// ```rust,no_run
/// use cel_cxx::{FunctionRegistry, Error};
/// use std::collections::HashMap;
/// use std::convert::Infallible;
///
/// let mut registry = FunctionRegistry::new();
///
/// // Simple function
/// fn double(x: i64) -> Result<i64, Infallible> {
///     Ok(x * 2)
/// }
/// registry.register_global("double", double)?;
///
/// // Function with multiple arguments
/// fn multiply(a: i64, b: i64) -> Result<i64, Infallible> {
///     Ok(a * b)
/// }
/// registry.register_global("multiply", multiply)?;
///
/// // Function with different error type
/// fn parse_int(s: String) -> Result<i64, std::num::ParseIntError> {
///     s.parse::<i64>()
/// }
/// registry.register_global("parse_int", parse_int)?;
///
/// // Function with complex types
/// fn map_size(map: HashMap<String, i64>) -> Result<i64, Infallible> {
///     Ok(map.len() as i64)
/// }
/// registry.register_global("map_size", map_size)?;
///
/// // Closure capturing environment
/// let multiplier = 3;
/// let closure = move |x: i64| -> Result<i64, Infallible> {
///     Ok(x * multiplier)
/// };
/// registry.register_global("triple", closure)?;
///
/// # Ok::<(), cel_cxx::Error>(())
/// ```
///
/// # Async Examples (requires `async` feature)
///
/// ```rust,no_run
/// # #[cfg(feature = "async")]
/// # {
/// use std::convert::Infallible;
/// use cel_cxx::{FunctionRegistry, Error};
///
/// let mut registry = FunctionRegistry::new();
///
/// // Async function
/// async fn fetch_data(url: String) -> Result<String, Infallible> {
///     // Simulate async operation
///     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
///     Ok(format!("Data from {}", url))
/// }
/// registry.register_global("fetch", fetch_data)?;
///
/// // Async closure
/// let timeout = 1000;
/// let async_closure = async |delay: i64| -> Result<i64, Infallible> {
///     tokio::time::sleep(tokio::time::Duration::from_millis(delay as u64)).await;
///     Ok(timeout)
/// };
/// registry.register_global("wait", async_closure)?;
///
/// # Ok::<(), cel_cxx::Error>(())
/// # }
/// ```
pub trait FnImpl<M, E, R, A>: private::Sealed<M, E, R, A> + Send + Sync {
    /// Calls the function with the given arguments.
    fn call<'this, 'a>(
        &'this self,
        args: Vec<Value>
    ) -> MaybeFuture<'a, Value, Error>
    where 'this: 'a, Self: 'a;

    /// Returns the argument types of the function.
    fn arguments(&self) -> Vec<Type>;

    /// Returns the result type of the function.
    fn result(&self) -> Type;

    /// Returns a type-erased reference to this function.
    fn erased<'this, 'f>(&'this self) -> &'this (dyn FnImpl<M, E, R, A> + 'f)
    where 'this: 'f, Self: 'f + Sized { self }

    /// Converts this function into a boxed trait object.
    fn into_erased<'f>(self) -> BoxedFunction<'f, M, E, R, A>
    where Self: Sized + 'f { Box::new(self) }
}

/// Type alias for boxed functions.
pub type BoxedFunction<'f, M, E, R, A> = Box<dyn FnImpl<M, E, R, A> + 'f>;

/// Type-erased function implementation trait.
///
/// Internal trait used for type erasure of generic [`FnImpl`] types.
/// This allows storing different function types in the same collection.
pub trait ErasedFnImpl: Send + Sync {
    /// Calls the function with the given arguments.
    fn call<'this, 'a>(
        &'this self,
        args: Vec<Value>
    ) -> MaybeFuture<'a, Value, Error>
    where 'this: 'a, Self: 'a;

    /// Returns the function declaration.
    fn decl(&self) -> FunctionDecl {
        FunctionDecl::new(self.result(), self.arguments())
    }

    /// Returns the function type.
    fn function_type(&self) -> FunctionType {
        FunctionType::new(self.result(), self.arguments())
    }

    /// Returns the argument types of the function.
    fn arguments(&self) -> Vec<Type>;

    /// Returns the result type of the function.
    fn result(&self) -> Type;

    /// Returns the argument kinds of the function.
    fn argument_kinds(&self) -> Vec<Kind> {
        self.arguments()
            .into_iter()
            .map(|t| t.kind())
            .collect()
    }

    /// Returns the result kind of the function.
    fn result_kind(&self) -> Kind {
        self.result().kind()
    }
}

impl<'a> std::fmt::Debug for dyn ErasedFnImpl + 'a {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<function>")
    }
}

/// Type alias for function implementations.
///
/// Represents a thread-safe type-erased function, wrapped with `Arc` to support shared ownership.
pub type FunctionImpl<'f> = Arc<dyn ErasedFnImpl + 'f>;

impl<'f, M, E, R, A> ErasedFnImpl for dyn FnImpl<M, E, R, A> + 'f {
    fn call<'this, 'a>(
        &'this self,
        args: Vec<Value>
    ) -> MaybeFuture<'a, Value, Error>
    where 'this: 'a, Self: 'a {
        FnImpl::call(self, args)
    }

    fn arguments(&self) -> Vec<Type> {
        FnImpl::arguments(self)
    }

    fn result(&self) -> Type {
        FnImpl::result(self)
    }
}

impl<T: ?Sized + ErasedFnImpl> ErasedFnImpl for Box<T> {
    fn call<'this, 'a>(
        &'this self,
        args: Vec<Value>
    ) -> MaybeFuture<'a, Value, Error>
    where 'this: 'a, Self: 'a {
        T::call(&**self, args)
    }

    fn arguments(&self) -> Vec<Type> {
        T::arguments(&**self)
    }

    fn result(&self) -> Type {
        T::result(&**self)
    }
}

const _: () = {
    macro_rules! impl_fn_impl {
        ($($ty:ident),*) => {
            paste::paste! {
                #[allow(unused_variables, unused_mut)]
                impl<'f, F, E, R, $($ty,)*> FnImpl<(), E, R, ($($ty,)*)> for F
                where
                    F: (Fn($($ty,)*) -> Result<R, E>) + Send + Sync + 'f,
                    E: Into<Error> + Send + Sync + 'static,
                    R: IntoValue + TypedValue + 'f,
                    $(
                        for<'a> $ty: FromValue + TypedValue + 'a,
                    )*
                {
                    fn call<'this, 'a>(
                        &'this self,
                        args: Vec<Value>
                    ) -> MaybeFuture<'a, Value, Error>
                    where 'this: 'a, Self: 'a {
                        let f = || {
                            let len = args.len();
                            let mut iter = args.into_iter();
                            $(
                                let [< $ty:lower >] = $ty::from_value(
                                    iter.next().ok_or(
                                        Error::invalid_argument(format!("expected {} arguments", len))
                                    )?
                                ).map_err(
                                    |v| Error::invalid_argument(format!("expected {}, got {}", v.value_type(), v))
                                )?;
                            )*
                            let result = self($([< $ty:lower >],)*)
                                .map_err(|e| e.into())?;
                            Ok(result.into_value())
                        };
                        f().into()
                    }

                    fn arguments(&self) -> Vec<Type> {
                        vec![$($ty::value_type(),)*]
                    }

                    fn result(&self) -> Type {
                        R::value_type()
                    }
                }
                impl<'f, F, E, R, $($ty,)*> private::Sealed<(), E, R, ($($ty,)*)> for F
                where
                    F: (Fn($($ty,)*) -> Result<R, E>) + Send + Sync + 'f,
                    E: Into<Error> + Send + Sync + 'static,
                    R: IntoValue + TypedValue + 'f,
                    $(
                        $ty: FromValue + TypedValue,
                    )*
                {}
            }
        }
    }

    impl_fn_impl!();
    impl_fn_impl!(A1);
    impl_fn_impl!(A1, A2);
    impl_fn_impl!(A1, A2, A3);
    impl_fn_impl!(A1, A2, A3, A4);
    impl_fn_impl!(A1, A2, A3, A4, A5);
    impl_fn_impl!(A1, A2, A3, A4, A5, A6);
    impl_fn_impl!(A1, A2, A3, A4, A5, A6, A7);
    impl_fn_impl!(A1, A2, A3, A4, A5, A6, A7, A8);
    impl_fn_impl!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
    impl_fn_impl!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
};

#[cfg(feature = "async")]
const _: () = {
    macro_rules! impl_fn_impl_async {
        ($($ty:ident),*) => {
            paste::paste! {
                #[allow(unused_variables, unused_mut)]
                impl<'f, F, Fut, E, R, $($ty,)*> FnImpl<Async, E, R, ($($ty,)*)> for F
                where
                    F: (Fn($($ty,)*) -> Fut) + Send + Sync + 'f,
                    Fut: futures::Future<Output = Result<R, E>> + Send + 'f,
                    E: Into<Error> + Send + Sync + 'static,
                    R: IntoValue + TypedValue + 'f,
                    $(
                        for<'a> $ty: FromValue + TypedValue + 'a,
                    )*
                {
                    fn call<'this, 'a>(
                        &'this self,
                        args: Vec<Value>
                    ) -> MaybeFuture<'a, Value, Error>
                    where 'this: 'a, Self: 'a {
                        let f = || async move {
                            let len = args.len();
                            let mut iter = args.into_iter();
                            $(
                                let [< $ty:lower >] = $ty::from_value(
                                    iter.next().ok_or(
                                        Error::invalid_argument(format!("expected {} arguments", len))
                                    )?
                                ).map_err(
                                    |v| Error::invalid_argument(format!("expected {}, got {}", v.value_type(), v))
                                )?;
                            )*
                            let result = self($([< $ty:lower >],)*).await
                                .map_err(|e| e.into())?;
                            Ok(result.into_value())
                        };
                        Box::pin(f()).into()
                    }

                    fn arguments(&self) -> Vec<Type> {
                        <($($ty,)*) as TypedArguments>::arguments_type()
                    }

                    fn result(&self) -> Type {
                        R::value_type()
                    }
                }
                impl<'f, F, Fut, E, R, $($ty,)*> private::Sealed<Async, E, R, ($($ty,)*)> for F
                where
                    F: (Fn($($ty,)*) -> Fut) + Send + Sync + 'f,
                    Fut: futures::Future<Output = Result<R, E>> + Send + 'f,
                    E: Into<Error> + Send + Sync + 'static,
                    R: IntoValue + TypedValue + 'f,
                    $(
                        $ty: FromValue + TypedValue,
                    )*
                {}
            }
        }
    }

    impl_fn_impl_async!();
    impl_fn_impl_async!(A1);
    impl_fn_impl_async!(A1, A2);
    impl_fn_impl_async!(A1, A2, A3);
    impl_fn_impl_async!(A1, A2, A3, A4);
    impl_fn_impl_async!(A1, A2, A3, A4, A5);
    impl_fn_impl_async!(A1, A2, A3, A4, A5, A6);
    impl_fn_impl_async!(A1, A2, A3, A4, A5, A6, A7);
    impl_fn_impl_async!(A1, A2, A3, A4, A5, A6, A7, A8);
    impl_fn_impl_async!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
    impl_fn_impl_async!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
};

mod private {
    pub trait Sealed<M, E, R, A> {}
}
