#![recursion_limit = "256"]

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

fn impl_wrapper(args: &syn::AttributeArgs, item: &syn::Item) -> proc_macro2::TokenStream {
    let fun = if let syn::Item::Fn(fun) = item {
        fun
    } else {
        panic!("`#[rusterlium]` attribute only supported on functions");
    };

    let name = &fun.ident;
    let decl = &fun.decl;
    let inputs = &decl.inputs;

    let erl_func_name = extract_attr_value(&args, "name")
        .map(|ref n| syn::Ident::new(n, Span::call_site()))
        .unwrap_or_else(|| fun.ident.clone());

    //let flags = extract_attr_value(&args, "schedule").unwrap_or_else(|| "Normal".to_string());

    if decl.variadic.is_some() {
        panic!("variadic functions are not supported")
    }

    // declare the function
    let function = item.clone().into_token_stream();
    let decoded_terms = extract_inputs(inputs);
    let argument_names = create_function_params(inputs);
    let arity = inputs.len();

    // Return a const ErlNifFunc.
    //
    // The name is based on the user function name followed by `_ErlNifFunc`.
    // For example: Given the user function `add`, the const will be named `add_ErlNifFunc`.
    let erl_nif_func_const = syn::Ident::new(&format!("{}_ErlNifFunc", name), Span::call_site());

    quote! {
        #function

        const #erl_nif_func_const: erlang_nif_sys::ErlNifFunc = erlang_nif_sys::ErlNifFunc {
            name: concat!(stringify!(#erl_func_name), "\x00") as *const str as *const u8,
            arity: #arity as u32,
            function: {
                unsafe extern "C" fn nif(nif_env: *mut ErlNifEnv, argc: c_int, argv: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
                    let lifetime = ();
                    let env = rustler::Env::new(&lifetime, nif_env);

                    let terms = std::slice::from_raw_parts(argv, argc as usize)
                        .iter()
                        .map(|term| rustler::Term::new(env, *term))
                        .collect::<Vec<rustler::Term>>();

                    fn wrapper<'a>(env: rustler::Env<'a>, args: &[rustler::Term<'a>]) -> rustler::Term<'a> {
                        let panic_result = std::panic::catch_unwind(|| {
                            #decoded_terms

                            let result = #name(#argument_names);

                            result.encode(env)
                        });

                        match panic_result {
                            Ok(result) => result.encode(env),
                            Err(err) => std::panic::resume_unwind(err)
                        }
                    }

                    wrapper(env, &terms).as_c_arg()
                }
                nif
            },
            flags: 0 as u32,
        };
    }
}

fn extract_attr_value<'a>(input: &'a syn::AttributeArgs, attr: &str) -> Option<String> {
    use syn::{Lit, Meta, MetaNameValue, NestedMeta};

    for item in input.iter() {
        if let NestedMeta::Meta(Meta::NameValue(meta_name_value)) = item {
            let MetaNameValue { ident, lit, .. } = meta_name_value;
            if ident == attr {
                if let Lit::Str(lit) = lit {
                    return Some(lit.value());
                }
            }
        }
    }

    None
}

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
        let name = if let syn::FnArg::Captured(syn::ArgCaptured {
            pat: syn::Pat::Ident(syn::PatIdent { ref ident, .. }),
            ..
        }) = *item
        {
            ident
        } else {
            panic!("unsupported input given: {:?}", stringify!(&item));
        };

        tokens.extend(quote!(
            #name,
        ));
    }

    tokens
}
