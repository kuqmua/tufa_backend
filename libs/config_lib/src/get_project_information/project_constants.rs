use std::collections::HashMap;

use crate::get_project_information::provider_kind_enum::ProviderKind;

pub const LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE: &str = "сan not load user_credentials file";
pub const LOAD_CONFIG_FILE_ERROR_MESSAGE: &str = "сan not load config file";
pub const PATH_TO_CONFIG: &str = "./config/";
pub const PATH_TO_CONFIG_FOR_TEST: &str = "../../config/";
pub const PROJECT_MODE: &str = "Development"; //later as ENV variable only
pub const USER_CREDENTIALS_FILE_NAME: &str = "User_credentials";

use crate::get_project_information::get_config::get_config_information::CONFIG;

pub fn get_config_provider_string_to_enum_struct() -> HashMap<String, ProviderKind> {
    let mut config_provider_string_to_enum_struct_hasmap: HashMap<String, ProviderKind> =
        HashMap::with_capacity(CONFIG.params.vec_of_provider_names.len());
    config_provider_string_to_enum_struct_hasmap.insert("arxiv".to_string(), ProviderKind::Arxiv);
    config_provider_string_to_enum_struct_hasmap
        .insert("biorxiv".to_string(), ProviderKind::Biorxiv);
    config_provider_string_to_enum_struct_hasmap.insert("github".to_string(), ProviderKind::Github);
    config_provider_string_to_enum_struct_hasmap.insert("habr".to_string(), ProviderKind::Habr);
    config_provider_string_to_enum_struct_hasmap
        .insert("medrxiv".to_string(), ProviderKind::Medrxiv);
    config_provider_string_to_enum_struct_hasmap.insert("reddit".to_string(), ProviderKind::Reddit);
    config_provider_string_to_enum_struct_hasmap
        .insert("twitter".to_string(), ProviderKind::Twitter);
    config_provider_string_to_enum_struct_hasmap
}

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
