use crate::absl::SpanElement;
use crate::absl::StringView;

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!(<absl/strings/string_view.h>);
        type string_view<'a> = super::StringView<'a>;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<base/kind.h>);
        type Kind = super::Kind;
        fn KindToString(kind: Kind) -> string_view<'static>;

        type TypeKind = super::TypeKind;
        fn TypeKindToKind(kind: TypeKind) -> Kind;
        fn KindIsTypeKind(kind: Kind) -> bool;
        fn TypeKindToString(kind: TypeKind) -> string_view<'static>;
        fn KindToTypeKind(kind: Kind) -> TypeKind;

        type ValueKind = super::ValueKind;
        fn ValueKindToKind(kind: ValueKind) -> Kind;
        fn KindIsValueKind(kind: Kind) -> bool;
        fn ValueKindToString(kind: ValueKind) -> string_view<'static>;
        fn KindToValueKind(kind: Kind) -> ValueKind;
    }
}

/// Represents the fundamental type categories in CEL.
///
/// This enum corresponds to the `Kind` type in CEL-CPP and serves as the base type
/// classification system for CEL values and types. It provides a unified way to
/// represent both value types and type kinds in the CEL type system.
///
/// # Variants
///
/// ## Primitive Types
/// - `Null` - The null value type
/// - `Bool` - Boolean values
/// - `Int` - 64-bit signed integers
/// - `Uint` - 64-bit unsigned integers
/// - `Double` - 64-bit floating-point numbers
/// - `String` - UTF-8 encoded strings
/// - `Bytes` - Raw byte sequences
///
/// ## Complex Types
/// - `Struct` - Structured data types
/// - `List` - Homogeneous lists
/// - `Map` - Key-value mappings
///
/// ## Protocol Buffer Types
/// - `Duration` - Time duration values
/// - `Timestamp` - Point-in-time values
/// - `BoolWrapper` - Protocol buffer bool wrapper
/// - `IntWrapper` - Protocol buffer int64 wrapper
/// - `UintWrapper` - Protocol buffer uint64 wrapper
/// - `DoubleWrapper` - Protocol buffer double wrapper
/// - `StringWrapper` - Protocol buffer string wrapper
/// - `BytesWrapper` - Protocol buffer bytes wrapper
///
/// ## Special Types
/// - `Unknown` - Type not yet determined
/// - `Type` - Type of a type (meta-type)
/// - `Error` - Error type
/// - `Any` - Dynamic type that can hold any value
/// - `Dyn` - Dynamic type for runtime type checking
/// - `Opaque` - Custom opaque types
/// - `TypeParam` - Type parameter in generic types
/// - `Function` - Function type
/// - `Enum` - Enumeration type
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Kind {
    Null = 0,
    Bool,
    Int,
    Uint,
    Double,
    String,
    Bytes,
    Struct,
    Duration,
    Timestamp,
    List,
    Map,
    Unknown,
    Type,
    Error,
    Any,

    Dyn,
    Opaque,

    BoolWrapper,
    IntWrapper,
    UintWrapper,
    DoubleWrapper,
    StringWrapper,
    BytesWrapper,

    TypeParam,
    Function,
    Enum,
}

unsafe impl cxx::ExternType for Kind {
    type Id = cxx::type_id!("cel::Kind");
    type Kind = cxx::kind::Trivial;
}

impl crate::SizedExternType for Kind {}
impl SpanElement for Kind {
    type TypeId = cxx::type_id!("rust::cel_cxx::Span_Kind");
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            ffi::KindToString(*self).to_str()
                .map_err(|_| std::fmt::Error)?
        )
    }
}

impl Kind {
    pub fn is_type_kind(&self) -> bool {
        ffi::KindIsTypeKind(*self)
    }

    pub fn is_value_kind(&self) -> bool {
        ffi::KindIsValueKind(*self)
    }
}

/// Represents the type kinds in CEL's type system.
///
/// This enum is a subset of [`Kind`] that specifically represents type kinds
/// in the CEL type system. It is used for type checking and type inference
/// operations. Each variant corresponds to a specific type category that can
/// be used in type declarations and type checking.
///
/// # Relationship with [`Kind`]
///
/// `TypeKind` is a specialized view of [`Kind`] that only includes variants
/// relevant to type system operations. It maintains the same variant values
/// as [`Kind`] for compatibility, but provides a more focused interface for
/// type-related operations.
///
/// # Examples
///
/// ```rust
/// use cel_cxx_ffi::common::kind::{Kind, TypeKind};
///
/// // TypeKind can be converted to Kind
/// let type_kind = TypeKind::Int;
/// let kind: Kind = type_kind.into();
/// assert!(kind.is_type_kind());
///
/// // Not all Kind variants are valid TypeKinds
/// assert!(Kind::Int.is_type_kind());
/// assert!(!Kind::Error.is_type_kind());
/// ```
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TypeKind {
    Null = Kind::Null as u8,
    Bool = Kind::Bool as u8,
    Int = Kind::Int as u8,
    Uint = Kind::Uint as u8,
    Double = Kind::Double as u8,
    String = Kind::String as u8,
    Bytes = Kind::Bytes as u8,
    Struct = Kind::Struct as u8,
    Duration = Kind::Duration as u8,
    Timestamp = Kind::Timestamp as u8,
    List = Kind::List as u8,
    Map = Kind::Map as u8,
    Unknown = Kind::Unknown as u8,
    Type = Kind::Type as u8,
    Error = Kind::Error as u8,
    Any = Kind::Any as u8,
    Dyn = Kind::Dyn as u8,
    Opaque = Kind::Opaque as u8,

    BoolWrapper = Kind::BoolWrapper as u8,
    IntWrapper = Kind::IntWrapper as u8,
    UintWrapper = Kind::UintWrapper as u8,
    DoubleWrapper = Kind::DoubleWrapper as u8,
    StringWrapper = Kind::StringWrapper as u8,
    BytesWrapper = Kind::BytesWrapper as u8,

    TypeParam = Kind::TypeParam as u8,
    Function = Kind::Function as u8,
    Enum = Kind::Enum as u8,
}


