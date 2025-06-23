/// Marker trait for runtime types.
/// 
/// This trait is used to mark types that represent different async runtimes
/// (such as Tokio or async-std) or the absence of an async runtime.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::marker::RuntimeMarker;
/// 
/// // The unit type represents no async runtime
/// fn sync_example<R: RuntimeMarker>() {
///     // This works with any runtime marker
/// }
/// ```
pub trait RuntimeMarker: private::Sealed {}

/// Implementation for the unit type (no async runtime).
impl RuntimeMarker for () {}

/// Marker trait for function types.
/// 
/// This trait is used to distinguish between synchronous and asynchronous
/// functions in the type system. It allows the library to provide different
/// behavior based on whether functions are sync or async.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::marker::FnMarker;
/// 
/// // The unit type represents synchronous functions
/// fn sync_function_example<F: FnMarker>() {
///     // This works with any function marker
/// }
/// ```
pub trait FnMarker: private::Sealed {}

/// Trait for aggregating function markers.
/// 
/// This trait is used to combine function markers from different sources
/// (environment, activation, etc.) to determine the overall function
/// capabilities of a context.
/// 
/// # Type Parameters
/// 
/// * `Other` - The other function marker to aggregate with
/// 
/// # Associated Types
/// 
/// * `Output` - The resulting function marker after aggregation
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::marker::{FnMarker, FnMarkerAggr};
/// 
/// fn aggregate_example<F1, F2>() 
/// where 
///     F1: FnMarker + FnMarkerAggr<F2>,
///     F2: FnMarker,
/// {
///     // F1::Output represents the combined capabilities
/// }
/// ```
pub trait FnMarkerAggr<Other: FnMarker> : FnMarker {
    /// The resulting function marker after aggregation.
    type Output: FnMarker;
}

/// Trait for determining function result types.
/// 
/// This trait maps function markers to their corresponding result types.
/// For synchronous functions, this is just the value type. For asynchronous
/// functions, this wraps the value in a Future.
/// 
/// # Type Parameters
/// 
/// * `'a` - Lifetime parameter for the result
/// * `T` - The base type that will be wrapped according to the marker
/// 
/// # Associated Types
/// 
/// * `Output` - The actual result type (T for sync, `Future<T>` for async)
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::marker::{FnMarker, FnResult};
/// 
/// fn result_example<'a, F, T>() -> F::Output
/// where 
///     F: FnMarker + FnResult<'a, T>,
/// {
///     // Return type depends on the function marker
///     todo!()
/// }
/// ```
pub trait FnResult<'a, T> : FnMarker {
    /// The output type for this function marker and base type.
    type Output;
}

/// Synchronous function marker implementation.
/// 
/// The unit type `()` represents synchronous functions.
impl FnMarker for () {}

/// Synchronous function aggregation.
/// 
/// Aggregating with synchronous functions yields synchronous functions.
impl FnMarkerAggr<()> for () {
    type Output = ();
}

/// Synchronous function results.
/// 
/// For synchronous functions, the result type is just the base type.
impl<'a, T> FnResult<'a, T> for () {
    type Output = T;
}

/// Async-specific marker types and implementations.
/// 
/// This module contains the marker types and trait implementations for
/// asynchronous function support. It's only available when the `async`
/// feature is enabled.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use async_marker::*;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
mod async_marker {
    use super::{RuntimeMarker, FnMarker, FnMarkerAggr, FnResult};
    use crate::r#async::Runtime;

    /// Runtime marker implementation for async runtime types.
    impl<T: Runtime> RuntimeMarker for T {}

    /// Marker trait for asynchronous functions.
    /// 
    /// This trait extends `FnMarker` to specifically mark asynchronous
    /// function types.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// # #[cfg(feature = "async")]
    /// # fn example() {
    /// use cel_cxx::marker::{FnMarkerAsync, Async};
    /// 
    /// fn async_function_example<F: FnMarkerAsync>() {
    ///     // This only works with async function markers
    /// }
    /// 
    /// // Async is an example of FnMarkerAsync
    /// async_function_example::<Async>();
    /// # }
    /// ```
    pub trait FnMarkerAsync: FnMarker {}

    /// Marker type for asynchronous functions.
    /// 
    /// This is a zero-sized type used to mark contexts that support
    /// asynchronous function evaluation.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// # #[cfg(feature = "async")]
    /// # fn example() -> Result<(), cel_cxx::Error> {
    /// use cel_cxx::{Env, marker::Async};
    /// 
    /// // Create an async-capable environment
    /// let env = Env::builder()
    ///     .force_async()
    ///     .build()?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// # }
    /// ```
    #[allow(missing_debug_implementations)]
    pub enum Async {}

    /// Function marker implementation for async functions.
    impl FnMarker for Async {}

    /// Async function marker trait implementation.
    impl FnMarkerAsync for Async {}

    /// Aggregating sync with async yields async.
    /// 
    /// When synchronous and asynchronous capabilities are combined,
    /// the result supports asynchronous operations.
    impl FnMarkerAggr<()> for Async {
        type Output = Async;
    }

    /// Aggregating async with sync yields async.
    /// 
    /// When asynchronous and synchronous capabilities are combined,
    /// the result supports asynchronous operations.
    impl FnMarkerAggr<Async> for () {
        type Output = Async;
    }

    /// Aggregating async with async yields async.
    /// 
    /// Combining two async contexts remains async.
    impl FnMarkerAggr<Async> for Async {
        type Output = Async;
    }

    /// Asynchronous function results.
    /// 
    /// For asynchronous functions, the result type is wrapped in a BoxFuture.
    /// This allows async functions to return futures that can be awaited.
    impl<'a, T> FnResult<'a, T> for Async {
        type Output = futures::future::BoxFuture<'a, T>;
    }
}

/// Private sealed trait to prevent external implementations.
/// 
/// This module ensures that marker traits can only be implemented by types
/// within this crate, providing type safety and preventing misuse.
mod private {
    #![allow(unused)]
    use super::*;
    #[cfg(feature = "async")]
    use crate::r#async::Runtime;

    /// Sealed trait to prevent external implementations.
    /// 
    /// This trait is implemented only for types that are allowed to be
    /// markers in the cel-cxx type system.
    pub trait Sealed: 'static {}
    
    /// Unit type can be a marker (represents sync operations).
    impl Sealed for () {}

    /// Async marker type can be a marker.
    #[cfg(feature = "async")]
    impl Sealed for Async {}

    /// Runtime types can be runtime markers.
    #[cfg(feature = "async")]
    impl<T: Runtime> Sealed for T {}
}

#[cfg(test)]
mod test {
    #![allow(unused)]
    use super::*;

    fn test_fn_marker_aggregation<M1, M2>() -> <M1 as FnMarkerAggr<M2>>::Output
    where
        M1: FnMarker + FnMarkerAggr<M2>,
        M2: FnMarker,
    {
        todo!()
    }

    fn test_eval_result<'a, M, T>() -> <M as FnResult<'a, T>>::Output
    where
        M: FnResult<'a, T> + FnMarker,
    {
        todo!()
    }

    fn t1() {
        let x = test_fn_marker_aggregation::<(), ()>();
        let v = test_eval_result::<(), i32>();

        #[cfg(feature = "async")]
        {
            use crate::marker::Async;

            let x = test_fn_marker_aggregation::<(), Async>();
            let x = test_fn_marker_aggregation::<Async, ()>();
            let x = test_fn_marker_aggregation::<Async, Async>();

            let v = test_eval_result::<Async, i32>();
        }
    }
}
