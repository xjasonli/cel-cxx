use crate::{types::*, values::*};
use super::{impl_typed, impl_into, impl_from};

impl_typed!(
    Null: Value {
        ()
    }
);

impl_into!(
    Null: Value, Constant {
        ()
    }
);

impl_from!(
    Null: Value {
        () => || (),
    }
);
