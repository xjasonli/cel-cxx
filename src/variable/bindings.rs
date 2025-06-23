use std::collections::HashMap;
use crate::function::*;
use crate::ValueType;
use crate::values::*;
use crate::marker::*;
use crate::Error;

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
///     std::time::SystemTime::now()
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
        self.entries.insert(name.into(), VariableBinding::from_value(value));
        Ok(self)
    }

    /// Binds a variable to a value provider.
    ///
    /// Binds a variable name to a provider function that computes the value dynamically.
    /// This enables lazy evaluation and allows variables to have values computed at runtime.
    ///
    /// # Type Parameters
    ///
    /// - `F`: Provider function type, must implement `IntoFunction`
    /// - `Fm`: Function marker type (sync/async)
    ///
    /// # Parameters
    ///
    /// - `name`: Variable name
    /// - `provider`: Provider function that computes the variable value
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
    /// bindings.bind_provider("current_time", || -> i64 {
    ///     std::time::SystemTime::now()
    ///         .duration_since(std::time::UNIX_EPOCH)
    ///         .unwrap()
    ///         .as_secs() as i64
    /// })?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn bind_provider<F, Fm>(
        &mut self, name: impl Into<String>, provider: F
    ) -> Result<&mut Self, Error>
    where
        F: IntoFunction<'f, Fm>,
        Fm: FnMarker,
    {
        self.entries.insert(name.into(), VariableBinding::from_provider(provider));
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
    ///
    /// # Returns
    ///
    /// Iterator yielding `(&str, &VariableBinding)` pairs
    pub fn entries(&self) -> impl Iterator<Item = (&str, &VariableBinding<'f>)> {
        self.entries.iter()
            .map(|(name, entry)| (name.as_str(), entry))
    }

    /// Returns a mutable iterator over all variable bindings.
    ///
    /// The iterator yields `(name, binding)` pairs and allows modifying the bindings.
    ///
    /// # Returns
    ///
    /// Iterator yielding `(&str, &mut VariableBinding)` pairs
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
    ///
    /// # Returns
    ///
    /// Number of bound variables
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the bindings are empty.
    ///
    /// # Returns
    ///
    /// `true` if no variables are bound, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
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
    Value((ValueType, Value)),
    /// Dynamic provider binding, computing values through functions
    Provider(Function<'f>),
}

impl<'f> VariableBinding<'f> {
    /// Creates a variable binding from a value.
    ///
    /// # Type Parameters
    ///
    /// - `T`: Value type, must implement `IntoValue + TypedValue`
    ///
    /// # Parameters
    ///
    /// - `value`: The value to bind
    ///
    /// # Returns
    ///
    /// New `VariableBinding::Value` containing the value and its type
    pub fn from_value<T: IntoValue + TypedValue>(value: T) -> Self {
        Self::Value((T::value_type(), value.into_value()))
    }

    /// Creates a variable binding from a provider function.
    ///
    /// # Type Parameters
    ///
    /// - `F`: Provider function type, must implement `IntoFunction`
    /// - `Fm`: Function marker type (sync/async)
    ///
    /// # Parameters
    ///
    /// - `provider`: The provider function
    ///
    /// # Returns
    ///
    /// New `VariableBinding::Provider` containing the provider function
    pub fn from_provider<F, Fm>(provider: F) -> Self
    where
        F: IntoFunction<'f, Fm>,
        Fm: FnMarker,
    {
        Self::Provider(provider.into_function())
    }

    /// Returns the value type of this binding.
    ///
    /// For value bindings, returns the stored type. For provider bindings,
    /// returns the return type of the provider function.
    ///
    /// # Returns
    ///
    /// The [`ValueType`] of this binding
    pub fn value_type(&self) -> ValueType {
        match self {
            Self::Value((ty, _)) => ty.clone(),
            Self::Provider(f) => f.function_type().result().clone(),
        }
    }

    /// Returns whether this is a value binding.
    ///
    /// # Returns
    ///
    /// `true` if this is a `Value` binding, `false` if it's a `Provider` binding
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    /// Returns whether this is a provider binding.
    ///
    /// # Returns
    ///
    /// `true` if this is a `Provider` binding, `false` if it's a `Value` binding
    pub fn is_provider(&self) -> bool {
        matches!(self, Self::Provider(_))
    }

    /// Returns the value if this is a value binding.
    ///
    /// # Returns
    ///
    /// `Some(&Value)` if this is a value binding, `None` if it's a provider binding
    pub fn as_value(&self) -> Option<&Value> {
        match self {
            Self::Value((_, value)) => Some(value),
            Self::Provider(_) => None,
        }
    }

    /// Returns the provider function if this is a provider binding.
    ///
    /// # Returns
    ///
    /// `Some(&Function)` if this is a provider binding, `None` if it's a value binding
    pub fn as_provider(&self) -> Option<&Function<'f>> {
        match self {
            Self::Value(_) => None,
            Self::Provider(f) => Some(f),
        }
    }

    /// Converts this binding into a value if it's a value binding.
    ///
    /// # Returns
    ///
    /// `Some(Value)` if this is a value binding, `None` if it's a provider binding
    pub fn into_value(self) -> Option<Value> {
        match self {
            Self::Value((_, value)) => Some(value),
            Self::Provider(_) => None,
        }
    }

    /// Converts this binding into a provider function if it's a provider binding.
    ///
    /// # Returns
    ///
    /// `Some(Function)` if this is a provider binding, `None` if it's a value binding
    pub fn into_provider(self) -> Option<Function<'f>> {
        match self {
            Self::Value(_) => None,
            Self::Provider(f) => Some(f),
        }
    }
}
