use super::{impl_from, impl_into, impl_typed};
use crate::{types::*, values::*};

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
