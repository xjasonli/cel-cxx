#ifndef CEL_CXX_FFI_INCLUDE_COMMON_TYPES_H_
#define CEL_CXX_FFI_INCLUDE_COMMON_TYPES_H_

#include <common/type.h>
#include <rust/cxx.h>

namespace rust::cel_cxx {

using Arena = google::protobuf::Arena;
using DescriptorPool = google::protobuf::DescriptorPool;

using Type = cel::Type;
using ListType = cel::ListType;
using MapType = cel::MapType;
using EnumType = cel::EnumType;
using FunctionType = cel::FunctionType;
using OpaqueType = cel::OpaqueType;
using OptionalType = cel::OptionalType;
using MessageType = cel::MessageType;
using StructType = cel::StructType;
using TypeParamType = cel::TypeParamType;
using TypeType = cel::TypeType;
using TypeParameters = cel::TypeParameters;

using Span_Type = absl::Span<const Type>;

inline Type Type_new_any() {
    return cel::Type(cel::AnyType());
}

inline Type Type_new_bool() {
    return cel::Type(cel::BoolType());
}

inline Type Type_new_bool_wrapper() {
    return cel::Type(cel::BoolWrapperType());
}

inline Type Type_new_bytes() {
    return cel::Type(cel::BytesType());
}

inline Type Type_new_bytes_wrapper() {
    return cel::Type(cel::BytesWrapperType());
}

inline Type Type_new_double() {
    return cel::Type(cel::DoubleType());
}

inline Type Type_new_double_wrapper() {
    return cel::Type(cel::DoubleWrapperType());
}

inline Type Type_new_duration() {
    return cel::Type(cel::DurationType());
}

inline Type Type_new_dyn() {
    return cel::Type(cel::DynType());
}

inline Type Type_new_enum(const EnumType& enum_type) {
    return cel::Type(enum_type);
}

inline Type Type_new_error() {
    return cel::Type(cel::ErrorType());
}

inline Type Type_new_function(const FunctionType& function_type) {
    return cel::Type(function_type);
}

inline Type Type_new_int() {
    return cel::Type(cel::IntType());
}

inline Type Type_new_int_wrapper() {
    return cel::Type(cel::IntWrapperType());
}

inline Type Type_new_list(const ListType& list_type) {
    return cel::Type(list_type);
}

inline Type Type_new_map(const MapType& map_type) {
    return cel::Type(map_type);
}

inline Type Type_new_message(const MessageType& message_type) {
    return cel::Type(message_type);
}

inline Type Type_new_null() {
    return cel::Type(cel::NullType());
}

inline Type Type_new_opaque(const OpaqueType& opaque_type) {
    return cel::Type(opaque_type);
}

inline Type Type_new_optional(const OptionalType& optional_type) {
    return cel::Type(optional_type);
}

inline Type Type_new_string() {
    return cel::Type(cel::StringType());
}

inline Type Type_new_string_wrapper() {
    return cel::Type(cel::StringWrapperType());
}

inline Type Type_new_struct(const StructType& struct_type) {
    return cel::Type(struct_type);
}

inline Type Type_new_timestamp() {
    return cel::Type(cel::TimestampType());
}

inline Type Type_new_type_param(const TypeParamType& type_param_type) {
    return cel::Type(type_param_type);
}

inline Type Type_new_type(const TypeType& type_type) {
    return cel::Type(type_type);
}

inline Type Type_new_uint() {
    return cel::Type(cel::UintType());
}

inline Type Type_new_uint_wrapper() {
    return cel::Type(cel::UintWrapperType());
}

inline Type Type_new_unknown() {
    return cel::Type(cel::UnknownType());
}


// ============== EnumType ==============
inline EnumType EnumType_new(const DescriptorPool& pool, Str name) {
    auto descriptor = pool.FindEnumTypeByName(std::string_view(name));
    return cel::EnumType(descriptor);
}

// ============== FunctionType ==============
inline FunctionType FunctionType_new(const Arena& arena, const Type& result, Slice<const Type> arguments) {
    auto args = absl::Span<const Type>(arguments.data(), arguments.size());
    return cel::FunctionType(&const_cast<Arena&>(arena), result, args);
}

// ============== ListType ==============
inline ListType ListType_new(const Arena& arena, const Type& element) {
    return cel::ListType(&const_cast<Arena&>(arena), element);
}

// ============== MapType ==============
inline MapType MapType_new(const Arena& arena, const Type& key, const Type& value) {
    return cel::MapType(&const_cast<Arena&>(arena), key, value);
}

// ============== MessageType ==============
inline MessageType MessageType_new(const DescriptorPool& pool, Str name) {
    auto descriptor = pool.FindMessageTypeByName(std::string_view(name));
    return cel::MessageType(descriptor);
}

// ============== OpaqueType ==============
inline OpaqueType OpaqueType_new(const Arena& arena, Str name, Slice<const Type> parameters) {
    auto name_string = std::make_unique<std::string>(name.data(), name.size());
    auto name_view = std::string_view(*name_string);
    const_cast<Arena&>(arena).Own(name_string.release());

    return cel::OpaqueType(
        &const_cast<Arena&>(arena),
        name_view,
        parameters
    );
}

// ============== OptionalType ==============
inline OptionalType OptionalType_default() {
    return cel::OptionalType();
}

inline OptionalType OptionalType_new(const Arena& arena, const Type& parameter) {
    return cel::OptionalType(&const_cast<Arena&>(arena), parameter);
}

// ============== StructType ==============
inline StructType StructType_default() {
    return cel::StructType();
}

inline StructType StructType_new_message(const MessageType& message_type) {
    return cel::StructType(message_type);
}

inline StructType StructType_new_basic(Str name) {
    auto basic_struct = cel::common_internal::MakeBasicStructType(std::string_view(name));
    return cel::StructType(basic_struct);
}

// ============== TypeParamType ==============
inline TypeParamType TypeParamType_default() {
    return cel::TypeParamType();
}

inline TypeParamType TypeParamType_new(Str name, const Arena& arena) {
    auto name_string = std::make_unique<std::string>(name.data(), name.size());
    auto name_view = std::string_view(*name_string);
    const_cast<Arena&>(arena).Own(name_string.release());

    return cel::TypeParamType(name_view);
}

inline Str TypeParamType_name(const TypeParamType& type_param_type) {
    auto name = type_param_type.name();
    return Str(name.data(), name.size());
}

// ============== TypeType ==============
inline TypeType TypeType_default() {
    return cel::TypeType();
}

inline TypeType TypeType_new(const Arena& arena, const Type& parameter) {
    return cel::TypeType(&const_cast<Arena&>(arena), parameter);
}

inline bool TypeType_has_type(const TypeType& type_type) {
    return !type_type.GetParameters().empty();
}

// ============== TypeParameters ==============
inline const Type& TypeParameters_get_unchecked(const TypeParameters& type_parameters, size_t index) {
    return type_parameters[index];
}

} // namespace cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_COMMON_TYPES_H_
