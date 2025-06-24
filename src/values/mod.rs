//! CEL Value Types and Operations
//!
//! This module provides the core value types used in CEL expressions, along with
//! comprehensive conversion and manipulation utilities. It forms the foundation
//! of the CEL type system, supporting both primitive and composite data types.
//!
//! # Value Type Hierarchy
//!
//! The CEL value system is built around several core types:
//!
//! ## Primitive Types
//! - **Null**: Represents absent values (`null`)
//! - **Bool**: Boolean values (`true`, `false`)
//! - **Int**: 64-bit signed integers (`i64`)
//! - **Uint**: 64-bit unsigned integers (`u64`) 
//! - **Double**: IEEE 754 double-precision floating point (`f64`)
//! - **String**: UTF-8 encoded strings
//! - **Bytes**: Arbitrary byte sequences
//!
//! ## Time Types
//! - **Duration**: Protocol Buffers Duration type (represents time spans)
//! - **Timestamp**: Protocol Buffers Timestamp type (represents points in time)
//!
//! ## Composite Types
//! - **List**: Ordered collections of values (`Vec<Value>`)
//! - **Map**: Key-value mappings (`HashMap<MapKey, Value>`)
//! - **Struct**: Protocol Buffers message types (not yet implemented)
//! - **Optional**: Wrapper for optional values
//!
//! ## Special Types
//! - **Type**: Meta-type representing CEL types themselves
//! - **Error**: Error values from failed operations
//! - **Unknown**: Partially evaluated expressions (not yet implemented)
//! - **Opaque**: Custom user-defined types
//!
//! # Type Conversion System
//!
//! The module provides a comprehensive type conversion system built on Generic
//! Associated Types (GATs) for safe, zero-cost conversions between Rust and CEL types.
//!
//! ## Core Conversion Traits
//!
//! - [`TypedValue`]: Types that have a known CEL type
//! - [`IntoValue`]: Convert Rust types to CEL values
//! - [`FromValue`]: Convert CEL values to Rust types (with GATs)
//! - [`IntoConstant`]: Convert to compile-time constants
//!
//! ## Map Key Conversion
//!
//! - [`TypedMapKey`]: Types that can be used as map keys
//! - [`IntoMapKey`]: Convert to CEL map keys
//! - [`FromMapKey`]: Convert from CEL map keys
//!
//! # Memory Management and Lifetimes
//!
//! The value system is designed for efficient memory usage:
//! - **Zero-copy conversions** where possible (`&str` from `String` values)
//! - **Controlled lifetime erasure** for safe reference handling
//! - **Reference counting** for shared data structures
//! - **Clone-on-write** semantics for expensive operations
//!
//! # Examples
//!
//! ## Basic Value Creation and Conversion
//!
//! ```rust
//! use cel_cxx::{Value, IntoValue, FromValue};
//!
//! // Create values from Rust types
//! let null_val = Value::Null;
//! let bool_val = true.into_value();
//! let int_val = 42i64.into_value();
//! let string_val = "hello".into_value();
//!
//! // Convert back to Rust types
//! let rust_bool: bool = bool_val.try_into()?;
//! let rust_int: i64 = int_val.try_into()?;
//! let rust_string: String = string_val.try_into()?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! ## Working with Collections
//!
//! ```rust
//! use cel_cxx::{Value, MapKey};
//! use std::collections::HashMap;
//!
//! // Create a list
//! let list = Value::List(vec![
//!     Value::Int(1),
//!     Value::Int(2),
//!     Value::Int(3),
//! ]);
//!
//! // Create a map
//! let mut map = HashMap::new();
//! map.insert(MapKey::String("name".to_string().into()), Value::String("Alice".to_string().into()));
//! map.insert(MapKey::String("age".to_string().into()), Value::Int(30));
//! let map_val = Value::Map(map);
//! ```
//!
//! ## Reference Conversions with Lifetimes
//!
//! ```rust
//! use cel_cxx::{Value, FromValue};
//!
//! let string_val = Value::String("hello world".to_string().into());
//!
//! // Convert to borrowed string slice (zero-copy)
//! let borrowed_str = <&str>::from_value(&string_val)?;
//! assert_eq!(borrowed_str, "hello world");
//!
//! // The original value owns the data
//! drop(string_val); // borrowed_str is no longer valid after this
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! ## Custom Type Integration
//!
//! For custom opaque types, use the derive macro instead of manual implementation:
//!
//! ```rust
//! use cel_cxx::{Opaque, IntoValue, FromValue};
//!
//! #[derive(Opaque, Debug, Clone, PartialEq)]
//! struct UserId(u64);
//! 
//! impl std::fmt::Display for UserId {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "User({})", self.0)
//!     }
//! }
//!
//! // All necessary traits (TypedValue, IntoValue, FromValue) are automatically implemented
//!
//! // Usage
//! let user_id = UserId(12345);
//! let value = user_id.into_value();
//! let converted_back = UserId::from_value(&value)?;
//! # Ok::<(), cel_cxx::Error>(())
//! ```
//!
//! # Error Handling
//!
//! The module provides comprehensive error handling through:
//! - [`FromValueError`]: Conversion failures from CEL values
//! - [`FromMapKeyError`]: Map key conversion failures
//! - Detailed error messages with type information
//!
//! ## Error Example
//!
//! ```rust
//! use cel_cxx::{Value, FromValue, FromValueError};
//!
//! let string_val = Value::String("not a number".to_string().into());
//! let result = i64::from_value(&string_val);
//!
//! match result {
//!     Ok(num) => println!("Converted: {}", num),
//!     Err(e) => {
//!         println!("{}", e);
//!     }
//! }
//! ```
//!
//! # Performance Characteristics
//!
//! - **Conversion overhead**: Minimal for primitive types, optimized for references
//! - **Memory usage**: Efficient representation, shared ownership where beneficial
//! - **Type checking**: Compile-time where possible, fast runtime checks otherwise
//! - **Collection operations**: Optimized for common access patterns
//!
//! # Thread Safety
//!
//! All value types are thread-safe:
//! - Values can be shared across threads (`Send + Sync`)
//! - Reference counting handles concurrent access safely
//! - Conversion operations are atomic where required

