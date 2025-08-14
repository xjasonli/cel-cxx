extern crate proc_macro;
use proc_macro::TokenStream;

mod opaque;
mod symbol;

/// Derive macro for creating opaque types for cxx-cel.
///
/// # Attributes
/// - `type = "..."`: The type name of the opaque type.
/// - `display`: Generates `impl std::fmt::Display` with `Debug` trait.
/// - `display = EXPR`: Generates `impl std::fmt::Display` with the given expression.
/// - `crate = ...`: The crate name of the cel_cxx crate, if you are using a different crate name in Cargo.toml.
///
/// # Example
///
/// ```ignore
/// #[derive(Opaque, Debug, Clone, PartialEq)]
/// #[cel_cxx(type = "my_app.my_module.User")]
/// // Generates `std::fmt::Display` impl with `Debug` trait.
/// #[cel_cxx(display)]
/// // or you can specify a custom format.
/// // Generates `std::fmt::Display` impl with custom format.
/// #[cel_cxx(display = write!(fmt, "User(id={id}, name={name})", id = self.id, name = self.name))]
/// struct User {
///     id: i32,
///     name: String,
///     email: String,
/// }
/// ```
/// 
///
#[proc_macro_derive(Opaque, attributes(cel_cxx))]
pub fn derive_opaque(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    opaque::expand_derive_opaque(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
