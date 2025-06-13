//use std::{ops::Deref, sync::Arc};
use super::OpaqueType;

/// Trait for opaque value types.
///
/// `OpaqueValue` defines the interface for custom value types that can be stored
/// in CEL [`crate::Value::Opaque`] variants. This trait is sealed and can only be implemented
/// by types that also implement the required marker traits.
///
/// # Required Traits
///
/// Types implementing `OpaqueValue` must also implement:
/// - `std::fmt::Debug` - For debugging output
/// - `std::fmt::Display` - For string representation
/// - `dyn_clone::DynClone` - For cloning trait objects
/// - `Send + Sync` - For thread safety
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{OpaqueValue, OpaqueType};
/// use std::fmt;
///
/// #[derive(Debug, Clone)]
/// struct CustomId(u64);
///
/// impl fmt::Display for CustomId {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "CustomId({})", self.0)
///     }
/// }
///
/// impl OpaqueValue for CustomId {
///     fn opaque_type(&self) -> OpaqueType {
///         OpaqueType::new("CustomId")
///     }
/// }
/// ```
pub trait OpaqueValue : std::fmt::Debug + std::fmt::Display
    + dyn_clone::DynClone
    + Send + Sync
    + private::Sealed
{
    /// Returns the opaque type information for this value.
    ///
    /// This method provides type metadata that can be used for type checking
    /// and runtime type identification.
    fn opaque_type(&self) -> OpaqueType;
}
dyn_clone::clone_trait_object!(OpaqueValue);

/// Trait for typed opaque values with static type information.
///
/// `TypedOpaqueValue` extends [`OpaqueValue`] to provide static type information.
/// This is useful for types that have a known, constant opaque type.
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{TypedOpaqueValue, OpaqueType};
///
/// struct UserId(u64);
///
/// impl TypedOpaqueValue for UserId {
///     fn opaque_type() -> OpaqueType {
///         OpaqueType::new("UserId")
///     }
/// }
/// ```
pub trait TypedOpaqueValue: OpaqueValue {
    /// Returns the static opaque type for this value type.
    fn opaque_type() -> OpaqueType;
}

/// Opaque is a wrapper around a boxed opaque value.
#[derive(Debug, PartialEq, Eq)]
pub struct Opaque<T = Box<dyn OpaqueValue>>(pub T);

impl<T: Clone> Clone for Opaque<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Opaque<T> {
    /// Extracts the inner value from the opaque container.
    ///
    /// # Returns
    ///
    /// The contained value of type `T`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Opaque;
    ///
    /// let opaque = Opaque::new(42u64);
    /// let inner = opaque.into_inner();
    /// assert_eq!(inner, 42u64);
    /// ```
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl Opaque {
    /// Creates a new opaque value container.
    ///
    /// # Parameters
    ///
    /// - `value`: The value to wrap, must implement [`OpaqueValue`]
    ///
    /// # Returns
    ///
    /// New `Opaque` instance containing the value
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::Opaque;
    ///
    /// let opaque = Opaque::new(MyCustomType::new());
    /// ```
    pub fn new<T: OpaqueValue>(value: T) -> Self {
        Opaque(Box::new(value))
    }

    /// Returns a reference to the contained value as a trait object.
    pub fn as_ref(&self) -> &dyn OpaqueValue {
        self.0.as_ref()
    }

    /// Returns a mutable reference to the contained value as a trait object.
    pub fn as_mut(&mut self) -> &mut dyn OpaqueValue {
        self.0.as_mut()
    }

    /// Returns the opaque type of the contained value.
    pub fn opaque_type(&self) -> OpaqueType {
        self.0.opaque_type()
    }

    /// Checks if the contained value is of a specific type.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type to check for
    ///
    /// # Returns
    ///
    /// `true` if the contained value is of type `T`, `false` otherwise
    pub fn is<T: OpaqueValue>(&self) -> bool {
        self.0.is::<T>()
    }

    /// Attempts to downcast to a specific typed opaque value.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The target type to downcast to
    ///
    /// # Returns
    ///
    /// - `Ok(Opaque<T>)`: Downcast successful
    /// - `Err(Self)`: Downcast failed, returns original value
    pub fn downcast<T: OpaqueValue>(self) -> Result<Opaque<T>, Self> {
        self.0.downcast::<T>()
            .map(Opaque)
            .map_err( Opaque)
    }

    /// Returns a reference to the contained value if it's of the specified type.
    pub fn downcast_ref<T: OpaqueValue>(&self) -> Option<&T> {
        self.0.downcast_ref::<T>()
    }

    /// Returns a mutable reference to the contained value if it's of the specified type.
    pub fn downcast_mut<T: OpaqueValue>(&mut self) -> Option<&mut T> {
        self.0.downcast_mut::<T>()
    }
}

impl<T: OpaqueValue> Opaque<T> {
    /// Returns a reference to the contained value.
    pub fn as_ref(&self) -> &T {
        &self.0
    }