mod opaque;
mod display;
mod traits;
mod impls;
mod optional;

use std::collections::HashMap;
use arc_slice::{ArcStr, ArcBytes};
use crate::{Kind, Error};
use crate::types::*;

pub use opaque::*;
pub use traits::*;
pub use optional::*;

/// CEL duration type.
pub type Duration = chrono::Duration;

/// CEL timestamp type.
pub type Timestamp = chrono::DateTime<chrono::Utc>;

/// CEL list value type.
pub type ListValue = Vec<Value>;

/// CEL map value type.
pub type MapValue = HashMap<MapKey, Value>;

/// CEL opaque value type.
pub type OpaqueValue = Box<dyn Opaque>;

/// CEL optional value type.
pub type OptionalValue = Optional<Value>;

/// Main CEL value type.
///
/// `Value` is the core value type of the CEL expression system, supporting all data types
/// defined by the CEL specification. Each variant corresponds to a CEL basic type or composite type.
///
/// # CEL Type Mapping
///
/// - `Null` → CEL null
/// - `Bool` → CEL bool
/// - `Int` → CEL int (64-bit signed integer)
/// - `Uint` → CEL uint (64-bit unsigned integer)
/// - `Double` → CEL double (64-bit floating point)
/// - `String` → CEL string
/// - `Bytes` → CEL bytes
/// - `Duration` → CEL duration (Protocol Buffers Duration)
/// - `Timestamp` → CEL timestamp (Protocol Buffers Timestamp)
/// - `List` → CEL list
/// - `Map` → CEL map
/// - `Type` → CEL type (type value)
/// - `Error` → CEL error
/// - `Opaque` → Opaque custom types
/// - `Optional` → Optional value type
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::Value;
///
/// // Basic types
/// let null_val = Value::Null;
/// let bool_val = Value::Bool(true);
/// let int_val = Value::Int(-42);
/// let uint_val = Value::Uint(42u64);
/// let double_val = Value::Double(3.14);
/// let string_val = Value::String("hello".to_string().into());
/// let bytes_val = Value::Bytes(vec![1, 2, 3].into());
///
/// // Time types
/// let duration = Value::Duration(chrono::Duration::seconds(30));
/// let timestamp = Value::Timestamp(chrono::Utc::now());
///
/// // Container types
/// let list = Value::List(vec![Value::Int(1), Value::Int(2)]);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Null value
    Null,

    /// Boolean value
    Bool(bool),

    /// Signed 64-bit integer
    Int(i64),

    /// Unsigned 64-bit integer
    Uint(u64),

    /// 64-bit floating point number
    Double(f64),

    /// UTF-8 string
    String(ArcStr),

    /// Byte array
    Bytes(ArcBytes),

    /// Struct (not yet implemented)
    Struct(()),

    /// Duration (Protocol Buffers Duration)
    Duration(Duration),

    /// Timestamp (Protocol Buffers Timestamp)
    Timestamp(Timestamp),

    /// List of values
    List(ListValue),

    /// Key-value map
    Map(MapValue),

    /// Unknown type (not yet implemented)
    Unknown(()),

    /// CEL type value
    Type(ValueType),

    /// Error value
    Error(Error),

    /// Opaque custom type
    Opaque(OpaqueValue),

    /// Optional value type
    Optional(OptionalValue),
}

