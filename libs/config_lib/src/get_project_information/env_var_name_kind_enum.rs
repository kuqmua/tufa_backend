use std::collections::HashMap;

use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;

use strum_macros::EnumIter;

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
pub enum EnvVarNameKind {
    GithubNameEnvName,
    GithubTokenEnvName,

    RedditUserAgentEnvName,
    RedditClientIdEnvName,
    RedditClientSecretEnvName,
    RedditUsernameEnvName,
    RedditPasswordEnvName,

    StartingCheckLinkEnvName,
    UserCredentialsDummyHandleEnvName,
    WarningLogsDirectoryNameEnvName,
    UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDirEnvName,
    EnableProvidersEnvName,
    EnableCleaningWarningLogsDirectoryEnvName,
    EnableCleaningWarningLogsDbInMongoEnvName,
    EnableCleaningWarningLogsDbCollectionsInMongoEnvName,
    EnableTimeMeasurementEnvName,
    EnableProviderLinksLimitEnvName,
    EnableCommonProvidersLinksLimitEnvName,
    CommonProvidersLinksLimitEnvName,
    EnableRandomizeOrderForProvidersLinkPartsForMongoEnvName,
    EnablePrintsEnvName,
    EnableErrorPrintsEnvName,
    EnableWarningHighPrintsEnvName,
    EnableWarningLowPrintsEnvName,
    EnableSuccessPrintsEnvName,
    EnablePartialSuccessPrintsEnvName,
    EnableTimeMeasurementPrintsEnvName,
    EnableCleaningWarningLogsDirectoryPrintsEnvName,
    EnableInfoPrintsEnvName,
    EnableAllProvidersPrintsEnvName,
    EnableErrorPrintsForAllProvidersEnvName,
    EnableWarningHighPrintsForAllProvidersEnvName,
    EnableWarningLowPrintsForAllProvidersEnvName,
    EnableSuccessPrintsForAllProvidersEnvName,
    EnablePartialSuccessPrintsForAllProvidersEnvName,
    EnableTimeMeasurementPrintsForAllProvidersEnvName,
    EnableCleaningWarningLogsDirectoryPrintsForAllProvidersEnvName,
    EnableInfoPrintsForAllProvidersEnvName,
    EnableWriteErrorLogsInLocalFolderEnvName,
    EnableWriteErrorLogsInMongoEnvName,
    EnableInitializeMongoWithProvidersLinkPartsEnvName,

    ProvidersDbNameHandleEnvName,
    ProvidersDbCollectionHandleSecondPartEnvName,
    ProvidersDbCollectionDocumentFieldNameHandleEnvName,
    PathToProviderLinkPartsFolderEnvName,
    LogFileExtensionEnvName,
    DbProvidersLogsNameHandleEnvName,
    DbProvidersLogsCollectionHandleSecondPartEnvName,
    DbProvidersLogsCollectionDocumentFieldNameHandleEnvName,

    EnableInitializeMongoWithArxivLinkPartsEnvName,
    EnableInitializeMongoWithBiorxivLinkPartsEnvName,
    EnableInitializeMongoWithGithubLinkPartsEnvName,
    EnableInitializeMongoWithHabrLinkPartsEnvName,
    EnableInitializeMongoWithMedrxivLinkPartsEnvName,
    EnableInitializeMongoWithRedditLinkPartsEnvName,
    EnableInitializeMongoWithTwitterLinkPartsEnvName,

    MongoFirstHandleUrlPartEnvName,
    MongoSecondHandleUrlPartEnvName,
    MongoThirdHandleUrlPartEnvName,
    MongoFourthHandleUrlPartEnvName,
    MongoFifthHandleUrlPartEnvName,

    MongoLoginEnvName,
    MongoPasswordEnvName,
    MongoIpEnvName,
    MongoPortEnvName,
    MongoParamsEnvName,

    PostgresFirstHandleUrlPartEnvName,
    PostgresSecondHandleUrlPartEnvName,
    PostgresThirdHandleUrlPartEnvName,
    PostgresFourthHandleUrlPartEnvName,
    PostgresFifthHandleUrlPartEnvName,

