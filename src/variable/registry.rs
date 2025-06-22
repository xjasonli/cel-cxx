use std::collections::HashMap;
use crate::ValueType;
use crate::values::*;
use crate::Error;

/// Compile-time variable registry.
///
/// `VariableRegistry` manages variable declarations and constant definitions during the CEL environment
/// compilation phase. It maintains a mapping from variable names to variable entries, where each entry
/// can be either:
///
/// - A constant value ([`Constant`]): A fixed value known at compile time
/// - A variable declaration: A type declaration with value provided at runtime
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{Type, VariableRegistry};
///
/// let mut registry = VariableRegistry::new();
///
/// // Define constants
/// registry.define_constant("PI", 3.14159)?;
/// registry.define_constant("APP_NAME", "MyApp")?;
///
/// // Declare variables
/// registry.declare_variable("user_input", Type::String)?;
///
/// assert_eq!(registry.len(), 3);
/// # Ok::<(), cel_cxx::Error>(())
/// ```
#[derive(Debug, Default)]
pub struct VariableRegistry {
    entries: HashMap<String, VariableDeclOrConstant>,
}


impl VariableRegistry {
    /// Creates a new empty variable registry.
    ///
    /// # Returns
    ///
    /// A new empty `VariableRegistry`
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    /// Defines a constant value.
    ///
    /// Constants are values that are known at compile time and don't change during evaluation.
    /// They can be used directly in CEL expressions without requiring runtime bindings.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The constant type, must implement [`IntoConstant`]
    ///
    /// # Parameters
    ///
    /// - `name`: The constant name
    /// - `value`: The constant value
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to support method chaining, or [`Error`] if an error occurs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::VariableRegistry;
    ///
    /// let mut registry = VariableRegistry::new();
    /// registry
    ///     .define_constant("PI", 3.14159)?
    ///     .define_constant("APP_NAME", "MyApp")?
    ///     .define_constant("MAX_USERS", 1000i64)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    ///
    /// [`IntoConstant`]: crate::values::IntoConstant
    pub fn define_constant<T>(&mut self, name: impl Into<String>, value: T) -> Result<&mut Self, Error>
    where
        T: IntoConstant,
    {
        self.entries.insert(name.into(), VariableDeclOrConstant::new_constant(value));
        Ok(self)
    }

    /// Declares a variable with a specific type.
    ///
    /// Variable declarations only specify the type, with actual values provided at runtime
    /// through [`Activation`]. The type is determined by the generic parameter `T` which
    /// must implement [`TypedValue`].
    ///
    /// # Type Parameters
    ///
    /// - `T`: The variable type, must implement [`TypedValue`]
    ///
    /// # Parameters
    ///
    /// - `name`: The variable name
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to support method chaining, or [`Error`] if an error occurs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::VariableRegistry;
    ///
    /// let mut registry = VariableRegistry::new();
    /// registry
    ///     .declare_variable::<String>("user_name")?
    ///     .declare_variable::<i64>("user_id")?
    ///     .declare_variable::<bool>("is_admin")?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    ///
    /// [`Activation`]: crate::Activation
    /// [`TypedValue`]: crate::TypedValue
    pub fn declare_variable<T>(&mut self, name: impl Into<String>) -> Result<&mut Self, Error>
    where
        T: TypedValue,
    {
        self.entries.insert(name.into(), VariableDeclOrConstant::new(T::value_type()));
        Ok(self)
    }

    /// Returns an iterator over all variable entries.
    ///
    /// The iterator yields `(name, entry)` pairs for all registered variables and constants.
    ///
    /// # Returns
    ///
    /// Iterator yielding `(&String, &VariableDeclOrConstant)` pairs
    pub fn entries(&self) -> impl Iterator<Item = (&String, &VariableDeclOrConstant)> {
        self.entries.iter()
    }

    /// Returns a mutable iterator over all variable entries.
    ///
    /// The iterator yields `(name, entry)` pairs and allows modifying the entries.
    ///
    /// # Returns
    ///
    /// Iterator yielding `(&String, &mut VariableDeclOrConstant)` pairs
    pub fn entries_mut(&mut self) -> impl Iterator<Item = (&String, &mut VariableDeclOrConstant)> {
        self.entries.iter_mut()
    }

