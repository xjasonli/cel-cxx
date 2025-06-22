use std::hash::Hash;
use super::*;
use crate::IntoError;

/// Trait for types that have a known CEL type.
///
/// This trait provides type information for values that can be used in CEL expressions.
/// It is used in conjunction with [`IntoValue`] and [`FromValue`] to provide complete type
/// information for value conversions.
///
/// # Implementation
///
/// **Important**: This trait should generally not be implemented manually. For most use cases:
/// - **Built-in types**: Already have implementations (primitives, collections, etc.)
/// - **Custom opaque types**: Use the `#[derive(Opaque)]` macro which automatically implements this trait
/// - **Special cases**: Only implement manually if you have very specific requirements
///
/// # Examples
///
/// ## Using Built-in Types
///
/// ```rust
/// use cel_cxx::{TypedValue, ValueType};
///
/// // Built-in types already implement TypedValue
/// assert_eq!(i64::value_type(), ValueType::Int);
/// assert_eq!(String::value_type(), ValueType::String);
/// assert_eq!(bool::value_type(), ValueType::Bool);
/// ```
///
/// ## Using Derive Macro for Custom Types
///
/// ```rust,no_run
/// use cel_cxx::Opaque;
///
/// #[derive(Opaque, Debug, Clone, PartialEq)]
/// struct CustomId(u64);
///
/// impl std::fmt::Display for CustomId {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "CustomId({})", self.0)
///     }
/// }
///
/// // TypedValue is automatically implemented by the derive macro
/// ```
pub trait TypedValue {
    /// Returns the CEL type for this value type.
    ///
    /// This method provides the [`ValueType`] that corresponds to this Rust type
    /// in the CEL type system. It is used for type checking, error messages,
    /// and runtime type validation.
    ///
    /// # Returns
    ///
    /// The [`ValueType`] that represents this type in CEL expressions.
    fn value_type() -> ValueType;
}

impl<T: TypedValue + ?Sized> TypedValue for &T {
    fn value_type() -> ValueType {
        T::value_type()
    }
}

impl<T: TypedValue + ?Sized> TypedValue for &mut T {
    fn value_type() -> ValueType {
        T::value_type()
    }
}

/// Trait for converting values into CEL values.
///
/// This trait enables automatic conversion from Rust types to CEL [`Value`] instances.
/// It is typically used alongside [`TypedValue`] to provide complete value conversion
/// functionality.
///
/// # Implementation
///
/// **Important**: This trait should generally not be implemented manually. For most use cases:
/// - **Built-in types**: Already have implementations
/// - **Custom opaque types**: Use the `#[derive(Opaque)]` macro
/// - **Collection types**: Automatically implemented for compatible element types
///
/// # Examples
///
/// ## Using Built-in Conversions
///
/// ```rust
/// use cel_cxx::{IntoValue, Value};
/// 
/// // Built-in types can be converted directly
/// let string_val = "hello".into_value();
/// let int_val = 42i64.into_value();
/// let bool_val = true.into_value();
/// 
/// assert_eq!(string_val, Value::String("hello".to_string().into()));
/// assert_eq!(int_val, Value::Int(42));
/// assert_eq!(bool_val, Value::Bool(true));
/// ```
///
/// ## Collection Conversions
///
/// ```rust
/// use cel_cxx::{IntoValue, Value};
///
/// let list = vec![1i64, 2i64, 3i64].into_value();
/// let map = std::collections::HashMap::from([
///     ("key".to_string(), "value".to_string())
/// ]).into_value();
/// ```
pub trait IntoValue: Sized {
    /// Converts this value into a CEL [`Value`].
    ///
    /// This method performs the conversion from a Rust type to its corresponding
    /// CEL value representation. The conversion should be lossless where possible.
    ///
    /// # Returns
    ///
    /// A [`Value`] that represents this Rust value in the CEL type system.
    fn into_value(self) -> Value;
}

impl<T: IntoValue + Clone + ?Sized> IntoValue for &T {
    fn into_value(self) -> Value {
        T::into_value(Clone::clone(self))
    }
}

impl<T: IntoValue + Clone + ?Sized> IntoValue for &mut T {
    fn into_value(self) -> Value {
        T::into_value(Clone::clone(self))
    }
}

