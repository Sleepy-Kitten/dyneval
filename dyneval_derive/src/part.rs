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
            PartInternal::Function(item_fn) => tokens.append_all(item_fn.into_token_stream()),
            PartInternal::Import(ident) => todo!(),
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
impl ToTokens for PartInternal {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Function(item_fn) => tokens.append(item_fn.sig.ident.clone()),
            Self::Import(ident) => tokens.append_all(quote! { #ident(#ident) }),
        }
    }
}
impl<T> From<PartInternal> for Part<T> {
    fn from(internal: PartInternal) -> Self {
        Part(internal, PhantomData::<T>)
    }
}
