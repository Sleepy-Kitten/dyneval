extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{self, token::Struct, Ident, ImplItem, ItemImpl, ItemMod};

#[proc_macro_attribute]
pub fn library(attr: TokenStream, item: TokenStream) -> TokenStream {
    let original = item.clone();
    let ident = syn::parse::<Ident>(attr).ok();
    let module = syn::parse::<ItemMod>(item.clone()).expect("a");
    let trait_impl = impl_library(&ident, &module);
    //original.append(trait_impl)

    trait_impl
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
fn impl_library(ident: &Option<syn::Ident>, module: &syn::ItemMod) -> TokenStream {
    let ident = ident.as_ref().or(Some(&module.ident)).expect("v");
    let span = ident.span();
    let mut name = ident.to_string();
    if !name.is_ascii() {
        panic!()
    }

    let namespace = name.to_ascii_lowercase();
    match name.get_mut(0..=1) {
        Some(slice) => slice.make_ascii_uppercase(),
        None => panic!("so short?"),
    };
    let name = syn::Ident::new(name.as_str(), span);

    let items = module.content.as_ref().expect("q").1.clone();

    let functions = items
        .into_iter()
        .filter_map(|item| match item {
            syn::Item::Fn(item_fn) => Some(item_fn),
            _ => None,
        })
        .collect::<Vec<_>>();

    let identifiers = functions
        .iter()
        .map(|item| item.sig.ident.clone())
        .collect::<Vec<_>>();

    let max_args = functions
        .iter()
        .map(|function| function.sig.inputs.len())
        .max()
        .unwrap_or(0);

    let tt = quote! {
        pub enum #name {
            #(#identifiers),*
        }
        impl<#name> crate::library::Library<#name> for #name {
            const NAMESPACE: &'static str = #namespace;
            const MAX_ARGS: usize = #max_args;
            fn from_string(namespaces: &[&str], identifier: &str) -> Result<#name, crate::error::Error> {
                match namespaces {
                    [namespace, ..] => match namespace {
                        Self::NAMESPACE => Self::from_string(namespaces[1..], identifier),
                        _ => Err(crate::error::Error::InvalidNamespace),
                    }
                    [] => match identifier {
                        #(stringify!(#identifiers) => Ok(Self::#identifiers)),*
                        _ => Err(crate::error::Error::UnkownFunction),
                    }
                }
            }
            fn call(&self, args: &[crate::value::Value]) -> Result<crate::value::Value, crate::error::Error> {
                todo!()
            }
            fn is_const(&self) -> bool {
                false
            }
        }
    }//.into();
    ;
    let tt = tt.to_string();
    quote! {
        pub fn test_print() {
            println!("{}", #tt);
        }
    }
    .into()
}
