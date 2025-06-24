use super::{impl_from, impl_into, impl_typed};
use crate::{types::*, values::*};

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
