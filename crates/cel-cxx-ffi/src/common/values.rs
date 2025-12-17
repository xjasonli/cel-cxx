use crate::absl::{Duration, SpanElement, Status, StringView, Timestamp};
use crate::common::{MessageType, OpaqueType, Type, ValueKind};
use crate::protobuf::{Arena, DescriptorPool, MessageFactory};
use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    #[namespace = "absl"]
    unsafe extern "C++" {
        include!(<absl/time/time.h>);
        type Duration = super::Duration;
        type Time = super::Timestamp;

        include!(<absl/status/status.h>);
        type Status = super::Status;

        type string_view<'a> = super::StringView<'a>;
    }

    #[namespace = "google::protobuf"]
    unsafe extern "C++" {
        include!(<google/protobuf/arena.h>);
        type Arena = super::Arena;

        include!(<google/protobuf/descriptor.h>);
        type DescriptorPool = super::DescriptorPool;
        type MessageFactory = super::MessageFactory;
    }

    #[namespace = "cel"]
    unsafe extern "C++" {
        include!(<base/kind.h>);
        type ValueKind = super::ValueKind;

        include!(<common/type.h>);
        type Type<'a> = super::Type<'a>;
        type OpaqueType<'a> = super::OpaqueType<'a>;

        include!(<common/types/message_type.h>);
        type MessageType<'a> = super::MessageType<'a>;

        include!(<common/value.h>);
        type Value<'a>;
        fn kind(self: &Value) -> ValueKind;
        #[rust_name = "runtime_type"]
        fn GetRuntimeType<'a>(self: &Value<'a>) -> Type<'a>;
        #[rust_name = "is_bool"]
        fn IsBool(self: &Value) -> bool;
        #[rust_name = "is_true"]
        fn IsTrue(self: &Value) -> bool;
        #[rust_name = "is_false"]
        fn IsFalse(self: &Value) -> bool;
        #[rust_name = "is_bytes"]
        fn IsBytes(self: &Value) -> bool;
        #[rust_name = "is_double"]
        fn IsDouble(self: &Value) -> bool;
        #[rust_name = "is_duration"]
        fn IsDuration(self: &Value) -> bool;
        #[rust_name = "is_error"]
        fn IsError(self: &Value) -> bool;
        #[rust_name = "is_int"]
        fn IsInt(self: &Value) -> bool;
        #[rust_name = "is_list"]
        fn IsList(self: &Value) -> bool;
        #[rust_name = "is_map"]
        fn IsMap(self: &Value) -> bool;
        #[rust_name = "is_message"]
        fn IsMessage(self: &Value) -> bool;
        #[rust_name = "is_null"]
        fn IsNull(self: &Value) -> bool;
        #[rust_name = "is_opaque"]
        fn IsOpaque(self: &Value) -> bool;
        #[rust_name = "is_optional"]
        fn IsOptional(self: &Value) -> bool;
        #[rust_name = "is_string"]
        fn IsString(self: &Value) -> bool;
        #[rust_name = "is_struct"]
        fn IsStruct(self: &Value) -> bool;
        #[rust_name = "is_timestamp"]
        fn IsTimestamp(self: &Value) -> bool;
        #[rust_name = "is_type"]
        fn IsType(self: &Value) -> bool;
        #[rust_name = "is_uint"]
        fn IsUint(self: &Value) -> bool;
        #[rust_name = "is_unknown"]
        fn IsUnknown(self: &Value) -> bool;

        // BoolValue
        type BoolValue;
        #[rust_name = "native_value"]
        fn NativeValue(self: &BoolValue) -> bool;

        type BytesValue<'a>;
        #[rust_name = "len"]
        fn Size(self: &BytesValue) -> usize;
        #[rust_name = "is_empty"]
        fn IsEmpty(self: &BytesValue) -> bool;

        type DoubleValue;
        #[rust_name = "native_value"]
        fn NativeValue(self: &DoubleValue) -> f64;

        type DurationValue;
        #[rust_name = "native_value"]
        fn NativeValue(self: &DurationValue) -> Duration;

        type ErrorValue<'a>;

        type IntValue;
        #[rust_name = "native_value"]
        fn NativeValue(self: &IntValue) -> i64;

        type ListValue<'a>;
        type ListValueBuilder<'a>;

        type MapValue<'a>;
        type MapValueBuilder<'a>;

        type MessageValue<'a>;
        #[rust_name = "runtime_type"]
        fn GetRuntimeType<'a>(self: &MessageValue<'a>) -> MessageType<'a>;

        type NullValue;
        type OpaqueValue<'a>;
        #[rust_name = "is_optional"]
        fn IsOptional(self: &OpaqueValue) -> bool;

        type OptionalValue<'a>;
        #[rust_name = "has_value"]
        fn HasValue(self: &OptionalValue) -> bool;

        type StringValue<'a>;
        #[rust_name = "len"]
        fn Size(self: &StringValue) -> usize;
        #[rust_name = "is_empty"]
        fn IsEmpty(self: &StringValue) -> bool;

        //type StructValue;
        // todo

        type TimestampValue;
        #[rust_name = "native_value"]
        fn NativeValue(self: &TimestampValue) -> Time;

        type TypeValue<'a>;
        #[rust_name = "native_value"]
        fn NativeValue<'a>(self: &TypeValue<'a>) -> &Type<'a>;

        type UintValue;
        #[rust_name = "native_value"]
        fn NativeValue(self: &UintValue) -> u64;

        //type UnknownValue;
        // todo

        type ValueIterator<'a>;
        #[rust_name = "has_next"]
        fn HasNext(self: Pin<&mut ValueIterator>) -> bool;

        // ValueBuilder
        type ValueBuilder<'a>;
    }

    #[namespace = "rust::cel_cxx"]
    unsafe extern "C++" {
        include!(<cel-cxx-ffi/include/absl.h>);
        include!(<cel-cxx-ffi/include/values.h>);

        // Value
        fn Value_size_of() -> usize;
        fn Value_swap<'a>(lhs: Pin<&mut Value<'a>>, rhs: Pin<&mut Value<'a>>);

        fn Value_new_bool(bool_value: &BoolValue) -> UniquePtr<Value<'static>>;
        fn Value_new_bytes<'a>(bytes_value: &BytesValue<'a>) -> UniquePtr<Value<'a>>;
        fn Value_new_double(double_value: &DoubleValue) -> UniquePtr<Value<'static>>;
        fn Value_new_duration(duration_value: &DurationValue) -> UniquePtr<Value<'static>>;
        fn Value_new_error<'a>(error_value: &ErrorValue<'a>) -> UniquePtr<Value<'a>>;
        fn Value_new_int(int_value: &IntValue) -> UniquePtr<Value<'static>>;
        fn Value_new_list<'a>(list_value: &ListValue<'a>) -> UniquePtr<Value<'a>>;
        fn Value_new_map<'a>(map_value: &MapValue<'a>) -> UniquePtr<Value<'a>>;
        fn Value_new_message<'a>(message_value: &MessageValue<'a>) -> UniquePtr<Value<'a>>;
        fn Value_new_null() -> UniquePtr<Value<'static>>;
        fn Value_new_opaque<'a>(opaque_value: &OpaqueValue<'a>) -> UniquePtr<Value<'a>>;
        fn Value_new_optional<'a>(optional_value: &OptionalValue<'a>) -> UniquePtr<Value<'a>>;
        fn Value_new_string<'a>(string_value: &StringValue<'a>) -> UniquePtr<Value<'a>>;
        fn Value_new_timestamp(timestamp_value: &TimestampValue) -> UniquePtr<Value<'static>>;
        fn Value_new_type<'a>(type_value: &TypeValue<'a>) -> UniquePtr<Value<'a>>;
        fn Value_new_uint(uint_value: &UintValue) -> UniquePtr<Value<'static>>;

        fn Value_get_bool(value: &Value) -> UniquePtr<BoolValue>;
        fn Value_get_bytes<'a>(value: &Value<'a>) -> UniquePtr<BytesValue<'a>>;
        fn Value_get_double(value: &Value) -> UniquePtr<DoubleValue>;
        fn Value_get_duration(value: &Value) -> UniquePtr<DurationValue>;
        fn Value_get_error<'a>(value: &Value<'a>) -> UniquePtr<ErrorValue<'a>>;
        fn Value_get_int(value: &Value) -> UniquePtr<IntValue>;
        fn Value_get_list<'a>(value: &Value<'a>) -> UniquePtr<ListValue<'a>>;
        fn Value_get_map<'a>(value: &Value<'a>) -> UniquePtr<MapValue<'a>>;
        fn Value_get_message<'a>(value: &Value<'a>) -> UniquePtr<MessageValue<'a>>;
        fn Value_get_null(value: &Value) -> UniquePtr<NullValue>;
        fn Value_get_opaque<'a>(value: &Value<'a>) -> UniquePtr<OpaqueValue<'a>>;
        fn Value_get_optional<'a>(value: &Value<'a>) -> UniquePtr<OptionalValue<'a>>;
        fn Value_get_string<'a>(value: &Value<'a>) -> UniquePtr<StringValue<'a>>;
        fn Value_get_timestamp(value: &Value) -> UniquePtr<TimestampValue>;
        fn Value_get_type<'a>(value: &Value<'a>) -> UniquePtr<TypeValue<'a>>;
        fn Value_get_uint(value: &Value) -> UniquePtr<UintValue>;

        // BoolValue
        fn BoolValue_new(value: bool) -> UniquePtr<BoolValue>;

        // BytesValue
        fn BytesValue_new<'a>(arena: &'a Arena, bytes: &[u8]) -> UniquePtr<BytesValue<'a>>;
        fn BytesValue_native_value(bytes_value: &BytesValue) -> Vec<u8>;

        // DoubleValue
        fn DoubleValue_new(value: f64) -> UniquePtr<DoubleValue>;

        // DurationValue
        fn DurationValue_new(value: Duration) -> UniquePtr<DurationValue>;

        // ErrorValue
        fn ErrorValue_new(status: Status) -> UniquePtr<ErrorValue<'static>>;
        fn ErrorValue_native_value(error_value: &ErrorValue) -> Status;

        // IntValue
        fn IntValue_new(value: i64) -> UniquePtr<IntValue>;

        // ListValue
        fn ListValue_is_empty(list_value: &ListValue) -> bool;
        fn ListValue_size(list_value: &ListValue) -> usize;
        fn ListValue_new_iterator<'a, 'this>(
            list_value: &'this ListValue<'a>,
            iterator: &mut UniquePtr<ValueIterator<'this>>,
        ) -> Status;

        // ListValueBuilder
        fn ListValueBuilder_new<'a>(arena: &'a Arena) -> UniquePtr<ListValueBuilder<'a>>;
        fn ListValueBuilder_add<'a>(
            builder: Pin<&mut ListValueBuilder<'a>>,
            value: &Value<'a>,
        ) -> Status;
        fn ListValueBuilder_build<'a>(
            builder: Pin<&mut ListValueBuilder<'a>>,
        ) -> UniquePtr<ListValue<'a>>;

        // MapValue
        fn MapValue_is_empty(map_value: &MapValue) -> bool;
        fn MapValue_size(map_value: &MapValue) -> usize;
        fn MapValue_new_iterator<'a, 'this>(
            map_value: &'this MapValue<'a>,
            iterator: &mut UniquePtr<ValueIterator<'this>>,
        ) -> Status;

        // MapValueBuilder
        fn MapValueBuilder_new<'a>(arena: &'a Arena) -> UniquePtr<MapValueBuilder<'a>>;
        fn MapValueBuilder_put<'a>(
            builder: Pin<&mut MapValueBuilder<'a>>,
            key: &Value<'a>,
            value: &Value<'a>,
        ) -> Status;
        fn MapValueBuilder_build<'a>(
            builder: Pin<&mut MapValueBuilder<'a>>,
        ) -> UniquePtr<MapValue<'a>>;

        // MessageValue
        // todo: implement

        // NullValue
        fn NullValue_new() -> UniquePtr<NullValue>;

        // OpaqueValue
        fn OpaqueValue_new<'a>(
            arena: &'a Arena,
            descriptor_pool: &'a DescriptorPool,
            ffi: Box<AnyFfiOpaqueValue<'a>>,
        ) -> UniquePtr<OpaqueValue<'a>>;
        fn OpaqueValue_get_optional<'a>(
            opaque_value: &OpaqueValue<'a>,
        ) -> UniquePtr<OptionalValue<'a>>;
        fn OpaqueValue_get_ffi<'a>(opaque_value: &OpaqueValue<'a>) -> Box<AnyFfiOpaqueValue<'a>>;

        // OptionalValue
        fn OptionalValue_new<'a>(
            arena: &'a Arena,
            value: &Value<'a>,
        ) -> UniquePtr<OptionalValue<'a>>;
        fn OptionalValue_none() -> UniquePtr<OptionalValue<'static>>;
        fn OptionalValue_get_value<'a>(optional_value: &OptionalValue<'a>) -> UniquePtr<Value<'a>>;

        // StringValue
        fn StringValue_new<'a>(arena: &'a Arena, value: &str) -> UniquePtr<StringValue<'a>>;
        fn StringValue_native_value(string_value: &StringValue) -> String;

        // TimestampValue
        fn TimestampValue_new(value: Time) -> UniquePtr<TimestampValue>;

        // TypeValue
        fn TypeValue_new<'a>(value: &Type<'a>) -> UniquePtr<TypeValue<'a>>;

        // UintValue
        fn UintValue_new(value: u64) -> UniquePtr<UintValue>;

        // ValueIterator
        fn ValueIterator_next1<'a, 'b>(
            value_iterator: Pin<&mut ValueIterator<'a>>,
            descriptor_pool: &DescriptorPool,
            message_factory: &MessageFactory,
            arena: &'b Arena,
            result: &mut UniquePtr<Value<'b>>,
        ) -> Status;
        fn ValueIterator_next2<'a, 'b>(
            value_iterator: Pin<&mut ValueIterator<'a>>,
            descriptor_pool: &DescriptorPool,
            message_factory: &MessageFactory,
            arena: &'b Arena,
            key: &mut UniquePtr<Value<'b>>,
            value: &mut UniquePtr<Value<'b>>,
        ) -> Status;

        // ValueBuilder
        fn ValueBuilder_new<'a>(ffi: Box<AnyFfiValueBuilder<'a>>) -> UniquePtr<ValueBuilder<'a>>;
        fn ValueBuilder_new_message<'a>(
            arena: &'a Arena,
            descriptor_pool: &'a DescriptorPool,
            message_factory: &'a MessageFactory,
            name: string_view<'_>,
        ) -> UniquePtr<ValueBuilder<'a>>;
    }

    #[namespace = "rust::cel_cxx"]
    extern "Rust" {
        type AnyFfiOpaqueValue<'a>;
        #[cxx_name = "Clone"]
        unsafe fn clone<'a>(self: &'a AnyFfiOpaqueValue<'a>) -> Box<AnyFfiOpaqueValue<'a>>;
        #[cxx_name = "DebugString"]
        unsafe fn to_string<'a>(self: &AnyFfiOpaqueValue<'a>) -> String;
        #[cxx_name = "GetTypeName"]
        unsafe fn type_name<'a>(self: &AnyFfiOpaqueValue<'a>) -> string_view<'a>;
        #[cxx_name = "GetRuntimeType"]
        unsafe fn runtime_type<'a>(self: &AnyFfiOpaqueValue<'a>) -> OpaqueType<'a>;
        #[cxx_name = "Equal"]
        unsafe fn equal<'a>(self: &AnyFfiOpaqueValue<'a>, other: &AnyFfiOpaqueValue<'a>) -> bool;

        type AnyFfiValueBuilder<'a>;
        #[cxx_name = "SetFieldByName"]
        unsafe fn set_field_by_name<'a>(self: &mut AnyFfiValueBuilder<'a>, name: string_view<'a>, value: &Value<'a>) -> Status;
        #[cxx_name = "SetFieldByNumber"]
        unsafe fn set_field_by_number<'a>(self: &mut AnyFfiValueBuilder<'a>, number: i64, value: &Value<'a>) -> Status;
        #[cxx_name = "Build"]
        unsafe fn build<'a>(self: &mut AnyFfiValueBuilder<'a>, result: &mut UniquePtr<Value<'a>>) -> Status;
    }
}

