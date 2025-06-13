use std::sync::Arc;
use crate::types::Type;
use crate::values::*;
use crate::maybe_future::*;
use crate::Error;

/// Variable value provider trait.
///
/// `Provider` defines the interface for dynamically computing variable values. This trait is
/// sealed and cannot be implemented directly by external crates. Instead, it is automatically
/// implemented for function types that meet specific criteria.
///
/// # Automatically Implemented For
///
/// The `Provider` trait is automatically implemented for:
///
/// - **Synchronous closures**: `Fn() -> Result<T, E>` where:
///   - `T: IntoValue + TypedValue`
///   - `E: Into<Error> + Send + Sync + 'static`
///   - The closure is `Send + Sync`
///
/// - **Asynchronous closures** (when `async` feature is enabled): `Fn() -> Fut` where:
///   - `Fut: Future<Output = Result<T, E>> + Send`
///   - `T: IntoValue + TypedValue`
///   - `E: Into<Error> + Send + Sync + 'static`
///   - The closure is `Send + Sync`
///
/// # Type Parameters
///
/// - `M`: Function marker indicating sync (`()`) or async (`Async`) mode
/// - `E`: Error type, must be convertible to [`Error`]
/// - `T`: Return value type, must implement [`IntoValue`] and [`TypedValue`]
///
/// # Examples of Valid Provider Functions
///
/// ```rust,no_run
/// use cel_cxx::{VariableBindings, Error};
///
/// let mut bindings = VariableBindings::new();
///
/// // Synchronous provider returning a string
/// bindings.bind_provider("app_name", || -> Result<String, Error> {
///     Ok("MyApp".to_string())
/// })?;
///
/// // Synchronous provider returning current timestamp
/// bindings.bind_provider("current_time", || -> Result<i64, std::time::SystemTimeError> {
///     std::time::SystemTime::now()
///         .duration_since(std::time::UNIX_EPOCH)
///         .map(|d| d.as_secs() as i64)
/// })?;
///
/// // Synchronous provider that never fails
/// bindings.bind_provider("lucky_number", || -> Result<i64, std::convert::Infallible> {
///     Ok(42)
/// })?;
///
/// # Ok::<(), cel_cxx::Error>(())
/// ```
///
/// # Async Examples (requires `async` feature)
///
/// ```rust,no_run
/// # #[cfg(feature = "async")]
/// # {
/// use cel_cxx::{VariableBindings, Error};
///
/// let mut bindings = VariableBindings::new();
///
/// // Async provider making HTTP request
/// bindings.bind_provider("api_data", || async {
///     // Simulate async operation
///     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
///     Ok::<String, Error>("API response".to_string())
/// })?;
///
/// # Ok::<(), cel_cxx::Error>(())
/// # }
/// ```
pub trait Provider<M, E, T>: private::Sealed<M, E, T> + Send + Sync {
    /// Calls the provider to get a value.
    ///
    /// This method is called each time the variable is accessed, returning either a future or
    /// a direct value wrapped in a future-like type.
    fn call<'this, 'f>(&'this self) -> MaybeFuture<'f, Value, Error>
    where 'this: 'f, Self: 'f;

    /// Returns the type of values produced by this provider.
    fn value_type(&self) -> Type;

    /// Returns a type-erased reference to this provider.
    fn erased<'this, 'f>(&'this self) -> &'this (dyn Provider<M, E, T> + 'f)
    where 'this: 'f, Self: 'f + Sized,
    {
        self
    }

    /// Converts this provider into a boxed trait object.
    fn into_erased<'f>(self) -> BoxedProvider<'f, M, E, T>
    where
        Self: Sized + 'f,
    {
        Box::new(self)
    }
}

/// Type alias for boxed providers.
pub type BoxedProvider<'f, M, E, T> = Box<dyn Provider<M, E, T> + 'f>;

/// Type-erased provider trait.
///
/// Internal trait used for type erasure of generic [`Provider`] types.
/// This allows storing different provider types in the same collection.
pub trait ErasedProvider: Send + Sync {
    /// Calls the provider to get a value.
    fn call<'this, 'a>(&'this self) -> MaybeFuture<'a, Value, Error>
    where 'this: 'a, Self: 'a;

    /// Returns the type of values produced by this provider.
    fn value_type(&self) -> Type;
}

impl<'a> std::fmt::Debug for dyn ErasedProvider + 'a {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<function>")
    }
}

/// Type alias for value providers.
///
/// Represents a thread-safe type-erased provider, wrapped with `Arc` to support shared ownership.
pub type ValueProvider<'a> = Arc<dyn ErasedProvider + 'a>;

impl<'f, M, E, T> ErasedProvider for dyn Provider<M, E, T> + 'f {
    fn call<'this, 'a>(&'this self) -> MaybeFuture<'a, Value, Error>
    where 'this: 'a, Self: 'a {
        Provider::call(self)
    }

    fn value_type(&self) -> Type {
        Provider::value_type(self)
    }
}

impl<T: ?Sized + ErasedProvider> ErasedProvider for Box<T> {
    fn call<'this, 'a>(&'this self) -> MaybeFuture<'a, Value, Error>
    where 'this: 'a, Self: 'a {
        T::call(&**self)
    }

    fn value_type(&self) -> Type {
        T::value_type(&**self)
    }
}

const _: () = {
    impl<'f, F, E, T> Provider<(), E, T> for F
    where
        F: (Fn() -> Result<T, E>) + Send + Sync + 'f,
        E: Into<Error> + Send + Sync + 'static,
        T: IntoValue + TypedValue + 'f,
    {
        fn call<'this, 'a>(&'this self) -> MaybeFuture<'a, Value, Error>
        where 'this: 'a, Self: 'a {
            let f = || {
                let result = self()
                    .map_err(|e| e.into())?;
                Ok(result.into_value())
            };
            f().into()
        }

        fn value_type(&self) -> Type {
            T::value_type()
        }
    }
    impl<'f, F, E, T> private::Sealed<(), E, T> for F
    where
        F: (Fn() -> Result<T, E>) + Send + Sync + 'f,
        E: Into<Error> + Send + Sync + 'static,
        T: IntoValue + TypedValue,
    {}
};

#[cfg(feature = "async")]
const _: () = {
    use crate::marker::*;

    impl<'f, F, Fut, E, T> Provider<Async, E, T> for F
    where
        F: (Fn() -> Fut) + Send + Sync + 'f,
        Fut: futures::Future<Output = Result<T, E>> + Send + 'f,
        E: Into<Error> + Send + Sync + 'static,
        T: IntoValue + TypedValue + 'f,
    {
        fn call<'this, 'a>(&'this self) -> MaybeFuture<'a, Value, Error>
        where 'this: 'a, Self: 'a {
            let f = || async move {
                let result = self().await
                    .map_err(|e| e.into())?;
                Ok(result.into_value())
            };
            Box::pin(f()).into()
        }

        fn value_type(&self) -> Type {
            T::value_type()
        }
    }
    impl<'f, F, Fut, E, T> private::Sealed<Async, E, T> for F
    where
        F: (Fn() -> Fut) + Send + Sync + 'f,
        Fut: futures::Future<Output = Result<T, E>> + Send + 'f,
        E: Into<Error> + Send + Sync + 'static,
        T: IntoValue + TypedValue,
    {}
};

mod private {
    pub trait Sealed<M, E, T>: Send + Sync {}
}
