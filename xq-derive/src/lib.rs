//#![no_std]
//extern crate alloc;
//use alloc::{format, vec};

//extern crate pro_macro;
//extern crate syn;
#[macro_use]
extern crate quote;

//use xq_std::*;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};

use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, Item, Meta, NestedMeta};

#[allow(dead_code)]
fn attach_error<A>(mut v: syn::Result<A>, msg: &str) -> syn::Result<A> {
    if let Err(e) = v.as_mut() {
        let span = e.span();
        e.combine(syn::Error::new(span, msg));
    }
    v
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn contains_attribute<'a, I: IntoIterator<Item = &'a Meta>>(iter: I, name: &str) -> bool {
    iter.into_iter().any(|attr| attr.path().is_ident(name))
}

#[allow(dead_code)]
fn contract_function_optional_args_tokens<'a, I: Copy + IntoIterator<Item = &'a Meta>>(
    attrs: I,
    amount_ident: &syn::Ident,
    required_args: &mut Vec<&str>,
) -> (proc_macro2::TokenStream, Vec<proc_macro2::TokenStream>) {
    let mut setup_fn_args = proc_macro2::TokenStream::new();
    let mut fn_args = vec![];
    if contains_attribute(attrs, "payable") {
        required_args.push("amount: Amount");
        fn_args.push(quote!(#amount_ident));
    } else {
        setup_fn_args.extend(quote! {
            if #amount_ident != 0 {
                return -1;
            }
        });
    };

    if contains_attribute(attrs, "enable_logger") {
        required_args.push("logger: &mut impl HasLogger");
        let logger_ident = format_ident!("logger");
        setup_fn_args.extend(quote!(let mut #logger_ident = xq_std::Logger::init();));
        fn_args.push(quote!(&mut #logger_ident));
    }
    (setup_fn_args, fn_args)
}

fn get_attribute(metas: Vec<NestedMeta>, value: &str) -> syn::Result<Option<syn::LitStr>> {
    for item in metas {
        match item {
            NestedMeta::Meta(meta) => match meta {
                Meta::NameValue(nv) => {
                    if nv.path.is_ident(value) {
                        if let syn::Lit::Str(lit) = nv.lit {
                            return Ok(Some(lit));
                        } else {
                            return Err(syn::Error::new(
                                nv.span(),
                                format!("the attribute must be a string literal."),
                            ));
                        }
                    }
                }
                Meta::Path(_) => {
                    return Err(syn::Error::new(
                        meta.span(),
                        format!("the attribute must be a string literal."),
                    ))
                }
                Meta::List(_) => {
                    return Err(syn::Error::new(
                        meta.span(),
                        format!("the attribute must be a string literal."),
                    ))
                }
            },
            NestedMeta::Lit(_) => {
                return Err(syn::Error::new(
                    item.span(),
                    format!("the attribute must be a string literal."),
                ))
            }
        }
    }
    return Ok(None);
}

fn contains_attribute2(metas: Vec<NestedMeta>, value: &str) -> bool {
    for attr in metas.iter() {
        match attr {
            NestedMeta::Meta(meta) => {
                if meta.path().is_ident(value) {
                    return true;
                }
            }
            NestedMeta::Lit(_) => {}
        }
    }
    //metas.into_iter().any(|attr| attr.path().is_ident(name));
    false
}

#[proc_macro_attribute]
pub fn init(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as AttributeArgs);
    let contract = get_attribute(attrs.clone(), "contract").unwrap().unwrap();
    //let pay = contains_attribute2(attrs.clone(), "payable");

    let mut setup_function_args = proc_macro2::TokenStream::new();
    let mut function_args = vec![];
    let amount_ident = format_ident!("amount");

    if contains_attribute2(attrs.clone(), "payable") {
        function_args.push(quote!(#amount_ident));
    } else {
        setup_function_args.extend(quote! {
            if #amount_ident != 0 {
                return -1;
            }
        });
    };

    let ast = parse_macro_input!(item as Item);
    let init_function_name = format_ident!("init_{}", contract.value());
    let init_name = format!("init_{}", contract.value());
    //let mut  output = proc_macro2::TokenStream::new();

    let function_name = if let syn::Item::Fn(itemfn) = ast.clone() {
        itemfn.sig.ident
    } else {
        return syn::Error::new(Span::call_site(), format!("#[init] must be function."))
            .into_compile_error()
            .into();
    };
    let output = quote! {
        #ast
        #[export_name = #init_name]
        pub extern "C" fn #init_function_name(amount:u64)->i32{
            use xq_std::{ContractContext,serde_json};
            let initctx =  ContractContext;
            #setup_function_args
            match #function_name(initctx, #(#function_args),*){
                Ok(o)=>{
                    let r = serde_json::to_string(&o).unwrap();
                    ContractContext.return_data(r.clone());
                    return 1
                }
                Err(e)=>{
                    let err = e.to_string();
                    ContractContext.error(err.clone());
                    return 0
                }
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn call(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as AttributeArgs);
    let contract = get_attribute(attrs.clone(), "contract").unwrap().unwrap();
    let function = get_attribute(attrs.clone(), "function").unwrap().unwrap();
    //let payable = contains_attribute2(attrs.clone(), "payable");

    let mut setup_function_args = proc_macro2::TokenStream::new();
    let mut function_args = vec![];
    let amount_ident = format_ident!("amount");

    if contains_attribute2(attrs.clone(), "payable") {
        function_args.push(quote!(#amount_ident));
    } else {
        setup_function_args.extend(quote! {
            if #amount_ident != 0 {
                return -1;
            }
        });
    };

    let ast = parse_macro_input!(item as Item);
    let call_function_name = format_ident!("{}_{}", contract.value(), function.value());
    //let call_function_name = format_ident!("call_{}", "abc");
    //let mut  output = proc_macro2::TokenStream::new();

    let function_name = if let syn::Item::Fn(itemfn) = ast.clone() {
        itemfn.sig.ident
    } else {
        return syn::Error::new(Span::call_site(), format!("#[call] must be function."))
            .into_compile_error()
            .into();
    };

    let output = quote! {
        #ast
        #[no_mangle]
        pub extern "C" fn #call_function_name(amount:u64)->i32{

            use xq_std::{ContractContext, serde_json};
            let initctx =  ContractContext;
            //let state = ContractContext.state_get();
            #setup_function_args
            match #function_name(initctx, #(#function_args),*){
                Ok(o)=>{
                    let r = serde_json::to_string(&o).unwrap();
                    ContractContext.return_data(r.clone());
                    return 1
                }
                Err(e)=>{
                    let err = e.to_string();
                    ContractContext.error(err.clone());
                    return 0
                }
            }

        }
    };

    TokenStream::from(output)
}

#[proc_macro_derive(Output)]
pub fn output_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input.clone()).unwrap();
    let enum_ident = ast.clone().ident;
    let enum_data = match &ast.data {
        syn::Data::Enum(data) => data,
        _ => {
            return syn::Error::new(ast.span(), "Output can only be derived for enums.")
                .to_compile_error()
                .into()
        }
    };
    let mut field_fmt = vec![];
    let mut field_string = vec![];
    for field in &enum_data.variants {
        field_fmt.push(field.ident.clone());
        field_string.push(field.ident.to_string());
    }

    let display = quote! {
        impl core::fmt::Display for #enum_ident{
            fn fmt(&self, f:&mut core::fmt::Formatter) -> core::fmt::Result{
                //#(core::fmt::write!(f, "{}", #field_fmt.clone().unwrap());)*
                //use core::fmt::write;
                match self{
                    #(#enum_ident::#field_fmt => core::write!(f, "{}", #field_string),)*
                }
            }
        }
    };

    let ge = quote! {
        #display
    };
    return ge.into();
}

#[proc_macro_attribute]
pub fn state(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut output = proc_macro2::TokenStream::new();
    let data_ident = if let Ok(ast) = syn::parse::<syn::ItemStruct>(item.clone()) {
        ast.to_tokens(&mut output);
        ast.ident
    } else {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(item),
            "#[state] only supports structs.",
        )
        .to_compile_error()
        .into();
    };

    let impl_state = quote! {
        impl #data_ident {
            fn contract_state_set(&mut self,){
                return
            }

            fn contract_state_get(&self){
                return
            }
        }

    };

    impl_state.to_tokens(&mut output);

    output.into()
}
