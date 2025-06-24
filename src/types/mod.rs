//! CEL type system types and type definitions.
//!
//! This module provides the core type system for CEL expressions, including
//! the [`ValueType`] enum and type conversion utilities.
//!
//! This module provides the complete type system for CEL (Common Expression Language).
//! It defines all the types that can be used in CEL expressions, from primitive types
//! like integers and strings to complex types like lists, maps, and Protocol Buffer messages.
//!
//! # Type Hierarchy
//!
//! The CEL type system is built around the [`ValueType`] enum, which represents all
//! possible types in CEL:
//!
//! ## Primitive Types
//! - **`null`**: Represents the absence of a value
//! - **`bool`**: Boolean values (`true`/`false`)
//! - **`int`**: 64-bit signed integers
//! - **`uint`**: 64-bit unsigned integers  
//! - **`double`**: Double-precision floating point numbers
//! - **`string`**: UTF-8 encoded text
//! - **`bytes`**: Byte arrays
//!
//! ## Time Types
//! - **`duration`**: Time spans (from Protocol Buffers)
//! - **`timestamp`**: Points in time (from Protocol Buffers)
//!
//! ## Collection Types
//! - **`list<T>`**: Ordered sequences of values
//! - **`map<K, V>`**: Key-value mappings
//!
//! ## Protocol Buffer Types
//! - **`struct`**: Protocol Buffer messages
//! - **Wrapper types**: `BoolValue`, `StringValue`, etc.
//! - **`any`**: Can hold any Protocol Buffer message
//! - **`enum`**: Protocol Buffer enumerations
//!
//! ## Advanced Types
//! - **`type`**: Represents types themselves as values
//! - **`function`**: Function signatures
//! - **`optional<T>`**: Nullable values
//! - **Opaque types**: Custom user-defined types
//! - **Type parameters**: For generic type definitions
//!
//! # Examples
//!
//! ## Working with primitive types
//!
//! ```rust,no_run
//! use cel_cxx::types::*;
//! use cel_cxx::Kind;
//!
//! // Create primitive types
//! let int_type = ValueType::Int;
//! let string_type = ValueType::String;
//! let bool_type = ValueType::Bool;
//!
//! // Check type kinds
//! assert_eq!(int_type.kind(), Kind::Int);
//! assert_eq!(string_type.kind(), Kind::String);
//! assert_eq!(bool_type.kind(), Kind::Bool);
//! ```
//!
//! ## Working with collection types
//!
//! ```rust,no_run
//! use cel_cxx::types::*;
//!
//! // List of strings: list<string>
//! let string_list = ValueType::List(ListType::new(ValueType::String));
//!
//! // Map from string to int: map<string, int>
//! let string_to_int_map = ValueType::Map(MapType::new(
//!     MapKeyType::String,
//!     ValueType::Int
//! ));
//!
//! // Nested types: list<map<string, int>>
//! let nested_type = ValueType::List(ListType::new(string_to_int_map));
//! ```
//!
//! ## Working with optional types
//!
//! ```rust,no_run
//! use cel_cxx::types::*;
//!
//! // Optional string: optional<string>
//! let optional_string = ValueType::Optional(OptionalType::new(ValueType::String));
//!
//! // Optional list: optional<list<int>>
//! let optional_list = ValueType::Optional(OptionalType::new(
//!     ValueType::List(ListType::new(ValueType::Int))
//! ));
//! ```
//!
//! ## Working with function types
//!
//! ```rust,no_run
//! use cel_cxx::types::*;
//!
//! // Function type: (string, int) -> bool
//! let func_type = ValueType::Function(FunctionType::new(
//!     ValueType::Bool,
//!     vec![ValueType::String, ValueType::Int]
//! ));
//! ```

use std::fmt::Debug;

mod display;
mod convert;
use crate::Kind;
use crate::values::TypedValue;

pub use convert::InvalidMapKeyType;