// Value
pub use ffi::Value;
unsafe impl<'a> Send for Value<'a> {}
unsafe impl<'a> Sync for Value<'a> {}

impl<'a> crate::SizedExternType for Value<'a> {
    fn size_of() -> usize {
        ffi::Value_size_of()
    }
}

impl<'a> SpanElement for Value<'a> {
    type TypeId = cxx::type_id!("rust::cel_cxx::Span_Value");
}

impl<'a> Value<'a> {
    pub fn swap(self: Pin<&mut Self>, other: Pin<&mut Self>) {
        ffi::Value_swap(self, other);
    }

    pub fn new_bool(value: &BoolValue) -> cxx::UniquePtr<Self> {
        ffi::Value_new_bool(value)
    }

    pub fn new_bytes(bytes: &BytesValue<'a>) -> cxx::UniquePtr<Self> {
        ffi::Value_new_bytes(bytes)
    }

    pub fn new_double(value: &DoubleValue) -> cxx::UniquePtr<Self> {
        ffi::Value_new_double(value)
    }

    pub fn new_duration(value: &DurationValue) -> cxx::UniquePtr<Self> {
        ffi::Value_new_duration(value)
    }

    pub fn new_error(value: &ErrorValue<'a>) -> cxx::UniquePtr<Self> {
        ffi::Value_new_error(value)
    }

