use proc_macro2::TokenStream;
use quote::quote;

pub fn run(input: TokenStream) -> TokenStream {
    quote! {
        println!("private");
    }
}