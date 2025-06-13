use std::sync::Arc;
use super::*;

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
    pub fn find(&self, member: bool, kinds: &[Kind]) -> Option<&FunctionKindOverload<T>>
    {
        self.0.iter()
            .find(|overload| {
                overload.member() == member && overload.argument_kinds() == &kinds
            })
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
        self.0.iter_mut()
            .find(|overload| {
                overload.member() == member && overload.argument_kinds() == &kinds
            })
    }

    /// Returns an iterator over all function overloads.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::function::FunctionOverloads;
    ///
    /// let overloads = FunctionOverloads::new();
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
        if let Some(index) = self.0.iter()
            .position(|overload| {
                overload.member() == member && overload.argument_kinds() == &kinds
            }) {
            self.0.remove(index);
        } else {
            return Err(Error::not_found(
                format!(
                    "Overload [{}] ({}) not found",
                    if member { "member" } else { "global" },
                    kinds.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(", "),
                )
            ));
        }
        Ok(())
    }
}

impl<'f> FunctionOverloads<FunctionDeclOrImpl<'f>> {
    /// Adds a function implementation to the overloads.
    ///
    /// # Type Parameters
    ///
    /// - `F`: Function implementation type
    /// - `M`: Function marker (sync/async)
    /// - `E`: Error type
    /// - `R`: Return type
    /// - `A`: Argument tuple type
    ///
    /// # Parameters
    ///
    /// - `member`: Whether this is a member function
    /// - `f`: Function implementation to add
    ///
    /// # Returns
    ///
    /// Mutable reference to self for chaining, or error if addition failed
    pub fn add_impl<F, M, E, R, A>(&mut self, member: bool, f: F) -> Result<&mut Self, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: 'f,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'f,
    {
        let erased = f.erased();
        let kinds = erased.argument_kinds();
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
    /// # Parameters
    ///
    /// - `member`: Whether this is a member function
    /// - `result`: Return type of the function
    /// - `args`: Argument types of the function
    ///
    /// # Returns
    ///
    /// Mutable reference to self for chaining, or error if addition failed
    pub fn add_decl<I>(&mut self, member: bool, result: Type, args: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = Type>,
    {
        let args = args.into_iter().collect::<Vec<_>>();
        let kinds = args.iter().map(|t| t.kind()).collect::<Vec<_>>();
        if let Some(overload) = self.find_mut(member, &kinds) {
            overload.add_decl(result, args)?;
        } else {
            let mut overload = FunctionKindOverload::new(member, kinds);
            overload.add_decl(result, args)?;
            self.0.push(overload);
        }
        Ok(self)
    }

    /// Adds a function (implementation or declaration) to the overloads.
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
    /// - `member`: Whether this is a member function
    /// - `f`: Function to add
    ///
    /// # Returns
    ///
    /// Mutable reference to self for chaining, or error if addition failed
    pub fn add<F, M, E, R, A>(&mut self, member: bool, f: F) -> Result<&mut Self, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: 'f,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'f,
    {
        self.add_impl(member, f)
    }
}

impl<'f> FunctionOverloads<FunctionImpl<'f>> {
    /// Adds a function implementation to the function implementation overloads.
    ///
    /// This method is specifically for adding function implementations to
    /// collections that only store implementations (not declarations).
    ///
    /// # Type Parameters
    ///
    /// - `F`: Function implementation type
    /// - `M`: Function marker (sync/async)
    /// - `E`: Error type
    /// - `R`: Return type
    /// - `A`: Argument tuple type
    ///
    /// # Parameters
    ///
    /// - `member`: Whether this is a member function
    /// - `f`: Function implementation to add
    ///
    /// # Returns
    ///
    /// Mutable reference to self for chaining, or error if addition failed
    pub fn add<F, M, E, R, A>(&mut self, member: bool, f: F) -> Result<&mut Self, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: 'f,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'f,
    {
        let erased = f.erased();
        if let Some(overload) = self.find_mut(member, &erased.argument_kinds()) {
            overload.add(f)?;
        } else {
            let mut overload = FunctionKindOverload::new(member, erased.argument_kinds());
            overload.add(f)?;
            self.0.push(overload);
        }
        Ok(self)
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
/// use cel_cxx::{Kind, function::FunctionKindOverload};
///
/// // Create overload for member function taking (string, int)
/// let overload = FunctionKindOverload::new(
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
    pub fn find(&self, args: &[Type]) -> Option<&T> {
        self.entries.iter()
            .find(|entry| &entry.arguments() == args)
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
    pub fn remove(&mut self, args: &[Type]) -> Result<(), Error> {
        if let Some(index) = self.entries.iter()
            .position(|entry| &entry.arguments() == args) {
            self.entries.remove(index);
        } else {
            return Err(Error::not_found(
                format!("Overload [{}] ({}) not found",
                    if self.member { "member" } else { "global" },
                    args.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "),
                )
            ));
        }
        Ok(())
    }
}

impl<'f> FunctionKindOverload<FunctionDeclOrImpl<'f>> {
    /// Adds a function implementation to this overload.
    ///
    /// # Type Parameters
    ///
    /// - `F`: Function implementation type
    /// - `M`: Function marker (sync/async)
    /// - `E`: Error type
    /// - `R`: Return type
    /// - `A`: Argument tuple type
    ///
    /// # Parameters
    ///
    /// - `f`: Function implementation to add
    ///
    /// # Returns
    ///
    /// Mutable reference to self for chaining, or error if addition failed
    pub fn add_impl<F, M, E, R, A>(&mut self, f: F) -> Result<&mut Self, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: 'f,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'f,
    {
        let erased = f.erased();
        if let Some(_entry) = self.find(&erased.arguments()) {
            return Err(Error::invalid_argument("Function already exists"));
        }
        self.entries.push(FunctionDeclOrImpl::new_impl(f));
        Ok(self)
    }

    /// Adds a function declaration to this overload.
    ///
    /// # Parameters
    ///
    /// - `result`: Return type of the function
    /// - `args`: Argument types of the function
    ///
    /// # Returns
    ///
    /// Mutable reference to self for chaining, or error if addition failed
    pub fn add_decl<I>(&mut self, result: Type, args: I) -> Result<&mut Self, Error>
    where
        I: IntoIterator<Item = Type>,
    {
        let args = args.into_iter().collect::<Vec<_>>();
        if let Some(_entry) = self.find(&args) {
            return Err(Error::invalid_argument("Function already exists"));
        }
        self.entries.push(FunctionDeclOrImpl::new_decl(result, args));
        Ok(self)
    }
}

impl<'f> FunctionKindOverload<FunctionImpl<'f>> {
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
    pub fn add<F, M, E, R, A>(&mut self, f: F) -> Result<&mut Self, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: 'f,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'f,
    {
        let erased = f.erased();
        if let Some(_entry) = self.find(&erased.arguments()) {
            return Err(Error::invalid_argument("Function already exists"));
        }
        self.entries.push(Arc::new(f.into_erased()));
        Ok(self)
    }
}

/// Trait for function type overloads.
///
/// `FunctionTypeOverload` provides a common interface for accessing type information
/// from function declarations and implementations. This trait is used internally
/// for type checking and function resolution.
pub trait FunctionTypeOverload: Send + Sync {
    /// Returns the return type of the function.
    fn result(&self) -> Type;
    
    /// Returns the argument types of the function.
    fn arguments(&self) -> Vec<Type>;
}

impl<'f> FunctionTypeOverload for FunctionImpl<'f> {
    fn result(&self) -> Type {
        ErasedFnImpl::result(&**self)
    }

    fn arguments(&self) -> Vec<Type> {
        ErasedFnImpl::arguments(&**self)
    }
}

/// Function declaration with type signature.
///
/// `FunctionDecl` represents a function declaration that provides only type information
/// without an actual implementation. This is used for compile-time type checking
/// when the implementation will be provided at runtime.
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{Type, function::FunctionDecl};
///
/// // Declare a function: (string, int) -> string
/// let decl = FunctionDecl::new(
///     Type::String,
///     vec![Type::String, Type::Int]
/// );
/// ```
#[derive(Debug)]
pub struct FunctionDecl {
    result: Type,
    arguments: Vec<Type>,
}

impl FunctionDecl {
    /// Creates a new function declaration.
    ///
    /// # Parameters
    ///
    /// - `result`: Return type of the function
    /// - `args`: Argument types of the function
    ///
    /// # Returns
    ///
    /// New `FunctionDecl` instance
    pub fn new<I>(result: Type, args: I) -> Self
    where
        I: IntoIterator<Item = Type>,
    {
        Self { result, arguments: args.into_iter().collect() }
    }

    /// Returns the return type of the function.
    ///
    /// # Returns
    ///
    /// Clone of the function's return type
    pub fn result(&self) -> Type {
        self.result.clone()
    }

    /// Returns the argument types of the function.
    ///
    /// # Returns
    ///
    /// Clone of the function's argument types
    pub fn arguments(&self) -> Vec<Type> {
        self.arguments.clone()
    }
}

impl FunctionTypeOverload for FunctionDecl {
    fn result(&self) -> Type {
        self.result.clone()
    }
    fn arguments(&self) -> Vec<Type> {
        self.arguments.clone()
    }
}

/// Function declaration or implementation.
///
/// `FunctionDeclOrImpl` is an enum that can hold either a function declaration
/// (type signature only) or a function implementation (callable code). This allows
/// the function system to handle both compile-time type checking and runtime execution.
///
/// # Variants
///
/// - `Decl`: Function declaration with type information only
/// - `Impl`: Function implementation (callable code)
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{Type, function::{FunctionDecl, FunctionDeclOrImpl}};
///
/// // Create a declaration
/// let decl = FunctionDecl::new(Type::String, vec![Type::Int]);
/// let decl_or_impl = FunctionDeclOrImpl::Decl(decl);
///
/// // Check what type it is
/// assert!(decl_or_impl.is_decl());
/// assert!(!decl_or_impl.is_impl());
/// ```
pub enum FunctionDeclOrImpl<'f> {
    /// Function declaration (type signature only)
    Decl(FunctionDecl),
    /// Function implementation (callable code)
    Impl(FunctionImpl<'f>),
}

impl<'f> FunctionTypeOverload for FunctionDeclOrImpl<'f> {
    fn result(&self) -> Type {
        match self {
            FunctionDeclOrImpl::Decl(d) => d.result(),
            FunctionDeclOrImpl::Impl(i) => i.result(),
        }
    }

    fn arguments(&self) -> Vec<Type> {
        match self {
            FunctionDeclOrImpl::Decl(d) => d.arguments(),
            FunctionDeclOrImpl::Impl(i) => i.arguments(),
        }
    }
}
impl<'f> FunctionDeclOrImpl<'f> {
    /// Creates a new function implementation variant.
    ///
    /// # Type Parameters
    ///
    /// - `F`: Function implementation type
    /// - `M`: Function marker (sync/async)
    /// - `E`: Error type
    /// - `R`: Return type
    /// - `A`: Argument tuple type
    ///
    /// # Parameters
    ///
    /// - `f`: Function implementation
    ///
    /// # Returns
    ///
    /// New `FunctionDeclOrImpl::Impl` variant
    pub fn new_impl<F, M, E, R, A>(f: F) -> Self
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: 'f,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for<'a> A: TypedArguments + 'f,
    {
        Self::Impl(Arc::new(f.into_erased()))
    }

    /// Creates a new function declaration variant.
    ///
    /// # Parameters
    ///
    /// - `result`: Return type of the function
    /// - `args`: Argument types of the function
    ///
    /// # Returns
    ///
    /// New `FunctionDeclOrImpl::Decl` variant
    pub fn new_decl<I>(result: Type, args: I) -> Self
    where
        I: IntoIterator<Item = Type>,
    {
        Self::Decl(FunctionDecl::new(result, args))
    }

    /// Returns `true` if this is a function implementation.
    ///
    /// # Returns
    ///
    /// `true` if this contains an implementation, `false` if it contains a declaration
    pub fn is_impl(&self) -> bool {
        matches!(self, FunctionDeclOrImpl::Impl(_))
    }

    /// Returns `true` if this is a function declaration.
    ///
    /// # Returns
    ///
    /// `true` if this contains a declaration, `false` if it contains an implementation
    pub fn is_decl(&self) -> bool {
        matches!(self, FunctionDeclOrImpl::Decl(_))
    }

    /// Returns a reference to the function implementation, if this is an implementation.
    ///
    /// # Returns
    ///
    /// `Some(&FunctionImpl)` if this contains an implementation, `None` if it contains a declaration
    pub fn as_impl(&self) -> Option<&FunctionImpl<'f>> {
        match self {
            FunctionDeclOrImpl::Decl(_) => None,
            FunctionDeclOrImpl::Impl(f) => Some(f),
        }
    }

    /// Returns a reference to the function declaration, if this is a declaration.
    ///
    /// # Returns
    ///
    /// `Some(&FunctionDecl)` if this contains a declaration, `None` if it contains an implementation
    pub fn as_decl(&self) -> Option<&FunctionDecl> {
        match self {
            FunctionDeclOrImpl::Decl(d) => Some(d),
            FunctionDeclOrImpl::Impl(_) => None,
        }
    }

    /// Converts this into a function implementation, if this is an implementation.
    ///
    /// # Returns
    ///
    /// `Some(FunctionImpl)` if this contains an implementation, `None` if it contains a declaration
    pub fn into_impl(self) -> Option<FunctionImpl<'f>> {
        match self {
            FunctionDeclOrImpl::Decl(_) => None,
            FunctionDeclOrImpl::Impl(i) => Some(i),
        }
    }

    /// Converts this into a function declaration, if this is a declaration.
    ///
    /// # Returns
    ///
    /// `Some(FunctionDecl)` if this contains a declaration, `None` if it contains an implementation
    pub fn into_decl(self) -> Option<FunctionDecl> {
        match self {
            FunctionDeclOrImpl::Decl(d) => Some(d),
            FunctionDeclOrImpl::Impl(_) => None,
        }
    }
}

impl<'f> std::fmt::Debug for FunctionDeclOrImpl<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionDeclOrImpl::Decl(d) => f.debug_tuple("Decl").field(d).finish(),
            FunctionDeclOrImpl::Impl(_) => f.debug_tuple("Impl").field(&"<function>").finish(),
        }
    }
}
