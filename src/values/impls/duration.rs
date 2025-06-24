use super::{impl_from, impl_into, impl_typed};
use crate::{types::*, values::*};

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
        Duration => |v| *v,
        &Duration as &'a Duration => |v| v,
    }
);
