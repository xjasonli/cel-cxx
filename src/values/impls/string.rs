use super::{impl_from, impl_into, impl_typed};
use crate::{types::*, values::*};

impl_typed!(
    String: Value, MapKey {
        ArcStr,
        String,
        Box<str>,
        str
    }
);

impl_into!(
    String: Value, MapKey, Constant {
        ArcStr => |self| self,
        String => |self| self.into(),
        Box<str> => |self| self.into(),
        &str => |self| self.into(),
    }
);

impl_from!(
    String: Value, MapKey {
        ArcStr => |v| v.clone(),
        String => |v| v.to_string(),
        Box<str> => |v| Box::from(v.as_slice()),
        &ArcStr as &'a ArcStr => |v| v,
        &str as &'a str => |v| v.as_slice(),
        //str: !Sized as &'a str => |v| v.as_slice(),
    }
);
