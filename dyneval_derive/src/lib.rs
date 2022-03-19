extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use std::fmt::Write;
use syn::{
    self,
    token::{Brace, Semi},
    Ident, Item, ItemFn, ItemMod, ItemType, Token,
};

#[proc_macro_attribute]
pub fn library_from_mod(attr: TokenStream, item: TokenStream) -> TokenStream {
    let original = item.clone();
    let ident = syn::parse::<Ident>(attr).ok();
    let module = syn::parse::<ItemMod>(item).expect("not a module");
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
    let trait_impl = impl_wrapper(&ident, &module);
    (original.to_string() + &trait_impl.to_string())
        .parse()
        .unwrap()
}
fn impl_wrapper(ident: &Option<syn::Ident>, module: &syn::ItemMod) -> TokenStream {
    impl_library(ident, module)
}
#[proc_macro]
pub fn library(item: TokenStream) -> TokenStream {
    library_macro(item)
}
#[proc_macro_attribute]
pub fn erase(attr: TokenStream, item: TokenStream) -> TokenStream {
    quote! {}.into()
}
enum Part {
    Function(ItemFn),
    Import(Ident),
}
fn library_macro(item: TokenStream) -> TokenStream {
    let mut iter = item.into_iter();

    let ident = syn::parse::<Ident>(iter.next().expect("expected identifier").into())
        .expect("expected identifier");

    syn::parse::<Token![;]>(iter.next().expect("expected separator").into())
        .expect("expected separator");

    let parts = iter
        .map(|tt| match syn::parse::<ItemFn>(tt.clone().into()) {
            Ok(item_fn) => Part::Function(item_fn),
            Err(_) => match syn::parse::<Ident>(tt.into()) {
                Ok(ident) => Part::Import(ident),
                Err(_) => panic!("not a function or a type that implements `Library`"),
            },
        })
        .collect::<Vec<_>>();
    "".parse().unwrap()
}
fn generate_enum(ident: Ident, parts: &[Part]) -> TokenStream {
    let idents = parts.iter().map(|part| match part {
        Part::Function(function) => function.sig.ident.to_owned(),
        Part::Import(ident) => ident.to_owned(),
    });
    quote! {
        #ident {
            #(#idents),*
        }
    }
    .into()
}
fn generate_impl(ident: &Ident, parts: &[Part]) -> TokenStream {
    let mut namespace = ident.clone().to_string();
    if !namespace.is_ascii() {
        panic!("non ascii ident");
    }
    namespace
        .get_mut(0..=1)
        .expect("0 length ident!?")
        .make_ascii_uppercase();

    let ifs = parts
        .iter()
        .map(|part| match part {
            Part::Function(function) => function.sig.inputs.len().to_string(),
            Part::Import(ident) => format!("{}::MAX_ARGS", ident),
        })
        .map(|count| {
            quote! {
                __interal_temp = #count;
                if __internal_temp > __internal_max { __internal_max = __internal_temp}
            }
        });
    let max_args = quote! {
        {
            let mut __internal_temp = 0;
            let mut __internal_max = 0;
            #(#ifs),*
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
                        #(#strings => Ok(#idents)),*
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
fn impl_library(ident: &Option<syn::Ident>, module: &syn::ItemMod) -> TokenStream {
    let ident = ident
        .as_ref()
        .or(Some(&module.ident))
        .expect("no identier?");
    let span = ident.span();
    let mut name = ident.to_string();
    if !name.is_ascii() {
        //compile_error!("non ascii name");
        panic!("non ascii name");
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
        #[allow(non_camel_case_type)]
        pub enum #name {
            #(#identifiers),*
        }
        impl<#name> crate::library::Library<#name> for #name {
            const NAMESPACE: &'static str = #namespace;
            const MAX_ARGS: usize = #max_args;
            fn from_string(namespaces: &[&str], identifier: &str) -> Result<#name, crate::error::Error> {
                match namespaces {
                    [namespace, ..] => match *namespace {
                        Self::NAMESPACE => Self::from_string(&namespaces[1..], identifier),
                        _ => Err(crate::error::Error::InvalidNamespace),
                    }
                    [] => match identifier {
                        //#(stringify!(#identifiers) => Ok(Self::#identifiers)),*
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
    }.into();
    tt
    /*
    ;
    let tt = tt.to_string();
    quote! {
        pub fn test_print() {
            println!("{}", #tt);
        }
    }
    .into()
    */
}
