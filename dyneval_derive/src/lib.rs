extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, ImplItem};
struct A {
    a: i32,
    b: i32,
}
#[proc_macro_attribute]
pub fn library_derive(attr: TokenStream, item: TokenStream) -> TokenStream {
    let test = A { a: 32, b: 32 };
    let ast = syn::parse::<ImplItem>(item).unwrap();
    impl_library(&ast)
}

fn impl_library(ast: &syn::DeriveInput) -> TokenStream {
    todo!()
}