/// CEL type representation.
/// 
/// This enum represents all possible types in the CEL type system. CEL supports
/// a rich type system including primitive types, complex types, and Protocol Buffer
/// types. Each type corresponds to values that can be used in CEL expressions.
/// 
/// # Examples
/// 
/// ## Basic Type Usage
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// // Create different types
/// let int_type = ValueType::Int;
/// let string_type = ValueType::String;
/// let list_type = ValueType::List(ListType::new(ValueType::String));
/// 
/// // Check the kind of a type
/// assert_eq!(int_type.kind(), Kind::Int);
/// ```
/// 
/// ## Map Types
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// let map_type = ValueType::Map(MapType::new(
///     MapKeyType::String,
///     ValueType::Int
/// ));
/// 
/// assert_eq!(map_type.kind(), Kind::Map);
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ValueType {
    /// Null type - represents the absence of a value.
    /// 
    /// This corresponds to CEL's `null` literal.
    Null,

    /// Boolean type - represents true/false values.
    /// 
    /// This corresponds to CEL's `bool` type and literals like `true` and `false`.
    Bool,

    /// Signed 64-bit integer type.
    /// 
    /// This corresponds to CEL's `int` type and integer literals like `42`.
    Int,

    /// Unsigned 64-bit integer type.
    /// 
    /// This corresponds to CEL's `uint` type and unsigned integer literals like `42u`.
    Uint,

    /// Double-precision floating point type.
    /// 
    /// This corresponds to CEL's `double` type and floating-point literals like `3.14`.
    Double,

    /// String type.
    /// 
    /// This corresponds to CEL's `string` type and string literals like `"hello"`.
    String,

    /// Byte array type.
    /// 
    /// This corresponds to CEL's `bytes` type and byte literals like `b"hello"`.
    Bytes,

    /// Struct type for Protocol Buffer messages.
    /// 
    /// This represents structured data types, typically Protocol Buffer messages.
    Struct(StructType),

    /// Duration type from Protocol Buffers.
    /// 
    /// This corresponds to `google.protobuf.Duration` and duration literals like `duration("1h")`.
    Duration,

    /// Timestamp type from Protocol Buffers.
    /// 
    /// This corresponds to `google.protobuf.Timestamp` and timestamp literals like `timestamp("2023-01-01T00:00:00Z")`.
    Timestamp,

    /// List type - represents ordered collections.
    /// 
    /// This corresponds to CEL's list type and literals like `[1, 2, 3]`.
    List(ListType),

    /// Map type - represents key-value mappings.
    /// 
    /// This corresponds to CEL's map type and literals like `{"key": "value"}`.
    Map(MapType),

    /// Unknown type - used for values that cannot be determined at compile time.
    /// 
    /// This is typically used in error conditions or for dynamic values.
    Unknown,

    /// Type type - represents type values themselves.
    /// 
    /// This corresponds to CEL's type system where types can be values, such as `int` or `string`.
    Type(TypeType),

    /// Error type - represents error values.
    /// 
    /// This is used when evaluation results in an error condition.
    Error,

    /// Any type from Protocol Buffers.
    /// 
    /// This corresponds to `google.protobuf.Any` which can hold any Protocol Buffer message.
    Any,

    /// Dynamic type - represents values whose type is determined at runtime.
    /// 
    /// This is used for values that can be of any type.
    Dyn,

    /// Opaque types - user-defined types that are not directly CEL types.
    /// 
    /// This allows integration of custom Rust types into CEL expressions.
    Opaque(OpaqueType),

    /// Optional type - represents values that may or may not be present.
    /// 
    /// This corresponds to CEL's optional types and the `optional` type constructor.
    Optional(OptionalType),

    /// Boolean wrapper type from Protocol Buffers.
    /// 
    /// This corresponds to `google.protobuf.BoolValue`.
    BoolWrapper,

    /// Integer wrapper type from Protocol Buffers.
    /// 
    /// This corresponds to `google.protobuf.Int32Value`.
    IntWrapper,

    /// Unsigned integer wrapper type from Protocol Buffers.
    /// 
    /// This corresponds to `google.protobuf.UInt32Value`.
    UintWrapper,

    /// Double wrapper type from Protocol Buffers.
    /// 
    /// This corresponds to `google.protobuf.DoubleValue`.
    DoubleWrapper,

    /// String wrapper type from Protocol Buffers.
    /// 
    /// This corresponds to `google.protobuf.StringValue`.
    StringWrapper,

    /// Bytes wrapper type from Protocol Buffers.
    /// 
    /// This corresponds to `google.protobuf.BytesValue`.
    BytesWrapper,

    /// Type parameter type - used in generic type definitions.
    /// 
    /// This represents type parameters in generic contexts.
    TypeParam(TypeParamType),

    /// Function type - represents function signatures.
    /// 
    /// This is used to represent the types of functions and their signatures.
    Function(FunctionType),

    /// Enum type - represents enumeration types.
    /// 
    /// This corresponds to Protocol Buffer enum types.
    Enum(EnumType),
}

