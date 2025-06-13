use std::collections::HashMap;
use crate::types::Type;
use crate::values::*;
use crate::Error;

/// Compile-time variable registry.
///
/// `VariableRegistry` manages variable declarations and constant definitions during the CEL environment
/// compilation phase. It maintains a mapping from variable names to variable entries, where each entry
/// can be either:
///
/// - A constant value ([`Constant`]): A fixed value known at compile time
/// - A variable declaration ([`VariableDecl`]): A type declaration with value provided at runtime
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
    entries: HashMap<String, ConstantOrDecl>,
}


impl VariableRegistry {
    /// Creates a new empty variable registry.
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    /// Defines a constant value.
    ///
    /// Constants are known at compile time and can be used for optimization and type inference.
    ///
    /// # Parameters
    ///
    /// - `name`: The constant name
    /// - `value`: The constant value, must implement [`Into<Constant>`]
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
    ///     .define_constant("MAX_SIZE", 1024i64)?
    ///     .define_constant("DEFAULT_NAME", "unnamed")?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn define_constant<T>(&mut self, name: impl Into<String>, value: T) -> Result<&mut Self, Error>
    where
        T: Into<Constant>,
    {
        self.entries.insert(name.into(), ConstantOrDecl::new_constant(value.into()));
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
        self.entries.insert(name.into(), ConstantOrDecl::new_decl(T::value_type()));
        Ok(self)
    }

    /// Returns an iterator over all variable entries.
    ///
    /// The iterator yields `(name, entry)` pairs for all registered variables and constants.
    pub fn entries(&self) -> impl Iterator<Item = (&String, &ConstantOrDecl)> {
        self.entries.iter()
    }

