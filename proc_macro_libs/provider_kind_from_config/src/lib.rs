use crate::syn::Ident;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ProviderKindFromConfigTrait)]
pub fn derive_provider_kind_from_config(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let ident: &Ident = &ast.ident;
    let gen = quote! {
        impl ProviderKindFromConfigTrait for #ident {
            fn is_mongo_initialization_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.mongo_enable_initialization_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.mongo_enable_initialization_for_biorxiv,
                    ProviderKind::Github => CONFIG.mongo_enable_initialization_for_github,
                    ProviderKind::Medrxiv => CONFIG.mongo_enable_initialization_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.mongo_enable_initialization_for_twitter,
                    ProviderKind::Reddit => CONFIG.mongo_enable_initialization_for_reddit,
                    ProviderKind::Habr => CONFIG.mongo_enable_initialization_for_habr,
                }
            }
            fn is_mongo_write_error_logs_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.mongo_enable_write_error_logs_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.mongo_enable_write_error_logs_for_biorxiv,
                    ProviderKind::Github => CONFIG.mongo_enable_write_error_logs_for_github,
                    ProviderKind::Medrxiv => CONFIG.mongo_enable_write_error_logs_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.mongo_enable_write_error_logs_for_twitter,
                    ProviderKind::Reddit => CONFIG.mongo_enable_write_error_logs_for_reddit,
                    ProviderKind::Habr => CONFIG.mongo_enable_write_error_logs_for_habr,
                }
            }
            fn is_mongo_cleaning_warning_logs_db_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.mongo_enable_cleaning_warning_logs_db_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.mongo_enable_cleaning_warning_logs_db_for_biorxiv,
                    ProviderKind::Github => CONFIG.mongo_enable_cleaning_warning_logs_db_for_github,
                    ProviderKind::Medrxiv => CONFIG.mongo_enable_cleaning_warning_logs_db_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.mongo_enable_cleaning_warning_logs_db_for_twitter,
                    ProviderKind::Reddit => CONFIG.mongo_enable_cleaning_warning_logs_db_for_reddit,
                    ProviderKind::Habr => CONFIG.mongo_enable_cleaning_warning_logs_db_for_habr,
                }
            }
            fn is_mongo_cleaning_warning_logs_db_collections_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_arxiv
            }
                    ProviderKind::Biorxiv => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_biorxiv
            }
                    ProviderKind::Github => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_github
            }
                    ProviderKind::Medrxiv => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_medrxiv
            }
                    ProviderKind::Twitter => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_twitter
            }
                    ProviderKind::Reddit => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_reddit
            }
                    ProviderKind::Habr => CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_habr,
                }
            }
            fn is_mongo_link_parts_randomize_order_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.mongo_enable_link_parts_randomize_order_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.mongo_enable_link_parts_randomize_order_for_biorxiv,
                    ProviderKind::Github => CONFIG.mongo_enable_link_parts_randomize_order_for_github,
                    ProviderKind::Medrxiv => CONFIG.mongo_enable_link_parts_randomize_order_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.mongo_enable_link_parts_randomize_order_for_twitter,
                    ProviderKind::Reddit => CONFIG.mongo_enable_link_parts_randomize_order_for_reddit,
                    ProviderKind::Habr => CONFIG.mongo_enable_link_parts_randomize_order_for_habr,
                }
            }
            fn is_postgres_initialization_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.postgres_enable_initialization_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.postgres_enable_initialization_for_biorxiv,
                    ProviderKind::Github => CONFIG.postgres_enable_initialization_for_github,
                    ProviderKind::Medrxiv => CONFIG.postgres_enable_initialization_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.postgres_enable_initialization_for_twitter,
                    ProviderKind::Reddit => CONFIG.postgres_enable_initialization_for_reddit,
                    ProviderKind::Habr => CONFIG.postgres_enable_initialization_for_habr,
                }
            }
            fn is_write_error_logs_in_local_folder_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.enable_write_error_logs_in_local_folder_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.enable_write_error_logs_in_local_folder_for_biorxiv,
                    ProviderKind::Github => CONFIG.enable_write_error_logs_in_local_folder_for_github,
                    ProviderKind::Medrxiv => CONFIG.enable_write_error_logs_in_local_folder_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.enable_write_error_logs_in_local_folder_for_twitter,
                    ProviderKind::Reddit => CONFIG.enable_write_error_logs_in_local_folder_for_reddit,
                    ProviderKind::Habr => CONFIG.enable_write_error_logs_in_local_folder_for_habr,
                }
            }
            fn is_cleaning_warning_logs_directory_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.enable_cleaning_warning_logs_directory_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.enable_cleaning_warning_logs_directory_for_biorxiv,
                    ProviderKind::Github => CONFIG.enable_cleaning_warning_logs_directory_for_github,
                    ProviderKind::Medrxiv => CONFIG.enable_cleaning_warning_logs_directory_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.enable_cleaning_warning_logs_directory_for_twitter,
                    ProviderKind::Reddit => CONFIG.enable_cleaning_warning_logs_directory_for_reddit,
                    ProviderKind::Habr => CONFIG.enable_cleaning_warning_logs_directory_for_habr,
                }
            }
            fn get_check_link(&self) -> &'static str {
                match self {
                    ProviderKind::Arxiv => &CONFIG.arxiv_check_link,
                    ProviderKind::Biorxiv => &CONFIG.biorxiv_check_link,
                    ProviderKind::Github => &CONFIG.github_check_link,
                    ProviderKind::Medrxiv => &CONFIG.medrxiv_check_link,
                    ProviderKind::Twitter => &CONFIG.twitter_check_link,
                    ProviderKind::Reddit => &CONFIG.reddit_check_link,
                    ProviderKind::Habr => &CONFIG.habr_check_link,
                }
            }
            fn is_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.enable_arxiv,
                    ProviderKind::Biorxiv => CONFIG.enable_biorxiv,
                    ProviderKind::Github => CONFIG.enable_github,
                    ProviderKind::Medrxiv => CONFIG.enable_medrxiv,
                    ProviderKind::Twitter => CONFIG.enable_twitter,
                    ProviderKind::Reddit => CONFIG.enable_reddit,
                    ProviderKind::Habr => CONFIG.enable_habr,
                }
            }
            fn is_prints_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.enable_prints_arxiv, //todo add for
                    ProviderKind::Biorxiv => CONFIG.enable_prints_biorxiv, //todo add for
                    ProviderKind::Github => CONFIG.enable_prints_github, //todo add for
                    ProviderKind::Medrxiv => CONFIG.enable_prints_medrxiv, //todo add for
                    ProviderKind::Twitter => CONFIG.enable_prints_twitter, //todo add for
                    ProviderKind::Reddit => CONFIG.enable_prints_reddit, //todo add for
                    ProviderKind::Habr => CONFIG.enable_prints_habr,   //todo add for
                }
            }
            fn is_warning_high_prints_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.enable_warning_high_prints_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.enable_warning_high_prints_for_biorxiv,
                    ProviderKind::Github => CONFIG.enable_warning_high_prints_for_github,
                    ProviderKind::Medrxiv => CONFIG.enable_warning_high_prints_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.enable_warning_high_prints_for_twitter,
                    ProviderKind::Reddit => CONFIG.enable_warning_high_prints_for_reddit,
                    ProviderKind::Habr => CONFIG.enable_warning_high_prints_for_habr,
                }
            }
            fn is_warning_low_prints_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.enable_warning_low_prints_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.enable_warning_low_prints_for_biorxiv,
                    ProviderKind::Github => CONFIG.enable_warning_low_prints_for_github,
                    ProviderKind::Medrxiv => CONFIG.enable_warning_low_prints_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.enable_warning_low_prints_for_twitter,
                    ProviderKind::Reddit => CONFIG.enable_warning_low_prints_for_reddit,
                    ProviderKind::Habr => CONFIG.enable_warning_low_prints_for_habr,
                }
            }
            fn is_success_prints_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.enable_success_prints_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.enable_success_prints_for_biorxiv,
                    ProviderKind::Github => CONFIG.enable_success_prints_for_github,
                    ProviderKind::Medrxiv => CONFIG.enable_success_prints_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.enable_success_prints_for_twitter,
                    ProviderKind::Reddit => CONFIG.enable_success_prints_for_reddit,
                    ProviderKind::Habr => CONFIG.enable_success_prints_for_habr,
                }
            }
            fn is_partial_success_prints_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.enable_partial_success_prints_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.enable_partial_success_prints_for_biorxiv,
                    ProviderKind::Github => CONFIG.enable_partial_success_prints_for_github,
                    ProviderKind::Medrxiv => CONFIG.enable_partial_success_prints_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.enable_partial_success_prints_for_twitter,
                    ProviderKind::Reddit => CONFIG.enable_partial_success_prints_for_reddit,
                    ProviderKind::Habr => CONFIG.enable_partial_success_prints_for_habr,
                }
            }
            fn is_error_prints_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.enable_error_prints_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.enable_error_prints_for_biorxiv,
                    ProviderKind::Github => CONFIG.enable_error_prints_for_github,
                    ProviderKind::Medrxiv => CONFIG.enable_error_prints_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.enable_error_prints_for_twitter,
                    ProviderKind::Reddit => CONFIG.enable_error_prints_for_reddit,
                    ProviderKind::Habr => CONFIG.enable_error_prints_for_habr,
                }
            }
            fn is_time_measurement_prints_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.enable_time_measurement_prints_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.enable_time_measurement_prints_for_biorxiv,
                    ProviderKind::Github => CONFIG.enable_time_measurement_prints_for_github,
                    ProviderKind::Medrxiv => CONFIG.enable_time_measurement_prints_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.enable_time_measurement_prints_for_twitter,
                    ProviderKind::Reddit => CONFIG.enable_time_measurement_prints_for_reddit,
                    ProviderKind::Habr => CONFIG.enable_time_measurement_prints_for_habr,
                }
            }
            fn is_info_prints_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.enable_info_prints_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.enable_info_prints_for_biorxiv,
                    ProviderKind::Github => CONFIG.enable_info_prints_for_github,
                    ProviderKind::Medrxiv => CONFIG.enable_info_prints_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.enable_info_prints_for_twitter,
                    ProviderKind::Reddit => CONFIG.enable_info_prints_for_reddit,
                    ProviderKind::Habr => CONFIG.enable_info_prints_for_habr,
                }
            }
            fn is_links_limit_enabled(&self) -> bool {
                match self {
                    ProviderKind::Arxiv => CONFIG.enable_links_limit_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.enable_links_limit_for_biorxiv,
                    ProviderKind::Github => CONFIG.enable_links_limit_for_github,
                    ProviderKind::Medrxiv => CONFIG.enable_links_limit_for_medrxiv,
                    ProviderKind::Twitter => CONFIG.enable_links_limit_for_twitter,
                    ProviderKind::Reddit => CONFIG.enable_links_limit_for_reddit,
                    ProviderKind::Habr => CONFIG.enable_links_limit_for_habr,
                }
            }
            fn get_links_limit(&self) -> i64 {
                match self {
                    ProviderKind::Arxiv => CONFIG.links_limit_for_arxiv,
                    ProviderKind::Biorxiv => CONFIG.links_limit_for_biorxiv,
                    ProviderKind::Github => CONFIG.links_limit_for_github,
                    ProviderKind::Habr => CONFIG.links_limit_for_habr,
                    ProviderKind::Medrxiv => CONFIG.links_limit_for_medrxiv,
                    ProviderKind::Reddit => CONFIG.links_limit_for_reddit,
                    ProviderKind::Twitter => CONFIG.links_limit_for_twitter,
                }
            }
        }
    };
    gen.into()
}
