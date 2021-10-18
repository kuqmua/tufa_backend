use std::collections::HashMap;

use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;

use strum_macros::EnumIter;

use dotenv::dotenv;

use crate::get_project_information::env_var_names_constants::COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_ERROR_PRINTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INFO_PRINTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PRINTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PROVIDERS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_SUCCESS_PRINTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_TIME_MEASUREMENT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME;
use crate::get_project_information::env_var_names_constants::STARTING_CHECK_LINK_ENV_NAME;
use crate::get_project_information::env_var_names_constants::UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME;
use crate::get_project_information::env_var_names_constants::WARNING_LOGS_DIRECTORY_NAME_ENV_NAME;

// [mongo_params]
use crate::get_project_information::env_var_names_constants::DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME;
use crate::get_project_information::env_var_names_constants::DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME;
use crate::get_project_information::env_var_names_constants::DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::env_var_names_constants::LOG_FILE_EXTENSION_ENV_NAME;
use crate::get_project_information::env_var_names_constants::PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME;
use crate::get_project_information::env_var_names_constants::PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME;
use crate::get_project_information::env_var_names_constants::PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME;
use crate::get_project_information::env_var_names_constants::PROVIDERS_DB_NAME_HANDLE_ENV_NAME;

// [mongo_params.mongo_url_parts]
use crate::get_project_information::env_var_names_constants::MONGO_FIRST_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_names_constants::MONGO_FOURTH_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_names_constants::MONGO_SECOND_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_names_constants::MONGO_THIRD_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_names_constants::MONGO_FIFTH_HANDLE_URL_PART_ENV_NAME;

// [mongo_params.enable_initialize_mongo_with_providers_link_parts]
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME;

// [postgres_params]
use crate::get_project_information::env_var_names_constants::POSTGRES_FIRST_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_names_constants::POSTGRES_FOURTH_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_names_constants::POSTGRES_SECOND_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_names_constants::POSTGRES_THIRD_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_names_constants::POSTGRES_FIFTH_HANDLE_URL_PART_ENV_NAME;

// [enable_providers]
use crate::get_project_information::env_var_names_constants::ARXIV_LINK_ENV_NAME;
use crate::get_project_information::env_var_names_constants::BIORXIV_LINK_ENV_NAME;
use crate::get_project_information::env_var_names_constants::GITHUB_LINK_ENV_NAME;
use crate::get_project_information::env_var_names_constants::HABR_LINK_ENV_NAME;
use crate::get_project_information::env_var_names_constants::MEDRXIV_LINK_ENV_NAME;
use crate::get_project_information::env_var_names_constants::REDDIT_LINK_ENV_NAME;
use crate::get_project_information::env_var_names_constants::TWITTER_LINK_ENV_NAME;

// [providers_check_links]
use crate::get_project_information::env_var_names_constants::ENABLE_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_TWITTER_ENV_NAME;

// [enable_providers_prints]
use crate::get_project_information::env_var_names_constants::ENABLE_PRINTS_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PRINTS_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PRINTS_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PRINTS_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PRINTS_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PRINTS_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PRINTS_TWITTER_ENV_NAME;

// [enable_warning_high_providers_prints]
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_warning_low_providers_prints]
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_error_providers_prints]
use crate::get_project_information::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_success_providers_prints]
use crate::get_project_information::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_partial_success_providers_prints]
use crate::get_project_information::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_providers_cleaning_warning_logs_directory]
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME;

// [enable_providers_cleaning_warning_logs_db_in_mongo]
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME;

// [enable_providers_cleaning_warning_logs_db_collections_in_mongo]
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME;

// [enable_providers_time_measurement]
use crate::get_project_information::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME;

// [enable_providers_info]
use crate::get_project_information::env_var_names_constants::ENABLE_INFO_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INFO_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INFO_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INFO_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INFO_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INFO_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INFO_FOR_TWITTER_ENV_NAME;

//[providers_links_limits]
use crate::get_project_information::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME;

// [enable_randomize_order_for_providers_link_parts_for_mongo]
use crate::get_project_information::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME;

// [providers_links_limits]
use crate::get_project_information::env_var_names_constants::LINKS_LIMIT_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::LINKS_LIMIT_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::LINKS_LIMIT_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_names_constants::LINKS_LIMIT_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_names_constants::LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_names_constants::LINKS_LIMIT_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::LINKS_LIMIT_FOR_TWITTER_ENV_NAME;

// [print_colors]
use crate::get_project_information::env_var_names_constants::ERROR_BLUE_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ERROR_GREEN_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ERROR_RED_ENV_NAME;

use crate::get_project_information::env_var_names_constants::WARNING_HIGH_BLUE_ENV_NAME;
use crate::get_project_information::env_var_names_constants::WARNING_HIGH_GREEN_ENV_NAME;
use crate::get_project_information::env_var_names_constants::WARNING_HIGH_RED_ENV_NAME;

use crate::get_project_information::env_var_names_constants::WARNING_LOW_BLUE_ENV_NAME;
use crate::get_project_information::env_var_names_constants::WARNING_LOW_GREEN_ENV_NAME;
use crate::get_project_information::env_var_names_constants::WARNING_LOW_RED_ENV_NAME;

use crate::get_project_information::env_var_names_constants::SUCCESS_BLUE_ENV_NAME;
use crate::get_project_information::env_var_names_constants::SUCCESS_GREEN_ENV_NAME;
use crate::get_project_information::env_var_names_constants::SUCCESS_RED_ENV_NAME;

use crate::get_project_information::env_var_names_constants::PARTIAL_SUCCESS_BLUE_ENV_NAME;
use crate::get_project_information::env_var_names_constants::PARTIAL_SUCCESS_GREEN_ENV_NAME;
use crate::get_project_information::env_var_names_constants::PARTIAL_SUCCESS_RED_ENV_NAME;

