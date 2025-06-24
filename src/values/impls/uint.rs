use super::{impl_from, impl_into, impl_typed};
use crate::{types::*, values::*};

impl_typed!(
    Uint: Value, MapKey {
        u64,
        u32,
        u16,
        usize
    }
);

impl_into!(
    Uint: Value, MapKey, Constant {
        u64 => |self| self,
        u32 => |self| self as u64,
        u16 => |self| self as u64,
        usize => |self| self as u64,
    }
);

impl_from!(
    Uint: Value, MapKey {
        u64 => |v| *v,
        &u64 as &'a u64 => |v| v,
        u32 => |v| *v as u32,
        u16 => |v| *v as u16,
        usize => |v| *v as usize,
    }
);
