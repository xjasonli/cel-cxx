use std::sync::Arc;
use std::collections::HashMap;
use crate::types::Type;
use crate::values::*;
use crate::marker::*;
use crate::Error;
use super::provider::*;

/// Runtime variable bindings.
///
/// `VariableBindings` manages variable bindings during CEL expression evaluation.
/// It maps variable names to concrete values or value providers.
///
/// Unlike compile-time [`VariableRegistry`], `VariableBindings` provides actual variable values
/// at runtime and supports two types of bindings:
///
/// - **Value bindings**: Directly stored variable values
/// - **Provider bindings**: Function-computed variable values through lazy evaluation
///
/// # Lifetime Parameters
///
/// - `'f`: Lifetime of function providers, allowing closures valid within specific scopes
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::VariableBindings;
///
/// let mut bindings = VariableBindings::new();
///
/// // Bind static values
/// bindings.bind("name", "Alice")?;
/// bindings.bind("age", 30i64)?;
///
/// // Bind dynamic value providers
/// bindings.bind_provider("current_time", || {
///     Ok(std::time::SystemTime::now())
/// })?;
///
/// assert_eq!(bindings.len(), 3);
/// # Ok::<(), cel_cxx::Error>(())
/// ```
///
/// [`VariableRegistry`]: crate::variable::VariableRegistry
#[derive(Debug, Default)]
pub struct VariableBindings<'f> {
    entries: HashMap<String, VariableBinding<'f>>,
}

impl<'f> VariableBindings<'f> {
    /// Creates a new empty variable bindings.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl<'f> VariableBindings<'f> {
    /// Binds a variable value.
    ///
    /// Binds a variable name to a concrete value. The value must implement [`IntoValue`] and [`TypedValue`] traits.
    ///
    /// # Type Parameters
    ///
    /// - `T`: Value type, must implement `IntoValue + TypedValue`
    ///
    /// # Parameters
    ///
    /// - `name`: Variable name
    /// - `value`: Variable value
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to support method chaining
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::VariableBindings;
    ///
    /// let mut bindings = VariableBindings::new();
    /// bindings
    ///     .bind("user_id", 123i64)?
    ///     .bind("is_admin", true)?
    ///     .bind("username", "alice")?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn bind<T>(
        &mut self, name: impl Into<String>, value: T
    ) -> Result<&mut Self, Error>
    where
        T: IntoValue + TypedValue,
    {
        self.entries.insert(name.into(), VariableBinding::new_value(value));
        Ok(self)
    }

    /// Binds a variable value provider.
    ///
    /// Binds a variable name to a function provider that is called each time the variable is accessed.
    /// This is useful for dynamically computed values (such as current time, random numbers, etc.).
    ///
    /// # Type Parameters
    ///
    /// - `F`: Provider function type
    /// - `M`: Function marker (sync or async)
    /// - `E`: Error type, must be convertible to [`Error`]
    /// - `T`: Return value type, must implement `IntoValue + TypedValue`
    ///
    /// # Parameters
    ///
    /// - `name`: Variable name
    /// - `provider`: Value provider function
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to support method chaining
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::VariableBindings;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let mut bindings = VariableBindings::new();
    ///
    /// // Bind dynamic timestamp
    /// bindings.bind_provider("timestamp", || {
    ///     Ok(SystemTime::now()
    ///         .duration_since(UNIX_EPOCH)
    ///         .unwrap()
    ///         .as_secs() as i64)
    /// })?;
    ///
    /// // Bind random number
    /// bindings.bind_provider("random", || {
    ///     Ok(rand::random::<f64>())
    /// })?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn bind_provider<F, M, E, T>(
        &mut self, name: impl Into<String>, provider: F
    ) -> Result<&mut Self, Error>
    where
        F: Provider<M, E, T> + Send + Sync + 'f,
        M: FnMarker,
        E: Into<Error> + Send + Sync + 'static,
        T: IntoValue + TypedValue + 'f,
    {
        self.entries.insert(name.into(), VariableBinding::new_provider(provider));
        Ok(self)
    }

    /// Finds a variable binding by name.
    ///
    /// # Parameters
    ///
    /// - `name`: Variable name
    ///
    /// # Returns
    ///
    /// Returns `Some(&VariableBinding)` if found, `None` otherwise
    pub fn find(&self, name: &str) -> Option<&VariableBinding<'f>> {
        self.entries.get(name)
    }

    /// Finds a mutable variable binding by name.
    ///
    /// # Parameters
    ///
    /// - `name`: Variable name
    ///
    /// # Returns
    ///
    /// Returns `Some(&mut VariableBinding)` if found, `None` otherwise
    pub fn find_mut(&mut self, name: &str) -> Option<&mut VariableBinding<'f>> {
        self.entries.get_mut(name)
    }

    /// Returns an iterator over all variable bindings.
    ///
    /// The iterator yields `(name, binding)` pairs for all bound variables.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &VariableBinding<'f>)> {
        self.entries.iter()
            .map(|(name, entry)| (name.as_str(), entry))
    }

    /// Returns a mutable iterator over all variable bindings.
    ///
    /// The iterator yields `(name, binding)` pairs and allows modifying the bindings.
    pub fn entries_mut(&mut self) -> impl Iterator<Item = (&str, &mut VariableBinding<'f>)> {
        self.entries.iter_mut()
            .map(|(name, entry)| (name.as_str(), entry))
    }

    /// Removes a variable binding by name.
    ///
    /// # Parameters
    ///
    /// - `name`: Variable name to remove
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if successfully removed, or an error if the variable doesn't exist
    pub fn remove(&mut self, name: &str) -> Result<(), Error> {
        if self.entries.remove(name).is_none() {
            return Err(Error::not_found(format!("Variable {} not found", name)));
        }
        Ok(())
    }

    /// Clears all variable bindings.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns the number of variable bindings.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

