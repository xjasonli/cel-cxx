//! Compiled CEL program evaluation.
//!
//! This module provides the [`Program`] type, which represents a compiled CEL expression
//! ready for evaluation. Programs are created by compiling CEL expressions using an
//! [`Env`](crate::Env) and can be evaluated multiple times with different variable
//! bindings (activations).
//!
//! # Key Features
//!
//! - **Compiled expressions**: CEL expressions are parsed and compiled once, then evaluated many times
//! - **Type safety**: Programs know their return type at compile time
//! - **Variable binding**: Support for dynamic variable values through activations
//! - **Async support**: Programs can contain and evaluate async functions
//! - **Runtime selection**: Choose between different async runtimes (Tokio, async-std)
//!
//! # Program Types
//!
//! Programs are parameterized by function and runtime markers:
//!
//! - **`Program<'f>`**: Synchronous program with sync functions only
//! - **`AsyncProgram<'f>`**: Program that can contain async functions
//! - **`Program<'f, Fm, Rm>`**: Full type with function marker `Fm` and runtime marker `Rm`
//!
//! # Evaluation Model
//!
//! Programs use an activation-based evaluation model:
//!
//! 1. **Compilation**: Parse and type-check the CEL expression
//! 2. **Activation**: Bind variables and functions for a specific evaluation
//! 3. **Evaluation**: Execute the compiled expression with the bound values
//!
//! # Examples
//!
//! ## Basic synchronous evaluation
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! // Create environment and compile expression
//! let env = Env::builder()
//!     .declare_variable::<String>("user_name")
//!     .declare_variable::<i64>("user_age")
//!     .build()?;
//!
//! let program = env.compile("'Hello ' + user_name + ', you are ' + string(user_age)")?;
//!
//! // Create activation with variable bindings
//! let activation = Activation::new()
//!     .bind_variable("user_name", "Alice".to_string())?
//!     .bind_variable("user_age", 30i64)?;
//!
//! // Evaluate the program
//! let result = program.evaluate(activation)?;
//! println!("{}", result); // "Hello Alice, you are 30"
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! ## Working with functions
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! // Register custom function
//! let env = Env::builder()
//!     .register_global_function("multiply", |a: i64, b: i64| a * b)?
//!     .declare_variable::<i64>("x")?
//!     .build()?;
//!
//! let program = env.compile("multiply(x, 2) + 1")?;
//!
//! let activation = Activation::new()
//!     .bind_variable("x", 21i64)?;
//!
//! let result = program.evaluate(activation)?;
//! assert_eq!(result, Value::Int(43));
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! ## Async evaluation
//!
//! ```rust,no_run
//! # #[cfg(feature = "async")]
//! # async fn example() -> Result<(), cel_cxx::Error> {
//! use cel_cxx::*;
//! use cel_cxx::r#async::Tokio;
//!
//! // Register async function
//! let env = Env::builder()
//!     .register_global_function("fetch_data", |url: String| async move {
//!         // Simulate async work
//!         tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
//!         format!("Data from {}", url)
//!     })?
//!     .build()?;
//!
//! let program = env.compile("fetch_data('https://api.example.com')")?
//!     .use_runtime::<Tokio>();
//!
//! let result = program.evaluate(()).await?;
//! println!("{}", result); // "Data from https://api.example.com"
//! # Ok(())
//! # }
//! ```
//!
//! ## Reusing programs
//!
//! ```rust,no_run
//! use cel_cxx::*;
//!
//! let env = Env::builder()
//!     .declare_variable::<i64>("value")
//!     .build()?;
//!
//! // Compile once
//! let program = env.compile("value * value")?;
//!
//! // Evaluate multiple times with different values
//! for i in 1..=5 {
//!     let activation = Activation::new()
//!         .bind_variable("value", i)?;
//!     
//!     let result = program.evaluate(activation)?;
//!     println!("{} * {} = {}", i, i, result);
//! }
//! # Ok::<(), cel_cxx::Error>(())
//! ```

use std::sync::Arc;
mod inner;
mod eval_dispatch;
use eval_dispatch::{EvalDispatcher, EvalDispatch};
use super::{ValueType, Value, Error, ActivationInterface};
use crate::{FnMarker, FnMarkerAggr, RuntimeMarker, FnResult};

pub(crate) use inner::ProgramInner;

#[cfg(feature = "async")]
use crate::marker::Async;

