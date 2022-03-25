use proc_macro2::{Ident, TokenStream};

use crate::part::ArgCount;
use crate::part::FromString;
use crate::part::Part;
use crate::part::PartInternal;
use crate::part::Variant;
use quote::quote;

pub(crate) fn generate_enum(ty: &Ident, parts: impl Iterator<Item = Part<Variant>>) -> TokenStream {
    quote! {
        #[allow(non_camel_case_types)]
        pub enum #ty {
            #(#parts),*
        }
    }
}

pub(crate) fn generate_impl(ty: &Ident, parts: &[PartInternal]) -> TokenStream {
    let namespace = ty.to_string().to_ascii_lowercase();
    if !namespace.is_ascii() {
        panic!("non ascii ident");
    }

    let ifs = parts
        .iter()
        .cloned()
        .map(Part::<ArgCount>::from)
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

    let idents = parts.iter().cloned().map(Part::<FromString>::from);
    let strings = idents.clone().map(|ident| ident.to_string());

    let impl_block = quote! {
        impl<#ty> crate::library::Library<#ty> for #ty {
            const NAMESPACE: &'static str = #namespace;
            const MAX_ARGS: usize = #max_args;
            fn from_string(namespaces: &[&str], identifier: &str) -> Result<#ty, crate::error::Error> {
                match namespaces {
                    [namespace, ..] => match *namespace {
                        Self::NAMESPACE => Self::from_string(&namespaces[1..], identifier),
                        _ => Err(crate::error::Error::InvalidNamespace),
                    }
                    [] => match identifier {
                        #(#strings => #ty::#idents),*,
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
    impl_block
}
