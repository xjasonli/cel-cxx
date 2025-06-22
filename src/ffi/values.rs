use super::*;

pub(crate) fn value_from_rust<'a>(
    value: &rust::Value,
    arena: &'a Arena,
    descriptor_pool: &'a DescriptorPool,
    message_factory: &'a MessageFactory,
) -> cxx::UniquePtr<Value<'a>> {
    match value {
        rust::Value::Null => Value::new_null(),
        rust::Value::Bool(b) => {
            let bool_value = BoolValue::new(*b);
            Value::new_bool(&bool_value)
        }
        rust::Value::Int(i) => {
            let int_value = IntValue::new(*i);
            Value::new_int(&int_value)
        }
        rust::Value::Uint(u) => {
            let uint_value = UintValue::new(*u);
            Value::new_uint(&uint_value)
        }
        rust::Value::Double(d) => {
            let double_value = DoubleValue::new(*d);
            Value::new_double(&double_value)
        }
        rust::Value::String(s) => {
            let string_value = StringValue::new(arena, s);
            Value::new_string(&string_value)
        }
        rust::Value::Bytes(b) => {
            let bytes_value = BytesValue::new(arena, b);
            Value::new_bytes(&bytes_value)
        }
        rust::Value::Struct(_s) => {
            todo!()
        }
        rust::Value::Duration(d) => {
            let duration = (*d).into();
            let duration_value = DurationValue::new(duration);
            Value::new_duration(&duration_value)
        }
        rust::Value::Timestamp(t) => {
            let timestamp = (*t).into();
            let timestamp_value = TimestampValue::new(timestamp);
            Value::new_timestamp(&timestamp_value)
        }
        rust::Value::List(l) => {
            let mut list_value_builder = ListValueBuilder::new(arena);
            for v in l {
                let value = value_from_rust(v, arena, descriptor_pool, message_factory);
                list_value_builder.pin_mut().add(&value);
            }
            let list_value = list_value_builder.pin_mut().build();
            Value::new_list(&list_value)
        }
        rust::Value::Map(m) => {
            let mut map_value_builder = MapValueBuilder::new(arena);
            for (k, v) in m {
                let key = value_from_rust(&k.clone().into_value(), arena, descriptor_pool, message_factory);
                let value = value_from_rust(v, arena, descriptor_pool, message_factory);
                map_value_builder.pin_mut().put(&key, &value);
            }
            let map_value = map_value_builder.pin_mut().build();
            Value::new_map(&map_value)
        }
        rust::Value::Unknown(..) => {
            todo!()
        }
        rust::Value::Type(t) => {
            let type_ = super::type_from_rust(t, arena, descriptor_pool);
            let type_value = TypeValue::new(&type_);
            Value::new_type(&type_value)
        }
        rust::Value::Error(e) => {
            let status = super::error_from_rust(e);
            let error_value = ErrorValue::new(status);
            Value::new_error(&error_value)
        }
        rust::Value::Opaque(o) => {
            let opaque_value = opaque_value_from_rust(o, arena, descriptor_pool);
            Value::new_opaque(&opaque_value)
        }
        rust::Value::Optional(o) => {
            let optional_value = optional_value_from_rust(o, arena, descriptor_pool, message_factory);
            Value::new_optional(&optional_value)
        }
    }
}

