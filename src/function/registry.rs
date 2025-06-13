use std::collections::HashMap;
use super::*;

/// Compile-time function registry.
///
/// `FunctionRegistry` manages function declarations and implementations during the CEL environment
/// compilation phase. It maintains a mapping from function names to function overloads, where each
/// overload set can contain multiple function implementations with different signatures.
///
/// The registry supports two types of functions:
///
/// - **Function declarations**: Type signatures only for compile-time checking
/// - **Function implementations**: Callable code for runtime execution
///
/// # Function Types
///
/// Functions can be registered as either:
///
/// - **Global functions**: Callable from any context
/// - **Member functions**: Called as methods on values (e.g., `value.method()`)
///
/// # Lifetime Parameters
///
/// - `'f`: Lifetime of function implementations, allowing closures valid within specific scopes
///
/// # Examples
///
/// ```rust,no_run
/// use cel_cxx::{FunctionRegistry, Error};
///
/// let mut registry = FunctionRegistry::new();
///
/// // Register function implementations
/// fn add(a: i64, b: i64) -> Result<i64, Error> {
///     Ok(a + b)
/// }
/// registry.register_global("add", add)?;
///
/// // Register member functions
/// fn string_length(s: String) -> Result<i64, Error> {
///     Ok(s.len() as i64)
/// }
/// registry.register_member("length", string_length)?;
///
/// // Register function declarations (type signatures only)
/// registry.declare_global::<fn(String) -> Result<bool, Error>>("is_valid")?;
///
/// assert_eq!(registry.len(), 3);
/// # Ok::<(), cel_cxx::Error>(())
/// ```
#[derive(Debug, Default)]
pub struct FunctionRegistry<'f> {
    entries: HashMap<String, FunctionOverloads<FunctionDeclOrImpl<'f>>>,
}

