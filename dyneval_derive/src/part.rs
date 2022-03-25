use std::marker::PhantomData;

use proc_macro2::TokenTree;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Ident, ItemFn};

pub struct Variant;
pub struct FromString;
pub struct ArgCount;

#[derive(Clone)]
pub(crate) enum PartInternal {
    Function(ItemFn),
    Import(Ident),
}

pub(crate) struct Part<T>(pub PartInternal, PhantomData<T>);

impl ToTokens for Part<Variant> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.0 {
            PartInternal::Function(item_fn) => tokens.append(item_fn.sig.ident.clone()),
            PartInternal::Import(ident) => tokens.append_all(quote! { #ident(#ident) }),
        }
    }
}
impl ToTokens for Part<FromString> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.0 {
            PartInternal::Function(item_fn) => tokens.append_all(quote! { Ok(#item_fn) }),
            PartInternal::Import(ident) => {
                tokens.append_all(quote! { #ident.from_string(namespaces, identifier) })
            }
        }
    }
}
impl std::fmt::Display for Part<FromString> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            PartInternal::Function(item_fn) => item_fn.sig.ident.fmt(f),
            PartInternal::Import(ident) => ident.fmt(f),
        }
    }
}
impl ToTokens for Part<ArgCount> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.0 {
            PartInternal::Function(item_fn) => {
                tokens.append_all(item_fn.sig.inputs.len().into_token_stream())
            }
            PartInternal::Import(ident) => tokens.append_all(quote! {#ident::MAX_ARGS}),
        }
    }
}
impl<T> From<PartInternal> for Part<T> {
    fn from(internal: PartInternal) -> Self {
        Part(internal, PhantomData::<T>)
    }
}
