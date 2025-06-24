//! Function declaration trait for CEL expression evaluation.
//!
//! This module provides the [`FunctionDecl`] trait for extracting compile-time type information
//! from function signatures. The trait is used for function registration and type checking
//! in CEL environments.
//!
//! # Features
//!
//! - **Compile-time type extraction**: Extract argument and return types from function signatures
//! - **Sealed trait**: Cannot be implemented outside this crate for type safety
//! - **Zero-annotation support**: Works with standard Rust function types without annotations
//! - **Generic function support**: Supports functions with 0 to 10 parameters
//! - **Type safety**: Ensures only valid CEL-compatible function types can be declared
//!
//! # Examples
//!
//! ## Basic function declaration
//!
//! ```rust
//! use cel_cxx::function::FunctionDecl;
//! use cel_cxx::types::ValueType;
//!
//! // Extract type information from function signatures
//! type AddFn = fn(i64, i64) -> i64;
//! let arg_types = AddFn::arguments();
//! let return_type = AddFn::result();
//!
//! assert_eq!(arg_types, vec![ValueType::Int, ValueType::Int]);
//! assert_eq!(return_type, ValueType::Int);
//! ```
//!
//! ## Complex function types
//!
//! ```rust
//! # use cel_cxx::function::FunctionDecl;
//! # use cel_cxx::types::ValueType;
//! // String processing function
//! type FormatFn = fn(String, i64) -> String;
//! let arg_types = FormatFn::arguments();
//! let return_type = FormatFn::result();
//!
//! assert_eq!(arg_types, vec![ValueType::String, ValueType::Int]);
//! assert_eq!(return_type, ValueType::String);
//! ```
//!
//! ## Zero-parameter functions
//!
//! ```rust
//! # use cel_cxx::function::FunctionDecl;
//! # use cel_cxx::types::ValueType;
//! // Constant function
//! type PiFn = fn() -> f64;
//! let arg_types = PiFn::arguments();
//! let return_type = PiFn::result();
//!
//! assert_eq!(arg_types, vec![]);
//! assert_eq!(return_type, ValueType::Double);
//! ```

use super::count_args;
use crate::types::*;
use crate::values::*;

// =============================================================================
// Public API
// =============================================================================

/// Trait for extracting compile-time type information from function signatures.
///
/// This trait provides static methods to extract argument and return type information
/// from function pointer types. It is used purely for type declarations and does not
/// require the ability to instantiate values, only to determine their types.
///
/// # Note
///
/// This trait is sealed and cannot be implemented outside this crate. It is
/// automatically implemented for function pointer types with compatible signatures.
///
/// # Automatically Implemented For
///
/// The trait is implemented for function pointer types:
/// - `fn() -> R`
/// - `fn(A1) -> R`
/// - `fn(A1, A2) -> R`
/// - `fn(A1, A2, A3) -> R`
/// - ... (up to 10 arguments)
///
/// Where:
/// - `R: TypedValue` (return type must have a known CEL type)
/// - `A1, A2, ...: TypedValue` (argument types must have known CEL types)
///
/// # Examples
///
/// ```rust
/// # use cel_cxx::function::FunctionDecl;
/// # use cel_cxx::types::ValueType;
/// // Simple arithmetic function
/// type MathFn = fn(i64, i64) -> i64;
/// assert_eq!(MathFn::arguments(), vec![ValueType::Int, ValueType::Int]);
/// assert_eq!(MathFn::result(), ValueType::Int);
///
/// // String manipulation function
/// type StringFn = fn(String, i64) -> String;
/// assert_eq!(StringFn::arguments(), vec![ValueType::String, ValueType::Int]);
/// assert_eq!(StringFn::result(), ValueType::String);
/// ```
pub trait FunctionDecl: private::Sealed {
    /// The number of arguments that this function signature accepts.
    ///
    /// This constant provides compile-time access to the arity (number of parameters)
    /// of the function signature. It is automatically computed by the implementation
    /// and matches the length of the vector returned by [`arguments()`](Self::arguments).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cel_cxx::function::FunctionDecl;
    /// type NoArgsFn = fn() -> i64;
    /// type TwoArgsFn = fn(i64, String) -> bool;
    /// type ThreeArgsFn = fn(i64, String, bool) -> f64;
    ///
    /// assert_eq!(NoArgsFn::ARGUMENTS_LEN, 0);
    /// assert_eq!(TwoArgsFn::ARGUMENTS_LEN, 2);
    /// assert_eq!(ThreeArgsFn::ARGUMENTS_LEN, 3);
    /// ```
    const ARGUMENTS_LEN: usize;

    /// Get the argument types of the function.
    ///
    /// Returns a vector of [`ValueType`] representing the expected argument types
    /// in the order they should be provided to the function.
    ///
    /// # Returns
    ///
    /// A vector of [`ValueType`] values representing each parameter type.
    fn arguments() -> Vec<ValueType>;

