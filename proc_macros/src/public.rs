use proc_macro2::TokenStream;
use syn::{parse2, DeriveInput};

pub fn run(attr: TokenStream, item: TokenStream) -> TokenStream {
   let ast: DeriveInput = parse2(item).unwrap();
    quote::quote! {
        println!("public");
    }
}