    PostgresLoginEnvName,
    PostgresPasswordEnvName,
    PostgresIpEnvName,
    PostgresPortEnvName,
    PostgresDbEnvName,

    EnableArxivEnvName,
    EnableBiorxivEnvName,
    EnableGithubEnvName,
    EnableHabrEnvName,
    EnableMedrxivEnvName,
    EnableRedditEnvName,
    EnableTwitterEnvName,

    ArxivLinkEnvName,
    BiorxivLinkEnvName,
    GithubLinkEnvName,
    HabrLinkEnvName,
    MedrxivLinkEnvName,
    RedditLinkEnvName,
    TwitterLinkEnvName,

    EnablePrintsArxivEnvName,
    EnablePrintsBiorxivEnvName,
    EnablePrintsGithubEnvName,
    EnablePrintsHabrEnvName,
    EnablePrintsMedrxivEnvName,
    EnablePrintsRedditEnvName,
    EnablePrintsTwitterEnvName,

    EnableWarningHighPrintsForArxivEnvName,
    EnableWarningHighPrintsForBiorxivEnvName,
    EnableWarningHighPrintsForGithubEnvName,
    EnableWarningHighPrintsForHabrEnvName,
    EnableWarningHighPrintsForMedrxivEnvName,
    EnableWarningHighPrintsForRedditEnvName,
    EnableWarningHighPrintsForTwitterEnvName,

    EnableWarningLowPrintsForArxivEnvName,
    EnableWarningLowPrintsForBiorxivEnvName,
    EnableWarningLowPrintsForGithubEnvName,
    EnableWarningLowPrintsForHabrEnvName,
    EnableWarningLowPrintsForMedrxivEnvName,
    EnableWarningLowPrintsForRedditEnvName,
    EnableWarningLowPrintsForTwitterEnvName,

    EnableErrorPrintsForArxivEnvName,
    EnableErrorPrintsForBiorxivEnvName,
    EnableErrorPrintsForGithubEnvName,
    EnableErrorPrintsForHabrEnvName,
    EnableErrorPrintsForMedrxivEnvName,
    EnableErrorPrintsForRedditEnvName,
    EnableErrorPrintsForTwitterEnvName,

    EnableSuccessPrintsForArxivEnvName,
    EnableSuccessPrintsForBiorxivEnvName,
    EnableSuccessPrintsForGithubEnvName,
    EnableSuccessPrintsForHabrEnvName,
    EnableSuccessPrintsForMedrxivEnvName,
    EnableSuccessPrintsForRedditEnvName,
    EnableSuccessPrintsForTwitterEnvName,

    EnablePartialSuccessPrintsForArxivEnvName,
    EnablePartialSuccessPrintsForBiorxivEnvName,
    EnablePartialSuccessPrintsForGithubEnvName,
    EnablePartialSuccessPrintsForHabrEnvName,
    EnablePartialSuccessPrintsForMedrxivEnvName,
    EnablePartialSuccessPrintsForRedditEnvName,
    EnablePartialSuccessPrintsForTwitterEnvName,

    EnableCleaningWarningLogsDirectoryForArxivEnvName,
    EnableCleaningWarningLogsDirectoryForBiorxivEnvName,
    EnableCleaningWarningLogsDirectoryForGithubEnvName,
    EnableCleaningWarningLogsDirectoryForHabrEnvName,
    EnableCleaningWarningLogsDirectoryForMedrxivEnvName,
    EnableCleaningWarningLogsDirectoryForRedditEnvName,
    EnableCleaningWarningLogsDirectoryForTwitterEnvName,

    EnableCleaningWarningLogsDbInMongoForArxivEnvName,
    EnableCleaningWarningLogsDbInMongoForBiorxivEnvName,
    EnableCleaningWarningLogsDbInMongoForGithubEnvName,
    EnableCleaningWarningLogsDbInMongoForHabrEnvName,
    EnableCleaningWarningLogsDbInMongoForMedrxivEnvName,
    EnableCleaningWarningLogsDbInMongoForRedditEnvName,
    EnableCleaningWarningLogsDbInMongoForTwitterEnvName,

