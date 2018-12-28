extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma, ArgCaptured,
    AttributeArgs, FnArg, ItemFn,
};

#[proc_macro_attribute]
pub fn rusterlium(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);

    let name = extract_attr_value(&args, "name").unwrap_or_else(|| input.ident.to_string());
    let flags = extract_attr_value(&args, "schedule").unwrap_or_else(|| "Normal".to_string());

    println!("{:?}", name);
    println!("{:?}", flags);

    let inputs = extract_inputs(&input.decl.inputs)
        .iter()
        .enumerate()
        .map(|(i, (pat, ty))| {
            parse_quote! {
                let #pat: #ty = args[#i].decode(env)?;
            }
        })
        .collect::<Vec<syn::Stmt>>();

    println!("{:#?}", inputs);

    let quoted = quote! {
        #input
    };

    println!("{}", quoted.to_string());

    quoted.into()
}

fn extract_attr_value<'a>(input: &'a AttributeArgs, attr: &str) -> Option<String> {
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

fn extract_inputs<'a>(inputs: &'a Punctuated<FnArg, Comma>) -> Vec<(syn::Pat, syn::Type)> {
    let mut results = Vec::new();

    for item in inputs.iter() {
        if let FnArg::Captured(ArgCaptured { pat, ty, .. }) = item {
            results.push((pat.clone(), ty.clone()));
        }
    }

    results
}
