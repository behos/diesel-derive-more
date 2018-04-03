#![recursion_limit = "128"]
//! # Diesel Derive More
//!
//! Additional derive functionality for the diesel
//! models which make it easier to get started
//!
//! ## DBEnum
//!
//! Represents an enum type as strings in the database. This allows the type to
//! be used in all cases. Check the db_enum module docs for additional information
//!
//! ## DefaultInsertable
//!
//! Creates a new struct which can be used for insertions.
//! Check the default_inserable module docs for additional information
//!
//! ## Examples
//!
//! For examples of usage check the corresponding tests in the tests/ dir


#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

mod default_insertable;
mod db_enum;

use proc_macro::TokenStream;

use default_insertable::impl_default_insertable;
use db_enum::impl_db_enum;
use quote::Tokens;
use syn::DeriveInput;


#[proc_macro_derive(DefaultInsertable, attributes(auto_increment, table_name))]
pub fn default_insertable(input: TokenStream) -> TokenStream {
    expand(input, impl_default_insertable)
}

#[proc_macro_derive(DBEnum)]
pub fn db_enum(input: TokenStream) -> TokenStream {
    expand(input, impl_db_enum)
}

fn expand(input: TokenStream, func: fn(&DeriveInput) -> Tokens) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let gen = func(&ast);
    TokenStream::from(gen)
}
