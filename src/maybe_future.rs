//! Conditional future types for async/sync compatibility.
//!
//! This module provides the [`MaybeFuture`] type, which has **different definitions**
//! depending on whether the `async` feature is enabled. This allows the same API to work
//! seamlessly in both synchronous and asynchronous contexts.
//!
//! # ⚠️ Feature-Dependent Type Definition
//!
//! **Critical**: [`MaybeFuture`] is implemented differently based on feature flags:
//!
//! | Feature State | Type Definition | Behavior |
//! |---------------|-----------------|----------|
//! | **No `async`** | `type MaybeFuture<'a, T, E> = Result<T, E>` | Simple type alias, zero overhead |
//! | **With `async`** | `enum MaybeFuture<'a, T, E> { Result(..), Future(..) }` | Can hold immediate results or futures |
//!
//! # Documentation Generation
//!
//! **Important**: When building documentation with `--all-features` (as on docs.rs), only the
//! async variant is shown because the `async` feature is enabled. The synchronous variant
//! (simple type alias) is only visible when building docs without the `async` feature.
//!
//! - **On docs.rs**: Shows the enum variant with `Result` and `Future` cases
//! - **Without async feature**: `MaybeFuture` is just a type alias for `Result<T, E>`
//! - Each definition is marked with appropriate `#[cfg(...)]` attributes
//!
//! # Usage Guidelines
//!
//! ## For Library Authors
//!
//! When returning [`MaybeFuture`] from your APIs:
//! ```ignore
//! use cel_cxx::MaybeFuture;
//!
//! // This signature works regardless of async feature
//! fn your_function() -> MaybeFuture<'_, YourType, YourError> {
//!     // Implementation varies by feature
//!     # unimplemented!()
//! }
//! ```
//!
//! ## For Library Users
//!
//! When consuming [`MaybeFuture`] values:
//! ```rust,no_run
//! # use cel_cxx::{Error, MaybeFuture};
//! // Sync mode (no async feature)
//! #[cfg(not(feature = "async"))]
//! fn example_usage<'a>(maybe_future: MaybeFuture<'a, i32, Error>) -> Result<(), Error> {
//!     let result = maybe_future?; // It's just Result<T, E>
//!     Ok(())
//! }
//!
//! // Async mode (with async feature)
//! #[cfg(feature = "async")]
//! async fn example_usage<'a>(maybe_future: MaybeFuture<'a, i32, Error>) -> Result<(), Error> {
//!     match maybe_future {
//!         MaybeFuture::Result(result) => {
//!             let value = result?; // Immediate result
//!         }
//!         MaybeFuture::Future(future) => {
//!             let result = future.await?; // Await the future
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//!
//! # Examples
//!
//! ## Synchronous mode (without `async` feature)
//!
//! ```rust
//! # #[cfg(not(feature = "async"))]
//! # {
//! use cel_cxx::{Error, MaybeFuture};
//!
//! // MaybeFuture is just Result<T, E> in sync mode
//! let result: MaybeFuture<'_, i32, Error> = Ok(42);
//! assert_eq!(result.unwrap(), 42);
//! # }
//! ```
//!
//! ## Asynchronous mode (with `async` feature)
//!
//! ```rust
//! # #[cfg(feature = "async")]
//! # async fn example() {
//! use cel_cxx::MaybeFuture;
//!
//! // Can hold either immediate results or futures
//! let immediate: MaybeFuture<'_, i32, &str> = MaybeFuture::Result(Ok(42));
//! let future_result: MaybeFuture<'_, i32, &str> = MaybeFuture::Future(
//!     Box::pin(async { Ok(100) })
//! );
//!
//! assert!(immediate.is_result());
//! assert!(future_result.is_future());
//! # }
//! ```

//! # Type Definitions
pub use imp::*;

#[cfg(not(feature = "async"))]
mod imp {
    /// A type alias for immediate results when async features are disabled.
    ///
    /// When the `async` feature is not enabled, `MaybeFuture` is simply a type alias
    /// for `Result<T, E>`, meaning all operations return immediately without any
    /// async overhead.
    ///
    /// # Type Parameters
    ///
    /// * `'a` - Lifetime parameter (unused in sync mode but kept for API compatibility)
    /// * `T` - The success type
    /// * `E` - The error type (defaults to [`crate::Error`])
    ///
    /// # Feature-Dependent Behavior
    ///
    /// This type has different definitions depending on feature flags:
    /// - **Without `async` feature**: Type alias to `Result<T, E>` (this definition)
    /// - **With `async` feature**: Enum with `Result` and `Future` variants
    ///
    /// See the [module documentation](crate::maybe_future) for more details about
    /// feature-dependent behavior.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # #[cfg(not(feature = "async"))]
    /// # {
    /// use cel_cxx::MaybeFuture;
    ///
    /// // In sync mode, MaybeFuture is just Result<T, E>
    /// let result: MaybeFuture<'_, i32> = Ok(42);
    /// assert_eq!(result.unwrap(), 42);
    /// # }
    /// ```
    #[cfg_attr(docsrs, doc(cfg(not(feature = "async"))))]
    pub type MaybeFuture<'a, T, E = crate::Error> = Result<T, E>;
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
mod imp {
    use futures::future::{BoxFuture, Future};
    use std::pin::Pin;