/// Trait for converting CEL values into Rust types.
///
/// This trait enables conversion from CEL [`Value`] instances back to Rust types,
/// with support for both owned and borrowed data through Generic Associated Types (GATs).
/// It is the inverse operation of [`IntoValue`].
///
/// # Generic Associated Types (GATs)
///
/// The `Output<'a>` associated type allows the trait to return either owned or borrowed
/// data depending on the source value:
/// - For `String`: `Output<'a> = String` (always owned)
/// - For `&str`: `Output<'a> = &'a str` (borrowed from the source Value)
/// - For primitives: `Output<'a> = Self` (copied)
///
/// # Implementation
///
/// **Important**: This trait should generally not be implemented manually. For most use cases:
/// - **Built-in types**: Already have implementations with proper lifetime handling
/// - **Custom opaque types**: Use the `#[derive(Opaque)]` macro
/// - **Manual implementation**: Only for very specific requirements and advanced use cases
///
/// # Examples
///
/// ## Using Built-in Conversions
///
/// ```rust
/// use cel_cxx::{FromValue, Value};
/// 
/// let cel_string = Value::String("hello".to_string().into());
/// let rust_string = String::from_value(&cel_string).unwrap();
/// assert_eq!(rust_string, "hello");
///
/// let cel_int = Value::Int(42);
/// let rust_int = i64::from_value(&cel_int).unwrap();
/// assert_eq!(rust_int, 42);
/// ```
///
/// ## Zero-Copy String Conversion
///
/// ```rust
/// use cel_cxx::{FromValue, Value};
/// 
/// let cel_string = Value::String("hello world".to_string().into());
/// 
/// // Convert to borrowed string slice (zero-copy)
/// let borrowed_str = <&str>::from_value(&cel_string).unwrap();
/// assert_eq!(borrowed_str, "hello world");
/// ```
pub trait FromValue: Sized {
    /// The output type for the `from_value` method.
    ///
    /// This type is parameterized by the lifetime of the value being converted.
    /// The lifetime is used to indicate whether the value is borrowed or owned.
    type Output<'a>;

    /// Attempts to convert a CEL [`Value`] into this type.
    ///
    /// This method performs type checking and conversion from a CEL value to the
    /// target Rust type. It returns an error if the conversion is not possible.
    ///
    /// # Arguments
    ///
    /// * `value` - The CEL value to convert
    ///
    /// # Returns
    ///
    /// A [`Result`] containing either the converted value or a [`FromValueError`]
    /// if the conversion failed.
    ///
    /// # Errors
    ///
    /// Returns [`FromValueError`] if:
    /// - The value type doesn't match the expected type
    /// - The value format is invalid for the target type
    /// - Type constraints are not satisfied
    fn from_value<'a>(value: &'a Value) -> Result<Self::Output<'a>, FromValueError>;
}

/// Error type for failed value conversions.
///
/// This error is returned when attempting to convert a CEL [`Value`] to a Rust type
/// using the [`FromValue`] trait, but the conversion fails due to type mismatch.
///
/// # Error Information
///
/// The error contains:
/// - **Source value**: The original CEL value that could not be converted
/// - **Target type**: String description of the intended conversion target
/// - **Context**: Additional information about why the conversion failed
///
/// # Common Causes
///
/// - **Type mismatch**: Trying to convert `Value::String` to `i64`
/// - **Invalid format**: Trying to convert malformed data
/// - **Constraint violation**: Value doesn't meet type constraints
///
/// # Examples
///
/// ```rust
/// use cel_cxx::{Value, FromValue, FromValueError};
///
/// let string_val = Value::String("not_a_number".to_string().into());
/// let result = i64::from_value(&string_val);
///
/// match result {
///     Ok(num) => println!("Converted: {}", num),
///     Err(error) => {
///         println!("Cannot convert value: {}", error);
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FromValueError {
    value: Value,
    to_type: String,
}

