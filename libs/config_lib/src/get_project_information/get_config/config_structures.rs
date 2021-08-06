use crate::get_project_information::project_constants::PROJECT_MODE;
use crate::get_project_information::provider_kind_enum::ProviderKind;
use config::{Config, ConfigError, File};
use std::fmt;

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)] //Default,
pub struct ConfigStruct {
    pub params: Params,
    pub mongo_params: MongoParams,
    pub enable_providers: EnableProviders,
    pub links: Links,
    pub enable_providers_prints: EnableProvidersPrints,
    pub enable_warning_high_providers_prints: EnableWarningHighProvidersPrints, //todo maybe rename it into  EnableWarningHighPrintsForProviders
    pub enable_warning_low_providers_prints: EnableWarningLowProvidersPrints,
    pub enable_success_providers_prints: EnableSuccessProvidersPrints,
    pub enable_partial_success_providers_prints: EnablePartialSuccessProvidersPrints,
    pub enable_error_providers_prints: EnableErrorProvidersPrints,
    pub enable_providers_cleaning_warning_logs_directory:
        EnableProvidersCleaningWarningLogsDirectory,
    pub enable_providers_time_measurement: EnableProvidersTimeMeasurement,
    pub enable_providers_links_limits: EnableProvidersLinksLimit,
    pub providers_links_limits: ProvidersLinksLimits,
    pub enable_randomize_order_for_providers_link_parts_for_mongo:
        EnableRandomizeOrderForProvidersLinkPartsForMongo,
    pub print_colors: PrintColors,
    pub env: Env,
}