impl ValueType {
    /// Returns the kind of this type.
    /// 
    /// The kind represents the basic category of the type, which is useful
    /// for type checking and dispatch logic.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// assert_eq!(ValueType::Int.kind(), Kind::Int);
    /// assert_eq!(ValueType::String.kind(), Kind::String);
    /// 
    /// let list_type = ValueType::List(ListType::new(ValueType::Int));
    /// assert_eq!(list_type.kind(), Kind::List);
    /// ```
    pub fn kind(&self) -> Kind {
        match self {
            ValueType::Null => Kind::Null,

            ValueType::Bool => Kind::Bool,
            ValueType::Int => Kind::Int,
            ValueType::Uint => Kind::Uint,
            ValueType::Double => Kind::Double,
            ValueType::String => Kind::String,
            ValueType::Bytes => Kind::Bytes,

            ValueType::Struct(_) => Kind::Struct,
            ValueType::Duration => Kind::Duration,
            ValueType::Timestamp => Kind::Timestamp,

            ValueType::List(_) => Kind::List,
            ValueType::Map(_) => Kind::Map,

            ValueType::Unknown => Kind::Unknown,
            ValueType::Type(_) => Kind::Type,
            ValueType::Error => Kind::Error,
            ValueType::Any => Kind::Any,

            ValueType::Dyn => Kind::Dyn,
            ValueType::Opaque(_) | ValueType::Optional(_) => Kind::Opaque,

            ValueType::BoolWrapper => Kind::BoolWrapper,
            ValueType::IntWrapper => Kind::IntWrapper,
            ValueType::UintWrapper => Kind::UintWrapper,
            ValueType::DoubleWrapper => Kind::DoubleWrapper,
            ValueType::StringWrapper => Kind::StringWrapper,
            ValueType::BytesWrapper => Kind::BytesWrapper,

            ValueType::TypeParam(_) => Kind::TypeParam,
            ValueType::Function(_) => Kind::Function,
            ValueType::Enum(_) => Kind::Enum,
        }
    }

