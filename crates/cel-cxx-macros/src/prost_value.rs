use crate::symbol::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_derive_prost_value(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ast = Ast::from_ast(&input)?;
    let tokens = [
        impl_typed_value(&ast),
        impl_into_value(&ast),
        impl_from_value(&ast),
    ]
    .into_iter()
    .collect();
    Ok(tokens)
}

struct Ast {
    ident: syn::Ident,
    cel_path: syn::Path,
    type_name_override: Option<syn::LitStr>,
}

impl Ast {
    fn from_ast(input: &syn::DeriveInput) -> syn::Result<Self> {
        if let Some(param) = input.generics.params.first() {
            return Err(syn::Error::new_spanned(
                param,
                "ProstValue cannot be derived for generic types",
            ));
        }

        let mut cel_path = None;
        let mut type_name_override = None;

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
                    type_name_override = Some(lit);
                } else {
                    return Err(syn::Error::new_spanned(meta.path, "unknown attribute"));
                }
                Ok(())
            })?;
        }

        let cel_path =
            cel_path.unwrap_or_else(|| syn::parse_str(&format!("::{CEL_CXX}")).unwrap());

        Ok(Self {
            ident: input.ident.clone(),
            cel_path,
            type_name_override,
        })
    }

    fn type_name_expr(&self) -> TokenStream {
        if let Some(ref lit) = self.type_name_override {
            quote! { #lit.to_string() }
        } else {
            quote! { <Self as ::prost::Name>::full_name() }
        }
    }
}

fn impl_typed_value(ast: &Ast) -> TokenStream {
    let name = &ast.ident;
    let cel = &ast.cel_path;
    let type_name_expr = ast.type_name_expr();

    quote! {
        impl #cel::values::TypedValue for #name {
            fn value_type() -> #cel::types::ValueType {
                #cel::types::ValueType::Struct(#cel::types::StructType::new(#type_name_expr))
            }
        }
    }
}

fn impl_into_value(ast: &Ast) -> TokenStream {
    let name = &ast.ident;
    let cel = &ast.cel_path;
    let type_name_expr = ast.type_name_expr();

    quote! {
        impl #cel::values::IntoValue for #name {
            fn into_value(self) -> #cel::values::Value {
                use ::prost::Message as _;
                let type_name = #type_name_expr;
                let bytes = self.encode_to_vec();
                #cel::values::Value::Struct(
                    #cel::values::StructValue::from_bytes(type_name, bytes),
                )
            }
        }

        impl ::std::convert::From<#name> for #cel::values::Value {
            fn from(value: #name) -> Self {
                <#name as #cel::values::IntoValue>::into_value(value)
            }
        }
    }
}

fn impl_from_value(ast: &Ast) -> TokenStream {
    let name = &ast.ident;
    let cel = &ast.cel_path;

    quote! {
        impl #cel::values::FromValue for #name {
            type Output<'a> = #name;

            fn from_value<'a>(
                value: &'a #cel::values::Value,
            ) -> ::std::result::Result<Self::Output<'a>, #cel::values::FromValueError> {
                match value {
                    #cel::values::Value::Struct(sv) => {
                        use ::prost::Message as _;
                        <#name>::decode(sv.to_bytes()).map_err(|_| {
                            #cel::values::FromValueError::new_typed::<#name>(value.clone())
                        })
                    }
                    _ => Err(#cel::values::FromValueError::new_typed::<#name>(value.clone())),
                }
            }
        }

        impl ::std::convert::TryFrom<#cel::values::Value> for #name {
            type Error = #cel::values::FromValueError;
            fn try_from(value: #cel::values::Value) -> ::std::result::Result<Self, Self::Error> {
                <Self as #cel::values::FromValue>::from_value(&value)
            }
        }

        impl ::std::convert::TryFrom<&#cel::values::Value> for #name {
            type Error = #cel::values::FromValueError;
            fn try_from(value: &#cel::values::Value) -> ::std::result::Result<Self, Self::Error> {
                <Self as #cel::values::FromValue>::from_value(value)
            }
        }
    }
}