    /// Get the return type of the function.
    ///
    /// Returns the [`ValueType`] that this function signature returns.
    ///
    /// # Returns
    ///
    /// The [`ValueType`] representing the function's return type.
    fn result() -> ValueType;

    /// Get the function type of the function.
    ///
    /// Returns the [`FunctionType`] that this function signature represents.
    ///
    /// # Returns
    ///
    /// The [`FunctionType`] is a tuple of the return type and the argument types.
    fn function_type() -> FunctionType {
        FunctionType::new(Self::result(), Self::arguments())
    }

    /// Get the number of arguments of the function.
    ///
    /// Returns the number of arguments that the function takes.
    ///
    /// # Returns
    ///
    /// The number of arguments that the function takes.
    fn arguments_len() -> usize {
        Self::ARGUMENTS_LEN
    }
}

// =============================================================================
// Implementation details
// =============================================================================

/// Macro to generate [`FunctionDecl`] implementations for function pointer types
/// with different arities (0 to 10 parameters).
macro_rules! impl_function_decl {
    ($($ty:ident),*) => {
        impl<R, $($ty,)*> FunctionDecl for fn($($ty,)*) -> R
        where
            R: IntoResult,
            $($ty: TypedValue,)*
        {
            fn arguments() -> Vec<ValueType> {
                vec![$($ty::value_type()),*]
            }

            fn result() -> ValueType {
                R::value_type()
            }

            const ARGUMENTS_LEN: usize = count_args!($($ty),*);
        }

        // Sealed implementation for function pointer types
        impl<R, $($ty,)*> private::Sealed for fn($($ty,)*) -> R
        where
            R: IntoResult,
            $($ty: TypedValue,)*
        {}
    };
}

// Generate implementations for functions with 0-10 parameters
impl_function_decl!();
impl_function_decl!(A1);
impl_function_decl!(A1, A2);
impl_function_decl!(A1, A2, A3);
impl_function_decl!(A1, A2, A3, A4);
impl_function_decl!(A1, A2, A3, A4, A5);
impl_function_decl!(A1, A2, A3, A4, A5, A6);
impl_function_decl!(A1, A2, A3, A4, A5, A6, A7);
impl_function_decl!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_function_decl!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_function_decl!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

// =============================================================================
// Private module for sealed traits
// =============================================================================

/// Private module for sealed traits to prevent external implementations.
mod private {
    /// Sealed trait to prevent external implementations of [`FunctionDecl`].
    pub trait Sealed {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_parameter_function() {
        // Test function with no parameters
        type ConstantFn = fn() -> i64;

        assert_eq!(ConstantFn::arguments(), vec![]);
        assert_eq!(ConstantFn::result(), ValueType::Int);
    }

    #[test]
    fn test_single_parameter_function() {
        // Test function with one parameter
        type SquareFn = fn(i64) -> i64;

        assert_eq!(SquareFn::arguments(), vec![ValueType::Int]);
        assert_eq!(SquareFn::result(), ValueType::Int);
    }

    #[test]
    fn test_multiple_parameter_function() {
        // Test function with multiple parameters
        type AddFn = fn(i64, i64, i64) -> i64;

        assert_eq!(
            AddFn::arguments(),
            vec![ValueType::Int, ValueType::Int, ValueType::Int]
        );
        assert_eq!(AddFn::result(), ValueType::Int);
    }

    #[test]
    fn test_mixed_type_function() {
        // Test function with mixed argument types
        type FormatFn = fn(String, i64, bool) -> String;

        assert_eq!(
            FormatFn::arguments(),
            vec![ValueType::String, ValueType::Int, ValueType::Bool]
        );
        assert_eq!(FormatFn::result(), ValueType::String);
    }

    #[test]
    fn test_string_function() {
        // Test function with string types
        type ConcatFn = fn(String, String) -> String;

        assert_eq!(
            ConcatFn::arguments(),
            vec![ValueType::String, ValueType::String]
        );
        assert_eq!(ConcatFn::result(), ValueType::String);
    }

    #[test]
    fn test_floating_point_function() {
        // Test function with floating point types
        type MathFn = fn(f64, f64) -> f64;

        assert_eq!(
            MathFn::arguments(),
            vec![ValueType::Double, ValueType::Double]
        );
        assert_eq!(MathFn::result(), ValueType::Double);
    }

    #[test]
    fn test_boolean_function() {
        // Test function with boolean types
        type LogicFn = fn(bool, bool) -> bool;

        assert_eq!(LogicFn::arguments(), vec![ValueType::Bool, ValueType::Bool]);
        assert_eq!(LogicFn::result(), ValueType::Bool);
    }

    #[test]
    fn test_maximum_parameters() {
        // Test function with maximum supported parameters (10)
        type MaxParamFn = fn(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) -> i64;

        let expected_args = vec![ValueType::Int; 10];
        assert_eq!(MaxParamFn::arguments(), expected_args);
        assert_eq!(MaxParamFn::result(), ValueType::Int);
    }
}
