//! Function overload management and resolution.
//!
//! This module provides types and utilities for managing function overloads,
//! allowing multiple function implementations with different signatures to be
//! registered under the same name.
//!
//! # Key Types
//!
//! - [`FunctionOverloads`]: Flat container of overloads for a single function name
//! - [`OverloadEntry`]: A single overload — the function item plus its call style
//! - [`FunctionDeclOrImpl`]: Union type for declarations and implementations
//!
//! # Overload Resolution
//!
//! Overloads are distinguished by full argument types (not just kinds), so
//! same-kind overloads with different element types (e.g. `list<int>` and
//! `list<string>`) coexist; exact-duplicate signatures are rejected. The
//! overload id (see [`FunctionType::id`](crate::types::FunctionType::id)) is used by
//! both the checker `OverloadDecl` and the runtime `FunctionDescriptor` to
//! select among overloads.

use super::*;

/// Flat collection of function overloads for a single function name.
///
/// `FunctionOverloads<T>` holds one [`OverloadEntry<T>`] per overload, keyed by
/// `(member, full argument types)`.
///
/// # Type Parameters
///
/// - `T`: the per-overload item — [`FunctionDeclOrImpl<'f>`] (registry) or
///   [`Function<'f>`](super::Function) (runtime bindings).
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::function::{FunctionOverloads, FunctionDeclOrImpl};
///
/// let overloads = FunctionOverloads::<FunctionDeclOrImpl<'_>>::new();
/// for overload in overloads.entries() {
///     // Process each overload entry
/// }
/// ```
#[derive(Debug, Default, Clone)]
pub struct FunctionOverloads<T>(Vec<OverloadEntry<T>>);

impl<T: FunctionTypeOverload> FunctionOverloads<T> {
    /// Creates a new empty function overload collection.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Finds a function overload by member flag and full argument types.
    ///
    /// # Parameters
    ///
    /// - `member`: whether to find member functions (true) or global functions (false)
    /// - `args`: argument types to match (full [`ValueType`]s, not just kinds)
    ///
    /// # Returns
    ///
    /// Returns `Some(&OverloadEntry)` if found, `None` otherwise.
    pub fn find(&self, member: bool, args: &[ValueType]) -> Option<&OverloadEntry<T>> {
        self.0
            .iter()
            .find(|e| e.member() == member && e.inner().arguments() == args)
    }

    /// Finds a mutable reference to a function overload by member flag and full
    /// argument types.
    pub fn find_mut(&mut self, member: bool, args: &[ValueType]) -> Option<&mut OverloadEntry<T>>
    where
        T: Send + Sync,
    {
        self.0
            .iter_mut()
            .find(|e| e.member() == member && e.inner().arguments() == args)
    }

    /// Returns an iterator over all function overload entries.
    pub fn entries(&self) -> impl Iterator<Item = &OverloadEntry<T>> {
        self.0.iter()
    }

    /// Returns a mutable iterator over all function overload entries.
    pub fn entries_mut(&mut self) -> impl Iterator<Item = &mut OverloadEntry<T>> {
        self.0.iter_mut()
    }

    /// Clears all function overloads.
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Removes a function overload by member flag and full argument types.
    ///
    /// # Returns
    ///
    /// `Ok(())` if removal was successful, `Err(Error)` if not found.
    pub fn remove(&mut self, member: bool, args: &[ValueType]) -> Result<(), Error> {
        if let Some(index) = self
            .0
            .iter()
            .position(|e| e.member() == member && e.inner().arguments() == args)
        {
            self.0.remove(index);
        } else {
            return Err(Error::not_found(format!(
                "Overload [{}] ({}) not found",
                if member { "member" } else { "global" },
                args.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            )));
        }
        Ok(())
    }
}

impl<'f> FunctionOverloads<FunctionDeclOrImpl<'f>> {
    /// Adds a function implementation to the overloads.
    ///
    /// Deduplicates by `(member, full argument types)`: an exact-duplicate
    /// signature is rejected; same-kind overloads with different element types
    /// (e.g. `list<int>` vs `list<string>`) coexist.
    ///
    /// # Arguments
    ///
    /// * `member` - Whether this is a member function
    /// * `f` - The function implementation to add
    ///
    /// # Errors
    ///
    /// Returns an error if a function with the same exact signature already
    /// exists, or if a member function is given zero arguments.
    pub fn add_impl(&mut self, member: bool, f: Function<'f>) -> Result<&mut Self, Error> {
        if member && f.arguments_len() == 0 {
            return Err(Error::invalid_argument(
                "Member functions cannot have zero arguments",
            ));
        }
        let args = f.arguments();
        if self.find(member, &args).is_some() {
            return Err(Error::invalid_argument("Function already exists"));
        }
        self.0
            .push(OverloadEntry::new(member, FunctionDeclOrImpl::new_impl(f)));
        Ok(self)
    }

    /// Adds a function declaration to the overloads.
    ///
    /// Deduplicates by `(member, full argument types)`. Declarations provide
    /// type information for compile-time checking without requiring an
    /// implementation.
    ///
    /// # Errors
    ///
    /// Returns an error if a function with the same exact signature already
    /// exists, or if a member function is given zero arguments.
    pub fn add_decl(&mut self, member: bool, f: FunctionType) -> Result<&mut Self, Error> {
        if member && f.arguments().is_empty() {
            return Err(Error::invalid_argument(
                "Member functions cannot have zero arguments",
            ));
        }
        let args = f.arguments().to_vec();
        if self.find(member, &args).is_some() {
            return Err(Error::invalid_argument("Function already exists"));
        }
        self.0
            .push(OverloadEntry::new(member, FunctionDeclOrImpl::new(f)));
        Ok(self)
    }
}

