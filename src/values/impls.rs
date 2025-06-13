use super::*;

macro_rules! impl_value_for_mapkey {
    ($ty:ty) => {
        const _:() = {
            impl TypedValue for $ty {
                fn value_type() -> Type {
                    <$ty as TypedMapKey>::mapkey_type().into()
                }
            }

            impl IntoValue for $ty {
                fn into_value(self) -> Value {
                    self.into_mapkey().into_value()
                }
            }

            impl FromValue for $ty {
                fn from_value(value: Value) -> Result<Self, Value> {
                    let mapkey = MapKey::from_value(value)?;
                    Self::from_mapkey(mapkey)
                        .map_err(|mapkey| mapkey.into_value())
                }
            }
        };
    };
}

macro_rules! impl_std_convert_for_value {
    ($(<$($generics:ident),+>)? for $ty:ty $(where $($bounds:tt)+)?) => {
        impl_std_convert_for_value!([Value] $(<$($generics),+>)? for $ty $(where $($bounds)+)?);
    };
    ([Value] $(<$($generics:ident),+>)? for $ty:ty $(where $($bounds:tt)+)?) => {
        impl$(<$($generics),+>)? From<$ty> for Value $(where $($bounds)+)? {
            fn from(value: $ty) -> Self {
                value.into_value()
            }
        }
        
        impl$(<$($generics),+>)? TryFrom<Value> for $ty $(where $($bounds)+)? {
            type Error = Value;
            fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
                Self::from_value(value)
            }
        }
    };
    ($ty:ty) => {
        impl_std_convert_for_value!(for $ty);
    };
    ([$cel:ty] $ty:ty) => {
        impl_std_convert_for_value!([$cel] for $ty);
    };
}
macro_rules! impl_std_convert_for_mapkey {
    ($(<$($generics:ident),+>)? for $ty:ty $(where $($bounds:tt)+)?) => {
        impl_std_convert_for_mapkey!([MapKey] $(<$($generics),+>)? for $ty $(where $($bounds)+)?);
    };
    ([MapKey] $(<$($generics:ident),+>)? for $ty:ty $(where $($bounds:tt)+)?) => {
        impl$(<$($generics),+>)? From<$ty> for MapKey $(where $($bounds)+)? {
            fn from(value: $ty) -> Self {
                value.into_mapkey()
            }
        }
        
        impl$(<$($generics),+>)? TryFrom<MapKey> for $ty $(where $($bounds)+)? {
            type Error = MapKey;
            fn try_from(value: MapKey) -> Result<Self, <Self as TryFrom<MapKey>>::Error> {
                Self::from_mapkey(value)
            }
        }
    };
    ($ty:ty) => {
        impl_std_convert_for_mapkey!(for $ty);
    };
    ([$cel:ty] $ty:ty) => {
        impl_std_convert_for_mapkey!([$cel] for $ty);
    };
}


/// CelValue for ()
const _:() = {
    impl TypedValue for () {
        fn value_type() -> Type { Type::Null }
    }
    impl IntoValue for () {
        fn into_value(self) -> Value {
            Value::Null
        }
    }
    impl FromValue for () {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Null => Ok(()),
                _ => Err(value),
            }
        }
    }
    impl_std_convert_for_value!(());
};

/// Cel for bool
const _:() = {
    impl TypedMapKey for bool { fn mapkey_type() -> MapKeyType { MapKeyType::Bool } }
    impl IntoMapKey for bool {
        fn into_mapkey(self) -> MapKey {
            MapKey::Bool(self)
        }
    }
    impl FromMapKey for bool {
        fn from_mapkey(mapkey: MapKey) -> Result<Self, MapKey> {
            match mapkey {
                MapKey::Bool(v) => Ok(v),
                _ => Err(mapkey),
            }
        }
    }
    impl_value_for_mapkey!(bool);
    impl_std_convert_for_mapkey!(bool);
    impl_std_convert_for_value!(bool);
};