impl Value {
    /// Returns the kind of this value.
    ///
    /// Returns the corresponding [`Kind`] enum for fast type checking.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{Value, Kind};
    ///
    /// let val = Value::String("hello".to_string().into());
    /// assert_eq!(val.kind(), Kind::String);
    ///
    /// let val = Value::List(vec![]);
    /// assert_eq!(val.kind(), Kind::List);
    /// ```
    pub fn kind(&self) -> Kind {
        match &self {
            Value::Null => Kind::Null,
            Value::Bool(_) => Kind::Bool,
            Value::Int(_i) => Kind::Int,
            Value::Uint(_u) => Kind::Uint,
            Value::Double(_d) => Kind::Double,
            Value::String(_s) => Kind::String,
            Value::Bytes(_b) => Kind::Bytes,
            Value::Struct(_s) => Kind::Struct,
            Value::Duration(_d) => Kind::Duration,
            Value::Timestamp(_t) => Kind::Timestamp,
            Value::List(_l) => Kind::List,
            Value::Map(_m) => Kind::Map,
            Value::Unknown(_u) => Kind::Unknown,
            Value::Type(_t) => Kind::Type,
            Value::Error(_e) => Kind::Error,
            Value::Opaque(_) | Value::Optional(_) => Kind::Opaque,
        }
    }
    