    /// Returns a mutable iterator over all variable entries.
    ///
    /// The iterator yields `(name, entry)` pairs and allows modifying the entries.
    pub fn entries_mut(&mut self) -> impl Iterator<Item = (&String, &mut ConstantOrDecl)> {
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
    /// Returns `Some(&ConstantOrDecl)` if found, `None` otherwise
    pub fn find(&self, name: &str) -> Option<&ConstantOrDecl> {
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
    /// Returns `Some(&mut ConstantOrDecl)` if found, `None` otherwise
    pub fn find_mut(&mut self, name: &str) -> Option<&mut ConstantOrDecl> {
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
    /// Returns `Some(ConstantOrDecl)` if the entry was found and removed, `None` otherwise
    pub fn remove(&mut self, name: &str) -> Option<ConstantOrDecl> {
        self.entries.remove(name)
    }

    /// Clears all variable entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns the number of variable entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

/// Variable entry that can be either a constant or a variable declaration.
///
/// `ConstantOrDecl` represents an entry in the compile-time variable registry, which can be:
///
/// - [`Constant`]: A compile-time known constant value
/// - [`VariableDecl`]: A runtime variable type declaration
#[derive(Debug)]
pub enum ConstantOrDecl {
    /// A constant value known at compile time
    Constant(Constant),
    /// A variable declaration containing only type information
    Decl(VariableDecl),
}

impl ConstantOrDecl {
    /// Creates a constant entry.
    ///
    /// # Parameters
    ///
    /// - `value`: The constant value, must implement [`Into<Constant>`]
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::ConstantOrDecl;
    ///
    /// let entry = ConstantOrDecl::new_constant(42i64);
    /// ```
    pub fn new_constant<T>(value: T) -> Self
    where
        T: Into<Constant>,
    {
        Self::Constant(value.into())
    }

    /// Creates a variable declaration entry.
    ///
    /// # Parameters
    ///
    /// - `r#type`: The variable type
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{Type, ConstantOrDecl};
    ///
    /// let entry = ConstantOrDecl::new_decl(Type::String);
    /// ```
    pub fn new_decl(r#type: Type) -> Self {
        Self::Decl(VariableDecl(r#type))
    }

    /// Returns the type of the variable.
    ///
    /// For constants, returns the type of their value; for variable declarations, returns the declared type.
    pub fn r#type(&self) -> Type {
        match self {
            Self::Constant(constant) => constant.value_type(),
            Self::Decl(decl) => decl.value_type(),
        }
    }

    /// Returns the value of the variable if it's a constant.
    ///
    /// # Returns
    ///
    /// - For constant entries, returns `Some(Value)` containing the constant value
    /// - For variable declaration entries, returns `None`
    pub fn value(&self) -> Option<Value> {
        match self {
            Self::Constant(constant) => Some(constant.value()),
            Self::Decl(_) => None,
        }
    }
}

impl VariableEntry for ConstantOrDecl {
    fn value_type(&self) -> Type {
        match self {
            Self::Constant(constant) => constant.value_type(),
            Self::Decl(decl) => decl.value_type(),
        }
    }
}

/// Trait for variable entries providing unified type access.
pub trait VariableEntry {
    /// Returns the type of the variable.
    fn value_type(&self) -> Type;
}

/// Variable type declaration.
///
/// `VariableDecl` represents a runtime variable's type declaration.
/// It doesn't contain an actual value; values are provided by [`Activation`] at runtime.
///
/// [`Activation`]: crate::Activation
#[derive(Debug)]
pub struct VariableDecl(Type);

impl VariableEntry for VariableDecl {
    fn value_type(&self) -> Type {
        self.0.clone()
    }
}

/// CEL constant value.
///
/// `Constant` represents constant values known at compile time, supporting CEL's basic data types.
/// Constants can be used for compile-time optimization and type inference.
///
/// # Supported Types
///
/// - `Null`: Null value
/// - `Bool`: Boolean value
/// - `Int`: 64-bit signed integer
/// - `Uint`: 64-bit unsigned integer
/// - `Double`: 64-bit floating point number
/// - `String`: UTF-8 string
/// - `Bytes`: Byte array
/// - `Duration`: Time duration
/// - `Timestamp`: Timestamp
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::Constant;
///
/// let null_const = Constant::Null;
/// let bool_const = Constant::Bool(true);
/// let int_const = Constant::Int(42);
/// let string_const = Constant::String("hello".to_string());
/// ```
#[derive(Debug, Clone, Default)]
pub enum Constant {
    /// Null constant
    #[default]
    Null,
    /// Boolean constant
    Bool(bool),
    /// Signed integer constant
    Int(i64),
    /// Unsigned integer constant
    Uint(u64),
    /// Floating point constant
    Double(f64),
    /// Byte array constant
    Bytes(Vec<u8>),
    /// String constant
    String(String),
    /// Duration constant
    Duration(chrono::Duration),
    /// Timestamp constant
    Timestamp(chrono::DateTime<chrono::Utc>),
}

impl IntoValue for Constant {
    fn into_value(self) -> Value {
        self.value()
    }
}

impl Constant {
    /// Returns the type of the constant.
    ///
    /// Returns the CEL type corresponding to the constant value.
    pub fn value_type(&self) -> Type {
        match self {
            Self::Null => Type::Null,
            Self::Bool(_) => Type::Bool,
            Self::Int(_) => Type::Int,
            Self::Uint(_) => Type::Uint,
            Self::Double(_) => Type::Double,
            Self::Bytes(_) => Type::Bytes,
            Self::String(_) => Type::String,
            Self::Duration(_) => Type::Duration,
            Self::Timestamp(_) => Type::Timestamp,
        }
    }

    /// Converts the constant to a CEL value.
    ///
    /// Converts the constant to the corresponding [`Value`] type.
    ///
    /// [`Value`]: crate::Value
    pub fn value(&self) -> Value {
        match self {
            Self::Null => Value::Null,
            Self::Bool(value) => Value::Bool(*value),
            Self::Int(value) => Value::Int(*value),
            Self::Uint(value) => Value::Uint(*value),
            Self::Double(value) => Value::Double(*value),
            Self::Bytes(value) => Value::Bytes(value.clone()),
            Self::String(value) => Value::String(value.clone()),
            Self::Duration(value) => Value::Duration(*value),
            Self::Timestamp(value) => Value::Timestamp(*value),
        }
    }
}

impl From<()> for Constant {
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl From<bool> for Constant {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for Constant {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<i32> for Constant {
    fn from(value: i32) -> Self {
        Self::Int(value as i64)
    }
}

impl From<i16> for Constant {
    fn from(value: i16) -> Self {
        Self::Int(value as i64)
    }
}

impl From<u64> for Constant {
    fn from(value: u64) -> Self {
        Self::Uint(value)
    }
}

impl From<u32> for Constant {
    fn from(value: u32) -> Self {
        Self::Uint(value as u64)
    }
}

impl From<u16> for Constant {
    fn from(value: u16) -> Self {
        Self::Uint(value as u64)
    }
}

impl From<f64> for Constant {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}

impl From<f32> for Constant {
    fn from(value: f32) -> Self {
        Self::Double(value as f64)
    }
}

impl From<Vec<u8>> for Constant {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl From<&[u8]> for Constant {
    fn from(value: &[u8]) -> Self {
        Self::Bytes(value.to_vec())
    }
}

impl From<String> for Constant {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Constant {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<chrono::Duration> for Constant {
    fn from(value: chrono::Duration) -> Self {
        Self::Duration(value)
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Constant {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self::Timestamp(value)
    }
}

impl From<std::time::SystemTime> for Constant {
    fn from(value: std::time::SystemTime) -> Self {
        Self::Timestamp(chrono::DateTime::from(value))
    }
}

impl VariableEntry for Constant {
    fn value_type(&self) -> Type {
        Constant::value_type(self)
    }
}