/// Cel for i64
const _:() = {
    impl TypedMapKey for i64 { fn mapkey_type() -> MapKeyType { MapKeyType::Int } }
    impl IntoMapKey for i64 {
        fn into_mapkey(self) -> MapKey {
            MapKey::Int(self)
        }
    }
    impl FromMapKey for i64 {
        fn from_mapkey(mapkey: MapKey) -> Result<Self, MapKey> {
            match mapkey {
                MapKey::Int(v) => Ok(v),
                _ => Err(mapkey),
            }
        }
    }
    impl_value_for_mapkey!(i64);
    impl_std_convert_for_mapkey!(i64);
    impl_std_convert_for_value!(i64);
};

/// Cel for i32
const _:() = {
    impl TypedMapKey for i32 { fn mapkey_type() -> MapKeyType { MapKeyType::Int } }
    impl IntoMapKey for i32 {
        fn into_mapkey(self) -> MapKey {
            MapKey::Int(self as i64)
        }
    }
    impl FromMapKey for i32 {
        fn from_mapkey(mapkey: MapKey) -> Result<Self, MapKey> {
            i64::from_mapkey(mapkey)
                .and_then(|v| Self::try_from(v).map_err(|_| MapKey::Int(v)))
        }
    }
    impl_value_for_mapkey!(i32);
    impl_std_convert_for_mapkey!(i32);
    impl_std_convert_for_value!(i32);
};

/// Cel for i16
const _:() = {
    impl TypedMapKey for i16 { fn mapkey_type() -> MapKeyType { MapKeyType::Int } }
    impl IntoMapKey for i16 {
        fn into_mapkey(self) -> MapKey {
            MapKey::Int(self as i64)
        }
    }
    impl FromMapKey for i16 {
        fn from_mapkey(mapkey: MapKey) -> Result<Self, MapKey> {
            i64::from_mapkey(mapkey)
                .and_then(|v| Self::try_from(v).map_err(|_| MapKey::Int(v)))
        }
    }
    impl_value_for_mapkey!(i16);
    impl_std_convert_for_mapkey!(i16);
    impl_std_convert_for_value!(i16);
};

/// Cel for u64
const _:() = {
    impl TypedMapKey for u64 { fn mapkey_type() -> MapKeyType { MapKeyType::Uint } }
    impl IntoMapKey for u64 {
        fn into_mapkey(self) -> MapKey {
            MapKey::Uint(self)
        }
    }
    impl FromMapKey for u64 {
        fn from_mapkey(mapkey: MapKey) -> Result<Self, MapKey> {
            match mapkey {
                MapKey::Uint(v) => Ok(v),
                _ => Err(mapkey),
            }
        }
    }
    impl_value_for_mapkey!(u64);
    impl_std_convert_for_mapkey!(u64);
    impl_std_convert_for_value!(u64);
};

/// Cel for u32
const _:() = {
    impl TypedMapKey for u32 { fn mapkey_type() -> MapKeyType { MapKeyType::Uint } }
    impl IntoMapKey for u32 {
        fn into_mapkey(self) -> MapKey {
            MapKey::Uint(self as u64)
        }
    }
    impl FromMapKey for u32 {
        fn from_mapkey(mapkey: MapKey) -> Result<Self, MapKey> {
            u64::from_mapkey(mapkey)
                .and_then(|v| Self::try_from(v).map_err(|_| MapKey::Uint(v)))
        }
    }
    impl_value_for_mapkey!(u32);
    impl_std_convert_for_mapkey!(u32);
    impl_std_convert_for_value!(u32);
};

/// Cel for u16
const _:() = {
    impl TypedMapKey for u16 { fn mapkey_type() -> MapKeyType { MapKeyType::Uint } }
    impl IntoMapKey for u16 {
        fn into_mapkey(self) -> MapKey {
            MapKey::Uint(self as u64)
        }
    }
    impl FromMapKey for u16 {
        fn from_mapkey(mapkey: MapKey) -> Result<Self, MapKey> {
            u64::from_mapkey(mapkey)
                .and_then(|v| Self::try_from(v).map_err(|_| MapKey::Uint(v)))
        }
    }
    impl_value_for_mapkey!(u16);
    impl_std_convert_for_mapkey!(u16);
    impl_std_convert_for_value!(u16);
};

