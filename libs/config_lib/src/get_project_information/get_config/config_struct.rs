use crate::get_project_information::get_config::enable_providers_struct::EnableProviders;
use crate::get_project_information::get_config::enable_providers_prints_struct::EnableProvidersPrints;
use crate::get_project_information::get_config::providers_check_links_struct::ProvidersCheckLinks;
use crate::get_project_information::get_config::mongo_params_struct::MongoParams;
use crate::get_project_information::get_config::postgres_params_struct::PostgresParams;
use crate::get_project_information::get_config::enable_error_providers_prints_struct::EnableErrorProvidersPrints;
use crate::get_project_information::get_config::enable_partial_success_providers_prints_struct::EnablePartialSuccessProvidersPrints;
use crate::get_project_information::get_config::enable_providers_cleaning_warning_logs_directory_struct::EnableProvidersCleaningWarningLogsDirectory;
use crate::get_project_information::get_config::enable_providers_links_limit_struct::EnableProvidersLinksLimit;
use crate::get_project_information::get_config::enable_providers_time_measurement_struct::EnableProvidersTimeMeasurement;
use crate::get_project_information::get_config::enable_providers_info_struct::EnableProvidersInfo;
use crate::get_project_information::get_config::enable_randomize_order_for_providers_link_parts_for_mongo_struct::EnableRandomizeOrderForProvidersLinkPartsForMongo;
use crate::get_project_information::get_config::enable_success_providers_prints_struct::EnableSuccessProvidersPrints;
use crate::get_project_information::get_config::enable_warning_high_providers_prints_struct::EnableWarningHighProvidersPrints;
use crate::get_project_information::get_config::enable_warning_low_providers_prints_struct::EnableWarningLowProvidersPrints;
use crate::get_project_information::get_config::params_struct::Params;
use crate::get_project_information::get_config::print_colors_struct::PrintColors;
use crate::get_project_information::get_config::providers_links_limits_struct::ProvidersLinksLimits;
use crate::get_project_information::get_config::enable_providers_cleaning_warning_logs_db_in_mongo_struct::EnableProvidersCleaningWarningLogsDbInMongo;
use crate::get_project_information::get_config::enable_providers_cleaning_warning_logs_db_collections_in_mongo_struct::EnableProvidersCleaningWarningLogsDbCollectionsInMongo;

use crate::get_project_information::project_constants::PROJECT_MODES;
use crate::get_project_information::project_constants::PROJECT_RUN_MODE_ENV_NAME;

use config::{Config, ConfigError, File};

use dotenv::dotenv;

use crate::get_project_information::project_constants::ARXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::BIORXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::GITHUB_NAME_TO_CHECK;
use crate::get_project_information::project_constants::HABR_NAME_TO_CHECK;
use crate::get_project_information::project_constants::MEDRXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::REDDIT_NAME_TO_CHECK;
use crate::get_project_information::project_constants::TWITTER_NAME_TO_CHECK;

use itertools::Itertools;
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)] //Default,
pub struct ConfigStruct {
    pub params: Params,
    pub mongo_params: MongoParams,
    pub postgres_params: PostgresParams,
    pub enable_providers: EnableProviders,
    pub providers_check_links: ProvidersCheckLinks,
    pub enable_providers_prints: EnableProvidersPrints,
    pub enable_warning_high_providers_prints: EnableWarningHighProvidersPrints, //todo maybe rename it into  EnableWarningHighPrintsForProviders
    pub enable_warning_low_providers_prints: EnableWarningLowProvidersPrints,
    pub enable_success_providers_prints: EnableSuccessProvidersPrints,
    pub enable_partial_success_providers_prints: EnablePartialSuccessProvidersPrints,
    pub enable_error_providers_prints: EnableErrorProvidersPrints,
    pub enable_providers_cleaning_warning_logs_directory:
        EnableProvidersCleaningWarningLogsDirectory,
    pub enable_providers_cleaning_warning_logs_db_in_mongo:
        EnableProvidersCleaningWarningLogsDbInMongo,
    pub enable_providers_cleaning_warning_logs_db_collections_in_mongo:
        EnableProvidersCleaningWarningLogsDbCollectionsInMongo,
    pub enable_providers_time_measurement: EnableProvidersTimeMeasurement,
    pub enable_providers_info: EnableProvidersInfo,
    pub enable_providers_links_limits: EnableProvidersLinksLimit,
    pub providers_links_limits: ProvidersLinksLimits,
    pub enable_randomize_order_for_providers_link_parts_for_mongo:
        EnableRandomizeOrderForProvidersLinkPartsForMongo,
    pub print_colors: PrintColors,
}

