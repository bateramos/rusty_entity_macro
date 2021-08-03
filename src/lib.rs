//! # RustyEntity
//!
//! Macro to facilitate the implementation of [Exercise](https://github.com/bateramos/rusty_types/blob/master/src/lib.rs) Structs.
//! For the struct field is available the properties: description, expected_result, expected_results and sort.
//!
//! # Example
//! ```
//! use rusty_entity_macro::RustyEntity;
//!
//! #[derive(RustyEntity)]
//! struct PrepositionExercise {
//!     #[entity(description)]
//!     prepo: String,
//!     #[entity(expected_result)]
//!     case: String,
//! }
//! ```
//!
//! This will generate a implementation as the following:
//! ```
//! use rusty_types::Exercise;
//!
//! struct PrepositionExercise {
//!     prepo: String,
//!     case: String,
//! }
//! impl Exercise for PrepositionExercise {
//!     fn get_description(&self) -> String {
//!         self.prepo.to_owned()
//!     }
//!
//!     fn get_expected_result(&self) -> String {
//!         self.case.to_owned()
//!     }
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use quote::quote;

#[proc_macro_derive(RustyEntity, attributes(entity))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let getters : Vec<(&str,syn::Ident)> = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed {ref named, .. } ), .. }) = ast.data {
        named.clone().into_iter().fold(Vec::new(), |mut acc, f| {
            // Necessary as on Struct field can have more than one attribute
            f.attrs.clone().iter().for_each(|attr| {
                if attr.path.is_ident("entity") {
                    let meta : syn::Meta = attr.parse_meta().unwrap();
                    if let syn::Meta::List(syn::MetaList { nested, .. }) = meta.clone() {
                        if let syn::NestedMeta::Meta(syn::Meta::Path(ref path)) = &nested[0] {
                            let ident = path.segments[0].ident.clone();
                            let typ = format!("{}", ident);

                            if typ == "description" {
                                acc.push(("description", f.ident.clone().unwrap()))
                            } else if typ == "expected_result" {
                                acc.push(("expected_result", f.ident.clone().unwrap()))
                            } else if typ == "expected_results" {
                                acc.push(("expected_results", f.ident.clone().unwrap()))
                            } else if typ == "sort" {
                                acc.push(("sort_property", f.ident.clone().unwrap()))
                            }
                        } else {
                            panic!("Something wrong here");
                        }
                    } else {
                        panic!("Something wrong here");
                    }
                }
            });
            acc
        })
    } else {
        panic!("Something wrong here");
    };

    // Find duplicated usage of the same property
    {
        let duplicated_getters_property = getters.iter().try_fold(std::collections::HashSet::new(), |mut acc, (typ, ident)| {
            if acc.contains(typ) {
                return Err(syn::Error::new_spanned(
                    ident, format!("expected unique usage for `entity({})`", typ)
                ));
            } else {
                acc.insert(typ);
            }

            Ok(acc)
        });

        if let Err(error) = duplicated_getters_property {
            return error.to_compile_error().into();
        }
    }

    let implemented_getters = getters.iter().map(|(typ, ident)| {
        let fn_ident = syn::Ident::new(&format!("get_{}", &typ), ident.clone().span());
        if *typ == "expected_results" {
            quote! {
                fn #fn_ident(&self) -> Vec<String> {
                    self.#ident.clone()
                }
            }
        } else {
            quote! {
                fn #fn_ident(&self) -> String {
                    self.#ident.to_owned()
                }
            }
        }
    });

    (quote! {
        impl rusty_types::Exercise for #name {
            #(#implemented_getters)*
        }
    }).into()
}
