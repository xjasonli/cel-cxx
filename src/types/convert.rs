use super::*;

impl From<StructType> for ValueType {
    fn from(value: StructType) -> Self {
        ValueType::Struct(value)
    }
}

impl From<ListType> for ValueType {
    fn from(value: ListType) -> Self {
        ValueType::List(value)
    }
}

impl From<MapType> for ValueType {
    fn from(value: MapType) -> Self {
        ValueType::Map(value)
    }
}

impl From<TypeType> for ValueType {
    fn from(value: TypeType) -> Self {
        ValueType::Type(value)
    }
}

impl From<OpaqueType> for ValueType {
    fn from(value: OpaqueType) -> Self {
        ValueType::Opaque(value)
    }
}

impl From<OptionalType> for ValueType {
    fn from(value: OptionalType) -> Self {
        ValueType::Optional(value)
    }
}

impl From<TypeParamType> for ValueType {
    fn from(value: TypeParamType) -> Self {
        ValueType::TypeParam(value)
    }
}

impl From<FunctionType> for ValueType {
    fn from(value: FunctionType) -> Self {
        ValueType::Function(value)
    }
}

impl From<EnumType> for ValueType {
    fn from(value: EnumType) -> Self {
        ValueType::Enum(value)
    }
}

/// Error type for invalid map key type conversions.
///
/// `InvalidMapKeyType` is returned when attempting to convert a [`ValueType`] to a [`MapKeyType`]
/// fails because the type is not a valid map key type. According to the CEL specification,
/// only basic comparable types (bool, int, uint, string) can be used as map keys.
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{Type, MapKeyType, InvalidMapKeyType};
///
/// // Valid map key type
/// let string_type = Type::String;
/// let map_key_type = MapKeyType::try_from(string_type).unwrap();
/// assert_eq!(map_key_type, MapKeyType::String);
///
/// // Invalid map key type
/// let list_type = Type::List(ListType::new(Type::Int));
/// let result = MapKeyType::try_from(list_type);
/// assert!(result.is_err());
/// ```
#[derive(Debug, Clone)]
pub struct InvalidMapKeyType(pub ValueType);

impl InvalidMapKeyType {
    /// Creates a new `InvalidMapKeyType` error.
    ///
    /// # Parameters
    ///
    /// - `ty`: The type that failed to convert to a map key type
    ///
    /// # Returns
    ///
    /// New `InvalidMapKeyType` instance
    pub fn new(ty: ValueType) -> Self {
        Self(ty)
    }
}

impl std::fmt::Display for InvalidMapKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid key type: {}", self.0)
    }
}

impl std::error::Error for InvalidMapKeyType {}

impl From<MapKeyType> for ValueType {
    fn from(key: MapKeyType) -> Self {
        match key {
            MapKeyType::Bool => ValueType::Bool,
            MapKeyType::Int => ValueType::Int,
            MapKeyType::Uint => ValueType::Uint,
            MapKeyType::String => ValueType::String,
            MapKeyType::Dyn => ValueType::Dyn,
            MapKeyType::TypeParam(tp) => ValueType::TypeParam(tp),
        }
    }
}

impl TryFrom<ValueType> for MapKeyType {
    type Error = InvalidMapKeyType;

    fn try_from(value: ValueType) -> Result<Self, Self::Error> {
        match value {
            ValueType::Bool => Ok(MapKeyType::Bool),
            ValueType::Int => Ok(MapKeyType::Int),
            ValueType::Uint => Ok(MapKeyType::Uint),
            ValueType::String => Ok(MapKeyType::String),
            ValueType::Dyn => Ok(MapKeyType::Dyn),
            ValueType::TypeParam(tp) => Ok(MapKeyType::TypeParam(tp)),
            _ => Err(InvalidMapKeyType::new(value)),
        }
    }
}