    pub fn new_int(value: &IntValue) -> cxx::UniquePtr<Self> {
        ffi::Value_new_int(value)
    }

    pub fn new_list(list: &ListValue<'a>) -> cxx::UniquePtr<Self> {
        ffi::Value_new_list(list)
    }

    pub fn new_map(map: &MapValue<'a>) -> cxx::UniquePtr<Self> {
        ffi::Value_new_map(map)
    }

    pub fn new_message(message: &MessageValue<'a>) -> cxx::UniquePtr<Self> {
        ffi::Value_new_message(message)
    }

    pub fn new_null() -> cxx::UniquePtr<Self> {
        ffi::Value_new_null()
    }

    pub fn new_opaque(opaque: &OpaqueValue<'a>) -> cxx::UniquePtr<Self> {
        ffi::Value_new_opaque(opaque)
    }

    pub fn new_optional(optional: &OptionalValue<'a>) -> cxx::UniquePtr<Self> {
        ffi::Value_new_optional(optional)
    }

    pub fn new_string(string: &StringValue<'a>) -> cxx::UniquePtr<Self> {
        ffi::Value_new_string(string)
    }

    pub fn new_timestamp(timestamp: &TimestampValue) -> cxx::UniquePtr<Self> {
        ffi::Value_new_timestamp(timestamp)
    }