/// Compiled CEL program ready for evaluation.
/// 
/// A `Program` represents a compiled CEL expression that can be evaluated
/// multiple times with different variable bindings (activations). Programs
/// are created by compiling CEL expressions using an [`Env`](crate::Env).
/// 
/// # Type Parameters
/// 
/// - `'f`: Lifetime of functions registered in the environment
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
/// 
/// let activation = Activation::new()
///     .bind_variable("name", "World")
///     .unwrap();
///     
/// let result = program.evaluate(activation).unwrap();
/// ```
/// 
/// ## Type Information
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// let env = Env::builder().build().unwrap();
/// let program = env.compile("42").unwrap();
/// 
/// // Check the return type
/// println!("Return type: {:?}", program.return_type());
/// ```
pub struct Program<'f, Fm: FnMarker = (), Rm: RuntimeMarker = ()> {
    pub(crate) inner: Arc<ProgramInner<'f>>,
    pub(crate) _fn_marker: std::marker::PhantomData<Fm>,
    pub(crate) _rt_marker: std::marker::PhantomData<Rm>,
}

/// Type alias for asynchronous CEL programs.
/// 
/// This is a convenience type alias for programs that support asynchronous
/// evaluation with async functions and/or async runtime.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub type AsyncProgram<'f, Rm = ()> = Program<'f, Async, Rm>;

impl<'f, Fm: FnMarker, Rm: RuntimeMarker> Program<'f, Fm, Rm> {
    /// Returns the return type of this program.
    /// 
    /// This method returns the CEL type that this program will produce when
    /// evaluated. The type is determined during compilation based on the
    /// expression and the declared variables and functions.
    /// 
    /// # Returns
    /// 
    /// A reference to the [`ValueType`] that this program returns.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder().build().unwrap();
    /// let program = env.compile("42").unwrap();
    /// 
    /// println!("Return type: {:?}", program.return_type());
    /// // Output: Return type: Int
    /// ```
    pub fn return_type(&self) -> &ValueType {
        self.inner.return_type()
    }

    /// Evaluates the program with the given activation.
    /// 
    /// This method evaluates the compiled CEL expression using the variable
    /// and function bindings provided in the activation. The return type
    /// of this method depends on the program and activation markers:
    /// 
    /// - For synchronous programs: Returns `Result<Value, Error>`
    /// - For asynchronous programs: Returns `BoxFuture<Result<Value, Error>>`
    /// 
    /// # Arguments
    /// 
    /// * `activation` - The activation containing variable and function bindings
    /// 
    /// # Type Parameters
    /// 
    /// * `A` - The activation type
    /// * `Afm` - The activation's function marker type
    /// 
    /// # Examples
    /// 
    /// ## Synchronous Evaluation
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder()
    ///     .declare_variable::<i64>("x")
    ///     .build()
    ///     .unwrap();
    ///     
    /// let program = env.compile("x * 2").unwrap();
    /// 
    /// let activation = Activation::new()
    ///     .bind_variable("x", 21i64)
    ///     .unwrap();
    ///     
    /// let result = program.evaluate(activation).unwrap();
    /// // result == Value::Int(42)
    /// ```
    /// 
    /// ## With Empty Activation
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let env = Env::builder().build().unwrap();
    /// let program = env.compile("1 + 2 * 3").unwrap();
    /// 
    /// let result = program.evaluate(()).unwrap();
    /// // result == Value::Int(7)
    /// ```
    pub fn evaluate<'a, A, Afm>(
        &self, activation: A
    ) -> <<Afm as FnMarkerAggr<Fm>>::Output as FnResult<'f, Result<Value, Error>>>::Output
    where
        'f: 'a,
        A: ActivationInterface<'f, Afm> + 'a,
        Afm: FnMarkerAggr<Fm>,
        <Afm as FnMarkerAggr<Fm>>::Output: FnResult<'f, Result<Value, Error>>,
        EvalDispatcher<<Afm as FnMarkerAggr<Fm>>::Output, Rm>:
            EvalDispatch<'f, A, Afm,
                Output = <<Afm as FnMarkerAggr<Fm>>::Output as FnResult<'f, Result<Value, Error>>>::Output
            >,
    {
        EvalDispatcher::<<Afm as FnMarkerAggr<Fm>>::Output, Rm>::new()
            .eval(self.inner.clone(), activation)
    }
}

