// use crate::syn::Ident;
// use proc_macro::TokenStream;
// use quote::quote;
// use syn;

// #[proc_macro_derive(ProviderKindFromConfigTrait)]
// pub fn derive_provider_kind_from_config(input: TokenStream) -> TokenStream {
//     let ast: syn::DeriveInput = syn::parse(input).unwrap();
//     let ident: &Ident = &ast.ident;
//     let config_name = syn::Ident::new("CONFIG", ident.span());
//     let mongo_enable_initialization_for_ =
//         syn::Ident::new("mongo_enable_initialization_for_", ident.span());
//     let test_arxiv = syn::Ident::new("arxiv", ident.span());
//     let gen = quote! {
//         impl ProviderKindFromConfigTrait for #ident {
//             fn is_mongo_initialization_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.mongo_enable_initialization_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.mongo_enable_initialization_for_biorxiv,
//                     ProviderKind::Github => #config_name.mongo_enable_initialization_for_github,
//                     ProviderKind::Medrxiv => #config_name.mongo_enable_initialization_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.mongo_enable_initialization_for_twitter,
//                     ProviderKind::Reddit => #config_name.mongo_enable_initialization_for_reddit,
//                     ProviderKind::Habr => #config_name.mongo_enable_initialization_for_habr,
//                 }
//             }
//         }
//     };
//     gen.into()
// }
use crate::syn::Attribute;
use crate::syn::Data;
use crate::syn::Generics;
use crate::syn::Ident;
use crate::syn::Visibility;

use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn;
use syn::punctuated::Punctuated;
use syn::token;
use syn::token::Token;
use syn::Fields;
use syn::Token;
use syn::Variant;

