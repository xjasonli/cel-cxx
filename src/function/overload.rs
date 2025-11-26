//! Function overload management and resolution.
//!
//! This module provides types and utilities for managing function overloads,
//! allowing multiple function implementations with different signatures to
//! be registered under the same name.
//!
//! # Key Types
//!
//! - [`FunctionOverloads`]: Container for multiple overloads of a function
//! - [`FunctionKindOverload`]: Overloads grouped by argument kinds  
//! - [`FunctionDeclOrImpl`]: Union type for declarations and implementations
//!
//! # Overload Resolution
//!
//! The system supports sophisticated overload resolution based on:
//! - **Argument count**: Number of parameters
//! - **Argument types**: CEL types of each parameter
//! - **Member vs global**: Whether the function is called as a method
//!
//! This enables natural function calls like:
//! ```text
//! // Different overloads of "format"
//! format("Hello")              // format(string) -> string
//! format("Hello %s", "World")  // format(string, string) -> string  
//! format(42)                   // format(int) -> string
//! ```

use super::*;
use crate::Kind;

/// Collection of function overloads grouped by argument kinds.
///
/// `FunctionOverloads<T>` manages multiple function overloads that share the same name
/// but have different argument signatures. This allows CEL to support function overloading
/// based on argument types, similar to many programming languages.
///
/// # Type Parameters
///
/// - `T`: The type of function stored (implementation, declaration, or either)
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::function::{FunctionOverloads, FunctionDeclOrImpl};
///
/// let mut overloads = FunctionOverloads::<FunctionDeclOrImpl>::new();
/// // Add different overloads for the same function name
/// ```
#[derive(Debug, Default, Clone)]
pub struct FunctionOverloads<T>(Vec<FunctionKindOverload<T>>);

impl<T: FunctionTypeOverload> FunctionOverloads<T> {
    /// Creates a new empty function overload collection.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Finds a function overload by member type and argument kinds.
    ///
    /// # Parameters
    ///
    /// - `member`: Whether to find member functions (true) or global functions (false)
    /// - `kinds`: Argument kinds to match
    ///
    /// # Returns
    ///
    /// Returns `Some(&FunctionKindOverload)` if found, `None` otherwise
    pub fn find(&self, member: bool, kinds: &[Kind]) -> Option<&FunctionKindOverload<T>> {
        self.0
            .iter()
            .find(|overload| overload.member() == member && overload.argument_kinds() == kinds)
    }

    /// Finds a mutable reference to a function overload by member flag and argument kinds.
    ///
    /// # Parameters
    ///
    /// - `member`: Whether to search for member functions
    /// - `kinds`: Argument kinds to match
    ///
    /// # Returns
    ///
    /// Mutable reference to the matching overload, or `None` if not found
    pub fn find_mut(&mut self, member: bool, kinds: &[Kind]) -> Option<&mut FunctionKindOverload<T>>
    where
        T: Send + Sync,
    {
        self.0
            .iter_mut()
            .find(|overload| overload.member() == member && overload.argument_kinds() == kinds)
    }

    /// Returns an iterator over all function overloads.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::function::{FunctionOverloads, FunctionDeclOrImpl};
    ///
    /// let overloads = FunctionOverloads::<FunctionDeclOrImpl<'_>>::new();
    /// for overload in overloads.entries() {
    ///     // Process each overload
    /// }
    /// ```
    pub fn entries(&self) -> impl Iterator<Item = &FunctionKindOverload<T>> {
        self.0.iter()
    }

    /// Returns a mutable iterator over all function overloads.
    pub fn entries_mut(&mut self) -> impl Iterator<Item = &mut FunctionKindOverload<T>> {
        self.0.iter_mut()
    }