    /// Returns the concrete type of this value.
    ///
    /// Returns detailed [`ValueType`] information including generic parameters.
    /// For container types (List, Map), infers element or key-value types.
    ///
    /// # Type Inference Rules
    ///
    /// - **List**: Returns specific `List<T>` if all elements have the same type; otherwise `List<dyn>`
    /// - **Map**: Infers key and value types; uses `dyn` types if inconsistent
    /// - **Optional**: Infers type from contained value; uses `Optional<dyn>` for empty values
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{Value, ValueType, ListType};
    ///
    /// let val = Value::String("hello".to_string().into());
    /// assert_eq!(val.value_type(), ValueType::String);
    ///
    /// // Homogeneous list
    /// let list = Value::List(vec![Value::Int(1), Value::Int(2)]);
    /// assert_eq!(list.value_type(), ValueType::List(ListType::new(ValueType::Int)));
    ///
    /// // Heterogeneous list
    /// let mixed_list = Value::List(vec![Value::Int(1), Value::String("hello".to_string().into())]);
    /// assert_eq!(mixed_list.value_type(), ValueType::List(ListType::new(ValueType::Dyn)));
    /// ```
    pub fn value_type(&self) -> ValueType {
        match &self {
            Value::Null => ValueType::Null,
            Value::Bool(_) => ValueType::Bool,
            Value::Int(_) => ValueType::Int,
            Value::Uint(_) => ValueType::Uint,
            Value::Double(_) => ValueType::Double,
            Value::String(_) => ValueType::String,
            Value::Bytes(_) => ValueType::Bytes,
            Value::Struct(_s) => {
                todo!()
            }
            Value::Duration(_) => ValueType::Duration,
            Value::Timestamp(_) => ValueType::Timestamp,
            Value::List(list) => {
                let mut iter = list.iter();
                if let Some(v) = iter.next() {
                    let elem_type = v.value_type();
                    if elem_type == ValueType::Dyn {
                        return ValueType::List(ListType::new(ValueType::Dyn));
                    }
                    for v in iter {
                        if v.value_type() != elem_type {
                            return ValueType::List(ListType::new(ValueType::Dyn));
                        }
                    }
                    return ValueType::List(ListType::new(elem_type));
                }
                ValueType::List(ListType::new(ValueType::Dyn))
            },
            Value::Map(m) => {
                let mut iter = m.iter();
                if let Some((k, v)) = iter.next() {
                    let mut key_type = Some(k.mapkey_type());
                    let mut val_type = v.value_type();
                    for (k, v) in iter {
                        if let Some(prev_key_type) = key_type.clone() {
                            if k.mapkey_type() != prev_key_type {
                                key_type = None;
                            }
                        }
                        if val_type != ValueType::Dyn {
                            if v.value_type() != val_type {
                                val_type = ValueType::Dyn;
                            }
                        }

                        if key_type.is_none() && val_type == ValueType::Dyn {
                            break;
                        }
                    }
                    return ValueType::Map(MapType::new(key_type.unwrap_or(MapKeyType::Dyn), val_type));
                } else {
                    return ValueType::Map(MapType::new(MapKeyType::Dyn, ValueType::Dyn));
                }
            },
            Value::Unknown(_u) => ValueType::Unknown,
            Value::Opaque(o) => ValueType::Opaque(o.opaque_type()),
            Value::Optional(opt) => {
                if let Some(v) = opt.as_option() {
                    return ValueType::Optional(OptionalType::new(v.value_type()));
                }
                ValueType::Optional(OptionalType::new(ValueType::Dyn))
            },
            Value::Type(_t) => ValueType::Type(TypeType::new(None)),
            Value::Error(_e) => ValueType::Error,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl From<MapKey> for Value {
    fn from(key: MapKey) -> Self {
        match key {
            MapKey::Bool(b) => Value::Bool(b),
            MapKey::Int(i) => Value::Int(i),
            MapKey::Uint(u) => Value::Uint(u),
            MapKey::String(s) => Value::String(s),
        }
    }
}

impl TryFrom<Value> for MapKey {
    type Error = FromValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(b) => Ok(MapKey::Bool(b)),
            Value::Int(i) => Ok(MapKey::Int(i)),
            Value::Uint(u) => Ok(MapKey::Uint(u)),
            Value::String(s) => Ok(MapKey::String(s)),
            _ => Err(FromValueError::new(value, "MapKey")),
        }
    }
}

/// CEL map key type.
///
/// `MapKey` represents value types that can be used as CEL map keys. According to the CEL
/// specification, only basic comparable types can be used as map keys.
///
/// # Supported Key Types
///
/// - `Bool`: Boolean keys
/// - `Int`: Signed integer keys
/// - `Uint`: Unsigned integer keys
/// - `String`: String keys
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{MapKey, Value};
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
///
/// // Different types of keys
/// map.insert(MapKey::String("name".to_string().into()), Value::String("Alice".to_string().into()));
/// map.insert(MapKey::Int(42), Value::String("answer".to_string().into()));
/// map.insert(MapKey::Bool(true), Value::String("yes".to_string().into()));
///
/// let map_value = Value::Map(map);
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum MapKey {
    /// Boolean key
    Bool(bool),
    /// Signed integer key
    Int(i64),
    /// Unsigned integer key
    Uint(u64),
    /// String key
    String(ArcStr),
}

impl MapKey {
    /// Returns the kind of this map key.
    ///
    /// Returns the corresponding [`Kind`] enum.
    pub fn kind(&self) -> Kind {
        match self {
            MapKey::Bool(_) => Kind::Bool,
            MapKey::Int(_) => Kind::Int,
            MapKey::Uint(_) => Kind::Uint,
            MapKey::String(_) => Kind::String,
        }
    }

