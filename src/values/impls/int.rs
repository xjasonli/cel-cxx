use super::{impl_from, impl_into, impl_typed};
use crate::{types::*, values::*};

impl_typed!(
    Int: Value, MapKey {
        i64,
        i32,
        i16,
        isize
    }
);

impl_into!(
    Int: Value, MapKey, Constant {
        i64 => |self| self,
        i32 => |self| self as i64,
        i16 => |self| self as i64,
        isize => |self| self as i64,
    }
);

impl_from!(
    Int: Value, MapKey {
        i64 => |v| *v,
        &i64 as &'a i64 => |v| v,
        i32 => |v| *v as i32,
        i16 => |v| *v as i16,
        isize => |v| *v as isize,
    }
);