// DeriveInput {
//     attrs: [],
//     vis: Inherited,
//     ident: Ident {
//         ident: "Example",
//         span: #0 bytes(5738..5745),
//     },
//     generics: Generics {
//         lt_token: None,
//         params: [],
//         gt_token: None,
//         where_clause: None,
//     },
//     data: Enum(
//         DataEnum {
//             enum_token: Enum,
//             brace_token: Brace,
//             variants: [
//                 Variant {
//                     attrs: [],
//                     ident: Ident {
//                         ident: "One",
//                         span: #0 bytes(5752..5755),
//                     },
//                     fields: Unit,
//                     discriminant: None,
//                 },
//                 Comma,
//                 Variant {
//                     attrs: [],
//                     ident: Ident {
//                         ident: "Two",
//                         span: #0 bytes(5761..5764),
//                     },
//                     fields: Unit,
//                     discriminant: None,
//                 },
//                 Comma,
//             ],
//         },
//     ),
// }
#[proc_macro_derive(SomeTrait)]
pub fn derive_provider_kind_from_config(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    println!("rr {:#?}", ast);
    /// Name of the struct or enum.
    let ident: &Ident = &ast.ident;
    // let one_ident = syn::Ident::new("One", ident.span()); //use something else instead?
    // let something_for_ident = syn::Ident::new("something_for_", ident.span()); //use something else instead?
    let attrs: &Vec<Attribute> = &ast.attrs;

    /// Visibility of the struct or enum.
    let vis: &Visibility = &ast.vis;

    /// Generics required to complete the definition.
    let generics: &Generics = &ast.generics;

    /// Data within the struct or enum.
    let data: Data = ast.data;
    let mut ident_name = String::from("");
    let mut vec_string = Vec::new();
    let mut vec = Vec::new();
    match data {
        Data::Struct(struct_handle) => panic!("its not fo struct"),
        Data::Enum(enum_handle) => {
            // let struct_token: Token![struct] = enum_handle.struct_token;
            // let fields: Fields = enum_handle.fields;
            // let semi_token: Option<Token![;]> = enum_handle.semi_token;
            let enum_token: Token![enum] = enum_handle.enum_token;
            let brace_token: token::Brace = enum_handle.brace_token;
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
                                                        println!("needed_str {}", needed_str);
                                                        println!("needed_ident {}", needed_ident);
                                                        
                                                        let prepare = format!("{}::{} => CONFIG.{},", needed_ident, needed_str, needed_str.to_lowercase());
                                                        ident_name = needed_ident;
                                                        let enum_variant_ident = format_ident!("{}", needed_str);
                                                        println!("prepare {}", prepare);
                                                        // #ident::#one => CONFIG.mongo_enable_initialization_for_arxiv
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
            // variants.iter().forEach(|v| vec.push(v.clone()));
            // let first_variant[0] = 
        }
        Data::Union(union_handle) => panic!(""),
    }
    if ident_name == "" {
        panic!("ident_name is empty!");
    }
    ///////
    /// https://github.com/dtolnay/quote
//     Repetition
// Repetition is done using #(...)* or #(...),* similar to macro_rules!. This iterates through the elements of any variable interpolated within the repetition and inserts a copy of the repetition body for each one. The variables in an interpolation may be anything that implements IntoIterator, including Vec or a pre-existing iterator.

// #(#var)* — no separators
// #(#var),* — the character before the asterisk is used as a separator
// #( struct #var; )* — the repetition can contain other things
// #( #k => println!("{}", #v), )* — even multiple interpolations
// Note that there is a difference between #(#var ,)* and #(#var),*—the latter does not produce a trailing comma. This matches the behavior of delimiters in macro_rules!.
    ///////
    
    // let start_match = quote! {match self };
    // let fff = quote! { } };
    // let braket_start = token::Bracket;
    // let braket_end = token::Bracket;
    // syn::parse_str::<Expr>("Values::Unteger(val)");
    println!("vec_string, {:#?}", vec_string);
    let mut summary = String::from("");
    // let mut summary = String::from("match self {");
    for i in vec_string {
        summary.push_str(&i);
    }
    // summary.push_str("}");
    println!("summary {}", summary);
    // let summary_ident = syn::Ident::new(&summary, ident.span());
    
    // let one = quote! { mongo_enable_initialization_for_arxiv };
    // let two = quote! { mongo_enable_initialization_for_biorxiv };
    // #ident::#one => CONFIG.mongo_enable_initialization_for_arxiv,//error
    // #ident::#two => CONFIG.mongo_enable_initialization_for_biorxiv,
    // let vec = vec![fff, ddd];

    // let end = quote! {
    //     impl SomeTrait for #ident {
    //         fn is_something_enabled(&self)-> bool {
    //             match self {
                   
    //                Example::mongo_enable_initialization_for_arxiv => CONFIG.mongo_enable_initialization_for_arxiv,Example::mongo_enable_initialization_for_biorxiv => CONFIG.mongo_enable_initialization_for_biorxiv,
    //             }
    //          }
    //     }
    // };
    // end.into()
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
// fn is_mongo_write_error_logs_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.mongo_enable_write_error_logs_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.mongo_enable_write_error_logs_for_biorxiv,
//                     ProviderKind::Github => #config_name.mongo_enable_write_error_logs_for_github,
//                     ProviderKind::Medrxiv => #config_name.mongo_enable_write_error_logs_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.mongo_enable_write_error_logs_for_twitter,
//                     ProviderKind::Reddit => #config_name.mongo_enable_write_error_logs_for_reddit,
//                     ProviderKind::Habr => #config_name.mongo_enable_write_error_logs_for_habr,
//                 }
//             }
//             fn is_mongo_cleaning_warning_logs_db_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.mongo_enable_cleaning_warning_logs_db_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.mongo_enable_cleaning_warning_logs_db_for_biorxiv,
//                     ProviderKind::Github => #config_name.mongo_enable_cleaning_warning_logs_db_for_github,
//                     ProviderKind::Medrxiv => #config_name.mongo_enable_cleaning_warning_logs_db_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.mongo_enable_cleaning_warning_logs_db_for_twitter,
//                     ProviderKind::Reddit => #config_name.mongo_enable_cleaning_warning_logs_db_for_reddit,
//                     ProviderKind::Habr => #config_name.mongo_enable_cleaning_warning_logs_db_for_habr,
//                 }
//             }
//             fn is_mongo_cleaning_warning_logs_db_collections_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => {
//                 #config_name.mongo_enable_cleaning_warning_logs_db_collections_for_arxiv
//             }
//                     ProviderKind::Biorxiv => {
//                 #config_name.mongo_enable_cleaning_warning_logs_db_collections_for_biorxiv
//             }
//                     ProviderKind::Github => {
//                 #config_name.mongo_enable_cleaning_warning_logs_db_collections_for_github
//             }
//                     ProviderKind::Medrxiv => {
//                 #config_name.mongo_enable_cleaning_warning_logs_db_collections_for_medrxiv
//             }
//                     ProviderKind::Twitter => {
//                 #config_name.mongo_enable_cleaning_warning_logs_db_collections_for_twitter
//             }
//                     ProviderKind::Reddit => {
//                 #config_name.mongo_enable_cleaning_warning_logs_db_collections_for_reddit
//             }
//                     ProviderKind::Habr => #config_name.mongo_enable_cleaning_warning_logs_db_collections_for_habr,
//                 }
//             }
//             fn is_mongo_link_parts_randomize_order_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.mongo_enable_link_parts_randomize_order_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.mongo_enable_link_parts_randomize_order_for_biorxiv,
//                     ProviderKind::Github => #config_name.mongo_enable_link_parts_randomize_order_for_github,
//                     ProviderKind::Medrxiv => #config_name.mongo_enable_link_parts_randomize_order_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.mongo_enable_link_parts_randomize_order_for_twitter,
//                     ProviderKind::Reddit => #config_name.mongo_enable_link_parts_randomize_order_for_reddit,
//                     ProviderKind::Habr => #config_name.mongo_enable_link_parts_randomize_order_for_habr,
//                 }
//             }
//             fn is_postgres_initialization_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.postgres_enable_initialization_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.postgres_enable_initialization_for_biorxiv,
//                     ProviderKind::Github => #config_name.postgres_enable_initialization_for_github,
//                     ProviderKind::Medrxiv => #config_name.postgres_enable_initialization_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.postgres_enable_initialization_for_twitter,
//                     ProviderKind::Reddit => #config_name.postgres_enable_initialization_for_reddit,
//                     ProviderKind::Habr => #config_name.postgres_enable_initialization_for_habr,
//                 }
//             }
//             fn is_write_error_logs_in_local_folder_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.enable_write_error_logs_in_local_folder_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.enable_write_error_logs_in_local_folder_for_biorxiv,
//                     ProviderKind::Github => #config_name.enable_write_error_logs_in_local_folder_for_github,
//                     ProviderKind::Medrxiv => #config_name.enable_write_error_logs_in_local_folder_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.enable_write_error_logs_in_local_folder_for_twitter,
//                     ProviderKind::Reddit => #config_name.enable_write_error_logs_in_local_folder_for_reddit,
//                     ProviderKind::Habr => #config_name.enable_write_error_logs_in_local_folder_for_habr,
//                 }
//             }
//             fn is_cleaning_warning_logs_directory_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.enable_cleaning_warning_logs_directory_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.enable_cleaning_warning_logs_directory_for_biorxiv,
//                     ProviderKind::Github => #config_name.enable_cleaning_warning_logs_directory_for_github,
//                     ProviderKind::Medrxiv => #config_name.enable_cleaning_warning_logs_directory_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.enable_cleaning_warning_logs_directory_for_twitter,
//                     ProviderKind::Reddit => #config_name.enable_cleaning_warning_logs_directory_for_reddit,
//                     ProviderKind::Habr => #config_name.enable_cleaning_warning_logs_directory_for_habr,
//                 }
//             }
//             fn get_check_link(&self) -> &'static str {
//                 match self {
//                     ProviderKind::Arxiv => &#config_name.arxiv_check_link,
//                     ProviderKind::Biorxiv => &#config_name.biorxiv_check_link,
//                     ProviderKind::Github => &#config_name.github_check_link,
//                     ProviderKind::Medrxiv => &#config_name.medrxiv_check_link,
//                     ProviderKind::Twitter => &#config_name.twitter_check_link,
//                     ProviderKind::Reddit => &#config_name.reddit_check_link,
//                     ProviderKind::Habr => &#config_name.habr_check_link,
//                 }
//             }
//             fn is_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.enable_arxiv,
//                     ProviderKind::Biorxiv => #config_name.enable_biorxiv,
//                     ProviderKind::Github => #config_name.enable_github,
//                     ProviderKind::Medrxiv => #config_name.enable_medrxiv,
//                     ProviderKind::Twitter => #config_name.enable_twitter,
//                     ProviderKind::Reddit => #config_name.enable_reddit,
//                     ProviderKind::Habr => #config_name.enable_habr,
//                 }
//             }
//             fn is_prints_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.enable_prints_arxiv, //todo add for
//                     ProviderKind::Biorxiv => #config_name.enable_prints_biorxiv, //todo add for
//                     ProviderKind::Github => #config_name.enable_prints_github, //todo add for
//                     ProviderKind::Medrxiv => #config_name.enable_prints_medrxiv, //todo add for
//                     ProviderKind::Twitter => #config_name.enable_prints_twitter, //todo add for
//                     ProviderKind::Reddit => #config_name.enable_prints_reddit, //todo add for
//                     ProviderKind::Habr => #config_name.enable_prints_habr,   //todo add for
//                 }
//             }
//             fn is_warning_high_prints_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.enable_warning_high_prints_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.enable_warning_high_prints_for_biorxiv,
//                     ProviderKind::Github => #config_name.enable_warning_high_prints_for_github,
//                     ProviderKind::Medrxiv => #config_name.enable_warning_high_prints_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.enable_warning_high_prints_for_twitter,
//                     ProviderKind::Reddit => #config_name.enable_warning_high_prints_for_reddit,
//                     ProviderKind::Habr => #config_name.enable_warning_high_prints_for_habr,
//                 }
//             }
//             fn is_warning_low_prints_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.enable_warning_low_prints_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.enable_warning_low_prints_for_biorxiv,
//                     ProviderKind::Github => #config_name.enable_warning_low_prints_for_github,
//                     ProviderKind::Medrxiv => #config_name.enable_warning_low_prints_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.enable_warning_low_prints_for_twitter,
//                     ProviderKind::Reddit => #config_name.enable_warning_low_prints_for_reddit,
//                     ProviderKind::Habr => #config_name.enable_warning_low_prints_for_habr,
//                 }
//             }
//             fn is_success_prints_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.enable_success_prints_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.enable_success_prints_for_biorxiv,
//                     ProviderKind::Github => #config_name.enable_success_prints_for_github,
//                     ProviderKind::Medrxiv => #config_name.enable_success_prints_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.enable_success_prints_for_twitter,
//                     ProviderKind::Reddit => #config_name.enable_success_prints_for_reddit,
//                     ProviderKind::Habr => #config_name.enable_success_prints_for_habr,
//                 }
//             }
//             fn is_partial_success_prints_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.enable_partial_success_prints_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.enable_partial_success_prints_for_biorxiv,
//                     ProviderKind::Github => #config_name.enable_partial_success_prints_for_github,
//                     ProviderKind::Medrxiv => #config_name.enable_partial_success_prints_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.enable_partial_success_prints_for_twitter,
//                     ProviderKind::Reddit => #config_name.enable_partial_success_prints_for_reddit,
//                     ProviderKind::Habr => #config_name.enable_partial_success_prints_for_habr,
//                 }
//             }
//             fn is_error_prints_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.enable_error_prints_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.enable_error_prints_for_biorxiv,
//                     ProviderKind::Github => #config_name.enable_error_prints_for_github,
//                     ProviderKind::Medrxiv => #config_name.enable_error_prints_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.enable_error_prints_for_twitter,
//                     ProviderKind::Reddit => #config_name.enable_error_prints_for_reddit,
//                     ProviderKind::Habr => #config_name.enable_error_prints_for_habr,
//                 }
//             }
//             fn is_time_measurement_prints_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.enable_time_measurement_prints_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.enable_time_measurement_prints_for_biorxiv,
//                     ProviderKind::Github => #config_name.enable_time_measurement_prints_for_github,
//                     ProviderKind::Medrxiv => #config_name.enable_time_measurement_prints_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.enable_time_measurement_prints_for_twitter,
//                     ProviderKind::Reddit => #config_name.enable_time_measurement_prints_for_reddit,
//                     ProviderKind::Habr => #config_name.enable_time_measurement_prints_for_habr,
//                 }
//             }
//             fn is_info_prints_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.enable_info_prints_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.enable_info_prints_for_biorxiv,
//                     ProviderKind::Github => #config_name.enable_info_prints_for_github,
//                     ProviderKind::Medrxiv => #config_name.enable_info_prints_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.enable_info_prints_for_twitter,
//                     ProviderKind::Reddit => #config_name.enable_info_prints_for_reddit,
//                     ProviderKind::Habr => #config_name.enable_info_prints_for_habr,
//                 }
//             }
//             fn is_links_limit_enabled(&self) -> bool {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.enable_links_limit_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.enable_links_limit_for_biorxiv,
//                     ProviderKind::Github => #config_name.enable_links_limit_for_github,
//                     ProviderKind::Medrxiv => #config_name.enable_links_limit_for_medrxiv,
//                     ProviderKind::Twitter => #config_name.enable_links_limit_for_twitter,
//                     ProviderKind::Reddit => #config_name.enable_links_limit_for_reddit,
//                     ProviderKind::Habr => #config_name.enable_links_limit_for_habr,
//                 }
//             }
//             fn get_links_limit(&self) -> i64 {
//                 match self {
//                     ProviderKind::Arxiv => #config_name.links_limit_for_arxiv,
//                     ProviderKind::Biorxiv => #config_name.links_limit_for_biorxiv,
//                     ProviderKind::Github => #config_name.links_limit_for_github,
//                     ProviderKind::Habr => #config_name.links_limit_for_habr,
//                     ProviderKind::Medrxiv => #config_name.links_limit_for_medrxiv,
//                     ProviderKind::Reddit => #config_name.links_limit_for_reddit,
//                     ProviderKind::Twitter => #config_name.links_limit_for_twitter,
//                 }
//             }
