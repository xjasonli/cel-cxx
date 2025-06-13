use crate::protobuf::{Arena, DescriptorPool};
use crate::common::TypeKind;
use crate::Rep;
use crate::absl::{Span, SpanElement, StringView};

#[cxx::bridge]
mod ffi {
    #[namespace = "google::protobuf"]
    unsafe extern "C++" {
        include!("google/protobuf/arena.h");
        type Arena = super::Arena;

        include!("google/protobuf/descriptor.h");
        type DescriptorPool = super::DescriptorPool;
    }

    #[namespace = "absl"]
    unsafe extern "C++" {
        include!("absl/strings/string_view.h");
        type string_view<'a> = super::StringView<'a>;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!("base/kind.h");
        type TypeKind = super::TypeKind;

        include!("common/type.h");
        type Type<'a> = super::Type<'a>;
        fn name<'a>(self: &Type<'a>) -> string_view<'a>;
        fn kind(self: &Type) -> TypeKind;

        #[rust_name = "is_any"]
        fn IsAny(self: &Type) -> bool;
        #[rust_name = "is_bool"]
        fn IsBool(self: &Type) -> bool;
        #[rust_name = "is_bool_wrapper"]
        fn IsBoolWrapper(self: &Type) -> bool;
        #[rust_name = "is_bytes"]
        fn IsBytes(self: &Type) -> bool;
        #[rust_name = "is_bytes_wrapper"]
        fn IsBytesWrapper(self: &Type) -> bool;
        #[rust_name = "is_double"]
        fn IsDouble(self: &Type) -> bool;
        #[rust_name = "is_double_wrapper"]
        fn IsDoubleWrapper(self: &Type) -> bool;
        #[rust_name = "is_duration"]
        fn IsDuration(self: &Type) -> bool;
        #[rust_name = "is_dyn"]
        fn IsDyn(self: &Type) -> bool;
        #[rust_name = "is_enum"]
        fn IsEnum(self: &Type) -> bool;
        #[rust_name = "is_error"]
        fn IsError(self: &Type) -> bool;
        #[rust_name = "is_function"]
        fn IsFunction(self: &Type) -> bool;
        #[rust_name = "is_int"]
        fn IsInt(self: &Type) -> bool;
        #[rust_name = "is_int_wrapper"]
        fn IsIntWrapper(self: &Type) -> bool;
        #[rust_name = "is_list"]
        fn IsList(self: &Type) -> bool;
        #[rust_name = "is_map"]
        fn IsMap(self: &Type) -> bool;
        #[rust_name = "is_message"]
        fn IsMessage(self: &Type) -> bool;
        #[rust_name = "is_null"]
        fn IsNull(self: &Type) -> bool;
        #[rust_name = "is_opaque"]
        fn IsOpaque(self: &Type) -> bool;
        #[rust_name = "is_optional"]
        fn IsOptional(self: &Type) -> bool;
        #[rust_name = "is_string"]
        fn IsString(self: &Type) -> bool;
        #[rust_name = "is_string_wrapper"]
        fn IsStringWrapper(self: &Type) -> bool;
        #[rust_name = "is_struct"]
        fn IsStruct(self: &Type) -> bool;
        #[rust_name = "is_timestamp"]
        fn IsTimestamp(self: &Type) -> bool;
        #[rust_name = "is_type_param"]
        fn IsTypeParam(self: &Type) -> bool;
        #[rust_name = "is_type"]
        fn IsType(self: &Type) -> bool;
        #[rust_name = "is_uint"]
        fn IsUint(self: &Type) -> bool;
        #[rust_name = "is_uint_wrapper"]
        fn IsUintWrapper(self: &Type) -> bool;
        #[rust_name = "is_unknown"]
        fn IsUnknown(self: &Type) -> bool;