    /// Returns the type of this map key.
    ///
    /// Returns the corresponding [`MapKeyType`] enum.
    pub fn mapkey_type(&self) -> MapKeyType {
        match self {
            MapKey::Bool(_) => MapKeyType::Bool,
            MapKey::Int(_) => MapKeyType::Int,
            MapKey::Uint(_) => MapKeyType::Uint,
            MapKey::String(_) => MapKeyType::String,
        }
    }

    /// Creates a map key from a CEL value.
    ///
    /// Attempts to convert a [`Value`] to [`MapKey`]. Only supported basic types
    /// can be converted successfully.
    ///
    /// # Parameters
    ///
    /// - `value`: The CEL value to convert
    ///
    /// # Returns
    ///
    /// - `Ok(MapKey)`: Conversion successful
    /// - `Err(Value)`: Conversion failed, returns original value
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{Value, MapKey};
    ///
    /// // Successful conversion
    /// let key = MapKey::from_value(Value::String("key".to_string().into())).unwrap();
    /// assert_eq!(key, MapKey::String("key".to_string().into()));
    ///
    /// // Failed conversion
    /// let result = MapKey::from_value(Value::List(vec![]));
    /// assert!(result.is_err());
    /// ```
    pub fn from_value(value: Value) -> Result<Self, Value> {
        match value {
            Value::Bool(b) => Ok(MapKey::Bool(b)),
            Value::Int(i) => Ok(MapKey::Int(i)),
            Value::Uint(u) => Ok(MapKey::Uint(u)),
            Value::String(s) => Ok(MapKey::String(s)),
            _ => Err(value),
        }
    }

