extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
#[macro_use]
extern crate quote;

#[proc_macro_derive(Nothing)]
pub fn nothing_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn nothing(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro]
pub fn other_nothing(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(ATrait)]
pub fn trait_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    TokenStream::from(quote! {
        impl ATrait for #ident {
            fn do_something(&self) -> usize {
                0
            }
        }
    })
}

#[proc_macro_attribute]
pub fn inherit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: syn::Ident = parse_macro_input!(attr);
    let mut s: syn::ItemStruct = parse_macro_input!(item);
    if let syn::Fields::Named(ref mut fields) = s.fields {
        let mut elems: syn::punctuated::Punctuated<syn::Type, syn::token::Comma> = syn::punctuated::Punctuated::new();
        elems.push(usize_type());
        elems.push(usize_type());
        fields.named.push_value(syn::Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            ident: Some(syn::Ident::new("source_location", proc_macro2::Span::call_site())),
            colon_token: Some(syn::token::Colon {
                spans: [proc_macro2::Span::call_site()]
            }),
            ty: syn::Type::Tuple(syn::TypeTuple {
                paren_token: syn::token::Paren { span: proc_macro2::Span::call_site() },
                elems, 
            })
        })
    }
    let ident = s.ident.clone();
    let imp = quote! {
        #s
        impl #attr for #ident {
            fn source_location(&self) -> (usize, usize) {
                self.source_location
            }
        }
    };
    imp.into()
}

fn usize_type() -> syn::Type {
    let mut segments: syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2> = syn::punctuated::Punctuated::new();
    segments
        .push_value(syn::PathSegment {
            ident: syn::Ident::new("usize", proc_macro2::Span::call_site()),
            arguments: syn::PathArguments::None
        });
    syn::Type::Path(
        syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments,
            }
        }
    )
}

use syn::parse::{Parse, ParseStream, Result};
#[derive(Debug)]
struct Args {
    args: Vec<syn::Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut args = Vec::new();
        while let Ok(arg) = input.parse() {
            args.push(arg);
            if input.peek(syn::token::Comma) {
                let _: Result<syn::token::Comma> = input.parse();
            }
        }
        Ok(Self {
            args,
        })
}
}

#[proc_macro]
pub fn bench(input: TokenStream) -> TokenStream {
    let idents = parse_macro_input!(input as Args);
    let mut block = syn::Block {
        brace_token: syn::token::Brace {
            span: proc_macro2::Span::call_site()
        },
        stmts: Vec::new(),
    };
    for ident in idents.args {
        block.stmts.push(
            syn::Stmt::Item(syn::Item::Fn(
                syn::ItemFn {
                    attrs: vec![],
                    vis: syn::Visibility::Inherited,
                    constness: None,
                    unsafety: None,
                    abi: None,
                    ident: syn::Ident::new("inner_bench_fn", proc_macro2::Span::call_site()),
                    
                }
            ))
        )
    }
    TokenStream::new()
}