use convert_case::Case;
use convert_case::Casing;

use syn;
use syn::punctuated::Punctuated;
use syn::Token;
use syn::Variant;

use proc_macro::TokenStream;

use crate::syn::Data;
use crate::syn::Ident;

#[proc_macro_derive(SomeTrait)]
pub fn derive_provider_kind_from_config(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap(); //if need to print ast use syn = { version = "1.0.75", features = ["extra-traits"]} instead of syn="1.0.75"
    let ident: &Ident = &ast.ident;
    let data: Data = ast.data;
    let mut ident_name = String::from("");
    let mut vec_string = Vec::new();
    match data {
        Data::Struct(_) => panic!("no implementation for Struct"),
        Data::Union(_) => panic!("no implementation for Union"),
        Data::Enum(enum_handle) => {
            let variants: Punctuated<Variant, Token![,]> = enum_handle.variants;
            for i in variants {
                let vvv = i.ident;
                let formatted = format!("{:?}", vvv);
                let symbol = "ident: \"";
                match formatted.find(symbol) {
                    None => panic!("cannot find first symbol {}", symbol),
                    Some(start_index) => {
                        let cutted = formatted[start_index + symbol.len()..].to_owned();
                        let second_symbol = "\",";
                        match cutted.find(second_symbol) {
                            None => panic!("cannot find first symbol {}", second_symbol),
                            Some(end_index) => {
                                let needed_str = cutted[..end_index].to_owned();
                                let formatted_ident = format!("{:?}", ident);
                                match formatted_ident.find(symbol) {
                                    None => panic!("cannot find first symbol {}", symbol),
                                    Some(start_index_ident) => {
                                        let cutted = formatted_ident
                                            [start_index_ident + symbol.len()..]
                                            .to_owned();
                                        match cutted.find(second_symbol) {
                                            None => panic!("cannot find first symbol {}", second_symbol),
                                            Some(end_index_ident) => {
                                                let needed_ident =
                                                    cutted[..end_index_ident].to_owned();
                                                let lowered_snake_case = needed_str.to_case(Case::Snake).to_lowercase();
                                                let prepare = format!(
                                                    "{}::{} => CONFIG.{},",
                                                    needed_ident,
                                                    needed_str,
                                                    lowered_snake_case
                                                );
                                                ident_name = needed_ident;
                                                vec_string.push(prepare);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if ident_name == "" {
        panic!("ident_name is empty!");
    }
    if vec_string.is_empty() {
        panic!("vec_string is empty!");
    }
    let mut summary = String::from("");
    for i in vec_string {
        summary.push_str(&i);
    }
    let string_from = format!(
        "
    impl SomeTrait for {} {{
        fn is_something_enabled(&self)-> bool {{
            match self {{
               {}
            }}
         }}
    }}
    ",
        ident_name, summary
    );
    string_from.parse().unwrap()
}
