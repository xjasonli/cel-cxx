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
    entries: HashMap<String, FunctionOverloads<FunctionImpl<'f>>>,
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
    /// Binds a function implementation.
    ///
    /// Binds a callable function that can be invoked during CEL expression evaluation.
    /// The function is automatically converted to the appropriate runtime representation.
    ///
    /// # Type Parameters
    ///
    /// - `F`: Function type, must implement [`FnImpl`]
    /// - `M`: Function marker (sync or async)
    /// - `E`: Error type, must be convertible to [`Error`]
    /// - `R`: Return type, must implement [`IntoValue`] and [`TypedValue`]
    /// - `A`: Argument tuple type, must implement [`TypedArguments`]
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
    /// ```rust,no_run
    /// use cel_cxx::{FunctionBindings, Error};
    ///
    /// let mut bindings = FunctionBindings::new();
    ///
    /// // Bind global function
    /// fn power(base: i64, exp: i64) -> Result<i64, Error> {
    ///     Ok(base.pow(exp as u32))
    /// }
    /// bindings.bind("power", false, power)?;
    ///
    /// // Bind member function
    /// fn is_empty(s: String) -> Result<bool, Error> {
    ///     Ok(s.is_empty())
    /// }
    /// bindings.bind("is_empty", true, is_empty)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn bind<F, M, E, R, A>(
        &mut self, name: impl Into<String>, member: bool, f: F
    ) -> Result<&mut Self, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: FnMarker,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'f,
    {
        let name = name.into();
        let entry = self.entries
            .entry(name)
            .or_insert_with(FunctionOverloads::new);
        entry.add(member, f)?;
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
    pub fn bind_member<F, M, E, R, A>(
        &mut self, name: impl Into<String>, f: F
    ) -> Result<&mut Self, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: FnMarker,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'f,
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
    pub fn bind_global<F, M, E, R, A>(
        &mut self, name: impl Into<String>, f: F
    ) -> Result<&mut Self, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: FnMarker,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'f,
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
    pub fn find(&self, name: &str) -> Option<&FunctionOverloads<FunctionImpl<'f>>> {
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
    pub fn find_mut(&mut self, name: &str) -> Option<&mut FunctionOverloads<FunctionImpl<'f>>> {
        self.entries.get_mut(name)
    }

    /// Returns an iterator over all function entries.
    ///
    /// The iterator yields `(name, overloads)` pairs for all bound functions.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &FunctionOverloads<FunctionImpl<'f>>)> {
        self.entries
            .iter()
            .map(|(name, entry)| (name.as_str(), entry))
    }

    /// Returns a mutable iterator over all function entries.
    ///
    /// The iterator yields `(name, overloads)` pairs and allows modifying the overloads.
    pub fn entries_mut(&mut self) -> impl Iterator<Item = (&str, &mut FunctionOverloads<FunctionImpl<'f>>)> {
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
