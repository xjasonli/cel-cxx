use crate::symbol::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_derive_opaque(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ast = Ast::from_ast(&input)?;
    let tokens = [impl_typed(&ast), impl_into(&ast), impl_from(&ast)]
        .into_iter()
        .flatten();
    Ok(tokens.collect())
}

fn add_generic_bounds(
    generic_params: &[syn::Ident],
    generics: &syn::Generics,
    bounds: Vec<syn::TypeParamBound>,
) -> syn::Generics {
    let mut generics = generics.clone();
    if bounds.is_empty() {
        return generics;
    }
    let bounds = bounds
        .into_iter()
        .collect::<syn::punctuated::Punctuated<syn::TypeParamBound, syn::Token![+]>>();

    for type_param in generic_params {
        if generics.where_clause.is_none() {
            generics.where_clause = Some(syn::parse_quote!(where));
        }
        generics
            .where_clause
            .as_mut()
            .unwrap()
            .predicates
            .push(syn::parse_quote!(#type_param: #bounds));
    }
    generics
}

fn impl_typed(input: &Ast) -> TokenStream {
    let name = &input.ident;
    let cel = input.cel_path();
    let generics = add_generic_bounds(
        &input.generic_params,
        &input.generics,
        vec![syn::parse_quote!(#cel::values::TypedValue)],
    );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let generic_params = &input.generic_params;
    let ty = input.opaque_type();

    let tokens = quote! {
        impl #impl_generics #cel::values::TypedOpaque for #name #ty_generics #where_clause {
            fn opaque_type() -> #cel::types::OpaqueType {
                #cel::types::OpaqueType::new(
                    #ty,
                    vec![
                        #( <#generic_params as #cel::values::TypedValue>::value_type() ),*
                    ],
                )
            }
        }
        impl #impl_generics #cel::values::TypedValue for #name #ty_generics #where_clause {
            fn value_type() -> #cel::types::ValueType {
                #cel::types::ValueType::Opaque(<Self as #cel::values::TypedOpaque>::opaque_type())
            }
        }
    };
    tokens
}

fn impl_into(input: &Ast) -> TokenStream {
    let name = &input.ident;
    let cel = input.cel_path();
    let generics = add_generic_bounds(
        &input.generic_params,
        &input.generics,
        vec![syn::parse_quote!(#cel::values::IntoValue)],
    );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let tokens = quote! {
        impl #impl_generics #cel::values::IntoValue for #name #ty_generics #where_clause {
            fn into_value(self) -> #cel::values::Value {
                #cel::values::Value::Opaque(Box::new(self))
            }
        }
        impl #impl_generics ::std::convert::From<#name #ty_generics> for #cel::values::Value {
            fn from(value: #name #ty_generics) -> Self {
                <#name #ty_generics as #cel::values::IntoValue>::into_value(value)
            }
        }
    };
    tokens
}

fn add_generic_lifetime(generics: &syn::Generics, lifetime: syn::Lifetime) -> syn::Generics {
    let mut generics = generics.clone();
    generics.params.push(syn::parse_quote!(#lifetime));
    generics
}

fn impl_from(input: &Ast) -> TokenStream {
    let name = &input.ident;
    let cel = input.cel_path();
    let generics = add_generic_bounds(
        &input.generic_params,
        &input.generics,
        vec![
            syn::parse_quote!(#cel::values::FromValue),
            syn::parse_quote!(#cel::values::TypedValue),
        ],
    );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let generics2 = add_generic_lifetime(&generics, syn::parse_quote!('opaque));
    let (impl_generics2, _, _) = generics2.split_for_impl();

    let tokens = quote! {
        impl #impl_generics #cel::values::FromValue for #name #ty_generics #where_clause {
            type Output<'a> = #name #ty_generics;

            fn from_value<'a>(value: &'a #cel::values::Value) -> ::std::result::Result<Self::Output<'a>, #cel::values::FromValueError> {
                match value {
                    #cel::values::Value::Opaque(v) => {
                        v.downcast_ref::<#name #ty_generics>()
                            .ok_or(#cel::values::FromValueError::new_typed::<#name #ty_generics>(value.clone()))
                            .map(|v| v.clone())
                    }
                    _ => Err(#cel::values::FromValueError::new_typed::<#name #ty_generics>(value.clone())),
                }
            }
        }

        impl #impl_generics ::std::convert::TryFrom<#cel::values::Value> for #name #ty_generics #where_clause {
            type Error = #cel::values::FromValueError;
            fn try_from(value: #cel::values::Value) -> ::std::result::Result<Self, Self::Error> {
                <Self as #cel::values::FromValue>::from_value(&value)
            }
        }
        impl #impl_generics ::std::convert::TryFrom<&#cel::values::Value> for #name #ty_generics #where_clause {
            type Error = #cel::values::FromValueError;
            fn try_from(value: &#cel::values::Value) -> ::std::result::Result<Self, Self::Error> {
                <Self as #cel::values::FromValue>::from_value(value)
            }
        }

        impl #impl_generics #cel::values::FromValue for &#name #ty_generics #where_clause {
            type Output<'a> = &'a #name #ty_generics;

            fn from_value<'a>(value: &'a #cel::values::Value) -> ::std::result::Result<Self::Output<'a>, #cel::values::FromValueError> {
                match value {
                    #cel::values::Value::Opaque(v) => {
                        v.downcast_ref::<#name #ty_generics>()
                            .ok_or(#cel::values::FromValueError::new_typed::<#name #ty_generics>(value.clone()))
                    }
                    _ => Err(#cel::values::FromValueError::new_typed::<#name #ty_generics>(value.clone())),
                }
            }
        }
        impl #impl_generics2 ::std::convert::TryFrom<&'opaque #cel::values::Value> for &'opaque #name #ty_generics #where_clause {
            type Error = #cel::values::FromValueError;
            fn try_from(value: &'opaque #cel::values::Value) -> ::std::result::Result<Self, Self::Error> {
                <Self as #cel::values::FromValue>::from_value(value)
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
                "opaque types cannot have const generic parameters",
            ));
        }

        let attrs = Attrs::from_ast(input)?;

        let mut generic_params = Vec::new();
        let mut generics = input.generics.clone();
        let cel_path = attrs.cel_path();
        let bound: syn::TraitBound = syn::parse2(quote! { #cel_path::values::TypedValue })?;
        for param in generics.params.iter_mut() {
            if let syn::GenericParam::Type(ref mut type_param) = *param {
                type_param
                    .bounds
                    .push(syn::TypeParamBound::Trait(bound.clone()));
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
// #[cel_cxx(type = "MyType")]
// #[cel_cxx(crate = my_crate)]
// #[cel_cxx(type = "MyType", crate = my_crate)]
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