        #[rust_name = "get_enum"]
        fn GetEnum(self: &Type) -> EnumType;
        #[rust_name = "get_function"]
        fn GetFunction(self: &Type) -> FunctionType;
        #[rust_name = "get_list"]
        fn GetList(self: &Type) -> ListType;
        #[rust_name = "get_map"]
        fn GetMap(self: &Type) -> MapType;
        #[rust_name = "get_message"]
        fn GetMessage(self: &Type) -> MessageType;
        #[rust_name = "get_opaque"]
        fn GetOpaque(self: &Type) -> OpaqueType;
        #[rust_name = "get_optional"]
        fn GetOptional(self: &Type) -> OptionalType;
        #[rust_name = "get_struct"]
        fn GetStruct(self: &Type) -> StructType;
        #[rust_name = "get_type_param"]
        fn GetTypeParam(self: &Type) -> TypeParamType;
        #[rust_name = "get_type"]
        fn GetType(self: &Type) -> TypeType;


        type EnumType<'a> = super::EnumType<'a>;
        fn name<'a>(self: &EnumType<'a>) -> string_view<'a>;

        type FunctionType<'a> = super::FunctionType<'a>;
        #[rust_name = "arguments"]
        fn args<'a>(self: &FunctionType<'a>) -> Span_Type<'a, 'a>;
        fn result<'a>(self: &FunctionType<'a>) -> &'a Type<'a>;

        type ListType<'a> = super::ListType<'a>;
        #[rust_name = "element"]
        fn GetElement(self: &ListType) -> Type;
        type MapType<'a> = super::MapType<'a>;
        #[rust_name = "key"]
        fn GetKey<'a>(self: &MapType<'a>) -> Type<'a>;
        #[rust_name = "value"]
        fn GetValue<'a>(self: &MapType<'a>) -> Type<'a>;

        type MessageType<'a> = super::MessageType<'a>;
        fn name<'a>(self: &MessageType<'a>) -> string_view<'a>;

        type OpaqueType<'a> = super::OpaqueType<'a>;
        fn name<'a>(self: &OpaqueType<'a>) -> string_view<'a>;
        #[rust_name = "parameters"]
        fn GetParameters<'a>(self: &OpaqueType<'a>) -> TypeParameters<'a>;
        #[rust_name = "is_optional"]
        fn IsOptional(self: &OpaqueType) -> bool;
        #[rust_name = "as_optional"]
        fn GetOptional<'a>(self: &OpaqueType<'a>) -> OptionalType<'a>;


        type OptionalType<'a> = super::OptionalType<'a>;
        #[rust_name = "parameter"]
        fn GetParameter<'a>(self: &OptionalType<'a>) -> Type<'a>;

        type StructType<'a> = super::StructType<'a>;
        fn name<'a>(self: &StructType<'a>) -> string_view<'a>;
        #[rust_name = "get_parameters"]
        fn GetParameters<'a>(self: &StructType<'a>) -> TypeParameters<'a>;
        #[rust_name = "is_message"]
        fn IsMessage(self: &StructType) -> bool;
        #[rust_name = "get_message"]
        fn GetMessage<'a>(self: &StructType<'a>) -> MessageType<'a>;

        type TypeParamType<'a> = super::TypeParamType<'a>;
        fn name<'a>(self: &TypeParamType<'a>) -> string_view<'a>;

        type TypeType<'a> = super::TypeType<'a>;
        #[rust_name = "get_type"]
        fn GetType<'a>(self: &TypeType<'a>) -> Type<'a>;

        type TypeParameters<'a> = super::TypeParameters<'a>;
        #[rust_name = "len"]
        fn size(self: &TypeParameters) -> usize;
        #[rust_name = "is_empty"]
        fn empty(self: &TypeParameters) -> bool;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!("cel-cxx-ffi/include/common/types.h");
        type Span_Type<'s, 'a> = super::Span<'s, Type<'a>>;

        fn Type_new_any() -> Type<'static>;
        fn Type_new_bool() -> Type<'static>;
        fn Type_new_bool_wrapper() -> Type<'static>;
        fn Type_new_bytes() -> Type<'static>;
        fn Type_new_bytes_wrapper() -> Type<'static>;
        fn Type_new_double() -> Type<'static>;
        fn Type_new_double_wrapper() -> Type<'static>;
        fn Type_new_duration() -> Type<'static>;
        fn Type_new_dyn() -> Type<'static>;
        fn Type_new_enum<'a>(enum_type: &EnumType<'a>) -> Type<'a>;
        fn Type_new_error() -> Type<'static>;
        fn Type_new_function<'a>(function_type: &FunctionType<'a>) -> Type<'static>;
        fn Type_new_int() -> Type<'static>;
        fn Type_new_int_wrapper() -> Type<'static>;
        fn Type_new_list<'a>(list_type: &ListType<'a>) -> Type<'a>;
        fn Type_new_map<'a>(map_type: &MapType<'a>) -> Type<'a>;
        fn Type_new_message<'a>(message_type: &MessageType<'a>) -> Type<'a>;
        fn Type_new_null() -> Type<'static>;
        fn Type_new_opaque<'a>(opaque_type: &OpaqueType<'a>) -> Type<'a>;
        fn Type_new_optional<'a>(optional_type: &OptionalType<'a>) -> Type<'a>;
        fn Type_new_string() -> Type<'static>;
        fn Type_new_string_wrapper() -> Type<'static>;
        fn Type_new_struct<'a>(struct_type: &StructType<'a>) -> Type<'a>;
        fn Type_new_timestamp() -> Type<'static>;
        fn Type_new_type_param<'a>(type_param_type: &TypeParamType<'a>) -> Type<'a>;
        fn Type_new_type<'a>(type_type: &TypeType<'a>) -> Type<'a>;
        fn Type_new_uint() -> Type<'static>;
        fn Type_new_uint_wrapper() -> Type<'static>;
        fn Type_new_unknown() -> Type<'static>;

        // EnumType
        fn EnumType_new<'a>(
            descriptor_pool: &'a DescriptorPool,
            name: &str,
        ) -> EnumType<'a>;

        // FunctionType
        fn FunctionType_new<'a>(
            arena: &'a Arena,
            result: &Type<'a>,
            arguments: &[Type<'a>],
        ) -> FunctionType<'a>;

        // ListType
        fn ListType_new<'a>(
            arena: &'a Arena,
            element: &Type<'a>,
        ) -> ListType<'a>;

        // MapType
        fn MapType_new<'a>(
            arena: &'a Arena,
            key: &Type<'a>,
            value: &Type<'a>,
        ) -> MapType<'a>;

        // MessageType
        fn MessageType_new<'a>(
            pool: &'a DescriptorPool,
            name: &str,
        ) -> MessageType<'a>;

        // OpaqueType
        fn OpaqueType_new<'a>(
            arena: &'a Arena,
            name: &str,
            parameters: &[Type<'a>],
        ) -> OpaqueType<'a>;

        // OptionalType
        fn OptionalType_default() -> OptionalType<'static>;
        fn OptionalType_new<'a>(
            arena: &'a Arena,
            parameter: &Type<'a>,
        ) -> OptionalType<'a>;

        // StructType
        fn StructType_default() -> StructType<'static>;
        fn StructType_new_message<'a>(
            message_type: &MessageType<'a>,
        ) -> StructType<'a>;
        fn StructType_new_basic<'a>(
            name: &'a str,
        ) -> StructType<'a>;

        // TypeParamType
        fn TypeParamType_default() -> TypeParamType<'static>;
        fn TypeParamType_new<'a>(name: &str, arena: &'a Arena) -> TypeParamType<'a>;

        // TypeType
        fn TypeType_default() -> TypeType<'static>;
        fn TypeType_new<'a>(
            arena: &'a Arena,
            parameter: &Type<'a>,
        ) -> TypeType<'a>;
        fn TypeType_has_type(type_type: &TypeType) -> bool;

        // TypeParameters
        unsafe fn TypeParameters_get_unchecked<'a>(type_parameters: &TypeParameters<'a>, index: usize) -> &'a Type<'a>;
    }
}