    /// Checks if this type matches the type of a specific [`TypedValue`] implementation.
    /// 
    /// This method provides a type-safe way to check if a [`ValueType`] corresponds
    /// to a particular Rust type that implements [`TypedValue`]. It's particularly
    /// useful for runtime type checking and validation.
    /// 
    /// # Type Parameters
    /// 
    /// * `T` - A type that implements [`TypedValue`], representing the Rust type to check against
    /// 
    /// # Returns
    /// 
    /// Returns `true` if this [`ValueType`] matches the CEL type representation of `T`,
    /// `false` otherwise.
    /// 
    /// # Examples
    /// 
    /// ## Basic Type Checking
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// // Check primitive types
    /// assert!(ValueType::Int.is::<i64>());
    /// assert!(ValueType::String.is::<String>());
    /// assert!(ValueType::Bool.is::<bool>());
    /// 
    /// // Check against wrong types
    /// assert!(!ValueType::Int.is::<String>());
    /// assert!(!ValueType::String.is::<bool>());
    /// ```
    /// 
    /// ## Collection Type Checking
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// use std::collections::HashMap;
    /// 
    /// let list_type = ValueType::List(ListType::new(ValueType::Int));
    /// let map_type = ValueType::Map(MapType::new(MapKeyType::String, ValueType::Int));
    /// 
    /// // Check collection types
    /// assert!(list_type.is::<Vec<i64>>());
    /// assert!(map_type.is::<HashMap<String, i64>>());
    /// 
    /// // Check against incompatible collection types
    /// assert!(!list_type.is::<Vec<String>>());
    /// assert!(!map_type.is::<HashMap<i64, String>>());
    /// ```
    /// 
    /// ## Optional Type Checking
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let optional_string = ValueType::Optional(OptionalType::new(ValueType::String));
    /// 
    /// // Check optional types
    /// assert!(optional_string.is::<Option<String>>());
    /// assert!(!optional_string.is::<Option<i64>>());
    /// assert!(!optional_string.is::<String>()); // Not optional
    /// ```
    /// 
    /// ## Custom Opaque Type Checking
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// #[derive(Opaque, Debug, Clone, PartialEq)]
    /// #[cel_cxx(type = "my.CustomType")]
    /// struct CustomType {
    ///     value: i32,
    /// }
    /// impl std::fmt::Display for CustomType {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         write!(f, "CustomType({})", self.value)
    ///     }
    /// }
    /// 
    /// let opaque_type = <CustomType as TypedValue>::value_type();
    /// 
    /// // Check custom opaque type
    /// assert!(opaque_type.is::<CustomType>());
    /// assert!(!opaque_type.is::<i32>());
    /// ```
    /// 
    /// # Use Cases
    /// 
    /// This method is commonly used in:
    /// - **Function implementations**: Validating argument types before processing
    /// - **Type guards**: Ensuring type safety in generic contexts  
    /// - **Error handling**: Providing detailed type mismatch error messages
    /// - **Debugging**: Runtime inspection of type compatibility
    /// 
    /// # See Also
    /// 
    /// - [`TypedValue::value_type()`]: Get the [`ValueType`] representation of a Rust type
    /// - [`ValueType::kind()`]: Get the basic kind category of a type
    pub fn is<T: TypedValue>(&self) -> bool {
        T::value_type() == *self
    }
}

/// Struct type for Protocol Buffer messages.
/// 
/// This represents a structured type, typically corresponding to a Protocol Buffer
/// message type. Struct types are identified by their fully qualified name.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// let struct_type = StructType::new("google.protobuf.Duration");
/// let type_instance = ValueType::Struct(struct_type);
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct StructType {
    /// The fully qualified name of the struct type.
    pub name: String,
}

impl StructType {
    /// Creates a new struct type with the given name.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The fully qualified name of the struct type
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let struct_type = StructType::new("my.package.MyMessage");
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}

/// List type representing ordered collections.
/// 
/// List types specify the type of elements they contain. All elements in a
/// CEL list must be of the same type (or compatible types).
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// // List of integers: list<int>
/// let int_list = ListType::new(ValueType::Int);
/// 
/// // List of strings: list<string>
/// let string_list = ListType::new(ValueType::String);
/// 
/// // Nested list: list<list<int>>
/// let nested_list = ListType::new(ValueType::List(int_list));
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ListType {
    /// The type of elements in this list.
    pub element: Box<ValueType>,
}

impl ListType {
    /// Creates a new list type with the given element type.
    /// 
    /// # Arguments
    /// 
    /// * `element` - The type of elements in the list
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let list_type = ListType::new(ValueType::String);
    /// assert_eq!(list_type.element(), &ValueType::String);
    /// ```
    pub fn new(element: ValueType) -> Self {
        Self { element: Box::new(element) }
    }

