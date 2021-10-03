use std::fs;

extern crate serde;

extern crate serde_json;

extern crate toml;

use config::{Config, ConfigError, File};

use dotenv::dotenv;

use itertools::Itertools;

use toml::Value;

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
// use crate::get_project_information::get_config::enable_initialize_mongo_with_providers_link_parts_struct::EnableInitializeMongoWithProvidersLinkParts;
// use crate::get_project_information::get_config::enable_mongo_own_url_parts_struct::EnableMongoOwnUrlParts;
// use crate::get_project_information::get_config::enable_mongo_cloud_url_parts_struct::EnableMongoCloudUrlParts;

use crate::get_project_information::project_constants::PATH_TO_CONFIG;

use crate::get_project_information::project_constants::ENABLE_OVERRIDING_ENV_FILE_ENV_NAME;

use crate::get_project_information::project_constants::PROJECT_MODES;

use crate::get_project_information::project_constants::ARXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::BIORXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::GITHUB_NAME_TO_CHECK;
use crate::get_project_information::project_constants::HABR_NAME_TO_CHECK;
use crate::get_project_information::project_constants::MEDRXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::REDDIT_NAME_TO_CHECK;
use crate::get_project_information::project_constants::TWITTER_NAME_TO_CHECK;

use crate::get_project_information::project_constants::PROJECT_RUN_MODE_ENV_NAME;

// [params]
use crate::get_project_information::project_constants::COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_ERROR_PRINTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INFO_PRINTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PRINTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PROVIDERS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_SUCCESS_PRINTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_TIME_MEASUREMENT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_HIGH_PRINTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_LOW_PRINTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME;
use crate::get_project_information::project_constants::STARTING_CHECK_LINK_ENV_NAME;
use crate::get_project_information::project_constants::UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME;
use crate::get_project_information::project_constants::USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME;
use crate::get_project_information::project_constants::VEC_OF_PROVIDER_NAMES_ENV_NAME;
use crate::get_project_information::project_constants::WARNING_LOGS_DIRECTORY_NAME_ENV_NAME;

// [mongo_params]
use crate::get_project_information::project_constants::DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME;
use crate::get_project_information::project_constants::DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME;
use crate::get_project_information::project_constants::DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::project_constants::IS_CLOUD_ENV_NAME;
use crate::get_project_information::project_constants::LOG_FILE_EXTENSION_ENV_NAME;
use crate::get_project_information::project_constants::PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME;
use crate::get_project_information::project_constants::PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME;
use crate::get_project_information::project_constants::PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME;
use crate::get_project_information::project_constants::PROVIDERS_DB_NAME_HANDLE_ENV_NAME;

// [mongo_params.enable_mongo_own_url_parts]
use crate::get_project_information::project_constants::MONGO_OWN_FIRST_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_OWN_FOURTH_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_OWN_SECOND_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_OWN_THIRD_HANDLE_URL_PART_ENV_NAME;

// [mongo_params.enable_mongo_cloud_url_parts]
use crate::get_project_information::project_constants::MONGO_CLOUD_FIRST_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_CLOUD_FOURTH_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_CLOUD_SECOND_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_CLOUD_THIRD_HANDLE_URL_PART_ENV_NAME;

// [mongo_params.enable_initialize_mongo_with_providers_link_parts]
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME;

// [postgres_params]
use crate::get_project_information::project_constants::POSTGRES_OWN_FIRST_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_OWN_FOURTH_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_OWN_SECOND_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_OWN_THIRD_HANDLE_URL_PART_ENV_NAME;

// [enable_providers]
use crate::get_project_information::project_constants::ARXIV_LINK_ENV_NAME;
use crate::get_project_information::project_constants::BIORXIV_LINK_ENV_NAME;
use crate::get_project_information::project_constants::GITHUB_LINK_ENV_NAME;
use crate::get_project_information::project_constants::HABR_LINK_ENV_NAME;
use crate::get_project_information::project_constants::MEDRXIV_LINK_ENV_NAME;
use crate::get_project_information::project_constants::REDDIT_LINK_ENV_NAME;
use crate::get_project_information::project_constants::TWITTER_LINK_ENV_NAME;

// [providers_check_links]
use crate::get_project_information::project_constants::ENABLE_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_TWITTER_ENV_NAME;

// [enable_providers_prints]
use crate::get_project_information::project_constants::ENABLE_PRINTS_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PRINTS_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PRINTS_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PRINTS_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PRINTS_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PRINTS_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PRINTS_TWITTER_ENV_NAME;

