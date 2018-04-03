#![recursion_limit = "128"]
//! # Diesel Derive More
//!
//! Additional derive functionality for the diesel
//! models which make it easier to get started
//!
//! ## DBEnum
//!
//! DBEnum provides diesel implementations for using an enum as a string field
//! in models. Deriving DBEnum in an enum provides serializationa and
//! deserialization traits.
//!
//! ## DefaultInsertable
//!
//! Default insertable provides a default implementation for a diesel model
//! It allows marking fields as auto_increment so that they are excluded from
//! the derived struct.
//!
//! The new struct is prefixed with 'New' and lives in the same module as the
//! original struct.
//!
//! When using the serialization feature, the stuct is also serializable through
//! serde
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
