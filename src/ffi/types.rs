use super::*;

pub(crate) fn type_from_rust<'a>(
    type_: &rust::ValueType,
    arena: &'a Arena,
    descriptor_pool: &'a DescriptorPool,
) -> Type<'a> {
    match type_ {
        rust::ValueType::Null => Type::new_null(),
        rust::ValueType::Bool => Type::new_bool(),
        rust::ValueType::Int => Type::new_int(),
        rust::ValueType::Uint => Type::new_uint(),
        rust::ValueType::Double => Type::new_double(),
        rust::ValueType::String => Type::new_string(),
        rust::ValueType::Bytes => Type::new_bytes(),
        rust::ValueType::Struct(_struct_type) => {
            todo!()
        }
        rust::ValueType::Duration => Type::new_duration(),
        rust::ValueType::Timestamp => Type::new_timestamp(),
        rust::ValueType::List(list_type) => {
            let list_type = list_type_from_rust(list_type, arena, descriptor_pool);
            Type::new_list(&list_type)
        }
        rust::ValueType::Map(map_type) => {
            let map_type = map_type_from_rust(map_type, arena, descriptor_pool);
            Type::new_map(&map_type)
        }
        rust::ValueType::Unknown => Type::new_unknown(),
        rust::ValueType::Type(type_type) => {
            let type_type = type_type_from_rust(type_type, arena, descriptor_pool);
            Type::new_type(&type_type)
        },
        rust::ValueType::Error => Type::new_error(),
        rust::ValueType::Any => Type::new_any(),
        rust::ValueType::Dyn => Type::new_dyn(),
        rust::ValueType::Opaque(opaque_type) => {
            let opaque_type = opaque_type_from_rust(opaque_type, arena, descriptor_pool);
            Type::new_opaque(&opaque_type)
        }
        rust::ValueType::BoolWrapper => Type::new_bool_wrapper(),
        rust::ValueType::IntWrapper => Type::new_int_wrapper(),
        rust::ValueType::UintWrapper => Type::new_uint_wrapper(),
        rust::ValueType::DoubleWrapper => Type::new_double_wrapper(),
        rust::ValueType::StringWrapper => Type::new_string_wrapper(),
        rust::ValueType::BytesWrapper => Type::new_bytes_wrapper(),
        rust::ValueType::Optional(optional_type) => {
            let optional_type = optional_type_from_rust(optional_type, arena, descriptor_pool);
            Type::new_optional(&optional_type)
        }
        rust::ValueType::TypeParam(type_param_type) => {
            let type_param_type = type_param_type_from_rust(type_param_type, arena);
            Type::new_type_param(&type_param_type)
        }
        rust::ValueType::Function(function_type) => {
            let function_type = function_type_from_rust(function_type, arena, descriptor_pool);
            Type::new_function(&function_type)
        }
        rust::ValueType::Enum(enum_type) => {
            let enum_type = enum_type_from_rust(enum_type, descriptor_pool);
            Type::new_enum(&enum_type)
        }
    }
}

fn map_key_type_from_rust<'a>(map_key: &rust::MapKeyType, arena: &'a Arena) -> Type<'a> {
    match map_key {
        rust::MapKeyType::Bool => Type::new_bool(),
        rust::MapKeyType::Int => Type::new_int(),
        rust::MapKeyType::Uint => Type::new_uint(),
        rust::MapKeyType::String => Type::new_string(),
        rust::MapKeyType::Dyn => Type::new_dyn(),
        rust::MapKeyType::TypeParam(type_param) => {
            let type_param_type = type_param_type_from_rust(type_param, arena);
            Type::new_type_param(&type_param_type)
        }
    }
}

fn list_type_from_rust<'a>(list: &rust::ListType, arena: &'a Arena, descriptor_pool: &'a DescriptorPool) -> ListType<'a> {
    ListType::new(arena, &type_from_rust(list.element(), arena, descriptor_pool))
}

