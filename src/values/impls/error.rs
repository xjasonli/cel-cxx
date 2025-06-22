use crate::{types::*, values::*};
use super::{impl_typed, impl_into, impl_from};

impl_typed!(
    Error: Value {
        Error
    }
);

impl_into!(
    Error: Value {
        Error => |self| self,
    }
);

impl_from!(
    Error: Value {
        Error => |v| v.clone(),
        &Error as &'a Error => |v| v,
    }
);
