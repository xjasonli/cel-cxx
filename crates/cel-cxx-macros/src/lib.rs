extern crate proc_macro;
use proc_macro::TokenStream;

mod opaque;
mod symbol;

/// Derive macro for creating opaque types for cxx-cel.
///
/// # Example
///
/// ```ignore
/// #[derive(Opaque, Debug, Clone, PartialEq)]
/// #[cel_cxx(type = "my_app.my_module.MyType")]
/// struct MyType {
///     value: i32,
/// }
/// ```
///
#[proc_macro_derive(Opaque, attributes(cel_cxx))]
pub fn derive_opaque(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    opaque::expand_derive_opaque(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