fn map_type_from_rust<'a>(map: &rust::MapType, arena: &'a Arena, descriptor_pool: &'a DescriptorPool) -> MapType<'a> {
    MapType::new(arena, &map_key_type_from_rust(map.key(), arena), &type_from_rust(map.value(), arena, descriptor_pool))
}

fn type_type_from_rust<'a>(type_type: &rust::TypeType, arena: &'a Arena, descriptor_pool: &'a DescriptorPool) -> TypeType<'a> {
    match type_type.parameter() {
        Some(parameter) => TypeType::new(
            arena,
            &type_from_rust(parameter, arena, descriptor_pool)
        ),
        None => TypeType::default(),
    }
}

#[allow(dead_code)]
fn message_type_from_rust<'a>(message: &str, descriptor_pool: &'a DescriptorPool) -> MessageType<'a> {
    MessageType::new(descriptor_pool, message)
}

pub(crate) fn opaque_type_from_rust<'a>(
    opaque_type: &rust::OpaqueType,
    arena: &'a Arena,
    descriptor_pool: &'a DescriptorPool,
) -> OpaqueType<'a> {
    let parameters = opaque_type.parameters()
        .iter()
        .map(|p| type_from_rust(p, arena, descriptor_pool))
        .collect::<Vec<_>>();
    OpaqueType::new(arena, opaque_type.name(), &parameters)
}

fn optional_type_from_rust<'a>(
    optional_type: &rust::OptionalType,
    arena: &'a Arena,
    descriptor_pool: &'a DescriptorPool,
) -> OptionalType<'a> {
    let parameter = type_from_rust(optional_type.parameter(), arena, descriptor_pool);
    OptionalType::new(arena, &parameter)
}

fn type_param_type_from_rust<'a>(type_param_type: &rust::TypeParamType, arena: &'a Arena) -> TypeParamType<'a> {
    TypeParamType::new(type_param_type.name(), arena)
}

fn function_type_from_rust<'a>(
    function_type: &rust::FunctionType,
    arena: &'a Arena,
    descriptor_pool: &'a DescriptorPool,
) -> FunctionType<'a> {
    let result = type_from_rust(function_type.result(), arena, descriptor_pool);
    let arguments = function_type.arguments()
        .iter()
        .map(|a| type_from_rust(a, arena, descriptor_pool))
        .collect::<Vec<_>>();
    FunctionType::new(arena, &result, &arguments)
}

fn enum_type_from_rust<'a>(
    enum_type: &rust::EnumType,
    descriptor_pool: &'a DescriptorPool,
) -> EnumType<'a> {
    EnumType::new(descriptor_pool, enum_type.name())
}

pub(crate) fn type_to_rust<'a>(type_: &Type<'a>) -> rust::ValueType {
    match type_.kind() {
        TypeKind::Null => rust::ValueType::Null,
        TypeKind::Bool => rust::ValueType::Bool,
        TypeKind::Int => rust::ValueType::Int,
        TypeKind::Uint => rust::ValueType::Uint,
        TypeKind::Double => rust::ValueType::Double,
        TypeKind::String => rust::ValueType::String,
        TypeKind::Bytes => rust::ValueType::Bytes,
        TypeKind::Struct => {
            let struct_type = type_.get_struct();
            rust::ValueType::Struct(struct_type_to_rust(&struct_type))
        }
        TypeKind::Duration => rust::ValueType::Duration,
        TypeKind::Timestamp => rust::ValueType::Timestamp,
        TypeKind::List => {
            let list_type = type_.get_list();
            rust::ValueType::List(list_type_to_rust(&list_type))
        }
        TypeKind::Map => {
            let map_type = type_.get_map();
            rust::ValueType::Map(map_type_to_rust(&map_type))
        }
        TypeKind::Unknown => rust::ValueType::Unknown,
        TypeKind::Type => {
            let type_type = type_.get_type();
            rust::ValueType::Type(type_type_to_rust(&type_type))
        }
        TypeKind::Error => rust::ValueType::Error,
        TypeKind::Any => rust::ValueType::Any,
        TypeKind::Dyn => rust::ValueType::Dyn,
        TypeKind::Opaque => {
            if type_.is_optional() {
                let optional_type = type_.get_optional();
                rust::ValueType::Optional(optional_type_to_rust(&optional_type))
            } else {
                let opaque_type = type_.get_opaque();
                rust::ValueType::Opaque(opaque_type_to_rust(&opaque_type))
            }
        }
        TypeKind::BoolWrapper => rust::ValueType::BoolWrapper,
        TypeKind::IntWrapper => rust::ValueType::IntWrapper,
        TypeKind::UintWrapper => rust::ValueType::UintWrapper,
        TypeKind::DoubleWrapper => rust::ValueType::DoubleWrapper,
        TypeKind::StringWrapper => rust::ValueType::StringWrapper,
        TypeKind::BytesWrapper => rust::ValueType::BytesWrapper,
        TypeKind::TypeParam => {
            let type_param_type = type_.get_type_param();
            rust::ValueType::TypeParam(type_param_type_to_rust(&type_param_type))
        }
        TypeKind::Function => {
            let function_type = type_.get_function();
            rust::ValueType::Function(function_type_to_rust(&function_type))
        }
        TypeKind::Enum => {
            let enum_type = type_.get_enum();
            rust::ValueType::Enum(enum_type_to_rust(&enum_type))
        }
    }
}

