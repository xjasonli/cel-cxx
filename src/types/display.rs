use itertools::Itertools;
use std::fmt::Display;
use super::*;

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null | Self::Unknown | Self::Error | Self::Any | Self::Dyn |
            Self::Bool | Self::Int | Self::Uint | Self::Double | Self::String | Self::Bytes |
            Self::BoolWrapper | Self::IntWrapper | Self::UintWrapper |
            Self::DoubleWrapper | Self::StringWrapper | Self::BytesWrapper |
            Self::Duration | Self::Timestamp => {
                Display::fmt(&self.kind(), f)
            }
            Self::Struct(struct_) => Display::fmt(struct_, f),
            Self::List(list) => Display::fmt(list, f),
            Self::Map(map) => Display::fmt(map, f),
            Self::Type(type_type) => Display::fmt(type_type, f),
            Self::Opaque(opaque) => Display::fmt(opaque, f),
            Self::Optional(optional) => Display::fmt(optional, f),
            Self::TypeParam(type_param) => Display::fmt(type_param, f),
            Self::Function(function) => Display::fmt(function, f),
            Self::Enum(enum_) => Display::fmt(enum_, f),
        }
    }
}

impl std::fmt::Display for StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::fmt::Display for ListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "list<{}>", self.element)
    }
}

impl std::fmt::Display for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "map<{}, {}>", self.key, self.value)
    }
}

impl std::fmt::Display for TypeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(parameter) = &self.parameter {
            write!(f, "type<{}>", parameter)
        } else {
            write!(f, "type")
        }
    }
}

impl std::fmt::Display for OpaqueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.parameters().is_empty() {
            write!(f, "{}", self.name())
        } else {
            write!(f, "{}<{}>", self.name(), self.parameters().iter().format(", "))
        }
    }
}

impl std::fmt::Display for OptionalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "optional<{}>", self.parameter)
    }
}

impl std::fmt::Display for TypeParamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::fmt::Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) -> {}", self.arguments.iter().format(", "), self.result)
    }
}

impl std::fmt::Display for EnumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::fmt::Display for MapKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => f.write_str("bool"),
            Self::Int => f.write_str("int"),
            Self::Uint => f.write_str("uint"),
            Self::String => f.write_str("string"),
            Self::Dyn => f.write_str("dyn"),
            Self::TypeParam(type_param) => Display::fmt(type_param, f),
        }
    }
} 
