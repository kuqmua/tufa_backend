use std::collections::HashMap;

use crate::get_project_information::get_config::get_config_information::CONFIG;
use crate::get_project_information::provider_kind_enum::ProviderKind;

pub const LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE: &str = "сan not load user_credentials file";
pub const LOAD_CONFIG_FILE_ERROR_MESSAGE: &str = "сan not load config file";
pub const PATH_TO_CONFIG: &str = "./config/";
pub const PATH_TO_CONFIG_FOR_TEST: &str = "../../config/";
pub const PROJECT_MODE: &str = "Development"; //later as ENV variable only
pub const USER_CREDENTIALS_FILE_NAME: &str = "User_credentials";

pub const ARXIV_NAME_TO_CHECK: &str = "arxiv";
pub const BIORXIV_NAME_TO_CHECK: &str = "biorxiv";
pub const GITHUB_NAME_TO_CHECK: &str = "github";
pub const HABR_NAME_TO_CHECK: &str = "habr";
pub const MEDRXIV_NAME_TO_CHECK: &str = "medrxiv";
pub const REDDIT_NAME_TO_CHECK: &str = "reddit";
pub const TWITTER_NAME_TO_CHECK: &str = "twitter";

pub const COMMON_PROVIDER_ITEM_HANDLE: &str = "</item>";
pub const GITHUB_PROVIDER_ITEM_HANDLE: &str = "</entry>";

pub const FIRST_TWITTER_FILTER_HANDLE_TO_REMOVE: &str = "<channel>";
pub const LAST_TWITTER_FILTER_HANDLE_TO_REMOVE: &str = "</channel>";

pub const TWITTER_FILTER_HANDLE_TO_REMOVE_1: &str = "<dc:creator>";
pub const TWITTER_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "bbb<creator>";
pub const TWITTER_FILTER_HANDLE_TO_REMOVE_2: &str = "</dc:creator>";
pub const TWITTER_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "bbb</creator>";
pub const TWITTER_FILTER_HANDLE_TO_REMOVE_3: &str = "<atom:link";
pub const TWITTER_FILTER_HANDLE_TO_REPLACE_REMOVED_3: &str = "<atom:link";

pub const MEDRXIV_FILTER_HANDLE_TO_REMOVE_1: &str = "<dc:title>";
pub const MEDRXIV_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "<dccfifle>";
pub const MEDRXIV_FILTER_HANDLE_TO_REMOVE_2: &str = "</dc:title>";
pub const MEDRXIV_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "</dccfifle>";

pub const BIORXIV_FILTER_HANDLE_TO_REMOVE_1: &str = "<dc:title>";
pub const BIORXIV_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "<dcstitle>";
pub const BIORXIV_FILTER_HANDLE_TO_REMOVE_2: &str = "</dc:title>";
pub const BIORXIV_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "</dcstitle>";

pub const HABR_FILTER_HANDLE_TO_REMOVE_1: &str = "<channel>";
pub const HABR_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "         ";
pub const HABR_FILTER_HANDLE_TO_REMOVE_2: &str = "</channel>";
pub const HABR_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "         ";

pub const I64_MAX: i64 = 9_223_372_036_854_775_807;
pub const U64_FOR_I64_MAX_CAST: u64 = 9_223_372_036_854_775_807;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn get_config_provider_string_to_enum_struct() -> HashMap<String, ProviderKind> {
    let mut config_provider_string_to_enum_struct_hasmap: HashMap<String, ProviderKind> =
        HashMap::with_capacity(CONFIG.params.vec_of_provider_names.len());
    config_provider_string_to_enum_struct_hasmap
        .insert(ARXIV_NAME_TO_CHECK.to_string(), ProviderKind::Arxiv);
    config_provider_string_to_enum_struct_hasmap
        .insert(BIORXIV_NAME_TO_CHECK.to_string(), ProviderKind::Biorxiv);
    config_provider_string_to_enum_struct_hasmap
        .insert(GITHUB_NAME_TO_CHECK.to_string(), ProviderKind::Github);
    config_provider_string_to_enum_struct_hasmap
        .insert(HABR_NAME_TO_CHECK.to_string(), ProviderKind::Habr);
    config_provider_string_to_enum_struct_hasmap
        .insert(MEDRXIV_NAME_TO_CHECK.to_string(), ProviderKind::Medrxiv);
    config_provider_string_to_enum_struct_hasmap
        .insert(REDDIT_NAME_TO_CHECK.to_string(), ProviderKind::Reddit);
    config_provider_string_to_enum_struct_hasmap
        .insert(TWITTER_NAME_TO_CHECK.to_string(), ProviderKind::Twitter);
    config_provider_string_to_enum_struct_hasmap
}

//     config_provider_string_to_enum_struct_hasmap

// pub struct ConfigProviderStringToEnumTypeStruct {
//     pub config_name_value: &'static str,
//     pub provider_kind_enum_type: ProviderKind,
// }

// impl ConfigProviderStringToEnumTypeStruct {
//     pub const fn new(
//         config_name_value: &'static str,
//         provider_kind_enum_type: ProviderKind,
//     ) -> Self {
//         ConfigProviderStringToEnumTypeStruct {
//             config_name_value,
//             provider_kind_enum_type,
//         }
//     }
// }