impl<'f> FunctionRegistry<'f> {
    /// Creates a new empty function registry.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl<'f> FunctionRegistry<'f> {
    /// Registers a function implementation.
    ///
    /// Registers a callable function that can be invoked during CEL expression evaluation.
    /// The function is automatically converted to the appropriate [`FnImpl`] trait object.
    ///
    /// # Type Parameters
    ///
    /// - `F`: Function type, must implement [`FnImpl`]
    /// - `M`: Function marker (sync or async)
    /// - `E`: Error type, must be convertible to [`Error`]
    /// - `R`: Return type, must implement [`IntoValue`] and [`TypedValue`]
    /// - `A`: Argument tuple type, must implement [`TypedArguments`]
    ///
    /// # Parameters
    ///
    /// - `name`: Function name
    /// - `member`: Whether this is a member function (true) or global function (false)
    /// - `f`: Function implementation
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to support method chaining
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{FunctionRegistry, Error};
    ///
    /// let mut registry = FunctionRegistry::new();
    ///
    /// // Register global function
    /// fn multiply(a: i64, b: i64) -> Result<i64, Error> {
    ///     Ok(a * b)
    /// }
    /// registry.register("multiply", false, multiply)?;
    ///
    /// // Register member function
    /// fn is_empty(s: String) -> Result<bool, Error> {
    ///     Ok(s.is_empty())
    /// }
    /// registry.register("is_empty", true, is_empty)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn register<F, M, E, R, A>(
        &mut self, name: impl Into<String>, member: bool, f: F
    ) -> Result<&mut Self, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: FnMarker,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'a,
    {
        let name = name.into();
        let entry = self.entries
            .entry(name)
            .or_insert_with(FunctionOverloads::new);
        entry.add_impl(member, f)?;
        Ok(self)
    }

    /// Registers a member function implementation.
    ///
    /// Convenience method for registering member functions that can be called as methods
    /// on values (e.g., `"hello".length()`).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{FunctionRegistry, Error};
    ///
    /// let mut registry = FunctionRegistry::new();
    ///
    /// fn to_upper(s: String) -> Result<String, Error> {
    ///     Ok(s.to_uppercase())
    /// }
    ///
    /// registry.register_member("upper", to_upper)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn register_member<F, M, E, R, A>(
        &mut self, name: impl Into<String>, f: F
    ) -> Result<&mut Self, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: FnMarker,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'a,
    {
        self.register(name, true, f)
    }

    /// Registers a global function implementation.
    ///
    /// Convenience method for registering global functions that can be called from any context.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{FunctionRegistry, Error};
    ///
    /// let mut registry = FunctionRegistry::new();
    ///
    /// fn max(a: i64, b: i64) -> Result<i64, Error> {
    ///     Ok(a.max(b))
    /// }
    ///
    /// registry.register_global("max", max)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn register_global<F, M, E, R, A>(
        &mut self, name: impl Into<String>, f: F
    ) -> Result<&mut Self, Error>
    where
        F: FnImpl<M, E, R, A> + 'f,
        M: FnMarker,
        E: Into<Error> + Send + Sync + 'static,
        R: IntoValue + TypedValue + 'f,
        for <'a> A: TypedArguments + 'a,
    {
        self.register(name, false, f)
    }

    /// Declares a function signature.
    ///
    /// Declares a function type signature for compile-time type checking without providing
    /// an implementation. The function signature is determined by the generic parameter `D`
    /// which must implement [`FnDecl`].
    ///
    /// # Type Parameters
    ///
    /// - `D`: Function declaration type, must implement [`FnDecl`]
    ///
    /// # Parameters
    ///
    /// - `name`: Function name
    /// - `member`: Whether this is a member function (true) or global function (false)
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to support method chaining
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{FunctionRegistry, Error};
    ///
    /// let mut registry = FunctionRegistry::new();
    ///
    /// // Declare function signatures for external implementation
    /// registry.declare::<fn(String) -> Result<bool, Error>>("validate", false)?;
    /// registry.declare::<fn(i64, i64) -> Result<i64, Error>>("compute", false)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn declare<D>(&mut self, name: impl Into<String>, member: bool) -> Result<&mut Self, Error>
    where
        D: FnDecl,
    {
        let name = name.into();
        let result = D::result();
        let args = D::arguments();
        let entry = self.entries
            .entry(name)
            .or_insert_with(FunctionOverloads::new);
        entry.add_decl(member, result, args)?;
        Ok(self)
    }

    /// Declares a member function signature.
    ///
    /// Convenience method for declaring member function signatures.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{FunctionRegistry, Error};
    ///
    /// let mut registry = FunctionRegistry::new();
    ///
    /// // Declare member function signature
    /// registry.declare_member::<fn(String) -> Result<i64, Error>>("size")?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn declare_member<D>(
        &mut self, name: impl Into<String>
    ) -> Result<&mut Self, Error>
    where
        D: FnDecl,
    {
        self.declare::<D>(name, true)
    }

    /// Declares a global function signature.
    ///
    /// Convenience method for declaring global function signatures.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cel_cxx::{FunctionRegistry, Error};
    ///
    /// let mut registry = FunctionRegistry::new();
    ///
    /// // Declare global function signature
    /// registry.declare_global::<fn(String, String) -> Result<String, Error>>("concat")?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn declare_global<D>(
        &mut self, name: impl Into<String>
    ) -> Result<&mut Self, Error>
    where
        D: FnDecl,
    {
        self.declare::<D>(name, false)
    }

    /// Finds a function overload set by name.
    ///
    /// # Parameters
    ///
    /// - `name`: Function name to search for
    ///
    /// # Returns
    ///
    /// Returns `Some(&FunctionOverloads)` if found, `None` otherwise
    pub fn find(&self, name: &str) -> Option<&FunctionOverloads<FunctionDeclOrImpl<'f>>> {
        self.entries.get(name)
    }

    /// Finds a mutable function overload set by name.
    ///
    /// # Parameters
    ///
    /// - `name`: Function name to search for
    ///
    /// # Returns
    ///
    /// Returns `Some(&mut FunctionOverloads)` if found, `None` otherwise
    pub fn find_mut(&mut self, name: &str) -> Option<&mut FunctionOverloads<FunctionDeclOrImpl<'f>>> {
        self.entries.get_mut(name)
    }

    /// Returns an iterator over all function entries.
    ///
    /// The iterator yields `(name, overloads)` pairs for all registered functions.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &FunctionOverloads<FunctionDeclOrImpl<'f>>)> {
        self.entries
            .iter()
            .map(|(name, entry)| (name.as_str(), entry))
    }

    /// Returns a mutable iterator over all function entries.
    ///
    /// The iterator yields `(name, overloads)` pairs and allows modifying the overloads.
    pub fn entries_mut(&mut self) -> impl Iterator<Item = (&str, &mut FunctionOverloads<FunctionDeclOrImpl<'f>>)> {
        self.entries
            .iter_mut()
            .map(|(name, entry)| (name.as_str(), entry))
    }

    /// Removes a function by name.
    ///
    /// # Parameters
    ///
    /// - `name`: Function name to remove
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if successfully removed, or an error if the function doesn't exist
    pub fn remove(&mut self, name: &str) -> Result<(), Error> {
        if self.entries.remove(name).is_none() {
            return Err(Error::not_found(format!("Function {} not found", name)));
        }
        Ok(())
    }

    /// Clears all function entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns the number of function entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}