    pub fn new_type(type_value: &TypeValue<'a>) -> cxx::UniquePtr<Self> {
        ffi::Value_new_type(type_value)
    }

    pub fn new_uint(uint: &UintValue) -> cxx::UniquePtr<Self> {
        ffi::Value_new_uint(uint)
    }

    pub fn get_bool(&self) -> cxx::UniquePtr<BoolValue> {
        ffi::Value_get_bool(self)
    }

    pub fn get_bytes(&self) -> cxx::UniquePtr<BytesValue<'a>> {
        ffi::Value_get_bytes(self)
    }

    pub fn get_double(&self) -> cxx::UniquePtr<DoubleValue> {
        ffi::Value_get_double(self)
    }

    pub fn get_duration(&self) -> cxx::UniquePtr<DurationValue> {
        ffi::Value_get_duration(self)
    }

    pub fn get_error(&self) -> cxx::UniquePtr<ErrorValue<'a>> {
        ffi::Value_get_error(self)
    }

    pub fn get_int(&self) -> cxx::UniquePtr<IntValue> {
        ffi::Value_get_int(self)
    }

    pub fn get_list(&self) -> cxx::UniquePtr<ListValue<'a>> {
        ffi::Value_get_list(self)
    }

    pub fn get_map(&self) -> cxx::UniquePtr<MapValue<'a>> {
        ffi::Value_get_map(self)
    }

    pub fn get_message(&self) -> cxx::UniquePtr<MessageValue<'a>> {
        ffi::Value_get_message(self)
    }

    pub fn get_null(&self) -> cxx::UniquePtr<NullValue> {
        ffi::Value_get_null(self)
    }

    pub fn get_opaque(&self) -> cxx::UniquePtr<OpaqueValue<'a>> {
        ffi::Value_get_opaque(self)
    }

    pub fn get_optional(&self) -> cxx::UniquePtr<OptionalValue<'a>> {
        ffi::Value_get_optional(self)
    }

    pub fn get_string(&self) -> cxx::UniquePtr<StringValue<'a>> {
        ffi::Value_get_string(self)
    }

    pub fn get_timestamp(&self) -> cxx::UniquePtr<TimestampValue> {
        ffi::Value_get_timestamp(self)
    }

    pub fn get_type(&self) -> cxx::UniquePtr<TypeValue<'a>> {
        ffi::Value_get_type(self)
    }

    pub fn get_uint(&self) -> cxx::UniquePtr<UintValue> {
        ffi::Value_get_uint(self)
    }
}

