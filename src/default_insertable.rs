//! Default insertable provides a default implementation for a diesel model
//! It allows marking fields as auto_increment so that they are excluded from
//! the derived struct.
//!
//! The new struct is prefixed with 'New' and lives in the same module as the
//! original struct.

use quote::Tokens;
use syn::{VariantData, MacroInput, Ident, Field, Attribute, Lit};
use syn::Body::Struct;
use syn::MetaItem::{Word, List, NameValue};


trait WithAttributes {
    fn get_attributes(&self) -> &Vec<Attribute>;

    fn has_attribute(&self, attribute_name: &str) -> bool {
        match self.get_attribute_by_name(attribute_name) {
            Some(_) => true,
            None => false
        }
    }

    fn get_attribute_value(&self, attribute_name: &str) -> Option<&Lit> {
        match self.get_attribute_by_name(attribute_name) {
            Some(ref attribute) => {
                match attribute.value {
                    NameValue(_, ref value) => Some(value),
                    _ => None
                }      
            },
            None => None
        }
    }

    fn get_attribute_by_name(&self, attribute_name: &str) -> Option<&Attribute> {
        let expected_ident = Ident::from(attribute_name);
        for attr in self.get_attributes() {
            match attr.value {
                Word(ref ident) => {
                    if ident == &expected_ident {
                        return Some(attr);
                    }
                },
                List(ref ident, _) => {
                    if ident == &expected_ident {
                        return Some(attr);
                    }
                },
                NameValue(ref ident, _) => {
                    if ident == &expected_ident {
                        return Some(attr);
                    }
                }                    
            }
        }
        None
    }
}

impl WithAttributes for Field {

    fn get_attributes(&self) -> &Vec<Attribute> {
        &self.attrs
    }
}


impl WithAttributes for MacroInput {

    fn get_attributes(&self) -> &Vec<Attribute> {
        &self.attrs
    }
}


pub fn impl_default_insertable(ast: &MacroInput) -> Tokens {
    let name = Ident::from(String::from("New") + &ast.ident.as_ref());    
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let table_name_attribute = match ast.get_attribute_by_name("table_name") {
        Some(attribute) => attribute,
        _ => panic!("Struct must be annotated with table_name")
    };

    match ast.body {
        Struct(VariantData::Struct(ref fields)) => {
            let mut args = vec![];

            for field in fields {
                if !field.has_attribute("auto_increment") {
                    args.push(quote!(#field))
                }
            }

            quote!(
                #[derive(Clone, Insertable, Deserialize)]
                #table_name_attribute
                pub struct #impl_generics #name #ty_generics #where_clause {
                    #(#args),*
                }
            )
        },
        _ => panic!("#[derive(DefaultInsertable)] can only be used with structs"),
    }
}
