use quote::Tokens;
use syn::{DeriveInput, Ident, Field, Attribute, DataStruct};
use syn::punctuated::Pair::End;
use syn::Data::Struct;


trait WithAttributes {
    fn get_attributes(&self) -> &Vec<Attribute>;

    fn has_attribute(&self, attribute_name: &str) -> bool {
        match self.get_attribute_by_name(attribute_name) {
            Some(_) => true,
            None => false,
        }
    }

    fn get_attribute_by_name(&self, attribute_name: &str) -> Option<&Attribute> {
        let expected_ident = Ident::from(attribute_name);
        for attr in self.get_attributes() {
            if let Some(End(segment)) = attr.path.segments.first() {
                if segment.ident == expected_ident {
                    return Some(attr);
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


impl WithAttributes for DeriveInput {
    fn get_attributes(&self) -> &Vec<Attribute> {
        &self.attrs
    }
}


pub fn impl_default_insertable(ast: &DeriveInput) -> Tokens {
    let name = Ident::from(String::from("New") + &ast.ident.as_ref());
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let table_name_attribute = match ast.get_attribute_by_name("table_name") {
        Some(attribute) => attribute,
        _ => panic!("Struct must be annotated with table_name"),
    };

    match ast.data {
        Struct(DataStruct { ref fields, .. }) => {
            let mut args = vec![];

            for field in fields.iter() {
                if !field.has_attribute("auto_increment") {
                    args.push(quote!(#field))
                }
            }

            quote!(
                #[derive(Clone, Insertable)]
                #[cfg_attr(feature="serialization", derive(Deserialize))]
                #table_name_attribute
                pub struct #impl_generics #name #ty_generics #where_clause {
                    #(#args),*
                }
            )
        }
        _ => panic!("#[derive(DefaultInsertable)] can only be used with structs"),
    }
}