// cel::Type is a variant of all subtypes.
// The largest type is BasicStructType, TypeParamType which contains a
// std::string_view.
//
// So we use 2 pointers size to store the type.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Type<'a>(Rep<'a, usize, 3>);

unsafe impl<'a> cxx::ExternType for Type<'a> {
    type Id = cxx::type_id!("cel::Type");
    type Kind = cxx::kind::Trivial;
}

impl<'a> crate::SizedExternType for Type<'a> {}

impl<'a> SpanElement for Type<'a> {
    type TypeId = cxx::type_id!("rust::cel_cxx::Span_Type");
}

impl<'a> Type<'a> {
    pub fn new_any() -> Self {
        ffi::Type_new_any()
    }
    pub fn new_bool() -> Self {
        ffi::Type_new_bool()
    }
    pub fn new_bool_wrapper() -> Self {
        ffi::Type_new_bool_wrapper()
    }
    pub fn new_bytes() -> Self {
        ffi::Type_new_bytes()
    }
    pub fn new_bytes_wrapper() -> Self {
        ffi::Type_new_bytes_wrapper()
    }
    pub fn new_double() -> Self {
        ffi::Type_new_double()
    }
    pub fn new_double_wrapper() -> Self {
        ffi::Type_new_double_wrapper()
    }
    pub fn new_duration() -> Self {
        ffi::Type_new_duration()
    }
    pub fn new_dyn() -> Self {
        ffi::Type_new_dyn()
    }
    pub fn new_enum(enum_type: &EnumType<'a>) -> Self {
        ffi::Type_new_enum(enum_type)
    }
    pub fn new_error() -> Self {
        ffi::Type_new_error()
    }
    pub fn new_function(function_type: &FunctionType<'a>) -> Self {
        ffi::Type_new_function(function_type)
    }
    pub fn new_int() -> Self {
        ffi::Type_new_int()
    }
    pub fn new_int_wrapper() -> Self {
        ffi::Type_new_int_wrapper()
    }
    pub fn new_list(list_type: &ListType<'a>) -> Self {
        ffi::Type_new_list(list_type)
    }
    pub fn new_map(map_type: &MapType<'a>) -> Self {
        ffi::Type_new_map(map_type)
    }
    pub fn new_message(message_type: &MessageType<'a>) -> Self {
        ffi::Type_new_message(message_type)
    }
    pub fn new_null() -> Self {
        ffi::Type_new_null()
    }
    pub fn new_opaque(opaque_type: &OpaqueType<'a>) -> Self {
        ffi::Type_new_opaque(opaque_type)
    }
    pub fn new_optional(optional_type: &OptionalType<'a>) -> Self {
        ffi::Type_new_optional(optional_type)
    }
    pub fn new_string() -> Self {
        ffi::Type_new_string()
    }
    pub fn new_string_wrapper() -> Self {
        ffi::Type_new_string_wrapper()
    }
    pub fn new_struct(struct_type: &StructType<'a>) -> Self {
        ffi::Type_new_struct(struct_type)
    }
    pub fn new_timestamp() -> Self {
        ffi::Type_new_timestamp()
    }
    pub fn new_type_param(type_param_type: &TypeParamType<'a>) -> Self {
        ffi::Type_new_type_param(type_param_type)
    }
    pub fn new_type(type_type: &TypeType<'a>) -> Self {
        ffi::Type_new_type(type_type)
    }
    pub fn new_uint() -> Self {
        ffi::Type_new_uint()
    }
    pub fn new_uint_wrapper() -> Self {
        ffi::Type_new_uint_wrapper()
    }
    pub fn new_unknown() -> Self {
        ffi::Type_new_unknown()
    }
}

