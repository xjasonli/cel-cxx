#include <common/value.h>
#include <cel-cxx-ffi/include/absl.h>
#include <cel-cxx-ffi/include/values.h>
#include <cel-cxx-ffi/src/common/values.rs.h>

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
    const google::protobuf::DescriptorPool* absl_nonnull descriptor_pool,
    google::protobuf::MessageFactory* absl_nonnull message_factory,
    google::protobuf::Arena* absl_nonnull arena,
    Value* absl_nonnull result) const {
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

OpaqueValue AnyFfiOpaqueValueWrapper::Clone(google::protobuf::Arena* absl_nonnull arena) const {
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

// ValueBuilder
class AnyFfiValueBuilderWrapper: public cel::ValueBuilder {
public:
    AnyFfiValueBuilderWrapper(Box<AnyFfiValueBuilder> ffi): ffi_(std::move(ffi)) {}
    Box<AnyFfiValueBuilder> ffi() const;
    Box<AnyFfiValueBuilder> release() && { return std::move(ffi_); }

    virtual absl::StatusOr<absl::optional<ErrorValue>> SetFieldByName(absl::string_view name, Value value) override {
        auto status = ffi_->SetFieldByName(name, value);
        if (!status.ok()) {
            return absl::Status(status.code(), status.message());
        }
        return absl::nullopt;
    }
    virtual absl::StatusOr<absl::optional<ErrorValue>> SetFieldByNumber(int64_t number, Value value) override {
        auto status = ffi_->SetFieldByNumber(number, value);
        if (!status.ok()) {
            return absl::Status(status.code(), status.message());
        }
        return absl::nullopt;
    }
    virtual absl::StatusOr<Value> Build() && override {
        std::unique_ptr<Value> result;
        auto status = ffi_->Build(result);
        if (!status.ok()) { 
            return absl::Status(status.code(), status.message());
        }
        return std::move(*result);
    }
private:
    Box<AnyFfiValueBuilder> ffi_;
};


std::unique_ptr<ValueBuilder> ValueBuilder_new(Box<AnyFfiValueBuilder> ffi) {
    return std::make_unique<AnyFfiValueBuilderWrapper>(std::move(ffi));
}

} // namespace rust::cel_cxx
