#include <cel-cxx-ffi/include/runtime.h>
#include <cel-cxx-ffi/src/runtime.rs.h>

// 32-bit ARM __sync runtime helpers libstdc++ needs; see the header
// comment in arm_sync_shim.inc for why this include lives here.
#include "arm_sync_shim.inc"

namespace rust::cel_cxx {

absl::StatusOr<bool> AnyFfiActivationWrapper::FindVariable(
    absl::string_view name,
    const google::protobuf::DescriptorPool* absl_nonnull descriptor_pool,
    google::protobuf::MessageFactory* absl_nonnull message_factory,
    google::protobuf::Arena* absl_nonnull arena, Value* absl_nonnull result) const
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
    const cel::Function::InvokeContext& context) const
{
    Value result;
    auto status = ffi_->Invoke(
        args,
        *context.descriptor_pool(),
        *context.message_factory(),
        *context.arena(),
        result);
    if (status.ok()) {
        return result;
    }
    return status;
}

} // namespace rust::cel_cxx
