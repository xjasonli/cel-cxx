mod bool;
mod bytes;
mod double;
mod duration;
mod error;
mod int;
mod list;
mod map;
mod null;
mod opaque;
mod optional;
mod string;
mod r#struct;
mod timestamp;
mod r#type;
mod uint;
mod unknown;

/// Implements the `TypedValue`/'TypedMapKey' trait for the given type.
macro_rules! impl_typed {
    (
        $variant:ident: $($kinds:ident),+ {
            $($entries:tt)*
        }
        $($rest:tt)*
    ) => {
        impl_typed!(@parse_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            $($entries)*
        );
        impl_typed!($($rest)*);
    };

    () => {};

    (@parse_entry
        #Variant $variant:ident,
        #Kinds [$($kinds:ident),+],
        $(@[$($generics:tt)+])?
        $ty:ty
        $(=> $expr:expr)?
        $(, $($entries:tt)*)?
    ) => {
        impl_typed!(@expand_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            #Generics [$(<$($generics)+>)?],
            #Type $ty,
            #Expr [$($expr)?],
        );
        impl_typed!(@parse_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            $($($entries)*)?
        );
    };

    (@parse_entry
        #Variant $variant:ident,
        #Kinds [$($kinds:ident),+],
    ) => {};

    (@expand_entry
        #Variant $variant:ident,
        #Kinds [$($kinds:ident),+],
        #Generics $generics:tt,
        #Type $ty:ty,
        #Expr $expr:tt,
    ) => {
        $(
            impl_typed!(@expand_kind
                #Variant $variant,
                #Kind $kinds,
                #Generics $generics,
                #Type $ty,
                #Expr $expr,
            );
        )+
    };

    (@expand_kind
        #Variant $variant:ident,
        #Kind Value,
        #Generics [$($generics:tt)*],
        #Type $ty:ty,
        #Expr [$($expr:expr)?],
    ) => {
        impl$($generics)* TypedValue for $ty {
            fn value_type() -> ValueType {
                ValueType::$variant $(( $expr ))?
            }
        }
    };

    (@expand_kind
        #Variant $variant:ident,
        #Kind MapKey,
        #Generics [$($generics:tt)*],
        #Type $ty:ty,
        #Expr [$($expr:expr)?],
    ) => {
        impl$(<$($generics)+>)? TypedMapKey for $ty {}
    };
}

/// Implements the `IntoValue`/'IntoMapKey'/'IntoConstant' trait for the given type.
macro_rules! impl_into {
    (
        $variant:ident: $($kinds:ident),+ {
            $($entries:tt)*
        }
        $($rest:tt)*
    ) => {
        impl_into!(@parse_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            $($entries)*
        );
        impl_into!($($rest)*);
    };
    () => {};

    (@parse_entry
        #Variant $variant:ident,
        #Kinds [$($kinds:ident),+],
        $(@[$($generics:tt)+])?
        $ty:ty
        $(=> |$self:ident| $expr:expr)?
        $(, $($entries:tt)*)?
    ) => {
        impl_into!(@expand_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            #Generics [$(<$($generics)+>)?],
            #Type $ty,
            #Self $($self)?,
            #Expr [$($expr)?],
        );
        impl_into!(@parse_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            $($($entries)*)?
        );
    };

    (@parse_entry
        #Variant $variant:ident,
        #Kinds [$($kinds:ident),+],
    ) => {};

    (@expand_entry
        #Variant $variant:ident,
        #Kinds [$($kinds:ident),+],
        #Generics $generics:tt,
        #Type $ty:ty,
        #Self ,
        #Expr [],
    ) => {
        impl_into!(@expand_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            #Generics $generics,
            #Type $ty,
            #Self self,
            #Expr [],
        );
    };

    (@expand_entry
        #Variant $variant:ident,
        #Kinds [$($kinds:ident),+],
        #Generics $generics:tt,
        #Type $ty:ty,
        #Self $self:ident,
        #Expr $expr:tt,
    ) => {
        $(
            impl_into!(@expand_kind
                #Variant $variant,
                #Kind $kinds,
                #Generics $generics,
                #Type $ty,
                #Self $self,
                #Expr $expr,
            );
        )+
    };

    (@expand_kind
        #Variant $variant:ident,
        #Kind Value,
        #Generics [$($generics:tt)*],
        #Type $ty:ty,
        #Self $self:ident,
        #Expr [$($expr:expr)?],
    ) => {
        impl$($generics)* IntoValue for $ty {
            fn into_value($self) -> Value {
                Value::$variant
                $(
                    ($expr)
                )?
            }
        }
        impl$($generics)* From<$ty> for Value {
            fn from(value: $ty) -> Self {
                <$ty as IntoValue>::into_value(value)
            }
        }
    };
    (@expand_kind
        #Variant $variant:ident,
        #Kind MapKey,
        #Generics [$($generics:tt)*],
        #Type $ty:ty,
        #Self $self:ident,
        #Expr [$($expr:expr)?],
    ) => {
        impl$($generics)* IntoMapKey for $ty {}
        impl$($generics)* From<$ty> for MapKey {
            fn from(value: $ty) -> Self {
                <$ty as IntoMapKey>::into_mapkey(value)
            }
        }
    };
    (@expand_kind
        #Variant $variant:ident,
        #Kind Constant,
        #Generics [$($generics:tt)*],
        #Type $ty:ty,
        #Self $self:ident,
        #Expr [$($expr:expr)?],
    ) => {
        impl$($generics)* IntoConstant for $ty {}
        impl$($generics)* From<$ty> for Constant {
            fn from(value: $ty) -> Self {
                <$ty as IntoConstant>::into_constant(value)
            }
        }
    };
}