// BoolValue
pub use ffi::BoolValue;
unsafe impl Send for BoolValue {}
unsafe impl Sync for BoolValue {}

impl BoolValue {
    pub fn new(value: bool) -> cxx::UniquePtr<Self> {
        ffi::BoolValue_new(value)
    }
}

// BytesValue
pub use ffi::BytesValue;
unsafe impl<'a> Send for BytesValue<'a> {}
unsafe impl<'a> Sync for BytesValue<'a> {}

impl<'a> BytesValue<'a> {
    pub fn new(arena: &'a Arena, bytes: &[u8]) -> cxx::UniquePtr<Self> {
        ffi::BytesValue_new(arena, bytes)
    }

    pub fn native_value(&self) -> Vec<u8> {
        ffi::BytesValue_native_value(self)
    }
}

// DoubleValue
pub use ffi::DoubleValue;
unsafe impl Send for DoubleValue {}
unsafe impl Sync for DoubleValue {}

impl DoubleValue {
    pub fn new(value: f64) -> cxx::UniquePtr<Self> {
        ffi::DoubleValue_new(value)
    }
}

// DurationValue
pub use ffi::DurationValue;
unsafe impl Send for DurationValue {}
unsafe impl Sync for DurationValue {}

impl DurationValue {
    pub fn new(value: Duration) -> cxx::UniquePtr<Self> {
        ffi::DurationValue_new(value)
    }
}

