use crate::{types::*, values::*};

impl IntoValue for StructValue {
    fn into_value(self) -> Value {
        Value::Struct(self)
    }
}

impl TypedValue for StructValue {
    /// Returns `Dyn` because `TypedValue` is a static trait method with no access
    /// to the instance's `type_name`. The actual message type is resolved at the
    /// FFI layer during evaluation via the DescriptorPool.
    fn value_type() -> ValueType {
        ValueType::Dyn
    }
}

impl FromValue for StructValue {
    type Output<'a> = StructValue;

    fn from_value<'a>(value: &'a Value) -> Result<Self::Output<'a>, FromValueError> {
        match value {
            Value::Struct(s) => Ok(s.clone()),
            _ => Err(FromValueError::new(value.clone(), "struct")),
        }
    }
}

impl FromValue for &StructValue {
    type Output<'a> = &'a StructValue;

    fn from_value<'a>(value: &'a Value) -> Result<Self::Output<'a>, FromValueError> {
        match value {
            Value::Struct(s) => Ok(s),
            _ => Err(FromValueError::new(value.clone(), "struct")),
        }
    }
}