impl<'f> FunctionOverloads<Function<'f>> {
    /// Adds a function (implementation or declaration) to the overloads.
    ///
    /// Deduplicates by `(member, full argument types)`. Used when working with
    /// pure function implementations (runtime bindings) without separate
    /// declarations.
    ///
    /// # Errors
    ///
    /// Returns an error if a function with the same exact signature already
    /// exists, or if a member function is given zero arguments.
    pub fn add(&mut self, member: bool, f: Function<'f>) -> Result<&mut Self, Error> {
        if member && f.arguments_len() == 0 {
            return Err(Error::invalid_argument(
                "Member functions cannot have zero arguments",
            ));
        }
        let args = f.arguments();
        if self.find(member, &args).is_some() {
            return Err(Error::invalid_argument("Function already exists"));
        }
        self.0.push(OverloadEntry::new(member, f));
        Ok(self)
    }
}

/// Trait for extracting argument types from function-like objects.
///
/// Implemented for [`FunctionDeclOrImpl`] (registry) and [`Function`](super::Function)
/// (bindings).
pub trait FunctionTypeOverload {
    /// Returns the argument types for this function (full [`ValueType`]s).
    fn arguments(&self) -> Vec<ValueType>;
}

impl FunctionTypeOverload for FunctionDeclOrImpl<'_> {
    fn arguments(&self) -> Vec<ValueType> {
        self.decl().arguments().to_vec()
    }
}

impl FunctionTypeOverload for Function<'_> {
    fn arguments(&self) -> Vec<ValueType> {
        self.arguments()
    }
}

/// A single overload: the function item plus its call style.
///
/// `member` distinguishes member calls from global calls; the item retains
/// its full [`FunctionType`].
///
/// # Type Parameters
///
/// - `T`: the function item — [`FunctionDeclOrImpl<'f>`] or [`Function<'f>`](super::Function)
#[derive(Debug, Clone)]
pub struct OverloadEntry<T> {
    member: bool,
    inner: T,
}

impl<T: FunctionTypeOverload> OverloadEntry<T> {
    /// Creates a new overload entry.
    pub fn new(member: bool, inner: T) -> Self {
        Self { member, inner }
    }

    /// Returns whether this overload is a member function.
    pub fn member(&self) -> bool {
        self.member
    }

    /// Returns the function item (declaration/impl or binding).
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

/// Union type representing either a function declaration or implementation.
///
/// `FunctionDeclOrImpl` can hold either:
/// - A function type signature (declaration) for compile-time type checking
/// - A concrete function implementation for runtime execution
/// - Both a declaration and implementation (preferred)
///
/// This allows the system to provide type information even when implementations
/// are not available, enabling better compile-time checking and error reporting.
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::function::{FunctionDeclOrImpl, IntoFunction};
/// use cel_cxx::types::{ValueType, FunctionType};
///
/// // Create from declaration only
/// let func_type = FunctionType::new(ValueType::Int, vec![ValueType::Int]);
/// let decl_only = FunctionDeclOrImpl::new(func_type);
///
/// // Create from implementation (includes both decl and impl)
/// let func = (|a: i32, b: i32| -> i32 { a + b }).into_function();
/// let with_impl = FunctionDeclOrImpl::new_impl(func);
/// ```
#[derive(Debug, Clone)]
pub struct FunctionDeclOrImpl<'f> {
    r#type: FunctionType,
    r#impl: Option<Function<'f>>,
}

impl<'f> FunctionDeclOrImpl<'f> {
    /// Creates a new `FunctionDeclOrImpl` from a function implementation.
    ///
    /// This constructor creates both the declaration (extracted from the function's
    /// type signature) and stores the implementation for runtime execution.
    ///
    /// # Arguments
    ///
    /// * `r#impl` - The function implementation
    ///
    /// # Returns
    ///
    /// New `FunctionDeclOrImpl` containing both declaration and implementation
    pub fn new_impl(r#impl: Function<'f>) -> Self {
        Self {
            r#type: r#impl.function_type(),
            r#impl: Some(r#impl),
        }
    }

    /// Creates a new `FunctionDeclOrImpl` from a function type declaration.
    ///
    /// This constructor creates a declaration-only entry, useful for providing
    /// type information without requiring an implementation.
    ///
    /// # Arguments
    ///
    /// * `r#type` - The function type signature
    ///
    /// # Returns
    ///
    /// New `FunctionDeclOrImpl` containing only the declaration
    pub fn new(r#type: FunctionType) -> Self {
        Self {
            r#type,
            r#impl: None,
        }
    }

    /// Returns whether this entry has a concrete implementation.
    ///
    /// # Returns
    ///
    /// `true` if this entry contains a function implementation, `false` if declaration-only
    pub fn is_impl(&self) -> bool {
        self.r#impl.is_some()
    }

    /// Gets the function type declaration.
    ///
    /// # Returns
    ///
    /// Reference to the function type signature
    pub fn decl(&self) -> &FunctionType {
        &self.r#type
    }

    /// Gets the function implementation, if available.
    ///
    /// # Returns
    ///
    /// `Some(&Function)` if implementation is available, `None` for declaration-only entries
    pub fn r#impl(&self) -> Option<&Function<'f>> {
        self.r#impl.as_ref()
    }
}
