use proc_macro2::TokenStream;
use quote::quote;
use crate::symbol::*;

pub fn expand_derive_opaque(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ast = Ast::from_ast(&input)?;
    let tokens = [
        impl_typed_opaque(&ast),
        impl_try_from_value(&ast),
        impl_into_value(&ast),
        impl_cel_trait(&ast),
    ].into_iter().flatten();
    Ok(tokens.collect())
}

fn impl_typed_opaque(input: &Ast) -> TokenStream {
    let name = &input.ident;
    let cel = input.cel_path();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let generic_params = &input.generic_params;
    let ty = input.opaque_type();

    let tokens = quote! {
        impl #impl_generics #cel::values::TypedOpaqueValue for #name #ty_generics #where_clause {
            fn opaque_type() -> #cel::types::OpaqueType {
                #cel::types::OpaqueType::new(
                    #ty,
                    vec![
                        #( <#generic_params as #cel::values::TypedValue>::value_type() ),*
                    ],
                )
            }
        }
    };
    tokens
}

fn impl_try_from_value(input: &Ast) -> TokenStream {
    let name = &input.ident;
    let cel = input.cel_path();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let tokens = quote! {
        impl #impl_generics TryFrom<#cel::values::Value> for #name #ty_generics #where_clause {
            type Error = #cel::values::Value;
            fn try_from(value: #cel::values::Value) -> Result<
                    #name #ty_generics,
                    <#name #ty_generics as TryFrom<#cel::values::Value>>::Error
                >
            {
                match value {
                    #cel::values::Value::Opaque(opaque) => {
                        Ok(
                            opaque.downcast::<#name #ty_generics>()
                                .map_err(#cel::values::Value::Opaque)?
                                .into_inner()
                        )
                    }
                    _ => Err(value),
                }
            }
        }
    };
    tokens
}

fn impl_into_value(input: &Ast) -> TokenStream {
    let name = &input.ident;
    let cel = input.cel_path();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let tokens = quote! {
        impl #impl_generics Into<#cel::values::Value> for #name #ty_generics #where_clause {
            fn into(self) -> #cel::values::Value {
                #cel::values::Value::Opaque(#cel::values::Opaque::new(self))
            }
        }
    };
    tokens
}

fn impl_cel_trait(input: &Ast) -> TokenStream {
    let name = &input.ident;
    let cel = input.cel_path();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let tokens = quote! {
        impl #impl_generics #cel::values::TypedValue for #name #ty_generics #where_clause {
            fn value_type() -> #cel::types::Type { #cel::types::Type::Opaque(<#name #ty_generics as #cel::values::TypedOpaqueValue>::opaque_type()) }
        }
        impl #impl_generics #cel::values::IntoValue for #name #ty_generics #where_clause {
            fn into_value(self) -> #cel::values::Value {
                #cel::values::Value::Opaque(#cel::values::Opaque::new(self))
            }
        }
        impl #impl_generics #cel::values::FromValue for #name #ty_generics #where_clause {
            fn from_value(value: #cel::values::Value) -> Result<Self, #cel::values::Value> {
                match value {
                    #cel::values::Value::Opaque(opaque) => {
                        Ok(opaque.downcast::<#name #ty_generics>()
                            .map_err(#cel::values::Value::Opaque)?
                            .into_inner())
                    }
                    _ => Err(value),
                }
            }
        }
    };
    tokens
}

struct Ast {
    pub ident: syn::Ident,
    pub attrs: Attrs,
    pub generics: syn::Generics,
    pub generic_params: Vec<syn::Ident>,
}

impl Ast {
    fn from_ast(input: &syn::DeriveInput) -> syn::Result<Self> {
        if let Some(const_param) = input.generics.const_params().next() {
            return Err(syn::Error::new_spanned(
                const_param,
                "opaque types cannot have const generic parameters"
            ));
        }

        let attrs = Attrs::from_ast(input)?;

        let mut generic_params = Vec::new();
        let mut generics = input.generics.clone();
        let cel_path = attrs.cel_path();
        let bound: syn::TraitBound = syn::parse2(quote! { #cel_path::values::TypedValue })?;
        for param in generics.params.iter_mut() {
            if let syn::GenericParam::Type(ref mut type_param) = *param {
                type_param.bounds.push(
                    syn::TypeParamBound::Trait(bound.clone())
                );
                generic_params.push(type_param.ident.clone());
            }
        }

        Ok(Self {
            ident: input.ident.clone(),
            attrs,
            generics,
            generic_params,
        })
    }

    fn cel_path(&self) -> syn::Path {
        self.attrs.cel_path()
    }

    fn opaque_type(&self) -> syn::Expr {
        self.attrs.opaque_type(&self.ident.to_string())
    }
}

//
// #[derive(Opaque)]
// #[cel(type = "MyType")]
// #[cel(crate = my_crate)]
// #[cel(type = "MyType", crate = my_crate)]
struct Attrs {
    pub type_name: Option<syn::Expr>,
    pub crate_path: Option<syn::Path>,
}

impl Attrs {
    fn from_ast(input: &syn::DeriveInput) -> syn::Result<Self> {
        let mut type_name = None;
        let mut crate_path = None;

        for attr in input.attrs.iter() {
            if !attr.path().is_ident(CEL_CXX) {
                continue;
            }
            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(TYPE) {
                    let expr: syn::Expr = meta.value()?.parse()?;
                    type_name = Some(expr);
                } else if meta.path.is_ident(CRATE) {
                    let path: syn::Path = meta.value()?.parse()?;
                    crate_path = Some(path);
                } else {
                    return Err(syn::Error::new_spanned(meta.path, "unknown attribute"));
                }
                Ok(())
            })?;
        }
        Ok(Self {
            type_name,
            crate_path,
        })
    }

    fn cel_path(&self) -> syn::Path {
        self.crate_path
            .clone()
            .unwrap_or_else(|| syn::parse_str(format!("::{}", CEL_CXX).as_str()).unwrap())
    }

    fn opaque_type(&self, default: &str) -> syn::Expr {
        self.type_name
            .clone()
            .unwrap_or_else(|| syn::parse_quote!(#default))
    }
}