/// Cel for usize
const _:() = {
    impl TypedMapKey for usize { fn mapkey_type() -> MapKeyType { MapKeyType::Uint } }
    impl IntoMapKey for usize {
        fn into_mapkey(self) -> MapKey {
            MapKey::Uint(self as u64)
        }
    }
    impl FromMapKey for usize {
        fn from_mapkey(mapkey: MapKey) -> Result<Self, MapKey> {
            u64::from_mapkey(mapkey)
                .and_then(|v| Self::try_from(v).map_err(|_| MapKey::Uint(v)))
        }
    }
    impl_value_for_mapkey!(usize);
    impl_std_convert_for_mapkey!(usize);
    impl_std_convert_for_value!(usize);
};

/// Cel for isize
const _:() = {
    impl TypedMapKey for isize { fn mapkey_type() -> MapKeyType { MapKeyType::Int } }
    impl IntoMapKey for isize {
        fn into_mapkey(self) -> MapKey {
            MapKey::Int(self as i64)
        }
    }
    impl FromMapKey for isize {
        fn from_mapkey(mapkey: MapKey) -> Result<Self, MapKey> {
            i64::from_mapkey(mapkey)
                .and_then(|v| Self::try_from(v).map_err(|_| MapKey::Int(v)))
        }
    }
    impl_value_for_mapkey!(isize);
    impl_std_convert_for_mapkey!(isize);
    impl_std_convert_for_value!(isize);
};

/// Cel for String
const _:() = {
    impl TypedMapKey for String { fn mapkey_type() -> MapKeyType { MapKeyType::String } }
    impl IntoMapKey for String {
        fn into_mapkey(self) -> MapKey {
            MapKey::String(self)
        }
    }
    impl FromMapKey for String {
        fn from_mapkey(mapkey: MapKey) -> Result<Self, MapKey> {
            match mapkey {
                MapKey::String(v) => Ok(v),
                _ => Err(mapkey),
            }
        }
    }
    impl_value_for_mapkey!(String);
    impl_std_convert_for_mapkey!(String);
    impl_std_convert_for_value!(String);
};

/// Cel for f64
const _:() = {
    impl TypedValue for f64 {
        fn value_type() -> Type { Type::Double }
    }
    impl IntoValue for f64 {
        fn into_value(self) -> Value {
            Value::Double(self)
        }
    }
    impl FromValue for f64 {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Double(v) => Ok(v),
                _ => Err(value),
            }
        }
    }
    impl_std_convert_for_value!(f64);
};

/// Cel for f32
const _:() = {
    impl TypedValue for f32 {
        fn value_type() -> Type { Type::Double }
    }
    impl IntoValue for f32 {
        fn into_value(self) -> Value {
            Value::Double(self as f64)
        }
    }
    impl FromValue for f32 {
        fn from_value(value: Value) -> Result<Self, Value> {
            f64::from_value(value)
                .map(|v| v as f32)
        }
    }
    impl_std_convert_for_value!(f32);
};

/// Cel for `Vec<u8>`
const _:() = {
    impl TypedValue for Vec<u8> {
        fn value_type() -> Type { Type::Bytes }
    }
    impl IntoValue for Vec<u8> {
        fn into_value(self) -> Value {
            Value::Bytes(self)
        }
    }
    impl FromValue for Vec<u8> {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Bytes(v) => Ok(v),
                _ => Err(value),
            }
        }
    }
    impl_std_convert_for_value!(Vec<u8>);
};

/// Cel for chrono::Duration
const _:() = {
    impl TypedValue for chrono::Duration {
        fn value_type() -> Type { Type::Duration }
    }
    impl IntoValue for chrono::Duration {
        fn into_value(self) -> Value {
            Value::Duration(self)
        }
    }
    impl FromValue for chrono::Duration {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Duration(v) => Ok(v),
                _ => Err(value),
            }
        }
    }
    impl_std_convert_for_value!(chrono::Duration);
};

/// Cel for chrono::DateTime<chrono::Utc>
const _:() = {
    impl TypedValue for chrono::DateTime<chrono::Utc> {
        fn value_type() -> Type { Type::Timestamp }
    }
    impl IntoValue for chrono::DateTime<chrono::Utc> {
        fn into_value(self) -> Value {
            Value::Timestamp(self)
        }
    }
    impl FromValue for chrono::DateTime<chrono::Utc> {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Timestamp(v) => Ok(v),
                _ => Err(value),
            }
        }
    }
    impl_std_convert_for_value!(chrono::DateTime<chrono::Utc>);
};

