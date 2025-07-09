use super::{impl_from, impl_into, impl_typed};
use crate::{types::*, values::*};

impl_typed!(
    Bytes: Value {
        BytesValue,
        Vec<u8>,
        Box<[u8]>,
        [u8]
    }
);

impl_into!(
    Bytes: Value, Constant {
        BytesValue => |self| self,
        Vec<u8> => |self| self.into(),
        Box<[u8]> => |self| self.into(),
        &[u8] => |self| self.into(),
    }
);

impl_from!(
    Bytes: Value {
        BytesValue => |v| v.clone(),
        Vec<u8> => |v| v.to_vec(),
        Box<[u8]> => |v| Box::from(v.as_slice()),
        &BytesValue as &'a BytesValue => |v| v,
        &[u8] as &'a [u8] => |v| v.as_slice(),
    }
);