unsafe impl cxx::ExternType for TypeKind {
    type Id = cxx::type_id!("cel::TypeKind");
    type Kind = cxx::kind::Trivial;
}

impl crate::SizedExternType for TypeKind {}
impl SpanElement for TypeKind {
    type TypeId = cxx::type_id!("rust::cel_cxx::Span_TypeKind");
}

impl std::fmt::Display for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            ffi::TypeKindToString(*self).to_str()
                .map_err(|_| std::fmt::Error)?
        )
    }
}

/// Represents the value kinds in CEL's value system.
///
/// This enum is a subset of [`Kind`] that specifically represents value kinds
/// in the CEL value system. It is used for runtime value type checking and
/// value operations. Each variant corresponds to a specific value category
/// that can be used in value operations and runtime type checking.
///
/// # Relationship with [`Kind`]
///
/// `ValueKind` is a specialized view of [`Kind`] that only includes variants
/// relevant to value operations. It maintains the same variant values as [`Kind`]
/// for compatibility, but provides a more focused interface for value-related
/// operations.
///
/// # Examples
///
/// ```rust
/// use cel_cxx_ffi::common::kind::{Kind, ValueKind};
///
/// // ValueKind can be converted to Kind
/// let value_kind = ValueKind::Int;
/// let kind: Kind = value_kind.into();
/// assert!(kind.is_value_kind());
///
/// // Not all Kind variants are valid ValueKinds
/// assert!(Kind::Int.is_value_kind());
/// assert!(!Kind::Type.is_value_kind());
/// ```
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValueKind {
    Null = Kind::Null as u8,
    Bool = Kind::Bool as u8,
    Int = Kind::Int as u8,
    Uint = Kind::Uint as u8,
    Double = Kind::Double as u8,
    String = Kind::String as u8,
    Bytes = Kind::Bytes as u8,
    Struct = Kind::Struct as u8,
    Duration = Kind::Duration as u8,
    Timestamp = Kind::Timestamp as u8,
    List = Kind::List as u8,
    Map = Kind::Map as u8,
    Unknown = Kind::Unknown as u8,
    Type = Kind::Type as u8,
    Error = Kind::Error as u8,
    Opaque = Kind::Opaque as u8,
}


unsafe impl cxx::ExternType for ValueKind {
    type Id = cxx::type_id!("cel::ValueKind");
    type Kind = cxx::kind::Trivial;
}

impl crate::SizedExternType for ValueKind {}
impl SpanElement for ValueKind {
    type TypeId = cxx::type_id!("rust::cel_cxx::Span_ValueKind");
}

impl std::fmt::Display for ValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            ffi::ValueKindToString(*self).to_str()
                .map_err(|_| std::fmt::Error)?
        )
    }
}

impl From<Kind> for TypeKind {
    fn from(kind: Kind) -> Self {
        ffi::KindToTypeKind(kind)
    }
}

impl From<Kind> for ValueKind {
    fn from(kind: Kind) -> Self {
        ffi::KindToValueKind(kind)
    }
}

impl From<TypeKind> for Kind {
    fn from(kind: TypeKind) -> Self {
        ffi::TypeKindToKind(kind)
    }
}

impl From<ValueKind> for Kind {
    fn from(kind: ValueKind) -> Self {
        ffi::ValueKindToKind(kind)
    }
}

impl std::cmp::PartialEq<TypeKind> for Kind {
    fn eq(&self, other: &TypeKind) -> bool {
        *self == ffi::TypeKindToKind(*other)
    }
}

impl std::cmp::PartialEq<ValueKind> for Kind {
    fn eq(&self, other: &ValueKind) -> bool {
        *self == ffi::ValueKindToKind(*other)
    }
}

impl std::cmp::PartialEq<Kind> for TypeKind {
    fn eq(&self, other: &Kind) -> bool {
        ffi::TypeKindToKind(*self) == *other
    }
}

impl std::cmp::PartialEq<Kind> for ValueKind {
    fn eq(&self, other: &Kind) -> bool {
        ffi::ValueKindToKind(*self) == *other
    }
}
