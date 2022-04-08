extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    self, bracketed, parse::Parse, parse_macro_input, punctuated::Punctuated, token::Bracket,
    FnArg, Ident, ItemFn, Token,
};

#[proc_macro]
pub fn library(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let macro_input = parse_macro_input!(item as MacroInput);
    let generated_enum = macro_input.generate_enum();
    let generated_impl = macro_input.generate_impl();
    quote! {
        #generated_enum
        #generated_impl
    }
    .into()
}
struct MacroInput {
    name: Ident,
    _punct_1: Token![;],
    _import_bracket: Bracket,
    imports: Punctuated<Ident, Token![,]>,
    _punct_2: Token![;],
    _function_bracket: Bracket,
    functions: Punctuated<ItemFn, Token![,]>,
}
impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let functions;
        let imports;
        Ok(MacroInput {
            name: input.parse()?,
            _punct_1: input.parse()?,
            _import_bracket: bracketed!(imports in input),
            imports: imports.parse_terminated(Ident::parse)?,
            _punct_2: input.parse()?,
            _function_bracket: bracketed!(functions in input),
            functions: functions.parse_terminated(ItemFn::parse)?,
        })
    }
}
impl MacroInput {
    pub(crate) fn generate_enum(&self) -> TokenStream {
        let name = &self.name;
        let imports = self.imports.iter().map(|ident| quote! {#ident(#ident)});
        let functions = self
            .functions
            .iter()
            .map(|item_fn| (&item_fn.sig.ident).into_token_stream());
        let variants = imports.chain(functions);
        quote! {
            #[allow(non_camel_case_types)]
            #[derive(Clone, Debug)]
            pub enum #name {
                #(#variants),*
            }
        }
    }
    pub(crate) fn generate_impl(&self) -> TokenStream {
        let name = &self.name;

        let namespace = name.to_string().to_ascii_lowercase();

        let function_arg_count = self
            .functions
            .iter()
            .map(|item_fn| item_fn.sig.inputs.len())
            .max()
            .unwrap_or(0);
        let import_arg_count = self.imports.iter().map(|ident| {
            let arg_count = quote! {#ident::MAX_ARGS};
            quote! {
                if #arg_count > max_count {
                    max_count = #arg_count;
                }
            }
        });
        let max_args = quote! {
            {
                let max_count = 0;
                #(#import_arg_count)*
                if max_count > #function_arg_count {
                    max_count
                } else {
                    #function_arg_count
                }
            }
        };

        let import_from_string = self.imports.iter().map(|ident| {
            quote! {
                #ident::NAMESPACE => #ident::from_string(rest, identifier),
            }
        });
        let function_from_string = self.functions.iter().map(|item_fn| {
            let name = item_fn.sig.ident.to_string().to_ascii_lowercase();
            let ident = &item_fn.sig.ident;
            quote! { #name => Ok(Self::#ident),}
        });
        let call = self.functions.iter().map(|item_fn| {
            let ident = &item_fn.sig.ident;
            let args = item_fn
                .sig
                .inputs
                .iter()
                .enumerate()
                .map(|(count, arg)| match arg {
                    FnArg::Typed(pat_type) => {
                        let ty = &pat_type.ty;
                        quote! { std::convert::TryInto::<#ty>::try_into(*args.get(#count).ok_or(Error::InvalidArg)?)?}
                    }
                    _ => panic!("functions taking self are not allowed"),
                });
            quote! {
                Self::#ident => {
                    #item_fn
                    #ident(#(#args),*)
                }
            }
        });
        quote! {
            impl Library<#name> for #name {
                const NAMESPACE: &'static str = #namespace;

                const MAX_ARGS: usize = #max_args;

                fn from_string(namespaces: &[&str], identifier: &str) -> Result<#name, Error> {
                    match namespaces {
                        [namespace] => match namespace {
                            &Self::NAMESPACE => Self::from_string(&[], identifier),
                            _ => Err(Error::InvalidNamespace),
                        }
                        [namespace, rest @ ..] => match namespace {
                            #(#import_from_string)*
                            _ => Err(Error::InvalidNamespace),
                        }
                        [] => match identifier {
                            #(#function_from_string)*
                            _ => Err(Error::UnknownFunction),
                        }
                    }
                }

                fn call(&self, args: &[Value]) -> Result<Value, Error> {
                    match self {
                        #(#call),*
                    }
                }

                fn is_const(&self) -> bool {
                    todo!()
                }
            }
        }
    }
}