/// Async-specific methods for programs.
/// 
/// These methods are only available when the `async` feature is enabled
/// and provide utilities for working with async runtimes.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
const _: () = {
    use crate::r#async::*;

    impl<'f, Fm: FnMarker> Program<'f, Fm, ()> {
        /// Configures this program to use a specific async runtime.
        /// 
        /// This method allows you to specify which async runtime should be
        /// used for evaluating this program when it contains async functions.
        /// 
        /// # Type Parameters
        /// 
        /// * `Rt` - The runtime type to use
        /// 
        /// # Examples
        /// 
        /// ```rust,no_run
        /// # #[cfg(feature = "tokio")]
        /// # fn example() {
        /// use cel_cxx::*;
        /// use cel_cxx::r#async::Tokio;
        /// 
        /// let env = Env::builder().build().unwrap();
        /// let program = env.compile("42").unwrap();
        /// 
        /// let async_program = program.use_runtime::<Tokio>();
        /// # }
        /// ```
        pub fn use_runtime<Rt: Runtime>(self) -> Program<'f, Fm, Rt> {
            Program {
                inner: self.inner,
                _fn_marker: self._fn_marker,
                _rt_marker: std::marker::PhantomData,
            }
        }

        /// Configures this program to use the Tokio async runtime.
        /// 
        /// This is a convenience method equivalent to `use_runtime::<Tokio>()`.
        /// 
        /// # Examples
        /// 
        /// ```rust,no_run
        /// # #[cfg(feature = "tokio")]
        /// # fn example() {
        /// use cel_cxx::*;
        /// 
        /// let env = Env::builder().build().unwrap();
        /// let program = env.compile("42").unwrap();
        /// 
        /// let tokio_program = program.use_tokio();
        /// # }
        /// ```
        #[cfg(feature = "tokio")]
        #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
        pub fn use_tokio(self) -> Program<'f, Fm, Tokio> {
            self.use_runtime::<Tokio>()
        }

        /// Configures this program to use the async-std runtime.
        /// 
        /// This is a convenience method equivalent to `use_runtime::<AsyncStd>()`.
        /// 
        /// # Examples
        /// 
        /// ```rust,no_run
        /// # #[cfg(feature = "async-std")]
        /// # fn example() {
        /// use cel_cxx::*;
        /// 
        /// let env = Env::builder().build().unwrap();
        /// let program = env.compile("42").unwrap();
        /// 
        /// let async_std_program = program.use_async_std();
        /// # }
        /// ```
        #[cfg(feature = "async-std")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async-std")))]
        pub fn use_async_std(self) -> Program<'f, Fm, AsyncStd> {
            self.use_runtime::<AsyncStd>()
        }
    }
};

impl<'f, Fm: FnMarker, Rm: RuntimeMarker> std::fmt::Debug for Program<'f, Fm, Rm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Program")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<'f, Fm: FnMarker, Rm: RuntimeMarker> Clone for Program<'f, Fm, Rm> {
    fn clone(&self) -> Self {
        Program {
            inner: self.inner.clone(),
            _fn_marker: self._fn_marker,
            _rt_marker: self._rt_marker,
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused)]

    use super::*;
    use crate::Activation;

    fn assert_eval_type<'f>(
        program: Program<'f, (), ()>,
        activation: Activation<'f, ()>,
    ) {
        let _result: Result<Value, Error> = program.evaluate(activation);
    }

    #[cfg(feature = "async")]
    const _: () = {
        use futures::future::BoxFuture;
        use crate::Async;
        use crate::r#async::Tokio;

        #[cfg(feature = "tokio")]
        const _: () = {
            fn assert_eval_type_async1<'f>(
                program: Program<'f, Async, Tokio>,
                activation: Activation<'f, ()>,
            ) {
                let _result: BoxFuture<'f, Result<Value, Error>> = program
                    .evaluate(activation);
            }

            fn assert_eval_type_async2<'f>(
                program: Program<'f, (), Tokio>,
                activation: Activation<'f, Async>,
            ) {
                let _result: BoxFuture<'f, Result<Value, Error>> = program
                    .evaluate(activation);
            }

            fn assert_eval_type_async3<'f>(
                program: Program<'f, Async, Tokio>,
                activation: Activation<'f, Async>,
            ) {
                let _result: BoxFuture<'f, Result<Value, Error>> = program
                    .evaluate(activation);
            }

            fn assert_eval_type_async4<'f>(
                program: Program<'f, Async, ()>,
                activation: Activation<'f, Async>,
            ) {
                let _result: BoxFuture<'f, Result<Value, Error>> = program
                    .use_tokio()
                    .evaluate(activation);
            }
        };
    };
}
