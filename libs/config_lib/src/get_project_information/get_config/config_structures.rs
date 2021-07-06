use crate::get_project_information::project_constants::PROJECT_MODE;
use config::{Config, ConfigError, File};
use std::fmt;
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)] //Default,
pub struct ConfigStruct {
    pub params: Params,
    pub mongo_params: MongoParams,
    pub enable_providers: EnableProvidersStruct,
    pub links: Links,
    pub enable_prints: EnablePrints,
    pub enable_warning_prints: EnableWarningPrints,
    pub enable_error_prints: EnableErrorPrints,
    pub enable_cleaning_warning_logs_directory: EnableCleaningWarningLogsDirectory,
    pub enable_time_measurement: EnableTimeMeasurement,
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
pub struct EnableProvidersStruct {
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
pub struct EnablePrints {
    pub enable_prints_arxiv: bool,
    pub enable_prints_biorxiv: bool,
    pub enable_prints_github: bool,
    pub enable_prints_habr: bool,
    pub enable_prints_medrxiv: bool,
    pub enable_prints_reddit: bool,
    pub enable_prints_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableWarningPrints {
    pub enable_warning_prints_for_arxiv: bool,
    pub enable_warning_prints_for_biorxiv: bool,
    pub enable_warning_prints_for_github: bool,
    pub enable_warning_prints_for_habr: bool,
    pub enable_warning_prints_for_medrxiv: bool,
    pub enable_warning_prints_for_reddit: bool,
    pub enable_warning_prints_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableErrorPrints {
    pub enable_error_prints_for_arxiv: bool,
    pub enable_error_prints_for_biorxiv: bool,
    pub enable_error_prints_for_github: bool,
    pub enable_error_prints_for_habr: bool,
    pub enable_error_prints_for_medrxiv: bool,
    pub enable_error_prints_for_reddit: bool,
    pub enable_error_prints_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableCleaningWarningLogsDirectory {
    pub enable_cleaning_warning_logs_directory_for_arxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_biorxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_github: bool,
    pub enable_cleaning_warning_logs_directory_for_habr: bool,
    pub enable_cleaning_warning_logs_directory_for_medrxiv: bool,
    pub enable_cleaning_warning_logs_directory_for_reddit: bool,
    pub enable_cleaning_warning_logs_directory_for_twitter: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableTimeMeasurement {
    pub enable_arxiv_time_measurement: bool,
    pub enable_biorxiv_time_measurement: bool,
    pub enable_github_time_measurement: bool,
    pub enable_habr_time_measurement: bool,
    pub enable_medrxiv_time_measurement: bool,
    pub enable_reddit_time_measurement: bool,
    pub enable_twitter_time_measurement: bool,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Params {
    pub user_credentials_dummy_handle: String,
    pub warning_logs_directory_name: String,
    pub unhandled_success_handled_success_are_there_items_initialized_posts_dir: String,
    pub enable_all_providers: bool,
    pub enable_all_providers_prints: bool,
    pub enable_warning_prints_for_all_providers: bool,
    pub enable_error_prints_for_all_providers: bool,
    pub enable_all_cleaning_warning_logs_directory: bool,
    pub enable_prints_handle: bool,
    pub enable_error_prints_handle: bool,
    pub enable_all_time_measurement: bool,
    pub enable_common_time_measurement: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MongoParams {
    pub is_cloud: bool,
    pub db_name_handle: String,
    pub db_collection_handle_second_part: String,
    pub db_collection_document_field_name_handle: String,
    pub path_to_provider_link_parts_folder: String,
    pub vec_of_provider_names: Vec<String>,
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
