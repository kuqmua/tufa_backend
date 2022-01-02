use std::collections::HashMap;

use convert_case::Case;
use convert_case::Casing;

use quote::quote;
use syn;

use proc_macro::TokenStream;

use crate::syn::Data;
use crate::syn::Ident;

#[proc_macro_derive(SomeTrait)]
pub fn derive_provider_kind_from_config(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap(); //if need to print ast use syn = { version = "1.0.75", features = ["extra-traits"]} instead of syn="1.0.75"
    let ident: &Ident = &ast.ident;
    let data: Data = ast.data;
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
    let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = data {
        variants
    } else {
        unimplemented!();
    };
    let variants_for_quote = variants.iter().map(|f| {
        let variant_name = &f.ident;
        let config_field_name = syn::Ident::new(
            &variant_name.to_string().to_case(Case::Snake).to_lowercase(),
            variant_name.span(),
        );
        quote! {
            #ident::#variant_name => CONFIG.#config_field_name
        }
    });
    let generated = quote! {
        impl SomeTrait for #ident {
            fn is_something_enabled(&self)-> bool {
                match self {
                   #(#variants_for_quote,)*
                }
             }
        }
    };
    generated.into()
}
