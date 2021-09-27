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
        /////////////////////////////////////////
        // match std::env::var(GITHUB_NAME_ENV_NAME) {
        //             Ok(handle) => {
        //                 handle_user_credentials.github_authorization.github_name = handle;
        //             }
        //             Err(e) => {
        //                 return Err(ConfigError::Message(format!("std::env::var(GITHUB_NAME_ENV_NAME({})) failed for console and .env file, error: {:#?}", GITHUB_NAME_ENV_NAME, e)))
        //             }
        //         }
        //     let mut handle_config: ConfigStruct = ConfigStruct {
        //         params: Params {
        //                 vec_of_provider_names = ,
        // starting_check_link = ,
        // user_credentials_dummy_handle = ,
        // warning_logs_directory_name = ,
        // unhandled_success_handled_success_are_there_items_initialized_posts_dir = ,
        // enable_providers = ,
        // enable_cleaning_warning_logs_directory = ,
        // enable_cleaning_warning_logs_db_in_mongo = ,
        // enable_cleaning_warning_logs_db_collections_in_mongo = ,
        // enable_time_measurement = ,
        // enable_provider_links_limit = ,
        // enable_common_providers_links_limit = ,
        // common_providers_links_limit = ,
        // enable_randomize_order_for_providers_link_parts_for_mongo = ,
        // //
        // enable_prints = ,
        // enable_error_prints = ,
        // enable_warning_high_prints = ,
        // enable_warning_low_prints = ,
        // enable_success_prints = ,
        // enable_partial_success_prints = ,
        // enable_time_measurement_prints = ,
        // enable_cleaning_warning_logs_directory_prints = ,
        // enable_info_prints = ,
        // //
        // enable_all_providers_prints = ,
        // enable_error_prints_for_all_providers = ,
        // enable_warning_high_prints_for_all_providers = ,
        // enable_warning_low_prints_for_all_providers = ,
        // enable_success_prints_for_all_providers = ,
        // enable_partial_success_prints_for_all_providers = ,
        // enable_time_measurement_prints_for_all_providers = ,
        // enable_cleaning_warning_logs_directory_prints_for_all_providers = ,
        // enable_info_prints_for_all_providers = ,
        // //
        // enable_write_error_logs_in_local_folder = ,
        // enable_write_error_logs_in_mongo = ,
        // enable_initialize_mongo_with_providers_link_parts = ,
        //         },
        // mongo_params: MongoParams {
        //     is_cloud = ,
        // providers_db_name_handle = ,
        // providers_db_collection_handle_second_part = ,
        // providers_db_collection_document_field_name_handle = ,
        // //
        // db_providers_logs_name_handle = ,
        // db_providers_logs_collection_handle_second_part = ,
        // db_providers_logs_collection_document_field_name_handle = ,
        // //
        // path_to_provider_link_parts_folder = ,
        // log_file_extension = ,
        // //
        // enable_initialize_mongo_with_providers_link_parts:
        //     EnableInitializeMongoWithProvidersLinkParts,
        // enable_mongo_own_url_parts: EnableMongoOwnUrlParts,
        // enable_mongo_cloud_url_parts: EnableMongoCloudUrlParts,
        // },
        // postgres_params: PostgresParams {
        //     postgres_own_first_handle_url_part = ,
        // postgres_own_second_handle_url_part = ,
        // postgres_own_third_handle_url_part = ,
        // postgres_own_fourth_handle_url_part = ,
        // },
        // enable_providers: EnableProviders {
        //      enable_arxiv = ,
        // enable_biorxiv = ,
        // enable_github = ,
        // enable_habr = ,
        // enable_medrxiv = ,
        // enable_reddit = ,
        // enable_twitter = ,
        // },
        // providers_check_links: ProvidersCheckLinks {
        //      arxiv_link = ,
        // biorxiv_link = ,
        // github_link = ,
        // habr_link = ,
        // medrxiv_link = ,
        // reddit_link = ,
        // twitter_link = ,
        // },
        // enable_providers_prints: EnableProvidersPrints {
        //       enable_prints_arxiv = ,
        // enable_prints_biorxiv = ,
        // enable_prints_github = ,
        // enable_prints_habr = ,
        // enable_prints_medrxiv = ,
        // enable_prints_reddit = ,
        // enable_prints_twitter = ,
        // },
        // enable_warning_high_providers_prints: EnableWarningHighProvidersPrints {
        //         enable_warning_high_prints_for_arxiv = ,
        // enable_warning_high_prints_for_biorxiv = ,
        // enable_warning_high_prints_for_github = ,
        // enable_warning_high_prints_for_habr = ,
        // enable_warning_high_prints_for_medrxiv = ,
        // enable_warning_high_prints_for_reddit = ,
        // enable_warning_high_prints_for_twitter = ,
        // }, //todo maybe rename it into  EnableWarningHighPrintsForProviders
        // enable_warning_low_providers_prints: EnableWarningLowProvidersPrints {
        //      enable_warning_low_prints_for_arxiv = ,
        // enable_warning_low_prints_for_biorxiv = ,
        // enable_warning_low_prints_for_github = ,
        // enable_warning_low_prints_for_habr = ,
        // enable_warning_low_prints_for_medrxiv = ,
        // enable_warning_low_prints_for_reddit = ,
        // enable_warning_low_prints_for_twitter = ,
        // },
        // enable_success_providers_prints: EnableSuccessProvidersPrints {
        //      enable_success_prints_for_arxiv = ,
        // enable_success_prints_for_biorxiv = ,
        // enable_success_prints_for_github = ,
        // enable_success_prints_for_habr = ,
        // enable_success_prints_for_medrxiv = ,
        // enable_success_prints_for_reddit = ,
        // enable_success_prints_for_twitter = ,
        // },
        // enable_partial_success_providers_prints: EnablePartialSuccessProvidersPrints {
        //      enable_partial_success_prints_for_arxiv = ,
        // enable_partial_success_prints_for_biorxiv = ,
        // enable_partial_success_prints_for_github = ,
        // enable_partial_success_prints_for_habr = ,
        // enable_partial_success_prints_for_medrxiv = ,
        // enable_partial_success_prints_for_reddit = ,
        // enable_partial_success_prints_for_twitter = ,
        // },
        // enable_error_providers_prints: EnableErrorProvidersPrints {
        //     enable_error_prints_for_arxiv = ,
        // enable_error_prints_for_biorxiv = ,
        // enable_error_prints_for_github = ,
        // enable_error_prints_for_habr = ,
        // enable_error_prints_for_medrxiv = ,
        // enable_error_prints_for_reddit = ,
        // enable_error_prints_for_twitter = ,
        // },
        // enable_providers_cleaning_warning_logs_directory:
        //     EnableProvidersCleaningWarningLogsDirectory {
        //          enable_cleaning_warning_logs_directory_for_arxiv = ,
        // enable_cleaning_warning_logs_directory_for_biorxiv = ,
        // enable_cleaning_warning_logs_directory_for_github = ,
        // enable_cleaning_warning_logs_directory_for_habr = ,
        // enable_cleaning_warning_logs_directory_for_medrxiv = ,
        // enable_cleaning_warning_logs_directory_for_reddit = ,
        // enable_cleaning_warning_logs_directory_for_twitter = ,
        //     },
        // enable_providers_cleaning_warning_logs_db_in_mongo:
        //     EnableProvidersCleaningWarningLogsDbInMongo {
        //          enable_cleaning_warning_logs_db_in_mongo_for_arxiv = ,
        // enable_cleaning_warning_logs_db_in_mongo_for_biorxiv = ,
        // enable_cleaning_warning_logs_db_in_mongo_for_github = ,
        // enable_cleaning_warning_logs_db_in_mongo_for_habr = ,
        // enable_cleaning_warning_logs_db_in_mongo_for_medrxiv = ,
        // enable_cleaning_warning_logs_db_in_mongo_for_reddit = ,
        // enable_cleaning_warning_logs_db_in_mongo_for_twitter = ,
        //     },
        // enable_providers_cleaning_warning_logs_db_collections_in_mongo:
        //     EnableProvidersCleaningWarningLogsDbCollectionsInMongo {
        //          enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv = ,
        // enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv = ,
        // enable_cleaning_warning_logs_db_collections_in_mongo_for_github = ,
        // enable_cleaning_warning_logs_db_collections_in_mongo_for_habr = ,
        // enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv = ,
        // enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit = ,
        // enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter = ,
        //     },
        // enable_providers_time_measurement: EnableProvidersTimeMeasurement {
        //      enable_time_measurement_for_arxiv = ,
        // enable_time_measurement_for_biorxiv = ,
        // enable_time_measurement_for_github = ,
        // enable_time_measurement_for_habr = ,
        // enable_time_measurement_for_medrxiv = ,
        // enable_time_measurement_for_reddit = ,
        // enable_time_measurement_for_twitter = ,
        // },
        // enable_providers_info: EnableProvidersInfo {
        //      enable_info_for_arxiv = ,
        // enable_info_for_biorxiv = ,
        // enable_info_for_github = ,
        // enable_info_for_habr = ,
        // enable_info_for_medrxiv = ,
        // enable_info_for_reddit = ,
        // enable_info_for_twitter = ,
        // },
        // enable_providers_links_limits: EnableProvidersLinksLimit {
        //      enable_links_limit_for_arxiv = ,
        // enable_links_limit_for_biorxiv = ,
        // enable_links_limit_for_github = ,
        // enable_links_limit_for_habr = ,
        // enable_links_limit_for_medrxiv = ,
        // enable_links_limit_for_reddit = ,
        // enable_links_limit_for_twitter = ,
        // },
        // providers_links_limits: ProvidersLinksLimits {
        //       links_limit_for_arxiv = ,
        // links_limit_for_biorxiv = ,
        // links_limit_for_github = ,
        // links_limit_for_habr = ,
        // links_limit_for_medrxiv = ,
        // links_limit_for_reddit = ,
        // links_limit_for_twitter = ,
        // },
        // enable_randomize_order_for_providers_link_parts_for_mongo:
        //     EnableRandomizeOrderForProvidersLinkPartsForMongo {
        //         enable_randomize_order_for_arxiv_link_parts_for_mongo = ,
        // enable_randomize_order_for_biorxiv_link_parts_for_mongo = ,
        // enable_randomize_order_for_github_link_parts_for_mongo = ,
        // enable_randomize_order_for_habr_link_parts_for_mongo = ,
        // enable_randomize_order_for_medrxiv_link_parts_for_mongo = ,
        // enable_randomize_order_for_reddit_link_parts_for_mongo = ,
        // enable_randomize_order_for_twitter_link_parts_for_mongo = ,
        //     },
        // print_colors: PrintColors {
        //     error_red = ,
        // error_green = ,
        // error_blue = ,
        // warning_high_red = ,
        // warning_high_green = ,
        // warning_high_blue = ,
        // warning_low_red = ,
        // warning_low_green = ,
        // warning_low_blue = ,
        // success_red = ,
        // success_green = ,
        // success_blue = ,
        // partial_success_red = ,
        // partial_success_green = ,
        // partial_success_blue = ,
        // cleaning_red = ,
        // cleaning_green = ,
        // cleaning_blue = ,
        // time_measurement_red = ,
        // time_measurement_green = ,
        // time_measurement_blue = ,
        // info_red = ,
        // info_green = ,
        // info_blue = ,
        // },
        //     }
        /////////////////////////////////////////
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
