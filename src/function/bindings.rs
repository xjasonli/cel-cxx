//! Function binding utilities for runtime variable capture.
//!
//! This module provides utilities for binding functions with captured
//! environment variables, enabling closures to access external state
//! during CEL expression evaluation.
//!
//! # Use Cases
//!
//! - **Configuration binding**: Capture configuration values in closures
//! - **Database connections**: Bind database handles to query functions
//! - **External services**: Capture service clients for API calls
//! - **State management**: Access mutable state from function implementations

use std::collections::HashMap;
use super::*;

/// Runtime function bindings.
///
/// `FunctionBindings` manages function implementations during CEL expression evaluation.
/// Unlike [`FunctionRegistry`] which is used at compile time, `FunctionBindings` provides
/// actual callable function implementations for runtime execution.
///
/// Each function name can have multiple overloads with different signatures, and each overload
/// can be either a global function or a member function.
///
/// # Lifetime Parameters
///
/// - `'f`: Lifetime of function implementations, allowing closures valid within specific scopes
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{FunctionBindings, Error};
///
/// let mut bindings = FunctionBindings::new();
///
/// // Bind global functions
/// fn add(a: i64, b: i64) -> Result<i64, Error> {
///     Ok(a + b)
/// }
/// bindings.bind_global("add", add)?;
///
/// // Bind member functions
/// fn string_length(s: String) -> Result<i64, Error> {
///     Ok(s.len() as i64)
/// }
/// bindings.bind_member("length", string_length)?;
///
/// // Bind closures
/// let multiplier = 10;
/// let closure = move |x: i64| -> Result<i64, Error> {
///     Ok(x * multiplier)
/// };
/// bindings.bind_global("multiply_by_ten", closure)?;
/// # Ok::<(), cel_cxx::Error>(())
/// ```
///
/// [`FunctionRegistry`]: crate::function::FunctionRegistry
#[derive(Debug, Default)]
pub struct FunctionBindings<'f> {
    entries: HashMap<String, FunctionOverloads<Function<'f>>>,
}

