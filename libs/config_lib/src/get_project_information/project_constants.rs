use crate::get_project_information::provider_kind_enum::ProviderKind;

pub const LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE: &str = "сan not load user_credentials file";
pub const LOAD_CONFIG_FILE_ERROR_MESSAGE: &str = "сan not load config file";
pub const PATH_TO_CONFIG: &str = "./config/";
pub const PATH_TO_CONFIG_FOR_TEST: &str = "../../config/";
pub const PROJECT_MODE: &str = "Development"; //later as ENV variable only
pub const USER_CREDENTIALS_FILE_NAME: &str = "User_credentials";
pub const ARXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT: ConfigProviderStringToEnumTypeStruct =
    ConfigProviderStringToEnumTypeStruct::new("arxiv", ProviderKind::Arxiv);
pub const BIORXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT: ConfigProviderStringToEnumTypeStruct =
    ConfigProviderStringToEnumTypeStruct::new("biorxiv", ProviderKind::Biorxiv);
pub const GITHUB_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT: ConfigProviderStringToEnumTypeStruct =
    ConfigProviderStringToEnumTypeStruct::new("github", ProviderKind::Github);
pub const HABR_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT: ConfigProviderStringToEnumTypeStruct =
    ConfigProviderStringToEnumTypeStruct::new("habr", ProviderKind::Habr);
pub const MEDRXIV_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT: ConfigProviderStringToEnumTypeStruct =
    ConfigProviderStringToEnumTypeStruct::new("medrxiv", ProviderKind::Medrxiv);
pub const REDDIT_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT: ConfigProviderStringToEnumTypeStruct =
    ConfigProviderStringToEnumTypeStruct::new("reddit", ProviderKind::Reddit);
pub const TWITTER_CONFIG_PROVIDER_STRING_TO_ENUM_STRUCT: ConfigProviderStringToEnumTypeStruct =
    ConfigProviderStringToEnumTypeStruct::new("twitter", ProviderKind::Twitter);
pub struct ConfigProviderStringToEnumTypeStruct {
    pub config_name_value: &'static str,
    pub provider_kind_enum_type: ProviderKind,
}

impl ConfigProviderStringToEnumTypeStruct {
    pub const fn new(
        config_name_value: &'static str,
        provider_kind_enum_type: ProviderKind,
    ) -> Self {
        ConfigProviderStringToEnumTypeStruct {
            config_name_value,
            provider_kind_enum_type,
        }
    }
}
