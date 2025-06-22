//use std::{ops::Deref, sync::Arc};
use super::*;

/// Trait for opaque types that can be stored in CEL values.
///
/// This trait allows custom Rust types to be embedded in CEL values as opaque
/// objects. Opaque types are useful for:
/// - **Foreign data structures**: Types from other libraries  
/// - **Complex business objects**: Domain-specific data models
/// - **Native performance**: Keep computations in native Rust
/// - **Type safety**: Maintain strong typing across CEL boundaries
///
/// # Opaque Type Characteristics
///
/// Opaque values in CEL:
/// - Cannot be directly manipulated by CEL expressions
/// - Can be passed between functions that understand the type
/// - Support method calls through registered member functions
/// - Maintain their Rust type identity and behavior
///
/// # Required Traits
///
/// Implementing `Opaque` requires several standard traits:
/// - [`Clone`]: For value duplication
/// - [`Debug`]: For debugging and error messages  
/// - [`std::fmt::Display`]: For string representation
/// - [`Send + Sync`]: For thread safety
/// - [`dyn_clone::DynClone`]: For trait object cloning
///
/// # Examples
///
/// ## Basic Opaque Type
///
/// ```rust
/// use cel_cxx::{Opaque, Value};
///
/// #[derive(Opaque, Debug, Clone, PartialEq)]
/// struct UserId(u64);
///
/// // All necessary traits are automatically implemented by the derive macro
///
/// // Usage in CEL values
/// let user_id = UserId(12345);
/// let value = user_id.into_value();
/// ```
///
/// ## Integration with Functions
///
/// ```rust
/// use cel_cxx::*;
///
/// #[derive(Opaque, Debug, Clone)]
/// struct BankAccount {
///     balance: f64,
///     account_number: String,
/// }
///
/// // Register methods for the opaque type
/// let env = Env::builder()
///     .register_member_function("balance", |account: &BankAccount| account.balance)?
///     .register_member_function("withdraw", |account: &mut BankAccount, amount: f64| {
///         if amount <= account.balance {
///             account.balance -= amount;
///             Ok(account.balance)
///         } else {
///             Err("Insufficient funds")
///         }
///     })?
///     .build()?;
/// ```
///
/// # Type Erasure and Downcasting
///
/// Opaque values support safe downcasting:
///
/// ```rust
/// use cel_cxx::{Value, Opaque};
///
/// # #[derive(Opaque, Debug, Clone)]
/// # struct UserId(u64);
/// let user_id = UserId(12345);
/// let value = user_id.into_value();
///
/// // Safe downcasting
/// if let Value::Opaque(opaque) = &value {
///     if opaque.is::<UserId>() {
///         let user_id_ref = opaque.downcast_ref::<UserId>().unwrap();
///         println!("User ID: {}", user_id_ref.0);
///     }
/// }
/// ```
pub trait Opaque
    : dyn_clone::DynClone
    + std::fmt::Debug + std::fmt::Display
    + Send + Sync
    + private::Sealed
{
    /// Returns the opaque type information for this value.
    ///
    /// This method provides type metadata that can be used for type checking
    /// and runtime type identification.
    fn opaque_type(&self) -> OpaqueType;
}
dyn_clone::clone_trait_object!(Opaque);

/// Trait for opaque types with additional type constraints.
///
/// This trait extends [`Opaque`] with additional requirements that enable
/// more sophisticated operations on opaque values. It adds:
/// - **Value equality**: Types can be compared for equality
/// - **Cloning**: Efficient cloning without heap allocation
/// - **Enhanced type safety**: Stronger compile-time guarantees
///
/// # When to Use TypedOpaque
///
/// Use `TypedOpaque` when your opaque type needs:
/// - Equality comparisons in CEL expressions
/// - Participation in hash-based collections  
/// - Advanced type checking and validation
/// - Integration with CEL's type inference system
///
/// # Additional Requirements
///
/// Beyond [`Opaque`], this trait requires:
/// - [`Clone`]: Direct cloning (not just through trait objects)
/// - [`PartialEq`]: Value equality comparison
/// - All the traits required by [`Opaque`]
///
/// # Examples
///
/// ## Comparable Opaque Type
///
/// ```rust
/// use cel_cxx::Opaque;
///
/// #[derive(Opaque, Debug, Clone, PartialEq)]
/// struct ProductId(String);
///
/// // All necessary traits are automatically implemented by the derive macro
/// ```
///
/// ## Usage in Collections
///
/// ```rust
/// use cel_cxx::*;
/// use std::collections::HashSet;
///
/// # #[derive(Opaque, Debug, Clone, PartialEq, Hash)]
/// # struct ProductId(String);
///
/// // Can be used in collections that require equality
/// let mut products = HashSet::new();
/// products.insert(ProductId("ABC123".to_string()));
/// products.insert(ProductId("DEF456".to_string()));
///
/// // Can be compared in CEL expressions (if registered properly)
/// let env = Env::builder()
///     .register_global_function("contains_product", move |products: HashSet<&ProductId>, id: &ProductId| {
///         products.contains(id)
///     })?
///     .build()?;
/// ```
pub trait TypedOpaque
    : Opaque
    + Clone + PartialEq
    + std::fmt::Debug + std::fmt::Display
    + Send + Sync
{
    /// Returns the static opaque type for this value type.
    fn opaque_type() -> OpaqueType;
}

impl dyn Opaque {
    /// Checks if this opaque value is of a specific type.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type to check for
    ///
    /// # Returns
    ///
    /// `true` if this value is of type `T`, `false` otherwise
    pub fn is<T: Opaque>(&self) -> bool {
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
    pub fn downcast<T: Opaque>(self: Box<Self>) -> Result<T, Box<Self>> {
        if self.is::<T>() {
            Ok(*private::Sealed::into_any(self).downcast::<T>().unwrap())
        } else {
            Err(self)
        }
    }

    /// Returns a reference to the contained value if it's of the specified type.
    pub fn downcast_ref<T: Opaque>(&self) -> Option<&T> {
        private::Sealed::as_any(self).downcast_ref::<T>()
    }

    /// Returns a mutable reference to the contained value if it's of the specified type.
    pub fn downcast_mut<T: Opaque>(&mut self) -> Option<&mut T> {
        private::Sealed::as_any_mut(self).downcast_mut::<T>()
    }
}

impl std::cmp::PartialEq for dyn Opaque {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(private::Sealed::as_any(other))
    }
}
impl std::cmp::Eq for dyn Opaque {}


impl<T: TypedOpaque + ?Sized> Opaque for T {
    fn opaque_type(&self) -> OpaqueType {
        <Self as TypedOpaque>::opaque_type()
    }
}

mod private {
    use super::Opaque;
    use std::any::Any;

    pub trait Sealed: Any {
        fn into_any(self: Box<Self>) -> Box<dyn Any>;
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;

        fn dyn_eq(&self, other: &dyn Any) -> bool;
    }

    impl<T: Opaque + PartialEq> Sealed for T {
        fn into_any(self: Box<Self>) -> Box<dyn Any> { self }
        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }

        fn dyn_eq(&self, other: &dyn Any) -> bool {
            other.downcast_ref::<T>().map_or(false, |other| self == other)
        }
    }
}
