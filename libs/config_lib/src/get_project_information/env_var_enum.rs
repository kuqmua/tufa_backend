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

use crate::get_project_information::get_config::github_authorization_struct::GithubAuthorization;
use crate::get_project_information::get_config::mongo_authorization_struct::MongoAuthorization;
use crate::get_project_information::get_config::postgres_authorization_struct::PostgresAuthorization;
use crate::get_project_information::get_config::reddit_authorization_struct::RedditAuthorization;

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

pub enum EnvVarsEnum {
    GithubNameEnvName (String),
    GithubTokenEnvName (String),

    RedditUserAgentEnvName (String),
    RedditClientIdEnvName (String),
    RedditClientSecretEnvName (String),
    RedditUsernameEnvName (String),
    RedditPasswordEnvName (String),

    StartingCheckLinkEnvName (String),
    UserCredentialsDummyHandleEnvName (String),
    WarningLogsDirectoryNameEnvName (String),
    UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDirEnvName (String),
    EnableProvidersEnvName (bool),
    EnableCleaningWarningLogsDirectoryEnvName (bool),
    EnableCleaningWarningLogsDbInMongoEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoEnvName (bool),
    EnableTimeMeasurementEnvName (bool),
    EnableProviderLinksLimitEnvName (bool),
    EnableCommonProvidersLinksLimitEnvName (bool),
    CommonProvidersLinksLimitEnvName (i64),
    EnableRandomizeOrderForProvidersLinkPartsForMongoEnvName (bool),
    EnablePrintsEnvName (bool),
    EnableErrorPrintsEnvName (bool),
    EnableWarningHighPrintsEnvName (bool),
    EnableWarningLowPrintsEnvName (bool),
    EnableSuccessPrintsEnvName (bool),
    EnablePartialSuccessPrintsEnvName (bool),
    EnableTimeMeasurementPrintsEnvName (bool),
    EnableCleaningWarningLogsDirectoryPrintsEnvName (bool),
    EnableInfoPrintsEnvName (bool),
    EnableAllProvidersPrintsEnvName (bool),
    EnableErrorPrintsForAllProvidersEnvName (bool),
    EnableWarningHighPrintsForAllProvidersEnvName (bool),
    EnableWarningLowPrintsForAllProvidersEnvName (bool),
    EnableSuccessPrintsForAllProvidersEnvName (bool),
    EnablePartialSuccessPrintsForAllProvidersEnvName (bool),
    EnableTimeMeasurementPrintsForAllProvidersEnvName (bool),
    EnableCleaningWarningLogsDirectoryPrintsForAllProvidersEnvName (bool),
    EnableInfoPrintsForAllProvidersEnvName (bool),
    EnableWriteErrorLogsInLocalFolderEnvName (bool),
    EnableWriteErrorLogsInMongoEnvName (bool),
    EnableInitializeMongoWithProvidersLinkPartsEnvName (bool),

    ProvidersDbNameHandleEnvName (String),
    ProvidersDbCollectionHandleSecondPartEnvName (String),
    ProvidersDbCollectionDocumentFieldNameHandleEnvName (String),
    PathToProviderLinkPartsFolderEnvName (String),
    LogFileExtensionEnvName (String),
    DbProvidersLogsNameHandleEnvName (String),
    DbProvidersLogsCollectionHandleSecondPartEnvName (String),
    DbProvidersLogsCollectionDocumentFieldNameHandleEnvName (String),

    EnableInitializeMongoWithArxivLinkPartsEnvName (bool),
    EnableInitializeMongoWithBiorxivLinkPartsEnvName (bool),
    EnableInitializeMongoWithGithubLinkPartsEnvName (bool),
    EnableInitializeMongoWithHabrLinkPartsEnvName (bool),
    EnableInitializeMongoWithMedrxivLinkPartsEnvName (bool),
    EnableInitializeMongoWithRedditLinkPartsEnvName (bool),
    EnableInitializeMongoWithTwitterLinkPartsEnvName (bool),

    MongoFirstHandleUrlPartEnvName (String),
    MongoSecondHandleUrlPartEnvName (String),
    MongoThirdHandleUrlPartEnvName (String),
    MongoFourthHandleUrlPartEnvName (String),
    MongoFifthHandleUrlPartEnvName (String),

    MongoLoginEnvName (String),
    MongoPasswordEnvName (String),
    MongoIpEnvName (String),
    MongoPortEnvName (String),
    MongoParamsEnvName (String),

