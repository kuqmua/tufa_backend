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

use crate::get_project_information::config_error_test::ConfigErrorInnerType;

use super::config_error_test::VarOrBoolParseError;
use super::config_error_test::VarOrIntParseError;

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
pub enum EnvVarKind {
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
    env_var_name_kind: EnvVarKind,
    was_dotenv_enable: bool,
    env_name: &'a str, 
    env_error: ConfigErrorInnerType
} 

impl EnvVarKind {
    pub fn get_env_name(env_var_name_kind: EnvVarKind) -> &'static str {
        match env_var_name_kind {
            EnvVarKind::GithubName => GITHUB_NAME_ENV_NAME,
            EnvVarKind::GithubToken => GITHUB_TOKEN_ENV_NAME,

            EnvVarKind::RedditUserAgent => REDDIT_USER_AGENT_ENV_NAME,
            EnvVarKind::RedditClientId => REDDIT_CLIENT_ID_ENV_NAME,
            EnvVarKind::RedditClientSecret => REDDIT_CLIENT_SECRET_ENV_NAME,
            EnvVarKind::RedditUsername => REDDIT_USERNAME_ENV_NAME,
            EnvVarKind::RedditPassword => REDDIT_PASSWORD_ENV_NAME,

            EnvVarKind::StartingCheckLink => STARTING_CHECK_LINK_ENV_NAME,
            EnvVarKind::UserCredentialsDummyHandle => USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME,
            EnvVarKind::WarningLogsDirectoryName => WARNING_LOGS_DIRECTORY_NAME_ENV_NAME,
            EnvVarKind::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir => UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME,
            EnvVarKind::EnableProviders => ENABLE_PROVIDERS_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDirectory => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbInMongo => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongo => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME,
            EnvVarKind::EnableTimeMeasurement => ENABLE_TIME_MEASUREMENT_ENV_NAME,
            EnvVarKind::EnableProviderLinksLimit => ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME,
            EnvVarKind::EnableCommonProvidersLinksLimit => ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME,
            EnvVarKind::CommonProvidersLinksLimit => COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME,
            EnvVarKind::EnableRandomizeOrderForProvidersLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarKind::EnablePrints => ENABLE_PRINTS_ENV_NAME,
            EnvVarKind::EnableErrorPrints => ENABLE_ERROR_PRINTS_ENV_NAME,
            EnvVarKind::EnableWarningHighPrints => ENABLE_WARNING_HIGH_PRINTS_ENV_NAME,
            EnvVarKind::EnableWarningLowPrints => ENABLE_WARNING_LOW_PRINTS_ENV_NAME,
            EnvVarKind::EnableSuccessPrints => ENABLE_SUCCESS_PRINTS_ENV_NAME,
            EnvVarKind::EnablePartialSuccessPrints => ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME,
            EnvVarKind::EnableTimeMeasurementPrints => ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDirectoryPrints => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME,
            EnvVarKind::EnableInfoPrints => ENABLE_INFO_PRINTS_ENV_NAME,
            EnvVarKind::EnableAllProvidersPrints => ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME,
            EnvVarKind::EnableErrorPrintsForAllProviders => ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarKind::EnableWarningHighPrintsForAllProviders => ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarKind::EnableWarningLowPrintsForAllProviders => ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarKind::EnableSuccessPrintsForAllProviders => ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarKind::EnablePartialSuccessPrintsForAllProviders => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarKind::EnableTimeMeasurementPrintsForAllProviders => ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDirectoryPrintsForAllProviders => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarKind::EnableInfoPrintsForAllProviders => ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarKind::EnableWriteErrorLogsInLocalFolder => ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME,
            EnvVarKind::EnableWriteErrorLogsInMongo => ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME,
            EnvVarKind::EnableInitializeMongoWithProvidersLinkParts => ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME,

            EnvVarKind::ProvidersDbNameHandle => PROVIDERS_DB_NAME_HANDLE_ENV_NAME,
            EnvVarKind::ProvidersDbCollectionHandleSecondPart => PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME,
            EnvVarKind::ProvidersDbCollectionDocumentFieldNameHandle => PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME,
            EnvVarKind::PathToProviderLinkPartsFolder => PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME,
            EnvVarKind::LogFileExtension => LOG_FILE_EXTENSION_ENV_NAME,
            EnvVarKind::DbProvidersLogsNameHandle => DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME,
            EnvVarKind::DbProvidersLogsCollectionHandleSecondPart => DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME,
            EnvVarKind::DbProvidersLogsCollectionDocumentFieldNameHandle => DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME,

            EnvVarKind::EnableInitializeMongoWithArxivLinkParts => ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME,
            EnvVarKind::EnableInitializeMongoWithBiorxivLinkParts => ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME,
            EnvVarKind::EnableInitializeMongoWithGithubLinkParts => ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME,
            EnvVarKind::EnableInitializeMongoWithHabrLinkParts => ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME,
            EnvVarKind::EnableInitializeMongoWithMedrxivLinkParts => ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME,
            EnvVarKind::EnableInitializeMongoWithRedditLinkParts => ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME,
            EnvVarKind::EnableInitializeMongoWithTwitterLinkParts => ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME,

            EnvVarKind::MongoFirstHandleUrlPart => MONGO_FIRST_HANDLE_URL_PART_ENV_NAME,
            EnvVarKind::MongoSecondHandleUrlPart => MONGO_SECOND_HANDLE_URL_PART_ENV_NAME,
            EnvVarKind::MongoThirdHandleUrlPart => MONGO_THIRD_HANDLE_URL_PART_ENV_NAME,
            EnvVarKind::MongoFourthHandleUrlPart => MONGO_FOURTH_HANDLE_URL_PART_ENV_NAME,
            EnvVarKind::MongoFifthHandleUrlPart => MONGO_FIFTH_HANDLE_URL_PART_ENV_NAME,

            EnvVarKind::MongoLogin => MONGO_LOGIN_ENV_NAME,
            EnvVarKind::MongoPassword => MONGO_PASSWORD_ENV_NAME,
            EnvVarKind::MongoIp => MONGO_IP_ENV_NAME,
            EnvVarKind::MongoPort => MONGO_PORT_ENV_NAME,
            EnvVarKind::MongoParams => MONGO_PARAMS_ENV_NAME,

            EnvVarKind::PostgresFirstHandleUrlPart => POSTGRES_FIRST_HANDLE_URL_PART_ENV_NAME,
            EnvVarKind::PostgresSecondHandleUrlPart => POSTGRES_SECOND_HANDLE_URL_PART_ENV_NAME,
            EnvVarKind::PostgresThirdHandleUrlPart => POSTGRES_THIRD_HANDLE_URL_PART_ENV_NAME,
            EnvVarKind::PostgresFourthHandleUrlPart => POSTGRES_FOURTH_HANDLE_URL_PART_ENV_NAME,
            EnvVarKind::PostgresFifthHandleUrlPart => POSTGRES_FIFTH_HANDLE_URL_PART_ENV_NAME,

            EnvVarKind::PostgresLogin => POSTGRES_LOGIN_ENV_NAME,
            EnvVarKind::PostgresPassword => POSTGRES_PASSWORD_ENV_NAME,
            EnvVarKind::PostgresIp => POSTGRES_IP_ENV_NAME,
            EnvVarKind::PostgresPort => POSTGRES_PORT_ENV_NAME,
            EnvVarKind::PostgresDb => POSTGRES_DB_ENV_NAME,

            EnvVarKind::EnableArxiv => ENABLE_ARXIV_ENV_NAME,
            EnvVarKind::EnableBiorxiv => ENABLE_BIORXIV_ENV_NAME,
            EnvVarKind::EnableGithub => ENABLE_GITHUB_ENV_NAME,
            EnvVarKind::EnableHabr => ENABLE_HABR_ENV_NAME,
            EnvVarKind::EnableMedrxiv => ENABLE_MEDRXIV_ENV_NAME,
            EnvVarKind::EnableReddit => ENABLE_REDDIT_ENV_NAME,
            EnvVarKind::EnableTwitter => ENABLE_TWITTER_ENV_NAME,

            EnvVarKind::ArxivLink => ARXIV_LINK_ENV_NAME,
            EnvVarKind::BiorxivLink => BIORXIV_LINK_ENV_NAME,
            EnvVarKind::GithubLink => GITHUB_LINK_ENV_NAME,
            EnvVarKind::HabrLink => HABR_LINK_ENV_NAME,
            EnvVarKind::MedrxivLink => MEDRXIV_LINK_ENV_NAME,
            EnvVarKind::RedditLink => REDDIT_LINK_ENV_NAME,
            EnvVarKind::TwitterLink => TWITTER_LINK_ENV_NAME,

            EnvVarKind::EnablePrintsArxiv => ENABLE_PRINTS_ARXIV_ENV_NAME,
            EnvVarKind::EnablePrintsBiorxiv => ENABLE_PRINTS_BIORXIV_ENV_NAME,
            EnvVarKind::EnablePrintsGithub => ENABLE_PRINTS_GITHUB_ENV_NAME,
            EnvVarKind::EnablePrintsHabr => ENABLE_PRINTS_HABR_ENV_NAME,
            EnvVarKind::EnablePrintsMedrxiv => ENABLE_PRINTS_MEDRXIV_ENV_NAME,
            EnvVarKind::EnablePrintsReddit => ENABLE_PRINTS_REDDIT_ENV_NAME,
            EnvVarKind::EnablePrintsTwitter => ENABLE_PRINTS_TWITTER_ENV_NAME,

            EnvVarKind::EnableWarningHighPrintsForArxiv => ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVarKind::EnableWarningHighPrintsForBiorxiv => ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVarKind::EnableWarningHighPrintsForGithub => ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVarKind::EnableWarningHighPrintsForHabr => ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME,
            EnvVarKind::EnableWarningHighPrintsForMedrxiv => ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVarKind::EnableWarningHighPrintsForReddit => ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVarKind::EnableWarningHighPrintsForTwitter => ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVarKind::EnableWarningLowPrintsForArxiv => ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVarKind::EnableWarningLowPrintsForBiorxiv => ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVarKind::EnableWarningLowPrintsForGithub => ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVarKind::EnableWarningLowPrintsForHabr => ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME,
            EnvVarKind::EnableWarningLowPrintsForMedrxiv => ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVarKind::EnableWarningLowPrintsForReddit => ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVarKind::EnableWarningLowPrintsForTwitter => ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVarKind::EnableErrorPrintsForArxiv => ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVarKind::EnableErrorPrintsForBiorxiv => ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVarKind::EnableErrorPrintsForGithub => ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVarKind::EnableErrorPrintsForHabr => ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME,
            EnvVarKind::EnableErrorPrintsForMedrxiv => ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVarKind::EnableErrorPrintsForReddit => ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVarKind::EnableErrorPrintsForTwitter => ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVarKind::EnableSuccessPrintsForArxiv => ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVarKind::EnableSuccessPrintsForBiorxiv => ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVarKind::EnableSuccessPrintsForGithub => ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVarKind::EnableSuccessPrintsForHabr => ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME,
            EnvVarKind::EnableSuccessPrintsForMedrxiv => ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVarKind::EnableSuccessPrintsForReddit => ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVarKind::EnableSuccessPrintsForTwitter => ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVarKind::EnablePartialSuccessPrintsForArxiv => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVarKind::EnablePartialSuccessPrintsForBiorxiv => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVarKind::EnablePartialSuccessPrintsForGithub => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVarKind::EnablePartialSuccessPrintsForHabr => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME,
            EnvVarKind::EnablePartialSuccessPrintsForMedrxiv => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVarKind::EnablePartialSuccessPrintsForReddit => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVarKind::EnablePartialSuccessPrintsForTwitter => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVarKind::EnableCleaningWarningLogsDirectoryForArxiv => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForBiorxiv => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForGithub => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForHabr => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForMedrxiv => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForReddit => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForTwitter => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME,

            EnvVarKind::EnableCleaningWarningLogsDbInMongoForArxiv => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForBiorxiv => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForGithub => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForHabr => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForMedrxiv => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForReddit => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForTwitter => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME,

            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForGithub => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForHabr => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForReddit => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME,

            EnvVarKind::EnableTimeMeasurementForArxiv => ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME,
            EnvVarKind::EnableTimeMeasurementForBiorxiv => ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME,
            EnvVarKind::EnableTimeMeasurementForGithub => ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME,
            EnvVarKind::EnableTimeMeasurementForHabr => ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME,
            EnvVarKind::EnableTimeMeasurementForMedrxiv => ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME,
            EnvVarKind::EnableTimeMeasurementForReddit => ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME,
            EnvVarKind::EnableTimeMeasurementForTwitter => ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME,

            EnvVarKind::EnableInfoForArxiv => ENABLE_INFO_FOR_ARXIV_ENV_NAME,
            EnvVarKind::EnableInfoForBiorxiv => ENABLE_INFO_FOR_BIORXIV_ENV_NAME,
            EnvVarKind::EnableInfoForGithub => ENABLE_INFO_FOR_GITHUB_ENV_NAME,
            EnvVarKind::EnableInfoForHabr => ENABLE_INFO_FOR_HABR_ENV_NAME,
            EnvVarKind::EnableInfoForMedrxiv => ENABLE_INFO_FOR_MEDRXIV_ENV_NAME,
            EnvVarKind::EnableInfoForReddit => ENABLE_INFO_FOR_REDDIT_ENV_NAME,
            EnvVarKind::EnableInfoForTwitter => ENABLE_INFO_FOR_TWITTER_ENV_NAME,

            EnvVarKind::EnableLinksLimitForArxiv => ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME,
            EnvVarKind::EnableLinksLimitForBiorxiv => ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME,
            EnvVarKind::EnableLinksLimitForGithub => ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME,
            EnvVarKind::EnableLinksLimitForHabr => ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME,
            EnvVarKind::EnableLinksLimitForMedrxiv => ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME,
            EnvVarKind::EnableLinksLimitForReddit => ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME,
            EnvVarKind::EnableLinksLimitForTwitter => ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME,

            EnvVarKind::EnableRandomizeOrderForArxivLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarKind::EnableRandomizeOrderForBiorxivLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarKind::EnableRandomizeOrderForGithubLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarKind::EnableRandomizeOrderForHabrLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarKind::EnableRandomizeOrderForMedrxivLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarKind::EnableRandomizeOrderForRedditLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarKind::EnableRandomizeOrderForTwitterLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME,

            EnvVarKind::LinksLimitForArxiv => LINKS_LIMIT_FOR_ARXIV_ENV_NAME,
            EnvVarKind::LinksLimitForBiorxiv => LINKS_LIMIT_FOR_BIORXIV_ENV_NAME,
            EnvVarKind::LinksLimitForGithub => LINKS_LIMIT_FOR_GITHUB_ENV_NAME,
            EnvVarKind::LinksLimitForHabr => LINKS_LIMIT_FOR_HABR_ENV_NAME,
            EnvVarKind::LinksLimitForMedrxiv => LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME,
            EnvVarKind::LinksLimitForReddit => LINKS_LIMIT_FOR_REDDIT_ENV_NAME,
            EnvVarKind::LinksLimitForTwitter => LINKS_LIMIT_FOR_TWITTER_ENV_NAME,

            EnvVarKind::ErrorRed => ERROR_RED_ENV_NAME,
            EnvVarKind::ErrorGreen => ERROR_GREEN_ENV_NAME,
            EnvVarKind::ErrorBlue => ERROR_BLUE_ENV_NAME,
            EnvVarKind::WarningHighRed => WARNING_HIGH_RED_ENV_NAME,
            EnvVarKind::WarningHighGreen => WARNING_HIGH_GREEN_ENV_NAME,
            EnvVarKind::WarningHighBlue => WARNING_HIGH_BLUE_ENV_NAME,
            EnvVarKind::WarningLowRed => WARNING_LOW_RED_ENV_NAME,
            EnvVarKind::WarningLowGreen => WARNING_LOW_GREEN_ENV_NAME,
            EnvVarKind::WarningLowBlue => WARNING_LOW_BLUE_ENV_NAME,
            EnvVarKind::SuccessRed => SUCCESS_RED_ENV_NAME,
            EnvVarKind::SuccessGreen => SUCCESS_GREEN_ENV_NAME,
            EnvVarKind::SuccessBlue => SUCCESS_BLUE_ENV_NAME,
            EnvVarKind::PartialSuccessRed => PARTIAL_SUCCESS_RED_ENV_NAME,
            EnvVarKind::PartialSuccessGreen => PARTIAL_SUCCESS_GREEN_ENV_NAME,
            EnvVarKind::PartialSuccessBlue => PARTIAL_SUCCESS_BLUE_ENV_NAME,
            EnvVarKind::CleaningRed => CLEANING_RED_ENV_NAME,
            EnvVarKind::CleaningGreen => CLEANING_GREEN_ENV_NAME,
            EnvVarKind::CleaningBlue => CLEANING_BLUE_ENV_NAME,
            EnvVarKind::TimeMeasurementRed => TIME_MEASUREMENT_RED_ENV_NAME,
            EnvVarKind::TimeMeasurementGreen => TIME_MEASUREMENT_GREEN_ENV_NAME,
            EnvVarKind::TimeMeasurementBlue => TIME_MEASUREMENT_BLUE_ENV_NAME,
            EnvVarKind::InfoRed => INFO_RED_ENV_NAME,
            EnvVarKind::InfoGreen => INFO_GREEN_ENV_NAME,
            EnvVarKind::InfoBlue => INFO_BLUE_ENV_NAME, 
        }
    }
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    pub fn into_vec() -> Vec<EnvVarKind> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvVarKind::get_length());
        for env_var_name_kind in EnvVarKind::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvVarKind)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvVarKind::get_length());
        for env_var_name_kind in EnvVarKind::iter() {
            env_var_name_kind_vec.push((EnvVarKind::get_env_name(env_var_name_kind),   env_var_name_kind));
        }
        env_var_name_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvVarKind> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<&'static str, EnvVarKind> =
        HashMap::with_capacity(EnvVarKind::get_length());
        for env_var_name_kind_kind in EnvVarKind::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(EnvVarKind::get_env_name(env_var_name_kind_kind),   env_var_name_kind_kind);
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
    pub fn get_type_handle_for_provider(env_var_name_kind: EnvVarKind) -> EnvVarTypeHandle {
        match env_var_name_kind {
            EnvVarKind::GithubName => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::GithubToken => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::RedditUserAgent => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::RedditClientId => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::RedditClientSecret => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::RedditUsername => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::RedditPassword => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::StartingCheckLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::UserCredentialsDummyHandle => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::WarningLogsDirectoryName => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::EnableProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDirectory => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbInMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableTimeMeasurement => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableProviderLinksLimit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCommonProvidersLinksLimit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::CommonProvidersLinksLimit => EnvVarTypeHandle::I64TypeHandle,
            EnvVarKind::EnableRandomizeOrderForProvidersLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableErrorPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningHighPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningLowPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableSuccessPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePartialSuccessPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableTimeMeasurementPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDirectoryPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInfoPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableAllProvidersPrints => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableErrorPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningHighPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningLowPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableSuccessPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePartialSuccessPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableTimeMeasurementPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDirectoryPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInfoPrintsForAllProviders => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWriteErrorLogsInLocalFolder => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWriteErrorLogsInMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInitializeMongoWithProvidersLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::ProvidersDbNameHandle => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::ProvidersDbCollectionHandleSecondPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::ProvidersDbCollectionDocumentFieldNameHandle => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::PathToProviderLinkPartsFolder => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::LogFileExtension => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::DbProvidersLogsNameHandle => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::DbProvidersLogsCollectionHandleSecondPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::DbProvidersLogsCollectionDocumentFieldNameHandle => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::EnableInitializeMongoWithArxivLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInitializeMongoWithBiorxivLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInitializeMongoWithGithubLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInitializeMongoWithHabrLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInitializeMongoWithMedrxivLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInitializeMongoWithRedditLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInitializeMongoWithTwitterLinkParts => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::MongoFirstHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::MongoSecondHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::MongoThirdHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::MongoFourthHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::MongoFifthHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::MongoLogin => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::MongoPassword => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::MongoIp => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::MongoPort => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::MongoParams => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::PostgresFirstHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::PostgresSecondHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::PostgresThirdHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::PostgresFourthHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::PostgresFifthHandleUrlPart => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::PostgresLogin => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::PostgresPassword => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::PostgresIp => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::PostgresPort => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::PostgresDb => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::EnableArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::ArxivLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::BiorxivLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::GithubLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::HabrLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::MedrxivLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::RedditLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::TwitterLink => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::EnablePrintsArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePrintsBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePrintsGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePrintsHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePrintsMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePrintsReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePrintsTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningHighPrintsForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningHighPrintsForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningHighPrintsForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningHighPrintsForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningHighPrintsForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningHighPrintsForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningHighPrintsForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningLowPrintsForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningLowPrintsForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningLowPrintsForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningLowPrintsForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningLowPrintsForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningLowPrintsForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableWarningLowPrintsForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableErrorPrintsForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableErrorPrintsForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableErrorPrintsForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableErrorPrintsForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableErrorPrintsForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableErrorPrintsForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableErrorPrintsForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableSuccessPrintsForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableSuccessPrintsForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableSuccessPrintsForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableSuccessPrintsForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableSuccessPrintsForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableSuccessPrintsForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableSuccessPrintsForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePartialSuccessPrintsForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePartialSuccessPrintsForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePartialSuccessPrintsForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePartialSuccessPrintsForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePartialSuccessPrintsForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePartialSuccessPrintsForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnablePartialSuccessPrintsForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDirectoryForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbInMongoForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableTimeMeasurementForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableTimeMeasurementForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableTimeMeasurementForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableTimeMeasurementForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableTimeMeasurementForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableTimeMeasurementForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableTimeMeasurementForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInfoForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInfoForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInfoForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInfoForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInfoForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInfoForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableInfoForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableLinksLimitForArxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableLinksLimitForBiorxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableLinksLimitForGithub => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableLinksLimitForHabr => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableLinksLimitForMedrxiv => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableLinksLimitForReddit => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableLinksLimitForTwitter => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableRandomizeOrderForArxivLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableRandomizeOrderForBiorxivLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableRandomizeOrderForGithubLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableRandomizeOrderForHabrLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableRandomizeOrderForMedrxivLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableRandomizeOrderForRedditLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::EnableRandomizeOrderForTwitterLinkPartsForMongo => EnvVarTypeHandle::BoolTypeHandle,
            EnvVarKind::LinksLimitForArxiv => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::LinksLimitForBiorxiv => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::LinksLimitForGithub => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::LinksLimitForHabr => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::LinksLimitForMedrxiv => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::LinksLimitForReddit => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::LinksLimitForTwitter => EnvVarTypeHandle::StrTypeHandle,
            EnvVarKind::ErrorRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::ErrorGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::ErrorBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::WarningHighRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::WarningHighGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::WarningHighBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::WarningLowRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::WarningLowGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::WarningLowBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::SuccessRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::SuccessGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::SuccessBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::PartialSuccessRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::PartialSuccessGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::PartialSuccessBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::CleaningRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::CleaningGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::CleaningBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::TimeMeasurementRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::TimeMeasurementGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::TimeMeasurementBlue => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::InfoRed => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::InfoGreen => EnvVarTypeHandle::U8TypeHandle,
            EnvVarKind::InfoBlue => EnvVarTypeHandle::U8TypeHandle,
        }
    }
    // let type_handle = EnvVarKind::get_type_handle_for_provider(env_var_name_kind);
    pub fn get_string_from_env_var(env_var_name_kind: EnvVarKind, was_dotenv_enable: bool) -> Result<String, ConfigTestError<'static>>{
        let string_name = EnvVarKind::get_env_name(env_var_name_kind);
        println!("---- {}", string_name);
        match std::env::var(string_name) {
            Ok(handle) => {
                Ok(handle)
            }
            Err(e) => {
                return Err(ConfigTestError {env_var_name_kind,  was_dotenv_enable, env_name: string_name, env_error: ConfigErrorInnerType::VarErrorHandle(e) })
            }   
        }
    }
    pub fn test_something() -> Result<HashMap::<EnvVarKind, EnvVarTypeValueHandle>, ConfigTestError<'static>> {
        //declare array or hashtable 
        let was_dotenv_enable: bool;
        match dotenv() {
            Ok(_) => {
                was_dotenv_enable = true;
            },
            Err(e) => {
                was_dotenv_enable = true;
                // println!("dotenv() failed, trying without {} error: {:?}", ENV_FILE_NAME, e);
            }
        }
        let mut hmap: HashMap::<EnvVarKind,(String, &str)> = HashMap::new();
        let mut error_option: Option<ConfigTestError> = None;
        for env_var_name_kind in EnvVarKind::iter() {
            match EnvVarKind::get_string_from_env_var(env_var_name_kind, was_dotenv_enable) {
                Ok(env_var_string) => {
                    hmap.insert(env_var_name_kind, (env_var_string, EnvVarKind::get_env_name(env_var_name_kind)));
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
        //vec 
        let mut hmap_to_return: HashMap::<EnvVarKind, EnvVarTypeValueHandle> = HashMap::new();
        let mut error_option_second: Option<ConfigTestError> = None;
        for (env_var_name_kind, (env_var_string, string_env_name)) in hmap {
            match EnvVarKind::get_type_handle_for_provider(env_var_name_kind) {
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
        //check type if not string
    }
}

