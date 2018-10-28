extern crate proc_macro;
extern crate quote;
extern crate syn;

//use quote::quote;
use proc_macro::TokenStream;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn nif_init(attrs: TokenStream, input: TokenStream) -> TokenStream {

    input
}

#[proc_macro_attribute]
pub fn nif(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let output = input.clone();
    //let attrs_item = syn::parse(attrs).expect("nif_fn failed to parse");
    let input_item: ItemFn = syn::parse(input).expect("nif_fn failed to parse");

    println!("{:#?}", attrs);
    //println!("{:#?}", input_item);

    output
}