impl<'f> FunctionBindings<'f> {
    /// Creates a new empty function bindings.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl<'f> FunctionBindings<'f> {
    /// Binds a function implementation with zero-annotation type inference.
    ///
    /// Binds a callable function that can be invoked during CEL expression evaluation.
    /// The function signature is automatically inferred from the Rust function type, enabling
    /// seamless runtime function binding without manual type annotations.
    ///
    /// # Zero-Annotation Features
    ///
    /// - **Automatic type inference**: Function signature extracted from Rust function type
    /// - **Runtime binding**: Functions are immediately available for CEL evaluation
    /// - **Lifetime handling**: Safe conversion of borrowed arguments like `&str`
    /// - **Error conversion**: Automatic conversion of `Result<T, E>` return types
    /// - **Async support**: Seamless handling of both sync and async functions
    ///
    /// # Type Parameters
    ///
    /// - `F`: Function type, must implement [`IntoFunction`]
    /// - `Fm`: Function marker (sync or async), automatically inferred
    /// - `Args`: Argument tuple type, automatically inferred from function signature
    ///
    /// # Parameters
    ///
    /// - `name`: Function name
    /// - `member`: Whether this is a member function (true) or global function (false)
    /// - `f`: Function implementation
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to support method chaining
    ///
    /// # Examples
    ///
    /// ## Basic function binding with automatic type inference
    ///
    /// ```rust,no_run
    /// use cel_cxx::FunctionBindings;
    ///
    /// let mut bindings = FunctionBindings::new();
    ///
    /// // Zero-annotation function binding
    /// bindings.bind("multiply", false, |a: i64, b: i64| a * b)?;
    /// bindings.bind("upper", true, |s: String| s.to_uppercase())?;
    /// bindings.bind("contains", true, |s: &str, substr: &str| s.contains(substr))?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    ///
    /// ## Runtime function binding with closures
    ///
    /// ```rust,no_run
    /// use cel_cxx::FunctionBindings;
    ///
    /// let mut bindings = FunctionBindings::new();
    /// let multiplier = 10;
    ///
    /// // Bind closure that captures environment
    /// bindings.bind("scale", false, move |x: i64| x * multiplier)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn bind<F, Fm, Args>(
        &mut self, name: impl Into<String>, member: bool, f: F
    ) -> Result<&mut Self, Error>
    where
        F: IntoFunction<'f, Fm, Args>,
        Fm: FnMarker,
        Args: Arguments,
    {
        let name = name.into();
        let entry = self.entries
            .entry(name)
            .or_insert_with(FunctionOverloads::new);
        entry.add(member, f.into_function())?;
        Ok(self)
    }

    /// Binds a member function implementation.
    ///
    /// Convenience method for binding member functions that can be called as methods
    /// on values (e.g., `"hello".upper()`).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{FunctionBindings, Error};
    ///
    /// let mut bindings = FunctionBindings::new();
    ///
    /// fn reverse(s: String) -> Result<String, Error> {
    ///     Ok(s.chars().rev().collect())
    /// }
    ///
    /// bindings.bind_member("reverse", reverse)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn bind_member<F, Fm, Args>(
        &mut self, name: impl Into<String>, f: F
    ) -> Result<&mut Self, Error>
    where
        F: IntoFunction<'f, Fm, Args>,
        Fm: FnMarker,
        Args: Arguments,
    {
        self.bind(name, true, f)
    }
    
    /// Binds a global function implementation.
    ///
    /// Convenience method for binding global functions that can be called from any context.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{FunctionBindings, Error};
    ///
    /// let mut bindings = FunctionBindings::new();
    ///
    /// fn min(a: i64, b: i64) -> Result<i64, Error> {
    ///     Ok(a.min(b))
    /// }
    ///
    /// bindings.bind_global("min", min)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn bind_global<F, Fm, Args>(
        &mut self, name: impl Into<String>, f: F
    ) -> Result<&mut Self, Error>
    where
        F: IntoFunction<'f, Fm, Args>,
        Fm: FnMarker,
        Args: Arguments,
    {
        self.bind(name, false, f)
    }

    /// Finds a function overload set by name.
    ///
    /// # Parameters
    ///
    /// - `name`: Function name to search for
    ///
    /// # Returns
    ///
    /// Returns `Some(&FunctionOverloads)` if found, `None` otherwise
    pub fn find(&self, name: &str) -> Option<&FunctionOverloads<Function<'f>>> {
        self.entries.get(name)
    }

    /// Finds a mutable function overload set by name.
    ///
    /// # Parameters
    ///
    /// - `name`: Function name to search for
    ///
    /// # Returns
    ///
    /// Returns `Some(&mut FunctionOverloads)` if found, `None` otherwise
    pub fn find_mut(&mut self, name: &str) -> Option<&mut FunctionOverloads<Function<'f>>> {
        self.entries.get_mut(name)
    }

    /// Returns an iterator over all function entries.
    ///
    /// The iterator yields `(name, overloads)` pairs for all bound functions.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &FunctionOverloads<Function<'f>>)> {
        self.entries
            .iter()
            .map(|(name, entry)| (name.as_str(), entry))
    }

    /// Returns a mutable iterator over all function entries.
    ///
    /// The iterator yields `(name, overloads)` pairs and allows modifying the overloads.
    pub fn entries_mut(&mut self) -> impl Iterator<Item = (&str, &mut FunctionOverloads<Function<'f>>)> {
        self.entries
            .iter_mut()
            .map(|(name, entry)| (name.as_str(), entry))
    }

    /// Removes a function by name.
    ///
    /// # Parameters
    ///
    /// - `name`: Function name to remove
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if successfully removed, or an error if the function doesn't exist
    pub fn remove(&mut self, name: &str) -> Result<(), Error> {
        if self.entries.remove(name).is_none() {
            return Err(Error::not_found(format!("Function {} not found", name)));
        }
        Ok(())
    }

    /// Clears all function bindings.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
