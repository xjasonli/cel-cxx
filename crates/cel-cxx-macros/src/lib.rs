extern crate proc_macro;
use proc_macro::TokenStream;

mod opaque;
mod symbol;

#[cfg(feature = "prost")]
mod prost_value;

#[cfg(feature = "protobuf-legacy")]
mod protobuf_legacy_value;

#[cfg(feature = "protobuf")]
mod protobuf_value;

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

/// Derive macro for prost-generated protobuf types.
///
/// Generates `TypedValue`, `IntoValue`, `FromValue`, `From<T>` for `Value`,
/// and `TryFrom<Value>` implementations using prost serialization.
///
/// # Requirements
///
/// The type must implement `prost::Message` and `prost::Name`.
/// Enable `prost::Name` generation via `.enable_type_names()` in prost-build.
///
/// # Attributes
/// - `type_name = "..."`: Override the protobuf type name (default: `prost::Name::full_name()`).
/// - `crate = ...`: Custom path to the cel_cxx crate.
///
/// # Setup
///
/// In your `build.rs`, inject the derive via `type_attribute()`:
///
/// ```rust,ignore
/// // build.rs
/// prost_build::Config::new()
///     .enable_type_names()
///     .type_attribute("my.package.MyMessage", "#[derive(::cel_cxx::ProstValue)]")
///     .compile_protos(&["proto/my.proto"], &["proto/"])
///     .unwrap();
/// ```
#[cfg(feature = "prost")]
#[proc_macro_derive(ProstValue, attributes(cel_cxx))]
pub fn derive_prost_value(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    prost_value::expand_derive_prost_value(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derive macro for protobuf v3 (stepancheg) generated types.
///
/// Generates `TypedValue`, `IntoValue`, `FromValue`, `From<T>` for `Value`,
/// and `TryFrom<Value>` implementations using protobuf v3 serialization.
///
/// # Requirements
///
/// The type must implement `protobuf::MessageFull`.
///
/// # Attributes
/// - `type_name = "..."`: Override the protobuf type name (default: from descriptor).
/// - `crate = ...`: Custom path to the cel_cxx crate.
///
/// # Setup
///
/// In your `build.rs`, inject the derive via `customize_callback()`:
///
/// ```rust,ignore
/// // build.rs
/// use protobuf::reflect::MessageDescriptor;
/// use protobuf_codegen::{Codegen, Customize, CustomizeCallback};
///
/// struct DeriveProtobufLegacyValue;
///
/// impl CustomizeCallback for DeriveProtobufLegacyValue {
///     fn message(&self, _message: &MessageDescriptor) -> Customize {
///         Customize::default().before("#[derive(::cel_cxx::ProtobufLegacyValue)]")
///     }
/// }
///
/// fn main() {
///     Codegen::new()
///         .cargo_out_dir("protos")
///         .input("proto/my.proto")
///         .include("proto/")
///         .customize_callback(DeriveProtobufLegacyValue)
///         .run_from_script();
/// }
/// ```
#[cfg(feature = "protobuf-legacy")]
#[proc_macro_derive(ProtobufLegacyValue, attributes(cel_cxx))]
pub fn derive_protobuf_legacy_value(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    protobuf_legacy_value::expand_derive_protobuf_legacy_value(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derive macro for protobuf v4 (cpp kernel) types — **stub, not yet implemented**.
///
/// All generated methods will panic with `unimplemented!()`.
///
/// # Attributes
/// - `type_name = "..."`: **Required.** The fully-qualified protobuf type name.
/// - `crate = ...`: Custom path to the cel_cxx crate.
#[cfg(feature = "protobuf")]
#[proc_macro_derive(ProtobufValue, attributes(cel_cxx))]
pub fn derive_protobuf_value(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    protobuf_value::expand_derive_protobuf_value(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