use crate::get_project_information::env_var_names_constants::CLEANING_BLUE_ENV_NAME;
use crate::get_project_information::env_var_names_constants::CLEANING_GREEN_ENV_NAME;
use crate::get_project_information::env_var_names_constants::CLEANING_RED_ENV_NAME;

use crate::get_project_information::env_var_names_constants::TIME_MEASUREMENT_BLUE_ENV_NAME;
use crate::get_project_information::env_var_names_constants::TIME_MEASUREMENT_GREEN_ENV_NAME;
use crate::get_project_information::env_var_names_constants::TIME_MEASUREMENT_RED_ENV_NAME;

use crate::get_project_information::env_var_names_constants::INFO_BLUE_ENV_NAME;
use crate::get_project_information::env_var_names_constants::INFO_GREEN_ENV_NAME;
use crate::get_project_information::env_var_names_constants::INFO_RED_ENV_NAME;

use crate::get_project_information::env_var_names_constants::GITHUB_NAME_ENV_NAME;
use crate::get_project_information::env_var_names_constants::GITHUB_TOKEN_ENV_NAME;

use crate::get_project_information::env_var_names_constants::MONGO_IP_ENV_NAME;
use crate::get_project_information::env_var_names_constants::MONGO_LOGIN_ENV_NAME;
use crate::get_project_information::env_var_names_constants::MONGO_PASSWORD_ENV_NAME;
use crate::get_project_information::env_var_names_constants::MONGO_PORT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::MONGO_PARAMS_ENV_NAME;

use crate::get_project_information::env_var_names_constants::POSTGRES_LOGIN_ENV_NAME;
use crate::get_project_information::env_var_names_constants::POSTGRES_PASSWORD_ENV_NAME;
use crate::get_project_information::env_var_names_constants::POSTGRES_IP_ENV_NAME;
use crate::get_project_information::env_var_names_constants::POSTGRES_PORT_ENV_NAME;
use crate::get_project_information::env_var_names_constants::POSTGRES_DB_ENV_NAME;

use crate::get_project_information::env_var_names_constants::REDDIT_CLIENT_ID_ENV_NAME;
use crate::get_project_information::env_var_names_constants::REDDIT_CLIENT_SECRET_ENV_NAME;
use crate::get_project_information::env_var_names_constants::REDDIT_PASSWORD_ENV_NAME;
use crate::get_project_information::env_var_names_constants::REDDIT_USERNAME_ENV_NAME;
use crate::get_project_information::env_var_names_constants::REDDIT_USER_AGENT_ENV_NAME;
use crate::get_project_information::project_constants::ENV_FILE_NAME;

use std::env::VarError;

#[derive(Debug)]
pub enum ConfigErrorInnerType {
    VarErrorHandle(VarError),
    VarOrBoolParseErrorHandle(VarOrBoolParseError),
    VarOrIntParseErrorErrorHandle(VarOrIntParseError)
}

use core::str::ParseBoolError;
use core::num::ParseIntError;

#[derive(Debug)] 
pub enum VarOrBoolParseError {
    Var(VarError),
    Bool(ParseBoolError)
}

#[derive(Debug)] 
pub enum VarOrIntParseError {
    Var(VarError),
    Int(ParseIntError)
}

