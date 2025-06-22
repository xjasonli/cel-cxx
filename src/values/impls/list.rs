use crate::{types::*, values::*};
use super::*;

use std::collections::{VecDeque, LinkedList};

impl_typed!(
    List: Value {
        @[T: TypedValue]
        Vec<T> => ListType::new(T::value_type()),

        @[T: TypedValue]
        VecDeque<T> => ListType::new(T::value_type()),

        @[T: TypedValue]
        LinkedList<T> => ListType::new(T::value_type()),

        @[T: TypedValue]
        [T] => ListType::new(T::value_type()),
    }
);

impl_into!(
    List: Value {
        @[T: IntoValue]
        Vec<T> => |self|
            self.into_iter()
                .map(IntoValue::into_value)
                .collect(),

        @[T: IntoValue + Clone]
        &[T] => |self|
            self.iter()
                .map(IntoValue::into_value)
                .collect(),

        @[T: IntoValue]
        VecDeque<T> => |self| 
            self.into_iter()
                .map(IntoValue::into_value)
                .collect(),

        @[T: IntoValue]
        LinkedList<T> => |self|
            self.into_iter()
                .map(IntoValue::into_value)
                .collect(),
    }
);

impl_from_list!(
    #[adder = push]
    Vec,

    #[adder = push_back]
    VecDeque,

    #[adder = push_back]
    LinkedList,
);
