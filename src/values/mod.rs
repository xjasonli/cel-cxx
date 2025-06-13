//! CEL value type system.
//!
//! This module defines the value type system for CEL expressions, including all basic and composite data types.
//!
//! # Primary Types
//!
//! - [`values::Value`]: Main CEL value type enum supporting all CEL data types
//! - [`values::MapKey`]: Map key type supporting only basic comparable types
//! - [`values::Optional`]: Optional value type similar to `Option<T>` but CEL-compatible
//! - [`values::Opaque`]: Opaque value type for storing externally-defined custom types
//!
//! # Conversion Traits
//!
//! - [`values::IntoValue`]: Convert Rust types to CEL values
//! - [`values::FromValue`]: Convert CEL values to Rust types
//! - [`values::TypedValue`]: Values with type information
//! - [`values::IntoMapKey`] / [`values::FromMapKey`]: Map key conversions
//!
//! # Examples
//!
//! ```rust,no_run
//! use cel_cxx::{Value, MapKey};
//! use std::collections::HashMap;
//!
//! // Basic value types
//! let null = Value::Null;
//! let boolean = Value::Bool(true);
//! let integer = Value::Int(42);
//! let string = Value::String("hello".to_string());
//!
//! // Composite types
//! let list = Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
//!
//! let mut map = HashMap::new();
//! map.insert(MapKey::String("key".to_string()), Value::String("value".to_string()));
//! let map_value = Value::Map(map);
//!
//! // Type checking
//! assert_eq!(integer.kind(), cel_cxx::Kind::Int);
//! assert_eq!(string.value_type(), cel_cxx::Type::String);
//! ```

mod opaque;
mod display;
mod traits;
mod impls;
mod optional;

use std::collections::HashMap;
use crate::{Kind, Error};
use crate::types::*;

pub use opaque::*;
pub use traits::*;
pub use optional::*;


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
/// let string_val = Value::String("hello".to_string());
/// let bytes_val = Value::Bytes(vec![1, 2, 3]);
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
    String(String),

    /// Byte array
    Bytes(Vec<u8>),

    /// Struct (not yet implemented)
    Struct(()),

    /// Duration (Protocol Buffers Duration)
    Duration(chrono::Duration),

    /// Timestamp (Protocol Buffers Timestamp)
    Timestamp(chrono::DateTime<chrono::Utc>),

    /// List of values
    List(Vec<Value>),

    /// Key-value map
    Map(HashMap<MapKey, Value>),

    /// Unknown type (not yet implemented)
    Unknown(()),

    /// CEL type value
    Type(Type),

    /// Error value
    Error(Error),

    /// Opaque custom type
    Opaque(Opaque),

    /// Optional value type
    Optional(Optional<Value>),
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
    /// let val = Value::String("hello".to_string());
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
    /// Returns detailed [`Type`] information including generic parameters.
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
    /// use cel_cxx::{Value, Type, ListType};
    ///
    /// let val = Value::String("hello".to_string());
    /// assert_eq!(val.value_type(), Type::String);
    ///
    /// // Homogeneous list
    /// let list = Value::List(vec![Value::Int(1), Value::Int(2)]);
    /// assert_eq!(list.value_type(), Type::List(ListType::new(Type::Int)));
    ///
    /// // Heterogeneous list
    /// let mixed_list = Value::List(vec![Value::Int(1), Value::String("hello".to_string())]);
    /// assert_eq!(mixed_list.value_type(), Type::List(ListType::new(Type::Dyn)));
    /// ```
    pub fn value_type(&self) -> Type {
        match &self {
            Value::Null => Type::Null,
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::Uint(_) => Type::Uint,
            Value::Double(_) => Type::Double,
            Value::String(_) => Type::String,
            Value::Bytes(_) => Type::Bytes,
            Value::Struct(_s) => {
                todo!()
            }
            Value::Duration(_) => Type::Duration,
            Value::Timestamp(_) => Type::Timestamp,
            Value::List(list) => {
                let mut iter = list.iter();
                if let Some(v) = iter.next() {
                    let elem_type = v.value_type();
                    if elem_type == Type::Dyn {
                        return Type::List(ListType::new(Type::Dyn));
                    }
                    for v in iter {
                        if v.value_type() != elem_type {
                            return Type::List(ListType::new(Type::Dyn));
                        }
                    }
                    return Type::List(ListType::new(elem_type));
                }
                Type::List(ListType::new(Type::Dyn))
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
                        if val_type != Type::Dyn {
                            if v.value_type() != val_type {
                                val_type = Type::Dyn;
                            }
                        }

                        if key_type.is_none() && val_type == Type::Dyn {
                            break;
                        }
                    }
                    return Type::Map(MapType::new(key_type.unwrap_or(MapKeyType::Dyn), val_type));
                } else {
                    return Type::Map(MapType::new(MapKeyType::Dyn, Type::Dyn));
                }
            },
            Value::Unknown(_u) => Type::Unknown,
            Value::Opaque(o) => Type::Opaque(o.opaque_type()),
            Value::Optional(opt) => {
                if let Some(v) = opt.as_option() {
                    return Type::Optional(OptionalType::new(v.value_type()));
                }
                Type::Optional(OptionalType::new(Type::Dyn))
            },
            Value::Type(_t) => Type::Type(TypeType::new(None)),
            Value::Error(_e) => Type::Error,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
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
/// map.insert(MapKey::String("name".to_string()), Value::String("Alice".to_string()));
/// map.insert(MapKey::Int(42), Value::String("answer".to_string()));
/// map.insert(MapKey::Bool(true), Value::String("yes".to_string()));
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
    String(String),
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
    /// let key = MapKey::from_value(Value::String("key".to_string())).unwrap();
    /// assert_eq!(key, MapKey::String("key".to_string()));
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
    /// let key = MapKey::String("hello".to_string());
    /// let value = key.into_value();
    /// assert_eq!(value, Value::String("hello".to_string()));
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

