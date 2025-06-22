use crate::values::*;
use super::impl_into;

impl_into!(
    Opaque: Value {
        OpaqueValue => |self| self,
    }
);


impl FromValue for OpaqueValue {
    type Output<'a> = OpaqueValue;

    fn from_value<'a>(value: &'a Value) -> Result<Self::Output<'a>, FromValueError> {
        match value {
            Value::Opaque(v) => Ok(v.clone()),
            _ => Err(FromValueError::new(value.clone(), "opaque")),
        }
    }
}

impl FromValue for &OpaqueValue {
    type Output<'a> = &'a OpaqueValue;

    fn from_value<'a>(value: &'a Value) -> Result<Self::Output<'a>, FromValueError> {
        match value {
            Value::Opaque(v) => Ok(v),
            _ => Err(FromValueError::new(value.clone(), "opaque")),
        }
    }
}