    /// Returns the element type of this list.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let list_type = ListType::new(ValueType::Int);
    /// assert_eq!(list_type.element(), &ValueType::Int);
    /// ```
    pub fn element(&self) -> &ValueType {
        &self.element
    }
}

/// Map type representing key-value mappings.
/// 
/// Map types specify both the type of keys and the type of values. Not all
/// types can be used as map keys - only certain primitive types are allowed.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// // Map from string to int: map<string, int>
/// let string_to_int = MapType::new(MapKeyType::String, ValueType::Int);
/// 
/// // Map from int to list of strings: map<int, list<string>>
/// let complex_map = MapType::new(
///     MapKeyType::Int,
///     ValueType::List(ListType::new(ValueType::String))
/// );
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MapType {
    /// The type of keys in this map.
    pub key: MapKeyType,
    /// The type of values in this map.
    pub value: Box<ValueType>,
}

impl MapType {
    /// Creates a new map type with the given key and value types.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The type of keys in the map
    /// * `value` - The type of values in the map
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let map_type = MapType::new(MapKeyType::String, ValueType::Int);
    /// assert_eq!(map_type.key(), &MapKeyType::String);
    /// assert_eq!(map_type.value(), &ValueType::Int);
    /// ```
    pub fn new(key: MapKeyType, value: ValueType) -> Self {
        Self { key, value: Box::new(value) }
    }

    /// Returns the key type of this map.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let map_type = MapType::new(MapKeyType::Int, ValueType::String);
    /// assert_eq!(map_type.key(), &MapKeyType::Int);
    /// ```
    pub fn key(&self) -> &MapKeyType {
        &self.key
    }

    /// Returns the value type of this map.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let map_type = MapType::new(MapKeyType::String, ValueType::Double);
    /// assert_eq!(map_type.value(), &ValueType::Double);
    /// ```
    pub fn value(&self) -> &ValueType {
        &self.value
    }
}

/// Type type representing type values.
/// 
/// This represents CEL's type system where types themselves can be values.
/// For example, the expression `type(42)` returns a type value representing `int`.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// // Generic type: type
/// let generic_type = TypeType::new(None);
/// 
/// // Parameterized type: type<string>
/// let parameterized_type = TypeType::new(Some(ValueType::String));
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TypeType {
    /// Optional type parameter for parameterized types.
    pub parameter: Option<Box<ValueType>>,
}

impl TypeType {
    /// Creates a new type type with an optional parameter.
    /// 
    /// # Arguments
    /// 
    /// * `parameter` - Optional type parameter
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// // Generic type
    /// let generic = TypeType::new(None);
    /// 
    /// // Specific type
    /// let specific = TypeType::new(Some(ValueType::String));
    /// ```
    pub fn new(parameter: Option<ValueType>) -> Self {
        Self { parameter: parameter.map(Box::new) }
    }

    /// Returns the type parameter if present.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let type_type = TypeType::new(Some(ValueType::Int));
    /// assert_eq!(type_type.parameter(), Some(&ValueType::Int));
    /// ```
    pub fn parameter(&self) -> Option<&ValueType> {
        self.parameter.as_ref().map(|p| &**p)
    }
}

/// Opaque type for user-defined types.
/// 
/// Opaque types allow you to integrate custom Rust types into CEL expressions.
/// They are identified by name and can have type parameters for generic types.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// // Simple opaque type: MyType
/// let simple = OpaqueType::new("MyType", vec![]);
/// 
/// // Generic opaque type: MyGeneric<string, int>
/// let generic = OpaqueType::new("MyGeneric", vec![ValueType::String, ValueType::Int]);
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct OpaqueType {
    /// The name of the opaque type.
    pub name: String,
    /// Type parameters for generic opaque types.
    pub parameters: Vec<ValueType>,
}