/// Implements the `FromValue`/`FromMapKey` trait for the given type.
macro_rules! impl_from {
    (
        $variant:ident: $($kinds:ident),+ {
            $($entries:tt)*
        }
        $($rest:tt)*
    ) => {
        impl_from!(@parse_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            $($entries)*
        );
        impl_from!($($rest)*);
    };

    () => {};

    (@parse_entry
        #Variant $variant:ident,
        #Kinds [$($kinds:ident),+],
        $ty:ty $(: !$sized:ident)? as $output:ty
        => |$var:ident| $expr:expr
        $(, $($entries:tt)*)?
    ) => {
        impl_from!(@expand_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            #Sized [$(!$sized)?],
            #Owned [],
            #Output $output,
            #Type $ty,
            #Var [$var],
            #Expr [$expr],
        );
        impl_from!(@parse_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            $($($entries)*)?
        );
    };

    (@parse_entry
        #Variant $variant:ident,
        #Kinds [$($kinds:ident),+],
        $ty:ty
        => |$var:ident| $expr:expr
        $(, $($entries:tt)*)?
    ) => {
        impl_from!(@expand_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            #Sized [],
            #Owned [$ty],
            #Output $ty,
            #Type $ty,
            #Var [$var],
            #Expr [$expr],
        );
        impl_from!(@parse_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            $($($entries)*)?
        );
    };

    (@parse_entry
        #Variant $variant:ident,
        #Kinds [$($kinds:ident),+],
        $ty:ty
        => || $expr:expr
        $(, $($entries:tt)*)?
    ) => {
        impl_from!(@expand_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            #Sized [],
            #Owned [$ty],
            #Output $ty,
            #Type $ty,
            #Var [],
            #Expr [$expr],
        );
        impl_from!(@parse_entry
            #Variant $variant,
            #Kinds [$($kinds),+],
            $($($entries)*)?
        );
    };

    (@parse_entry
        #Variant $variant:ident,
        #Kinds [$($kinds:ident),+],
    ) => {};

    (@expand_entry
        #Variant $variant:ident,
        #Kinds [$($kinds:ident),+],
        #Sized $sized:tt,
        #Owned $owned:tt,
        #Output $output:ty,
        #Type $ty:ty,
        #Var $var:tt,
        #Expr $expr:tt,
    ) => {
        $(
            impl_from!(@expand_kind
                #Variant $variant,
                #Kind $kinds,
                #Sized $sized,
                #Owned $owned,
                #Output $output,
                #Type $ty,
                #Var $var,
                #Expr $expr,
            );
        )+
    };

    (@expand_kind
        #Variant $variant:ident,
        #Kind Value,
        #Sized [$(!$sized:ident)?],
        #Owned [$($owned:ty)?],
        #Output $output:ty,
        #Type $ty:ty,
        #Var [$($var:ident)?],
        #Expr [$expr:expr],
    ) => {
        impl FromValue for $ty {
            type Output<'a> = $output;

            fn from_value<'a>(value: &'a Value) -> Result<Self::Output<'a>, FromValueError> {
                match value {
                    Value::$variant $( ($var) )? => Ok($expr),
                    _ => Err(FromValueError::new_typed::<Self>(value.clone())),
                }
            }
        }

        impl_from!(@expand_try_from_value
            #Variant $variant,
            #Output $output,
            #Sized $(!$sized)?,
            #Owned $($owned)?,
            #Type $ty,
        );
    };

    // Sized + Owned
    (@expand_try_from_value
        #Variant $variant:ident,
        #Output $output:ty,
        #Sized ,
        #Owned $owned:ty,
        #Type $ty:ty,
    ) => {
        impl TryFrom<&Value> for $owned {
            type Error = FromValueError;

            fn try_from(value: &Value) -> Result<Self, <Self as TryFrom<&Value>>::Error> {
                Self::from_value(value)
            }
        }
        impl TryFrom<Value> for $owned {
            type Error = FromValueError;

            fn try_from(value: Value) -> Result<Self, <Self as TryFrom<Value>>::Error> {
                Self::from_value(&value)
            }
        }
    };

    // Sized + !Owned
    (@expand_try_from_value
        #Variant $variant:ident,
        #Output $output:ty,
        #Sized ,
        #Owned ,
        #Type $ty:ty,
    ) => {
        impl<'a> TryFrom<&'a Value> for $output {
            type Error = FromValueError;

            fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
                Self::from_value(value)
            }
        }
    };

    // !Sized
    (@expand_try_from_value
        #Variant $variant:ident,
        #Output $output:ty,
        #Sized !Sized,
        #Owned $($owned:ty)?,
        #Type $ty:ty,
    ) => {};

    (@expand_kind
        #Variant $variant:ident,
        #Kind MapKey,
        #Sized [$(!$sized:ident)?],
        #Owned [$($owned:ty)?],
        #Output $output:ty,
        #Type $ty:ty,
        #Var [$($var:ident)?],
        #Expr [$expr:expr],
    ) => {
        impl FromMapKey for $ty {
            fn from_mapkey<'a>(mapkey: &'a MapKey) -> Result<<Self as FromValue>::Output<'a>, FromMapKeyError> {
                match mapkey {
                    MapKey::$variant $( ($var))? => Ok($expr),
                    _ => Err(FromMapKeyError::new_typed::<Self>(mapkey.clone())),
                }
            }
        }

        impl_from!(@expand_try_from_mapkey
            #Variant $variant,
            #Output $output,
            #Sized $(!$sized)?,
            #Owned $($owned)?,
            #Type $ty,
        );
    };

    // Sized + Owned
    (@expand_try_from_mapkey
        #Variant $variant:ident,
        #Output $output:ty,
        #Sized ,
        #Owned $owned:ty,
        #Type $ty:ty,
    ) => {
        impl TryFrom<&MapKey> for $owned {
            type Error = FromMapKeyError;

            fn try_from(mapkey: &MapKey) -> Result<Self, <Self as TryFrom<&MapKey>>::Error> {
                Self::from_mapkey(mapkey)
            }
        }
        impl TryFrom<MapKey> for $owned {
            type Error = FromMapKeyError;

            fn try_from(mapkey: MapKey) -> Result<Self, <Self as TryFrom<MapKey>>::Error> {
                Self::from_mapkey(&mapkey)
            }
        }
    };

    // Sized + !Owned
    (@expand_try_from_mapkey
        #Variant $variant:ident,
        #Output $output:ty,
        #Sized ,
        #Owned ,
        #Type $ty:ty,
    ) => {
        impl<'a> TryFrom<&'a MapKey> for $output {
            type Error = FromMapKeyError;

            fn try_from(mapkey: &'a MapKey) -> Result<Self, Self::Error> {
                Self::from_mapkey(mapkey)
            }
        }
    };

    // !Sized
    (@expand_try_from_mapkey
        #Variant $variant:ident,
        #Output $output:ty,
        #Sized !Sized,
        #Owned $($owned:ty)?,
        #Type $ty:ty,
    ) => {};
}

