use proc_macro::TokenStream;
use quote::quote;
use std::path::PathBuf;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn exists(input: TokenStream) -> TokenStream {
    let value = PathBuf::from(parse_macro_input!(input as LitStr).value()).exists();

    quote! { #value }.into()
}
