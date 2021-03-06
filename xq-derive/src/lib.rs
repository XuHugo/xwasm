//#![no_std]

//extern crate pro_macro;
//extern crate syn;
//#[macro_use]
//extern crate quote;


use xq_std::*;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, format_ident, ToTokens};
#[cfg(feature = "build-schema")]
use std::collections::HashMap;
use std::{convert::TryFrom, ops::Neg};
use syn::{
    parse::Parser, parse_macro_input, punctuated::*, spanned::Spanned, DataEnum, Ident, Meta, Token, AttributeArgs, NestedMeta, Item
};


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

fn contains_attribute<'a, I: IntoIterator<Item = &'a Meta>>(iter: I, name: &str) -> bool {
    iter.into_iter().any(|attr| attr.path().is_ident(name))
}

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
            if #amount_ident.micro_gtu != 0 {
                return -1;
            }
        });
    };

    if contains_attribute(attrs, "enable_logger") {
        required_args.push("logger: &mut impl HasLogger");
        let logger_ident = format_ident!("logger");
        setup_fn_args.extend(quote!(let mut #logger_ident = concordium_std::Logger::init();));
        fn_args.push(quote!(&mut #logger_ident));
    }
    (setup_fn_args, fn_args)
}

fn get_attribute(metas:Vec<NestedMeta>, value: &str)-> syn::Result<Option<syn::LitStr>>{
    for item in  metas{
        match item{
            NestedMeta::Meta(meta)=>{
                match meta{
                    Meta::NameValue(nv)=>{
                        if nv.path.is_ident(value){
                            if let syn::Lit::Str(lit) = nv.lit{
                                return Ok(Some(lit))
                            }else{
                                return Err(syn::Error::new(nv.span(), format!("the attribute must be a string literal.")))
                            }
                        }
                    }, 
                    Meta::Path(_)=>{
                        return Err(syn::Error::new(meta.span(), format!("the attribute must be a string literal.")))
                    },
                    Meta::List(_)=>{
                        return Err(syn::Error::new(meta.span(), format!("the attribute must be a string literal.")))
                    },
                }
            },
            NestedMeta::Lit(_)=>{
                return Err(syn::Error::new(item.span(), format!("the attribute must be a string literal.")))
            },
        }
    }
    return Ok(None)
}

fn contains_attribute2(metas:Vec<NestedMeta>, value: &str) -> bool {
    for attr in metas.iter(){
        match attr{
            NestedMeta::Meta(meta)=>{
                eprintln!("contains_attribute2 :{:?}",meta);
                if meta.path().is_ident(value){
                    return true
                }
            }
            NestedMeta::Lit(_)=>{}
        }
    }
    //metas.into_iter().any(|attr| attr.path().is_ident(name));
    false
}

#[proc_macro_attribute]
pub fn init(attr: TokenStream, item: TokenStream) -> TokenStream {

    let attrs = parse_macro_input!(attr as AttributeArgs);
    eprintln!("init attrs :{:?}",attrs);

    let c = get_attribute(attrs.clone(), "contract").unwrap().unwrap();
    eprintln!("init contract  :{:?}",c);

    let pay = contains_attribute2(attrs.clone(), "payable");
    eprintln!("init contains payable :{:?}",pay);

    let mut setup_fn_args = proc_macro2::TokenStream::new();
    let mut fn_args = vec![];
    let amount_ident = format_ident!("amount");

    if contains_attribute2(attrs.clone(), "payable"){
        fn_args.push(quote!(#amount_ident));
    } else {
        setup_fn_args.extend(quote! {
            if #amount_ident != 0 {
                return -1;
            }
        });
    };


    let ast = parse_macro_input!(item as Item);
    let init_fn_name = format_ident!("init_{}",c.value());
    let mut  output = proc_macro2::TokenStream::new();

    let fn_name = if let syn::Item::Fn(itemfn) =ast.clone(){
        let fn_nam = itemfn.sig.ident;
        fn_nam

    }else{
        return syn::Error::new(Span::call_site(), format!("#[init] must be function.")).into_compile_error().into()
    };
    output = quote! {
        #ast
        pub extern "C" fn #init_fn_name(amount:u64)->i32{
            println!("new init function !");
            use xq_std::{ContractContext};
            let initctx =  ContractContext;
            #setup_fn_args
            match #fn_name(initctx, #(#fn_args),*){
                Ok(o)=>{
                    let r = serde_json::to_string(&o).unwrap();
                    ContractContext.return_data(r.clone());
                    println!{"init ok {:?}", o};
                    return 1
                }
                Err(e)=>{
                    let err = e.to_string();
                    ContractContext.error(err.clone());
                    println!{"init err{:?}",err};
                    return 0
                }
            }
            0
        }
    };

    // let v = init_worker(attr, item);
    // match v {
    //     Ok(ts) => ts,
    //     Err(e) => e.to_compile_error().into(),
    // }
    eprintln!("init  == {}", output);
    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn call(attr: TokenStream, item: TokenStream) -> TokenStream {

    let attrs = parse_macro_input!(attr as AttributeArgs);
    eprintln!("call attrs :{:?}",attrs);

    let c = get_attribute(attrs.clone(), "contract").unwrap().unwrap();
    eprintln!("call contract  :{:?}",c);

    let f = get_attribute(attrs.clone(), "func").unwrap().unwrap();
    eprintln!("call func  :{:?}", f);

    let pay = contains_attribute2(attrs.clone(), "payable");
    eprintln!("call contains payable :{:?}",pay);

    let mut setup_fn_args = proc_macro2::TokenStream::new();
    let mut fn_args = vec![];
    let amount_ident = format_ident!("amount");

    if contains_attribute2(attrs.clone(), "payable"){
        fn_args.push(quote!(#amount_ident));
    } else {
        setup_fn_args.extend(quote! {
            if #amount_ident != 0 {
                return -1;
            }
        });
    };


    let ast = parse_macro_input!(item as Item);
    let call_fn_name = format_ident!("call_{}",f.value());
    let mut  output = proc_macro2::TokenStream::new();

    let fn_name = if let syn::Item::Fn(itemfn) =ast.clone(){
        let fn_nam = itemfn.sig.ident;
        fn_nam

    }else{
        return syn::Error::new(Span::call_site(), format!("#[call] must be function.")).into_compile_error().into()
    };
    output = quote! {
        #ast
        pub extern "C" fn #call_fn_name(amount:u64)->i32{
            println!("new call function !");
            use xq_std::{ContractContext};
            let initctx =  ContractContext;
            //let state = ContractContext.state_get();
            #setup_fn_args
            match #fn_name(initctx, #(#fn_args),*){
                Ok(o)=>{
                    let r = serde_json::to_string(&o).unwrap();
                    ContractContext.return_data(r.clone());
                    println!{"call ok {:?}", r};
                    return 1
                }
                Err(e)=>{
                    let err = e.to_string();
                    ContractContext.error(err.clone());
                    println!{"call err{:?}",err};
                    return 0
                }
            }
            0
        }
    };
    eprintln!("call == {}", output);
    TokenStream::from(output)
}

#[proc_macro_derive(Output)]
pub fn output_derive(input: TokenStream) -> TokenStream{
    let ast: syn::DeriveInput = syn::parse(input.clone()).unwrap();
    let enum_ident = ast.clone().ident;
    let enum_data = match &ast.data {
        syn::Data::Enum(data) => data,
        _ => return syn::Error::new(ast.span(), "Reject can only be derived for enums.").to_compile_error().into(),
    };
    let mut field_fmt= vec![];
    let mut field_string= vec![];
    for field in &enum_data.variants{
        field_fmt.push(field.ident.clone());
        field_string.push(field.ident.to_string());
    }

    let display = quote! {
        impl core::fmt::Display for #enum_ident{
            fn fmt(&self, f:&mut core::fmt::Formatter) -> core::fmt::Result{
                //#(core::fmt::write!(f, "{}", #field_fmt.clone().unwrap());)*
                //use core::fmt::write;
                match self{
                    #(#field_fmt => core::write!(f, "{}", #field_string),)*
                }
            }
        }
    };
    eprintln!("eeeeeeerror :{}",display);

    let ge = quote! {
        #display
    };
    return ge.into()
}

#[proc_macro_attribute]
pub fn state(_attr:TokenStream, item:TokenStream) -> TokenStream{
    let mut output = proc_macro2::TokenStream::new();
    let data_ident = if let Ok(ast) = syn::parse::<syn::ItemStruct>(item.clone()) {
        ast.to_tokens(&mut output);
        ast.ident
    }else{
         return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(item),
            "#[state] only supports structs.",
        ).to_compile_error().into()
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

    eprintln!("state == {}",output);
    output.into()
}