    /// Clears all function overloads.
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Removes a function overload by member flag and argument kinds.
    ///
    /// # Parameters
    ///
    /// - `member`: Whether to remove member functions
    /// - `args`: Argument kinds to match for removal
    ///
    /// # Returns
    ///
    /// `Ok(())` if removal was successful, `Err(Error)` if not found
    pub fn remove<I, K>(&mut self, member: bool, args: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = K>,
        K: Into<Kind>,
    {
        let kinds = args.into_iter().map(|k| k.into()).collect::<Vec<_>>();
        if let Some(index) = self
            .0
            .iter()
            .position(|overload| overload.member() == member && overload.argument_kinds() == &kinds)
        {
            self.0.remove(index);
        } else {
            return Err(Error::not_found(format!(
                "Overload [{}] ({}) not found",
                if member { "member" } else { "global" },
                kinds
                    .iter()
                    .map(|k| k.to_string())
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
    /// This method adds a concrete function implementation to the appropriate overload
    /// group based on its signature. If no matching overload exists, a new one is created.
    ///
    /// # Arguments
    ///
    /// * `member` - Whether this is a member function
    /// * `f` - The function implementation to add
    ///
    /// # Returns
    ///
    /// `Ok(&mut Self)` for method chaining, or `Err(Error)` if addition fails
    ///
    /// # Errors
    ///
    /// Returns error if the function signature conflicts with existing registrations.
    pub fn add_impl(&mut self, member: bool, f: Function<'f>) -> Result<&mut Self, Error> {
        if member && f.arguments_len() == 0 {
            return Err(Error::invalid_argument("Member functions cannot have zero arguments"));
        }
        let kinds = f
            .arguments()
            .into_iter()
            .map(|t| t.kind())
            .collect::<Vec<_>>();
        if let Some(overload) = self.find_mut(member, &kinds) {
            overload.add_impl(f)?;
        } else {
            let mut overload = FunctionKindOverload::new(member, kinds);
            overload.add_impl(f)?;
            self.0.push(overload);
        }
        Ok(self)
    }

    /// Adds a function declaration to the overloads.
    ///
    /// This method adds a function type signature (declaration) to the appropriate
    /// overload group. Declarations provide type information for compile-time checking
    /// without requiring an implementation.
    ///
    /// # Arguments
    ///
    /// * `member` - Whether this is a member function declaration
    /// * `f` - The function type signature to add
    ///
    /// # Returns
    ///
    /// `Ok(&mut Self)` for method chaining, or `Err(Error)` if addition fails
    ///
    /// # Errors
    ///
    /// Returns error if the function signature conflicts with existing registrations.
    pub fn add_decl(&mut self, member: bool, f: FunctionType) -> Result<&mut Self, Error> {
        if member && f.arguments().is_empty() {
            return Err(Error::invalid_argument("Member functions cannot have zero arguments"));
        }
        let kinds = f.arguments().iter().map(|t| t.kind()).collect::<Vec<_>>();
        if let Some(overload) = self.find_mut(member, &kinds) {
            overload.add_decl(f)?;
        } else {
            let mut overload = FunctionKindOverload::new(member, kinds);
            overload.add_decl(f)?;
            self.0.push(overload);
        }
        Ok(self)
    }
}

impl<'f> FunctionOverloads<Function<'f>> {
    /// Adds a function to the overloads.
    ///
    /// This method adds a function to the appropriate overload group based on its
    /// signature. This is used when working with pure function implementations
    /// without separate declarations.
    ///
    /// # Arguments
    ///
    /// * `member` - Whether this is a member function
    /// * `f` - The function to add
    ///
    /// # Returns
    ///
    /// `Ok(&mut Self)` for method chaining, or `Err(Error)` if addition fails
    ///
    /// # Errors
    ///
    /// Returns error if the function signature conflicts with existing registrations.
    pub fn add(&mut self, member: bool, f: Function<'f>) -> Result<&mut Self, Error> {
        let kinds = f
            .arguments()
            .into_iter()
            .map(|t| t.kind())
            .collect::<Vec<_>>();
        if let Some(overload) = self.find_mut(member, &kinds) {
            overload.add(f)?;
        } else {
            let mut overload = FunctionKindOverload::new(member, kinds);
            overload.add(f)?;
            self.0.push(overload);
        }
        Ok(self)
    }
}

/// Trait for extracting argument types from function-like objects.
///
/// This trait provides a unified interface for getting argument type information
/// from different kinds of function objects (implementations, declarations, etc.).
/// It is used internally by the overload resolution system.
///
/// # Implementation Note
///
/// This trait is automatically implemented for function types that can provide
/// argument type information. It should not be implemented manually.
pub trait FunctionTypeOverload {
    /// Returns the argument types for this function.
    ///
    /// # Returns
    ///
    /// A vector of [`ValueType`] representing the function's argument types.
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

/// Function overload for a specific argument kind signature.
///
/// `FunctionKindOverload` groups function implementations and declarations that have
/// the same member flag and argument kinds. This allows for efficient lookup and
/// type checking during function resolution.
///
/// # Type Parameters
///
/// - `T`: The type of function stored (implementation or declaration)
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{Kind, function::{FunctionKindOverload, Function}};
///
/// // Create overload for member function taking (string, int)
/// let overload = FunctionKindOverload::<Function<'_>>::new(
///     true,
///     vec![Kind::String, Kind::Int]
/// );
/// ```
#[derive(Debug, Clone)]
pub struct FunctionKindOverload<T> {
    member: bool,
    argument_kinds: Vec<Kind>,
    entries: Vec<T>,
}

impl<T: FunctionTypeOverload> FunctionKindOverload<T> {
    /// Creates a new function kind overload.
    ///
    /// # Parameters
    ///
    /// - `member`: Whether this overload is for member functions
    /// - `argument_kinds`: The argument kinds this overload handles
    ///
    /// # Returns
    ///
    /// New `FunctionKindOverload` instance
    pub fn new(member: bool, argument_kinds: Vec<Kind>) -> Self {
        Self {
            member,
            argument_kinds,
            entries: Vec::new(),
        }
    }

    /// Returns whether this overload is for member functions.
    ///
    /// # Returns
    ///
    /// `true` if this overload handles member functions, `false` for global functions
    pub fn member(&self) -> bool {
        self.member
    }

    /// Returns the argument kinds for this overload.
    ///
    /// # Returns
    ///
    /// Reference to the vector of argument kinds this overload handles
    pub fn argument_kinds(&self) -> &Vec<Kind> {
        &self.argument_kinds
    }

    /// Finds a function by exact argument types.
    ///
    /// # Parameters
    ///
    /// - `args`: Argument types to match
    ///
    /// # Returns
    ///
    /// Reference to the matching function, or `None` if not found
    pub fn find(&self, args: &[ValueType]) -> Option<&T> {
        self.entries.iter().find(|entry| entry.arguments() == args)
    }

    /// Returns an iterator over all functions in this overload.
    ///
    /// # Returns
    ///
    /// Iterator over references to all functions in this overload
    pub fn entries(&self) -> impl Iterator<Item = &T> {
        self.entries.iter()
    }

    /// Returns a mutable iterator over all functions in this overload.
    ///
    /// # Returns
    ///
    /// Iterator over mutable references to all functions in this overload
    pub fn entries_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.entries.iter_mut()
    }

    /// Clears all functions from this overload.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Removes a function by exact argument types.
    ///
    /// # Parameters
    ///
    /// - `args`: Argument types to match for removal
    ///
    /// # Returns
    ///
    /// `Ok(())` if removal was successful, `Err(Error)` if not found
    pub fn remove(&mut self, args: &[ValueType]) -> Result<(), Error> {
        if let Some(index) = self
            .entries
            .iter()
            .position(|entry| entry.arguments() == args)
        {
            self.entries.remove(index);
        } else {
            return Err(Error::not_found(format!(
                "Overload [{}] ({}) not found",
                if self.member { "member" } else { "global" },
                args.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            )));
        }
        Ok(())
    }
}

impl<'f> FunctionKindOverload<FunctionDeclOrImpl<'f>> {
    /// Adds a function implementation to this overload group.
    ///
    /// This method adds a concrete function implementation to the overload group.
    /// The function signature must match the argument kinds of this overload group.
    ///
    /// # Arguments
    ///
    /// * `f` - The function implementation to add
    ///
    /// # Returns
    ///
    /// `Ok(&mut Self)` for method chaining, or `Err(Error)` if the function already exists
    ///
    /// # Errors
    ///
    /// Returns an error if a function with the same exact signature already exists.
    pub fn add_impl(&mut self, f: Function<'f>) -> Result<&mut Self, Error> {
        if let Some(_entry) = self.find(&f.arguments()) {
            return Err(Error::invalid_argument("Function already exists"));
        }
        self.entries.push(FunctionDeclOrImpl::new_impl(f));
        Ok(self)
    }

    /// Adds a function declaration to this overload group.
    ///
    /// This method adds a function type signature (declaration) to the overload group.
    /// The function signature must match the argument kinds of this overload group.
    ///
    /// # Arguments
    ///
    /// * `r#type` - The function type signature to add
    ///
    /// # Returns
    ///
    /// `Ok(&mut Self)` for method chaining, or `Err(Error)` if the function already exists
    ///
    /// # Errors
    ///
    /// Returns an error if a function with the same exact signature already exists.
    pub fn add_decl(&mut self, r#type: FunctionType) -> Result<&mut Self, Error> {
        if let Some(_entry) = self.find(r#type.arguments()) {
            return Err(Error::invalid_argument("Function already exists"));
        }
        self.entries.push(FunctionDeclOrImpl::new(r#type));
        Ok(self)
    }
}

impl<'f> FunctionKindOverload<Function<'f>> {
    /// Adds a function (implementation or declaration) to this overload.
    ///
    /// This is a convenience method that automatically determines whether to add
    /// an implementation or declaration based on the function type.
    ///
    /// # Type Parameters
    ///
    /// - `F`: Function type
    /// - `M`: Function marker (sync/async)
    /// - `E`: Error type
    /// - `R`: Return type
    /// - `A`: Argument tuple type
    ///
    /// # Parameters
    ///
    /// - `f`: Function to add
    ///
    /// # Returns
    ///
    /// Mutable reference to self for chaining, or error if addition failed
    pub fn add(&mut self, f: Function<'f>) -> Result<&mut Self, Error> {
        if let Some(_entry) = self.find(&f.arguments()) {
            return Err(Error::invalid_argument("Function already exists"));
        }
        self.entries.push(f);
        Ok(self)
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
