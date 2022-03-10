extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, ImplItem, ItemImpl};

#[proc_macro_attribute]
pub fn library_derive(attr: TokenStream, item: TokenStream) -> TokenStream {
    let original = item.clone();
    let item_impl = syn::parse::<ItemImpl>(item).unwrap();
    let trait_impl = impl_library(&item_impl);
    quote! {
        item_impl
        trait_impl
    }.into()

}

fn impl_library(item_impl: &syn::ItemImpl) -> TokenStream {
    let ty = &item_impl.self_ty;
    let name = &item_impl.self_ty;
    quote! {
        impl<#ty> Library<#ty> for #ty {
            const NAMESPACE: &'static str = stringify!(#ty);
            const MAX_ARGS = 5
        }
    }.into()
}
