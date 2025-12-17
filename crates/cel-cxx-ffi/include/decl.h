#ifndef CEL_CXX_FFI_INCLUDE_COMMON_DECL_H_
#define CEL_CXX_FFI_INCLUDE_COMMON_DECL_H_

#include <rust/cxx.h>
#include <common/decl.h>

namespace rust::cel_cxx {

using Duration = absl::Duration;
using Timestamp = absl::Time;
using Type = cel::Type;
using VariableDecl = cel::VariableDecl;
using FunctionDecl = cel::FunctionDecl;
using OverloadDecl = cel::OverloadDecl;
using Constant = cel::Constant;

// VariableDecl
inline std::unique_ptr<VariableDecl> VariableDecl_new(Str name, const Type& type) {
    return std::make_unique<VariableDecl>(MakeVariableDecl(std::string_view(name.data(), name.size()), type));
}

inline std::unique_ptr<VariableDecl> VariableDecl_new_constant(Str name, const Constant& value) {
    Type type;
    if (value.has_null_value()) {
        type = Type(cel::NullType());
    } else if (value.has_bool_value()) {
        type = Type(cel::BoolType());
    } else if (value.has_int_value()) {
        type = Type(cel::IntType());
    } else if (value.has_uint_value()) {
        type = Type(cel::UintType());
    } else if (value.has_double_value()) {
        type = Type(cel::DoubleType());
    } else if (value.has_bytes_value()) {
        type = Type(cel::BytesType());
    } else if (value.has_string_value()) {
        type = Type(cel::StringType());
    } else if (value.has_duration_value()) {
        type = Type(cel::DurationType());
    } else if (value.has_timestamp_value()) {
        type = Type(cel::TimestampType());
    } else {
        return nullptr;
    }
    return std::make_unique<VariableDecl>(MakeConstantVariableDecl(std::string(name), std::move(type), value));
}

// FunctionDecl
inline std::unique_ptr<FunctionDecl> FunctionDecl_new(Str name) {
    auto function = std::make_unique<FunctionDecl>();
    function->set_name(std::string(name));
    return function;
}

// OverloadDecl
inline std::unique_ptr<OverloadDecl> OverloadDecl_new(Str id, bool member, const Type& result, Slice<const Type> args) {
    auto overload = std::make_unique<OverloadDecl>();
    overload->set_id(std::string(id));
    overload->set_member(member);
    overload->set_result(result);
    overload->mutable_args().reserve(args.size());
    for (const auto& arg : args) {
        overload->mutable_args().push_back(arg);
    }
    return overload;
}

} // namespace rust::cel_cxx

#endif // CEL_CXX_FFI_INCLUDE_COMMON_DECL_H_
