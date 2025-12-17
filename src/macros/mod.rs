//! CEL macro system for extending the expression language.
//!
//! This module provides functionality for defining and registering custom macros
//! that can expand CEL expressions at compile time. Macros enable syntactic sugar
//! and domain-specific language extensions without modifying the core parser.
//!
//! # Overview
//!
//! CEL macros transform parsed expressions into new expression trees before type
//! checking and evaluation. This allows you to:
//!
//! - Add new syntactic constructs (e.g., `has(foo.bar)` for presence testing)
//! - Implement domain-specific operators (e.g., `all(list, predicate)`)
//! - Optimize common patterns by rewriting expressions
//! - Extend CEL with custom comprehension forms
//!
//! # Macro Types
//!
//! There are two types of macros:
//!
//! - **Global macros**: Called as functions, e.g., `all([1, 2, 3], x, x > 0)`
//! - **Receiver macros**: Called as methods, e.g., `list.filter(x, x > 0)`
//!
//! Both types can accept fixed or variable numbers of arguments.
//!
//! # Examples
//!
//! ## Defining a global macro
//!
//! ```rust,no_run
//! use cel_cxx::macros::{Macro, MacroExprFactory, Expr};
//!
//! // Macro that creates a constant expression with doubled value
//! let double_macro = Macro::new_global("double", 1, |factory, args| {
//!     if let Some(expr) = args.first() {
//!         if let Some(val) = expr.kind().and_then(|k| k.as_constant()) {
//!             if let Some(int_val) = val.as_int() {
//!                 return Some(factory.new_const(int_val * 2));
//!             }
//!         }
//!     }
//!     None
//! });
//! ```
//!
//! ## Defining a receiver macro
//!
//! ```rust,no_run
//! # use cel_cxx::macros::{Macro, MacroExprFactory, Expr};
//! // Macro for optional element access: list.maybe_get(index)
//! let maybe_get_macro = Macro::new_receiver("maybe_get", 1, |factory, target, args| {
//!     // Implementation would handle optional list indexing
//!     Some(factory.new_call("_?.[]", &[target, args[0].clone()]))
//! });
//! ```

use crate::Error;
use crate::ffi::{
    Macro as FfiMacro,
    Expr as FfiExpr,
    GlobalMacroExpander as FfiGlobalMacroExpander,
    ReceiverMacroExpander as FfiReceiverMacroExpander,
};

mod expr;
mod factory;
mod expander;

pub use expr::*;
pub use factory::*;
pub use expander::*;

/// A CEL macro that expands expressions at compile time.
///
/// A `Macro` represents a compile-time transformation rule that can be registered
/// with a CEL environment. When the parser encounters a matching function or method
/// call, the macro's expander is invoked to transform the expression tree.
///
/// # Macro Expansion
///
/// During compilation, macros are expanded before type checking:
/// 1. Parser identifies function/method calls matching registered macros
/// 2. Macro expander receives the call expression and its arguments
/// 3. Expander returns a new expression tree or `None` to keep original
/// 4. Type checker validates the expanded expression
///
/// # Thread Safety
///
/// The expander functions captured in macros must be `Send + Sync + 'static`,
/// ensuring thread-safe usage across the CEL environment.
///
/// # Error Handling
///
/// Macro creation methods return `Result<Self, Error>` and can fail if:
/// - The macro name is invalid
/// - Internal FFI allocation fails
/// - The expander closure cannot be registered
///
/// # Examples
///
/// ## Fixed argument count
///
/// ```rust,no_run
/// # use cel_cxx::macros::{Macro, MacroExprFactory, Expr};
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // A macro that requires exactly 1 argument
/// let add_one_macro = Macro::new_global("add_one", 1, |factory, mut args| {
///     let arg = args.pop()?;
///     Some(factory.new_call("_+_", &[arg, factory.new_const(1)]))
/// })?;
/// # Ok(())
/// # }
/// ```
///
/// ## Variable argument count
///
/// ```rust,no_run
/// # use cel_cxx::macros::{Macro, MacroExprFactory, Expr};
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // A macro that accepts any number of arguments
/// let debug_macro = Macro::new_global_var_arg("debug", |factory, args| {
///     // Wrap all arguments in a list for debugging
///     let elements: Vec<_> = args.iter()
///         .map(|arg| factory.new_list_element(arg, false))
///         .collect();
///     Some(factory.new_list(&elements))
/// })?;
/// # Ok(())
/// # }
/// ```
pub struct Macro(pub(crate) cxx::UniquePtr<FfiMacro>);

