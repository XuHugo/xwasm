#![no_std]

extern crate pro_macro;
extern crate syn;
#[macro_use]
extern crate quote;


use xq_common::*;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
#[cfg(feature = "build-schema")]
use std::collections::HashMap;
use std::{convert::TryFrom, ops::Neg};
use syn::{
    parse::Parser, parse_macro_input, punctuated::*, spanned::Spanned, DataEnum, Ident, Meta, Token,
};


fn unwrap_or_report(v: syn::Result<TokenStream>) -> TokenStream {
    match v {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

fn attach_error<A>(mut v: syn::Result<A>, msg: &str) -> syn::Result<A> {
    if let Err(e) = v.as_mut() {
        let span = e.span();
        e.combine(syn::Error::new(span, msg));
    }
    v
}

fn get_attribute_value<'a, I: IntoIterator<Item = &'a Meta>>(
    iter: I,
    name: &str,
) -> syn::Result<Option<&'a syn::LitStr>> {
    for attr in iter.into_iter() {
        match attr {
            Meta::NameValue(mnv) => {
                if mnv.path.is_ident(name) {
                    if let syn::Lit::Str(lit) = &mnv.lit {
                        return Ok(Some(lit));
                    } else {
                        return Err(syn::Error::new(
                            mnv.span(),
                            format!("The `{}` attribute must be a string literal.", name),
                        ));
                    }
                }
            }
            Meta::Path(p) => {
                if p.is_ident(name) {
                    return Err(syn::Error::new(
                        attr.span(),
                        format!("The `{}` attribute must have a string literal value.", name),
                    ));
                }
            }
            Meta::List(p) => {
                if p.path.is_ident(name) {
                    return Err(syn::Error::new(
                        attr.span(),
                        format!("The `{}` attribute must have a string literal value.", name),
                    ));
                }
            }
        }
    }
    Ok(None)
}

#[proc_macro_attribute]
pub fn init(attr: TokenStream, item: TokenStream) -> TokenStream {
    unwrap_or_report(init_worker(attr, item))
}

fn init_worker(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let attrs = Punctuated::<Meta, Token![,]>::parse_terminated.parse(attr)?;

    let contract_name = get_attribute_value(attrs.iter(), "contract")?.ok_or_else(|| {
        syn::Error::new(
            Span::call_site(),
            "A name for the contract must be provided, using the contract attribute. For example, \
             #[init(contract = \"my-contract\")]",
        )
    })?;

    let ast: syn::ItemFn =
        attach_error(syn::parse(item), "#[init] can only be applied to functions.")?;

    let fn_name = &ast.sig.ident;
    let rust_export_fn_name = format_ident!("export_{}", fn_name);
    let wasm_export_fn_name = format!("init_{}", contract_name.value());

    let amount_ident = format_ident!("amount");

    
}
