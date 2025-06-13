#include <common/value.h>
#include <cel-cxx-ffi/include/absl.h>
#include <cel-cxx-ffi/include/common/values.h>
#include "cel-cxx-ffi/src/common/values.rs.h"

namespace rust::cel_cxx {

Box<AnyFfiOpaqueValue> AnyFfiOpaqueValueWrapper::ffi() const {
    return ffi_->Clone();
}

std::string AnyFfiOpaqueValueWrapper::DebugString() const {
    return std::string(ffi_->DebugString());
}

std::string_view AnyFfiOpaqueValueWrapper::GetTypeName() const {
    return ffi_->GetTypeName();
}

OpaqueType AnyFfiOpaqueValueWrapper::GetRuntimeType() const {
    return ffi_->GetRuntimeType();
}

Status AnyFfiOpaqueValueWrapper::Equal(
    const OpaqueValue& other,
    const google::protobuf::DescriptorPool* ABSL_NONNULL descriptor_pool,
    google::protobuf::MessageFactory* ABSL_NONNULL message_factory,
    google::protobuf::Arena* ABSL_NONNULL arena,
    Value* ABSL_NONNULL result) const {
    auto other_wrapper = dynamic_cast<const AnyFfiOpaqueValueWrapper*>(other.interface());
    if (other_wrapper == nullptr) {
        *result = cel::FalseValue();
        return absl::OkStatus();
    }

    if (ffi_->Equal(*other_wrapper->ffi_)) {
        *result = cel::TrueValue();
    } else {
        *result = cel::FalseValue();
    }

    return absl::OkStatus();
}

OpaqueValue AnyFfiOpaqueValueWrapper::Clone(google::protobuf::Arena* ABSL_NONNULL arena) const {
    auto wrapper = arena->Create<AnyFfiOpaqueValueWrapper>(arena, ffi_->Clone());
    return OpaqueValue(wrapper, arena);
}

cel::NativeTypeId AnyFfiOpaqueValueWrapper::GetNativeTypeId() const {
    return cel::NativeTypeId::For<AnyFfiOpaqueValueWrapper>();
}

std::unique_ptr<OpaqueValue> OpaqueValue_new(
    const Arena& arena,
    const google::protobuf::DescriptorPool& descriptor_pool,
    Box<AnyFfiOpaqueValue> ffi)
{
    auto arena_ptr = &const_cast<Arena&>(arena);
    auto wrapper = Arena::Create<AnyFfiOpaqueValueWrapper>(
        arena_ptr, std::move(ffi));
    return std::make_unique<OpaqueValue>(
        wrapper, arena_ptr);
}

Box<AnyFfiOpaqueValue> OpaqueValue_get_ffi(const OpaqueValue& opaque_value) {
    auto wrapper = dynamic_cast<const AnyFfiOpaqueValueWrapper*>(opaque_value.interface());
    return wrapper->ffi();
}

} // namespace rust::cel_cxx
