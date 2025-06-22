use crate::{types::*, values::*};
use super::{impl_typed, impl_into, impl_from};

impl_typed!(
    Double: Value {
        f64,
        f32
    }
);

impl_into!(
    Double: Value, Constant {
        f64 => |self| self,
        f32 => |self| self as f64,
    }
);

impl_from!(
    Double: Value {
        f64 => |v| *v,
        &f64 as &'a f64 => |v| v,
        f32 => |v| *v as f32,
    }
);
