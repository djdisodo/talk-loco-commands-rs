/*
 * Created on Tue Dec 01 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self};

#[proc_macro_derive(BsonData)]
pub fn bson_data(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_bson_data(&ast)
}

fn impl_bson_data(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl crate::BsonData<'_> for #name {} //for implementation check

        impl loco_protocol::Encode for #name {
            fn encode<U: std::io::Write>(&self, buffer: &mut U) {
                bson::ser::to_document(&self).unwrap().to_writer(buffer).unwrap();
            }
        }

        impl loco_protocol::Decode for #name {
            fn decode<U: std::io::Read>(&self, buffer: &mut U) -> Self {
                bson::de::from_document(bson::Document::from_reader(buffer).unwrap()).unwrap()
            }
        }
    };
    
    gen.into()
}