impl OpaqueType {
    /// Creates a new opaque type with the given name and type parameters.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the opaque type
    /// * `parameters` - Type parameters for generic opaque types
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// // Simple opaque type
    /// let simple = OpaqueType::new("MyType", vec![]);
    /// 
    /// // Generic opaque type
    /// let generic = OpaqueType::new("Container", vec![ValueType::String]);
    /// ```
    pub fn new<S: Into<String>>(name: S, parameters: Vec<ValueType>) -> Self {
        Self { name: name.into(), parameters }
    }

    /// Returns the name of this opaque type.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let opaque_type = OpaqueType::new("MyType", vec![]);
    /// assert_eq!(opaque_type.name(), "MyType");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the type parameters of this opaque type.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let opaque_type = OpaqueType::new("Container", vec![ValueType::Int, ValueType::String]);
    /// assert_eq!(opaque_type.parameters().len(), 2);
    /// ```
    pub fn parameters(&self) -> &[ValueType] {
        &self.parameters
    }
}

/// Optional type representing values that may or may not be present.
/// 
/// Optional types wrap another type to indicate that values of that type
/// may be absent. This is similar to Rust's `Option<T>` type.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// // Optional string: optional<string>
/// let optional_string = OptionalType::new(ValueType::String);
/// 
/// // Optional list: optional<list<int>>
/// let optional_list = OptionalType::new(
///     ValueType::List(ListType::new(ValueType::Int))
/// );
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct OptionalType {
    /// The type that may or may not be present.
    pub parameter: Box<ValueType>,
}

impl OptionalType {
    /// Creates a new optional type wrapping the given type.
    /// 
    /// # Arguments
    /// 
    /// * `parameter` - The type that may be optional
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let optional_int = OptionalType::new(ValueType::Int);
    /// assert_eq!(optional_int.parameter(), &ValueType::Int);
    /// ```
    pub fn new(parameter: ValueType) -> Self {
        Self { parameter: Box::new(parameter) }
    }

    /// Returns the wrapped type.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let optional_string = OptionalType::new(ValueType::String);
    /// assert_eq!(optional_string.parameter(), &ValueType::String);
    /// ```
    pub fn parameter(&self) -> &ValueType {
        &self.parameter
    }
}

/// Type parameter type used in generic type definitions.
/// 
/// Type parameters represent placeholders for types in generic contexts.
/// They are typically used in function signatures and generic type definitions.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// let type_param = TypeParamType::new("T");
/// assert_eq!(type_param.name(), "T");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TypeParamType {
    /// The name of the type parameter.
    pub name: String,
}

impl TypeParamType {
    /// Creates a new type parameter with the given name.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the type parameter (e.g., "T", "U", "Key", "Value")
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let type_param = TypeParamType::new("T");
    /// let another_param = TypeParamType::new("Key");
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }

    /// Returns the name of this type parameter.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let type_param = TypeParamType::new("T");
    /// assert_eq!(type_param.name(), "T");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Function type representing function signatures.
/// 
/// Function types describe the signature of functions, including their
/// parameter types and return type. This is used for type checking
/// function calls and declarations.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// // Function type: (string, int) -> bool
/// let func_type = FunctionType::new(
///     ValueType::Bool,
///     vec![ValueType::String, ValueType::Int]
/// );
/// 
/// // No-argument function: () -> string
/// let no_arg_func = FunctionType::new(ValueType::String, vec![]);
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FunctionType {
    /// The return type of the function.
    pub result: Box<ValueType>,
    /// The parameter types of the function.
    pub arguments: Vec<ValueType>,
}

impl FunctionType {
    /// Creates a new function type with the given return type and parameters.
    /// 
    /// # Arguments
    /// 
    /// * `result` - The return type of the function
    /// * `arguments` - The parameter types of the function
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// // Function that takes two ints and returns a string
    /// let func_type = FunctionType::new(
    ///     ValueType::String,
    ///     vec![ValueType::Int, ValueType::Int]
    /// );
    /// ```
    pub fn new(result: ValueType, arguments: Vec<ValueType>) -> Self {
        Self {
            result: Box::new(result),
            arguments,
        }
    }

