use crate::{types::*, values::*};
use super::{impl_typed, impl_into, impl_from};

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
        Timestamp => |v| v.clone(),
        SystemTime => |v| (*v).into(),
    }
);