#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use std::{collections::HashMap, sync::Arc};

    use crate::{Error, types::{OpaqueType, Type}, values::{TypedOpaqueValue, Value}, values::OpaqueValue};

    use super::*;

    fn f1(v: i64) -> Result<i64, Error> { Ok(5)}
    fn f11(a1: i64, a2: i64) -> Result<i64, Error> { Ok(5)}

    fn f2(a1: HashMap<i64, i64>) -> Result<i64, Error> { Ok(5)}
    fn f3(a1: String) -> Result<i64, Error> { Ok(5)}
    fn f4(a1: String, a2: String) -> Result<i64, Error> { Ok(5)}
    fn f5(a1: String, a2: String, a3: HashMap<u64, Vec<Vec<i64>>>) -> Result<i64, Error> { Ok(5)}

    #[derive(Debug, Clone)]
    struct MyOpaque;

    impl std::fmt::Display for MyOpaque {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyOpaque")
        }
    }

    impl TypedOpaqueValue for MyOpaque {
        fn opaque_type() -> crate::types::OpaqueType {
            OpaqueType::new("fff", vec![])
        }
    }
    impl PartialEq for MyOpaque {
        fn eq(&self, other: &Self) -> bool {
            true
        }
    }

    fn f6(a1: String, a2: Opaque<MyOpaque>, a3: HashMap<u64, Vec<Vec<i64>>>) -> Result<i64, Error> { Ok(5)}

    async fn async_f1(a1: String, a2: i64, a3: Option<String>) -> Result<i64, Error> { Ok(5)}

    #[test]
    fn test_register() -> Result<(), Error>{
        let n = 100;
        let lifetime_closure = |a: i64, b: i64| {
            Ok::<_, Error>(a + b + n)
        };
        let mut registry = FunctionRegistry::new();
        registry
            .register_member("f1", f1)?
            .register_member("f11", f11)?
            .register_member("f2", f2)?
            .register_member("f3", f3)?
            .register_member("f4", f4)?
            .register_member("f5", f5)?
            .register_global("lifetime_closure", lifetime_closure)?
            .register_global("f6", f6)?
        ;
        
        #[cfg(feature = "async")]
        let registry = registry.register_global("async_f1", async_f1)?;

        //let variable = VariableDecl::builder()
        //    .add::<i64, _>("v1")
        //    .build(None);
        //let env = Env::new(function, variable);

        let x = MyOpaque;
        let y: Box<dyn OpaqueValue> = Box::new(x);

        let value = Opaque::new(MyOpaque);
        assert_eq!(value.is::<MyOpaque>(), true);
        assert_eq!(value.downcast_ref::<MyOpaque>().is_some(), true);
        let value2 = value.clone().downcast::<MyOpaque>();
        assert_eq!(value2.is_ok(), true);

        let value3 = value.into_inner();
        assert_eq!(value3.is::<MyOpaque>(), true);

        let value = Opaque(MyOpaque);
        let value4 = value.upcast();
        assert_eq!(value4.is::<MyOpaque>(), true);
        assert_eq!(value4.clone().downcast::<MyOpaque>().is_ok(), true);
        let value5 = value4.downcast::<MyOpaque>();
        assert_eq!(value5.is_ok(), true);

        Ok(())
    }
}