// EnumType stores a pointer to the descriptor.
//
// So we use 1 pointer size to store the type.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EnumType<'a>(Rep<'a, usize, 1>);

unsafe impl<'a> cxx::ExternType for EnumType<'a> {
    type Id = cxx::type_id!("cel::EnumType");
    type Kind = cxx::kind::Trivial;
}

impl<'a> EnumType<'a> {
    pub fn new(descriptor_pool: &'a DescriptorPool, name: &str) -> Self {
        ffi::EnumType_new(descriptor_pool, name)
    }
}

// FunctionType stores a pointer.
//
// So we use 1 pointer size to store the type.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct FunctionType<'a>(Rep<'a, usize, 1>);

unsafe impl<'a> cxx::ExternType for FunctionType<'a> {
    type Id = cxx::type_id!("cel::FunctionType");
    type Kind = cxx::kind::Trivial;
}


impl<'a> FunctionType<'a> {
    pub fn new(arena: &'a Arena, result: &Type<'a>, arguments: &[Type<'a>]) -> Self {
        ffi::FunctionType_new(arena, result, arguments)
    }
}

// ListType stores a `uintptr_t`.
//
// So we use 1 pointer size to store the type.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ListType<'a>(Rep<'a, usize, 1>);

unsafe impl<'a> cxx::ExternType for ListType<'a> {
    type Id = cxx::type_id!("cel::ListType");
    type Kind = cxx::kind::Trivial;
}

impl<'a> ListType<'a> {
    pub fn new(arena: &'a Arena, element: &Type<'a>) -> Self {
        ffi::ListType_new(arena, element)
    }
}

// MapType stores a `uintptr_t`.
//
// So we use 1 pointer size to store the type.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MapType<'a>(Rep<'a, usize, 1>);

unsafe impl<'a> cxx::ExternType for MapType<'a> {
    type Id = cxx::type_id!("cel::MapType");
    type Kind = cxx::kind::Trivial;
}

impl<'a> MapType<'a> {
    pub fn new(arena: &'a Arena, key: &Type<'a>, value: &Type<'a>) -> Self {
        ffi::MapType_new(arena, key, value)
    }
}


// MessageType stores a pointer to the descriptor.
//
// So we use 1 pointer size to store the type.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MessageType<'a>(Rep<'a, usize, 1>);

unsafe impl<'a> cxx::ExternType for MessageType<'a> {
    type Id = cxx::type_id!("cel::MessageType");
    type Kind = cxx::kind::Trivial;
}

impl<'a> MessageType<'a> {
    pub fn new(pool: &'a DescriptorPool, name: &str) -> Self {
        ffi::MessageType_new(pool, name)
    }
}


// OpaqueType stores a pointer to the data.
//
// So we use 1 pointer size to store the type.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct OpaqueType<'a>(Rep<'a, usize, 1>);

unsafe impl<'a> cxx::ExternType for OpaqueType<'a> {
    type Id = cxx::type_id!("cel::OpaqueType");
    type Kind = cxx::kind::Trivial;
}

impl<'a> OpaqueType<'a> {
    pub fn new(arena: &'a Arena, name: &str, parameters: &[Type<'a>]) -> Self {
        ffi::OpaqueType_new(arena, name, parameters)
    }
}


