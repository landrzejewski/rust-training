mod greet;
mod public;
mod private;

use proc_macro::TokenStream;

#[proc_macro_derive(Greet)]
pub fn greet_macro(input: TokenStream) -> TokenStream {
    greet::run(input.into()).into()
}


#[proc_macro_attribute]
pub fn public(attr: TokenStream, input: TokenStream) -> TokenStream {
    public::run(attr.into(), input.into()).into()
}

#[proc_macro]
pub fn private(input: TokenStream) -> TokenStream {
    private::run(input.into()).into()
}