/// Cel for std::time::SystemTime
const _:() = {
    impl TypedValue for std::time::SystemTime {
        fn value_type() -> Type { Type::Timestamp }
    }
    impl IntoValue for std::time::SystemTime {
        fn into_value(self) -> Value {
            Value::Timestamp(self.into())
        }
    }
    impl FromValue for std::time::SystemTime {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Timestamp(v) => Ok(v.into()),
                _ => Err(value),
            }
        }
    }
    impl_std_convert_for_value!(std::time::SystemTime);
};

/// Cel for `Vec<T>`
const _:() = {
    impl<T> TypedValue for Vec<T> where T: TypedValue {
        fn value_type() -> Type { Type::List(ListType::new(T::value_type())) }
    }
    impl<T> IntoValue for Vec<T> where T: IntoValue {
        fn into_value(self) -> Value {
            Value::List(self.into_iter().map(|v| v.into_value()).collect())
        }
    }
    impl<T> FromValue for Vec<T> where T: FromValue {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::List(list) => {
                    let convert = || -> Option<_> {
                        let mut result = Vec::new();
                        for v in list.iter() {
                            let v = T::from_value(v.clone()).ok()?;
                            result.push(v);
                        }
                        Some(result)
                    };
                    convert().ok_or(Value::List(list))
                }
                _ => Err(value),
            }
        }
    }
    impl<T: IntoValue> From<Vec<T>> for Value {
        fn from(value: Vec<T>) -> Self {
            value.into_value()
        }
    }

    impl<T: FromValue> TryFrom<Value> for Vec<T> {
        type Error = Value;
        fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
            Self::from_value(value)
        }
    }
};

/// Cel for HashMap<K, V>
const _:() = {
    use std::collections::HashMap;
    impl<K: TypedMapKey, V: TypedValue> TypedValue for HashMap<K, V> {
        fn value_type() -> Type { Type::Map(MapType::new(K::mapkey_type(), V::value_type())) }
    }
    impl<K: IntoMapKey, V: IntoValue> IntoValue for HashMap<K, V> {
        fn into_value(self) -> Value {
            Value::Map(self.into_iter().map(|(k, v)| (k.into_mapkey(), v.into_value())).collect())
        }
    }
    impl<K: FromMapKey, V: FromValue> FromValue for HashMap<K, V> {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Map(map) => {
                    let convert = || -> Option<_> {
                        let mut result = HashMap::new();
                        for (k, v) in map.iter() {
                            let k = K::from_mapkey(k.clone()).ok()?;
                            let v = V::from_value(v.clone()).ok()?;
                            result.insert(k, v);
                        }
                        Some(result)
                    };
                    convert().ok_or(Value::Map(map))
                }
                _ => Err(value),
            }
        }
    }
    impl<K: IntoMapKey, V: IntoValue> From<HashMap<K, V>> for Value {
        fn from(value: HashMap<K, V>) -> Self {
            value.into_value()
        }
    }
    impl<K: FromMapKey, V: FromValue> TryFrom<Value> for HashMap<K, V> {
        type Error = Value;
        fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
            Self::from_value(value)
        }
    }
};

/// Cel for BTreeMap<K, V>
const _:() = {
    use std::collections::BTreeMap;
    impl<K: TypedMapKey, V: TypedValue> TypedValue for BTreeMap<K, V> {
        fn value_type() -> Type { Type::Map(MapType::new(K::mapkey_type(), V::value_type())) }
    }
    impl<K: IntoMapKey, V: IntoValue> IntoValue for BTreeMap<K, V> {
        fn into_value(self) -> Value {
            Value::Map(self.into_iter().map(|(k, v)| (k.into_mapkey(), v.into_value())).collect())
        }
    }
    impl<K: FromMapKey, V: FromValue> FromValue for BTreeMap<K, V> {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Map(map) => {
                    let convert = || -> Option<_> {
                        let mut result = BTreeMap::new();
                        for (k, v) in map.iter() {
                            let k = K::from_mapkey(k.clone()).ok()?;
                            let v = V::from_value(v.clone()).ok()?;
                            result.insert(k, v);
                        }
                        Some(result)
                    };
                    convert().ok_or(Value::Map(map))
                }
                _ => Err(value),
            }
        }
    }
    impl<K: IntoMapKey, V: IntoValue> From<BTreeMap<K, V>> for Value {
        fn from(value: BTreeMap<K, V>) -> Self {
            value.into_value()
        }
    }
    impl<K: FromMapKey, V: FromValue> TryFrom<Value> for BTreeMap<K, V> {
        type Error = Value;
        fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
            Self::from_value(value)
        }
    }
};