// ErrorValue
pub use ffi::ErrorValue;
unsafe impl<'a> Send for ErrorValue<'a> {}
unsafe impl<'a> Sync for ErrorValue<'a> {}

impl<'a> ErrorValue<'a> {
    pub fn new(status: Status) -> cxx::UniquePtr<Self> {
        ffi::ErrorValue_new(status)
    }

    pub fn native_value(&self) -> Status {
        ffi::ErrorValue_native_value(self)
    }
}

// IntValue
pub use ffi::IntValue;
unsafe impl Send for IntValue {}
unsafe impl Sync for IntValue {}

impl IntValue {
    pub fn new(value: i64) -> cxx::UniquePtr<Self> {
        ffi::IntValue_new(value)
    }
}

// ListValue
pub use ffi::ListValue;
unsafe impl<'a> Send for ListValue<'a> {}
unsafe impl<'a> Sync for ListValue<'a> {}

impl<'a> ListValue<'a> {
    pub fn is_empty(&self) -> bool {
        ffi::ListValue_is_empty(self)
    }

    pub fn len(&self) -> usize {
        ffi::ListValue_size(self)
    }

    pub fn new_iterator<'this>(
        &'this self,
    ) -> Result<cxx::UniquePtr<ffi::ValueIterator<'this>>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::ListValue_new_iterator(self, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }
}

// ListValueBuilder
pub use ffi::ListValueBuilder;
unsafe impl<'a> Send for ListValueBuilder<'a> {}
unsafe impl<'a> Sync for ListValueBuilder<'a> {}

impl<'a> ListValueBuilder<'a> {
    pub fn new(arena: &'a Arena) -> cxx::UniquePtr<Self> {
        ffi::ListValueBuilder_new(arena)
    }

    pub fn add(self: Pin<&mut Self>, value: &Value<'a>) -> Status {
        ffi::ListValueBuilder_add(self, value)
    }

    pub fn build(self: Pin<&mut Self>) -> cxx::UniquePtr<ListValue<'a>> {
        ffi::ListValueBuilder_build(self)
    }
}

// MapValue
pub use ffi::MapValue;
unsafe impl<'a> Send for MapValue<'a> {}
unsafe impl<'a> Sync for MapValue<'a> {}

impl<'a> MapValue<'a> {
    pub fn is_empty(&self) -> bool {
        ffi::MapValue_is_empty(self)
    }

    pub fn len(&self) -> usize {
        ffi::MapValue_size(self)
    }

    pub fn new_iterator<'this>(
        &'this self,
    ) -> Result<cxx::UniquePtr<ffi::ValueIterator<'this>>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status = ffi::MapValue_new_iterator(self, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }
}

// MapValueBuilder
pub use ffi::MapValueBuilder;
unsafe impl<'a> Send for MapValueBuilder<'a> {}
unsafe impl<'a> Sync for MapValueBuilder<'a> {}

impl<'a> MapValueBuilder<'a> {
    pub fn new(arena: &'a Arena) -> cxx::UniquePtr<Self> {
        ffi::MapValueBuilder_new(arena)
    }

    pub fn put(self: Pin<&mut Self>, key: &Value<'a>, value: &Value<'a>) -> Status {
        ffi::MapValueBuilder_put(self, key, value)
    }

    pub fn build(self: Pin<&mut Self>) -> cxx::UniquePtr<MapValue<'a>> {
        ffi::MapValueBuilder_build(self)
    }
}

pub use ffi::MessageValue;
unsafe impl<'a> Send for MessageValue<'a> {}
unsafe impl<'a> Sync for MessageValue<'a> {}

impl<'a> MessageValue<'a> {}

pub use ffi::NullValue;
unsafe impl Send for NullValue {}
unsafe impl Sync for NullValue {}

impl NullValue {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::NullValue_new()
    }
}

pub use ffi::OpaqueValue;
unsafe impl<'a> Send for OpaqueValue<'a> {}
unsafe impl<'a> Sync for OpaqueValue<'a> {}

impl<'a> OpaqueValue<'a> {
    pub fn new<T: FfiOpaqueValue + 'a>(
        arena: &'a Arena,
        descriptor_pool: &'a DescriptorPool,
        ffi: T,
    ) -> cxx::UniquePtr<Self> {
        ffi::OpaqueValue_new(
            arena,
            descriptor_pool,
            Box::new(AnyFfiOpaqueValue::new(ffi, arena, descriptor_pool)),
        )
    }

    pub fn get_optional(&self) -> cxx::UniquePtr<OptionalValue<'a>> {
        ffi::OpaqueValue_get_optional(self)
    }

    pub fn downcast<T: FfiOpaqueValue>(&self) -> Option<Box<T>> {
        let ffi = ffi::OpaqueValue_get_ffi(self);
        ffi.downcast::<T>()
    }
}