impl Macro {
    /// Creates a new global macro with a fixed number of arguments.
    ///
    /// Global macros are invoked like regular functions, e.g., `macro_name(arg1, arg2)`.
    /// The macro will only be expanded when called with exactly `argument_count` arguments.
    ///
    /// # Parameters
    ///
    /// - `name`: The function name that triggers this macro (as a string reference)
    /// - `argument_count`: The exact number of arguments required
    /// - `expander`: The expansion function that transforms the expression
    ///
    /// # Returns
    ///
    /// - `Ok(Macro)`: Successfully created macro
    /// - `Err(Error)`: Failed to create macro (e.g., invalid name or FFI error)
    ///
    /// # Expander Function
    ///
    /// The expander receives:
    /// - `factory`: A mutable reference to [`MacroExprFactory`] for creating new expressions
    /// - `args`: A vector of argument expressions
    ///
    /// The expander should return:
    /// - `Some(Expr)`: The expanded expression to replace the original call
    /// - `None`: Keep the original expression unchanged
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{Macro, MacroExprFactory, Expr};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Macro: not_zero(x) expands to x != 0
    /// let macro_def = Macro::new_global("not_zero", 1, |factory, mut args| {
    ///     let arg = args.pop()?;
    ///     Some(factory.new_call("_!=_", &[arg, factory.new_const(0)]))
    /// })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_global(
        name: impl AsRef<str>,
        argument_count: usize,
        expander: impl GlobalMacroExpander + 'static,
    ) -> Result<Self, Error> {
        let ffi_expander: cxx::UniquePtr<FfiGlobalMacroExpander> = FfiGlobalMacroExpander::new(
            move |ffi_factory, ffi_expr| -> cxx::UniquePtr<FfiExpr> {
                let factory = MacroExprFactory::new(ffi_factory);
                let expr = ffi_expr.into_iter()
                    .map(|ffi_expr| Expr::from(&**ffi_expr))
                    .collect::<Vec<_>>();
                if let Some(result) = expander(&factory, expr) {
                    result.into()
                } else {
                    cxx::UniquePtr::null()
                }
            }
        );
        let ffi_macro = FfiMacro::new_global(name.as_ref().into(), argument_count, ffi_expander)?;
        Ok(Macro(ffi_macro))
    }

    /// Creates a new global macro that accepts a variable number of arguments.
    ///
    /// Variable-argument macros can handle any number of arguments, from zero to many.
    /// The expander function receives all arguments and decides how to handle them.
    ///
    /// # Parameters
    ///
    /// - `name`: The function name that triggers this macro (as a string reference)
    /// - `expander`: The expansion function that transforms the expression
    ///
    /// # Returns
    ///
    /// - `Ok(Macro)`: Successfully created macro
    /// - `Err(Error)`: Failed to create macro (e.g., invalid name or FFI error)
    ///
    /// # Expander Function
    ///
    /// The expander receives:
    /// - `factory`: A mutable reference to [`MacroExprFactory`] for creating new expressions
    /// - `args`: A vector of argument expressions (can be empty)
    ///
    /// The expander should return:
    /// - `Some(Expr)`: The expanded expression to replace the original call
    /// - `None`: Keep the original expression unchanged
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{Macro, MacroExprFactory, Expr};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Macro: max(args...) finds maximum of all arguments
    /// let max_macro = Macro::new_global_var_arg("max", |factory, args| {
    ///     if args.is_empty() {
    ///         return None;
    ///     }
    ///     // Implementation would build comparison chain
    ///     Some(args.into_iter().reduce(|acc, arg| {
    ///         factory.new_call("_>_?_:_", &[acc, arg.clone(), acc.clone(), arg])
    ///     })?)
    /// })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_global_var_arg(
        name: impl AsRef<str>,
        expander: impl GlobalMacroExpander + 'static,
    ) -> Result<Self, Error> {
        let ffi_expander: cxx::UniquePtr<FfiGlobalMacroExpander> = FfiGlobalMacroExpander::new(
            move |ffi_factory, ffi_expr| -> cxx::UniquePtr<FfiExpr> {
                let factory = MacroExprFactory::new(ffi_factory);
                let expr = ffi_expr.into_iter()
                    .map(|ffi_expr| Expr::from(&**ffi_expr))
                    .collect::<Vec<_>>();
                if let Some(result) = expander(&factory, expr) {
                    result.into()
                } else {
                    cxx::UniquePtr::null()
                }
            }
        );
        let ffi_macro = FfiMacro::new_global_var_arg(name.as_ref().into(), ffi_expander)?;
        Ok(Macro(ffi_macro))
    }

    /// Creates a new receiver macro with a fixed number of arguments.
    ///
    /// Receiver macros are invoked as method calls on a target expression,
    /// e.g., `target.macro_name(arg1, arg2)`. The macro will only be expanded
    /// when called with exactly `argument_count` arguments.
    ///
    /// # Parameters
    ///
    /// - `name`: The method name that triggers this macro (as a string reference)
    /// - `argument_count`: The exact number of arguments required (not including receiver)
    /// - `expander`: The expansion function that receives the target and arguments
    ///
    /// # Returns
    ///
    /// - `Ok(Macro)`: Successfully created macro
    /// - `Err(Error)`: Failed to create macro (e.g., invalid name or FFI error)
    ///
    /// # Expander Function
    ///
    /// The expander receives:
    /// - `factory`: A mutable reference to [`MacroExprFactory`] for creating new expressions
    /// - `target`: The receiver expression (the object before the dot)
    /// - `args`: A vector of argument expressions
    ///
    /// The expander should return:
    /// - `Some(Expr)`: The expanded expression to replace the original call
    /// - `None`: Keep the original expression unchanged
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{Macro, MacroExprFactory, Expr};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Macro: list.is_empty() expands to size(list) == 0
    /// let is_empty_macro = Macro::new_receiver("is_empty", 0, |factory, target, _args| {
    ///     let size_call = factory.new_call("size", &[target]);
    ///     Some(factory.new_call("_==_", &[size_call, factory.new_const(0)]))
    /// })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_receiver(
        name: impl AsRef<str>,
        argument_count: usize,
        expander: impl ReceiverMacroExpander + 'static,
    ) -> Result<Self, Error> {
        let ffi_expander: cxx::UniquePtr<FfiReceiverMacroExpander> = FfiReceiverMacroExpander::new(
            move |ffi_factory, ffi_target, ffi_expr| -> cxx::UniquePtr<FfiExpr> {
                let factory = MacroExprFactory::new(ffi_factory);
                let target = Expr::from(&*ffi_target);
                let expr = ffi_expr.into_iter()
                    .map(|ffi_expr| Expr::from(&**ffi_expr))
                    .collect::<Vec<_>>();
                if let Some(result) = expander(&factory, target, expr) {
                    result.into()
                } else {
                    cxx::UniquePtr::null()
                }
            }
        );
        let ffi_macro = FfiMacro::new_receiver(name.as_ref().into(), argument_count, ffi_expander)?;
        Ok(Macro(ffi_macro))
    }

    /// Creates a new receiver macro that accepts a variable number of arguments.
    ///
    /// Variable-argument receiver macros can handle any number of arguments beyond
    /// the target expression. The expander receives the target and all arguments.
    ///
    /// # Parameters
    ///
    /// - `name`: The method name that triggers this macro (as a string reference)
    /// - `expander`: The expansion function that receives the target and arguments
    ///
    /// # Returns
    ///
    /// - `Ok(Macro)`: Successfully created macro
    /// - `Err(Error)`: Failed to create macro (e.g., invalid name or FFI error)
    ///
    /// # Expander Function
    ///
    /// The expander receives:
    /// - `factory`: A mutable reference to [`MacroExprFactory`] for creating new expressions
    /// - `target`: The receiver expression (the object before the dot)
    /// - `args`: A vector of argument expressions (can be empty)
    ///
    /// The expander should return:
    /// - `Some(Expr)`: The expanded expression to replace the original call
    /// - `None`: Keep the original expression unchanged
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::{Macro, MacroExprFactory, Expr};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Macro: str.concat(parts...) concatenates multiple strings
    /// let concat_macro = Macro::new_receiver_var_arg("concat", |factory, target, args| {
    ///     let mut result = target;
    ///     for arg in args {
    ///         result = factory.new_call("_+_", &[result, arg]);
    ///     }
    ///     Some(result)
    /// })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_receiver_var_arg(
        name: impl AsRef<str>,
        expander: impl ReceiverMacroExpander + 'static,
    ) -> Result<Self, Error> {
        let ffi_expander: cxx::UniquePtr<FfiReceiverMacroExpander> = FfiReceiverMacroExpander::new(
            move |ffi_factory, ffi_target, ffi_expr| -> cxx::UniquePtr<FfiExpr> {
                let factory = MacroExprFactory::new(ffi_factory);
                let target = Expr::from(&*ffi_target);
                let expr = ffi_expr.into_iter()
                    .map(|ffi_expr| Expr::from(&**ffi_expr))
                    .collect::<Vec<_>>();
                if let Some(result) = expander(&factory, target, expr) {
                    result.into()
                } else {
                    cxx::UniquePtr::null()
                }
            }
        );
        let ffi_macro = FfiMacro::new_receiver_var_arg(name.as_ref().into(), ffi_expander)?;
        Ok(Macro(ffi_macro))
    }

    /// Returns the name of this macro as a byte slice.
    ///
    /// This is the function or method name that triggers macro expansion.
    /// The returned byte slice is UTF-8 encoded.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::Macro;
    /// # fn example(macro_def: &Macro) {
    /// let name = macro_def.name();
    /// let name_str = std::str::from_utf8(name).unwrap();
    /// println!("Macro name: {}", name_str);
    /// # }
    /// ```
    pub fn name(&self) -> &[u8] {
        self.0.function().as_bytes()
    }

    /// Returns the expected number of arguments for this macro.
    ///
    /// Returns `Some(n)` for fixed-argument macros requiring exactly `n` arguments,
    /// or `None` for variable-argument macros that accept any number of arguments.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cel_cxx::macros::Macro;
    /// # fn example(macro_def: &Macro) {
    /// match macro_def.argument_count() {
    ///     Some(n) => println!("Fixed macro with {} arguments", n),
    ///     None => println!("Variable-argument macro"),
    /// }
    /// # }
    /// ```
    pub fn argument_count(&self) -> Option<usize> {
        if self.0.is_variadic() {
            None
        } else {
            Some(self.0.argument_count())
        }
    }

    /// Returns whether this is a receiver-style macro.
    ///
    /// Receiver-style macros are invoked as method calls (e.g., `target.method(args)`),
    /// while non-receiver macros are invoked as function calls (e.g., `function(args)`).
    ///
    /// # Returns
    ///
    /// - `true`: This is a receiver macro (created with [`new_receiver`] or [`new_receiver_var_arg`])
    /// - `false`: This is a global macro (created with [`new_global`] or [`new_global_var_arg`])
    ///
    /// [`new_receiver`]: Self::new_receiver
    /// [`new_receiver_var_arg`]: Self::new_receiver_var_arg
    /// [`new_global`]: Self::new_global
    /// [`new_global_var_arg`]: Self::new_global_var_arg
    pub fn is_receiver_style(&self) -> bool {
        self.0.is_receiver_style()
    }

    /// Returns the unique key identifying this macro.
    ///
    /// The key is an internal identifier used by the CEL parser to match macro
    /// invocations. It encodes the macro name, receiver style, and argument count.
    ///
    /// # Returns
    ///
    /// A byte slice representing the macro's unique key (UTF-8 encoded).
    ///
    /// # Note
    ///
    /// This method is primarily for internal use and debugging. Most users should
    /// use [`name()`], [`is_receiver_style()`], and [`argument_count()`] instead.
    ///
    /// [`name()`]: Self::name
    /// [`is_receiver_style()`]: Self::is_receiver_style
    /// [`argument_count()`]: Self::argument_count
    pub fn key(&self) -> &[u8] {
        self.0.key().as_bytes()
    }
}

impl From<Macro> for cxx::UniquePtr<crate::ffi::Macro> {
    fn from(value: Macro) -> Self {
        value.0
    }
}

impl From<cxx::UniquePtr<crate::ffi::Macro>> for Macro {
    fn from(value: cxx::UniquePtr<crate::ffi::Macro>) -> Self {
        Macro(value)
    }
}

impl std::fmt::Debug for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Macro({})", String::from_utf8_lossy(self.key()))
    }
}

impl std::hash::Hash for Macro {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key().hash(state);
    }
}

impl std::cmp::PartialEq for Macro {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl std::cmp::Eq for Macro {}
