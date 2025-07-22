#include <absl/base/no_destructor.h>
#include <checker/internal/builtins_arena.h>
#include <compiler/compiler.h>
#include <internal/status_macros.h>
#include <common/type.h>
#include <common/decl.h>
#include <base/function_adapter.h>
#include <runtime/internal/errors.h>
#include <cel-cxx-ffi/include/optional.h>

namespace rust::cel_cxx {
// OptionalCheckerLibrary
using Type = cel::Type;
using OptionalType = cel::OptionalType;
using ListType = cel::ListType;
using TypeParamType = cel::TypeParamType;
using cel::checker_internal::BuiltinsArena;

Type OptionalOfV() {
  static const absl::NoDestructor<OptionalType> kInstance(
      BuiltinsArena(), TypeParamType("V"));

  return *kInstance;
}
Type ListOfV() {
  static const absl::NoDestructor<ListType> kInstance(
      BuiltinsArena(), TypeParamType("V"));

  return *kInstance;
}
Type ListOfOptionalV() {
  static const absl::NoDestructor<ListType> kInstance(
      BuiltinsArena(), OptionalOfV());

  return *kInstance;
}

cel::TypeCheckerBuilderConfigurer PatchOptionalTypeCheckerBuilderConfigurer(cel::TypeCheckerBuilderConfigurer origin) {
    return [origin=std::move(origin)](cel::TypeCheckerBuilder& builder) -> absl::Status {
        CEL_ASSIGN_OR_RETURN(
            auto unwrap,
            MakeFunctionDecl(
                "optional.unwrap",
                MakeOverloadDecl("optional_unwrap_list", ListOfV(), ListOfOptionalV())));
        CEL_ASSIGN_OR_RETURN(
            auto unwrap_opt,
            MakeFunctionDecl(
                "unwrapOpt",
                MakeMemberOverloadDecl("optional_unwrapOpt_list", ListOfV(), ListOfOptionalV())));

        CEL_RETURN_IF_ERROR(builder.AddFunction(std::move(unwrap)));
        CEL_RETURN_IF_ERROR(builder.AddFunction(std::move(unwrap_opt)));

        return origin(builder);
    };
}

cel::CheckerLibrary PatchOptionalCheckerLibrary(cel::CheckerLibrary origin) {
    cel::CheckerLibrary checker_library = {
        .id = std::move(origin.id),
        .configure = std::move(PatchOptionalTypeCheckerBuilderConfigurer(std::move(origin.configure))),
    };
    return checker_library;
}

cel::CompilerLibrary PatchOptionalCompilerLibrary(cel::CompilerLibrary origin) {
    return cel::CompilerLibrary(
        std::move(origin.id), std::move(origin.configure_parser),
        PatchOptionalTypeCheckerBuilderConfigurer(std::move(origin.configure_checker)));
}

absl::StatusOr<cel::Value> OptionalOrValue(
    const cel::OpaqueValue& opaque_value, const cel::Value& value,
    const google::protobuf::DescriptorPool* descriptor_pool,
    google::protobuf::MessageFactory* message_factory,
    google::protobuf::Arena* arena) {
    if (auto optional_value = opaque_value.AsOptional(); optional_value) {
        if (optional_value->HasValue()) {
            return optional_value->Value();
        } else {
            return value;
        }
    }
    return cel::ErrorValue{
        cel::runtime_internal::CreateNoMatchingOverloadError("orValue")};
}

absl::Status PatchOptionalRuntimeBuilder(cel::RuntimeBuilder& builder) {
    auto& registry = builder.function_registry();
    CEL_RETURN_IF_ERROR(registry.Register(
        cel::BinaryFunctionAdapter<absl::StatusOr<cel::Value>,
            cel::OpaqueValue, cel::Value>::CreateDescriptor("orValue", true),
        cel::BinaryFunctionAdapter<absl::StatusOr<cel::Value>,
            cel::OpaqueValue, cel::Value>::WrapFunction(&OptionalOrValue)));

    return absl::OkStatus();
}

absl::Status EnableOptionalTypes(cel::RuntimeBuilder& builder) {
    CEL_RETURN_IF_ERROR(cel::extensions::EnableOptionalTypes(builder));
    CEL_RETURN_IF_ERROR(PatchOptionalRuntimeBuilder(builder));
    return absl::OkStatus();
}


} // namespace rust::cel_cxx