#[derive(
    EnumVariantCount,
    EnumIter,
    Clone,
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
)]
pub enum EnvVar {
    GithubName,
    GithubToken,
    RedditUserAgent,
    RedditClientId,
    RedditClientSecret,
    RedditUsername,
    RedditPassword,
    StartingCheckLink,
    UserCredentialsDummyHandle,
    WarningLogsDirectoryName,
    UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir,
    EnableProviders,
    EnableCleaningWarningLogsDirectory,
    EnableCleaningWarningLogsDbInMongo,
    EnableCleaningWarningLogsDbCollectionsInMongo,
    EnableTimeMeasurement,
    EnableProviderLinksLimit,
    EnableCommonProvidersLinksLimit,
    CommonProvidersLinksLimit,
    EnableRandomizeOrderForProvidersLinkPartsForMongo,
    EnablePrints,
    EnableErrorPrints,
    EnableWarningHighPrints,
    EnableWarningLowPrints,
    EnableSuccessPrints,
    EnablePartialSuccessPrints,
    EnableTimeMeasurementPrints,
    EnableCleaningWarningLogsDirectoryPrints,
    EnableInfoPrints,
    EnableAllProvidersPrints,
    EnableErrorPrintsForAllProviders,
    EnableWarningHighPrintsForAllProviders,
    EnableWarningLowPrintsForAllProviders,
    EnableSuccessPrintsForAllProviders,
    EnablePartialSuccessPrintsForAllProviders,
    EnableTimeMeasurementPrintsForAllProviders,
    EnableCleaningWarningLogsDirectoryPrintsForAllProviders,
    EnableInfoPrintsForAllProviders,
    EnableWriteErrorLogsInLocalFolder,
    EnableWriteErrorLogsInMongo,
    EnableInitializeMongoWithProvidersLinkParts,
    ProvidersDbNameHandle,
    ProvidersDbCollectionHandleSecondPart,
    ProvidersDbCollectionDocumentFieldNameHandle,
    PathToProviderLinkPartsFolder,
    LogFileExtension,
    DbProvidersLogsNameHandle,
    DbProvidersLogsCollectionHandleSecondPart,
    DbProvidersLogsCollectionDocumentFieldNameHandle,
    EnableInitializeMongoWithArxivLinkParts,
    EnableInitializeMongoWithBiorxivLinkParts,
    EnableInitializeMongoWithGithubLinkParts,
    EnableInitializeMongoWithHabrLinkParts,
    EnableInitializeMongoWithMedrxivLinkParts,
    EnableInitializeMongoWithRedditLinkParts,
    EnableInitializeMongoWithTwitterLinkParts,
    MongoFirstHandleUrlPart,
    MongoSecondHandleUrlPart,
    MongoThirdHandleUrlPart,
    MongoFourthHandleUrlPart,
    MongoFifthHandleUrlPart,
    MongoLogin,
    MongoPassword,
    MongoIp,
    MongoPort,
    MongoParams,
    PostgresFirstHandleUrlPart,
    PostgresSecondHandleUrlPart,
    PostgresThirdHandleUrlPart,
    PostgresFourthHandleUrlPart,
    PostgresFifthHandleUrlPart,
    PostgresLogin,
    PostgresPassword,
    PostgresIp,
    PostgresPort,
    PostgresDb,
    EnableArxiv,
    EnableBiorxiv,
    EnableGithub,
    EnableHabr,
    EnableMedrxiv,
    EnableReddit,
    EnableTwitter,
    ArxivLink,
    BiorxivLink,
    GithubLink,
    HabrLink,
    MedrxivLink,
    RedditLink,
    TwitterLink,
    EnablePrintsArxiv,
    EnablePrintsBiorxiv,
    EnablePrintsGithub,
    EnablePrintsHabr,
    EnablePrintsMedrxiv,
    EnablePrintsReddit,
    EnablePrintsTwitter,
    EnableWarningHighPrintsForArxiv,
    EnableWarningHighPrintsForBiorxiv,
    EnableWarningHighPrintsForGithub,
    EnableWarningHighPrintsForHabr,
    EnableWarningHighPrintsForMedrxiv,
    EnableWarningHighPrintsForReddit,
    EnableWarningHighPrintsForTwitter,
    EnableWarningLowPrintsForArxiv,
    EnableWarningLowPrintsForBiorxiv,
    EnableWarningLowPrintsForGithub,
    EnableWarningLowPrintsForHabr,
    EnableWarningLowPrintsForMedrxiv,
    EnableWarningLowPrintsForReddit,
    EnableWarningLowPrintsForTwitter,
    EnableErrorPrintsForArxiv,
    EnableErrorPrintsForBiorxiv,
    EnableErrorPrintsForGithub,
    EnableErrorPrintsForHabr,
    EnableErrorPrintsForMedrxiv,
    EnableErrorPrintsForReddit,
    EnableErrorPrintsForTwitter,
    EnableSuccessPrintsForArxiv,
    EnableSuccessPrintsForBiorxiv,
    EnableSuccessPrintsForGithub,
    EnableSuccessPrintsForHabr,
    EnableSuccessPrintsForMedrxiv,
    EnableSuccessPrintsForReddit,
    EnableSuccessPrintsForTwitter,
    EnablePartialSuccessPrintsForArxiv,
    EnablePartialSuccessPrintsForBiorxiv,
    EnablePartialSuccessPrintsForGithub,
    EnablePartialSuccessPrintsForHabr,
    EnablePartialSuccessPrintsForMedrxiv,
    EnablePartialSuccessPrintsForReddit,
    EnablePartialSuccessPrintsForTwitter,
    EnableCleaningWarningLogsDirectoryForArxiv,
    EnableCleaningWarningLogsDirectoryForBiorxiv,
    EnableCleaningWarningLogsDirectoryForGithub,
    EnableCleaningWarningLogsDirectoryForHabr,
    EnableCleaningWarningLogsDirectoryForMedrxiv,
    EnableCleaningWarningLogsDirectoryForReddit,
    EnableCleaningWarningLogsDirectoryForTwitter,
    EnableCleaningWarningLogsDbInMongoForArxiv,
    EnableCleaningWarningLogsDbInMongoForBiorxiv,
    EnableCleaningWarningLogsDbInMongoForGithub,
    EnableCleaningWarningLogsDbInMongoForHabr,
    EnableCleaningWarningLogsDbInMongoForMedrxiv,
    EnableCleaningWarningLogsDbInMongoForReddit,
    EnableCleaningWarningLogsDbInMongoForTwitter,
    EnableCleaningWarningLogsDbCollectionsInMongoForArxiv,
    EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv,
    EnableCleaningWarningLogsDbCollectionsInMongoForGithub,
    EnableCleaningWarningLogsDbCollectionsInMongoForHabr,
    EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv,
    EnableCleaningWarningLogsDbCollectionsInMongoForReddit,
    EnableCleaningWarningLogsDbCollectionsInMongoForTwitter,
    EnableTimeMeasurementForArxiv,
    EnableTimeMeasurementForBiorxiv,
    EnableTimeMeasurementForGithub,
    EnableTimeMeasurementForHabr,
    EnableTimeMeasurementForMedrxiv,
    EnableTimeMeasurementForReddit,
    EnableTimeMeasurementForTwitter,
    EnableInfoForArxiv,
    EnableInfoForBiorxiv,
    EnableInfoForGithub,
    EnableInfoForHabr,
    EnableInfoForMedrxiv,
    EnableInfoForReddit,
    EnableInfoForTwitter,
    EnableLinksLimitForArxiv,
    EnableLinksLimitForBiorxiv,
    EnableLinksLimitForGithub,
    EnableLinksLimitForHabr,
    EnableLinksLimitForMedrxiv,
    EnableLinksLimitForReddit,
    EnableLinksLimitForTwitter,
    EnableRandomizeOrderForArxivLinkPartsForMongo,
    EnableRandomizeOrderForBiorxivLinkPartsForMongo,
    EnableRandomizeOrderForGithubLinkPartsForMongo,
    EnableRandomizeOrderForHabrLinkPartsForMongo,
    EnableRandomizeOrderForMedrxivLinkPartsForMongo,
    EnableRandomizeOrderForRedditLinkPartsForMongo,
    EnableRandomizeOrderForTwitterLinkPartsForMongo,
    LinksLimitForArxiv,
    LinksLimitForBiorxiv,
    LinksLimitForGithub,
    LinksLimitForHabr,
    LinksLimitForMedrxiv,
    LinksLimitForReddit,
    LinksLimitForTwitter,
    ErrorRed,
    ErrorGreen,
    ErrorBlue,
    WarningHighRed,
    WarningHighGreen,
    WarningHighBlue,
    WarningLowRed,
    WarningLowGreen,
    WarningLowBlue,
    SuccessRed,
    SuccessGreen,
    SuccessBlue,
    PartialSuccessRed,
    PartialSuccessGreen,
    PartialSuccessBlue,
    CleaningRed,
    CleaningGreen,
    CleaningBlue,
    TimeMeasurementRed,
    TimeMeasurementGreen,
    TimeMeasurementBlue,
    InfoRed,
    InfoGreen,
    InfoBlue,
}

