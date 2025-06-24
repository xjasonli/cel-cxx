use super::{impl_from, impl_into, impl_typed};
use crate::{types::*, values::*};

impl_typed!(
    Bool: Value, MapKey {
        bool
    }
);

impl_into!(
    Bool: Value, MapKey, Constant {
        bool => |self| self,
    }
);

impl_from!(
    Bool: Value, MapKey {
        bool => |v| *v,
        &bool as &'a bool => |v| v,
    }
);
