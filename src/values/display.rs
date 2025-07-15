use super::{MapKey, Value};
use itertools::Itertools;

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Uint(u) => write!(f, "{u}"),
            Value::Double(d) => write!(f, "{d}"),
            Value::String(s) => write!(f, "{s:?}"),
            Value::Bytes(b) => write!(f, "{}", display_bytes(b)),
            Value::Struct(_s) => write!(f, ""),
            Value::Duration(d) => write!(f, "{d}"),
            Value::Timestamp(t) => write!(f, "{t}"),
            Value::List(l) => write!(f, "[{}]", l.iter().format(", ")),
            Value::Map(m) => write!(
                f,
                "{{{}}}",
                m.iter()
                    .format_with(", ", |(k, v), f| { f(&format_args!("{k}: {v}")) })
            ),
            Value::Unknown(_u) => write!(f, ""),
            Value::Type(t) => write!(f, "{t}"),
            Value::Error(e) => write!(f, "{e}"),
            Value::Opaque(o) => write!(f, "{o}"),
            Value::Optional(opt) => write!(f, "{opt}"),
        }
    }
}

impl std::fmt::Display for MapKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapKey::Bool(b) => write!(f, "{b}"),
            MapKey::Int(i) => write!(f, "{i}"),
            MapKey::Uint(u) => write!(f, "{u}"),
            MapKey::String(s) => write!(f, "{s:?}"),
        }
    }
}

fn display_bytes<B: AsRef<[u8]>>(buf: B) -> String {
    String::from_utf8(
        buf.as_ref()
            .iter()
            .flat_map(|b| std::ascii::escape_default(*b))
            .collect(),
    )
    .unwrap()
}