// OptionalType is a wrapper on OpaqueType.
// 
// So we use 1 pointer size to store the type.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct OptionalType<'a>(Rep<'a, usize, 1>);

unsafe impl<'a> cxx::ExternType for OptionalType<'a> {
    type Id = cxx::type_id!("cel::OptionalType");
    type Kind = cxx::kind::Trivial;
}

impl Default for OptionalType<'static> {
    fn default() -> Self {
        ffi::OptionalType_default()
    }
}

impl<'a> OptionalType<'a> {
    pub fn new(arena: &'a Arena, parameter: &Type<'a>) -> Self {
        ffi::OptionalType_new(arena, parameter)
    }
}


// StructType contains a StructTypeVariant, which largest member is
// BasicStructType contains a std::string_view.
//
// So we use 2 pointers size to store the type.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StructType<'a>(Rep<'a, usize, 2>);

unsafe impl<'a> cxx::ExternType for StructType<'a> {
    type Id = cxx::type_id!("cel::StructType");
    type Kind = cxx::kind::Trivial;
}

impl Default for StructType<'static> {
    fn default() -> Self {
        ffi::StructType_default()
    }
}

impl<'a> StructType<'a> {
    pub fn new_message(message_type: &MessageType<'a>) -> Self {
        ffi::StructType_new_message(message_type)
    }
    pub fn new_basic(name: &'a str) -> Self {
        ffi::StructType_new_basic(name)
    }
}