    /// A type that can represent either an immediate result or a future.
    ///
    /// When the `async` feature is enabled, `MaybeFuture` is an enum that can hold
    /// either an immediate `Result<T, E>` or a boxed future that will eventually
    /// resolve to a `Result<T, E>`. This allows the same API to handle both
    /// synchronous and asynchronous operations seamlessly.
    ///
    /// # Type Parameters
    ///
    /// * `'a` - The lifetime of the future
    /// * `T` - The success type
    /// * `E` - The error type
    ///
    /// # Variants
    ///
    /// * `Result(Result<T, E>)` - An immediate result
    /// * `Future(BoxFuture<'a, Result<T, E>>)` - A future that will resolve to a result
    ///
    /// # Feature-Dependent Behavior
    ///
    /// This type has different definitions depending on feature flags:
    /// - **Without `async` feature**: Type alias to `Result<T, E>`
    /// - **With `async` feature**: Enum with `Result` and `Future` variants (this definition)
    ///
    /// See the [module documentation](crate::maybe_future) for more details about
    /// feature-dependent behavior.
    ///
    /// # Examples
    ///
    /// ## Working with immediate results
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "async")]
    /// # {
    /// use cel_cxx::MaybeFuture;
    ///
    /// let immediate: MaybeFuture<'_, i32, &str> = MaybeFuture::Result(Ok(42));
    /// assert!(immediate.is_result());
    /// assert_eq!(immediate.into_result().unwrap().unwrap(), 42);
    /// # }
    /// ```
    ///
    /// ## Working with futures
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "async")]
    /// # async fn example() {
    /// use cel_cxx::MaybeFuture;
    /// use futures::future::BoxFuture;
    ///
    /// let future_result: BoxFuture<'_, Result<i32, &str>> =
    ///     Box::pin(async { Ok(42) });
    /// let maybe_future: MaybeFuture<'_, i32, &str> =
    ///     MaybeFuture::Future(future_result);
    ///
    /// assert!(maybe_future.is_future());
    /// let result = maybe_future.into_future().unwrap().await.unwrap();
    /// assert_eq!(result, 42);
    /// # }
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub enum MaybeFuture<'a, T, E> {
        /// An immediate result value.
        Result(Result<T, E>),
        /// A future that will resolve to a result value.
        Future(BoxFuture<'a, Result<T, E>>),
    }

    impl<'a, T, E> MaybeFuture<'a, T, E> {
        /// Returns `true` if this is an immediate result.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # {
        /// use cel_cxx::MaybeFuture;
        ///
        /// let immediate: MaybeFuture<'_, i32, &str> = MaybeFuture::Result(Ok(42));
        /// assert!(immediate.is_result());
        /// # }
        /// ```
        pub fn is_result(&self) -> bool {
            matches!(self, MaybeFuture::Result(_))
        }

        /// Returns `true` if this is a future.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # {
        /// use cel_cxx::MaybeFuture;
        /// use futures::future::BoxFuture;
        ///
        /// let future: BoxFuture<'_, Result<i32, &str>> = Box::pin(async { Ok(42) });
        /// let maybe_future: MaybeFuture<'_, i32, &str> = MaybeFuture::Future(future);
        /// assert!(maybe_future.is_future());
        /// # }
        /// ```
        pub fn is_future(&self) -> bool {
            matches!(self, MaybeFuture::Future(_))
        }

        /// Returns a reference to the result if this is an immediate result.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # {
        /// use cel_cxx::MaybeFuture;
        ///
        /// let immediate: MaybeFuture<'_, i32, &str> = MaybeFuture::Result(Ok(42));
        /// assert_eq!(immediate.result_ref().unwrap().as_ref().unwrap(), &42);
        /// # }
        /// ```
        pub fn result_ref(&self) -> Option<&Result<T, E>> {
            match self {
                MaybeFuture::Result(t) => Some(t),
                MaybeFuture::Future(_) => None,
            }
        }

        /// Returns a reference to the future if this is a future.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # {
        /// use cel_cxx::MaybeFuture;
        /// use futures::future::BoxFuture;
        ///
        /// let future: BoxFuture<'_, Result<i32, &str>> = Box::pin(async { Ok(42) });
        /// let maybe_future: MaybeFuture<'_, i32, &str> = MaybeFuture::Future(future);
        /// assert!(maybe_future.future_ref().is_some());
        /// # }
        /// ```
        pub fn future_ref(&self) -> Option<&BoxFuture<'a, Result<T, E>>> {
            match self {
                MaybeFuture::Result(_) => None,
                MaybeFuture::Future(f) => Some(f),
            }
        }

        /// Returns a mutable reference to the result if this is an immediate result.
        pub fn result_mut(&mut self) -> Option<&mut Result<T, E>> {
            match self {
                MaybeFuture::Result(t) => Some(t),
                MaybeFuture::Future(_) => None,
            }
        }

        /// Returns a mutable reference to the future if this is a future.
        pub fn future_mut(&mut self) -> Option<&mut BoxFuture<'a, Result<T, E>>> {
            match self {
                MaybeFuture::Result(_) => None,
                MaybeFuture::Future(f) => Some(f),
            }
        }

        /// Consumes this value and returns the result if it's an immediate result.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # {
        /// use cel_cxx::MaybeFuture;
        ///
        /// let immediate: MaybeFuture<'_, i32, &str> = MaybeFuture::Result(Ok(42));
        /// let result = immediate.into_result().unwrap();
        /// assert_eq!(result.unwrap(), 42);
        /// # }
        /// ```
        pub fn into_result(self) -> Option<Result<T, E>> {
            match self {
                MaybeFuture::Result(t) => Some(t),
                MaybeFuture::Future(_) => None,
            }
        }

        /// Consumes this value and returns the future if it's a future.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # async fn example() {
        /// use cel_cxx::MaybeFuture;
        /// use futures::future::BoxFuture;
        ///
        /// let future: BoxFuture<'_, Result<i32, &str>> = Box::pin(async { Ok(42) });
        /// let maybe_future: MaybeFuture<'_, i32, &str> = MaybeFuture::Future(future);
        /// let extracted_future = maybe_future.into_future().unwrap();
        /// let result = extracted_future.await.unwrap();
        /// assert_eq!(result, 42);
        /// # }
        /// ```
        pub fn into_future(self) -> Option<BoxFuture<'a, Result<T, E>>> {
            match self {
                MaybeFuture::Result(_) => None,
                MaybeFuture::Future(f) => Some(f),
            }
        }

        /// Extracts the result, panicking if this is a future.
        ///
        /// # Panics
        ///
        /// Panics if this `MaybeFuture` contains a future rather than an immediate result.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # {
        /// use cel_cxx::MaybeFuture;
        ///
        /// let immediate: MaybeFuture<'_, i32, &str> = MaybeFuture::Result(Ok(42));
        /// let result = immediate.expect_result("Expected immediate result");
        /// assert_eq!(result.unwrap(), 42);
        /// # }
        /// ```
        pub fn expect_result(self, msg: &str) -> Result<T, E> {
            self.into_result().expect(msg)
        }

        /// Extracts the future, panicking if this is an immediate result.
        ///
        /// # Panics
        ///
        /// Panics if this `MaybeFuture` contains an immediate result rather than a future.
        ///
        /// # Examples
        ///
        /// ```rust,no_run
        /// # #[cfg(feature = "async")]
        /// # async fn example() {
        /// use cel_cxx::MaybeFuture;
        /// use futures::future::BoxFuture;
        ///
        /// let future: BoxFuture<'_, Result<i32, &str>> = Box::pin(async { Ok(42) });
        /// let maybe_future: MaybeFuture<'_, i32, &str> = MaybeFuture::Future(future);
        /// let extracted_future = maybe_future.expect_future("Expected future");
        /// let result = extracted_future.await.unwrap();
        /// assert_eq!(result, 42);
        /// # }
        /// ```
        pub fn expect_future(self, msg: &str) -> BoxFuture<'a, Result<T, E>> {
            self.into_future().expect(msg)
        }

        /// Extracts the result, panicking if this is a future.
        ///
        /// This is equivalent to `expect_result` but with a default panic message.
        ///
        /// # Panics
        ///
        /// Panics if this `MaybeFuture` contains a future rather than an immediate result.
        pub fn unwrap_result(self) -> Result<T, E> {
            self.into_result().unwrap()
        }

        /// Extracts the future, panicking if this is an immediate result.
        ///
        /// This is equivalent to `expect_future` but with a default panic message.
        ///
        /// # Panics
        ///
        /// Panics if this `MaybeFuture` contains an immediate result rather than a future.
        pub fn unwrap_future(self) -> BoxFuture<'a, Result<T, E>> {
            self.into_future().unwrap()
        }
    }

    impl<'a, T: std::fmt::Debug, E: std::fmt::Debug> std::fmt::Debug for MaybeFuture<'a, T, E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MaybeFuture::Result(t) => t.fmt(f),
                MaybeFuture::Future(_) => f.write_str("<future>"),
            }
        }
    }

    impl<'a, T, E> From<Result<T, E>> for MaybeFuture<'a, T, E> {
        fn from(t: Result<T, E>) -> Self {
            MaybeFuture::Result(t)
        }
    }

    impl<'a, Fut, T, E> From<Pin<Box<Fut>>> for MaybeFuture<'a, T, E>
    where
        Fut: Future<Output = Result<T, E>> + Send + 'a,
    {
        fn from(f: Pin<Box<Fut>>) -> Self {
            MaybeFuture::Future(f)
        }
    }
}