impl FromValueError {
    /// Create a new conversion error with a custom target type description.
    ///
    /// # Arguments
    ///
    /// * `value` - The CEL value that could not be converted
    /// * `to` - Description of the target type (will be converted to string)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::{Value, FromValueError};
    ///
    /// let error = FromValueError::new(Value::String("text".to_string().into()), "integer");
    /// // Error created successfully with custom type description
    /// ```
    pub fn new(value: Value, to: impl ToString) -> Self {
        Self { value, to_type: to.to_string() }
    }

    /// Create a new conversion error with automatic type name detection.
    ///
    /// This method uses the [`TypedValue`] trait to automatically determine
    /// the CEL type information of the target type, providing more consistent error messages.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target type that implements [`TypedValue`]
    ///
    /// # Arguments
    ///
    /// * `value` - The CEL value that could not be converted
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::{Value, FromValueError};
    ///
    /// let error = FromValueError::new_typed::<i64>(Value::String("text".to_string().into()));
    /// // error contains type information for the target type
    /// ```
    pub fn new_typed<T: TypedValue>(value: Value) -> Self {
        Self::new(value, T::value_type())
    }
}

impl std::fmt::Display for FromValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot convert value {} to type {}", self.value, self.to_type)
    }
}

/// Trait for types that can be used as map keys and have a known CEL map key type.
///
/// `TypedMapKey` provides static type information for map key types.
/// This trait is used alongside [`IntoMapKey`] for complete map key functionality.
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{TypedMapKey, MapKeyType};
///
/// // Built-in types implement this automatically
/// assert_eq!(String::mapkey_type(), MapKeyType::String);
/// assert_eq!(i64::mapkey_type(), MapKeyType::Int);
/// ```
pub trait TypedMapKey: TypedValue {
    /// Returns the CEL map key type for this type.
    fn mapkey_type() -> MapKeyType {
        MapKeyType::try_from(Self::value_type())
            .expect("invalid map key type")
    }
}

impl<T: TypedMapKey + ?Sized> TypedMapKey for &T {
    fn mapkey_type() -> MapKeyType {
        T::mapkey_type()
    }
}

impl<T: TypedMapKey + ?Sized> TypedMapKey for &mut T {
    fn mapkey_type() -> MapKeyType {
        T::mapkey_type()
    }
}

/// Trait for converting values into CEL map keys.
///
/// `IntoMapKey` provides conversion from Rust types to CEL [`MapKey`] instances.
/// Only types that are valid CEL map key types can implement this trait.
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{IntoMapKey, MapKey};
///
/// // Convert various types to map keys
/// let string_key = "key".into_mapkey();
/// let int_key = 42i64.into_mapkey();
/// let bool_key = true.into_mapkey();
///
/// assert_eq!(string_key, MapKey::String("key".to_string().into()));
/// assert_eq!(int_key, MapKey::Int(42));
/// assert_eq!(bool_key, MapKey::Bool(true));
/// ```
pub trait IntoMapKey: IntoValue + Eq + Hash + Ord {
    /// Converts this value into a CEL [`MapKey`].
    fn into_mapkey(self) -> MapKey {
        <Self as IntoValue>::into_value(self)
            .try_into()
            .expect("invalid map key type")
    }
}

impl<T: IntoMapKey + Clone + ?Sized> IntoMapKey for &T {
    fn into_mapkey(self) -> MapKey {
        T::into_mapkey(Clone::clone(self))
    }
}

impl<T: IntoMapKey + Clone + ?Sized> IntoMapKey for &mut T {
    fn into_mapkey(self) -> MapKey {
        T::into_mapkey(Clone::clone(self))
    }
}

/// Trait for converting CEL map keys into Rust types.
///
/// `FromMapKey` provides conversion from CEL [`MapKey`] instances to Rust types.
/// This is the inverse operation of [`IntoMapKey`].
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{FromMapKey, MapKey};
///
/// // Convert map keys back to Rust types
/// let cel_key = MapKey::String("hello".to_string().into());
/// let rust_string = String::from_mapkey(&cel_key).unwrap();
/// assert_eq!(rust_string, "hello");
///
/// let cel_key = MapKey::Int(42);
/// let rust_int = i64::from_mapkey(&cel_key).unwrap();
/// assert_eq!(rust_int, 42);
/// ```
pub trait FromMapKey: FromValue + Eq + Hash + Ord
where
    for<'a> Self::Output<'a>: Eq + Hash + Ord,
{
    /// Attempts to convert a CEL [`MapKey`] into this type.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)`: Conversion successful
    /// - `Err(MapKey)`: Conversion failed, returns original map key
    fn from_mapkey<'a>(key: &'a MapKey) -> Result<Self::Output<'a>, FromMapKeyError>;
}