// [enable_warning_high_providers_prints]
use crate::get_project_information::project_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_warning_low_providers_prints]
use crate::get_project_information::project_constants::ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_error_providers_prints]
use crate::get_project_information::project_constants::ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_success_providers_prints]
use crate::get_project_information::project_constants::ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_partial_success_providers_prints]
use crate::get_project_information::project_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_providers_cleaning_warning_logs_directory]
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME;

// [enable_providers_cleaning_warning_logs_db_in_mongo]
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME;

// [enable_providers_cleaning_warning_logs_db_collections_in_mongo]
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME;

// [enable_providers_time_measurement]
use crate::get_project_information::project_constants::ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME;

// [enable_providers_info]
use crate::get_project_information::project_constants::ENABLE_INFO_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INFO_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INFO_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INFO_FOR_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INFO_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INFO_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INFO_FOR_TWITTER_ENV_NAME;

//[providers_links_limits]
use crate::get_project_information::project_constants::ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME;

// [enable_randomize_order_for_providers_link_parts_for_mongo]
use crate::get_project_information::project_constants::ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME;

// [providers_links_limits]
use crate::get_project_information::project_constants::LINKS_LIMIT_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::project_constants::LINKS_LIMIT_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::project_constants::LINKS_LIMIT_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::project_constants::LINKS_LIMIT_FOR_HABR_ENV_NAME;
use crate::get_project_information::project_constants::LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::project_constants::LINKS_LIMIT_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::project_constants::LINKS_LIMIT_FOR_TWITTER_ENV_NAME;

// [print_colors]
use crate::get_project_information::project_constants::ERROR_BLUE_ENV_NAME;
use crate::get_project_information::project_constants::ERROR_GREEN_ENV_NAME;
use crate::get_project_information::project_constants::ERROR_RED_ENV_NAME;

use crate::get_project_information::project_constants::WARNING_HIGH_BLUE_ENV_NAME;
use crate::get_project_information::project_constants::WARNING_HIGH_GREEN_ENV_NAME;
use crate::get_project_information::project_constants::WARNING_HIGH_RED_ENV_NAME;

use crate::get_project_information::project_constants::WARNING_LOW_BLUE_ENV_NAME;
use crate::get_project_information::project_constants::WARNING_LOW_GREEN_ENV_NAME;
use crate::get_project_information::project_constants::WARNING_LOW_RED_ENV_NAME;

use crate::get_project_information::project_constants::SUCCESS_BLUE_ENV_NAME;
use crate::get_project_information::project_constants::SUCCESS_GREEN_ENV_NAME;
use crate::get_project_information::project_constants::SUCCESS_RED_ENV_NAME;

use crate::get_project_information::project_constants::PARTIAL_SUCCESS_BLUE_ENV_NAME;
use crate::get_project_information::project_constants::PARTIAL_SUCCESS_GREEN_ENV_NAME;
use crate::get_project_information::project_constants::PARTIAL_SUCCESS_RED_ENV_NAME;

use crate::get_project_information::project_constants::CLEANING_BLUE_ENV_NAME;
use crate::get_project_information::project_constants::CLEANING_GREEN_ENV_NAME;
use crate::get_project_information::project_constants::CLEANING_RED_ENV_NAME;

use crate::get_project_information::project_constants::TIME_MEASUREMENT_BLUE_ENV_NAME;
use crate::get_project_information::project_constants::TIME_MEASUREMENT_GREEN_ENV_NAME;
use crate::get_project_information::project_constants::TIME_MEASUREMENT_RED_ENV_NAME;

use crate::get_project_information::project_constants::INFO_BLUE_ENV_NAME;
use crate::get_project_information::project_constants::INFO_GREEN_ENV_NAME;
use crate::get_project_information::project_constants::INFO_RED_ENV_NAME;

#[derive(Deserialize, Debug, Clone)]
pub struct Parent {
    // #[serde(deserialize_with = "string_or_seq_string")]
    pub strings: Vec<String>,
}

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