pub trait FfiOpaqueValue:
    'static + std::fmt::Debug + std::fmt::Display + dyn_clone::DynClone + private::Sealed + Send + Sync
{
    fn opaque_type<'a>(
        &self,
        arena: &'a Arena,
        descriptor_pool: &'a DescriptorPool,
    ) -> OpaqueType<'a>;
}
dyn_clone::clone_trait_object!(FfiOpaqueValue);

#[derive(Clone)]
struct AnyFfiOpaqueValue<'a> {
    ffi: Box<dyn FfiOpaqueValue>,
    opaque_type: OpaqueType<'a>,
}

impl<'a> AnyFfiOpaqueValue<'a> {
    pub fn new<T: FfiOpaqueValue + 'static>(
        value: T,
        arena: &'a Arena,
        descriptor_pool: &'a DescriptorPool,
    ) -> Self {
        let opaque_type = value.opaque_type(arena, descriptor_pool);
        Self {
            ffi: Box::new(value),
            opaque_type,
        }
    }

    pub fn clone(&self) -> Box<AnyFfiOpaqueValue<'a>> {
        Box::new(Clone::clone(self))
    }

    pub fn type_name(&self) -> StringView<'a> {
        self.opaque_type.name()
    }

    pub fn runtime_type(&self) -> OpaqueType<'a> {
        self.opaque_type
    }

    pub fn equal<'b>(&self, other: &AnyFfiOpaqueValue<'b>) -> bool {
        self == other
    }

    pub fn downcast<T: FfiOpaqueValue>(&self) -> Option<Box<T>> {
        self.ffi.clone().into_any().downcast::<T>().ok()
    }
}

impl<'a> std::fmt::Display for AnyFfiOpaqueValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnyFfiOpaqueValue")
    }
}

impl<'a> PartialEq for AnyFfiOpaqueValue<'a> {
    fn eq(&self, other: &Self) -> bool {
        *self.ffi == *other.ffi
    }
}

impl<'a> Eq for AnyFfiOpaqueValue<'a> {}

impl<'a> std::fmt::Debug for AnyFfiOpaqueValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnyFfiOpaqueValue({:?})", self.ffi)
    }
}

impl PartialEq for dyn FfiOpaqueValue {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(private::Sealed::as_any(other))
    }
}

impl Eq for dyn FfiOpaqueValue {}

mod private {
    use super::FfiOpaqueValue;
    use std::any::Any;

    pub trait Sealed: Any {
        fn into_any(self: Box<Self>) -> Box<dyn Any>;
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;

        fn dyn_eq(&self, other: &dyn Any) -> bool;
    }

    impl<T: FfiOpaqueValue + PartialEq + Eq> Sealed for T {
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }

        fn dyn_eq(&self, other: &dyn Any) -> bool {
            other.downcast_ref::<T>() == Some(self)
        }
    }
}

pub use ffi::OptionalValue;
unsafe impl<'a> Send for OptionalValue<'a> {}
unsafe impl<'a> Sync for OptionalValue<'a> {}

impl<'a> OptionalValue<'a> {
    pub fn new(arena: &'a Arena, value: &Value<'a>) -> cxx::UniquePtr<Self> {
        ffi::OptionalValue_new(arena, value)
    }

    pub fn none() -> cxx::UniquePtr<Self> {
        ffi::OptionalValue_none()
    }

    pub fn get_value(&self) -> cxx::UniquePtr<Value<'a>> {
        ffi::OptionalValue_get_value(self)
    }
}

pub use ffi::StringValue;
unsafe impl<'a> Send for StringValue<'a> {}
unsafe impl<'a> Sync for StringValue<'a> {}

impl<'a> StringValue<'a> {
    pub fn new(arena: &'a Arena, value: &str) -> cxx::UniquePtr<Self> {
        ffi::StringValue_new(arena, value)
    }

    pub fn native_value(&self) -> String {
        ffi::StringValue_native_value(self)
    }
}

pub use ffi::TimestampValue;
unsafe impl Send for TimestampValue {}
unsafe impl Sync for TimestampValue {}

impl TimestampValue {
    pub fn new(value: Timestamp) -> cxx::UniquePtr<Self> {
        ffi::TimestampValue_new(value)
    }
}

