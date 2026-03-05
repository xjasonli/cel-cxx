use crate::symbol::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_derive_protobuf_value(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ast = Ast::from_ast(&input)?;
    Ok(impl_stub(&ast))
}

struct Ast {
    ident: syn::Ident,
    cel_path: syn::Path,
    type_name: syn::LitStr,
}

impl Ast {
    fn from_ast(input: &syn::DeriveInput) -> syn::Result<Self> {
        if let Some(param) = input.generics.params.first() {
            return Err(syn::Error::new_spanned(
                param,
                "ProtobufValue cannot be derived for generic types",
            ));
        }

        let mut cel_path = None;
        let mut type_name = None;

        for attr in &input.attrs {
            if !attr.path().is_ident(CEL_CXX) {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(CRATE) {
                    let path: syn::Path = meta.value()?.parse()?;
                    cel_path = Some(path);
                } else if meta.path.is_ident(TYPE_NAME) {
                    let lit: syn::LitStr = meta.value()?.parse()?;
                    type_name = Some(lit);
                } else {
                    return Err(syn::Error::new_spanned(meta.path, "unknown attribute"));
                }
                Ok(())
            })?;
        }

        let type_name = type_name.ok_or_else(|| {
            syn::Error::new_spanned(
                &input.ident,
                "ProtobufValue requires #[cel_cxx(type_name = \"...\")] attribute",
            )
        })?;

        let cel_path =
            cel_path.unwrap_or_else(|| syn::parse_str(&format!("::{CEL_CXX}")).unwrap());

        Ok(Self {
            ident: input.ident.clone(),
            cel_path,
            type_name,
        })
    }
}

fn impl_stub(ast: &Ast) -> TokenStream {
    let name = &ast.ident;
    let cel = &ast.cel_path;
    let type_name = &ast.type_name;

    quote! {
        impl #cel::values::TypedValue for #name {
            fn value_type() -> #cel::types::ValueType {
                #cel::types::ValueType::Struct(#cel::types::StructType::new(#type_name))
            }
        }

        impl #cel::values::IntoValue for #name {
            fn into_value(self) -> #cel::values::Value {
                unimplemented!("protobuf v4 cpp kernel support is not yet available")
            }
        }

        impl ::std::convert::From<#name> for #cel::values::Value {
            fn from(_value: #name) -> Self {
                unimplemented!("protobuf v4 cpp kernel support is not yet available")
            }
        }

        impl #cel::values::FromValue for #name {
            type Output<'a> = #name;

            fn from_value<'a>(
                _value: &'a #cel::values::Value,
            ) -> ::std::result::Result<Self::Output<'a>, #cel::values::FromValueError> {
                unimplemented!("protobuf v4 cpp kernel support is not yet available")
            }
        }

        impl ::std::convert::TryFrom<#cel::values::Value> for #name {
            type Error = #cel::values::FromValueError;
            fn try_from(_value: #cel::values::Value) -> ::std::result::Result<Self, Self::Error> {
                unimplemented!("protobuf v4 cpp kernel support is not yet available")
            }
        }

        impl ::std::convert::TryFrom<&#cel::values::Value> for #name {
            type Error = #cel::values::FromValueError;
            fn try_from(_value: &#cel::values::Value) -> ::std::result::Result<Self, Self::Error> {
                unimplemented!("protobuf v4 cpp kernel support is not yet available")
            }
        }
    }
}