impl From<&str> for MapKey {
    fn from(value: &str) -> Self {
        MapKey::String(value.to_owned())
    }
}

impl From<&String> for MapKey {
    fn from(value: &String) -> Self {
        MapKey::String(value.to_owned())
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
            (Value::String("test".to_string()), Kind::String),
            (Value::Bytes(vec![1,2,3]), Kind::Bytes), 
            (Value::Duration(chrono::Duration::seconds(1)), Kind::Duration),
            (Value::Timestamp(chrono::Utc::now()), Kind::Timestamp),
            (Value::List(vec![]), Kind::List),
            (Value::Map(HashMap::new()), Kind::Map),
            (Value::Type(Type::Null), Kind::Type),
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
            (MapKey::String("test".to_string()), Kind::String),
        ];

        for (key, expected_kind) in cases {
            assert_eq!(key.kind(), expected_kind);
        }
    }

    #[test]
    fn test_value_type() {
        let cases = vec![
            (Value::Null, Type::Null),
            (Value::Bool(true), Type::Bool),
            (Value::Int(1), Type::Int),
            (Value::Uint(1), Type::Uint),
            (Value::Double(1.0), Type::Double),
            (Value::String("test".to_string()), Type::String),
            (Value::Bytes(vec![1,2,3]), Type::Bytes),
            (Value::Duration(chrono::Duration::seconds(1)), Type::Duration),
            (Value::Timestamp(chrono::Utc::now()), Type::Timestamp),
            (Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]), Type::List(ListType::new(Type::Int))),
            (Value::Map(HashMap::from([(MapKey::String("test".to_string()), Value::Int(1))])), Type::Map(MapType::new(MapKeyType::String, Type::Int))),
            (Value::Type(Type::Double), Type::Type(TypeType::new(None))),
            (Value::Error(Error::invalid_argument("invalid").into()), Type::Error),
            (Value::Optional(Optional::new(Value::Int(5))), Type::Optional(OptionalType::new(Type::Int))),
        ];

        for (i, (value, expected_type)) in cases.into_iter().enumerate() {
            assert_eq!(value.value_type(), expected_type, "case {} failed", i);
        }
    }
}
