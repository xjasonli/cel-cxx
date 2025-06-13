use super::*;

impl From<StructType> for Type {
    fn from(value: StructType) -> Self {
        Type::Struct(value)
    }
}

impl From<ListType> for Type {
    fn from(value: ListType) -> Self {
        Type::List(value)
    }
}

impl From<MapType> for Type {
    fn from(value: MapType) -> Self {
        Type::Map(value)
    }
}

impl From<TypeType> for Type {
    fn from(value: TypeType) -> Self {
        Type::Type(value)
    }
}

impl From<OpaqueType> for Type {
    fn from(value: OpaqueType) -> Self {
        Type::Opaque(value)
    }
}

impl From<OptionalType> for Type {
    fn from(value: OptionalType) -> Self {
        Type::Optional(value)
    }
}

impl From<TypeParamType> for Type {
    fn from(value: TypeParamType) -> Self {
        Type::TypeParam(value)
    }
}

impl From<FunctionType> for Type {
    fn from(value: FunctionType) -> Self {
        Type::Function(value)
    }
}

impl From<EnumType> for Type {
    fn from(value: EnumType) -> Self {
        Type::Enum(value)
    }
}

/// Error type for invalid map key type conversions.
///
/// `InvalidMapKeyType` is returned when attempting to convert a [`Type`] to a [`MapKeyType`]
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
pub struct InvalidMapKeyType(pub Type);

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
    pub fn new(ty: Type) -> Self {
        Self(ty)
    }
}

impl std::fmt::Display for InvalidMapKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid key type: {}", self.0)
    }
}

impl std::error::Error for InvalidMapKeyType {}

impl From<MapKeyType> for Type {
    fn from(key: MapKeyType) -> Self {
        match key {
            MapKeyType::Bool => Type::Bool,
            MapKeyType::Int => Type::Int,
            MapKeyType::Uint => Type::Uint,
            MapKeyType::String => Type::String,
            MapKeyType::Dyn => Type::Dyn,
            MapKeyType::TypeParam(tp) => Type::TypeParam(tp),
        }
    }
}

impl TryFrom<Type> for MapKeyType {
    type Error = InvalidMapKeyType;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        match value {
            Type::Bool => Ok(MapKeyType::Bool),
            Type::Int => Ok(MapKeyType::Int),
            Type::Uint => Ok(MapKeyType::Uint),
            Type::String => Ok(MapKeyType::String),
            Type::Dyn => Ok(MapKeyType::Dyn),
            Type::TypeParam(tp) => Ok(MapKeyType::TypeParam(tp)),
            _ => Err(InvalidMapKeyType::new(value)),
        }
    }
}