/// Cel for Type
const _:() = {
    impl TypedValue for Type {
        fn value_type() -> Type { Type::Type(TypeType::new(None)) }
    }
    impl IntoValue for Type {
        fn into_value(self) -> Value {
            Value::Type(self)
        }
    }
    impl FromValue for Type {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Type(ty) => Ok(ty),
                _ => Err(value),
            }
        }
    }
    impl_std_convert_for_value!(Type);
};

/// Cel for Error
const _:() = {
    impl TypedValue for Error {
        fn value_type() -> Type { Type::Error }
    }
    impl IntoValue for Error {
        fn into_value(self) -> Value { Value::Error(self) }
    }
    impl FromValue for Error {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Error(err) => Ok(err),
                _ => Err(value),
            }
        }
    }
    impl_std_convert_for_value!(Error);
};

/// Cel for `Opaque<T>`
const _:() = {
    impl<T: TypedOpaqueValue> TypedValue for Opaque<T> {
        fn value_type() -> Type { Type::Opaque(<T as TypedOpaqueValue>::opaque_type()) }
    }
    impl<T: TypedOpaqueValue> IntoValue for Opaque<T> {
        fn into_value(self) -> Value {
            let dyn_value: Opaque = self.into();
            Value::Opaque(dyn_value)
        }
    }
    impl<T: TypedOpaqueValue> FromValue for Opaque<T> {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Opaque(opaque) => {
                    Ok(
                        opaque.downcast::<T>()
                            .map_err(Value::Opaque)?
                    )
                }
                _ => Err(value),
            }
        }
    }
    impl<T: TypedOpaqueValue> From<Opaque<T>> for Value {
        fn from(value: Opaque<T>) -> Self {
            value.into_value()
        }
    }
    impl<T: TypedOpaqueValue> TryFrom<Value> for Opaque<T> {
        type Error = Value;
        fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
            Self::from_value(value)
        }
    }
};

/// Cel for `Optional<T>`
const _:() = {
    impl<T: TypedValue> TypedValue for Optional<T> {
        fn value_type() -> Type { Type::Optional(OptionalType::new(T::value_type())) }
    }
    impl<T: IntoValue> IntoValue for Optional<T> {
        fn into_value(self) -> Value {
            Value::Optional(self.map(|v| v.into_value()))
        }
    }
    impl<T: FromValue> FromValue for Optional<T> {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Optional(o) => {
                    o.map(|v|
                        T::from_value(v)
                            .map_err(|v| Value::Optional(Optional::new(v)))
                    )
                    .transpose()
                }
                _ => Err(value),
            }
        }
    }
    impl<T: IntoValue> From<Optional<T>> for Value {
        fn from(value: Optional<T>) -> Self {
            value.into_value()
        }
    }
    impl<T: FromValue> TryFrom<Value> for Optional<T> {
        type Error = Value;
        fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
            Self::from_value(value)
        }
    }
};

macro_rules! impl_cel_for_option {
    ($(<$($generics:ident),+>)? for $ty:ty $(where $($bounds:tt)+)?) => {
        impl$(<$($generics),+>)? TypedValue for Option<$ty> $(where $($bounds)+)? {
            fn value_type() -> Type { Type::Optional(OptionalType::new(<$ty as TypedValue>::value_type())) }
        }
        impl$(<$($generics),+>)? IntoValue for Option<$ty> $(where $($bounds)+)? {
            fn into_value(self) -> Value {
                let option_value = self.map(|v| v.into_value());
                Value::Optional(Optional::from_option(option_value))
            }
        }
        impl$(<$($generics),+>)? FromValue for Option<$ty> $(where $($bounds)+)? {
            fn from_value(value: Value) -> Result<Self, Value> {
                match value {
                    Value::Optional(o) => {
                        o.into_option()
                            .map(|v| <$ty as FromValue>::from_value(v))
                            .transpose()
                            .map_err(|v| Value::Optional(Optional::new(v)))
                    }
                    _ => Err(value),
                }
            }
        }
        impl_std_convert_for_value!($(<$($generics),+>)? for Option<$ty> $(where $($bounds)+)?);
    };
    ($ty:ty) => {
        impl_cel_for_option!(for $ty);
    };
}