// TypeParamType stores a std::string_view.
//
// So we use 1 pointer size to store the type.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct TypeParamType<'a>(Rep<'a, usize, 1>);

unsafe impl<'a> cxx::ExternType for TypeParamType<'a> {
    type Id = cxx::type_id!("cel::TypeParamType");
    type Kind = cxx::kind::Trivial;
}

impl Default for TypeParamType<'static> {
    fn default() -> Self {
        ffi::TypeParamType_default()
    }
}

impl<'a> TypeParamType<'a> {
    pub fn new(name: &str, arena: &'a Arena) -> Self {
        ffi::TypeParamType_new(name, arena)
    }
}

// TypeType stores a pointer to the data.
//
// So we use 1 pointer size to store the type.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct TypeType<'a>(Rep<'a, usize, 1>);

unsafe impl<'a> cxx::ExternType for TypeType<'a> {
    type Id = cxx::type_id!("cel::TypeType");
    type Kind = cxx::kind::Trivial;
}

impl Default for TypeType<'static> {
    fn default() -> Self {
        ffi::TypeType_default()
    }
}

impl<'a> TypeType<'a> {
    pub fn new(arena: &'a Arena, parameter: &Type<'a>) -> Self {
        ffi::TypeType_new(arena, parameter)
    }
    pub fn has_type(&self) -> bool {
        ffi::TypeType_has_type(self)
    }
}


// TypeParameters stores a size_t and a array of two `Type`s.
//
// So we use 5 pointers size to store the type.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct TypeParameters<'a>(Rep<'a, usize, 5>);

unsafe impl<'a> cxx::ExternType for TypeParameters<'a> {
    type Id = cxx::type_id!("cel::TypeParameters");
    type Kind = cxx::kind::Trivial;
}

impl<'a> TypeParameters<'a> {
    pub fn iter(&self) -> TypeParametersIter<'a> {
        TypeParametersIter {
            type_parameters: *self,
            index: 0,
        }
    }

    pub fn get(&self, index: usize) -> Option<&Type<'a>> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &Type<'a> {
        unsafe { ffi::TypeParameters_get_unchecked(self, index) }
    }
}

impl<'a> std::iter::IntoIterator for TypeParameters<'a> {
    type Item = Type<'a>;
    type IntoIter = TypeParametersIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> std::ops::Index<usize> for TypeParameters<'a> {
    type Output = Type<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.get_unchecked(index) }
    }
}

pub struct TypeParametersIter<'a> {
    type_parameters: TypeParameters<'a>,
    index: usize,
}

impl<'a> std::iter::Iterator for TypeParametersIter<'a> {
    type Item = Type<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.type_parameters.len() {
            let item = self.type_parameters[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.type_parameters.len() - self.index, Some(self.type_parameters.len() - self.index))
    }
}

impl<'a> std::iter::ExactSizeIterator for TypeParametersIter<'a> {
    fn len(&self) -> usize {
        self.type_parameters.len() - self.index
    }
}

impl<'a> std::iter::FusedIterator for TypeParametersIter<'a> {}