#[derive(Debug)]
pub enum EnvVarTypeHandle {
    BoolTypeHandle,
    StrTypeHandle, 
    U8TypeHandle,
    I64TypeHandle,
}

#[derive(Debug)]
pub enum EnvVarTypeValueHandle {
    BoolTypeValue (bool),
    StringTypeValue (String), 
    U8TypeValue (u8),
    I64TypeValue (i64),
}
#[derive(Debug)] 
pub struct ConfigTestError<'a> {
    env_var_name_kind: EnvVar,
    was_dotenv_enable: bool,
    env_name: &'a str, 
    env_error: ConfigErrorInnerType
} 

impl EnvVar {
    pub fn get_env_name(env_var_name_kind: EnvVar) -> &'static str {
        match env_var_name_kind {
            EnvVar::GithubName => GITHUB_NAME_ENV_NAME,
            EnvVar::GithubToken => GITHUB_TOKEN_ENV_NAME,

            EnvVar::RedditUserAgent => REDDIT_USER_AGENT_ENV_NAME,
            EnvVar::RedditClientId => REDDIT_CLIENT_ID_ENV_NAME,
            EnvVar::RedditClientSecret => REDDIT_CLIENT_SECRET_ENV_NAME,
            EnvVar::RedditUsername => REDDIT_USERNAME_ENV_NAME,
            EnvVar::RedditPassword => REDDIT_PASSWORD_ENV_NAME,

            EnvVar::StartingCheckLink => STARTING_CHECK_LINK_ENV_NAME,
            EnvVar::UserCredentialsDummyHandle => USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME,
            EnvVar::WarningLogsDirectoryName => WARNING_LOGS_DIRECTORY_NAME_ENV_NAME,
            EnvVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir => UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME,
            EnvVar::EnableProviders => ENABLE_PROVIDERS_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDirectory => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbInMongo => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongo => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME,
            EnvVar::EnableTimeMeasurement => ENABLE_TIME_MEASUREMENT_ENV_NAME,
            EnvVar::EnableProviderLinksLimit => ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME,
            EnvVar::EnableCommonProvidersLinksLimit => ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME,
            EnvVar::CommonProvidersLinksLimit => COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME,
            EnvVar::EnableRandomizeOrderForProvidersLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVar::EnablePrints => ENABLE_PRINTS_ENV_NAME,
            EnvVar::EnableErrorPrints => ENABLE_ERROR_PRINTS_ENV_NAME,
            EnvVar::EnableWarningHighPrints => ENABLE_WARNING_HIGH_PRINTS_ENV_NAME,
            EnvVar::EnableWarningLowPrints => ENABLE_WARNING_LOW_PRINTS_ENV_NAME,
            EnvVar::EnableSuccessPrints => ENABLE_SUCCESS_PRINTS_ENV_NAME,
            EnvVar::EnablePartialSuccessPrints => ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME,
            EnvVar::EnableTimeMeasurementPrints => ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDirectoryPrints => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME,
            EnvVar::EnableInfoPrints => ENABLE_INFO_PRINTS_ENV_NAME,
            EnvVar::EnableAllProvidersPrints => ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME,
            EnvVar::EnableErrorPrintsForAllProviders => ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVar::EnableWarningHighPrintsForAllProviders => ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVar::EnableWarningLowPrintsForAllProviders => ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVar::EnableSuccessPrintsForAllProviders => ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVar::EnablePartialSuccessPrintsForAllProviders => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVar::EnableTimeMeasurementPrintsForAllProviders => ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDirectoryPrintsForAllProviders => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVar::EnableInfoPrintsForAllProviders => ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVar::EnableWriteErrorLogsInLocalFolder => ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME,
            EnvVar::EnableWriteErrorLogsInMongo => ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME,
            EnvVar::EnableInitializeMongoWithProvidersLinkParts => ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME,

            EnvVar::ProvidersDbNameHandle => PROVIDERS_DB_NAME_HANDLE_ENV_NAME,
            EnvVar::ProvidersDbCollectionHandleSecondPart => PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME,
            EnvVar::ProvidersDbCollectionDocumentFieldNameHandle => PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME,
            EnvVar::PathToProviderLinkPartsFolder => PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME,
            EnvVar::LogFileExtension => LOG_FILE_EXTENSION_ENV_NAME,
            EnvVar::DbProvidersLogsNameHandle => DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME,
            EnvVar::DbProvidersLogsCollectionHandleSecondPart => DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME,
            EnvVar::DbProvidersLogsCollectionDocumentFieldNameHandle => DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME,

            EnvVar::EnableInitializeMongoWithArxivLinkParts => ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME,
            EnvVar::EnableInitializeMongoWithBiorxivLinkParts => ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME,
            EnvVar::EnableInitializeMongoWithGithubLinkParts => ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME,
            EnvVar::EnableInitializeMongoWithHabrLinkParts => ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME,
            EnvVar::EnableInitializeMongoWithMedrxivLinkParts => ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME,
            EnvVar::EnableInitializeMongoWithRedditLinkParts => ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME,
            EnvVar::EnableInitializeMongoWithTwitterLinkParts => ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME,

            EnvVar::MongoFirstHandleUrlPart => MONGO_FIRST_HANDLE_URL_PART_ENV_NAME,
            EnvVar::MongoSecondHandleUrlPart => MONGO_SECOND_HANDLE_URL_PART_ENV_NAME,
            EnvVar::MongoThirdHandleUrlPart => MONGO_THIRD_HANDLE_URL_PART_ENV_NAME,
            EnvVar::MongoFourthHandleUrlPart => MONGO_FOURTH_HANDLE_URL_PART_ENV_NAME,
            EnvVar::MongoFifthHandleUrlPart => MONGO_FIFTH_HANDLE_URL_PART_ENV_NAME,

            EnvVar::MongoLogin => MONGO_LOGIN_ENV_NAME,
            EnvVar::MongoPassword => MONGO_PASSWORD_ENV_NAME,
            EnvVar::MongoIp => MONGO_IP_ENV_NAME,
            EnvVar::MongoPort => MONGO_PORT_ENV_NAME,
            EnvVar::MongoParams => MONGO_PARAMS_ENV_NAME,

            EnvVar::PostgresFirstHandleUrlPart => POSTGRES_FIRST_HANDLE_URL_PART_ENV_NAME,
            EnvVar::PostgresSecondHandleUrlPart => POSTGRES_SECOND_HANDLE_URL_PART_ENV_NAME,
            EnvVar::PostgresThirdHandleUrlPart => POSTGRES_THIRD_HANDLE_URL_PART_ENV_NAME,
            EnvVar::PostgresFourthHandleUrlPart => POSTGRES_FOURTH_HANDLE_URL_PART_ENV_NAME,
            EnvVar::PostgresFifthHandleUrlPart => POSTGRES_FIFTH_HANDLE_URL_PART_ENV_NAME,

            EnvVar::PostgresLogin => POSTGRES_LOGIN_ENV_NAME,
            EnvVar::PostgresPassword => POSTGRES_PASSWORD_ENV_NAME,
            EnvVar::PostgresIp => POSTGRES_IP_ENV_NAME,
            EnvVar::PostgresPort => POSTGRES_PORT_ENV_NAME,
            EnvVar::PostgresDb => POSTGRES_DB_ENV_NAME,

            EnvVar::EnableArxiv => ENABLE_ARXIV_ENV_NAME,
            EnvVar::EnableBiorxiv => ENABLE_BIORXIV_ENV_NAME,
            EnvVar::EnableGithub => ENABLE_GITHUB_ENV_NAME,
            EnvVar::EnableHabr => ENABLE_HABR_ENV_NAME,
            EnvVar::EnableMedrxiv => ENABLE_MEDRXIV_ENV_NAME,
            EnvVar::EnableReddit => ENABLE_REDDIT_ENV_NAME,
            EnvVar::EnableTwitter => ENABLE_TWITTER_ENV_NAME,

            EnvVar::ArxivLink => ARXIV_LINK_ENV_NAME,
            EnvVar::BiorxivLink => BIORXIV_LINK_ENV_NAME,
            EnvVar::GithubLink => GITHUB_LINK_ENV_NAME,
            EnvVar::HabrLink => HABR_LINK_ENV_NAME,
            EnvVar::MedrxivLink => MEDRXIV_LINK_ENV_NAME,
            EnvVar::RedditLink => REDDIT_LINK_ENV_NAME,
            EnvVar::TwitterLink => TWITTER_LINK_ENV_NAME,

            EnvVar::EnablePrintsArxiv => ENABLE_PRINTS_ARXIV_ENV_NAME,
            EnvVar::EnablePrintsBiorxiv => ENABLE_PRINTS_BIORXIV_ENV_NAME,
            EnvVar::EnablePrintsGithub => ENABLE_PRINTS_GITHUB_ENV_NAME,
            EnvVar::EnablePrintsHabr => ENABLE_PRINTS_HABR_ENV_NAME,
            EnvVar::EnablePrintsMedrxiv => ENABLE_PRINTS_MEDRXIV_ENV_NAME,
            EnvVar::EnablePrintsReddit => ENABLE_PRINTS_REDDIT_ENV_NAME,
            EnvVar::EnablePrintsTwitter => ENABLE_PRINTS_TWITTER_ENV_NAME,

            EnvVar::EnableWarningHighPrintsForArxiv => ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVar::EnableWarningHighPrintsForBiorxiv => ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVar::EnableWarningHighPrintsForGithub => ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVar::EnableWarningHighPrintsForHabr => ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME,
            EnvVar::EnableWarningHighPrintsForMedrxiv => ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVar::EnableWarningHighPrintsForReddit => ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVar::EnableWarningHighPrintsForTwitter => ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVar::EnableWarningLowPrintsForArxiv => ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVar::EnableWarningLowPrintsForBiorxiv => ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVar::EnableWarningLowPrintsForGithub => ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVar::EnableWarningLowPrintsForHabr => ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME,
            EnvVar::EnableWarningLowPrintsForMedrxiv => ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVar::EnableWarningLowPrintsForReddit => ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVar::EnableWarningLowPrintsForTwitter => ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVar::EnableErrorPrintsForArxiv => ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVar::EnableErrorPrintsForBiorxiv => ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVar::EnableErrorPrintsForGithub => ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVar::EnableErrorPrintsForHabr => ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME,
            EnvVar::EnableErrorPrintsForMedrxiv => ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVar::EnableErrorPrintsForReddit => ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVar::EnableErrorPrintsForTwitter => ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVar::EnableSuccessPrintsForArxiv => ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVar::EnableSuccessPrintsForBiorxiv => ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVar::EnableSuccessPrintsForGithub => ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVar::EnableSuccessPrintsForHabr => ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME,
            EnvVar::EnableSuccessPrintsForMedrxiv => ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVar::EnableSuccessPrintsForReddit => ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVar::EnableSuccessPrintsForTwitter => ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVar::EnablePartialSuccessPrintsForArxiv => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVar::EnablePartialSuccessPrintsForBiorxiv => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVar::EnablePartialSuccessPrintsForGithub => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVar::EnablePartialSuccessPrintsForHabr => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME,
            EnvVar::EnablePartialSuccessPrintsForMedrxiv => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVar::EnablePartialSuccessPrintsForReddit => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVar::EnablePartialSuccessPrintsForTwitter => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVar::EnableCleaningWarningLogsDirectoryForArxiv => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDirectoryForBiorxiv => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDirectoryForGithub => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDirectoryForHabr => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDirectoryForMedrxiv => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDirectoryForReddit => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDirectoryForTwitter => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME,

            EnvVar::EnableCleaningWarningLogsDbInMongoForArxiv => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbInMongoForBiorxiv => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbInMongoForGithub => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbInMongoForHabr => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbInMongoForMedrxiv => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbInMongoForReddit => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbInMongoForTwitter => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME,

            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForGithub => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForHabr => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForReddit => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME,

            EnvVar::EnableTimeMeasurementForArxiv => ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME,
            EnvVar::EnableTimeMeasurementForBiorxiv => ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME,
            EnvVar::EnableTimeMeasurementForGithub => ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME,
            EnvVar::EnableTimeMeasurementForHabr => ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME,
            EnvVar::EnableTimeMeasurementForMedrxiv => ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME,
            EnvVar::EnableTimeMeasurementForReddit => ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME,
            EnvVar::EnableTimeMeasurementForTwitter => ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME,

            EnvVar::EnableInfoForArxiv => ENABLE_INFO_FOR_ARXIV_ENV_NAME,
            EnvVar::EnableInfoForBiorxiv => ENABLE_INFO_FOR_BIORXIV_ENV_NAME,
            EnvVar::EnableInfoForGithub => ENABLE_INFO_FOR_GITHUB_ENV_NAME,
            EnvVar::EnableInfoForHabr => ENABLE_INFO_FOR_HABR_ENV_NAME,
            EnvVar::EnableInfoForMedrxiv => ENABLE_INFO_FOR_MEDRXIV_ENV_NAME,
            EnvVar::EnableInfoForReddit => ENABLE_INFO_FOR_REDDIT_ENV_NAME,
            EnvVar::EnableInfoForTwitter => ENABLE_INFO_FOR_TWITTER_ENV_NAME,

            EnvVar::EnableLinksLimitForArxiv => ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME,
            EnvVar::EnableLinksLimitForBiorxiv => ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME,
            EnvVar::EnableLinksLimitForGithub => ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME,
            EnvVar::EnableLinksLimitForHabr => ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME,
            EnvVar::EnableLinksLimitForMedrxiv => ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME,
            EnvVar::EnableLinksLimitForReddit => ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME,
            EnvVar::EnableLinksLimitForTwitter => ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME,

            EnvVar::EnableRandomizeOrderForArxivLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVar::EnableRandomizeOrderForBiorxivLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVar::EnableRandomizeOrderForGithubLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVar::EnableRandomizeOrderForHabrLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVar::EnableRandomizeOrderForMedrxivLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVar::EnableRandomizeOrderForRedditLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVar::EnableRandomizeOrderForTwitterLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME,

            EnvVar::LinksLimitForArxiv => LINKS_LIMIT_FOR_ARXIV_ENV_NAME,
            EnvVar::LinksLimitForBiorxiv => LINKS_LIMIT_FOR_BIORXIV_ENV_NAME,
            EnvVar::LinksLimitForGithub => LINKS_LIMIT_FOR_GITHUB_ENV_NAME,
            EnvVar::LinksLimitForHabr => LINKS_LIMIT_FOR_HABR_ENV_NAME,
            EnvVar::LinksLimitForMedrxiv => LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME,
            EnvVar::LinksLimitForReddit => LINKS_LIMIT_FOR_REDDIT_ENV_NAME,
            EnvVar::LinksLimitForTwitter => LINKS_LIMIT_FOR_TWITTER_ENV_NAME,

            EnvVar::ErrorRed => ERROR_RED_ENV_NAME,
            EnvVar::ErrorGreen => ERROR_GREEN_ENV_NAME,
            EnvVar::ErrorBlue => ERROR_BLUE_ENV_NAME,
            EnvVar::WarningHighRed => WARNING_HIGH_RED_ENV_NAME,
            EnvVar::WarningHighGreen => WARNING_HIGH_GREEN_ENV_NAME,
            EnvVar::WarningHighBlue => WARNING_HIGH_BLUE_ENV_NAME,
            EnvVar::WarningLowRed => WARNING_LOW_RED_ENV_NAME,
            EnvVar::WarningLowGreen => WARNING_LOW_GREEN_ENV_NAME,
            EnvVar::WarningLowBlue => WARNING_LOW_BLUE_ENV_NAME,
            EnvVar::SuccessRed => SUCCESS_RED_ENV_NAME,
            EnvVar::SuccessGreen => SUCCESS_GREEN_ENV_NAME,
            EnvVar::SuccessBlue => SUCCESS_BLUE_ENV_NAME,
            EnvVar::PartialSuccessRed => PARTIAL_SUCCESS_RED_ENV_NAME,
            EnvVar::PartialSuccessGreen => PARTIAL_SUCCESS_GREEN_ENV_NAME,
            EnvVar::PartialSuccessBlue => PARTIAL_SUCCESS_BLUE_ENV_NAME,
            EnvVar::CleaningRed => CLEANING_RED_ENV_NAME,
            EnvVar::CleaningGreen => CLEANING_GREEN_ENV_NAME,
            EnvVar::CleaningBlue => CLEANING_BLUE_ENV_NAME,
            EnvVar::TimeMeasurementRed => TIME_MEASUREMENT_RED_ENV_NAME,
            EnvVar::TimeMeasurementGreen => TIME_MEASUREMENT_GREEN_ENV_NAME,
            EnvVar::TimeMeasurementBlue => TIME_MEASUREMENT_BLUE_ENV_NAME,
            EnvVar::InfoRed => INFO_RED_ENV_NAME,
            EnvVar::InfoGreen => INFO_GREEN_ENV_NAME,
            EnvVar::InfoBlue => INFO_BLUE_ENV_NAME, 
        }
    }
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    pub fn into_vec() -> Vec<EnvVar> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvVar::get_length());
        for env_var_name_kind in EnvVar::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvVar)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvVar::get_length());
        for env_var_name_kind in EnvVar::iter() {
            env_var_name_kind_vec.push((EnvVar::get_env_name(env_var_name_kind),   env_var_name_kind));
        }
        env_var_name_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvVar> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<&'static str, EnvVar> =
        HashMap::with_capacity(EnvVar::get_length());
        for env_var_name_kind_kind in EnvVar::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(EnvVar::get_env_name(env_var_name_kind_kind),   env_var_name_kind_kind);
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
    pub fn get_type_handle_for_provider(env_var_name_kind: EnvVar) -> EnvVarTypeHandle {
        match env_var_name_kind {
            EnvVar::GithubName => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::GithubToken => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::RedditUserAgent => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::RedditClientId => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::RedditClientSecret => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::RedditUsername => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::RedditPassword => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::StartingCheckLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::UserCredentialsDummyHandle => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::WarningLogsDirectoryName => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::EnableProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDirectory => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbInMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableTimeMeasurement => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableProviderLinksLimit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCommonProvidersLinksLimit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::CommonProvidersLinksLimit => EnvVarTypeHandle::I64TypeHandle,
            EnvVar::EnableRandomizeOrderForProvidersLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableErrorPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningHighPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningLowPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableSuccessPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePartialSuccessPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableTimeMeasurementPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDirectoryPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInfoPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableAllProvidersPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableErrorPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningHighPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningLowPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableSuccessPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePartialSuccessPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableTimeMeasurementPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDirectoryPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInfoPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWriteErrorLogsInLocalFolder => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWriteErrorLogsInMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInitializeMongoWithProvidersLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::ProvidersDbNameHandle => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::ProvidersDbCollectionHandleSecondPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::ProvidersDbCollectionDocumentFieldNameHandle => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::PathToProviderLinkPartsFolder => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::LogFileExtension => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::DbProvidersLogsNameHandle => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::DbProvidersLogsCollectionHandleSecondPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::DbProvidersLogsCollectionDocumentFieldNameHandle => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::EnableInitializeMongoWithArxivLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInitializeMongoWithBiorxivLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInitializeMongoWithGithubLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInitializeMongoWithHabrLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInitializeMongoWithMedrxivLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInitializeMongoWithRedditLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInitializeMongoWithTwitterLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::MongoFirstHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::MongoSecondHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::MongoThirdHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::MongoFourthHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::MongoFifthHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::MongoLogin => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::MongoPassword => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::MongoIp => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::MongoPort => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::MongoParams => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::PostgresFirstHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::PostgresSecondHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::PostgresThirdHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::PostgresFourthHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::PostgresFifthHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::PostgresLogin => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::PostgresPassword => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::PostgresIp => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::PostgresPort => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::PostgresDb => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::EnableArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::ArxivLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::BiorxivLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::GithubLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::HabrLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::MedrxivLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::RedditLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::TwitterLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::EnablePrintsArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePrintsBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePrintsGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePrintsHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePrintsMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePrintsReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePrintsTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningHighPrintsForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningHighPrintsForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningHighPrintsForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningHighPrintsForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningHighPrintsForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningHighPrintsForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningHighPrintsForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningLowPrintsForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningLowPrintsForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningLowPrintsForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningLowPrintsForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningLowPrintsForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningLowPrintsForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableWarningLowPrintsForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableErrorPrintsForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableErrorPrintsForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableErrorPrintsForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableErrorPrintsForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableErrorPrintsForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableErrorPrintsForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableErrorPrintsForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableSuccessPrintsForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableSuccessPrintsForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableSuccessPrintsForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableSuccessPrintsForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableSuccessPrintsForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableSuccessPrintsForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableSuccessPrintsForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePartialSuccessPrintsForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePartialSuccessPrintsForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePartialSuccessPrintsForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePartialSuccessPrintsForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePartialSuccessPrintsForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePartialSuccessPrintsForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnablePartialSuccessPrintsForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDirectoryForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDirectoryForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDirectoryForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDirectoryForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDirectoryForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDirectoryForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDirectoryForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbInMongoForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbInMongoForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbInMongoForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbInMongoForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbInMongoForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbInMongoForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbInMongoForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableTimeMeasurementForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableTimeMeasurementForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableTimeMeasurementForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableTimeMeasurementForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableTimeMeasurementForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableTimeMeasurementForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableTimeMeasurementForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInfoForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInfoForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInfoForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInfoForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInfoForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInfoForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableInfoForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableLinksLimitForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableLinksLimitForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableLinksLimitForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableLinksLimitForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableLinksLimitForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableLinksLimitForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableLinksLimitForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableRandomizeOrderForArxivLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableRandomizeOrderForBiorxivLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableRandomizeOrderForGithubLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableRandomizeOrderForHabrLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableRandomizeOrderForMedrxivLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableRandomizeOrderForRedditLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::EnableRandomizeOrderForTwitterLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVar::LinksLimitForArxiv => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::LinksLimitForBiorxiv => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::LinksLimitForGithub => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::LinksLimitForHabr => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::LinksLimitForMedrxiv => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::LinksLimitForReddit => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::LinksLimitForTwitter => EnvVarTypeHandle::StrTypeHandle,
            EnvVar::ErrorRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::ErrorGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::ErrorBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::WarningHighRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::WarningHighGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::WarningHighBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::WarningLowRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::WarningLowGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::WarningLowBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::SuccessRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::SuccessGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::SuccessBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::PartialSuccessRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::PartialSuccessGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::PartialSuccessBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::CleaningRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::CleaningGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::CleaningBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::TimeMeasurementRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::TimeMeasurementGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::TimeMeasurementBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::InfoRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::InfoGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVar::InfoBlue => EnvVarTypeHandle::U8TypeHandle,
        }
    }
    pub fn get_string_from_env_var(env_var_name_kind: EnvVar, was_dotenv_enable: bool) -> Result<String, ConfigTestError<'static>>{
        let string_name = EnvVar::get_env_name(env_var_name_kind);
        match std::env::var(string_name) {
            Ok(handle) => {
                Ok(handle)
            }
            Err(e) => {
                return Err(ConfigTestError {env_var_name_kind,  was_dotenv_enable, env_name: string_name, env_error: ConfigErrorInnerType::VarErrorHandle(e) })
            }   
        }
    }
    pub fn test_something() -> Result<HashMap::<EnvVar, EnvVarTypeValueHandle>, ConfigTestError<'static>> {
        let was_dotenv_enable: bool;
        match dotenv() {
            Ok(_) => {
                was_dotenv_enable = true;
            },
            Err(e) => {
                was_dotenv_enable = false;
                println!("dotenv() failed, trying without {} error: {:?}", ENV_FILE_NAME, e);
            }
        }
        let mut hmap: HashMap::<EnvVar,(String, &str)> = HashMap::new();
        let mut error_option: Option<ConfigTestError> = None;
        for env_var_name_kind in EnvVar::iter() {
            match EnvVar::get_string_from_env_var(env_var_name_kind, was_dotenv_enable) {
                Ok(env_var_string) => {
                    hmap.insert(env_var_name_kind, (env_var_string, EnvVar::get_env_name(env_var_name_kind)));
                }
                Err(e) => {
                    error_option = Some(e);
                    break;
                }
            }
        }
        if let Some(error) = error_option {
            return Err(error)
        }
        let mut hmap_to_return: HashMap::<EnvVar, EnvVarTypeValueHandle> = HashMap::new();
        let mut error_option_second: Option<ConfigTestError> = None;
        for (env_var_name_kind, (env_var_string, string_env_name)) in hmap {
            match EnvVar::get_type_handle_for_provider(env_var_name_kind) {
                EnvVarTypeHandle::BoolTypeHandle => {
                    match env_var_string.parse::<bool>() {
                        Ok(handle) => {
                            hmap_to_return.insert(env_var_name_kind, EnvVarTypeValueHandle::BoolTypeValue(handle));
                        },
                        Err(e) => {
                            error_option_second = Some(ConfigTestError {env_var_name_kind,  was_dotenv_enable, env_name: string_env_name, env_error: ConfigErrorInnerType::VarOrBoolParseErrorHandle(VarOrBoolParseError::Bool(e)) });
                            break;
                        }
                    }
                },
                EnvVarTypeHandle::StrTypeHandle => {
                    hmap_to_return.insert(env_var_name_kind, EnvVarTypeValueHandle::StringTypeValue(env_var_string));
                },
                EnvVarTypeHandle::U8TypeHandle => {
                    match env_var_string.parse::<u8>() {
                        Ok(handle) => {
                            hmap_to_return.insert(env_var_name_kind, EnvVarTypeValueHandle::U8TypeValue(handle));
                        },
                        Err(e) => {
                            error_option_second = Some(ConfigTestError {env_var_name_kind,  was_dotenv_enable, env_name: string_env_name, env_error: ConfigErrorInnerType::VarOrIntParseErrorErrorHandle(VarOrIntParseError::Int(e)) });
                            break;
                        }
                    }
                },
                EnvVarTypeHandle::I64TypeHandle => {
                        match env_var_string.parse::<i64>() {
                        Ok(handle) => {
                            hmap_to_return.insert(env_var_name_kind, EnvVarTypeValueHandle::I64TypeValue(handle));
                        },
                        Err(e) => {
                            error_option_second = Some(ConfigTestError {env_var_name_kind,  was_dotenv_enable, env_name: string_env_name, env_error: ConfigErrorInnerType::VarOrIntParseErrorErrorHandle(VarOrIntParseError::Int(e)) });
                            break;
                        }
                    }
                },
            }
        }
        if let Some(error) = error_option_second {
            return Err(error)
        }
        Ok(hmap_to_return)
    }
}

