use std::collections::HashMap;

use convert_case::Case;
use convert_case::Casing;

use quote::quote;
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
    let cloned_data = data.clone();
    let mut ident_name = String::from("");
    let mut vec_string = Vec::new();
    let mut vec_ident = Vec::new();
    let mut function_hashmap = HashMap::from([
        ("is_mongo_initialization_enabled", "bool"),
        ("is_mongo_write_error_logs_enabled", "bool"),
        ("is_mongo_cleaning_warning_logs_db_enabled", "bool"),
        (
            "is_mongo_cleaning_warning_logs_db_collections_enabled",
            "bool",
        ),
        ("is_mongo_link_parts_randomize_order_enabled", "bool"),
        ("is_postgres_initialization_enabled", "bool"),
        ("is_write_error_logs_in_local_folder_enabled", "bool"),
        ("is_cleaning_warning_logs_directory_enabled", "bool"),
        ("check_link", "&'static str"),
        ("is_enabled", "bool"),
        ("is_prints_enabled", "bool"),
        ("is_warning_high_prints_enabled", "bool"),
        ("is_warning_low_prints_enabled", "bool"),
        ("is_success_prints_enabled", "bool"),
        ("is_partial_success_prints_enabled", "bool"),
        ("is_error_prints_enabled", "bool"),
        ("is_time_measurement_prints_enabled", "bool"),
        ("is_info_prints_enabled", "bool"),
        ("is_links_limit_enabled", "bool"),
        ("links_limit", "i64"),
    ]);
    let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = cloned_data {
        variants
    } else {
        unimplemented!();
    };

    // let variants_for_quote = variants.iter().map(|f| {
    //     let variant_name = &f.ident;
    //     quote! {

    //     }
    // }
    //  );
    let variants: Punctuated<Variant, Token![,]> = variants;
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
                                let cutted =
                                    formatted_ident[start_index_ident + symbol.len()..].to_owned();
                                match cutted.find(second_symbol) {
                                    None => panic!("cannot find first symbol {}", second_symbol),
                                    Some(end_index_ident) => {
                                        let needed_ident = cutted[..end_index_ident].to_owned();
                                        let lowered_snake_case =
                                            needed_str.to_case(Case::Snake).to_lowercase();
                                        let prepare = format!(
                                            "{}::{} => CONFIG.{},",
                                            needed_ident, needed_str, lowered_snake_case
                                        );
                                        let ggg =
                                            syn::Ident::new(&lowered_snake_case, ident.span());
                                        let qoute_handle = quote! {
                                            #ident::#vvv => CONFIG.#ggg
                                        };
                                        ident_name = needed_ident;
                                        vec_string.push(prepare);
                                        vec_ident.push(qoute_handle);
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
    //     let string_from = quote! {
    //     impl SomeTrait for #ident {
    //         fn is_something_enabled(&self)-> bool {
    //             match self {
    //                #(#vec_ident,)
    //             }
    //          }
    //     }
    // };
    // let string_from = quote! {
    //     impl SomeTrait for #ident {
    //         fn is_something_enabled(&self)-> bool {
    //             match self {
    //                #(#vec_ident,)
    //             }
    //          }
    //     }
    // };

    string_from.parse().unwrap()
}

// fn generate_function(indent_name: String, function: String) -> Str{
//     let mut string_to_return = String::from("
//     impl SomeTrait for {} {{
//         {}
//     }}
//     ", ident_name, function);
// }