/// Implements the `FromValue` trait for the given list type.
macro_rules! impl_from_list {
    (
        #[adder = $adder:ident]
        $cont:ident
        $(, $($rest:tt)*)?
    ) => {
        impl_from_list!(@expand_cont
            #Adder $adder,
            $cont
        );
        impl_from_list!($( $($rest)* )?);
    };

    () => {};

    (@expand_cont
        #Adder $adder:ident,
        $cont:ident
    ) => {
        impl<T: FromValue + TypedValue> FromValue for $cont<T> {
            type Output<'a> = $cont<T::Output<'a>>;

            fn from_value<'a>(value: &'a Value) -> Result<Self::Output<'a>, FromValueError> {
                match value {
                    Value::List(list) => {
                        let mut result = $cont::new();
                        for v in list {
                            result.$adder(T::from_value(v)?);
                        }
                        Ok(result)
                    }
                    _ => Err(FromValueError::new_typed::<Self>(value.clone())),
                }
            }
        }

        impl<T: TypedValue + TryFrom<Value, Error = FromValueError>> TryFrom<Value> for $cont<T> {
            type Error = FromValueError;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    Value::List(list) => {
                        let mut result = $cont::new();
                        for v in list {
                            result.$adder(T::try_from(v)?);
                        }
                        Ok(result)
                    }
                    _ => Err(FromValueError::new_typed::<Self>(value)),
                }
            }
        }

        impl<'a, T: TryFrom<&'a Value, Error = FromValueError> + TypedValue> TryFrom<&'a Value> for $cont<T> {
            type Error = FromValueError;

            fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
                match value {
                    Value::List(list) => {
                        let mut result = $cont::new();
                        for v in list {
                            result.$adder(T::try_from(v)?);
                        }
                        Ok(result)
                    }
                    _ => Err(FromValueError::new_typed::<Self>(value.clone())),
                }
            }
        }
    };
}

