use std::hash::Hash;
use super::*;

/// Trait for types that have a known CEL type.
///
/// `TypedValue` provides static type information for values that can be converted to CEL types.
/// This trait is used in conjunction with [`IntoValue`] to provide complete type information
/// for value conversions.
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{TypedValue, Type};
///
/// // Implement for a custom type
/// struct CustomId(u64);
///
/// impl TypedValue for CustomId {
///     fn value_type() -> Type {
///         Type::Uint
///     }
/// }
/// ```
pub trait TypedValue {
    /// Returns the CEL type for this value type.
    fn value_type() -> Type;
}

/// Trait for converting values into CEL values.
///
/// `IntoValue` provides conversion from Rust types to CEL [`Value`] instances.
/// This trait is typically implemented alongside [`TypedValue`] to provide complete
/// value conversion functionality.
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{IntoValue, Value};
///
/// // Convert various types to CEL values
/// let string_val = "hello".into_value();
/// let int_val = 42i64.into_value();
/// let bool_val = true.into_value();
///
/// assert_eq!(string_val, Value::String("hello".to_string()));
/// assert_eq!(int_val, Value::Int(42));
/// assert_eq!(bool_val, Value::Bool(true));
/// ```
pub trait IntoValue {
    /// Converts this value into a CEL [`Value`].
    fn into_value(self) -> Value;
}

/// Trait for converting CEL values into Rust types.
///
/// `FromValue` provides conversion from CEL [`Value`] instances to Rust types.
/// This is the inverse operation of [`IntoValue`].
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{FromValue, Value};
///
/// // Convert CEL values back to Rust types
/// let cel_string = Value::String("hello".to_string());
/// let rust_string = String::from_value(cel_string).unwrap();
/// assert_eq!(rust_string, "hello");
///
/// let cel_int = Value::Int(42);
/// let rust_int = i64::from_value(cel_int).unwrap();
/// assert_eq!(rust_int, 42);
/// ```
pub trait FromValue: Sized {
    /// Attempts to convert a CEL [`Value`] into this type.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)`: Conversion successful
    /// - `Err(Value)`: Conversion failed, returns original value
    fn from_value(value: Value) -> Result<Self, Value>;
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
pub trait TypedMapKey {
    /// Returns the CEL map key type for this type.
    fn mapkey_type() -> MapKeyType;
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
/// assert_eq!(string_key, MapKey::String("key".to_string()));
/// assert_eq!(int_key, MapKey::Int(42));
/// assert_eq!(bool_key, MapKey::Bool(true));
/// ```
pub trait IntoMapKey: Eq + Hash + Ord {
    /// Converts this value into a CEL [`MapKey`].
    fn into_mapkey(self) -> MapKey;
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
/// let cel_key = MapKey::String("hello".to_string());
/// let rust_string = String::from_mapkey(cel_key).unwrap();
/// assert_eq!(rust_string, "hello");
///
/// let cel_key = MapKey::Int(42);
/// let rust_int = i64::from_mapkey(cel_key).unwrap();
/// assert_eq!(rust_int, 42);
/// ```
pub trait FromMapKey: Sized + Eq + Hash + Ord {
    /// Attempts to convert a CEL [`MapKey`] into this type.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)`: Conversion successful
    /// - `Err(MapKey)`: Conversion failed, returns original map key
    fn from_mapkey(key: MapKey) -> Result<Self, MapKey>;
}

/// Trait for typed argument tuples.
///
/// `TypedArguments` provides type information for function argument tuples.
/// This trait is automatically implemented for tuples of types that implement
/// [`FromValue`] and [`TypedValue`].
///
/// This trait is sealed and cannot be implemented outside this crate.
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{TypedArguments, Type};
///
/// // Function with typed arguments
/// fn example_args() -> Vec<Type> {
///     <(String, i64, bool)>::arguments_type()
/// }
///
/// let arg_types = example_args();
/// assert_eq!(arg_types, vec![Type::String, Type::Int, Type::Bool]);
/// ```
pub trait TypedArguments: private::Sealed {
    /// Returns the types of all arguments in this tuple.
    fn arguments_type() -> Vec<Type>;
}


macro_rules! impl_typed_arguments {
    ($($ty:ident),*) => {
        impl<$($ty: FromValue + TypedValue),*> TypedArguments for ($($ty,)*) {
            fn arguments_type() -> Vec<Type> {
                vec![$($ty::value_type()),*]
            }
        }
        impl<$($ty: FromValue + TypedValue),*> private::Sealed for ($($ty,)*) {}
    };
}

impl_typed_arguments!();
impl_typed_arguments!(A1);
impl_typed_arguments!(A1, A2);
impl_typed_arguments!(A1, A2, A3);
impl_typed_arguments!(A1, A2, A3, A4);
impl_typed_arguments!(A1, A2, A3, A4, A5);
impl_typed_arguments!(A1, A2, A3, A4, A5, A6);
impl_typed_arguments!(A1, A2, A3, A4, A5, A6, A7);
impl_typed_arguments!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_typed_arguments!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_typed_arguments!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);


mod private {
    pub trait Sealed {}
}