    PostgresFirstHandleUrlPartEnvName (String),
    PostgresSecondHandleUrlPartEnvName (String),
    PostgresThirdHandleUrlPartEnvName (String),
    PostgresFourthHandleUrlPartEnvName (String),
    PostgresFifthHandleUrlPartEnvName (String),

    PostgresLoginEnvName (String),
    PostgresPasswordEnvName (String),
    PostgresIpEnvName (String),
    PostgresPortEnvName (String),
    PostgresDbEnvName (String),

    EnableArxivEnvName (bool),
    EnableBiorxivEnvName (bool),
    EnableGithubEnvName (bool),
    EnableHabrEnvName (bool),
    EnableMedrxivEnvName (bool),
    EnableRedditEnvName (bool),
    EnableTwitterEnvName (bool),

    ArxivLinkEnvName (String),
    BiorxivLinkEnvName (String),
    GithubLinkEnvName (String),
    HabrLinkEnvName (String),
    MedrxivLinkEnvName (String),
    RedditLinkEnvName (String),
    TwitterLinkEnvName (String),

    EnablePrintsArxivEnvName (bool),
    EnablePrintsBiorxivEnvName (bool),
    EnablePrintsGithubEnvName (bool),
    EnablePrintsHabrEnvName (bool),
    EnablePrintsMedrxivEnvName (bool),
    EnablePrintsRedditEnvName (bool),
    EnablePrintsTwitterEnvName (bool),

    EnableWarningHighPrintsForArxivEnvName (bool),
    EnableWarningHighPrintsForBiorxivEnvName (bool),
    EnableWarningHighPrintsForGithubEnvName (bool),
    EnableWarningHighPrintsForHabrEnvName (bool),
    EnableWarningHighPrintsForMedrxivEnvName (bool),
    EnableWarningHighPrintsForRedditEnvName (bool),
    EnableWarningHighPrintsForTwitterEnvName (bool),

    EnableWarningLowPrintsForArxivEnvName (bool),
    EnableWarningLowPrintsForBiorxivEnvName (bool),
    EnableWarningLowPrintsForGithubEnvName (bool),
    EnableWarningLowPrintsForHabrEnvName (bool),
    EnableWarningLowPrintsForMedrxivEnvName (bool),
    EnableWarningLowPrintsForRedditEnvName (bool),
    EnableWarningLowPrintsForTwitterEnvName (bool),

    EnableErrorPrintsForArxivEnvName (bool),
    EnableErrorPrintsForBiorxivEnvName (bool),
    EnableErrorPrintsForGithubEnvName (bool),
    EnableErrorPrintsForHabrEnvName (bool),
    EnableErrorPrintsForMedrxivEnvName (bool),
    EnableErrorPrintsForRedditEnvName (bool),
    EnableErrorPrintsForTwitterEnvName (bool),

    EnableSuccessPrintsForArxivEnvName (bool),
    EnableSuccessPrintsForBiorxivEnvName (bool),
    EnableSuccessPrintsForGithubEnvName (bool),
    EnableSuccessPrintsForHabrEnvName (bool),
    EnableSuccessPrintsForMedrxivEnvName (bool),
    EnableSuccessPrintsForRedditEnvName (bool),
    EnableSuccessPrintsForTwitterEnvName (bool),

    EnablePartialSuccessPrintsForArxivEnvName (bool),
    EnablePartialSuccessPrintsForBiorxivEnvName (bool),
    EnablePartialSuccessPrintsForGithubEnvName (bool),
    EnablePartialSuccessPrintsForHabrEnvName (bool),
    EnablePartialSuccessPrintsForMedrxivEnvName (bool),
    EnablePartialSuccessPrintsForRedditEnvName (bool),
    EnablePartialSuccessPrintsForTwitterEnvName (bool),

    EnableCleaningWarningLogsDirectoryForArxivEnvName (bool),
    EnableCleaningWarningLogsDirectoryForBiorxivEnvName (bool),
    EnableCleaningWarningLogsDirectoryForGithubEnvName (bool),
    EnableCleaningWarningLogsDirectoryForHabrEnvName (bool),
    EnableCleaningWarningLogsDirectoryForMedrxivEnvName (bool),
    EnableCleaningWarningLogsDirectoryForRedditEnvName (bool),
    EnableCleaningWarningLogsDirectoryForTwitterEnvName (bool),

