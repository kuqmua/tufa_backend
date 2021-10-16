use crate::get_project_information::get_config::get_lazy_config_information::CONFIG;

pub const ENV_FILE_NAME: &str = ".env";

pub const LOAD_CONFIG_FILE_ERROR_MESSAGE: &str = "cannot create config";

pub const LOGS_COMMON_FOLDER_NAME: &str = "common_folder";

pub const ARXIV_NAME_TO_CHECK: &str = "arxiv";
pub const BIORXIV_NAME_TO_CHECK: &str = "biorxiv";
pub const GITHUB_NAME_TO_CHECK: &str = "github";
pub const HABR_NAME_TO_CHECK: &str = "habr";
pub const MEDRXIV_NAME_TO_CHECK: &str = "medrxiv";
pub const REDDIT_NAME_TO_CHECK: &str = "reddit";
pub const TWITTER_NAME_TO_CHECK: &str = "twitter";

pub const COMMON_PROVIDER_ITEM_HANDLE: &str = "</item>";
pub const GITHUB_PROVIDER_ITEM_HANDLE: &str = "</entry>";

pub const TWITTER_FILTER_HANDLE_TO_REMOVE_1: &str = "<dc:creator>";
pub const TWITTER_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "bbb<creator>";
pub const TWITTER_FILTER_HANDLE_TO_REMOVE_2: &str = "</dc:creator>";
pub const TWITTER_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "bbb</creator>";
pub const TWITTER_FILTER_HANDLE_TO_REMOVE_3: &str = "<atom:link";
pub const TWITTER_FILTER_HANDLE_TO_REPLACE_REMOVED_3: &str = "<atomllink";

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

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn get_mongo_url() -> String {
    let mongo_first_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_first_handle_url_part;
        let mongo_login = &CONFIG.mongo_params.mongo_authorization.mongo_login;
        let mongo_second_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_second_handle_url_part;
        let mongo_password = &CONFIG.mongo_params.mongo_authorization.mongo_password;
        let mongo_third_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_third_handle_url_part;
        let mongo_ip = &CONFIG.mongo_params.mongo_authorization.mongo_ip;
        let mongo_fourth_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_fourth_handle_url_part;
        let mongo_port = &CONFIG.mongo_params.mongo_authorization.mongo_port;
        let mongo_fifth_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_fifth_handle_url_part;
        let mongo_params = &CONFIG.mongo_params.mongo_authorization.mongo_params;
        let mongo_url = format!(
            "{}{}{}{}{}{}{}{}{}{}",
            mongo_first_handle_url_part,
            mongo_login,
            mongo_second_handle_url_part,
            mongo_password,
            mongo_third_handle_url_part,
            mongo_ip,
            mongo_fourth_handle_url_part,
            mongo_port,
            mongo_fifth_handle_url_part,
            mongo_params
        );
    mongo_url
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
