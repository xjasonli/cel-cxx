#ifndef CEL_CXX_FFI_INCLUDE_COMMON_AST_H_
#define CEL_CXX_FFI_INCLUDE_COMMON_AST_H_

#include <rust/cxx.h>
#include <common/ast.h>
#include <common/ast/ast_impl.h>
#include <common/type.h>

namespace rust::cel_cxx {

using Arena = google::protobuf::Arena;
using DescriptorPool = google::protobuf::DescriptorPool;
using Type = cel::Type;

using Ast = cel::Ast;
using AstImpl = cel::ast_internal::AstImpl;
using AstType = cel::ast_internal::Type;
using AstPrimitiveType = cel::ast_internal::PrimitiveType;
using AstWellKnownType = cel::ast_internal::WellKnownType;

inline const AstImpl& AstImpl_cast_from_public_ast(const Ast& ast) {
    return AstImpl::CastFromPublicAst(ast);
}

inline Type AstType_to_type(
    const AstType& ast_type,
    const DescriptorPool& const_descriptor_pool,
    const Arena& const_arena)
{
    auto& arena = const_cast<Arena&>(const_arena);
    auto& descriptor_pool = const_cast<DescriptorPool&>(const_descriptor_pool);

    if (ast_type.has_dyn()) {
        return Type(cel::DynType());
    } else if (ast_type.has_null()) {
        return Type(cel::NullType());
    } else if (ast_type.has_primitive()) {
        switch (ast_type.primitive()) {
            case AstPrimitiveType::kPrimitiveTypeUnspecified:
                return Type(cel::DynType());
            case AstPrimitiveType::kBool:
                return Type(cel::BoolType());
            case AstPrimitiveType::kInt64:
                return Type(cel::IntType());
            case AstPrimitiveType::kUint64:
                return Type(cel::UintType());
            case AstPrimitiveType::kDouble:
                return Type(cel::DoubleType());
            case AstPrimitiveType::kString:
                return Type(cel::StringType());
            case AstPrimitiveType::kBytes:
                return Type(cel::BytesType());
            default:
                return Type(cel::DynType());
        }
    } else if (ast_type.has_wrapper()) {
        switch (ast_type.wrapper()) {
            case AstPrimitiveType::kPrimitiveTypeUnspecified:
                return Type(cel::DynType());
            case AstPrimitiveType::kBool:
                return Type(cel::BoolWrapperType());
            case AstPrimitiveType::kInt64:
                return Type(cel::IntWrapperType());
            case AstPrimitiveType::kUint64:
                return Type(cel::UintWrapperType());
            case AstPrimitiveType::kDouble:
                return Type(cel::DoubleWrapperType());
            case AstPrimitiveType::kString:
                return Type(cel::StringWrapperType());
            case AstPrimitiveType::kBytes:
                return Type(cel::BytesWrapperType());
            default:
                return Type(cel::DynType());
        }
    } else if (ast_type.has_well_known()) {
        switch (ast_type.well_known()) {
            case AstWellKnownType::kWellKnownTypeUnspecified:
                return Type(cel::DynType());
            case AstWellKnownType::kAny:
                return Type(cel::AnyType());
            case AstWellKnownType::kTimestamp:
                return Type(cel::TimestampType());
            case AstWellKnownType::kDuration:
                return Type(cel::DurationType());
            default:
                return Type(cel::DynType());
        }
    } else if (ast_type.has_list_type()) {
        auto list_type = ast_type.list_type();
        auto elem_type = AstType_to_type(list_type.elem_type(), const_descriptor_pool, const_arena);
        return Type(cel::ListType(&arena, elem_type));
    } else if (ast_type.has_map_type()) {
        auto map_type = ast_type.map_type();
        auto key_type = AstType_to_type(map_type.key_type(), const_descriptor_pool, const_arena);
        auto value_type = AstType_to_type(map_type.value_type(), const_descriptor_pool, const_arena);
        return Type(cel::MapType(&arena, key_type, value_type));
    } else if (ast_type.has_function()) {
        auto function_type = ast_type.function();
        auto result_type = AstType_to_type(function_type.result_type(), const_descriptor_pool, const_arena);
        auto ast_arg_types = function_type.arg_types();
        std::vector<Type> arg_types_vec;
        for (const auto& ast_arg_type : ast_arg_types) {
            arg_types_vec.push_back(AstType_to_type(ast_arg_type, const_descriptor_pool, const_arena));
        }
        return Type(cel::FunctionType(&arena, result_type, arg_types_vec));
    } else if (ast_type.has_message_type()) {
        auto message_type = ast_type.message_type();
        auto message_name = message_type.type();
        auto message_descriptor = descriptor_pool.FindMessageTypeByName(message_name);
        if (!message_descriptor) {
            return Type(cel::DynType());
        }
        return Type(cel::MessageType(message_descriptor));
    } else if (ast_type.has_type_param()) {
        auto& type_param = ast_type.type_param();
        auto type_param_name = new std::string(type_param.type());
        arena.Own(type_param_name);
        return Type(cel::TypeParamType(*type_param_name));
    } else if (ast_type.has_type()) {
        auto& ast_type_inner = ast_type.type();
        auto type_inner = AstType_to_type(ast_type_inner, const_descriptor_pool, const_arena);
        return Type(type_inner);
    } else if (ast_type.has_error()) {
        return Type(cel::ErrorType());
    } else if (ast_type.has_abstract_type()) {
        auto& abstract_type = ast_type.abstract_type();
        auto name = new std::string(abstract_type.name());
        auto& parameter_types = abstract_type.parameter_types();
        std::vector<Type> parameter_types_vec;
        for (const auto& parameter_type : parameter_types) {
            parameter_types_vec.push_back(AstType_to_type(parameter_type, const_descriptor_pool, const_arena));
        }
        arena.Own(name);
        return Type(cel::OpaqueType(&arena, *name, parameter_types_vec));
    } else {
        return Type(cel::DynType());
    }
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_COMMON_AST_H_
