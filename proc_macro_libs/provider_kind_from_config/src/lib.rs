use crate::syn::Data;
use crate::syn::Ident;

use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn;
use syn::punctuated::Punctuated;
use syn::Token;
use syn::Variant;

#[proc_macro_derive(SomeTrait)]
pub fn derive_provider_kind_from_config(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();//if need to print ast use syn = { version = "1.0.75", features = ["extra-traits"]} instead of syn="1.0.75"
    let ident: &Ident = &ast.ident;
    let data: Data = ast.data;
    let mut ident_name = String::from("");
    let mut vec_string = Vec::new();
    let mut vec = Vec::new();
    match data {
        Data::Struct(_) => panic!("no implementation for Struct"),
        Data::Union(_) => panic!("no implementation for Union"),
        Data::Enum(enum_handle) => {
            let variants: Punctuated<Variant, Token![,]> = enum_handle.variants;
            for i in variants {
                let vvv = i.ident;
                // let name = vvv.ident;
                let formatted = format!("{:?}", vvv);
                let symbol = "ident: \"";
                match formatted.find(symbol) {
                    Some(start_index) => {
                            let cutted = formatted[start_index + symbol.len()..].to_owned();
                            let second_symbol = "\",";
                            match cutted.find(second_symbol) {
                                Some(end_index) => {
                                    let needed_str = cutted[..end_index].to_owned();
                                    //////
                                    let formatted_ident = format!("{:?}", ident);
                                    match formatted_ident.find(symbol) {
                                        Some(start_index_ident) => {
                                                let cutted = formatted_ident[start_index_ident + symbol.len()..].to_owned();
                                                match cutted.find(second_symbol) {
                                                    Some(end_index_ident) => {
                                                        let needed_ident = cutted[..end_index_ident].to_owned();
                                                        let prepare = format!("{}::{} => CONFIG.{},", needed_ident, needed_str, needed_str.to_lowercase());
                                                        ident_name = needed_ident;
                                                        let enum_variant_ident = format_ident!("{}", needed_str);
                                                        vec.push(quote! { #ident::#enum_variant_ident => CONFIG.#enum_variant_ident, });
                                                        vec_string.push(prepare);
                                                    },
                                                    None => panic!("cannot find first symbol {}", second_symbol),
                                                }
                                       
                                        }
                                        _ => panic!("cannot find first symbol {}", symbol)
                                    }
                                },
                                None => panic!("cannot find first symbol {}", second_symbol),
                            }
                   
                    }
                    _ => panic!("cannot find first symbol {}", symbol)
                }
                
            }
        }
    }
    if ident_name == "" {
        panic!("ident_name is empty!");
    }
    let mut summary = String::from("");
    for i in vec_string {
        summary.push_str(&i);
    }
    let string_from = format!("
    impl SomeTrait for {} {{
        fn is_something_enabled(&self)-> bool {{
            match self {{
               
               {}
            }}
         }}
    }}
    ", ident_name, summary);
    string_from.parse().unwrap()
}
