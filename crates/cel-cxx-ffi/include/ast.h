#ifndef CEL_CXX_FFI_INCLUDE_COMMON_AST_H_
#define CEL_CXX_FFI_INCLUDE_COMMON_AST_H_

#include <rust/cxx.h>
#include <common/ast.h>
#include <common/type.h>

namespace rust::cel_cxx {

using Arena = google::protobuf::Arena;
using DescriptorPool = google::protobuf::DescriptorPool;
using Type = cel::Type;

using Ast = cel::Ast;
using TypeSpec = cel::TypeSpec;
using PrimitiveType = cel::PrimitiveType;
using WellKnownTypeSpec = cel::WellKnownTypeSpec;

inline Type TypeSpec_to_type(
    const TypeSpec& type_spec,
    const DescriptorPool& const_descriptor_pool,
    const Arena& const_arena)
{
    auto& arena = const_cast<Arena&>(const_arena);
    auto& descriptor_pool = const_cast<DescriptorPool&>(const_descriptor_pool);

    if (type_spec.has_dyn()) {
        return Type(cel::DynType());
    } else if (type_spec.has_null()) {
        return Type(cel::NullType());
    } else if (type_spec.has_primitive()) {
        switch (type_spec.primitive()) {
            case PrimitiveType::kPrimitiveTypeUnspecified:
                return Type(cel::DynType());
            case PrimitiveType::kBool:
                return Type(cel::BoolType());
            case PrimitiveType::kInt64:
                return Type(cel::IntType());
            case PrimitiveType::kUint64:
                return Type(cel::UintType());
            case PrimitiveType::kDouble:
                return Type(cel::DoubleType());
            case PrimitiveType::kString:
                return Type(cel::StringType());
            case PrimitiveType::kBytes:
                return Type(cel::BytesType());
            default:
                return Type(cel::DynType());
        }
    } else if (type_spec.has_wrapper()) {
        switch (type_spec.wrapper()) {
            case PrimitiveType::kPrimitiveTypeUnspecified:
                return Type(cel::DynType());
            case PrimitiveType::kBool:
                return Type(cel::BoolWrapperType());
            case PrimitiveType::kInt64:
                return Type(cel::IntWrapperType());
            case PrimitiveType::kUint64:
                return Type(cel::UintWrapperType());
            case PrimitiveType::kDouble:
                return Type(cel::DoubleWrapperType());
            case PrimitiveType::kString:
                return Type(cel::StringWrapperType());
            case PrimitiveType::kBytes:
                return Type(cel::BytesWrapperType());
            default:
                return Type(cel::DynType());
        }
    } else if (type_spec.has_well_known()) {
        switch (type_spec.well_known()) {
            case WellKnownTypeSpec::kWellKnownTypeUnspecified:
                return Type(cel::DynType());
            case WellKnownTypeSpec::kAny:
                return Type(cel::AnyType());
            case WellKnownTypeSpec::kTimestamp:
                return Type(cel::TimestampType());
            case WellKnownTypeSpec::kDuration:
                return Type(cel::DurationType());
            default:
                return Type(cel::DynType());
        }
    } else if (type_spec.has_list_type()) {
        auto list_type = type_spec.list_type();
        auto elem_type = TypeSpec_to_type(list_type.elem_type(), const_descriptor_pool, const_arena);
        return Type(cel::ListType(&arena, elem_type));
    } else if (type_spec.has_map_type()) {
        auto map_type = type_spec.map_type();
        auto key_type = TypeSpec_to_type(map_type.key_type(), const_descriptor_pool, const_arena);
        auto value_type = TypeSpec_to_type(map_type.value_type(), const_descriptor_pool, const_arena);
        return Type(cel::MapType(&arena, key_type, value_type));
    } else if (type_spec.has_function()) {
        auto function_type = type_spec.function();
        auto result_type = TypeSpec_to_type(function_type.result_type(), const_descriptor_pool, const_arena);
        auto type_spec_arg_types = function_type.arg_types();
        std::vector<Type> arg_types_vec;
        for (const auto& type_spec_arg_type : type_spec_arg_types) {
            arg_types_vec.push_back(TypeSpec_to_type(type_spec_arg_type, const_descriptor_pool, const_arena));
        }
        return Type(cel::FunctionType(&arena, result_type, arg_types_vec));
    } else if (type_spec.has_message_type()) {
        auto message_type = type_spec.message_type();
        auto message_name = message_type.type();
        auto message_descriptor = descriptor_pool.FindMessageTypeByName(message_name);
        if (!message_descriptor) {
            return Type(cel::DynType());
        }
        return Type(cel::MessageType(message_descriptor));
    } else if (type_spec.has_type_param()) {
        auto& type_param = type_spec.type_param();
        auto type_param_name = new std::string(type_param.type());
        arena.Own(type_param_name);
        return Type(cel::TypeParamType(*type_param_name));
    } else if (type_spec.has_type()) {
        auto& type_spec_inner = type_spec.type();
        auto type_inner = TypeSpec_to_type(type_spec_inner, const_descriptor_pool, const_arena);
        return Type(type_inner);
    } else if (type_spec.has_error()) {
        return Type(cel::ErrorType());
    } else if (type_spec.has_abstract_type()) {
        auto& abstract_type = type_spec.abstract_type();
        auto name = new std::string(abstract_type.name());
        auto& parameter_types = abstract_type.parameter_types();
        std::vector<Type> parameter_types_vec;
        for (const auto& parameter_type : parameter_types) {
            parameter_types_vec.push_back(TypeSpec_to_type(parameter_type, const_descriptor_pool, const_arena));
        }
        arena.Own(name);
        return Type(cel::OpaqueType(&arena, *name, parameter_types_vec));
    } else {
        return Type(cel::DynType());
    }
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_COMMON_AST_H_