//todo: create custom error type
impl ConfigStruct {
    pub fn new() -> Result<Self, ConfigError> {
        let is_env_file_exists: bool;
        let env_file_message_handle: &str;
        match dotenv() {
            Ok(_) => {
                env_file_message_handle = ".env file";
                is_env_file_exists = true
            }
            Err(e) => {
                //todo: add this message to error message or enum type
                env_file_message_handle = "(cannot load .env file)";
                is_env_file_exists = false
            }
        }
        let enable_override_config_as_string: String;
        match std::env::var(ENABLE_OVERRIDING_ENV_FILE_ENV_NAME) {
            Ok(handle) => enable_override_config_as_string = handle,
            Err(e) => {
                return {
                    Err(ConfigError::Message(format!(
                    "std::env::var(\"{}_ENV_NAME\") failed for console, docker-compose, {}, error: {:#?}",
                    ENABLE_OVERRIDING_ENV_FILE_ENV_NAME, env_file_message_handle, e
                )))
                }
            }
        }
        let enable_override_config: bool;
        match enable_override_config_as_string.parse::<bool>() {
            Ok(handle) => enable_override_config = handle,
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                    ENABLE_OVERRIDING_ENV_FILE_ENV_NAME, e
                )))
            }
        }
        let mode_string_unchecked: String;
        match std::env::var(PROJECT_RUN_MODE_ENV_NAME) {
            Ok(mode_unchecked_is_valid) => {
                mode_string_unchecked = mode_unchecked_is_valid
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var(\"{}_ENV_NAME\") failed for console, docker-compose, {}, error: {:#?}",
                    PROJECT_RUN_MODE_ENV_NAME, env_file_message_handle, e
                )))
            }
        }
        let mut is_mode_valid: bool = false;
        for project_mode in PROJECT_MODES {
            if project_mode == &mode_string_unchecked {
                is_mode_valid = true;
                break;
            }
        }
        if !is_mode_valid {
            return Err(ConfigError::Message(format!(
                "no such project_mode: {}",
                mode_string_unchecked
            )));
        }
        let valid_project_mode: String = mode_string_unchecked;
        ///
        let mode_string: String;
        let enable_override_config: bool;

        match dotenv() {
            Ok(_) => {
                match std::env::var(ENABLE_OVERRIDING_ENV_FILE_ENV_NAME) {
                    Ok(handle) => match handle.parse::<bool>() {
                        Ok(handle) => {
                            match std::env::var(PROJECT_RUN_MODE_ENV_NAME) {
                                Ok(mode) => {
                                    let mut check: bool = false;
                                    for project_mode in PROJECT_MODES {
                                        if project_mode == &mode {
                                            check = true;
                                            break;
                                        }
                                    }
                                    if !check {
                                         return Err(ConfigError::Message(format!(
                                            "no such project_mode: {}",
                                            mode
                                        )));
                                    }
                                    enable_override_config = handle;
                                    mode_string = mode;
                                }
                                Err(e) => {
                                    return Err(ConfigError::Message(format!(
                                        "std::env::var(\"{}_ENV_NAME\") failed for console and .env file, error: {:#?}",
                                        PROJECT_RUN_MODE_ENV_NAME, e
                                    )))
                                }
                            }
                        }
                        Err(e) => {
                            return Err(ConfigError::Message(format!(
                                "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                                ENABLE_OVERRIDING_ENV_FILE_ENV_NAME, e
                            )))
                        }
                    },
                    Err(e) => {
                        return Err(ConfigError::Message(format!(
                        "std::env::var(\"{}_ENV_NAME\") failed for console and docker-compose, error: {:#?}",
                        ENABLE_OVERRIDING_ENV_FILE_ENV_NAME, e
                    )))
                    }
                }
            }
            Err(e) => {
                println!(
                    "dotenv() failed, trying without using .env file, error: {:#?}",
                    e
                );
                match std::env::var(ENABLE_OVERRIDING_ENV_FILE_ENV_NAME) {
                    Ok(handle) => match handle.parse::<bool>() {
                        Ok(handle) => {
                            if !handle {
                                return Err(ConfigError::Message(format!(
                                    "{} is false, returning...",
                                    ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME
                                )));
                            }
                            match std::env::var(PROJECT_RUN_MODE_ENV_NAME) {
                                Ok(mode) => {
                                    let mut check: bool = false;
                                    for project_mode in PROJECT_MODES {
                                        if project_mode == &mode {
                                            check = true;
                                            break;
                                        }
                                    }
                                    if !check {
                                         return Err(ConfigError::Message(format!(
                                            "no such project_mode: {}",
                                            mode
                                        )));
                                    }
                                    enable_override_config = handle;
                                    mode_string = mode;
                                }
                                Err(e) => {
                                    return Err(ConfigError::Message(format!(
                                        "std::env::var(\"{}_ENV_NAME\") failed for console and docker-compose, error: {:#?}",
                                        PROJECT_RUN_MODE_ENV_NAME, e
                                    )))
                                }
                            }
                        }
                        Err(e) => {
                            return Err(ConfigError::Message(format!(
                                "parse::<bool> {}_ENV_NAME failed for console and docker-compose, error: {:#?}",
                                ENABLE_OVERRIDING_ENV_FILE_ENV_NAME, e
                            )))
                        }
                    },
                    Err(e) => {
                        return Err(ConfigError::Message(format!(
                        "std::env::var(\"{}_ENV_NAME\") failed for console and docker-compose, error: {:#?}",
                        ENABLE_OVERRIDING_ENV_FILE_ENV_NAME, e
                    )))
                    }
                }
            }
        }
        println!("mode: {}", mode_string);

        let mmm = "./config/Development.toml";
        // println!("mmm {}", mmm);
        let result_of_opening_file = fs::read_to_string(mmm);
        match result_of_opening_file {
            Ok(stringified_file) => {
                let value = stringified_file.parse::<Value>().unwrap();
                // match
                // matc
                let m: ConfigStruct = value.try_into().unwrap();
                // println!("m {:#?}", m)
            }
            Err(err) => {
                println!("errorr {:#?}", err);
            }
        }
        let mut config = Config::new();
        match config.merge(File::with_name(&format!(
            "{}{}",
            PATH_TO_CONFIG, mode_string
        ))) {
            Ok(_) => match config.try_into() {
                Ok(config_handle_from_file) => {
                    if enable_override_config {
                        //not going to thought error
                        match ConfigStruct::override_config_not_from_env_file(
                            config_handle_from_file,
                        ) {
                            Ok(overrided_config) => {
                                return ConfigStruct::wrap_config_checks(overrided_config)
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    return ConfigStruct::wrap_config_checks(config_handle_from_file);
                }
                Err(e) => {
                    // through error
                    return Err(ConfigError::Message(format!(
                        "config.try_into failed, error: {:#?}",
                        e
                    )));
                }
            },
            Err(e) => {
                // through error
                return Err(ConfigError::Message(format!(
                    "{}{}\nconfig.merge(File::with_name({}{})) error: {:#?}",
                    file!().to_string(),
                    line!().to_string(),
                    PATH_TO_CONFIG,
                    mode_string,
                    e
                )));
            }
        }
    }

    fn override_config_not_from_env_file(
        mut handle_config: ConfigStruct,
        // is_env_file_exists: bool,
    ) -> Result<ConfigStruct, ConfigError> {
        // match std::env::var(VEC_OF_PROVIDER_NAMES_ENV_NAME) {
        //     Ok(handle) => {
        //         let list_of_strings: Result<Parent, serde_json::Error> =
        //             serde_json::from_str(&handle);
        //         match list_of_strings {
        //             Ok(handle) => {
        //                 handle_config.params.vec_of_provider_names = handle.strings;
        //             }
        //             Err(e) => {
        //                 return Err(ConfigError::Message(format!(
        //                     "serde_json::from_str {}_ENV_NAME failed, error: {:#?}",
        //                     VEC_OF_PROVIDER_NAMES_ENV_NAME, e
        //                 )))
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         return Err(ConfigError::Message(format!(
        //             "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
        //             VEC_OF_PROVIDER_NAMES_ENV_NAME, e
        //         )))
        //     }
        // }
        match std::env::var(STARTING_CHECK_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config.params.starting_check_link = handle;
            }
            Err(e) => {
                // if !is_env_file_exists {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    STARTING_CHECK_LINK_ENV_NAME, e
                )));
                // }
            }
        }
        match std::env::var(USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME) {
            Ok(handle) => {
                handle_config.params.user_credentials_dummy_handle = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME, e
                )))
            }
        }
        match std::env::var(WARNING_LOGS_DIRECTORY_NAME_ENV_NAME) {
            Ok(handle) => {
                handle_config.params.warning_logs_directory_name = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    WARNING_LOGS_DIRECTORY_NAME_ENV_NAME, e
                )))
            }
        }
        match std::env::var(
            UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME,
        ) {
            Ok(handle) => {
                handle_config
                    .params
                    .unhandled_success_handled_success_are_there_items_initialized_posts_dir =
                    handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME, e
            )))
            }
        }
        match std::env::var(ENABLE_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PROVIDERS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PROVIDERS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_cleaning_warning_logs_directory = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .params
                        .enable_cleaning_warning_logs_db_in_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .params
                        .enable_cleaning_warning_logs_db_collections_in_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_TIME_MEASUREMENT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_time_measurement = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_TIME_MEASUREMENT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_TIME_MEASUREMENT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_provider_links_limit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_common_providers_links_limit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config.params.common_providers_links_limit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .params
                        .enable_randomize_order_for_providers_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PRINTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PRINTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_ERROR_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_error_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_ERROR_PRINTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_ERROR_PRINTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_warning_high_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_HIGH_PRINTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_HIGH_PRINTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_warning_low_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_LOW_PRINTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_LOW_PRINTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_SUCCESS_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_success_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_SUCCESS_PRINTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_SUCCESS_PRINTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_partial_success_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_time_measurement_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .params
                        .enable_cleaning_warning_logs_directory_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INFO_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_info_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INFO_PRINTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INFO_PRINTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_all_providers_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_error_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .params
                        .enable_warning_high_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .params
                        .enable_warning_low_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_success_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .params
                        .enable_partial_success_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .params
                        .enable_time_measurement_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(
            ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
        ) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .params
                        .enable_cleaning_warning_logs_directory_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_info_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_write_error_logs_in_local_folder = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.params.enable_write_error_logs_in_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .params
                        .enable_initialize_mongo_with_providers_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(IS_CLOUD_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.mongo_params.is_cloud = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        IS_CLOUD_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    IS_CLOUD_ENV_NAME, e
                )))
            }
        }
        match std::env::var(PROVIDERS_DB_NAME_HANDLE_ENV_NAME) {
            Ok(handle) => {
                handle_config.mongo_params.providers_db_name_handle = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    PROVIDERS_DB_NAME_HANDLE_ENV_NAME, e
                )))
            }
        }
        match std::env::var(PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .providers_db_collection_handle_second_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .providers_db_collection_document_field_name_handle = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME, e
                )))
            }
        }
        match std::env::var(DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME) {
            Ok(handle) => {
                handle_config.mongo_params.db_providers_logs_name_handle = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME, e
                )))
            }
        }
        match std::env::var(DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .db_providers_logs_collection_handle_second_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .db_providers_logs_collection_document_field_name_handle = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME, e
                )))
            }
        }
        match std::env::var(PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .path_to_provider_link_parts_folder = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME, e
                )))
            }
        }
        match std::env::var(LOG_FILE_EXTENSION_ENV_NAME) {
            Ok(handle) => {
                handle_config.mongo_params.log_file_extension = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    LOG_FILE_EXTENSION_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .mongo_params
                        .enable_initialize_mongo_with_providers_link_parts
                        .enable_initialize_mongo_with_arxiv_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .mongo_params
                        .enable_initialize_mongo_with_providers_link_parts
                        .enable_initialize_mongo_with_biorxiv_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .mongo_params
                        .enable_initialize_mongo_with_providers_link_parts
                        .enable_initialize_mongo_with_github_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .mongo_params
                        .enable_initialize_mongo_with_providers_link_parts
                        .enable_initialize_mongo_with_habr_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .mongo_params
                        .enable_initialize_mongo_with_providers_link_parts
                        .enable_initialize_mongo_with_medrxiv_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .mongo_params
                        .enable_initialize_mongo_with_providers_link_parts
                        .enable_initialize_mongo_with_reddit_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .mongo_params
                        .enable_initialize_mongo_with_providers_link_parts
                        .enable_initialize_mongo_with_twitter_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME, e
                )))
            }
        }
        match std::env::var(MONGO_OWN_FIRST_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .enable_mongo_own_url_parts
                    .mongo_own_first_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    MONGO_OWN_FIRST_HANDLE_URL_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(MONGO_OWN_SECOND_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .enable_mongo_own_url_parts
                    .mongo_own_second_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    MONGO_OWN_SECOND_HANDLE_URL_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(MONGO_OWN_THIRD_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .enable_mongo_own_url_parts
                    .mongo_own_third_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    MONGO_OWN_THIRD_HANDLE_URL_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(MONGO_OWN_FOURTH_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .enable_mongo_own_url_parts
                    .mongo_own_fourth_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    MONGO_OWN_FOURTH_HANDLE_URL_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(MONGO_CLOUD_FIRST_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .enable_mongo_cloud_url_parts
                    .mongo_cloud_first_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    MONGO_CLOUD_FIRST_HANDLE_URL_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(MONGO_CLOUD_SECOND_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .enable_mongo_cloud_url_parts
                    .mongo_cloud_second_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    MONGO_CLOUD_SECOND_HANDLE_URL_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(MONGO_CLOUD_THIRD_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .enable_mongo_cloud_url_parts
                    .mongo_cloud_third_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    MONGO_CLOUD_THIRD_HANDLE_URL_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(MONGO_CLOUD_FOURTH_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .mongo_params
                    .enable_mongo_cloud_url_parts
                    .mongo_cloud_third_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    MONGO_CLOUD_THIRD_HANDLE_URL_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(POSTGRES_OWN_FIRST_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .postgres_params
                    .postgres_own_first_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    POSTGRES_OWN_FIRST_HANDLE_URL_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(POSTGRES_OWN_SECOND_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .postgres_params
                    .postgres_own_second_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    POSTGRES_OWN_SECOND_HANDLE_URL_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(POSTGRES_OWN_THIRD_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .postgres_params
                    .postgres_own_third_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    POSTGRES_OWN_THIRD_HANDLE_URL_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(POSTGRES_OWN_FOURTH_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config
                    .postgres_params
                    .postgres_own_fourth_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    POSTGRES_OWN_FOURTH_HANDLE_URL_PART_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers.enable_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers.enable_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers.enable_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers.enable_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers.enable_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers.enable_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers.enable_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_TWITTER_ENV_NAME, e
                )))
            }
        }
        //
        match std::env::var(ARXIV_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config.providers_check_links.arxiv_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ARXIV_LINK_ENV_NAME, e
                )))
            }
        }
        match std::env::var(BIORXIV_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config.providers_check_links.biorxiv_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    BIORXIV_LINK_ENV_NAME, e
                )))
            }
        }
        match std::env::var(GITHUB_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config.providers_check_links.github_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    GITHUB_LINK_ENV_NAME, e
                )))
            }
        }
        match std::env::var(HABR_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config.providers_check_links.habr_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    HABR_LINK_ENV_NAME, e
                )))
            }
        }
        match std::env::var(MEDRXIV_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config.providers_check_links.medrxiv_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    MEDRXIV_LINK_ENV_NAME, e
                )))
            }
        }
        match std::env::var(REDDIT_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config.providers_check_links.reddit_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    REDDIT_LINK_ENV_NAME, e
                )))
            }
        }
        match std::env::var(TWITTER_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config.providers_check_links.twitter_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    TWITTER_LINK_ENV_NAME, e
                )))
            }
        }
        //
        match std::env::var(ENABLE_PRINTS_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_prints.enable_prints_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PRINTS_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PRINTS_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PRINTS_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_prints.enable_prints_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PRINTS_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PRINTS_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PRINTS_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_prints.enable_prints_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PRINTS_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PRINTS_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PRINTS_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_prints.enable_prints_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PRINTS_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PRINTS_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PRINTS_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_prints.enable_prints_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PRINTS_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PRINTS_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PRINTS_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_prints.enable_prints_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PRINTS_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PRINTS_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PRINTS_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_prints.enable_prints_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PRINTS_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PRINTS_TWITTER_ENV_NAME, e
                )))
            }
        }
        //
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_high_providers_prints
                        .enable_warning_high_prints_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_high_providers_prints
                        .enable_warning_high_prints_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_high_providers_prints
                        .enable_warning_high_prints_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_high_providers_prints
                        .enable_warning_high_prints_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_high_providers_prints
                        .enable_warning_high_prints_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_high_providers_prints
                        .enable_warning_high_prints_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_high_providers_prints
                        .enable_warning_high_prints_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_low_providers_prints
                        .enable_warning_low_prints_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_low_providers_prints
                        .enable_warning_low_prints_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_low_providers_prints
                        .enable_warning_low_prints_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_low_providers_prints
                        .enable_warning_low_prints_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_low_providers_prints
                        .enable_warning_low_prints_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_low_providers_prints
                        .enable_warning_low_prints_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_warning_low_providers_prints
                        .enable_warning_low_prints_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME, e
                )))
            }
        }
        //
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_success_providers_prints
                        .enable_success_prints_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_success_providers_prints
                        .enable_success_prints_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_success_providers_prints
                        .enable_success_prints_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_success_providers_prints
                        .enable_success_prints_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_success_providers_prints
                        .enable_success_prints_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_success_providers_prints
                        .enable_success_prints_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_success_providers_prints
                        .enable_success_prints_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME, e
                )))
            }
        }
        //
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_partial_success_providers_prints
                        .enable_partial_success_prints_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_partial_success_providers_prints
                        .enable_partial_success_prints_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_partial_success_providers_prints
                        .enable_partial_success_prints_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_partial_success_providers_prints
                        .enable_partial_success_prints_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_partial_success_providers_prints
                        .enable_partial_success_prints_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_partial_success_providers_prints
                        .enable_partial_success_prints_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_partial_success_providers_prints
                        .enable_partial_success_prints_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME, e
                )))
            }
        }
        //
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_error_providers_prints
                        .enable_error_prints_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_error_providers_prints
                        .enable_error_prints_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_error_providers_prints
                        .enable_error_prints_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_error_providers_prints
                        .enable_error_prints_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_error_providers_prints
                        .enable_error_prints_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_error_providers_prints
                        .enable_error_prints_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_error_providers_prints
                        .enable_error_prints_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_error_providers_prints
                        .enable_error_prints_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_in_mongo
                        .enable_cleaning_warning_logs_db_in_mongo_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_in_mongo
                        .enable_cleaning_warning_logs_db_in_mongo_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_in_mongo
                        .enable_cleaning_warning_logs_db_in_mongo_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_in_mongo
                        .enable_cleaning_warning_logs_db_in_mongo_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_in_mongo
                        .enable_cleaning_warning_logs_db_in_mongo_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_in_mongo
                        .enable_cleaning_warning_logs_db_in_mongo_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_in_mongo
                        .enable_cleaning_warning_logs_db_in_mongo_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME)
        {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                        .enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(
            ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME,
        ) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                        .enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME,
                        e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(
            ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME,
        ) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                        .enable_cleaning_warning_logs_db_collections_in_mongo_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME)
        {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                        .enable_cleaning_warning_logs_db_collections_in_mongo_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(
            ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
        ) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                        .enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
                        e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(
            ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME,
        ) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                        .enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(
            ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME,
        ) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                        .enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME,
                        e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_time_measurement
                        .enable_time_measurement_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_time_measurement
                        .enable_time_measurement_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_time_measurement
                        .enable_time_measurement_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_time_measurement
                        .enable_time_measurement_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_time_measurement
                        .enable_time_measurement_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_time_measurement
                        .enable_time_measurement_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_time_measurement
                        .enable_time_measurement_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INFO_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_info.enable_info_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INFO_FOR_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INFO_FOR_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INFO_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_info.enable_info_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INFO_FOR_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INFO_FOR_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INFO_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_info.enable_info_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INFO_FOR_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INFO_FOR_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INFO_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_info.enable_info_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INFO_FOR_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INFO_FOR_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INFO_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_info.enable_info_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INFO_FOR_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INFO_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INFO_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_info.enable_info_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INFO_FOR_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INFO_FOR_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_INFO_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config.enable_providers_info.enable_info_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_INFO_FOR_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_INFO_FOR_TWITTER_ENV_NAME, e
                )))
            }
        }
        //
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_links_limits
                        .enable_links_limit_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_links_limits
                        .enable_links_limit_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_links_limits
                        .enable_links_limit_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_links_limits
                        .enable_links_limit_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_links_limits
                        .enable_links_limit_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_links_limits
                        .enable_links_limit_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_providers_links_limits
                        .enable_links_limit_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME, e
                )))
            }
        }
        match std::env::var(LINKS_LIMIT_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config.providers_links_limits.links_limit_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        LINKS_LIMIT_FOR_ARXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    LINKS_LIMIT_FOR_ARXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(LINKS_LIMIT_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config.providers_links_limits.links_limit_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        LINKS_LIMIT_FOR_BIORXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    LINKS_LIMIT_FOR_BIORXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(LINKS_LIMIT_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config.providers_links_limits.links_limit_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        LINKS_LIMIT_FOR_GITHUB_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    LINKS_LIMIT_FOR_GITHUB_ENV_NAME, e
                )))
            }
        }
        match std::env::var(LINKS_LIMIT_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config.providers_links_limits.links_limit_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        LINKS_LIMIT_FOR_HABR_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    LINKS_LIMIT_FOR_HABR_ENV_NAME, e
                )))
            }
        }
        match std::env::var(LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config.providers_links_limits.links_limit_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME, e
                )))
            }
        }
        match std::env::var(LINKS_LIMIT_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config.providers_links_limits.links_limit_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        LINKS_LIMIT_FOR_REDDIT_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    LINKS_LIMIT_FOR_REDDIT_ENV_NAME, e
                )))
            }
        }
        match std::env::var(LINKS_LIMIT_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config.providers_links_limits.links_limit_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        LINKS_LIMIT_FOR_TWITTER_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    LINKS_LIMIT_FOR_TWITTER_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_randomize_order_for_providers_link_parts_for_mongo
                        .enable_randomize_order_for_arxiv_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_randomize_order_for_providers_link_parts_for_mongo
                        .enable_randomize_order_for_biorxiv_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_randomize_order_for_providers_link_parts_for_mongo
                        .enable_randomize_order_for_github_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_randomize_order_for_providers_link_parts_for_mongo
                        .enable_randomize_order_for_habr_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_randomize_order_for_providers_link_parts_for_mongo
                        .enable_randomize_order_for_medrxiv_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_randomize_order_for_providers_link_parts_for_mongo
                        .enable_randomize_order_for_reddit_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config
                        .enable_randomize_order_for_providers_link_parts_for_mongo
                        .enable_randomize_order_for_twitter_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ERROR_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.error_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ERROR_RED_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ERROR_RED_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ERROR_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.error_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ERROR_GREEN_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ERROR_GREEN_ENV_NAME, e
                )))
            }
        }
        match std::env::var(ERROR_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.error_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        ERROR_BLUE_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    ERROR_BLUE_ENV_NAME, e
                )))
            }
        }
        match std::env::var(WARNING_HIGH_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.warning_high_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        WARNING_HIGH_RED_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    WARNING_HIGH_RED_ENV_NAME, e
                )))
            }
        }
        match std::env::var(WARNING_HIGH_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.warning_high_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        WARNING_HIGH_GREEN_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    WARNING_HIGH_GREEN_ENV_NAME, e
                )))
            }
        }
        match std::env::var(WARNING_HIGH_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.warning_high_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        WARNING_HIGH_BLUE_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    WARNING_HIGH_BLUE_ENV_NAME, e
                )))
            }
        }
        match std::env::var(WARNING_LOW_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.warning_low_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        WARNING_LOW_RED_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    WARNING_LOW_RED_ENV_NAME, e
                )))
            }
        }
        match std::env::var(WARNING_LOW_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.warning_low_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        WARNING_LOW_GREEN_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    WARNING_LOW_GREEN_ENV_NAME, e
                )))
            }
        }
        match std::env::var(WARNING_LOW_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.warning_low_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        WARNING_LOW_BLUE_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    WARNING_LOW_BLUE_ENV_NAME, e
                )))
            }
        }
        //
        match std::env::var(SUCCESS_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.success_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        SUCCESS_RED_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    SUCCESS_RED_ENV_NAME, e
                )))
            }
        }
        match std::env::var(SUCCESS_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.success_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        SUCCESS_GREEN_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    SUCCESS_GREEN_ENV_NAME, e
                )))
            }
        }
        match std::env::var(SUCCESS_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.success_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        SUCCESS_BLUE_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    SUCCESS_BLUE_ENV_NAME, e
                )))
            }
        }
        match std::env::var(PARTIAL_SUCCESS_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.partial_success_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        PARTIAL_SUCCESS_RED_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    PARTIAL_SUCCESS_RED_ENV_NAME, e
                )))
            }
        }
        match std::env::var(PARTIAL_SUCCESS_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.partial_success_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        PARTIAL_SUCCESS_GREEN_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    PARTIAL_SUCCESS_GREEN_ENV_NAME, e
                )))
            }
        }
        match std::env::var(PARTIAL_SUCCESS_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.partial_success_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        PARTIAL_SUCCESS_BLUE_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    PARTIAL_SUCCESS_BLUE_ENV_NAME, e
                )))
            }
        }
        match std::env::var(CLEANING_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.cleaning_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        CLEANING_RED_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    CLEANING_RED_ENV_NAME, e
                )))
            }
        }
        match std::env::var(CLEANING_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.cleaning_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        CLEANING_GREEN_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    CLEANING_GREEN_ENV_NAME, e
                )))
            }
        }
        match std::env::var(CLEANING_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.cleaning_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        CLEANING_BLUE_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    CLEANING_BLUE_ENV_NAME, e
                )))
            }
        }
        //
        match std::env::var(TIME_MEASUREMENT_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.time_measurement_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        TIME_MEASUREMENT_RED_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    TIME_MEASUREMENT_RED_ENV_NAME, e
                )))
            }
        }
        match std::env::var(TIME_MEASUREMENT_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.time_measurement_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        TIME_MEASUREMENT_GREEN_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    TIME_MEASUREMENT_GREEN_ENV_NAME, e
                )))
            }
        }
        match std::env::var(TIME_MEASUREMENT_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.time_measurement_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        TIME_MEASUREMENT_BLUE_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    TIME_MEASUREMENT_BLUE_ENV_NAME, e
                )))
            }
        }
        match std::env::var(INFO_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.info_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        INFO_RED_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    INFO_RED_ENV_NAME, e
                )))
            }
        }
        match std::env::var(INFO_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.info_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        INFO_GREEN_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    INFO_GREEN_ENV_NAME, e
                )))
            }
        }
        match std::env::var(INFO_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config.print_colors.info_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::Message(format!(
                        "parse::<bool> {}_ENV_NAME failed, error: {:#?}",
                        INFO_BLUE_ENV_NAME, e
                    )))
                }
            },
            Err(e) => {
                return Err(ConfigError::Message(format!(
                    "std::env::var({}_ENV_NAME) failed for console and .env file, error: {:#?}",
                    INFO_BLUE_ENV_NAME, e
                )))
            }
        }
        Ok(handle_config)
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