/// Error type for failed map key conversions.
/// 
/// This error is returned when attempting to convert a CEL [`MapKey`] to a Rust type
/// using the [`FromMapKey`] trait, but the conversion fails due to type mismatch.
///
/// # Error Information
///
/// The error contains:
/// - **Source key**: The original CEL map key that could not be converted
/// - **Target type**: The intended map key type for the conversion
///
/// # Map Key Constraints
///
/// Only certain types can be used as CEL map keys:
/// - `bool` - Boolean keys
/// - `i64` - Signed integer keys  
/// - `u64` - Unsigned integer keys
/// - `String` - String keys
///
/// # Common Causes
///
/// - **Type mismatch**: Trying to convert `MapKey::String` to `i64`
/// - **Invalid key type**: Attempting conversion to unsupported key type
/// - **Format issues**: Key value doesn't meet target type constraints
///
/// # Examples
///
/// ```rust
/// use cel_cxx::{MapKey, FromMapKey, FromMapKeyError};
///
/// let string_key = MapKey::String("not_a_number".to_string().into());
/// let result = i64::from_mapkey(&string_key);
///
/// match result {
///     Ok(num) => println!("Converted key: {}", num),
///     Err(error) => {
///         println!("Cannot convert key: {}", error);
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FromMapKeyError {
    key: MapKey,
    to_type: MapKeyType,
}

impl FromMapKeyError {
    /// Create a new map key conversion error.
    ///
    /// # Arguments
    ///
    /// * `key` - The CEL map key that could not be converted
    /// * `to` - The target map key type for the conversion
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::{MapKey, MapKeyType, FromMapKeyError};
    ///
    /// let error = FromMapKeyError::new(
    ///     MapKey::String("text".to_string().into()),
    ///     MapKeyType::Int
    /// );
    /// // Error created successfully
    /// ```
    pub fn new(key: MapKey, to: MapKeyType) -> Self {
        Self { key, to_type: to }
    }

    /// Create a new map key conversion error with automatic type detection.
    ///
    /// This method uses the [`TypedMapKey`] trait to automatically determine
    /// the CEL type information of the target type, providing more consistent error messages.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target map key type that implements [`TypedMapKey`]
    ///
    /// # Arguments
    ///
    /// * `key` - The CEL map key that could not be converted
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::{MapKey, FromMapKeyError};
    ///
    /// let error = FromMapKeyError::new_typed::<i64>(MapKey::String("text".to_string().into()));
    /// // error contains type information for the target type
    /// ```
    pub fn new_typed<T: TypedMapKey>(key: MapKey) -> Self {
        Self::new(key, T::mapkey_type())
    }
}

impl std::fmt::Display for FromMapKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot convert map key {} to type {}", self.key, self.to_type)
    }
}

impl From<FromMapKeyError> for FromValueError {
    fn from(error: FromMapKeyError) -> Self {
        FromValueError::new(error.key.into(), error.to_type)
    }
}

