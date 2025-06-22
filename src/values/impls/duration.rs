use crate::{types::*, values::*};
use super::{impl_typed, impl_into, impl_from};

impl_typed!(
    Duration: Value {
        Duration
    }
);

impl_into!(
    Duration: Value, Constant {
        Duration => |self| self,
    }
);

impl_from!(
    Duration: Value {
        Duration => |v| v.clone(),
        &Duration as &'a Duration => |v| v,
    }
);
