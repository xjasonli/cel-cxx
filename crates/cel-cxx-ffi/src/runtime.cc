#include <cel-cxx-ffi/include/runtime.h>
#include <cel-cxx-ffi/src/runtime.rs.h>

namespace rust::cel_cxx {

absl::StatusOr<bool> AnyFfiActivationWrapper::FindVariable(
    absl::string_view name,
    const google::protobuf::DescriptorPool* ABSL_NONNULL descriptor_pool,
    google::protobuf::MessageFactory* ABSL_NONNULL message_factory,
    google::protobuf::Arena* ABSL_NONNULL arena, Value* ABSL_NONNULL result) const
{
    std::unique_ptr<Value> result_ptr;
    auto status = ffi_->FindVariable(
        rust::Str(name.data(), name.size()),
        *descriptor_pool,
        *message_factory,
        *arena,
        result_ptr);
    if (!status.ok()) {
        return status;
    }
    if (result_ptr) {
        *result = std::move(*result_ptr);
        return true;
    }
    return false;
}

std::vector<FunctionOverloadReference> AnyFfiActivationWrapper::FindFunctionOverloads(
    absl::string_view name) const
{
    auto functions = ffi_->FindFunctionOverloads(rust::Str(name.data(), name.size()));
    std::vector<FunctionOverloadReference> result;
    for (const auto& function : functions) {
        result.push_back(function);
    }
    return result;
}

absl::StatusOr<Value> AnyFfiFunctionWrapper::Invoke(
    absl::Span<const Value> args,
    const google::protobuf::DescriptorPool* ABSL_NONNULL descriptor_pool,
    google::protobuf::MessageFactory* ABSL_NONNULL message_factory,
    google::protobuf::Arena* ABSL_NONNULL arena,
    absl::Span<const std::string> overload_id) const
{
    Value result;
    auto status = ffi_->Invoke(
        args,
        *descriptor_pool,
        *message_factory,
        *arena,
        overload_id,
        result);
    if (status.ok()) {
        return result;
    }
    return status;
}

} // namespace rust::cel_cxx
