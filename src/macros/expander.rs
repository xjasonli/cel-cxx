use super::{MacroExprFactory, Expr};

/// Trait for global macro expansion functions.
///
/// A global macro expander receives a list of arguments and produces a transformed
/// expression. Global macros are triggered by function calls, not method calls.
///
/// # Type Requirements
///
/// The expander function must be:
/// - `Fn(&mut MacroExprFactory, Vec<Expr>) -> Option<Expr>`: The expansion signature
/// - `Send + Sync`: Thread-safe for use in concurrent environments
/// - `'static`: No borrowed references in the closure
///
/// # Return Value
///
/// - `Some(expr)`: The macro expansion succeeded and `expr` replaces the original call
/// - `None`: The macro cannot be expanded (keeps the original expression)
///
/// # Examples
///
/// ```rust,no_run
/// # use cel_cxx::macros::{GlobalMacroExpander, MacroExprFactory, Expr};
/// // Simple constant-folding macro
/// fn optimize_add(factory: &mut MacroExprFactory, args: Vec<Expr>) -> Option<Expr> {
///     if args.len() != 2 {
///         return None;
///     }
///     
///     // If both args are constant integers, fold them
///     let left = args[0].kind()?.as_constant()?.as_int()?;
///     let right = args[1].kind()?.as_constant()?.as_int()?;
///     
///     Some(factory.new_const(left + right))
/// }
/// ```
pub trait GlobalMacroExpander
    : Fn(&MacroExprFactory<'_>, Vec<Expr>) -> Option<Expr> + Send + Sync
{}

impl<'f, F> GlobalMacroExpander for F where F
    : Fn(&MacroExprFactory<'_>, Vec<Expr>) -> Option<Expr> + Send + Sync
    + 'f
{}

/// Trait for receiver macro expansion functions.
///
/// A receiver macro expander receives a target expression and a list of arguments,
/// then produces a transformed expression. Receiver macros are triggered by method
/// calls on a target object.
///
/// # Type Requirements
///
/// The expander function must be:
/// - `Fn(&mut MacroExprFactory, Expr, Vec<Expr>) -> Option<Expr>`: The expansion signature
/// - `Send + Sync`: Thread-safe for use in concurrent environments
/// - `'static`: No borrowed references in the closure
///
/// # Parameters
///
/// - `factory`: Factory for creating new expression nodes
/// - `target`: The receiver expression (the object before the dot)
/// - `args`: The argument list passed to the method
///
/// # Return Value
///
/// - `Some(expr)`: The macro expansion succeeded and `expr` replaces the original call
/// - `None`: The macro cannot be expanded (keeps the original expression)
///
/// # Examples
///
/// ```rust,no_run
/// # use cel_cxx::macros::{ReceiverMacroExpander, MacroExprFactory, Expr};
/// // Macro for optional chaining: target.get_or(default)
/// fn get_or_macro(
///     factory: &mut MacroExprFactory,
///     target: Expr,
///     mut args: Vec<Expr>
/// ) -> Option<Expr> {
///     if args.len() != 1 {
///         return None;
///     }
///     
///     let default_value = args.pop()?;
///     
///     // Expand to: target != null ? target : default_value
///     Some(factory.new_call("_?_:_", &[
///         factory.new_call("_!=_", &[target.clone(), factory.new_const(())]),
///         target,
///         default_value,
///     ]))
/// }
/// ```
pub trait ReceiverMacroExpander
    : Fn(&MacroExprFactory<'_>, Expr, Vec<Expr>) -> Option<Expr> + Send + Sync
{}

impl<'f, F> ReceiverMacroExpander for F where F
    : Fn(&MacroExprFactory<'_>, Expr, Vec<Expr>) -> Option<Expr> + Send + Sync
    + 'f
{}
