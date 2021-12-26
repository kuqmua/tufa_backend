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
use quote::quote;
use syn;
#[proc_macro_derive(SomeTrait)]
pub fn derive_provider_kind_from_config(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
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
    let data: &Data = &ast.data;
    let gen = quote! {
        impl SomeTrait for #ident {
            fn is_something_enabled(&self, config: Config) -> bool {
                match self {
                    #ident::One => config.something_for_one,//error
                    #ident::Two => config.something_for_two,
                }
            }
        }
    };
    gen.into()
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
