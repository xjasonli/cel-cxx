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
    /// Registers a function implementation with zero-annotation type inference.
    ///
    /// Registers a callable function that can be invoked during CEL expression evaluation.
    /// The function signature is automatically inferred from the Rust function type, eliminating
    /// the need for manual type annotations.
    ///
    /// # Zero-Annotation Features
    ///
    /// - **Automatic type inference**: Function signature extracted from Rust function type
    /// - **Lifetime handling**: Safe conversion of borrowed arguments like `&str`
    /// - **Error conversion**: Automatic conversion of `Result<T, E>` return types
    /// - **Async support**: Seamless handling of both sync and async functions
    ///
    /// # Type Parameters
    ///
    /// - `F`: Function type, must implement [`IntoFunction`]
    /// - `Fm`: Function marker (sync or async), automatically inferred
    /// - `Args`: Argument tuple type, automatically inferred from function signature
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
    /// ## Basic functions with automatic type inference
    ///
    /// ```rust,no_run
    /// use cel_cxx::FunctionRegistry;
    ///
    /// let mut registry = FunctionRegistry::new();
    ///
    /// // Zero-annotation function registration
    /// registry.register("add", false, |a: i64, b: i64| a + b)?;
    /// registry.register("greet", false, |name: &str| format!("Hello, {}!", name))?;
    /// registry.register("is_empty", true, |s: String| s.is_empty())?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    ///
    /// ## Error handling with automatic conversion
    ///
    /// ```rust,no_run
    /// use cel_cxx::{FunctionRegistry, Error};
    ///
    /// let mut registry = FunctionRegistry::new();
    ///
    /// // Functions returning Result are automatically handled
    /// registry.register("divide", false, |a: i64, b: i64| -> Result<i64, Error> {
    ///     if b == 0 {
    ///         Err(Error::invalid_argument("Division by zero"))
    ///     } else {
    ///         Ok(a / b)
    ///     }
    /// })?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn register<F, Fm, Args>(
        &mut self, name: impl Into<String>, member: bool, f: F
    ) -> Result<&mut Self, Error>
    where
        F: IntoFunction<'f, Fm, Args>,
        Fm: FnMarker,
        Args: Arguments,
    {
        let name = name.into();
        let entry = self.entries
            .entry(name)
            .or_insert_with(FunctionOverloads::new);
        entry.add_impl(member, f.into_function())?;
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
    pub fn register_member<F, Fm, Args>(
        &mut self, name: impl Into<String>, f: F
    ) -> Result<&mut Self, Error>
    where
        F: IntoFunction<'f, Fm, Args>,
        Fm: FnMarker,
        Args: Arguments,
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
    pub fn register_global<F, Fm, Args>(
        &mut self, name: impl Into<String>, f: F
    ) -> Result<&mut Self, Error>
    where
        F: IntoFunction<'f, Fm, Args>,
        Fm: FnMarker,
        Args: Arguments,
    {
        self.register(name, false, f)
    }

    /// Declares a function signature.
    ///
    /// Declares a function type signature for compile-time type checking without providing
    /// an implementation. The function signature is determined by the generic parameter `D`
    /// which must implement [`FunctionDecl`].
    ///
    /// # Type Parameters
    ///
    /// - `D`: Function declaration type, must implement [`FunctionDecl`]
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
    /// // Declare global function signature
    /// registry.declare::<fn(String) -> Result<bool, Error>>("validate", false)?;
    ///
    /// // Declare member function signature  
    /// registry.declare::<fn(String) -> Result<i64, Error>>("size", true)?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn declare<D>(&mut self, name: impl Into<String>, member: bool) -> Result<&mut Self, Error>
    where
        D: FunctionDecl,
    {
        let name = name.into();
        let entry = self.entries
            .entry(name)
            .or_insert_with(FunctionOverloads::new);
        entry.add_decl(member, D::function_type())?;
        Ok(self)
    }

    /// Declares a member function signature.
    ///
    /// Convenience method for declaring member function signatures for compile-time
    /// type checking without providing implementations.
    ///
    /// # Type Parameters
    ///
    /// - `D`: Function declaration type, must implement [`FunctionDecl`]
    ///
    /// # Parameters
    ///
    /// - `name`: Function name
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
    /// // Declare member function signature
    /// registry.declare_member::<fn(String) -> i64>("hash")?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn declare_member<D>(
        &mut self, name: impl Into<String>
    ) -> Result<&mut Self, Error>
    where
        D: FunctionDecl,
    {
        self.declare::<D>(name, true)
    }

    /// Declares a global function signature.
    ///
    /// Convenience method for declaring global function signatures for compile-time
    /// type checking without providing implementations.
    ///
    /// # Type Parameters
    ///
    /// - `D`: Function declaration type, must implement [`FunctionDecl`]
    ///
    /// # Parameters
    ///
    /// - `name`: Function name
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
    /// // Declare global function signature
    /// registry.declare_global::<fn(String, String) -> Result<String, Error>>("concat")?;
    /// # Ok::<(), cel_cxx::Error>(())
    /// ```
    pub fn declare_global<D>(
        &mut self, name: impl Into<String>
    ) -> Result<&mut Self, Error>
    where
        D: FunctionDecl,
    {
        self.declare::<D>(name, false)
    }

    /// Finds function overloads by name.
    ///
    /// # Parameters
    ///
    /// - `name`: Function name to search for
    ///
    /// # Returns
    ///
    /// `Some(&FunctionOverloads)` if found, `None` if not found
    pub fn find(&self, name: &str) -> Option<&FunctionOverloads<FunctionDeclOrImpl<'f>>> {
        self.entries.get(name)
    }

    /// Finds function overloads by name (mutable).
    ///
    /// # Parameters
    ///
    /// - `name`: Function name to search for
    ///
    /// # Returns
    ///
    /// `Some(&mut FunctionOverloads)` if found, `None` if not found
    pub fn find_mut(&mut self, name: &str) -> Option<&mut FunctionOverloads<FunctionDeclOrImpl<'f>>> {
        self.entries.get_mut(name)
    }

    /// Returns an iterator over all function entries.
    ///
    /// # Returns
    ///
    /// Iterator yielding `(&str, &FunctionOverloads)` pairs
    pub fn entries(&self) -> impl Iterator<Item = (&str, &FunctionOverloads<FunctionDeclOrImpl<'f>>)> {
        self.entries.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// Returns a mutable iterator over all function entries.
    ///
    /// # Returns
    ///
    /// Iterator yielding `(&str, &mut FunctionOverloads)` pairs
    pub fn entries_mut(&mut self) -> impl Iterator<Item = (&str, &mut FunctionOverloads<FunctionDeclOrImpl<'f>>)> {
        self.entries.iter_mut().map(|(k, v)| (k.as_str(), v))
    }

    /// Removes all overloads for a function name.
    ///
    /// # Parameters
    ///
    /// - `name`: Function name to remove
    ///
    /// # Returns
    ///
    /// `Ok(())` if removal was successful, `Err(Error)` if function not found
    pub fn remove(&mut self, name: &str) -> Result<(), Error> {
        self.entries.remove(name).ok_or_else(|| Error::not_found(format!("Function '{}' not found", name)))?;
        Ok(())
    }

    /// Clears all registered functions.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns the number of registered function names.
    ///
    /// Note: This counts function names, not individual overloads.
    ///
    /// # Returns
    ///
    /// Number of registered function names
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the registry is empty.
    ///
    /// # Returns
    ///
    /// `true` if no functions are registered, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}


#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use std::{collections::{BTreeMap, HashMap}, sync::Arc};
    use crate::{Opaque, Error, types::{OpaqueType, ValueType}, values::{TypedOpaque, Value}, values::OpaqueValue, values::Optional};

    use super::*;

    fn f1(v: i64) -> Result<i64, Error> { Ok(5)}
    fn f11(a1: i64, a2: i64) -> Result<i64, Error> { Ok(5)}

    fn f2(a1: HashMap<i64, i64>) -> Result<i64, Error> { Ok(5)}
    fn f3(a1: String) -> Result<i64, Error> { Ok(5)}
    fn f4(a1: String, a2: String) -> Result<i64, Error> { Ok(5)}
    fn f5(a1: String, a2: String, a3: HashMap<u64, Vec<Vec<i64>>>) -> Result<i64, Error> { Ok(5)}

    #[derive(Opaque, Debug, Clone, PartialEq)]
    #[cel_cxx(type = "MyOpaque", crate = crate)]
    struct MyOpaque(pub i64);

    impl std::fmt::Display for MyOpaque {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyOpaque({})", self.0)
        }
    }

    fn f6(a1: String, a2: MyOpaque, a3: HashMap<u64, Vec<Vec<i64>>>) -> Result<i64, Error> { Ok(5)}

    fn f7(a1: &str) -> Result<(), Error> { Ok(())}

    fn f8(a1: &MyOpaque) -> &MyOpaque { a1 }

    fn f9(a1: Optional<&MyOpaque>) -> Result<Option<&MyOpaque>, Error> { Ok(a1.into_option()) }

    async fn async_f1(a1: String, a2: i64, a3: Option<String>) -> Result<i64, Error> { Ok(5)}

    fn f_map1(a1: HashMap<String, Vec<u8>>) -> Result<(), Error> { Ok(()) }
    fn f_map2(a1: HashMap<&str, &[u8]>) -> Result<(), Error> { Ok(()) }
    fn f_map3<'a>(a1: HashMap<&'a str, &'a str>) -> Result<BTreeMap<&'a str, &'a str>, Error> {
        Ok(BTreeMap::from_iter(a1.into_iter().map(|(k, v)| (k, v))))
    }

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
            .register_global("f7", f7)?
            .register_global("f8", f8)?
            .register_global("f9", f9)?
            .register_global("f_map1", f_map1)?
            .register_global("f_map2", f_map2)?
            .register_global("f_map3", f_map3)?
        ;

        
        #[cfg(feature = "async")]
        let registry = registry.register_global("async_f1", async_f1)?;

        //let variable = VariableDecl::builder()
        //    .add::<i64, _>("v1")
        //    .build(None);
        //let env = Env::new(function, variable);

        let x = MyOpaque(1);
        let y: Box<dyn Opaque> = Box::new(x);

        let value: OpaqueValue = Box::new(MyOpaque(1));
        assert_eq!(value.is::<MyOpaque>(), true);
        assert_eq!(value.downcast_ref::<MyOpaque>().is_some(), true);
        let value2 = value.clone().downcast::<MyOpaque>();
        assert_eq!(value2.is_ok(), true);

        let value3 = value.clone();
        assert_eq!(value3.is::<MyOpaque>(), true);

        let value4: OpaqueValue = Box::new(MyOpaque(1));
        assert_eq!(value4.is::<MyOpaque>(), true);
        assert_eq!(value4.clone().downcast::<MyOpaque>().is_ok(), true);
        let value5 = value4.downcast::<MyOpaque>();
        assert_eq!(value5.is_ok(), true);

        Ok(())
    }
}