    EnableCleaningWarningLogsDbCollectionsInMongoForArxivEnvName,
    EnableCleaningWarningLogsDbCollectionsInMongoForBiorxivEnvName,
    EnableCleaningWarningLogsDbCollectionsInMongoForGithubEnvName,
    EnableCleaningWarningLogsDbCollectionsInMongoForHabrEnvName,
    EnableCleaningWarningLogsDbCollectionsInMongoForMedrxivEnvName,
    EnableCleaningWarningLogsDbCollectionsInMongoForRedditEnvName,
    EnableCleaningWarningLogsDbCollectionsInMongoForTwitterEnvName,

    EnableTimeMeasurementForArxivEnvName,
    EnableTimeMeasurementForBiorxivEnvName,
    EnableTimeMeasurementForGithubEnvName,
    EnableTimeMeasurementForHabrEnvName,
    EnableTimeMeasurementForMedrxivEnvName,
    EnableTimeMeasurementForRedditEnvName,
    EnableTimeMeasurementForTwitterEnvName,

    EnableInfoForArxivEnvName,
    EnableInfoForBiorxivEnvName,
    EnableInfoForGithubEnvName,
    EnableInfoForHabrEnvName,
    EnableInfoForMedrxivEnvName,
    EnableInfoForRedditEnvName,
    EnableInfoForTwitterEnvName,

    EnableLinksLimitForArxivEnvName,
    EnableLinksLimitForBiorxivEnvName,
    EnableLinksLimitForGithubEnvName,
    EnableLinksLimitForHabrEnvName,
    EnableLinksLimitForMedrxivEnvName,
    EnableLinksLimitForRedditEnvName,
    EnableLinksLimitForTwitterEnvName,

    EnableRandomizeOrderForArxivLinkPartsForMongoEnvName,
    EnableRandomizeOrderForBiorxivLinkPartsForMongoEnvName,
    EnableRandomizeOrderForGithubLinkPartsForMongoEnvName,
    EnableRandomizeOrderForHabrLinkPartsForMongoEnvName,
    EnableRandomizeOrderForMedrxivLinkPartsForMongoEnvName,
    EnableRandomizeOrderForRedditLinkPartsForMongoEnvName,
    EnableRandomizeOrderForTwitterLinkPartsForMongoEnvName,

    LinksLimitForArxivEnvName,
    LinksLimitForBiorxivEnvName,
    LinksLimitForGithubEnvName,
    LinksLimitForHabrEnvName,
    LinksLimitForMedrxivEnvName,
    LinksLimitForRedditEnvName,
    LinksLimitForTwitterEnvName,

    ErrorRedEnvName,
    ErrorGreenEnvName,
    ErrorBlueEnvName,
    WarningHighRedEnvName,
    WarningHighGreenEnvName,
    WarningHighBlueEnvName,
    WarningLowRedEnvName,
    WarningLowGreenEnvName,
    WarningLowBlueEnvName,
    SuccessRedEnvName,
    SuccessGreenEnvName,
    SuccessBlueEnvName,
    PartialSuccessRedEnvName,
    PartialSuccessGreenEnvName,
    PartialSuccessBlueEnvName,
    CleaningRedEnvName,
    CleaningGreenEnvName,
    CleaningBlueEnvName,
    TimeMeasurementRedEnvName,
    TimeMeasurementGreenEnvName,
    TimeMeasurementBlueEnvName,
    InfoRedEnvName,
    InfoGreenEnvName,
    InfoBlueEnvName,
}

// pub enum ALL ENV VARS NAMES
// HABR_NAME_TO_CHECKand Hashtable

impl EnvVarNameKind {
    // pub fn new() -> Self {

