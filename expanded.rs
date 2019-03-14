#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
extern crate proc_macro;
use proc_macro::TokenStream;
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
pub mod decls {
    extern crate proc_macro;
    #[rustc_proc_macro_decls]
    pub static _DECLS: &[proc_macro::bridge::client::ProcMacro] = &[
        proc_macro::bridge::client::ProcMacro::custom_derive("Nothing", &[], crate::nothing_derive),
        proc_macro::bridge::client::ProcMacro::attr("nothing", crate::nothing),
        proc_macro::bridge::client::ProcMacro::bang("other_nothing", crate::other_nothing),
    ];
}
