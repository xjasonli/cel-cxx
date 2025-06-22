use crate::{types::*, values::*};
use super::{impl_typed, impl_into, impl_from};

impl_typed!(
    Type: Value {
        ValueType => TypeType::new(None),
    }
);

impl_into!(
    Type: Value {
        ValueType => |self| self,
    }
);

impl_from!(
    Type: Value {
        ValueType => |v| v.clone(),
        &ValueType as &'a ValueType => |v| v,
    }
);