/// Variable binding: value or provider.
///
/// `VariableBinding` represents a runtime variable binding, which can be:
///
/// - **Value binding** (`Value`): Directly stored variable value
/// - **Provider binding** (`Provider`): Dynamically computed variable value through functions
///
/// # Lifetime Parameters
///
/// - `'f`: Lifetime of provider functions
#[derive(Debug, Clone)]
pub enum VariableBinding<'f> {
    /// Directly stored value binding, containing (type, value) tuple
    Value((Type, Value)),
    /// Dynamic provider binding, computing values through functions
    Provider(ValueProvider<'f>),
}

impl<'f> VariableBinding<'f> {
    /// Creates a value binding.
    ///
    /// # Type Parameters
    ///
    /// - `T`: Value type, must implement `IntoValue + TypedValue`
    ///
    /// # Parameters
    ///
    /// - `value`: Value to bind
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::VariableBinding;
    ///
    /// let binding = VariableBinding::new_value("hello");
    /// ```
    pub fn new_value<T>(value: T) -> Self
    where
        T: IntoValue + TypedValue,
    {
        Self::Value((T::value_type(), value.into_value()))
    }

    /// Creates a provider binding.
    ///
    /// # Type Parameters
    ///
    /// - `F`: Provider function type
    /// - `M`: Function marker
    /// - `E`: Error type
    /// - `T`: Return value type
    ///
    /// # Parameters
    ///
    /// - `provider`: Value provider function
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::VariableBinding;
    ///
    /// let binding = VariableBinding::new_provider(|| Ok(42i64));
    /// ```
    pub fn new_provider<F, M, E, T>(provider: F) -> Self
    where
        F: Provider<M, E, T> + Send + Sync + 'f,
        M: 'f,
        E: Into<Error> + Send + Sync + 'static,
        T: IntoValue + TypedValue + 'f,
    {
        let provider = provider.into_erased();
        Self::Provider(Arc::new(provider))
    }

    /// Returns the type of the variable.
    ///
    /// Returns the CEL type of the variable value.
    pub fn value_type(&self) -> Type {
        match self {
            VariableBinding::Value((value_type, _value)) => value_type.clone(),
            VariableBinding::Provider(provider) => provider.value_type(),
        }
    }

    /// Checks if this is a value binding.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is a value binding, `false` otherwise
    pub fn is_value(&self) -> bool {
        matches!(self, VariableBinding::Value(_))
    }

    /// Checks if this is a provider binding.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is a provider binding, `false` otherwise
    pub fn is_provider(&self) -> bool {
        matches!(self, VariableBinding::Provider(_))
    }

    /// Returns a reference to the value (if this is a value binding).
    ///
    /// # Returns
    ///
    /// - For value bindings, returns `Some(&Value)`
    /// - For provider bindings, returns `None`
    pub fn as_value(&self) -> Option<&Value> {
        match self {
            VariableBinding::Value((_, value)) => Some(value),
            VariableBinding::Provider(_) => None,
        }
    }

    /// Returns a reference to the provider (if this is a provider binding).
    ///
    /// # Returns
    ///
    /// - For provider bindings, returns `Some(&ValueProvider)`
    /// - For value bindings, returns `None`
    pub fn as_provider(&self) -> Option<&ValueProvider<'f>> {
        match self {
            VariableBinding::Provider(provider) => Some(provider),
            VariableBinding::Value(_) => None,
        }
    }

    /// Extracts the value (if this is a value binding).
    ///
    /// # Returns
    ///
    /// - For value bindings, returns `Some(Value)` and consumes the binding
    /// - For provider bindings, returns `None`
    pub fn into_value(self) -> Option<Value> {
        match self {
            VariableBinding::Value((_, value)) => Some(value),
            VariableBinding::Provider(_) => None,
        }
    }

    /// Extracts the provider (if this is a provider binding).
    ///
    /// # Returns
    ///
    /// - For provider bindings, returns `Some(ValueProvider)` and consumes the binding
    /// - For value bindings, returns `None`
    pub fn into_provider(self) -> Option<ValueProvider<'f>> {
        match self {
            VariableBinding::Provider(provider) => Some(provider),
            VariableBinding::Value(_) => None,
        }
    }
}