pub use ffi::TypeValue;
unsafe impl<'a> Send for TypeValue<'a> {}
unsafe impl<'a> Sync for TypeValue<'a> {}

impl<'a> TypeValue<'a> {
    pub fn new(value: &Type<'a>) -> cxx::UniquePtr<Self> {
        ffi::TypeValue_new(value)
    }
}

pub use ffi::UintValue;
unsafe impl Send for UintValue {}
unsafe impl Sync for UintValue {}

impl UintValue {
    pub fn new(value: u64) -> cxx::UniquePtr<Self> {
        ffi::UintValue_new(value)
    }
}

pub use ffi::ValueIterator;
unsafe impl<'a> Send for ValueIterator<'a> {}
unsafe impl<'a> Sync for ValueIterator<'a> {}

impl<'a> ValueIterator<'a> {
    pub fn next1<'b>(
        self: Pin<&mut Self>,
        descriptor_pool: &DescriptorPool,
        message_factory: &MessageFactory,
        arena: &'b Arena,
    ) -> Result<cxx::UniquePtr<Value<'b>>, Status> {
        let mut result = cxx::UniquePtr::null();
        let status =
            ffi::ValueIterator_next1(self, descriptor_pool, message_factory, arena, &mut result);
        if status.is_ok() {
            Ok(result)
        } else {
            Err(status)
        }
    }

    pub fn next2<'b>(
        self: Pin<&mut Self>,
        descriptor_pool: &DescriptorPool,
        message_factory: &MessageFactory,
        arena: &'b Arena,
    ) -> Result<(cxx::UniquePtr<Value<'b>>, cxx::UniquePtr<Value<'b>>), Status> {
        let mut key = cxx::UniquePtr::null();
        let mut value = cxx::UniquePtr::null();
        let status = ffi::ValueIterator_next2(
            self,
            descriptor_pool,
            message_factory,
            arena,
            &mut key,
            &mut value,
        );
        if status.is_ok() {
            Ok((key, value))
        } else {
            Err(status)
        }
    }
}

// ValueBuilder
pub trait FfiValueBuilder<'a> {
    fn set_field_by_name(
        &mut self,
        name: StringView<'_>,
        value: &Value<'a>,
    ) -> Result<(), Status>;
    fn set_field_by_number(
        &mut self,
        number: i64,
        value: &Value<'a>,
    ) -> Result<(), Status>;

    fn build(self: Box<Self>) -> Result<cxx::UniquePtr<Value<'a>>, Status>;
}

struct AnyFfiValueBuilder<'a>(Option<Box<dyn FfiValueBuilder<'a> + 'a>>);

impl<'a> AnyFfiValueBuilder<'a> {
    fn new<T: FfiValueBuilder<'a> + 'a>(ffi_value_builder: T) -> Self {
        Self(Some(Box::new(ffi_value_builder)))
    }

    fn set_field_by_name(
        &mut self,
        name: StringView<'_>,
        value: &Value<'a>,
    ) -> Status {
        let ffi = self.0.as_mut().expect("ffi_value_builder is not set");
        match ffi.set_field_by_name(name, value) {
            Ok(_) => {
                Status::ok()
            }
            Err(status) => status,
        }
    }
    fn set_field_by_number(
        &mut self,
        number: i64,
        value: &Value<'a>,
    ) -> Status {
        let ffi = self.0.as_mut().expect("ffi_value_builder is not set");
        match ffi.set_field_by_number(number, value) {
            Ok(_) => {
                Status::ok()
            }
            Err(status) => status,
        }
    }

    fn build(&mut self, result: &mut cxx::UniquePtr<Value<'a>>) -> Status {
        let ffi = self.0.take().expect("ffi_value_builder is not set");
        match ffi.build() {
            Ok(value) => {
                *result = value;
                Status::ok()
            }
            Err(status) => status,
        }
    }
}

pub use ffi::ValueBuilder;
unsafe impl<'a> Send for ValueBuilder<'a> {}
unsafe impl<'a> Sync for ValueBuilder<'a> {}

impl<'a> ValueBuilder<'a> {
    pub fn new<T: FfiValueBuilder<'a> + 'a>(ffi_value_builder: T) -> cxx::UniquePtr<Self> {
        ffi::ValueBuilder_new(Box::new(AnyFfiValueBuilder::new(ffi_value_builder)))
    }

    pub fn new_message(
        arena: &'a Arena,
        descriptor_pool: &'a DescriptorPool,
        message_factory: &'a MessageFactory,
        name: StringView<'_>,
    ) -> cxx::UniquePtr<Self> {
        ffi::ValueBuilder_new_message(arena, descriptor_pool, message_factory, name)
    }
}