    EnableCleaningWarningLogsDbInMongoForArxivEnvName (bool),
    EnableCleaningWarningLogsDbInMongoForBiorxivEnvName (bool),
    EnableCleaningWarningLogsDbInMongoForGithubEnvName (bool),
    EnableCleaningWarningLogsDbInMongoForHabrEnvName (bool),
    EnableCleaningWarningLogsDbInMongoForMedrxivEnvName (bool),
    EnableCleaningWarningLogsDbInMongoForRedditEnvName (bool),
    EnableCleaningWarningLogsDbInMongoForTwitterEnvName (bool),

    EnableCleaningWarningLogsDbCollectionsInMongoForArxivEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoForBiorxivEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoForGithubEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoForHabrEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoForMedrxivEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoForRedditEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoForTwitterEnvName (bool),

    EnableTimeMeasurementForArxivEnvName (bool),
    EnableTimeMeasurementForBiorxivEnvName (bool),
    EnableTimeMeasurementForGithubEnvName (bool),
    EnableTimeMeasurementForHabrEnvName (bool),
    EnableTimeMeasurementForMedrxivEnvName (bool),
    EnableTimeMeasurementForRedditEnvName (bool),
    EnableTimeMeasurementForTwitterEnvName (bool),

    EnableInfoForArxivEnvName (bool),
    EnableInfoForBiorxivEnvName (bool),
    EnableInfoForGithubEnvName (bool),
    EnableInfoForHabrEnvName (bool),
    EnableInfoForMedrxivEnvName (bool),
    EnableInfoForRedditEnvName (bool),
    EnableInfoForTwitterEnvName (bool),

    EnableLinksLimitForArxivEnvName (bool),
    EnableLinksLimitForBiorxivEnvName (bool),
    EnableLinksLimitForGithubEnvName (bool),
    EnableLinksLimitForHabrEnvName (bool),
    EnableLinksLimitForMedrxivEnvName (bool),
    EnableLinksLimitForRedditEnvName (bool),
    EnableLinksLimitForTwitterEnvName (bool),

    EnableRandomizeOrderForArxivLinkPartsForMongoEnvName (bool),
    EnableRandomizeOrderForBiorxivLinkPartsForMongoEnvName (bool),
    EnableRandomizeOrderForGithubLinkPartsForMongoEnvName (bool),
    EnableRandomizeOrderForHabrLinkPartsForMongoEnvName (bool),
    EnableRandomizeOrderForMedrxivLinkPartsForMongoEnvName (bool),
    EnableRandomizeOrderForRedditLinkPartsForMongoEnvName (bool),
    EnableRandomizeOrderForTwitterLinkPartsForMongoEnvName (bool),

    LinksLimitForArxivEnvName (i64),
    LinksLimitForBiorxivEnvName (i64),
    LinksLimitForGithubEnvName (i64),
    LinksLimitForHabrEnvName (i64),
    LinksLimitForMedrxivEnvName (i64),
    LinksLimitForRedditEnvName (i64),
    LinksLimitForTwitterEnvName (i64),

    ErrorRedEnvName (u8),
    ErrorGreenEnvName (u8),
    ErrorBlueEnvName (u8),
    WarningHighRedEnvName (u8),
    WarningHighGreenEnvName (u8),
    WarningHighBlueEnvName (u8),
    WarningLowRedEnvName (u8),
    WarningLowGreenEnvName (u8),
    WarningLowBlueEnvName (u8),
    SuccessRedEnvName (u8),
    SuccessGreenEnvName (u8),
    SuccessBlueEnvName (u8),
    PartialSuccessRedEnvName (u8),
    PartialSuccessGreenEnvName (u8),
    PartialSuccessBlueEnvName (u8),
    CleaningRedEnvName (u8),
    CleaningGreenEnvName (u8),
    CleaningBlueEnvName (u8),
    TimeMeasurementRedEnvName (u8),
    TimeMeasurementGreenEnvName (u8),
    TimeMeasurementBlueEnvName (u8),
    InfoRedEnvName (u8),
    InfoGreenEnvName (u8),
    InfoBlueEnvName (u8),
}

// pub enum ALL ENV VARS NAMES
// HABR_NAME_TO_CHECKand Hashtable

// impl EnvVarsEnum {
//     pub fn new() -> Self {

//     }
// }




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