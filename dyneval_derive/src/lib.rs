extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, TokenStreamExt};
use std::fmt::Write;
use syn::{
    self,
    Ident, Item, ItemFn, ItemMod, ItemType, Token,
};

#[proc_macro_attribute]
pub fn library_from_mod(attr: TokenStream, item: TokenStream) -> TokenStream {
    let module = syn::parse::<ItemMod>(item).expect("not a module");
    let ident = syn::parse::<Ident>(attr).ok();
    let ident = ident.or(Some(module.ident)).unwrap();

    let content = &module.content.as_ref().expect("empty module").1;
    let parts = content
        .iter()
        .map(|item| match item {
            Item::Fn(item_fn) => Part::Function(item_fn.to_owned()),
            Item::Verbatim(tt) => Part::Import(
                syn::parse::<Ident>(tt.to_owned().into()).expect("not an ident or a function"),
            ),
            _ => todo!(),
        })
        .collect::<Vec<_>>();
    let enumeration: TokenStream2 = generate_enum(&ident, &parts).into();
    let implementation: TokenStream2 = generate_impl(&ident, &parts).into();
    quote! {
        #enumeration
        #implementation
    }
    .into()
}
#[proc_macro]
pub fn library_from_func(item: TokenStream) -> TokenStream {
    let parts = item
} 
enum Part {
    Function(ItemFn),
    Import(Ident),
}
fn generate_enum(ident: &Ident, parts: &[Part]) -> TokenStream {
    let idents = parts.iter().map(|part| match part {
        Part::Function(function) => function.sig.ident.to_owned(),
        Part::Import(ident) => ident.to_owned(),
    });
    quote! {
        #[allow(non_camel_case_types)]
        pub enum #ident {
            #(#idents),*
        }
    }
    .into()
}
fn generate_impl(ident: &Ident, parts: &[Part]) -> TokenStream {
    let namespace = ident.to_string().to_ascii_lowercase();
    if !namespace.is_ascii() {
        panic!("non ascii ident");
    }
    let ifs = parts
        .iter()
        .map(|part| match part {
            Part::Function(function) => function
                .sig
                .inputs
                .len()
                .to_string()
                .parse::<TokenStream2>()
                .unwrap(),
            Part::Import(ident) => format!("{}::MAX_ARGS", ident)
                .parse::<TokenStream2>()
                .unwrap(),
        })
        .map(|count| {
            quote! {
                __internal_temp = #count;
                if __internal_temp > __internal_max { __internal_max = __internal_temp}
            }
        });
    let max_args = quote! {
        {
            let mut __internal_temp = 0;
            let mut __internal_max = 0;
            #(#ifs);*;
            __internal_max
        }
    };

    let idents = parts.iter().map(|part| match part {
        Part::Function(function) => function.sig.ident.to_owned(),
        Part::Import(ident) => ident.to_owned(),
    });
    let strings = idents.clone().map(|ident| ident.to_string());

    let stream = quote! {
        impl<#ident> crate::library::Library<#ident> for #ident {
            const NAMESPACE: &'static str = #namespace;
            const MAX_ARGS: usize = #max_args;
            fn from_string(namespaces: &[&str], identifier: &str) -> Result<#ident, crate::error::Error> {
                match namespaces {
                    [namespace, ..] => match *namespace {
                        Self::NAMESPACE => Self::from_string(&namespaces[1..], identifier),
                        _ => Err(crate::error::Error::InvalidNamespace),
                    }
                    [] => match identifier {
                        #(#strings => Ok(#ident::#idents)),*,
                        _ => Err(crate::error::Error::UnknownFunction),
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
    };
    stream.into()
}