fn mapkey_type_to_rust<'a>(type_: &Type<'a>) -> rust::MapKeyType {
    match type_.kind() {
        TypeKind::Bool => rust::MapKeyType::Bool,
        TypeKind::Int => rust::MapKeyType::Int,
        TypeKind::Uint => rust::MapKeyType::Uint,
        TypeKind::String => rust::MapKeyType::String,
        TypeKind::Dyn => rust::MapKeyType::Dyn,
        TypeKind::TypeParam => {
            let type_param_type = type_.get_type_param();
            rust::MapKeyType::TypeParam(type_param_type_to_rust(&type_param_type))
        }
        _ => rust::MapKeyType::Dyn,
    }
}

fn struct_type_to_rust<'a>(struct_type: &StructType<'a>) -> rust::StructType {
    rust::StructType::new(struct_type.name().to_string_lossy())
}

fn list_type_to_rust<'a>(list_type: &ListType<'a>) -> rust::ListType {
    rust::ListType::new(type_to_rust(&list_type.element()))
}

fn map_type_to_rust<'a>(map_type: &MapType<'a>) -> rust::MapType {
    rust::MapType::new(
        mapkey_type_to_rust(&map_type.key()),
        type_to_rust(&map_type.value())
    )
}

fn type_type_to_rust<'a>(type_type: &TypeType<'a>) -> rust::TypeType {
    if type_type.has_type() {
        rust::TypeType::new(Some(type_to_rust(&type_type.get_type())))
    } else {
        rust::TypeType::new(None)
    }
}

fn opaque_type_to_rust<'a>(opaque_type: &OpaqueType<'a>) -> rust::OpaqueType {
    let parameters = opaque_type.parameters()
        .iter()
        .map(|p| type_to_rust(&p))
        .collect::<Vec<_>>();
    rust::OpaqueType::new(opaque_type.name().to_string_lossy(), parameters)
}

fn optional_type_to_rust<'a>(optional_type: &OptionalType<'a>) -> rust::OptionalType {
    rust::OptionalType::new(type_to_rust(&optional_type.parameter()))
}

fn type_param_type_to_rust<'a>(type_param_type: &TypeParamType<'a>) -> rust::TypeParamType {
    rust::TypeParamType::new(type_param_type.name().to_string_lossy())
}

fn function_type_to_rust<'a>(function_type: &FunctionType<'a>) -> rust::FunctionType {
    rust::FunctionType::new(
        type_to_rust(function_type.result()),
        function_type.arguments()
            .iter()
            .map(|a| type_to_rust(&a))
            .collect::<Vec<_>>()
    )
}

fn enum_type_to_rust<'a>(enum_type: &EnumType<'a>) -> rust::EnumType {
    rust::EnumType::new(enum_type.name().to_string_lossy())
}