/// Implements the `FromValue` trait for the given map type.
macro_rules! impl_from_map {
    (
        $cont:ident
        $(, $($rest:tt)*)?
    ) => {
        impl_from_map!(@expand_cont
            $cont
        );
        impl_from_map!($( $($rest)* )?);
    };

    () => {};

    (@expand_cont
        $cont:ident
    ) => {
        impl<K, V> FromValue for $cont<K, V>
        where
            K: FromMapKey + TypedMapKey,
            V: FromValue + TypedValue,
            for<'a> K::Output<'a>: Eq + std::hash::Hash + Ord,
        {
            type Output<'a> = $cont<K::Output<'a>, V::Output<'a>>;

            fn from_value<'a>(value: &'a Value) -> Result<Self::Output<'a>, FromValueError> {
                match value {
                    Value::Map(map) => {
                        let mut result = $cont::new();
                        for (k, v) in map {
                            let item = [(K::from_mapkey(k)?, V::from_value(v)?)];
                            std::iter::Extend::extend(&mut result, item);
                        }
                        Ok(result)
                    }
                    _ => Err(FromValueError::new_typed::<Self>(value.clone())),
                }
            }
        }

        impl<K, V> TryFrom<Value> for $cont<K, V>
        where
            K: TryFrom<MapKey, Error = FromMapKeyError> + TypedMapKey,
            V: TryFrom<Value, Error = FromValueError> + TypedValue,
            K: Eq + std::hash::Hash + Ord,
        {
            type Error = FromValueError;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    Value::Map(map) => {
                        let mut result = $cont::new();
                        for (k, v) in map {
                            let item = [(K::try_from(k)?, V::try_from(v)?)];
                            std::iter::Extend::extend(&mut result, item);
                        }
                        Ok(result)
                    }
                    _ => Err(FromValueError::new_typed::<Self>(value)),
                }
            }
        }

        impl<'a, K, V> TryFrom<&'a Value> for $cont<K, V>
        where
            K: TryFrom<&'a MapKey, Error = FromMapKeyError> + TypedMapKey,
            V: TryFrom<&'a Value, Error = FromValueError> + TypedValue,
            K: Eq + std::hash::Hash + Ord,
        {
            type Error = FromValueError;

            fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
                match value {
                    Value::Map(map) => {
                        let mut result = $cont::new();
                        for (k, v) in map {
                            let item = [(K::try_from(k)?, V::try_from(v)?)];
                            std::iter::Extend::extend(&mut result, item);
                        }
                        Ok(result)
                    }
                    _ => Err(FromValueError::new_typed::<Self>(value.clone())),
                }
            }
        }
    };
}

use impl_from;
use impl_from_list;
use impl_from_map;
use impl_into;
use impl_typed;
