use crate::get_project_information::project_constants::PROJECT_MODE;
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
    pub print_colors: PrintColors,
    pub env: Env,
}

impl ConfigStruct {
    pub fn new(mode_handler: Option<&str>, path_to_config: &str) -> Result<Self, ConfigError> {
        match mode_handler {
            Some(mode) => {
                //for tests - maybe remove and copy code for testing later but its more comfortable for now
                let mut config = Config::new();
                config.set("env", mode)?;
                config.merge(File::with_name(&format!("{}{}", path_to_config, mode)))?;
                config.try_into()
            }
            None => {
                // RUN_ENV=Testing cargo run
                let env = std::env::var("RUN_ENV").unwrap_or_else(|_| PROJECT_MODE.into());
                let mut config = Config::new();
                config.set("env", env.clone())?;
                config.merge(File::with_name(&format!("{}{}", path_to_config, env)))?;
                config.try_into()
            }
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
    pub links_limit_for_arxiv: u32,
    pub links_limit_for_biorxiv: u32,
    pub links_limit_for_github: u32,
    pub links_limit_for_habr: u32,
    pub links_limit_for_medrxiv: u32,
    pub links_limit_for_reddit: u32,
    pub links_limit_for_twitter: u32,
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