    /// Returns a mutable reference to the contained value.
    pub fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }

    /// Returns the opaque type of the contained value.
    pub fn opaque_type(&self) -> OpaqueType {
        self.0.opaque_type()
    }

    /// Converts this typed opaque value to a type-erased opaque value.
    ///
    /// # Returns
    ///
    /// Type-erased `Opaque` containing the same value
    pub fn upcast(self) -> Opaque {
        Opaque(Box::new(self.0))
    }
}

impl<T: OpaqueValue> From<Opaque<T>> for Opaque {
    fn from(value: Opaque<T>) -> Self {
        value.upcast()
    }
}

impl<T: OpaqueValue> From<T> for Opaque<T> {
    fn from(value: T) -> Self {
        Opaque(value)
    }
}
impl<T: OpaqueValue> From<Box<T>> for Opaque<T> {
    fn from(value: Box<T>) -> Self {
        Opaque(*value)
    }
}

impl<T> OpaqueValue for T where T: TypedOpaqueValue {
    fn opaque_type(&self) -> OpaqueType {
        <Self as TypedOpaqueValue>::opaque_type()
    }
}

impl dyn OpaqueValue {
    /// Checks if this opaque value is of a specific type.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type to check for
    ///
    /// # Returns
    ///
    /// `true` if this value is of type `T`, `false` otherwise
    pub fn is<T: OpaqueValue>(&self) -> bool {
        private::Sealed::as_any(self).is::<T>()
    }

    /// Attempts to downcast this boxed opaque value to a specific type.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The target type to downcast to
    ///
    /// # Returns
    ///
    /// - `Ok(T)`: Downcast successful
    /// - `Err(Box<Self>)`: Downcast failed, returns original boxed value
    pub fn downcast<T: OpaqueValue>(self: Box<Self>) -> Result<T, Box<Self>> {
        if self.is::<T>() {
            Ok(*private::Sealed::into_any(self).downcast::<T>().unwrap())
        } else {
            Err(self)
        }
    }

    /// Returns a reference to the contained value if it's of the specified type.
    pub fn downcast_ref<T: OpaqueValue>(&self) -> Option<&T> {
        private::Sealed::as_any(self).downcast_ref::<T>()
    }

    /// Returns a mutable reference to the contained value if it's of the specified type.
    pub fn downcast_mut<T: OpaqueValue>(&mut self) -> Option<&mut T> {
        private::Sealed::as_any_mut(self).downcast_mut::<T>()
    }
}

impl std::cmp::PartialEq for dyn OpaqueValue {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(private::Sealed::as_any(other))
    }
}
impl std::cmp::Eq for dyn OpaqueValue {}


mod private {
    use super::OpaqueValue;
    use std::any::Any;

    pub trait Sealed: Any {
        fn into_any(self: Box<Self>) -> Box<dyn Any>;
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;

        fn dyn_eq(&self, other: &dyn Any) -> bool;
    }

    impl<T: OpaqueValue + PartialEq> Sealed for T {
        fn into_any(self: Box<Self>) -> Box<dyn Any> { self }
        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }

        fn dyn_eq(&self, other: &dyn Any) -> bool {
            other.downcast_ref::<T>().map_or(false, |other| self == other)
        }
    }
}