const _:() = {
    impl_cel_for_option!(());
    impl_cel_for_option!(bool);
    impl_cel_for_option!(i64);
    impl_cel_for_option!(i32);
    impl_cel_for_option!(i16);
    impl_cel_for_option!(u64);
    impl_cel_for_option!(u32);
    impl_cel_for_option!(u16);
    impl_cel_for_option!(String);
    impl_cel_for_option!(Vec<u8>);
    impl_cel_for_option!(chrono::Duration);
    impl_cel_for_option!(chrono::DateTime<chrono::Utc>);
    impl_cel_for_option!(std::time::SystemTime);
    impl_cel_for_option!(f64);
    impl_cel_for_option!(f32);
    impl_cel_for_option!(Type);
    impl_cel_for_option!(Error);
};

const _:() = {
    impl<T: TypedValue> TypedValue for Option<Vec<T>> {
        fn value_type() -> Type { Type::Optional(OptionalType::new(Vec::<T>::value_type())) }
    }
    impl<T: IntoValue> IntoValue for Option<Vec<T>> {
        fn into_value(self) -> Value {
            let option_value = self.map(|v| v.into_value());
            Value::Optional(Optional::from_option(option_value))
        }
    }
    impl<T: FromValue> FromValue for Option<Vec<T>> {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Optional(o) => {
                    o.into_option()
                        .map(|v| Vec::<T>::from_value(v))
                        .transpose()
                        .map_err(|v| Value::Optional(Optional::new(v)))
                }
                _ => Err(value),
            }
        }
    }
    impl<T: IntoValue> From<Option<Vec<T>>> for Value {
        fn from(value: Option<Vec<T>>) -> Self {
            value.into_value()
        }
    }
    impl<T: FromValue> TryFrom<Value> for Option<Vec<T>> {
        type Error = Value;
        fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
            Self::from_value(value)
        }
    }
};

const _:() = {
    use std::collections::HashMap;
    impl<K: TypedMapKey, V: TypedValue> TypedValue for Option<HashMap<K, V>> {
        fn value_type() -> Type { Type::Optional(OptionalType::new(HashMap::<K, V>::value_type())) }
    }
    impl<K: IntoMapKey, V: IntoValue> IntoValue for Option<HashMap<K, V>> {
        fn into_value(self) -> Value {
            let option_value = self.map(|v| v.into_value());
            Value::Optional(Optional::from_option(option_value))
        }
    }
    impl<K: FromMapKey, V: FromValue> FromValue for Option<HashMap<K, V>> {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Optional(o) => {
                    o.into_option()
                        .map(|v| HashMap::<K, V>::from_value(v))
                        .transpose()
                        .map_err(|v| Value::Optional(Optional::new(v)))
                }
                _ => Err(value),
            }
        }
    }
    impl<K: IntoMapKey, V: IntoValue> From<Option<HashMap<K, V>>> for Value {
        fn from(value: Option<HashMap<K, V>>) -> Self {
            value.into_value()
        }
    }
    impl<K: FromMapKey, V: FromValue> TryFrom<Value> for Option<HashMap<K, V>> {
        type Error = Value;
        fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
            Self::from_value(value)
        }
    }
};

const _:() = {
    use std::collections::BTreeMap;
    impl<K: TypedMapKey, V: TypedValue> TypedValue for Option<BTreeMap<K, V>> {
        fn value_type() -> Type { Type::Optional(OptionalType::new(BTreeMap::<K, V>::value_type())) }
    }
    impl<K: IntoMapKey, V: IntoValue> IntoValue for Option<BTreeMap<K, V>> {
        fn into_value(self) -> Value {
            let option_value = self.map(|v| v.into_value());
            Value::Optional(Optional::from_option(option_value))
        }
    }
    impl<K: FromMapKey, V: FromValue> FromValue for Option<BTreeMap<K, V>> {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Optional(o) => {
                    o.into_option()
                        .map(|v| BTreeMap::<K, V>::from_value(v))
                        .transpose()
                        .map_err(|v| Value::Optional(Optional::new(v)))
                }
                _ => Err(value),
            }
        }
    }
    impl<K: IntoMapKey, V: IntoValue> From<Option<BTreeMap<K, V>>> for Value {
        fn from(value: Option<BTreeMap<K, V>>) -> Self {
            value.into_value()
        }
    }
    impl<K: FromMapKey, V: FromValue> TryFrom<Value> for Option<BTreeMap<K, V>> {
        type Error = Value;
        fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
            Self::from_value(value)
        }
    }
};

