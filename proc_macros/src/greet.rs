use proc_macro2::TokenStream;
use syn::{parse2, DeriveInput};

pub fn run(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(input).unwrap();
    let name = ast.ident;

    quote::quote! {
        impl Greet for #name {
            fn greet(&self) {
                println!("Hello, {}!", stringify!(#name));
            }
        }
    }
}