    // }
    pub fn get_string_name(env_var_name_kind: EnvVarNameKind) -> &'static str {
        match env_var_name_kind {
            EnvVarNameKind::GithubNameEnvName => GITHUB_NAME_ENV_NAME,
            EnvVarNameKind::GithubTokenEnvName => GITHUB_TOKEN_ENV_NAME,

            EnvVarNameKind::RedditUserAgentEnvName => REDDIT_USER_AGENT_ENV_NAME,
            EnvVarNameKind::RedditClientIdEnvName => REDDIT_CLIENT_ID_ENV_NAME,
            EnvVarNameKind::RedditClientSecretEnvName => REDDIT_CLIENT_SECRET_ENV_NAME,
            EnvVarNameKind::RedditUsernameEnvName => REDDIT_USERNAME_ENV_NAME,
            EnvVarNameKind::RedditPasswordEnvName => REDDIT_PASSWORD_ENV_NAME,

            EnvVarNameKind::StartingCheckLinkEnvName => STARTING_CHECK_LINK_ENV_NAME,
            EnvVarNameKind::UserCredentialsDummyHandleEnvName => USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME,
            EnvVarNameKind::WarningLogsDirectoryNameEnvName => WARNING_LOGS_DIRECTORY_NAME_ENV_NAME,
            EnvVarNameKind::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDirEnvName => UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME,
            EnvVarNameKind::EnableProvidersEnvName => ENABLE_PROVIDERS_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDirectoryEnvName => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbInMongoEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbCollectionsInMongoEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME,
            EnvVarNameKind::EnableTimeMeasurementEnvName => ENABLE_TIME_MEASUREMENT_ENV_NAME,
            EnvVarNameKind::EnableProviderLinksLimitEnvName => ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME,
            EnvVarNameKind::EnableCommonProvidersLinksLimitEnvName => ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME,
            EnvVarNameKind::CommonProvidersLinksLimitEnvName => COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME,
            EnvVarNameKind::EnableRandomizeOrderForProvidersLinkPartsForMongoEnvName => ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarNameKind::EnablePrintsEnvName => ENABLE_PRINTS_ENV_NAME,
            EnvVarNameKind::EnableErrorPrintsEnvName => ENABLE_ERROR_PRINTS_ENV_NAME,
            EnvVarNameKind::EnableWarningHighPrintsEnvName => ENABLE_WARNING_HIGH_PRINTS_ENV_NAME,
            EnvVarNameKind::EnableWarningLowPrintsEnvName => ENABLE_WARNING_LOW_PRINTS_ENV_NAME,
            EnvVarNameKind::EnableSuccessPrintsEnvName => ENABLE_SUCCESS_PRINTS_ENV_NAME,
            EnvVarNameKind::EnablePartialSuccessPrintsEnvName => ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME,
            EnvVarNameKind::EnableTimeMeasurementPrintsEnvName => ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDirectoryPrintsEnvName => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME,
            EnvVarNameKind::EnableInfoPrintsEnvName => ENABLE_INFO_PRINTS_ENV_NAME,
            EnvVarNameKind::EnableAllProvidersPrintsEnvName => ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME,
            EnvVarNameKind::EnableErrorPrintsForAllProvidersEnvName => ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarNameKind::EnableWarningHighPrintsForAllProvidersEnvName => ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarNameKind::EnableWarningLowPrintsForAllProvidersEnvName => ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarNameKind::EnableSuccessPrintsForAllProvidersEnvName => ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarNameKind::EnablePartialSuccessPrintsForAllProvidersEnvName => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarNameKind::EnableTimeMeasurementPrintsForAllProvidersEnvName => ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDirectoryPrintsForAllProvidersEnvName => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarNameKind::EnableInfoPrintsForAllProvidersEnvName => ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvVarNameKind::EnableWriteErrorLogsInLocalFolderEnvName => ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME,
            EnvVarNameKind::EnableWriteErrorLogsInMongoEnvName => ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME,
            EnvVarNameKind::EnableInitializeMongoWithProvidersLinkPartsEnvName => ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME,

            EnvVarNameKind::ProvidersDbNameHandleEnvName => PROVIDERS_DB_NAME_HANDLE_ENV_NAME,
            EnvVarNameKind::ProvidersDbCollectionHandleSecondPartEnvName => PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME,
            EnvVarNameKind::ProvidersDbCollectionDocumentFieldNameHandleEnvName => PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME,
            EnvVarNameKind::PathToProviderLinkPartsFolderEnvName => PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME,
            EnvVarNameKind::LogFileExtensionEnvName => LOG_FILE_EXTENSION_ENV_NAME,
            EnvVarNameKind::DbProvidersLogsNameHandleEnvName => DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME,
            EnvVarNameKind::DbProvidersLogsCollectionHandleSecondPartEnvName => DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME,
            EnvVarNameKind::DbProvidersLogsCollectionDocumentFieldNameHandleEnvName => DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME,

            EnvVarNameKind::EnableInitializeMongoWithArxivLinkPartsEnvName => ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME,
            EnvVarNameKind::EnableInitializeMongoWithBiorxivLinkPartsEnvName => ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME,
            EnvVarNameKind::EnableInitializeMongoWithGithubLinkPartsEnvName => ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME,
            EnvVarNameKind::EnableInitializeMongoWithHabrLinkPartsEnvName => ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME,
            EnvVarNameKind::EnableInitializeMongoWithMedrxivLinkPartsEnvName => ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME,
            EnvVarNameKind::EnableInitializeMongoWithRedditLinkPartsEnvName => ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME,
            EnvVarNameKind::EnableInitializeMongoWithTwitterLinkPartsEnvName => ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME,

            EnvVarNameKind::MongoFirstHandleUrlPartEnvName => MONGO_FIRST_HANDLE_URL_PART_ENV_NAME,
            EnvVarNameKind::MongoSecondHandleUrlPartEnvName => MONGO_SECOND_HANDLE_URL_PART_ENV_NAME,
            EnvVarNameKind::MongoThirdHandleUrlPartEnvName => MONGO_THIRD_HANDLE_URL_PART_ENV_NAME,
            EnvVarNameKind::MongoFourthHandleUrlPartEnvName => MONGO_FOURTH_HANDLE_URL_PART_ENV_NAME,
            EnvVarNameKind::MongoFifthHandleUrlPartEnvName => MONGO_FIFTH_HANDLE_URL_PART_ENV_NAME,

            EnvVarNameKind::MongoLoginEnvName => MONGO_LOGIN_ENV_NAME,
            EnvVarNameKind::MongoPasswordEnvName => MONGO_PASSWORD_ENV_NAME,
            EnvVarNameKind::MongoIpEnvName => MONGO_IP_ENV_NAME,
            EnvVarNameKind::MongoPortEnvName => MONGO_PORT_ENV_NAME,
            EnvVarNameKind::MongoParamsEnvName => MONGO_PARAMS_ENV_NAME,

            EnvVarNameKind::PostgresFirstHandleUrlPartEnvName => POSTGRES_FIRST_HANDLE_URL_PART_ENV_NAME,
            EnvVarNameKind::PostgresSecondHandleUrlPartEnvName => POSTGRES_SECOND_HANDLE_URL_PART_ENV_NAME,
            EnvVarNameKind::PostgresThirdHandleUrlPartEnvName => POSTGRES_THIRD_HANDLE_URL_PART_ENV_NAME,
            EnvVarNameKind::PostgresFourthHandleUrlPartEnvName => POSTGRES_FOURTH_HANDLE_URL_PART_ENV_NAME,
            EnvVarNameKind::PostgresFifthHandleUrlPartEnvName => POSTGRES_FIFTH_HANDLE_URL_PART_ENV_NAME,

            EnvVarNameKind::PostgresLoginEnvName => POSTGRES_LOGIN_ENV_NAME,
            EnvVarNameKind::PostgresPasswordEnvName => POSTGRES_PASSWORD_ENV_NAME,
            EnvVarNameKind::PostgresIpEnvName => POSTGRES_IP_ENV_NAME,
            EnvVarNameKind::PostgresPortEnvName => POSTGRES_PORT_ENV_NAME,
            EnvVarNameKind::PostgresDbEnvName => POSTGRES_DB_ENV_NAME,

            EnvVarNameKind::EnableArxivEnvName => ENABLE_ARXIV_ENV_NAME,
            EnvVarNameKind::EnableBiorxivEnvName => ENABLE_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnableGithubEnvName => ENABLE_GITHUB_ENV_NAME,
            EnvVarNameKind::EnableHabrEnvName => ENABLE_HABR_ENV_NAME,
            EnvVarNameKind::EnableMedrxivEnvName => ENABLE_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnableRedditEnvName => ENABLE_REDDIT_ENV_NAME,
            EnvVarNameKind::EnableTwitterEnvName => ENABLE_TWITTER_ENV_NAME,

            EnvVarNameKind::ArxivLinkEnvName => ARXIV_LINK_ENV_NAME,
            EnvVarNameKind::BiorxivLinkEnvName => BIORXIV_LINK_ENV_NAME,
            EnvVarNameKind::GithubLinkEnvName => GITHUB_LINK_ENV_NAME,
            EnvVarNameKind::HabrLinkEnvName => HABR_LINK_ENV_NAME,
            EnvVarNameKind::MedrxivLinkEnvName => MEDRXIV_LINK_ENV_NAME,
            EnvVarNameKind::RedditLinkEnvName => REDDIT_LINK_ENV_NAME,
            EnvVarNameKind::TwitterLinkEnvName => TWITTER_LINK_ENV_NAME,

            EnvVarNameKind::EnablePrintsArxivEnvName => ENABLE_PRINTS_ARXIV_ENV_NAME,
            EnvVarNameKind::EnablePrintsBiorxivEnvName => ENABLE_PRINTS_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnablePrintsGithubEnvName => ENABLE_PRINTS_GITHUB_ENV_NAME,
            EnvVarNameKind::EnablePrintsHabrEnvName => ENABLE_PRINTS_HABR_ENV_NAME,
            EnvVarNameKind::EnablePrintsMedrxivEnvName => ENABLE_PRINTS_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnablePrintsRedditEnvName => ENABLE_PRINTS_REDDIT_ENV_NAME,
            EnvVarNameKind::EnablePrintsTwitterEnvName => ENABLE_PRINTS_TWITTER_ENV_NAME,

            EnvVarNameKind::EnableWarningHighPrintsForArxivEnvName => ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVarNameKind::EnableWarningHighPrintsForBiorxivEnvName => ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnableWarningHighPrintsForGithubEnvName => ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVarNameKind::EnableWarningHighPrintsForHabrEnvName => ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME,
            EnvVarNameKind::EnableWarningHighPrintsForMedrxivEnvName => ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnableWarningHighPrintsForRedditEnvName => ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVarNameKind::EnableWarningHighPrintsForTwitterEnvName => ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVarNameKind::EnableWarningLowPrintsForArxivEnvName => ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVarNameKind::EnableWarningLowPrintsForBiorxivEnvName => ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnableWarningLowPrintsForGithubEnvName => ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVarNameKind::EnableWarningLowPrintsForHabrEnvName => ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME,
            EnvVarNameKind::EnableWarningLowPrintsForMedrxivEnvName => ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnableWarningLowPrintsForRedditEnvName => ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVarNameKind::EnableWarningLowPrintsForTwitterEnvName => ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVarNameKind::EnableErrorPrintsForArxivEnvName => ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVarNameKind::EnableErrorPrintsForBiorxivEnvName => ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnableErrorPrintsForGithubEnvName => ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVarNameKind::EnableErrorPrintsForHabrEnvName => ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME,
            EnvVarNameKind::EnableErrorPrintsForMedrxivEnvName => ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnableErrorPrintsForRedditEnvName => ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVarNameKind::EnableErrorPrintsForTwitterEnvName => ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVarNameKind::EnableSuccessPrintsForArxivEnvName => ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVarNameKind::EnableSuccessPrintsForBiorxivEnvName => ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnableSuccessPrintsForGithubEnvName => ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVarNameKind::EnableSuccessPrintsForHabrEnvName => ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME,
            EnvVarNameKind::EnableSuccessPrintsForMedrxivEnvName => ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnableSuccessPrintsForRedditEnvName => ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVarNameKind::EnableSuccessPrintsForTwitterEnvName => ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVarNameKind::EnablePartialSuccessPrintsForArxivEnvName => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVarNameKind::EnablePartialSuccessPrintsForBiorxivEnvName => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnablePartialSuccessPrintsForGithubEnvName => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvVarNameKind::EnablePartialSuccessPrintsForHabrEnvName => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME,
            EnvVarNameKind::EnablePartialSuccessPrintsForMedrxivEnvName => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnablePartialSuccessPrintsForRedditEnvName => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvVarNameKind::EnablePartialSuccessPrintsForTwitterEnvName => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvVarNameKind::EnableCleaningWarningLogsDirectoryForArxivEnvName => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDirectoryForBiorxivEnvName => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDirectoryForGithubEnvName => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDirectoryForHabrEnvName => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDirectoryForMedrxivEnvName => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDirectoryForRedditEnvName => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDirectoryForTwitterEnvName => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME,

            EnvVarNameKind::EnableCleaningWarningLogsDbInMongoForArxivEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbInMongoForBiorxivEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbInMongoForGithubEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbInMongoForHabrEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbInMongoForMedrxivEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbInMongoForRedditEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbInMongoForTwitterEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME,

            EnvVarNameKind::EnableCleaningWarningLogsDbCollectionsInMongoForArxivEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxivEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbCollectionsInMongoForGithubEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbCollectionsInMongoForHabrEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxivEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbCollectionsInMongoForRedditEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME,
            EnvVarNameKind::EnableCleaningWarningLogsDbCollectionsInMongoForTwitterEnvName => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME,

            EnvVarNameKind::EnableTimeMeasurementForArxivEnvName => ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME,
            EnvVarNameKind::EnableTimeMeasurementForBiorxivEnvName => ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnableTimeMeasurementForGithubEnvName => ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME,
            EnvVarNameKind::EnableTimeMeasurementForHabrEnvName => ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME,
            EnvVarNameKind::EnableTimeMeasurementForMedrxivEnvName => ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnableTimeMeasurementForRedditEnvName => ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME,
            EnvVarNameKind::EnableTimeMeasurementForTwitterEnvName => ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME,

            EnvVarNameKind::EnableInfoForArxivEnvName => ENABLE_INFO_FOR_ARXIV_ENV_NAME,
            EnvVarNameKind::EnableInfoForBiorxivEnvName => ENABLE_INFO_FOR_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnableInfoForGithubEnvName => ENABLE_INFO_FOR_GITHUB_ENV_NAME,
            EnvVarNameKind::EnableInfoForHabrEnvName => ENABLE_INFO_FOR_HABR_ENV_NAME,
            EnvVarNameKind::EnableInfoForMedrxivEnvName => ENABLE_INFO_FOR_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnableInfoForRedditEnvName => ENABLE_INFO_FOR_REDDIT_ENV_NAME,
            EnvVarNameKind::EnableInfoForTwitterEnvName => ENABLE_INFO_FOR_TWITTER_ENV_NAME,

            EnvVarNameKind::EnableLinksLimitForArxivEnvName => ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME,
            EnvVarNameKind::EnableLinksLimitForBiorxivEnvName => ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME,
            EnvVarNameKind::EnableLinksLimitForGithubEnvName => ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME,
            EnvVarNameKind::EnableLinksLimitForHabrEnvName => ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME,
            EnvVarNameKind::EnableLinksLimitForMedrxivEnvName => ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME,
            EnvVarNameKind::EnableLinksLimitForRedditEnvName => ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME,
            EnvVarNameKind::EnableLinksLimitForTwitterEnvName => ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME,

            EnvVarNameKind::EnableRandomizeOrderForArxivLinkPartsForMongoEnvName => ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarNameKind::EnableRandomizeOrderForBiorxivLinkPartsForMongoEnvName => ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarNameKind::EnableRandomizeOrderForGithubLinkPartsForMongoEnvName => ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarNameKind::EnableRandomizeOrderForHabrLinkPartsForMongoEnvName => ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarNameKind::EnableRandomizeOrderForMedrxivLinkPartsForMongoEnvName => ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarNameKind::EnableRandomizeOrderForRedditLinkPartsForMongoEnvName => ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvVarNameKind::EnableRandomizeOrderForTwitterLinkPartsForMongoEnvName => ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME,

            EnvVarNameKind::LinksLimitForArxivEnvName => LINKS_LIMIT_FOR_ARXIV_ENV_NAME,
            EnvVarNameKind::LinksLimitForBiorxivEnvName => LINKS_LIMIT_FOR_BIORXIV_ENV_NAME,
            EnvVarNameKind::LinksLimitForGithubEnvName => LINKS_LIMIT_FOR_GITHUB_ENV_NAME,
            EnvVarNameKind::LinksLimitForHabrEnvName => LINKS_LIMIT_FOR_HABR_ENV_NAME,
            EnvVarNameKind::LinksLimitForMedrxivEnvName => LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME,
            EnvVarNameKind::LinksLimitForRedditEnvName => LINKS_LIMIT_FOR_REDDIT_ENV_NAME,
            EnvVarNameKind::LinksLimitForTwitterEnvName => LINKS_LIMIT_FOR_TWITTER_ENV_NAME,

            EnvVarNameKind::ErrorRedEnvName => ERROR_RED_ENV_NAME,
            EnvVarNameKind::ErrorGreenEnvName => ERROR_GREEN_ENV_NAME,
            EnvVarNameKind::ErrorBlueEnvName => ERROR_BLUE_ENV_NAME,
            EnvVarNameKind::WarningHighRedEnvName => WARNING_HIGH_RED_ENV_NAME,
            EnvVarNameKind::WarningHighGreenEnvName => WARNING_HIGH_GREEN_ENV_NAME,
            EnvVarNameKind::WarningHighBlueEnvName => WARNING_HIGH_BLUE_ENV_NAME,
            EnvVarNameKind::WarningLowRedEnvName => WARNING_LOW_RED_ENV_NAME,
            EnvVarNameKind::WarningLowGreenEnvName => WARNING_LOW_GREEN_ENV_NAME,
            EnvVarNameKind::WarningLowBlueEnvName => WARNING_LOW_BLUE_ENV_NAME,
            EnvVarNameKind::SuccessRedEnvName => SUCCESS_RED_ENV_NAME,
            EnvVarNameKind::SuccessGreenEnvName => SUCCESS_GREEN_ENV_NAME,
            EnvVarNameKind::SuccessBlueEnvName => SUCCESS_BLUE_ENV_NAME,
            EnvVarNameKind::PartialSuccessRedEnvName => PARTIAL_SUCCESS_RED_ENV_NAME,
            EnvVarNameKind::PartialSuccessGreenEnvName => PARTIAL_SUCCESS_GREEN_ENV_NAME,
            EnvVarNameKind::PartialSuccessBlueEnvName => PARTIAL_SUCCESS_BLUE_ENV_NAME,
            EnvVarNameKind::CleaningRedEnvName => CLEANING_RED_ENV_NAME,
            EnvVarNameKind::CleaningGreenEnvName => CLEANING_GREEN_ENV_NAME,
            EnvVarNameKind::CleaningBlueEnvName => CLEANING_BLUE_ENV_NAME,
            EnvVarNameKind::TimeMeasurementRedEnvName => TIME_MEASUREMENT_RED_ENV_NAME,
            EnvVarNameKind::TimeMeasurementGreenEnvName => TIME_MEASUREMENT_GREEN_ENV_NAME,
            EnvVarNameKind::TimeMeasurementBlueEnvName => TIME_MEASUREMENT_BLUE_ENV_NAME,
            EnvVarNameKind::InfoRedEnvName => INFO_RED_ENV_NAME,
            EnvVarNameKind::InfoGreenEnvName => INFO_GREEN_ENV_NAME,
            EnvVarNameKind::InfoBlueEnvName => INFO_BLUE_ENV_NAME, 
        }
    }
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    pub fn into_vec() -> Vec<EnvVarNameKind> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvVarNameKind::get_length());
        for env_var_name_kind in EnvVarNameKind::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvVarNameKind)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvVarNameKind::get_length());
        for env_var_name_kind in EnvVarNameKind::iter() {
            env_var_name_kind_vec.push((EnvVarNameKind::get_string_name(env_var_name_kind),   env_var_name_kind));
        }
        env_var_name_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvVarNameKind> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<&'static str, EnvVarNameKind> =
        HashMap::with_capacity(EnvVarNameKind::get_length());
        for env_var_name_kind_kind in EnvVarNameKind::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(EnvVarNameKind::get_string_name(env_var_name_kind_kind),   env_var_name_kind_kind);
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
}




//     config_provider_string_to_enum_struct_hasmap

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