    /// Converts this map key to a CEL value.
    ///
    /// Converts [`MapKey`] to the corresponding [`Value`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{MapKey, Value};
    ///
    /// let key = MapKey::String("hello".to_string().into());
    /// let value = key.into_value();
    /// assert_eq!(value, Value::String("hello".to_string().into()));
    /// ```
    pub fn into_value(self) -> Value {
        match self {
            MapKey::Bool(b) => Value::Bool(b),
            MapKey::Int(i) => Value::Int(i),
            MapKey::Uint(u) => Value::Uint(u),
            MapKey::String(s) => Value::String(s),
        }
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
/// let string_const = Constant::String("hello".to_string().into());
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
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
    Bytes(ArcBytes),
    /// String constant
    String(ArcStr),
    /// Duration constant
    Duration(chrono::Duration),
    /// Timestamp constant
    Timestamp(chrono::DateTime<chrono::Utc>),
}

impl Constant {
    /// Returns the type of the constant.
    ///
    /// Returns the CEL type corresponding to the constant value.
    pub fn value_type(&self) -> ValueType {
        match self {
            Self::Null => ValueType::Null,
            Self::Bool(_) => ValueType::Bool,
            Self::Int(_) => ValueType::Int,
            Self::Uint(_) => ValueType::Uint,
            Self::Double(_) => ValueType::Double,
            Self::Bytes(_) => ValueType::Bytes,
            Self::String(_) => ValueType::String,
            Self::Duration(_) => ValueType::Duration,
            Self::Timestamp(_) => ValueType::Timestamp,
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

impl TryFrom<Value> for Constant {
    type Error = FromValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(Constant::Null),
            Value::Bool(b) => Ok(Constant::Bool(b)),
            Value::Int(i) => Ok(Constant::Int(i)),
            Value::Uint(u) => Ok(Constant::Uint(u)),
            Value::Double(d) => Ok(Constant::Double(d)),
            Value::Bytes(b) => Ok(Constant::Bytes(b)),
            Value::String(s) => Ok(Constant::String(s)),
            Value::Duration(d) => Ok(Constant::Duration(d)),
            Value::Timestamp(t) => Ok(Constant::Timestamp(t)),
            _ => Err(FromValueError::new(value, "Constant")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value_kind() {
        let cases = vec![
            (Value::Null, Kind::Null),
            (Value::Bool(true), Kind::Bool),
            (Value::Int(1), Kind::Int),
            (Value::Uint(1), Kind::Uint),
            (Value::Double(1.0), Kind::Double),
            (Value::String("test".into()), Kind::String),
            (Value::Bytes(b"abc".into()), Kind::Bytes), 
            (Value::Duration(chrono::Duration::seconds(1)), Kind::Duration),
            (Value::Timestamp(chrono::Utc::now()), Kind::Timestamp),
            (Value::List(vec![]), Kind::List),
            (Value::Map(HashMap::new()), Kind::Map),
            (Value::Type(ValueType::Null), Kind::Type),
            (Value::Error(Error::invalid_argument("invalid").into()), Kind::Error),
            (Value::Optional(Optional::none()), Kind::Opaque),
        ];

        for (value, expected_kind) in cases {
            assert_eq!(value.kind(), expected_kind);
        }
    }

    #[test]
    fn test_key_kind() {
        let cases = vec![
            (MapKey::Bool(true), Kind::Bool),
            (MapKey::Int(1), Kind::Int), 
            (MapKey::Uint(1), Kind::Uint),
            (MapKey::String("test".into()), Kind::String),
        ];

        for (key, expected_kind) in cases {
            assert_eq!(key.kind(), expected_kind);
        }
    }

    #[test]
    fn test_value_type() {
        let cases = vec![
            (Value::Null, ValueType::Null),
            (Value::Bool(true), ValueType::Bool),
            (Value::Int(1), ValueType::Int),
            (Value::Uint(1), ValueType::Uint),
            (Value::Double(1.0), ValueType::Double),
            (Value::String("test".into()), ValueType::String),
            (Value::Bytes(b"abc".into()), ValueType::Bytes),
            (Value::Duration(chrono::Duration::seconds(1)), ValueType::Duration),
            (Value::Timestamp(chrono::Utc::now()), ValueType::Timestamp),
            (Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]), ValueType::List(ListType::new(ValueType::Int))),
            (Value::Map(HashMap::from([(MapKey::String("test".into()), Value::Int(1))])), ValueType::Map(MapType::new(MapKeyType::String, ValueType::Int))),
            (Value::Type(ValueType::Double), ValueType::Type(TypeType::new(None))),
            (Value::Error(Error::invalid_argument("invalid").into()), ValueType::Error),
            (Value::Optional(Optional::new(Value::Int(5))), ValueType::Optional(OptionalType::new(ValueType::Int))),
        ];

        for (i, (value, expected_type)) in cases.into_iter().enumerate() {
            assert_eq!(value.value_type(), expected_type, "case {} failed", i);
        }
    }
}