impl ConfigStruct {
    pub fn new(mode_handler: Option<&str>, path_to_config: &str) -> Result<Self, ConfigError> {
        match mode_handler {
            Some(mode) => {
                //for tests - maybe remove and copy code for testing later but its more comfortable for now
                let mut config = Config::new();
                match config.set("env", mode) {
                    Ok(_) => {
                        match config.merge(File::with_name(&format!("{}{}", path_to_config, mode)))
                        {
                            Ok(_) => {
                                let config_result: Result<Self, ConfigError> = config.try_into();
                                match config_result {
                                    Ok(config_handle) => {
                                        ConfigStruct::wrap_custom_config_checks(config_handle)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            None => {
                // RUN_ENV=Testing cargo run
                let env = std::env::var("RUN_ENV").unwrap_or_else(|_| PROJECT_MODE.into());
                let mut config = Config::new();
                match config.set("env", env.clone()) {
                    Ok(_) => {
                        match config.merge(File::with_name(&format!("{}{}", path_to_config, env))) {
                            Ok(_) => {
                                let config_result: Result<Self, ConfigError> = config.try_into();
                                match config_result {
                                    Ok(config_handle) => {
                                        ConfigStruct::wrap_custom_config_checks(config_handle)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
    fn wrap_custom_config_checks(config_handle: ConfigStruct) -> Result<Self, ConfigError> {
        let is_common_providers_links_limit_valid =
            ConfigStruct::check_valid_i64_common_providers_links_limit_for_mongo(&config_handle);
        let are_providers_links_limits_valid =
            ConfigStruct::check_valid_i64_providers_links_limits_for_mongo(&config_handle);
        let is_warning_logs_directory_name_valid =
            ConfigStruct::check_valid_warning_logs_directory_name(&config_handle);
        let is_unhandled_success_handled_success_are_there_items_initialized_posts_dir_valid =
            ConfigStruct::check_valid_unhandled_success_handled_success_are_there_items_initialized_posts_dir(
                &config_handle,
            );
        let is_db_name_handle_valid =
            ConfigStruct::check_valid_db_name_handle_for_mongo(&config_handle);
        if !is_db_name_handle_valid {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "db_name_handle is not valid".to_string(),
            ));
            drop(error);
        }
        if !is_unhandled_success_handled_success_are_there_items_initialized_posts_dir_valid {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                    "unhandled_success_handled_success_are_there_items_initialized_posts_dir is not valid".to_string(),
                ));
            drop(error);
        }
        if !is_warning_logs_directory_name_valid {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "warning_logs_directory_name is not valid".to_string(),
            ));
            drop(error);
        }
        if !is_common_providers_links_limit_valid {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "common_providers_links_limit is not valid".to_string(),
            ));
            drop(error);
        }
        if !are_providers_links_limits_valid {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "providers_links_limits are not valid".to_string(),
            ));
            drop(error);
        }
        Ok(config_handle)
    }

    // db_collection_handle_second_part = "_link_parts"
    // db_collection_document_field_name_handle = "link_part"
    // path_to_provider_link_parts_folder = "./providers_link_parts/"
    // file_extension = ".json"
    fn check_valid_db_name_handle_for_mongo(config_handle: &ConfigStruct) -> bool {
        !config_handle.mongo_params.db_name_handle.is_empty()
    }
    fn check_valid_unhandled_success_handled_success_are_there_items_initialized_posts_dir(
        config_handle: &ConfigStruct,
    ) -> bool {
        !config_handle
            .params
            .unhandled_success_handled_success_are_there_items_initialized_posts_dir
            .is_empty()
    }
    fn check_valid_warning_logs_directory_name(config_handle: &ConfigStruct) -> bool {
        !config_handle.params.warning_logs_directory_name.is_empty()
    }
    fn check_valid_i64_common_providers_links_limit_for_mongo(
        config_handle: &ConfigStruct,
    ) -> bool {
        config_handle.params.common_providers_links_limit > 0
    }
    fn check_valid_i64_providers_links_limits_for_mongo(config_handle: &ConfigStruct) -> bool {
        let mut checker = true;
        if config_handle.providers_links_limits.links_limit_for_arxiv <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_biorxiv <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_github <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_habr <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_medrxiv <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_reddit <= 0 {
            checker = false;
        }
        if config_handle.providers_links_limits.links_limit_for_twitter <= 0 {
            checker = false;
        }
        checker
    }
    pub fn get_links_limit_wrapper_for_provider(self, provider_kind: &ProviderKind) -> i64 {
        match provider_kind {
            ProviderKind::Arxiv => self.providers_links_limits.links_limit_for_arxiv,
            ProviderKind::Biorxiv => self.providers_links_limits.links_limit_for_biorxiv,
            ProviderKind::Github => self.providers_links_limits.links_limit_for_github,
            ProviderKind::Habr => self.providers_links_limits.links_limit_for_habr,
            ProviderKind::Medrxiv => self.providers_links_limits.links_limit_for_medrxiv,
            ProviderKind::Reddit => self.providers_links_limits.links_limit_for_reddit,
            ProviderKind::Twitter => self.providers_links_limits.links_limit_for_twitter,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Params {
    pub vec_of_provider_names: Vec<String>,
    pub user_credentials_dummy_handle: String, //for ci tests
    pub warning_logs_directory_name: String,
    pub unhandled_success_handled_success_are_there_items_initialized_posts_dir: String,
    pub enable_providers: bool,
    pub enable_cleaning_warning_logs_directory: bool,
    pub enable_time_measurement: bool,
    pub enable_provider_links_limit: bool,
    pub enable_common_providers_links_limit: bool,
    pub common_providers_links_limit: i64,
    pub enable_randomize_order_for_providers_link_parts_for_mongo: bool,
    //
    pub enable_prints: bool,
    pub enable_error_prints: bool,
    pub enable_warning_high_prints: bool,
    pub enable_warning_low_prints: bool,
    pub enable_success_prints: bool,
    pub enable_partial_success_prints: bool,
    pub enable_time_measurement_prints: bool,
    pub enable_cleaning_warning_logs_directory_prints: bool,
    //
    pub enable_all_providers_prints: bool,
    pub enable_error_prints_for_all_providers: bool,
    pub enable_warning_high_prints_for_all_providers: bool,
    pub enable_warning_low_prints_for_all_providers: bool,
    pub enable_success_prints_for_all_providers: bool,
    pub enable_partial_success_prints_for_all_providers: bool,
    pub enable_time_measurement_prints_for_all_providers: bool,
    pub enable_cleaning_warning_logs_directory_prints_for_all_providers: bool,
    //
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MongoParams {
    pub is_cloud: bool,
    pub db_name_handle: String,
    pub db_collection_handle_second_part: String,
    pub db_collection_document_field_name_handle: String,
    pub path_to_provider_link_parts_folder: String,
    pub file_extension: String,
    //
    pub mongo_own_first_handle_url_part: String,
    pub mongo_own_second_handle_url_part: String,
    pub mongo_own_third_handle_url_part: String,
    pub mongo_own_fourth_handle_url_part: String,
    //
    pub mongo_cloud_first_handle_url_part: String,
    pub mongo_cloud_second_handle_url_part: String,
    pub mongo_cloud_third_handle_url_part: String,
    pub mongo_cloud_fourth_handle_url_part: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableProviders {
    pub enable_arxiv: bool,
    pub enable_biorxiv: bool,
    pub enable_github: bool,
    pub enable_habr: bool,
    pub enable_medrxiv: bool,
    pub enable_reddit: bool,
    pub enable_twitter: bool,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Links {
    pub starting_check_link: String,
    pub arxiv_link: String,
    pub biorxiv_link: String,
    pub github_link: String,
    pub habr_link: String,
    pub medrxiv_link: String,
    pub reddit_link: String,
    pub twitter_link: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableProvidersPrints {
    pub enable_prints_arxiv: bool,
    pub enable_prints_biorxiv: bool,
    pub enable_prints_github: bool,
    pub enable_prints_habr: bool,
    pub enable_prints_medrxiv: bool,
    pub enable_prints_reddit: bool,
    pub enable_prints_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableWarningHighProvidersPrints {
    pub enable_warning_high_prints_for_arxiv: bool,
    pub enable_warning_high_prints_for_biorxiv: bool,
    pub enable_warning_high_prints_for_github: bool,
    pub enable_warning_high_prints_for_habr: bool,
    pub enable_warning_high_prints_for_medrxiv: bool,
    pub enable_warning_high_prints_for_reddit: bool,
    pub enable_warning_high_prints_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableWarningLowProvidersPrints {
    pub enable_warning_low_prints_for_arxiv: bool,
    pub enable_warning_low_prints_for_biorxiv: bool,
    pub enable_warning_low_prints_for_github: bool,
    pub enable_warning_low_prints_for_habr: bool,
    pub enable_warning_low_prints_for_medrxiv: bool,
    pub enable_warning_low_prints_for_reddit: bool,
    pub enable_warning_low_prints_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableErrorProvidersPrints {
    pub enable_error_prints_for_arxiv: bool,
    pub enable_error_prints_for_biorxiv: bool,
    pub enable_error_prints_for_github: bool,
    pub enable_error_prints_for_habr: bool,
    pub enable_error_prints_for_medrxiv: bool,
    pub enable_error_prints_for_reddit: bool,
    pub enable_error_prints_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableSuccessProvidersPrints {
    pub enable_success_prints_for_arxiv: bool,
    pub enable_success_prints_for_biorxiv: bool,
    pub enable_success_prints_for_github: bool,
    pub enable_success_prints_for_habr: bool,
    pub enable_success_prints_for_medrxiv: bool,
    pub enable_success_prints_for_reddit: bool,
    pub enable_success_prints_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnablePartialSuccessProvidersPrints {
    pub enable_partial_success_prints_for_arxiv: bool,
    pub enable_partial_success_prints_for_biorxiv: bool,
    pub enable_partial_success_prints_for_github: bool,
    pub enable_partial_success_prints_for_habr: bool,
    pub enable_partial_success_prints_for_medrxiv: bool,
    pub enable_partial_success_prints_for_reddit: bool,
    pub enable_partial_success_prints_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableProvidersCleaningWarningLogsDirectory {
    pub enable_cleaning_warning_logs_directory_for_arxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_biorxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_github: bool,
    pub enable_cleaning_warning_logs_directory_for_habr: bool,
    pub enable_cleaning_warning_logs_directory_for_medrxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_reddit: bool,
    pub enable_cleaning_warning_logs_directory_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableProvidersTimeMeasurement {
    pub enable_time_measurement_for_arxiv: bool,
    pub enable_time_measurement_for_biorxiv: bool,
    pub enable_time_measurement_for_github: bool,
    pub enable_time_measurement_for_habr: bool,
    pub enable_time_measurement_for_medrxiv: bool,
    pub enable_time_measurement_for_reddit: bool,
    pub enable_time_measurement_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableProvidersLinksLimit {
    pub enable_links_limit_for_arxiv: bool,
    pub enable_links_limit_for_biorxiv: bool,
    pub enable_links_limit_for_github: bool,
    pub enable_links_limit_for_habr: bool,
    pub enable_links_limit_for_medrxiv: bool,
    pub enable_links_limit_for_reddit: bool,
    pub enable_links_limit_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ProvidersLinksLimits {
    pub links_limit_for_arxiv: i64,
    pub links_limit_for_biorxiv: i64,
    pub links_limit_for_github: i64,
    pub links_limit_for_habr: i64,
    pub links_limit_for_medrxiv: i64,
    pub links_limit_for_reddit: i64,
    pub links_limit_for_twitter: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableRandomizeOrderForProvidersLinkPartsForMongo {
    pub enable_randomize_order_for_arxiv_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_biorxiv_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_github_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_habr_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_medrxiv_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_reddit_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_twitter_link_parts_for_mongo: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PrintColors {
    pub error_red: u8,
    pub error_green: u8,
    pub error_blue: u8,
    pub warning_high_red: u8,
    pub warning_high_green: u8,
    pub warning_high_blue: u8,
    pub warning_low_red: u8,
    pub warning_low_green: u8,
    pub warning_low_blue: u8,
    pub success_red: u8,
    pub success_green: u8,
    pub success_blue: u8,
    pub partial_success_red: u8,
    pub partial_success_green: u8,
    pub partial_success_blue: u8,
    pub cleaning_red: u8,
    pub cleaning_green: u8,
    pub cleaning_blue: u8,
    pub time_measurement_red: u8,
    pub time_measurement_green: u8,
    pub time_measurement_blue: u8,
}

#[derive(Clone, Debug, serde_derive::Deserialize, PartialEq, serde_derive::Serialize)]
pub enum Env {
    Development,
    Testing,
    Production,
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Env::Development => write!(f, "Development"),
            Env::Production => write!(f, "Production"),
            Env::Testing => write!(f, "Testing"),
        }
    }
}

impl From<&str> for Env {
    fn from(env: &str) -> Self {
        match env {
            "Development" => Env::Development,
            "Production" => Env::Production,
            "Testing" => Env::Testing,
            _ => Env::Development,
        }
    }
}