impl ConfigStruct {
    pub fn new(mode_handler: Option<&str>, path_to_config: &str) -> Result<Self, ConfigError> {
        let mode_string: String;
        if let Some(mode) = mode_handler {
            mode_string = mode.to_string();
        } else {
            let dotenv_result = dotenv();
            match dotenv_result {
                Ok(_) => {
                    //working from console like "ENV_NAME=value cargo run" and from .env file
                    match std::env::var(PROJECT_RUN_MODE_ENV_NAME) {
                        Ok(mode) => {
                            let mut check: bool = false;
                            for project_mode in PROJECT_MODES {
                                if project_mode == &mode {
                                    check = true;
                                    break;
                                }
                            }
                            if check {
                                mode_string = mode;
                            } else {
                                return Err(ConfigError::Message(format!(
                                    "no such project_mode: {}",
                                    mode
                                )));
                            }
                        }
                        Err(e) => {
                            return Err(ConfigError::Message(format!(
                            "std::env::var(\"{}\") failed for console and .env file, error: {:#?}",
                            PROJECT_RUN_MODE_ENV_NAME, e
                        )))
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "dotenv() failed, trying to get PROJECT_RUN_MODE_ENV_NAME from console, error: {:#?}",
                        e
                    );
                    //working from console like "ENV_NAME=value cargo run" and from .env file
                    match std::env::var(PROJECT_RUN_MODE_ENV_NAME) {
                        Ok(mode) => {
                            let mut check: bool = false;
                            for project_mode in PROJECT_MODES {
                                if project_mode == &mode {
                                    check = true;
                                    break;
                                }
                            }
                            if check {
                                mode_string = mode;
                            } else {
                                return Err(ConfigError::Message(format!(
                                    "no such project_mode: {}",
                                    mode
                                )));
                            }
                        }
                        Err(e) => {
                            return Err(ConfigError::Message(format!(
                            "std::env::var(\"{}\") failed for console and .env file, error: {:#?}",
                            PROJECT_RUN_MODE_ENV_NAME, e
                        )))
                        }
                    }
                }
            }
        }
        println!("mode: {}", mode_string);
        let mut config = Config::new();
        match config.set("env", mode_string.clone()) {
            Ok(config_set_env_ok) => {
                match config_set_env_ok.merge(File::with_name(&format!(
                    "{}{}",
                    path_to_config, mode_string
                ))) {
                    Ok(_) => {
                        match config.try_into() {
                            Ok(config_handle) => ConfigStruct::wrap_config_checks(config_handle),
                            Err(e) => {
                                //cannot use print_colorful_message coz circular dependency
                                println!(
                                    "{}{}\nconfig.try_into error: {:#?}",
                                    file!().to_string(),
                                    line!().to_string(),
                                    e
                                );
                                Err(e)
                            }
                        }
                    }
                    Err(e) => {
                        //cannot use print_colorful_message coz circular dependency
                        println!(
                            "{}{}\nconfig.merge(File::with_name({}{})) error: {:#?}",
                            file!().to_string(),
                            line!().to_string(),
                            path_to_config,
                            mode_string,
                            e
                        );
                        Err(e)
                    }
                }
            }
            Err(e) => {
                //cannot use print_colorful_message coz circular dependency
                println!(
                    "{}{}\nconfig.set(\"env\", {}) error: {:#?}",
                    file!().to_string(),
                    line!().to_string(),
                    mode_string,
                    e
                );
                Err(e)
            }
        }
    }

    fn wrap_config_checks(config_handle: ConfigStruct) -> Result<Self, ConfigError> {
        if config_handle.mongo_params.log_file_extension.is_empty() {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle.mongo_params.log_file_extension is not empty".to_string(),
            ));
            drop(error);
        }
        if config_handle
            .mongo_params
            .path_to_provider_link_parts_folder
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle
            .mongo_params
            .path_to_provider_link_parts_folder is empty"
                    .to_string(),
            ));
            drop(error);
        }
        if config_handle
            .mongo_params
            .providers_db_collection_document_field_name_handle
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle
            .mongo_params
            .db_collection_document_field_name_handle is empty"
                    .to_string(),
            ));
            drop(error);
        }
        if config_handle
            .mongo_params
            .providers_db_collection_handle_second_part
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle
            .mongo_params
            .db_collection_handle_second_part is empty"
                    .to_string(),
            ));
            drop(error);
        }
        if config_handle
            .mongo_params
            .providers_db_name_handle
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle.mongo_params.db_name_handle is empty".to_string(),
            ));
            drop(error);
        }
        if config_handle
            .params
            .unhandled_success_handled_success_are_there_items_initialized_posts_dir
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                    "config_handle.params.unhandled_success_handled_success_are_there_items_initialized_posts_dir is empty".to_string(),
                ));
            drop(error);
        }
        if config_handle.params.warning_logs_directory_name.is_empty() {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle.params.warning_logs_directory_name is empty".to_string(),
            ));
            drop(error);
        }
        if config_handle.params.common_providers_links_limit > 0 {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "config_handle.params.common_providers_links_limit <= 0".to_string(),
            ));
            drop(error);
        }
        if !ConfigStruct::check_valid_i64_providers_links_limits_for_mongo(&config_handle) {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "providers_links_limits are not valid".to_string(),
            ));
            drop(error);
        }
        if !ConfigStruct::check_valid_vec_of_provider_names(&config_handle) {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "vec_of_provider_names is not valid".to_string(),
            ));
            drop(error);
        }
        Ok(config_handle)
    }

    fn check_valid_vec_of_provider_names(config_handle: &ConfigStruct) -> bool {
        if config_handle.params.vec_of_provider_names.len() == 0 {
            return true;
        }
        for potential_provider_name in &config_handle.params.vec_of_provider_names {
            if !(potential_provider_name == ARXIV_NAME_TO_CHECK
                || potential_provider_name == BIORXIV_NAME_TO_CHECK
                || potential_provider_name == GITHUB_NAME_TO_CHECK
                || potential_provider_name == HABR_NAME_TO_CHECK
                || potential_provider_name == MEDRXIV_NAME_TO_CHECK
                || potential_provider_name == REDDIT_NAME_TO_CHECK
                || potential_provider_name == TWITTER_NAME_TO_CHECK)
            {
                return false;
            }
        }
        let unique_vec_of_provider_names: Vec<String> = config_handle
            .params
            .vec_of_provider_names
            .clone()
            .into_iter()
            .unique()
            .collect();
        if config_handle.params.vec_of_provider_names == unique_vec_of_provider_names {
            return true;
        } else {
            return false;
        }
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
}
