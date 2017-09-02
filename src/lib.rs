#![recursion_limit = "128"]
//! Models Derive
//!
//! Models derive provides some additional functionality for the diesel
//! models which make it easier to get started


#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate syn;

mod default_insertable;
mod db_enum;

use proc_macro::TokenStream;

use default_insertable::impl_default_insertable;
use db_enum::impl_db_enum;
use quote::Tokens;
use syn::MacroInput;


#[proc_macro_derive(DefaultInsertable, attributes(auto_increment, table_name))]
pub fn default_insertable(input: TokenStream) -> TokenStream {
    expand(input, impl_default_insertable)
}

#[proc_macro_derive(DBEnum)]
pub fn db_enum(input: TokenStream) -> TokenStream {
    expand(input, impl_db_enum)
}

fn expand(input: TokenStream, func: fn(&MacroInput) -> Tokens) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = func(&ast);
    gen.parse().unwrap()
}
