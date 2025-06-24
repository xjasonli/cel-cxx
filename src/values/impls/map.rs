use super::*;
use crate::{types::*, values::*};

use std::collections::{BTreeMap, HashMap, LinkedList, VecDeque};

impl_typed!(
    Map: Value {
        @[K: TypedMapKey, V: TypedValue]
        HashMap<K, V> => MapType::new(K::mapkey_type(), V::value_type()),

        @[K: TypedMapKey, V: TypedValue]
        BTreeMap<K, V> => MapType::new(K::mapkey_type(), V::value_type()),

        @[K: TypedMapKey, V: TypedValue]
        [(K, V)] => MapType::new(K::mapkey_type(), V::value_type()),

        @[K: TypedMapKey, V: TypedValue]
        Vec<(K, V)> => MapType::new(K::mapkey_type(), V::value_type()),

        @[K: TypedMapKey, V: TypedValue]
        VecDeque<(K, V)> => MapType::new(K::mapkey_type(), V::value_type()),

        @[K: TypedMapKey, V: TypedValue]
        LinkedList<(K, V)> => MapType::new(K::mapkey_type(), V::value_type()),
    }
);

impl_into!(
    Map: Value {
        @[K: IntoMapKey, V: IntoValue]
        HashMap<K, V> => |self|
            self.into_iter()
                .map(|(k, v)| (k.into_mapkey(), v.into_value()))
                .collect(),

        @[K: IntoMapKey, V: IntoValue]
        BTreeMap<K, V> => |self|
            self.into_iter()
                .map(|(k, v)| (k.into_mapkey(), v.into_value()))
                .collect(),

        @[K: IntoMapKey + Clone, V: IntoValue + Clone]
        &[(K, V)] => |self|
            self.iter()
                .map(|(k, v)| (k.into_mapkey(), v.into_value()))
                .collect(),

        @[K: IntoMapKey, V: IntoValue]
        Vec<(K, V)> => |self|
            self.into_iter()
                .map(|(k, v)| (k.into_mapkey(), v.into_value()))
                .collect(),

        @[K: IntoMapKey, V: IntoValue]
        VecDeque<(K, V)> => |self|
            self.into_iter()
                .map(|(k, v)| (k.into_mapkey(), v.into_value()))
                .collect(),

        @[K: IntoMapKey, V: IntoValue]
        LinkedList<(K, V)> => |self|
            self.into_iter()
                .map(|(k, v)| (k.into_mapkey(), v.into_value()))
                .collect(),
    }
);

type VecMap<K, V> = Vec<(K, V)>;
type VecDequeMap<K, V> = VecDeque<(K, V)>;
type LinkedListMap<K, V> = LinkedList<(K, V)>;

#[rustfmt::skip]
impl_from_map!(
    HashMap,

    BTreeMap,

    VecMap,

    VecDequeMap,

    LinkedListMap,
);