    /// Finds a variable entry by name.
    ///
    /// # Parameters
    ///
    /// - `name`: The variable name to search for
    ///
    /// # Returns
    ///
    /// Returns `Some(&VariableDeclOrConstant)` if found, `None` otherwise
    pub fn find(&self, name: &str) -> Option<&VariableDeclOrConstant> {
        self.entries.get(name)
    }

    /// Finds a mutable variable entry by name.
    ///
    /// # Parameters
    ///
    /// - `name`: The variable name to search for
    ///
    /// # Returns
    ///
    /// Returns `Some(&mut VariableDeclOrConstant)` if found, `None` otherwise
    pub fn find_mut(&mut self, name: &str) -> Option<&mut VariableDeclOrConstant> {
        self.entries.get_mut(name)
    }

    /// Removes a variable entry by name.
    ///
    /// # Parameters
    ///
    /// - `name`: The variable name to remove
    ///
    /// # Returns
    ///
    /// Returns `Some(VariableDeclOrConstant)` if the entry was found and removed, `None` otherwise
    pub fn remove(&mut self, name: &str) -> Option<VariableDeclOrConstant> {
        self.entries.remove(name)
    }

    /// Clears all variable entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns the number of variable entries.
    ///
    /// # Returns
    ///
    /// Number of registered variables and constants
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the registry is empty.
    ///
    /// # Returns
    ///
    /// `true` if no variables or constants are registered, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Union type representing either a variable declaration or constant definition.
///
/// `VariableDeclOrConstant` can hold either:
/// - A constant value ([`Constant`]) that is known at compile time
/// - A variable declaration ([`ValueType`]) that specifies the type for runtime binding
///
/// This allows the registry to handle both compile-time constants and runtime variables
/// in a unified way.
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::variable::VariableDeclOrConstant;
/// use cel_cxx::types::ValueType;
///
/// // Create from constant
/// let constant_entry = VariableDeclOrConstant::new_constant(42i64);
/// assert!(constant_entry.is_constant());
///
/// // Create from declaration
/// let decl_entry = VariableDeclOrConstant::new_decl(ValueType::String);
/// assert!(decl_entry.is_decl());
/// ```
///
/// [`Constant`]: crate::values::Constant
#[derive(Debug)]
pub struct VariableDeclOrConstant {
    r#type: ValueType,
    constant: Option<Constant>,
}

impl VariableDeclOrConstant {
    /// Creates a new constant entry.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The constant type, must implement [`IntoConstant`]
    ///
    /// # Parameters
    ///
    /// - `value`: The constant value
    ///
    /// # Returns
    ///
    /// New `VariableDeclOrConstant` containing the constant value
    ///
    /// [`IntoConstant`]: crate::values::IntoConstant
    pub fn new_constant<T>(value: T) -> Self
    where
        T: IntoConstant,
    {
        Self { r#type: T::value_type(), constant: Some(value.into_constant()) }
    }

    /// Creates a new variable declaration entry.
    ///
    /// # Parameters
    ///
    /// - `r#type`: The variable type
    ///
    /// # Returns
    ///
    /// New `VariableDeclOrConstant` containing the type declaration
    pub fn new(r#type: ValueType) -> Self {
        Self { r#type, constant: None }
    }

    /// Returns whether this entry is a constant.
    ///
    /// # Returns
    ///
    /// `true` if this entry contains a constant value, `false` if it's a declaration
    pub fn is_constant(&self) -> bool {
        self.constant.is_some()
    }

    /// Gets the type of this entry.
    ///
    /// For constants, this is the type of the constant value.
    /// For declarations, this is the declared variable type.
    ///
    /// # Returns
    ///
    /// Reference to the [`ValueType`] of this entry
    pub fn decl(&self) -> &ValueType {
        &self.r#type
    }

    /// Gets the constant value, if this entry is a constant.
    ///
    /// # Returns
    ///
    /// `Some(&Constant)` if this is a constant entry, `None` if it's a declaration
    pub fn constant(&self) -> Option<&Constant> {
        self.constant.as_ref()
    }
}
