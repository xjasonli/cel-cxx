use crate::{types::*, values::*};
use super::{impl_typed, impl_into};

impl_typed!(
    Optional: Value {
        @[T: TypedValue]
        Optional<T> => OptionalType::new(T::value_type()),

        @[T: TypedValue]
        Option<T> => OptionalType::new(T::value_type()),
    }
);

impl_into!(
    Optional: Value {
        @[T: IntoValue]
        Optional<T> => |self| self.map(IntoValue::into_value),

        @[T: IntoValue]
        Option<T> => |self| self.map(IntoValue::into_value).into(),
    }
);

// FromValue for Optional<T>
impl<T: FromValue + TypedValue> FromValue for Optional<T> {
    type Output<'a> = Optional<T::Output<'a>>;

    fn from_value<'a>(value: &'a Value) -> Result<Self::Output<'a>, FromValueError> {
        match value {
            Value::Optional(o) => {
                o.as_ref()
                    .map(|v|
                        T::from_value(v)
                    )
                    .transpose()
            }
            _ => Err(FromValueError::new_typed::<Self>(value.clone())),
        }
    }
}

// TryFrom<&Value> for Optional<T>
impl<'a, T: TryFrom<&'a Value, Error = FromValueError> + TypedValue> TryFrom<&'a Value> for Optional<T> {
    type Error = FromValueError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Optional(o) => {
                o.as_ref()
                    .map(|v|
                        T::try_from(v)
                    )
                    .transpose()
            }
            _ => Err(FromValueError::new_typed::<Self>(value.clone())),
        }
    }
}

// TryFrom<Value> for Optional<T>
impl<T: TryFrom<Value, Error = FromValueError> + TypedValue> TryFrom<Value> for Optional<T> {
    type Error = FromValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Optional(o) => {
                o.map(|v|
                        T::try_from(v)
                    )
                    .transpose()
            }
            _ => Err(FromValueError::new_typed::<Self>(value)),
        }
    }
}

// FromValue for Option<T>
impl<T: FromValue + TypedValue> FromValue for Option<T> {
    type Output<'a> = Option<T::Output<'a>>;

    fn from_value<'a>(value: &'a Value) -> Result<Self::Output<'a>, FromValueError> {
        match value {
            Value::Optional(o) => {
                o.as_ref()
                    .map(|v|
                        T::from_value(v)
                    )
                    .transpose()
                    .map(|optional| optional.into())
            }
            _ => Err(FromValueError::new_typed::<Self>(value.clone())),
        }
    }
}

// TryFrom<&Value> for Option<T>
impl<'a, T: TryFrom<&'a Value, Error = FromValueError> + TypedValue> TryFrom<&'a Value> for Option<T> {
    type Error = FromValueError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Optional(o) => {
                o.as_ref()
                    .map(|v|
                        T::try_from(v)
                    )
                    .transpose()
                    .map(|optional| optional.into())
            }
            _ => Err(FromValueError::new_typed::<Self>(value.clone())),
        }
    }
}

// TryFrom<Value> for Option<T>
impl<T: TryFrom<Value, Error = FromValueError> + TypedValue> TryFrom<Value> for Option<T> {
    type Error = FromValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Optional(o) => {
                o.map(|v|
                        T::try_from(v)
                    )
                    .transpose()
                    .map(|optional| optional.into())
            }
            _ => Err(FromValueError::new_typed::<Self>(value)),
        }
    }
}