pub(crate) fn value_to_rust<'a>(
    value: &Value<'a>,
    arena: &'a Arena,
    descriptor_pool: &'a DescriptorPool,
    message_factory: &'a MessageFactory,
) -> Result<rust::Value, rust::Error> {
    match value.kind() {
        ValueKind::Null => Ok(rust::Value::Null),
        ValueKind::Bool => {
            let bool_value = value.get_bool();
            Ok(rust::Value::Bool(bool_value.native_value()))
        }
        ValueKind::Int => {
            let int_value = value.get_int();
            Ok(rust::Value::Int(int_value.native_value()))
        }
        ValueKind::Uint => {
            let uint_value = value.get_uint();
            Ok(rust::Value::Uint(uint_value.native_value()))
        }
        ValueKind::Double => {
            let double_value = value.get_double();
            Ok(rust::Value::Double(double_value.native_value()))
        }
        ValueKind::String => {
            let string_value = value.get_string();
            Ok(rust::Value::String(string_value.native_value().into()))
        }
        ValueKind::Bytes => {
            let bytes_value = value.get_bytes();
            Ok(rust::Value::Bytes(bytes_value.native_value().into()))
        }
        ValueKind::Struct => {
            todo!()
            //let struct_value = value.get_message();
            //rust::Value::Message(struct_value.native_value())
        }
        ValueKind::Duration => {
            let duration_value = value.get_duration();
            Ok(rust::Value::Duration(duration_value.native_value().into()))
        }
        ValueKind::Timestamp => {
            let timestamp_value = value.get_timestamp();
            Ok(rust::Value::Timestamp(timestamp_value.native_value().into()))
        }
        ValueKind::List => {
            let mut result = Vec::new();
            let list_value = value.get_list();
            let mut iter = list_value.new_iterator()
                .map_err(|e| super::error_to_rust(&e))?;
            while iter.pin_mut().has_next() {
                let item = iter.pin_mut()
                    .next1(descriptor_pool, message_factory, arena)
                    .map_err(|e| super::error_to_rust(&e))?;
                result.push(value_to_rust(&item, arena, descriptor_pool, message_factory)?);
            }
            Ok(rust::Value::List(result))
        }
        ValueKind::Map => {
            let mut result = std::collections::HashMap::new();
            let map_value = value.get_map();
            let mut iter = map_value.new_iterator()
                .map_err(|e| super::error_to_rust(&e))?;
            while iter.pin_mut().has_next() {
                let (key, value) = iter.pin_mut()
                    .next2(descriptor_pool, message_factory, arena)
                    .map_err(|e| super::error_to_rust(&e))?;
                result.insert(
                    rust::MapKey::from_value(value_to_rust(&key, arena, descriptor_pool, message_factory)?)
                        .map_err(|v| rust::Error::invalid_argument(v.to_string()))?,
                    value_to_rust(&value, arena, descriptor_pool, message_factory)?,
                );
            }
            Ok(rust::Value::Map(result))
        }
        ValueKind::Unknown => {
            todo!()
        }
        ValueKind::Type => {
            let type_value = value.get_type();
            Ok(rust::Value::Type(type_to_rust(type_value.native_value())))
        }
        ValueKind::Error => {
            let error_value = value.get_error();
            Ok(rust::Value::Error(error_to_rust(&error_value.native_value())))
        }
        ValueKind::Opaque => {
            if value.is_optional() {
                let optional_value = value.get_optional();
                let result = if optional_value.has_value() {
                    rust::Optional::new(
                        value_to_rust(
                            &optional_value.get_value(),
                            arena,
                            descriptor_pool,
                            message_factory,
                        )?
                    )
                } else {
                    rust::Optional::none()
                };
                Ok(rust::Value::Optional(result))
            } else {
                let opaque_value = value.get_opaque();
                let result = opaque_value.downcast::<FfiOpaqueValueImpl>()
                    .map(|v| v.0)
                    .ok_or(rust::Error::internal("Failed to downcast opaque value"))?;
                Ok(rust::Value::Opaque(result))
            }
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub(crate) struct FfiOpaqueValueImpl(pub rust::OpaqueValue);
impl std::fmt::Display for FfiOpaqueValueImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::cmp::PartialEq for FfiOpaqueValueImpl {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl FfiOpaqueValue for FfiOpaqueValueImpl {
    fn opaque_type<'a>(&self, arena: &'a Arena, descriptor_pool: &'a DescriptorPool) -> OpaqueType<'a> {
        super::opaque_type_from_rust(&self.0.opaque_type(), arena, descriptor_pool)
    }
}

pub(crate) fn opaque_value_from_rust<'a>(
    value: &rust::OpaqueValue,
    arena: &'a Arena,
    descriptor_pool: &'a DescriptorPool
) -> cxx::UniquePtr<OpaqueValue<'a>> {
    let ffi = FfiOpaqueValueImpl(value.clone());
    OpaqueValue::new(arena, descriptor_pool, ffi)
}

fn optional_value_from_rust<'a>(
    value: &rust::Optional<rust::Value>,
    arena: &'a Arena,
    descriptor_pool: &'a DescriptorPool,
    message_factory: &'a MessageFactory,
) -> cxx::UniquePtr<OptionalValue<'a>> {
    match value.as_option() {
        Some(v) => {
            let value = value_from_rust(v, arena, descriptor_pool, message_factory);
            OptionalValue::new(arena, &value)
        }
        None => OptionalValue::none(),
    }
}
