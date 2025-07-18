#ifndef CEL_CXX_FFI_INCLUDE_COMMON_VALUES_H_
#define CEL_CXX_FFI_INCLUDE_COMMON_VALUES_H_

#include <rust/cxx.h>
#include <common/value.h>

namespace rust::cel_cxx {

using absl::Status;
using absl::Time;
using absl::Duration;
using Arena = google::protobuf::Arena;
using DescriptorPool = const google::protobuf::DescriptorPool;
using MessageFactory = google::protobuf::MessageFactory;

using Type = cel::Type;
using OpaqueType = cel::OpaqueType;

using Value = cel::Value;
using BoolValue = cel::BoolValue;
using BytesValue = cel::BytesValue;
using DoubleValue = cel::DoubleValue;
using DurationValue = cel::DurationValue;
using ErrorValue = cel::ErrorValue;
using IntValue = cel::IntValue;
using ListValue = cel::ListValue;
using ListValueBuilder = cel::ListValueBuilder;
using MapValue = cel::MapValue;
using MapValueBuilder = cel::MapValueBuilder;
using MessageValue = cel::MessageValue;
using NullValue = cel::NullValue;
using OpaqueValue = cel::OpaqueValue;
using OptionalValue = cel::OptionalValue;
using StringValue = cel::StringValue;
using StructValue = cel::StructValue;
using TimestampValue = cel::TimestampValue;
using TypeValue = cel::TypeValue;
using UintValue = cel::UintValue;
using UnknownValue = cel::UnknownValue;

using ValueIterator = cel::ValueIterator;

struct AnyFfiOpaqueValue;

// Value
inline size_t Value_size_of() {
    return sizeof(Value);
}

inline void Value_swap(Value& lhs, Value& rhs) {
    swap(lhs, rhs);
}

inline std::unique_ptr<Value> Value_new_bool(const BoolValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_bytes(const BytesValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_double(const DoubleValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_duration(const DurationValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_error(const ErrorValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_int(const IntValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_list(const ListValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_map(const MapValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_message(const MessageValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_null() {
    return std::make_unique<Value>();
}

inline std::unique_ptr<Value> Value_new_opaque(const OpaqueValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_optional(const OptionalValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_string(const StringValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_timestamp(const TimestampValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_type(const TypeValue& value) {
    return std::make_unique<Value>(value);
}

inline std::unique_ptr<Value> Value_new_uint(const UintValue& value) {
    return std::make_unique<Value>(value);
}


inline std::unique_ptr<BoolValue> Value_get_bool(const Value& value) {
    return std::make_unique<BoolValue>(value.GetBool());
}

inline std::unique_ptr<BytesValue> Value_get_bytes(const Value& value) {
    return std::make_unique<BytesValue>(value.GetBytes());
}

inline std::unique_ptr<DoubleValue> Value_get_double(const Value& value) {
    return std::make_unique<DoubleValue>(value.GetDouble());
}

inline std::unique_ptr<DurationValue> Value_get_duration(const Value& value) {
    return std::make_unique<DurationValue>(value.GetDuration());
}

inline std::unique_ptr<ErrorValue> Value_get_error(const Value& value) {
    return std::make_unique<ErrorValue>(value.GetError());
}

inline std::unique_ptr<IntValue> Value_get_int(const Value& value) {
    return std::make_unique<IntValue>(value.GetInt());
}

inline std::unique_ptr<ListValue> Value_get_list(const Value& value) {
    return std::make_unique<ListValue>(value.GetList());
}

inline std::unique_ptr<MapValue> Value_get_map(const Value& value) {
    return std::make_unique<MapValue>(value.GetMap());
}

inline std::unique_ptr<MessageValue> Value_get_message(const Value& value) {
    return std::make_unique<MessageValue>(value.GetMessage());
}

inline std::unique_ptr<NullValue> Value_get_null(const Value& value) {
    return std::make_unique<NullValue>(value.GetNull());
}

inline std::unique_ptr<OpaqueValue> Value_get_opaque(const Value& value) {
    return std::make_unique<OpaqueValue>(value.GetOpaque());
}

inline std::unique_ptr<OptionalValue> Value_get_optional(const Value& value) {
    return std::make_unique<OptionalValue>(value.GetOptional());
}

inline std::unique_ptr<StringValue> Value_get_string(const Value& value) {
    return std::make_unique<StringValue>(value.GetString());
}

inline std::unique_ptr<TimestampValue> Value_get_timestamp(const Value& value) {
    return std::make_unique<TimestampValue>(value.GetTimestamp());
}

inline std::unique_ptr<TypeValue> Value_get_type(const Value& value) {
    return std::make_unique<TypeValue>(value.GetType());
}

inline std::unique_ptr<UintValue> Value_get_uint(const Value& value) {
    return std::make_unique<UintValue>(value.GetUint());
}

// BoolValue
inline std::unique_ptr<BoolValue> BoolValue_new(bool value) {
    String a;
    return std::make_unique<BoolValue>(value);
}

// BytesValue
inline std::unique_ptr<BytesValue> BytesValue_new(const Arena& arena, Slice<const uint8_t> bytes) {
    auto bytes_value = cel::BytesValue::From(
        std::string_view(
            reinterpret_cast<const char*>(bytes.data()),
            bytes.size()
        ),
        &const_cast<Arena&>(arena)
    );
    return std::make_unique<BytesValue>(std::move(bytes_value));
}

inline Vec<uint8_t> BytesValue_native_value(const BytesValue& bytes_value) {
    auto s = bytes_value.ToString();
    Vec<uint8_t> vec;
    vec.reserve(s.size());
    std::move(s.begin(), s.end(), std::back_inserter(vec));
    return vec;
}

// DoubleValue
inline std::unique_ptr<DoubleValue> DoubleValue_new(double value) {
    return std::make_unique<DoubleValue>(value);
}

// DurationValue
inline std::unique_ptr<DurationValue> DurationValue_new(Duration value) {
    return std::make_unique<DurationValue>(value);
}

// ErrorValue
inline std::unique_ptr<ErrorValue> ErrorValue_new(Status status) {
    return std::make_unique<ErrorValue>(status);
}

inline Status ErrorValue_native_value(const ErrorValue& error_value) {
    return error_value.ToStatus();
}

// IntValue
inline std::unique_ptr<IntValue> IntValue_new(int64_t value) {
    return std::make_unique<IntValue>(value);
}

// ListValue
inline bool ListValue_is_empty(const ListValue& list_value) {
    return list_value.IsEmpty().value();
}

inline size_t ListValue_size(const ListValue& list_value) {
    return list_value.Size().value();
}

inline Status ListValue_new_iterator(
    const ListValue& list_value,
    std::unique_ptr<ValueIterator>& iterator
) {
    auto status_or = list_value.NewIterator();
    if (!status_or.ok()) {
        return status_or.status();
    }
    iterator.reset(status_or->release());
    return Status();
}

// ListValueBuilder
inline std::unique_ptr<ListValueBuilder> ListValueBuilder_new(const Arena& arena) {
    return cel::NewListValueBuilder(&const_cast<Arena&>(arena));
}

inline Status ListValueBuilder_add(
    ListValueBuilder& builder,
    const Value& value
) {
    return builder.Add(value);
}

inline std::unique_ptr<ListValue> ListValueBuilder_build(
    ListValueBuilder& builder
) {
    return std::make_unique<ListValue>(std::move(builder).Build());
}

// MapValue
inline bool MapValue_is_empty(const MapValue& map_value) {
    return map_value.IsEmpty().value();
}

inline size_t MapValue_size(const MapValue& map_value) {
    return map_value.Size().value();
}

inline Status MapValue_new_iterator(
    const MapValue& map_value,
    std::unique_ptr<ValueIterator>& iterator
) {
    auto status_or = map_value.NewIterator();
    if (!status_or.ok()) {
        return status_or.status();
    }
    iterator.reset(status_or->release());
    return Status();
}

// MapValueBuilder
inline std::unique_ptr<MapValueBuilder> MapValueBuilder_new(const Arena& arena) {
    return cel::NewMapValueBuilder(&const_cast<Arena&>(arena));
}

inline Status MapValueBuilder_put(
    MapValueBuilder& builder,
    const Value& key,
    const Value& value
) {
    return builder.Put(key, value);
}

inline std::unique_ptr<MapValue> MapValueBuilder_build(
    MapValueBuilder& builder
) {
    return std::make_unique<MapValue>(std::move(builder).Build());
}


// MessageValue

// NullValue
inline std::unique_ptr<NullValue> NullValue_new() {
    return std::make_unique<NullValue>();
}

// OpaqueValue
class AnyFfiOpaqueValueWrapper: public cel::OpaqueValueInterface {
public:
    AnyFfiOpaqueValueWrapper(Box<AnyFfiOpaqueValue> ffi): ffi_(std::move(ffi)) {}
    Box<AnyFfiOpaqueValue> ffi() const;
    Box<AnyFfiOpaqueValue> release() && { return std::move(ffi_); }

    virtual std::string DebugString() const override;
    virtual std::string_view GetTypeName() const override;
    virtual OpaqueType GetRuntimeType() const override;
    virtual Status Equal(
        const OpaqueValue& other,
        const google::protobuf::DescriptorPool* ABSL_NONNULL descriptor_pool,
        google::protobuf::MessageFactory* ABSL_NONNULL message_factory,
        google::protobuf::Arena* ABSL_NONNULL arena,
        Value* ABSL_NONNULL result) const override;
    virtual OpaqueValue Clone(google::protobuf::Arena* ABSL_NONNULL arena) const override;
    virtual cel::NativeTypeId GetNativeTypeId() const override;

    struct Content {
        const OpaqueValueInterface* ABSL_NONNULL interface;
        google::protobuf::Arena* ABSL_NONNULL arena;
    };
private:
    Box<AnyFfiOpaqueValue> ffi_;
};

std::unique_ptr<OpaqueValue> OpaqueValue_new(
    const Arena& arena,
    const google::protobuf::DescriptorPool& descriptor_pool,
    Box<AnyFfiOpaqueValue> ffi);

inline std::unique_ptr<OptionalValue> OpaqueValue_get_optional(const OpaqueValue& opaque_value) {
    return std::make_unique<OptionalValue>(opaque_value.GetOptional());
}

Box<AnyFfiOpaqueValue> OpaqueValue_get_ffi(const OpaqueValue& opaque_value);

// OptionalValue
inline std::unique_ptr<OptionalValue> OptionalValue_new(const Arena& arena, const Value& value) {
    auto optional_value = OptionalValue::Of(value, &const_cast<Arena&>(arena));
    return std::make_unique<OptionalValue>(std::move(optional_value));
}

inline std::unique_ptr<OptionalValue> OptionalValue_none() {
    return std::make_unique<OptionalValue>();
}

inline std::unique_ptr<Value> OptionalValue_get_value(const OptionalValue& optional_value) {
    return std::make_unique<Value>(optional_value.Value());
}

// StringValue
inline std::unique_ptr<StringValue> StringValue_new(const Arena& arena, Str value) {
    auto string_value = StringValue::From(
        std::string_view(value.data(), value.size()),
        &const_cast<Arena&>(arena));
    return std::make_unique<StringValue>(std::move(string_value));
}

inline String StringValue_native_value(const StringValue& string_value) {
    return String::lossy(string_value.ToString());
}

// TimestampValue
inline std::unique_ptr<TimestampValue> TimestampValue_new(Time value) {
    return std::make_unique<TimestampValue>(value);
}

// TypeValue
inline std::unique_ptr<TypeValue> TypeValue_new(const Type& value) {
    return std::make_unique<TypeValue>(value);
}

// UintValue
inline std::unique_ptr<UintValue> UintValue_new(uint64_t value) {
    return std::make_unique<UintValue>(value);
}

// ValueIterator
inline Status ValueIterator_next1(
    ValueIterator& iterator,
    const google::protobuf::DescriptorPool& descriptor_pool,
    const google::protobuf::MessageFactory& message_factory,
    const google::protobuf::Arena& arena,
    std::unique_ptr<Value>& result
) {
    auto value = std::make_unique<Value>();
    auto status_or = iterator.Next1(
        &descriptor_pool,
        &const_cast<MessageFactory&>(message_factory),
        &const_cast<Arena&>(arena),
        value.get()
    );
    if (!status_or.ok()) {
        return status_or.status();
    }
    if (!status_or.value()) {
        return absl::OutOfRangeError("No more values");
    }
    result.swap(value);
    return Status();
}

inline Status ValueIterator_next2(
    ValueIterator& iterator,
    const google::protobuf::DescriptorPool& descriptor_pool,
    const google::protobuf::MessageFactory& message_factory,
    const google::protobuf::Arena& arena,
    std::unique_ptr<Value>& key,
    std::unique_ptr<Value>& value
) {
    auto key_value = std::make_unique<Value>();
    auto value_value = std::make_unique<Value>();
    auto status_or = iterator.Next2(
        &descriptor_pool,
        &const_cast<MessageFactory&>(message_factory),
        &const_cast<Arena&>(arena),
        key_value.get(),
        value_value.get()
    );
    if (!status_or.ok()) {
        return status_or.status();
    }
    if (!status_or.value()) {
        return absl::OutOfRangeError("No more values");
    }
    key.swap(key_value);
    value.swap(value_value);
    return Status();
}

} // namespace rust::cel_cxx
#endif // CEL_CXX_FFI_INCLUDE_COMMON_VALUES_H_