/// Trait for types that can be converted into compile-time CEL constants.
///
/// This trait extends [`IntoValue`] and [`TypedValue`] to provide conversion
/// to [`Constant`] values, which can be used for compile-time optimization
/// and type inference in CEL expressions.
///
/// # Compile-Time vs Runtime Values
///
/// - **Constants**: Known at compile-time, can be folded and optimized
/// - **Values**: Computed at runtime, require full evaluation
///
/// Constants enable the CEL compiler to:
/// - Perform constant folding optimizations
/// - Detect type errors earlier
/// - Generate more efficient evaluation code
/// - Support constant expression evaluation
///
/// # Automatic Implementation
///
/// This trait is automatically implemented for all types that implement
/// both [`IntoValue`] and [`TypedValue`], providing a default conversion
/// from value to constant.
///
/// # Examples
///
/// ## Basic Constant Creation
///
/// ```rust
/// use cel_cxx::{IntoConstant, Constant};
///
/// // Primitive constants
/// let null_const = ().into_constant();          // Constant::Null
/// let bool_const = true.into_constant();        // Constant::Bool(true)
/// let int_const = 42i64.into_constant();        // Constant::Int(42)
/// let string_const = "hello".into_constant();   // Constant::String("hello".to_string())
/// ```
///
/// ## Usage in Environment Building
///
/// ```rust,no_run
/// use cel_cxx::{Env, IntoConstant};
///
/// let env = Env::builder()
///     .define_constant("PI", 3.14159)?
///     .define_constant("APP_NAME", "MyApp")?
///     .define_constant("MAX_RETRIES", 5i64)?
///     .build()?;
///
/// // These constants can be used in expressions:
/// // "PI * radius * radius"
/// // "APP_NAME + ' v1.0'"  
/// // "retries < MAX_RETRIES"
/// ```
///
/// ## Custom Type Constants
///
/// ```rust,no_run
/// use cel_cxx::Opaque;
///
/// #[derive(Opaque, Debug, Clone, PartialEq)]
/// struct Version { major: u32, minor: u32 }
///
/// impl std::fmt::Display for Version {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{}.{}", self.major, self.minor)
///     }
/// }
///
/// // IntoConstant is automatically implemented by the derive macro
/// let version = Version { major: 1, minor: 0 };
/// let version_const = version.into_constant();
/// ```
pub trait IntoConstant: IntoValue + TypedValue {
    /// Convert this value into a compile-time CEL constant.
    ///
    /// This method creates a [`Constant`] from the value, which can be used
    /// for compile-time optimizations and constant expression evaluation.
    ///
    /// # Returns
    ///
    /// A [`Constant`] representing this value that can be used at compile-time.
    ///
    /// # Implementation Note
    ///
    /// The default implementation converts the value using [`IntoValue::into_value`]
    /// and then wraps it as a constant. Custom implementations can provide
    /// more efficient constant creation if needed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cel_cxx::{IntoConstant, Constant};
    ///
    /// let const_val = 42i64.into_constant();
    /// let const_str = "hello".into_constant();
    /// // Constants created successfully
    /// ```
    fn into_constant(self) -> Constant {
        <Self as IntoValue>::into_value(self)
            .try_into()
            .expect("invalid constant type")
    }
}

impl<T: IntoConstant + Clone + ?Sized> IntoConstant for &T {
    fn into_constant(self) -> Constant {
        T::into_constant(Clone::clone(self))
    }
}

impl<T: IntoConstant + Clone + ?Sized> IntoConstant for &mut T {
    fn into_constant(self) -> Constant {
        T::into_constant(Clone::clone(self))
    }
}

/// Trait for converting return values to CEL results.
///
/// This trait provides automatic conversion for function return types,
/// supporting both direct values and `Result` types for error handling.
///
/// # Note
///
/// This trait is sealed and cannot be implemented outside this crate.
///
/// # Implementations
///
/// - `T` where `T: IntoValue + TypedValue` - Direct value conversion
/// - `Result<T, E>` where `T: IntoValue + TypedValue, E: IntoError` - Error handling
pub trait IntoResult: private::Sealed {
    /// Convert the value into a CEL result.
    fn into_result(self) -> Result<Value, Error>;
    
    /// Get the CEL type of the resulting value.
    fn value_type() -> ValueType;
}

impl<T: IntoValue + TypedValue, E: IntoError> IntoResult for Result<T, E> {
    fn into_result(self) -> Result<Value, Error> {
        self.map(|v| v.into_value())
            .map_err(|e| e.into_error())
    }

    fn value_type() -> ValueType {
        <T as TypedValue>::value_type()
    }
}

impl<T: IntoValue + TypedValue> IntoResult for T {
    fn into_result(self) -> Result<Value, Error> {
        Ok(self.into_value())
    }

    fn value_type() -> ValueType {
        <T as TypedValue>::value_type()
    }
}

// Sealed implementations for IntoResult
impl<T: IntoValue + TypedValue, E: IntoError> private::Sealed for Result<T, E> {}
impl<T: IntoValue + TypedValue> private::Sealed for T {}


mod private {
    pub trait Sealed {}
}
