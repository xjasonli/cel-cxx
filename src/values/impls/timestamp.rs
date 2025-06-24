use super::{impl_from, impl_into, impl_typed};
use crate::{types::*, values::*};

use std::time::SystemTime;

impl_typed!(
    Timestamp: Value {
        Timestamp,
        SystemTime
    }
);

impl_into!(
    Timestamp: Value, Constant {
        Timestamp => |self| self,
        SystemTime => |self| self.into(),
    }
);

impl_from!(
    Timestamp: Value {
        Timestamp => |v| *v,
        SystemTime => |v| (*v).into(),
    }
);
