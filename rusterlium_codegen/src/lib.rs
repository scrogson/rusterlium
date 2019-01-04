#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;

#[proc_macro_attribute]
pub fn rusterlium(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let ast: syn::Item = syn::parse_macro_input!(input as syn::Item);
    let expanded: proc_macro2::TokenStream = impl_wrapper(&args, &ast);

    TokenStream::from(expanded)
}

fn impl_wrapper(_args: &syn::AttributeArgs, item: &syn::Item) -> proc_macro2::TokenStream {
    let fun = if let syn::Item::Fn(fun) = item {
        fun
    } else {
        panic!("`#[rusterlium]` attribute only supported on functions");
    };

    //let name = extract_attr_value(&args, "name").unwrap_or_else(|| input.ident.to_string());
    //let flags = extract_attr_value(&args, "schedule").unwrap_or_else(|| "Normal".to_string());

    let name = &fun.ident;
    let decl = &fun.decl;
    let inputs = &decl.inputs;

    if decl.variadic.is_some() {
        panic!("variadic functions are not supported")
    }

    // declare the function
    let function = item.clone().into_token_stream();
    let wrapper = syn::Ident::new(&format!("{}_wrapper", name), Span::call_site());

    let decoded_terms = extract_inputs(inputs);
    let argument_names = create_function_params(inputs);

    // wrap the original function in a wrapper function
    let wrapped = quote! {
        #[no_mangle]
        pub extern "C" fn #wrapper<'a>(env: rustler::Env<'a>, args: &[rustler::Term<'a>]) -> rustler::Term<'a> {
            use std::panic;
            use rustler::{Decoder, Encoder};

            let panic_result = panic::catch_unwind(|| {
                #function
                #decoded_terms

                let result = #name(#argument_names);

                result.encode(env)
            });

            match panic_result {
                Ok(result) => {
                    result.encode(env)
                }
                Err(err) => {
                    panic::resume_unwind(err)
                }
            }
        }
    };

    wrapped
}

//fn extract_attr_value<'a>(input: &'a syn::AttributeArgs, attr: &str) -> Option<String> {
//use syn::{Lit, Meta, MetaNameValue, NestedMeta};

//for item in input.iter() {
//if let NestedMeta::Meta(Meta::NameValue(meta_name_value)) = item {
//let MetaNameValue { ident, lit, .. } = meta_name_value;
//if ident == attr {
//if let Lit::Str(lit) = lit {
//return Some(lit.value());
//}
//}
//}
//}

//None
//}

fn extract_inputs<'a>(inputs: &'a Punctuated<syn::FnArg, Comma>) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();

    for (i, item) in inputs.iter().enumerate() {
        let (name, typ) = if let syn::FnArg::Captured(ref captured) = *item {
            (&captured.pat, &captured.ty)
        } else {
            panic!("unsupported input given: {:?}", stringify!(&item));
        };

        let error = format!(
            "unsupported function argument type `{}` for `{}`",
            quote!(#typ),
            quote!(#name)
        );

        let arg = quote! {
            let #name: #typ = args[#i]
                .decode()
                .map_err(|_| #error)
                .expect(#error);
        };

        tokens.extend(arg);
    }

    tokens
}

fn create_function_params<'a>(
    inputs: &'a Punctuated<syn::FnArg, Comma>,
) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();

    for item in inputs.iter() {
        let name = if let syn::FnArg::Captured(ref captured) = *item {
            &captured.pat
        } else {
            panic!("unsupported input given: {:?}", stringify!(&item));
        };

        tokens.extend(quote!(
            #name,
        ));
    }

    tokens
}