    /// Returns the return type of this function.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let func_type = FunctionType::new(ValueType::Bool, vec![ValueType::String]);
    /// assert_eq!(func_type.result(), &ValueType::Bool);
    /// ```
    pub fn result(&self) -> &ValueType {
        &self.result
    }

    /// Returns the parameter types of this function.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let func_type = FunctionType::new(ValueType::Int, vec![ValueType::String, ValueType::Bool]);
    /// assert_eq!(func_type.arguments().len(), 2);
    /// ```
    pub fn arguments(&self) -> &[ValueType] {
        &self.arguments
    }

    /// Generates a unique identifier for this function type.
    /// 
    /// This creates a string representation of the function signature that
    /// can be used for function resolution and overload disambiguation.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the function
    /// * `member` - Whether this is a member function
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let func_type = FunctionType::new(ValueType::Int, vec![ValueType::String]);
    /// let id = func_type.id("myFunc", false);
    /// // Results in something like "myFunc(string)"
    /// ```
    pub fn id(&self, name: &str, member: bool) -> String {
        use itertools::Itertools;
        if member && self.arguments.len() > 0 {
            format!("({}).{}({})", self.arguments[0], name, self.arguments[1..].iter().format(", "))
        } else {
            format!("{}({})", name, self.arguments.iter().format(", "))
        }
    }
}

/// Enum type representing enumeration types.
/// 
/// Enum types correspond to Protocol Buffer enum types and represent
/// a fixed set of named integer constants.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// let enum_type = EnumType::new("my.package.Color");
/// assert_eq!(enum_type.name(), "my.package.Color");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct EnumType {
    /// The fully qualified name of the enum type.
    pub name: String,
}

impl EnumType {
    /// Creates a new enum type with the given name.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The fully qualified name of the enum type
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let enum_type = EnumType::new("google.rpc.Code");
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }

    /// Returns the name of this enum type.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// let enum_type = EnumType::new("my.Status");
    /// assert_eq!(enum_type.name(), "my.Status");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Types that can be used as map keys.
/// 
/// CEL maps can only use certain types as keys. This enum represents
/// the allowed key types in CEL map literals and map type definitions.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use cel_cxx::*;
/// 
/// // String keys are common
/// let string_key_map = MapType::new(MapKeyType::String, ValueType::Int);
/// 
/// // Integer keys are also supported
/// let int_key_map = MapType::new(MapKeyType::Int, ValueType::String);
/// 
/// // Check the kind of a key type
/// assert_eq!(MapKeyType::String.kind(), Kind::String);
/// ```
/// 
/// # Note
/// 
/// Not all CEL types can be used as map keys. Only primitive types that
/// are hashable and comparable are allowed.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum MapKeyType {
    /// Boolean keys - `true` and `false`.
    Bool,
    
    /// Signed integer keys.
    Int,
    
    /// Unsigned integer keys.
    Uint,
    
    /// String keys (most common).
    String,
    
    /// Dynamic key type - determined at runtime.
    Dyn,
    
    /// Type parameter key type - used in generic contexts.
    TypeParam(TypeParamType),
}

impl MapKeyType {
    /// Returns the kind of this map key type.
    /// 
    /// This provides the basic category of the key type for type checking
    /// and dispatch purposes.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use cel_cxx::*;
    /// 
    /// assert_eq!(MapKeyType::String.kind(), Kind::String);
    /// assert_eq!(MapKeyType::Int.kind(), Kind::Int);
    /// assert_eq!(MapKeyType::Bool.kind(), Kind::Bool);
    /// ```
    pub fn kind(&self) -> Kind {
        match self {
            MapKeyType::Bool => Kind::Bool,
            MapKeyType::Int => Kind::Int,
            MapKeyType::Uint => Kind::Uint,
            MapKeyType::String => Kind::String,
            MapKeyType::Dyn => Kind::Dyn,
            MapKeyType::TypeParam(_) => Kind::TypeParam,
        }
    }
}