const _:() = {
    impl<T: TypedOpaqueValue> TypedValue for Option<Opaque<T>> {
        fn value_type() -> Type { Type::Opaque(<T as TypedOpaqueValue>::opaque_type()) }
    }
    impl<T: TypedOpaqueValue> IntoValue for Option<Opaque<T>> {
        fn into_value(self) -> Value {
            let option_value = self.map(|v| v.into_value());
            Value::Optional(Optional::from_option(option_value))
        }
    }
    impl<T: TypedOpaqueValue> FromValue for Option<Opaque<T>> {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Optional(o) => {
                    o.into_option()
                        .map(|v| Opaque::<T>::from_value(v))
                        .transpose()
                        .map_err(|v| Value::Optional(Optional::new(v)))
                }
                _ => Err(value),
            }
        }
    }
    impl<T: TypedOpaqueValue> From<Option<Opaque<T>>> for Value {
        fn from(value: Option<Opaque<T>>) -> Self {
            value.into_value()
        }
    }
    impl<T: TypedOpaqueValue> TryFrom<Value> for Option<Opaque<T>> {
        type Error = Value;
        fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
            Self::from_value(value)
        }
    }
};

const _:() = {
    impl<T: TypedValue> TypedValue for Option<Option<T>> where Option<T>: TypedValue {
        fn value_type() -> Type { Type::Optional(OptionalType::new(Option::<T>::value_type())) }
    }
    impl<T: IntoValue> IntoValue for Option<Option<T>> where Option<T>: IntoValue {
        fn into_value(self) -> Value {
            let option_value = self.map(|v| v.into_value());
            Value::Optional(Optional::from_option(option_value))
        }
    }
    impl<T: FromValue> FromValue for Option<Option<T>> where Option<T>: FromValue {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Optional(o) => {
                    o.into_option()
                        .map(|v| Option::<T>::from_value(v))
                        .transpose()
                        .map_err(|v| Value::Optional(Optional::new(v)))
                }
                _ => Err(value),
            }
        }
    }
    impl<T: IntoValue> From<Option<Option<T>>> for Value where Option<T>: IntoValue {
        fn from(value: Option<Option<T>>) -> Self {
            value.into_value()
        }
    }
    impl<T: FromValue> TryFrom<Value> for Option<Option<T>> where Option<T>: FromValue {
        type Error = Value;
        fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
            Self::from_value(value)
        }
    }
};

const _:() = {
    impl<T: TypedValue> TypedValue for Option<Optional<T>> {
        fn value_type() -> Type { Type::Optional(OptionalType::new(Optional::<T>::value_type())) }
    }
    impl<T: IntoValue> IntoValue for Option<Optional<T>> {
        fn into_value(self) -> Value {
            let option_value = self.map(|v| v.into_value());
            Value::Optional(Optional::from_option(option_value))
        }
    }
    impl<T: FromValue> FromValue for Option<Optional<T>> {
        fn from_value(value: Value) -> Result<Self, Value> {
            match value {
                Value::Optional(o) => {
                    o.into_option()
                        .map(|v| Optional::<T>::from_value(v))
                        .transpose()
                        .map_err(|v| Value::Optional(Optional::new(v)))
                }
                _ => Err(value),
            }
        }
    }
    impl<T: IntoValue> From<Option<Optional<T>>> for Value where Option<T>: IntoValue {
        fn from(value: Option<Optional<T>>) -> Self {
            value.into_value()
        }
    }
    impl<T: FromValue> TryFrom<Value> for Option<Optional<T>> where Option<T>: FromValue {
        type Error = Value;
        fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
            Self::from_value(value)
        }
    }
};
