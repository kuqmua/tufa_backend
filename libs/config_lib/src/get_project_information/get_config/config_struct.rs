extern crate toml;

use itertools::Itertools;

use dotenv::dotenv;

use crate::get_project_information::provider_kind_enum::ProviderKind;

use crate::get_project_information::project_constants::ENV_FILE_NAME;

use crate::get_project_information::get_config::enable_providers_struct::EnableProviders;
use crate::get_project_information::get_config::enable_providers_prints_struct::EnableProvidersPrints;
use crate::get_project_information::get_config::providers_check_links_struct::ProvidersCheckLinks;
use crate::get_project_information::get_config::mongo_params_struct::MongoParams;
use crate::get_project_information::get_config::postgres_params_struct::PostgresParams;
use crate::get_project_information::get_config::postgres_url_parts_struct::PostgresUrlParts;
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
use crate::get_project_information::get_config::enable_initialize_mongo_with_providers_link_parts_struct::EnableInitializeMongoWithProvidersLinkParts;
use crate::get_project_information::get_config::mongo_url_parts_struct::MongoUrlParts;

use crate::get_project_information::project_constants::ARXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::BIORXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::GITHUB_NAME_TO_CHECK;
use crate::get_project_information::project_constants::HABR_NAME_TO_CHECK;
use crate::get_project_information::project_constants::MEDRXIV_NAME_TO_CHECK;
use crate::get_project_information::project_constants::REDDIT_NAME_TO_CHECK;
use crate::get_project_information::project_constants::TWITTER_NAME_TO_CHECK;

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
use crate::get_project_information::project_constants::WARNING_LOGS_DIRECTORY_NAME_ENV_NAME;

// [mongo_params]
use crate::get_project_information::project_constants::DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME;
use crate::get_project_information::project_constants::DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME;
use crate::get_project_information::project_constants::DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::get_project_information::project_constants::LOG_FILE_EXTENSION_ENV_NAME;
use crate::get_project_information::project_constants::PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME;
use crate::get_project_information::project_constants::PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME;
use crate::get_project_information::project_constants::PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME;
use crate::get_project_information::project_constants::PROVIDERS_DB_NAME_HANDLE_ENV_NAME;

// [mongo_params.mongo_url_parts]
use crate::get_project_information::project_constants::MONGO_FIRST_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_FOURTH_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_SECOND_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_THIRD_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_FIFTH_HANDLE_URL_PART_ENV_NAME;

// [mongo_params.enable_initialize_mongo_with_providers_link_parts]
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME;
use crate::get_project_information::project_constants::ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME;

// [postgres_params]
use crate::get_project_information::project_constants::POSTGRES_FIRST_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_FOURTH_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_SECOND_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_THIRD_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_FIFTH_HANDLE_URL_PART_ENV_NAME;

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

use crate::get_project_information::get_config::github_authorization_struct::GithubAuthorization;
use crate::get_project_information::get_config::mongo_authorization_struct::MongoAuthorization;
use crate::get_project_information::get_config::postgres_authorization_struct::PostgresAuthorization;
use crate::get_project_information::get_config::reddit_authorization_struct::RedditAuthorization;

use crate::get_project_information::project_constants::GITHUB_NAME_ENV_NAME;
use crate::get_project_information::project_constants::GITHUB_TOKEN_ENV_NAME;

use crate::get_project_information::project_constants::MONGO_IP_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_LOGIN_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_PASSWORD_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_PORT_ENV_NAME;
use crate::get_project_information::project_constants::MONGO_PARAMS_ENV_NAME;


use crate::get_project_information::project_constants::POSTGRES_LOGIN_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_PASSWORD_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_IP_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_PORT_ENV_NAME;
use crate::get_project_information::project_constants::POSTGRES_DB_ENV_NAME;

use crate::get_project_information::project_constants::REDDIT_CLIENT_ID_ENV_NAME;
use crate::get_project_information::project_constants::REDDIT_CLIENT_SECRET_ENV_NAME;
use crate::get_project_information::project_constants::REDDIT_PASSWORD_ENV_NAME;
use crate::get_project_information::project_constants::REDDIT_USERNAME_ENV_NAME;
use crate::get_project_information::project_constants::REDDIT_USER_AGENT_ENV_NAME;

use crate::get_project_information::get_config::config_error::ConfigError;
use crate::get_project_information::get_config::config_error::VarOrBoolParseError;
use crate::get_project_information::get_config::config_error::VarOrIntParseError;

#[derive(Debug, Clone, PartialEq)] //Default,//serde_derive::Serialize, serde_derive::Deserialize
pub struct ConfigStruct {
    pub github_authorization: GithubAuthorization,
    pub reddit_authorization: RedditAuthorization,
    //
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
    pub fn new() -> Result<Self, ConfigError<'static>> {
        //todo: build without it
        let was_dotenv_enable: bool;
        match dotenv() {
            Ok(_) => {
                was_dotenv_enable = true;
            },
            Err(e) => {
                was_dotenv_enable = true;
                println!("dotenv() failed, trying without {} error: {:?}", ENV_FILE_NAME, e);
            }
        }
        let handle_config_github_authorization_github_name: String;
        match std::env::var(GITHUB_NAME_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_github_authorization_github_name = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::GithubNameError { was_dotenv_enable, env_name: GITHUB_NAME_ENV_NAME, env_error: e })
                    }
                }
        let handle_config_github_authorization_github_token: String;
        match std::env::var(GITHUB_TOKEN_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_github_authorization_github_token = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::GithubTokenError { was_dotenv_enable, env_name: GITHUB_NAME_ENV_NAME, env_error: e })
                    }
                }
        let handle_config_reddit_authorization_reddit_user_agent: String;
        match std::env::var(REDDIT_USER_AGENT_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_reddit_authorization_reddit_user_agent = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::RedditUserAgentError { was_dotenv_enable, env_name: REDDIT_USER_AGENT_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_reddit_authorization_reddit_client_id: String;
        match std::env::var(REDDIT_CLIENT_ID_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_reddit_authorization_reddit_client_id = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::RedditClientIdError { was_dotenv_enable, env_name: REDDIT_CLIENT_ID_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_reddit_authorization_reddit_client_secret: String;
        match std::env::var(REDDIT_CLIENT_SECRET_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_reddit_authorization_reddit_client_secret = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::RedditClientSecretError { was_dotenv_enable, env_name: REDDIT_CLIENT_SECRET_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_reddit_authorization_reddit_username: String;
        match std::env::var(REDDIT_USERNAME_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_reddit_authorization_reddit_username = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::RedditUsernameError { was_dotenv_enable, env_name: REDDIT_USERNAME_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_reddit_authorization_reddit_password: String;
        match std::env::var(REDDIT_PASSWORD_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_reddit_authorization_reddit_password = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::RedditPasswordError { was_dotenv_enable, env_name: REDDIT_PASSWORD_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_params_starting_check_link: String;
        match std::env::var(STARTING_CHECK_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config_params_starting_check_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::StartingCheckLinkError { was_dotenv_enable, env_name: STARTING_CHECK_LINK_ENV_NAME, env_error: e });
            }
        }
        let handle_config_params_user_credentials_dummy_handle: String;
        match std::env::var(USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME) {
            Ok(handle) => {
                handle_config_params_user_credentials_dummy_handle = handle;
            }
            Err(e) => {
                return Err(ConfigError::UserCredentialsDummyHandleError { was_dotenv_enable, env_name: USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME, env_error: e });
            }
        }
        let handle_config_params_warning_logs_directory_name: String;
        match std::env::var(WARNING_LOGS_DIRECTORY_NAME_ENV_NAME) {
            Ok(handle) => {
                handle_config_params_warning_logs_directory_name = handle;
            }
            Err(e) => {
                return Err(ConfigError::WarningLogsDirectoryNameError { was_dotenv_enable, env_name: WARNING_LOGS_DIRECTORY_NAME_ENV_NAME, env_error: e });
            }
        }
        let handle_config_params_unhandled_success_handled_success_are_there_items_initialized_posts_dir: String;
        match std::env::var(
            UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME,
        ) {
            Ok(handle) => {
                handle_config_params_unhandled_success_handled_success_are_there_items_initialized_posts_dir =
                    handle;
            }
            Err(e) => {
                return Err(ConfigError::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDirError { was_dotenv_enable, env_name: UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME, env_error: e });
            }
        }
        let handle_config_params_enable_providers: bool;
        match std::env::var(ENABLE_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableProvidersError {
                        was_dotenv_enable,
                        env_name: ENABLE_PROVIDERS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableProvidersError { 
                    was_dotenv_enable, 
                    env_name: ENABLE_PROVIDERS_ENV_NAME, 
                    env_error: VarOrBoolParseError::Var(e), 
                });
            }
        }
        let handle_config_params_enable_cleaning_warning_logs_directory: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_cleaning_warning_logs_directory = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDirectoryError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDirectoryError { 
                    was_dotenv_enable, 
                    env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME, 
                    env_error: VarOrBoolParseError::Var(e), 
                });
            }
        }
        let handle_config_params_enable_cleaning_warning_logs_db_in_mongo: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_cleaning_warning_logs_db_in_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbInMongoError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbInMongoError { 
                    was_dotenv_enable, 
                    env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME, 
                    env_error: VarOrBoolParseError::Var(e), 
                 });
            }
        }
        let handle_config_params_enable_cleaning_warning_logs_db_collections_in_mongo: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_cleaning_warning_logs_db_collections_in_mongo =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoError { 
                    was_dotenv_enable, 
                    env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME, 
                    env_error: VarOrBoolParseError::Var(e), 
                });
            }
        }
        let handle_config_params_enable_time_measurement: bool;
        match std::env::var(ENABLE_TIME_MEASUREMENT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_time_measurement = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableTimeMeasurementError {
                        was_dotenv_enable,
                        env_name: ENABLE_TIME_MEASUREMENT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableTimeMeasurementError { was_dotenv_enable, env_name: ENABLE_TIME_MEASUREMENT_ENV_NAME, env_error: VarOrBoolParseError::Var(e),  });
            }
        }
        let handle_config_params_enable_provider_links_limit: bool;
        match std::env::var(ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_provider_links_limit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableProviderLinksLimitError {
                        was_dotenv_enable,
                        env_name: ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableProviderLinksLimitError { 
                    was_dotenv_enable, 
                    env_name: ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME, 
                    env_error: VarOrBoolParseError::Var(e), 
                });
            }
        }
        let handle_config_params_enable_common_providers_links_limit: bool;
        match std::env::var(ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_common_providers_links_limit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCommonProvidersLinksLimitError {
                        was_dotenv_enable,
                        env_name: ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCommonProvidersLinksLimitError { 
                    was_dotenv_enable, 
                    env_name: ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e),  });
            }
        }
        let handle_config_params_common_providers_links_limit: i64;
        match std::env::var(COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config_params_common_providers_links_limit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::CommonProvidersLinksLimitError {
                        was_dotenv_enable,
                        env_name: COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::CommonProvidersLinksLimitError { was_dotenv_enable, env_name: COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME, env_error: VarOrIntParseError::Var(e),  });
            }
        }
        let handle_config_params_enable_randomize_order_for_providers_link_parts_for_mongo: bool;
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_randomize_order_for_providers_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableRandomizeOrderForProvidersLinkPartsForMongoError {
                        was_dotenv_enable,
                        env_name: ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableRandomizeOrderForProvidersLinkPartsForMongoError { was_dotenv_enable, env_name: ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_prints: bool;
        match std::env::var(ENABLE_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePrintsError {
                        was_dotenv_enable,
                        env_name: ENABLE_PRINTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePrintsError { was_dotenv_enable, env_name: ENABLE_PRINTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_error_prints: bool;
        match std::env::var(ENABLE_ERROR_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_error_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableErrorPrintsError {
                        was_dotenv_enable,
                        env_name: ENABLE_ERROR_PRINTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableErrorPrintsError { was_dotenv_enable, env_name: ENABLE_ERROR_PRINTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_warning_high_prints: bool;
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_warning_high_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningHighPrintsError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_HIGH_PRINTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningHighPrintsError { was_dotenv_enable, env_name: ENABLE_WARNING_HIGH_PRINTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_warning_low_prints: bool;
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_warning_low_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningLowPrintsError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_LOW_PRINTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningLowPrintsError { was_dotenv_enable, env_name: ENABLE_WARNING_LOW_PRINTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_success_prints: bool;
        match std::env::var(ENABLE_SUCCESS_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_success_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableSuccessPrintsError {
                        was_dotenv_enable,
                        env_name: ENABLE_SUCCESS_PRINTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableSuccessPrintsError { was_dotenv_enable, env_name: ENABLE_SUCCESS_PRINTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_partial_success_prints: bool;
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_partial_success_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePartialSuccessPrintsError {
                        was_dotenv_enable,
                        env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePartialSuccessPrintsError { was_dotenv_enable, env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_time_measurement_prints: bool;
        match std::env::var(ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_time_measurement_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableTimeMeasurementPrintsError {
                        was_dotenv_enable,
                        env_name: ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableTimeMeasurementPrintsError { was_dotenv_enable, env_name: ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_cleaning_warning_logs_directory_prints: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_cleaning_warning_logs_directory_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDirectoryPrintsError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDirectoryPrintsError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_info_prints: bool;
        match std::env::var(ENABLE_INFO_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_info_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInfoPrintsError {
                        was_dotenv_enable,
                        env_name: ENABLE_INFO_PRINTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInfoPrintsError { was_dotenv_enable, env_name: ENABLE_INFO_PRINTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_all_providers_prints: bool;
        match std::env::var(ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_all_providers_prints = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableAllProvidersPrintsError {
                        was_dotenv_enable,
                        env_name: ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableAllProvidersPrintsError { was_dotenv_enable, env_name: ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_error_prints_for_all_providers: bool;
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_error_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableErrorPrintsForAllProvidersError {
                        was_dotenv_enable,
                        env_name: ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableErrorPrintsForAllProvidersError { was_dotenv_enable, env_name: ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_warning_high_prints_for_all_providers: bool;
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_warning_high_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningHighPrintsForAllProvidersError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningHighPrintsForAllProvidersError { was_dotenv_enable, env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_warning_low_prints_for_all_providers: bool;
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_warning_low_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningLowPrintsForAllProvidersError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningLowPrintsForAllProvidersError { was_dotenv_enable, env_name: ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_success_prints_for_all_providers: bool;
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_success_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableSuccessPrintsForAllProvidersError {
                        was_dotenv_enable,
                        env_name: ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableSuccessPrintsForAllProvidersError { was_dotenv_enable, env_name: ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_partial_success_prints_for_all_providers: bool;
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_partial_success_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePartialSuccessPrintsForAllProvidersError {
                        was_dotenv_enable,
                        env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePartialSuccessPrintsForAllProvidersError { was_dotenv_enable, env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_time_measurement_prints_for_all_providers: bool;
        match std::env::var(ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_time_measurement_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableTimeMeasurementPrintsForAllProvidersError {
                        was_dotenv_enable,
                        env_name: ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableTimeMeasurementPrintsForAllProvidersError { was_dotenv_enable, env_name: ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_cleaning_warning_logs_directory_prints_for_all_providers: bool;
        match std::env::var(
            ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
        ) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_cleaning_warning_logs_directory_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDirectoryPrintsForAllProvidersError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDirectoryPrintsForAllProvidersError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_info_prints_for_all_providers: bool;
        match std::env::var(ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_info_prints_for_all_providers = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInfoPrintsForAllProvidersError {
                        was_dotenv_enable,
                        env_name: ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInfoPrintsForAllProvidersError { was_dotenv_enable, env_name: ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_write_error_logs_in_local_folder: bool;
        match std::env::var(ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_write_error_logs_in_local_folder = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWriteErrorLogsInLocalFolderError {
                        was_dotenv_enable,
                        env_name: ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWriteErrorLogsInLocalFolderError { was_dotenv_enable, env_name: ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_write_error_logs_in_mongo: bool;
        match std::env::var(ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_write_error_logs_in_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWriteErrorLogsInMongoError {
                        was_dotenv_enable,
                        env_name: ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWriteErrorLogsInMongoError { was_dotenv_enable, env_name: ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_params_enable_initialize_mongo_with_providers_link_parts: bool;
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_params_enable_initialize_mongo_with_providers_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInitializeMongoWithProvidersLinkPartsError {
                        was_dotenv_enable,
                        env_name: ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInitializeMongoWithProvidersLinkPartsError { was_dotenv_enable, env_name: ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_mongo_params_providers_db_name_handle: String;
        match std::env::var(PROVIDERS_DB_NAME_HANDLE_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_providers_db_name_handle = handle;
            }
            Err(e) => {
                return Err(ConfigError::ProvidersDbNameHandleError { was_dotenv_enable, env_name: PROVIDERS_DB_NAME_HANDLE_ENV_NAME, env_error: e });
            }
        }
        let handle_config_mongo_params_providers_db_collection_handle_second_part: String;
        match std::env::var(PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_providers_db_collection_handle_second_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::ProvidersDbCollectionHandleSecondPartError { was_dotenv_enable, env_name: PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME, env_error: e });
            }
        }
        let handle_config_mongo_params_providers_db_collection_document_field_name_handle: String;
        match std::env::var(PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_providers_db_collection_document_field_name_handle =
                    handle;
            }
            Err(e) => {
                return Err(ConfigError::ProvidersDbCollectionDocumentFieldNameHandleError { was_dotenv_enable, env_name: PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME, env_error: e });
            }
        }
        let handle_config_mongo_params_db_providers_logs_name_handle: String;
        match std::env::var(DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_db_providers_logs_name_handle = handle;
            }
            Err(e) => {
                return Err(ConfigError::DbProvidersLogsNameHandleError { was_dotenv_enable, env_name: DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME, env_error: e });
            }
        }
        let handle_config_mongo_params_db_providers_logs_collection_handle_second_part: String;
        match std::env::var(DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_db_providers_logs_collection_handle_second_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::DbProvidersLogsCollectionHandleSecondPartError { was_dotenv_enable, env_name: DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME, env_error: e });
            }
        }
        let handle_config_mongo_params_db_providers_logs_collection_document_field_name_handle: String;
        match std::env::var(DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_db_providers_logs_collection_document_field_name_handle = handle;
            }
            Err(e) => {
                return Err(ConfigError::DbProvidersLogsCollectionDocumentFieldNameHandleError { was_dotenv_enable, env_name: DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME, env_error: e });
            }
        }
        let handle_config_mongo_params_path_to_provider_link_parts_folder: String;
        match std::env::var(PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_path_to_provider_link_parts_folder = handle;
            }
            Err(e) => {
                return Err(ConfigError::PathToProviderLinkPartsFolderError { was_dotenv_enable, env_name: PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME, env_error: e });
            }
        }
        let handle_config_mongo_params_log_file_extension: String;
        match std::env::var(LOG_FILE_EXTENSION_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_log_file_extension = handle;
            }
            Err(e) => {
                return Err(ConfigError::LogFileExtensionError { was_dotenv_enable, env_name: LOG_FILE_EXTENSION_ENV_NAME, env_error: e });
            }
        }
        let handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_arxiv_link_parts: bool;
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_arxiv_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInitializeMongoWithArxivLinkPartsError {
                        was_dotenv_enable,
                        env_name: ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInitializeMongoWithArxivLinkPartsError { was_dotenv_enable, env_name: ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_biorxiv_link_parts: bool;
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_biorxiv_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInitializeMongoWithBiorxivLinkPartsError {
                        was_dotenv_enable,
                        env_name: ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInitializeMongoWithBiorxivLinkPartsError { was_dotenv_enable, env_name: ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_github_link_parts: bool;
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_github_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInitializeMongoWithGithubLinkPartsError {
                        was_dotenv_enable,
                        env_name: ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInitializeMongoWithGithubLinkPartsError  { was_dotenv_enable, env_name: ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_habr_link_parts: bool;
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_habr_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInitializeMongoWithHabrLinkPartsError {
                        was_dotenv_enable,
                        env_name: ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInitializeMongoWithHabrLinkPartsError { was_dotenv_enable, env_name: ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_medrxiv_link_parts: bool;
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_medrxiv_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInitializeMongoWithMedrxivLinkPartsError {
                        was_dotenv_enable,
                        env_name: ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInitializeMongoWithMedrxivLinkPartsError { was_dotenv_enable, env_name: ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let  handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_reddit_link_parts: bool;
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_reddit_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInitializeMongoWithRedditLinkPartsError {
                        was_dotenv_enable,
                        env_name: ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInitializeMongoWithRedditLinkPartsError { was_dotenv_enable, env_name: ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_twitter_link_parts: bool;
        match std::env::var(ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_twitter_link_parts = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInitializeMongoWithTwitterLinkPartsError {
                        was_dotenv_enable,
                        env_name: ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInitializeMongoWithTwitterLinkPartsError { was_dotenv_enable, env_name: ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_mongo_params_mongo_url_parts_mongo_first_handle_url_part: String;
        match std::env::var(MONGO_FIRST_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_mongo_url_parts_mongo_first_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::MongoFirstHandleUrlPartError { was_dotenv_enable, env_name: MONGO_FIRST_HANDLE_URL_PART_ENV_NAME, env_error: e });
            }
        }
        let handle_config_mongo_params_mongo_url_parts_mongo_second_handle_url_part: String;
        match std::env::var(MONGO_SECOND_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_mongo_url_parts_mongo_second_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::MongoSecondHandleUrlPartError { was_dotenv_enable, env_name: MONGO_SECOND_HANDLE_URL_PART_ENV_NAME, env_error: e });
            }
        }
        let handle_config_mongo_params_mongo_url_parts_mongo_third_handle_url_part: String;
        match std::env::var(MONGO_THIRD_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_mongo_url_parts_mongo_third_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::MongoThirdHandleUrlPartError { was_dotenv_enable, env_name: MONGO_THIRD_HANDLE_URL_PART_ENV_NAME, env_error: e });
            }
        }
        let  handle_config_mongo_params_mongo_url_parts_mongo_fourth_handle_url_part: String;
        match std::env::var(MONGO_FOURTH_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_mongo_url_parts_mongo_fourth_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::MongoFourthHandleUrlPartError { was_dotenv_enable, env_name: MONGO_FOURTH_HANDLE_URL_PART_ENV_NAME, env_error: e });
            }
        }
        let  handle_config_mongo_params_mongo_url_parts_mongo_fifth_handle_url_part: String;
        match std::env::var(MONGO_FIFTH_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config_mongo_params_mongo_url_parts_mongo_fifth_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::MongoFifthHandleUrlPartError { was_dotenv_enable, env_name: MONGO_FIFTH_HANDLE_URL_PART_ENV_NAME, env_error: e });
            }
        }
        let handle_config_mongo_authorization_mongo_login: String;
        match std::env::var(MONGO_LOGIN_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_mongo_authorization_mongo_login = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::MongoLoginError { was_dotenv_enable, env_name: MONGO_LOGIN_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_mongo_authorization_mongo_password: String;
        match std::env::var(MONGO_PASSWORD_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_mongo_authorization_mongo_password = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::MongoPasswordError { was_dotenv_enable, env_name: MONGO_PASSWORD_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_mongo_authorization_mongo_ip: String;
        match std::env::var(MONGO_IP_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_mongo_authorization_mongo_ip = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::MongoIpError { was_dotenv_enable, env_name: MONGO_IP_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_mongo_authorization_mongo_port: String;
        match std::env::var(MONGO_PORT_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_mongo_authorization_mongo_port = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::MongoPortError { was_dotenv_enable, env_name: MONGO_PORT_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_mongo_authorization_mongo_params: String;
        match std::env::var(MONGO_PARAMS_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_mongo_authorization_mongo_params = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::MongoParamsError { was_dotenv_enable, env_name: MONGO_PARAMS_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_postgres_params_postgres_url_parts_postgres_first_handle_url_part: String;
        match std::env::var(POSTGRES_FIRST_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config_postgres_params_postgres_url_parts_postgres_first_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::PostgresFirstHandleUrlPartError { was_dotenv_enable, env_name: POSTGRES_FIRST_HANDLE_URL_PART_ENV_NAME, env_error: e });
            }
        }
        let handle_config_postgres_params_postgres_url_parts_postgres_second_handle_url_part: String;
        match std::env::var(POSTGRES_SECOND_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config_postgres_params_postgres_url_parts_postgres_second_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::PostgresSecondHandleUrlPartError { was_dotenv_enable, env_name: POSTGRES_SECOND_HANDLE_URL_PART_ENV_NAME, env_error: e });
            }
        }
        let handle_config_postgres_params_postgres_url_parts_postgres_third_handle_url_part: String;
        match std::env::var(POSTGRES_THIRD_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config_postgres_params_postgres_url_parts_postgres_third_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::PostgresThirdHandleUrlPartError { was_dotenv_enable, env_name: POSTGRES_THIRD_HANDLE_URL_PART_ENV_NAME, env_error: e });
            }
        }
        let handle_config_postgres_params_postgres_url_parts_postgres_fourth_handle_url_part: String;
        match std::env::var(POSTGRES_FOURTH_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config_postgres_params_postgres_url_parts_postgres_fourth_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::PostgresFourthHandleUrlPartError { was_dotenv_enable, env_name: POSTGRES_FOURTH_HANDLE_URL_PART_ENV_NAME, env_error: e });
            }
        }
        let handle_config_postgres_params_postgres_url_parts_postgres_fifth_handle_url_part: String;
        match std::env::var(POSTGRES_FIFTH_HANDLE_URL_PART_ENV_NAME) {
            Ok(handle) => {
                handle_config_postgres_params_postgres_url_parts_postgres_fifth_handle_url_part = handle;
            }
            Err(e) => {
                return Err(ConfigError::PostgresFifthHandleUrlPartError { was_dotenv_enable, env_name: POSTGRES_FIFTH_HANDLE_URL_PART_ENV_NAME, env_error: e });
            }
        }
        
        let handle_config_postgres_authorization_postgres_login: String;
        match std::env::var(POSTGRES_LOGIN_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_postgres_authorization_postgres_login = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::PostgresLoginError { was_dotenv_enable, env_name: POSTGRES_LOGIN_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_postgres_authorization_postgres_password: String;
        match std::env::var(POSTGRES_PASSWORD_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_postgres_authorization_postgres_password = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::PostgresPasswordError { was_dotenv_enable, env_name: POSTGRES_PASSWORD_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_postgres_authorization_postgres_ip: String;
        match std::env::var(POSTGRES_IP_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_postgres_authorization_postgres_ip = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::PostgresIpError { was_dotenv_enable, env_name: POSTGRES_IP_ENV_NAME, env_error: e });
                    }
                }
                let handle_config_postgres_authorization_postgres_port: String;
        match std::env::var(POSTGRES_PORT_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_postgres_authorization_postgres_port = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::PostgresPortError { was_dotenv_enable, env_name: POSTGRES_PORT_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_postgres_authorization_postgres_db: String;
        match std::env::var(POSTGRES_DB_ENV_NAME) {
                    Ok(handle) => {
                        handle_config_postgres_authorization_postgres_db = handle;
                    }
                    Err(e) => {
                        return Err(ConfigError::PostgresDbError { was_dotenv_enable, env_name: POSTGRES_DB_ENV_NAME, env_error: e });
                    }
                }
        let handle_config_enable_providers_enable_arxiv: bool;
        match std::env::var(ENABLE_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_enable_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableArxivError { was_dotenv_enable, env_name: ENABLE_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_enable_biorxiv: bool;
        match std::env::var(ENABLE_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_enable_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableBiorxivError { was_dotenv_enable, env_name: ENABLE_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_enable_github: bool;
        match std::env::var(ENABLE_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_enable_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableGithubError { was_dotenv_enable, env_name: ENABLE_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_enable_habr: bool;
        match std::env::var(ENABLE_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_enable_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableHabrError { was_dotenv_enable, env_name: ENABLE_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_enable_medrxiv: bool;
        match std::env::var(ENABLE_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_enable_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableMedrxivError { was_dotenv_enable, env_name: ENABLE_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_enable_reddit: bool;
        match std::env::var(ENABLE_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_enable_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableRedditError { was_dotenv_enable, env_name: ENABLE_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_enable_twitter: bool;
        match std::env::var(ENABLE_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_enable_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableTwitterError  {
                        was_dotenv_enable,
                        env_name: ENABLE_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableTwitterError { was_dotenv_enable, env_name: ENABLE_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_providers_check_links_arxiv_link: String;
        match std::env::var(ARXIV_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config_providers_check_links_arxiv_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::ArxivLinkError { was_dotenv_enable, env_name: ARXIV_LINK_ENV_NAME, env_error: e });
            }
        }
        let handle_config_providers_check_links_biorxiv_link: String;
        match std::env::var(BIORXIV_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config_providers_check_links_biorxiv_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::BiorxivLinkError { was_dotenv_enable, env_name: BIORXIV_LINK_ENV_NAME, env_error: e });
            }
        }
        let handle_config_providers_check_links_github_link: String;
        match std::env::var(GITHUB_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config_providers_check_links_github_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::GithubLinkError { was_dotenv_enable, env_name: GITHUB_LINK_ENV_NAME, env_error: e });
            }
        }
        let handle_config_providers_check_links_habr_link: String;
        match std::env::var(HABR_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config_providers_check_links_habr_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::HabrLinkError { was_dotenv_enable, env_name: HABR_LINK_ENV_NAME, env_error: e });
            }
        }
        let handle_config_providers_check_links_medrxiv_link: String;
        match std::env::var(MEDRXIV_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config_providers_check_links_medrxiv_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::MedrxivLinkError { was_dotenv_enable, env_name: MEDRXIV_LINK_ENV_NAME, env_error: e });
            }
        }
        let handle_config_providers_check_links_reddit_link: String;
        match std::env::var(REDDIT_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config_providers_check_links_reddit_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::RedditLinkError { was_dotenv_enable, env_name: REDDIT_LINK_ENV_NAME, env_error: e });
            }
        }
        let handle_config_providers_check_links_twitter_link: String;
        match std::env::var(TWITTER_LINK_ENV_NAME) {
            Ok(handle) => {
                handle_config_providers_check_links_twitter_link = handle;
            }
            Err(e) => {
                return Err(ConfigError::TwitterLinkError { was_dotenv_enable, env_name: TWITTER_LINK_ENV_NAME, env_error: e });
            }
        }
        let handle_config_enable_providers_prints_enable_prints_arxiv: bool;
        match std::env::var(ENABLE_PRINTS_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_prints_enable_prints_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePrintsArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_PRINTS_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePrintsArxivError { was_dotenv_enable, env_name: ENABLE_PRINTS_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_prints_enable_prints_biorxiv: bool;
        match std::env::var(ENABLE_PRINTS_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_prints_enable_prints_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePrintsBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_PRINTS_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePrintsBiorxivError { was_dotenv_enable, env_name: ENABLE_PRINTS_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_prints_enable_prints_github: bool;
        match std::env::var(ENABLE_PRINTS_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_prints_enable_prints_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePrintsGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_PRINTS_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePrintsGithubError { was_dotenv_enable, env_name: ENABLE_PRINTS_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_prints_enable_prints_habr: bool;
        match std::env::var(ENABLE_PRINTS_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_prints_enable_prints_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePrintsHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_PRINTS_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePrintsHabrError { was_dotenv_enable, env_name: ENABLE_PRINTS_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_prints_enable_prints_medrxiv: bool;
        match std::env::var(ENABLE_PRINTS_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_prints_enable_prints_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePrintsMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_PRINTS_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePrintsMedrxivError { was_dotenv_enable, env_name: ENABLE_PRINTS_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_prints_enable_prints_reddit: bool;
        match std::env::var(ENABLE_PRINTS_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_prints_enable_prints_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePrintsRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_PRINTS_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePrintsRedditError { was_dotenv_enable, env_name: ENABLE_PRINTS_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_prints_enable_prints_twitter: bool;
        match std::env::var(ENABLE_PRINTS_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_prints_enable_prints_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePrintsTwitterError {
                        was_dotenv_enable,
                        env_name: ENABLE_PRINTS_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePrintsTwitterError { was_dotenv_enable, env_name: ENABLE_PRINTS_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_arxiv: bool;
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningHighPrintsForArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningHighPrintsForArxivError { was_dotenv_enable, env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_biorxiv: bool;
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningHighPrintsForBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningHighPrintsForBiorxivError { was_dotenv_enable, env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_github: bool;
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningHighPrintsForGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningHighPrintsForGithubError { was_dotenv_enable, env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_habr: bool;
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningHighPrintsForHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningHighPrintsForHabrError { was_dotenv_enable, env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_medrxiv: bool;
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningHighPrintsForMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningHighPrintsForMedrxivError { was_dotenv_enable, env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_reddit: bool;
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningHighPrintsForRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningHighPrintsForRedditError { was_dotenv_enable, env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_twitter: bool;
        match std::env::var(ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningHighPrintsForTwitterError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningHighPrintsForTwitterError { was_dotenv_enable, env_name: ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_arxiv: bool;
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningLowPrintsForArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningLowPrintsForArxivError { was_dotenv_enable, env_name: ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_biorxiv: bool;
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningLowPrintsForBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningLowPrintsForBiorxivError { was_dotenv_enable, env_name: ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_github: bool;
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningLowPrintsForGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningLowPrintsForGithubError { was_dotenv_enable, env_name: ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_habr: bool;
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningLowPrintsForHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningLowPrintsForHabrError { was_dotenv_enable, env_name: ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_medrxiv: bool;
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningLowPrintsForMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningLowPrintsForMedrxivError { was_dotenv_enable, env_name: ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_reddit: bool;
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningLowPrintsForRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningLowPrintsForRedditError { was_dotenv_enable, env_name: ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_twitter: bool;
        match std::env::var(ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableWarningLowPrintsForTwitterError {
                        was_dotenv_enable,
                        env_name: ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableWarningLowPrintsForTwitterError { was_dotenv_enable, env_name: ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_success_providers_prints_enable_success_prints_for_arxiv: bool;
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_success_providers_prints_enable_success_prints_for_arxiv =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableErrorPrintsForArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableErrorPrintsForArxivError { was_dotenv_enable, env_name: ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_success_providers_prints_enable_success_prints_for_biorxiv: bool;
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_success_providers_prints_enable_success_prints_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableErrorPrintsForBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableErrorPrintsForBiorxivError { was_dotenv_enable, env_name: ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_success_providers_prints_enable_success_prints_for_github: bool;
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_success_providers_prints_enable_success_prints_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableErrorPrintsForGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableErrorPrintsForGithubError { was_dotenv_enable, env_name: ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_success_providers_prints_enable_success_prints_for_habr: bool;
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_success_providers_prints_enable_success_prints_for_habr =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableErrorPrintsForHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableErrorPrintsForHabrError { was_dotenv_enable, env_name: ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_success_providers_prints_enable_success_prints_for_medrxiv: bool;
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_success_providers_prints_enable_success_prints_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableErrorPrintsForMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableErrorPrintsForMedrxivError { was_dotenv_enable, env_name: ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_success_providers_prints_enable_success_prints_for_reddit: bool;
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_success_providers_prints_enable_success_prints_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableErrorPrintsForRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableErrorPrintsForRedditError { was_dotenv_enable, env_name: ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_success_providers_prints_enable_success_prints_for_twitter: bool;
        match std::env::var(ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_success_providers_prints_enable_success_prints_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableErrorPrintsForTwitterError {
                        was_dotenv_enable,
                        env_name: ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableErrorPrintsForTwitterError { was_dotenv_enable, env_name: ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_arxiv: bool;
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableSuccessPrintsForArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableSuccessPrintsForArxivError { was_dotenv_enable, env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_biorxiv: bool;
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableSuccessPrintsForBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableSuccessPrintsForBiorxivError { was_dotenv_enable, env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_github: bool;
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableSuccessPrintsForGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableSuccessPrintsForGithubError { was_dotenv_enable, env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_habr: bool;
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableSuccessPrintsForHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableSuccessPrintsForHabrError { was_dotenv_enable, env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_medrxiv: bool;
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableSuccessPrintsForMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableSuccessPrintsForMedrxivError { was_dotenv_enable, env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_reddit: bool;
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableSuccessPrintsForRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableSuccessPrintsForRedditError { was_dotenv_enable, env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_twitter: bool;
        match std::env::var(ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableSuccessPrintsForTwitterError {
                        was_dotenv_enable,
                        env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableSuccessPrintsForTwitterError { was_dotenv_enable, env_name: ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_error_providers_prints_enable_error_prints_for_arxiv: bool;
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_error_providers_prints_enable_error_prints_for_arxiv =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePartialSuccessPrintsForArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePartialSuccessPrintsForArxivError { was_dotenv_enable, env_name: ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_error_providers_prints_enable_error_prints_for_biorxiv: bool;
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_error_providers_prints_enable_error_prints_for_biorxiv =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePartialSuccessPrintsForBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePartialSuccessPrintsForBiorxivError { was_dotenv_enable, env_name: ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_error_providers_prints_enable_error_prints_for_github: bool;
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_error_providers_prints_enable_error_prints_for_github =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePartialSuccessPrintsForGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePartialSuccessPrintsForGithubError { was_dotenv_enable, env_name: ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_error_providers_prints_enable_error_prints_for_habr: bool;
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_error_providers_prints_enable_error_prints_for_habr =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePartialSuccessPrintsForHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePartialSuccessPrintsForHabrError { was_dotenv_enable, env_name: ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_error_providers_prints_enable_error_prints_for_medrxiv: bool;
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_error_providers_prints_enable_error_prints_for_medrxiv =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePartialSuccessPrintsForMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePartialSuccessPrintsForMedrxivError { was_dotenv_enable, env_name: ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_error_providers_prints_enable_error_prints_for_reddit: bool;
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_error_providers_prints_enable_error_prints_for_reddit =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePartialSuccessPrintsForRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePartialSuccessPrintsForRedditError { was_dotenv_enable, env_name: ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_error_providers_prints_enable_error_prints_for_twitter: bool;
        match std::env::var(ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_error_providers_prints_enable_error_prints_for_twitter =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnablePartialSuccessPrintsForTwitterError {
                        was_dotenv_enable,
                        env_name: ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnablePartialSuccessPrintsForTwitterError { was_dotenv_enable, env_name: ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_arxiv: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDirectoryForArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDirectoryForArxivError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_biorxiv: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDirectoryForBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDirectoryForBiorxivError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_github: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDirectoryForGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDirectoryForGithubError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_habr: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDirectoryForHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDirectoryForHabrError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_medrxiv: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDirectoryForMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDirectoryForMedrxivError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_reddit: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDirectoryForRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDirectoryForRedditError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_twitter: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDirectoryForTwitterError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDirectoryForTwitterError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_arxiv: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForArxivError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_biorxiv: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForBiorxivError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_github: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForGithubError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_habr: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForHabrError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_medrxiv: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForMedrxivError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_reddit: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForRedditError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_twitter: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForTwitterError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbInMongoForTwitterError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME)
        {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForArxivError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv: bool;
        match std::env::var(
            ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME,
        ) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxivError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_github: bool;
        match std::env::var(
            ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME,
        ) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForGithubError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_habr: bool;
        match std::env::var(ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME)
        {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForHabrError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv: bool;
        match std::env::var(
            ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
        ) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxivError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit: bool;
        match std::env::var(
            ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME,
        ) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForRedditError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter: bool;
        match std::env::var(
            ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME,
        ) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForTwitterError {
                        was_dotenv_enable,
                        env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableCleaningWarningLogsDbCollectionsInMongoForTwitterError { was_dotenv_enable, env_name: ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_time_measurement_enable_time_measurement_for_arxiv: bool;
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_time_measurement_enable_time_measurement_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableTimeMeasurementForArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableTimeMeasurementForArxivError { was_dotenv_enable, env_name: ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_time_measurement_enable_time_measurement_for_biorxiv: bool;
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_time_measurement_enable_time_measurement_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableTimeMeasurementForBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableTimeMeasurementForBiorxivError { was_dotenv_enable, env_name: ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_time_measurement_enable_time_measurement_for_github: bool;
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_time_measurement_enable_time_measurement_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableTimeMeasurementForGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableTimeMeasurementForGithubError { was_dotenv_enable, env_name: ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_time_measurement_enable_time_measurement_for_habr: bool;
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_time_measurement_enable_time_measurement_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableTimeMeasurementForHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableTimeMeasurementForHabrError { was_dotenv_enable, env_name: ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_time_measurement_enable_time_measurement_for_medrxiv: bool;
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_time_measurement_enable_time_measurement_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableTimeMeasurementForMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableTimeMeasurementForMedrxivError { was_dotenv_enable, env_name: ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_time_measurement_enable_time_measurement_for_reddit: bool;
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_time_measurement_enable_time_measurement_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableTimeMeasurementForRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableTimeMeasurementForRedditError { was_dotenv_enable, env_name: ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_time_measurement_enable_time_measurement_for_twitter: bool;
        match std::env::var(ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_time_measurement_enable_time_measurement_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableTimeMeasurementForTwitterError {
                        was_dotenv_enable,
                        env_name: ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableTimeMeasurementForTwitterError { was_dotenv_enable, env_name: ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_info_enable_info_for_arxiv: bool;
        match std::env::var(ENABLE_INFO_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_info_enable_info_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInfoForArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_INFO_FOR_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInfoForArxivError { was_dotenv_enable, env_name: ENABLE_INFO_FOR_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_info_enable_info_for_biorxiv: bool;
        match std::env::var(ENABLE_INFO_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_info_enable_info_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInfoForBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_INFO_FOR_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInfoForBiorxivError { was_dotenv_enable, env_name: ENABLE_INFO_FOR_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_info_enable_info_for_github: bool;
        match std::env::var(ENABLE_INFO_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_info_enable_info_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInfoForGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_INFO_FOR_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInfoForGithubError { was_dotenv_enable, env_name: ENABLE_INFO_FOR_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_info_enable_info_for_habr: bool;
        match std::env::var(ENABLE_INFO_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_info_enable_info_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInfoForHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_INFO_FOR_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInfoForHabrError { was_dotenv_enable, env_name: ENABLE_INFO_FOR_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_info_enable_info_for_medrxiv: bool;
        match std::env::var(ENABLE_INFO_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_info_enable_info_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInfoForMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_INFO_FOR_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInfoForMedrxivError { was_dotenv_enable, env_name: ENABLE_INFO_FOR_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_info_enable_info_for_reddit: bool;
        match std::env::var(ENABLE_INFO_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_info_enable_info_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInfoForRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_INFO_FOR_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInfoForRedditError { was_dotenv_enable, env_name: ENABLE_INFO_FOR_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_info_enable_info_for_twitter: bool;
        match std::env::var(ENABLE_INFO_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_info_enable_info_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableInfoForTwitterError {
                        was_dotenv_enable,
                        env_name: ENABLE_INFO_FOR_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableInfoForTwitterError { was_dotenv_enable, env_name: ENABLE_INFO_FOR_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_links_limits_enable_links_limit_for_arxiv: bool;
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_links_limits_enable_links_limit_for_arxiv =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableLinksLimitForArxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableLinksLimitForArxivError { was_dotenv_enable, env_name: ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_links_limits_enable_links_limit_for_biorxiv: bool;
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_links_limits_enable_links_limit_for_biorxiv =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableLinksLimitForBiorxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableLinksLimitForBiorxivError { was_dotenv_enable, env_name: ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_links_limits_enable_links_limit_for_github: bool;
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_links_limits_enable_links_limit_for_github =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableLinksLimitForGithubError {
                        was_dotenv_enable,
                        env_name: ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableLinksLimitForGithubError { was_dotenv_enable, env_name: ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_links_limits_enable_links_limit_for_habr: bool;
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_links_limits_enable_links_limit_for_habr =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableLinksLimitForHabrError {
                        was_dotenv_enable,
                        env_name: ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableLinksLimitForHabrError { was_dotenv_enable, env_name: ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_links_limits_enable_links_limit_for_medrxiv: bool;
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_links_limits_enable_links_limit_for_medrxiv =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableLinksLimitForMedrxivError {
                        was_dotenv_enable,
                        env_name: ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableLinksLimitForMedrxivError { was_dotenv_enable, env_name: ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_links_limits_enable_links_limit_for_reddit: bool;
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_links_limits_enable_links_limit_for_reddit =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableLinksLimitForRedditError {
                        was_dotenv_enable,
                        env_name: ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableLinksLimitForRedditError { was_dotenv_enable, env_name: ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_providers_links_limits_enable_links_limit_for_twitter: bool;
        match std::env::var(ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_providers_links_limits_enable_links_limit_for_twitter =
                        handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableLinksLimitForTwitterError {
                        was_dotenv_enable,
                        env_name: ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableLinksLimitForTwitterError { was_dotenv_enable, env_name: ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_providers_links_limits_links_limit_for_arxiv: i64;
        match std::env::var(LINKS_LIMIT_FOR_ARXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config_providers_links_limits_links_limit_for_arxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::LinksLimitForArxivError {
                        was_dotenv_enable,
                        env_name: LINKS_LIMIT_FOR_ARXIV_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::LinksLimitForArxivError { was_dotenv_enable, env_name: LINKS_LIMIT_FOR_ARXIV_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_providers_links_limits_links_limit_for_biorxiv: i64;
        match std::env::var(LINKS_LIMIT_FOR_BIORXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config_providers_links_limits_links_limit_for_biorxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::LinksLimitForBiorxivError {
                        was_dotenv_enable,
                        env_name: LINKS_LIMIT_FOR_BIORXIV_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::LinksLimitForBiorxivError { was_dotenv_enable, env_name: LINKS_LIMIT_FOR_BIORXIV_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_providers_links_limits_links_limit_for_github: i64;
        match std::env::var(LINKS_LIMIT_FOR_GITHUB_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config_providers_links_limits_links_limit_for_github = handle;
                }
                Err(e) => {
                    return Err(ConfigError::LinksLimitForGithubError {
                        was_dotenv_enable,
                        env_name: LINKS_LIMIT_FOR_GITHUB_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::LinksLimitForGithubError { was_dotenv_enable, env_name: LINKS_LIMIT_FOR_GITHUB_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_providers_links_limits_links_limit_for_habr: i64;
        match std::env::var(LINKS_LIMIT_FOR_HABR_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config_providers_links_limits_links_limit_for_habr = handle;
                }
                Err(e) => {
                    return Err(ConfigError::LinksLimitForHabrError {
                        was_dotenv_enable,
                        env_name: LINKS_LIMIT_FOR_HABR_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::LinksLimitForHabrError { was_dotenv_enable, env_name: LINKS_LIMIT_FOR_HABR_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_providers_links_limits_links_limit_for_medrxiv: i64;
        match std::env::var(LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config_providers_links_limits_links_limit_for_medrxiv = handle;
                }
                Err(e) => {
                    return Err(ConfigError::LinksLimitForMedrxivError {
                        was_dotenv_enable,
                        env_name: LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::LinksLimitForMedrxivError { was_dotenv_enable, env_name: LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_providers_links_limits_links_limit_for_reddit: i64;
        match std::env::var(LINKS_LIMIT_FOR_REDDIT_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config_providers_links_limits_links_limit_for_reddit = handle;
                }
                Err(e) => {
                    return Err(ConfigError::LinksLimitForRedditError {
                        was_dotenv_enable,
                        env_name: LINKS_LIMIT_FOR_REDDIT_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::LinksLimitForRedditError { was_dotenv_enable, env_name: LINKS_LIMIT_FOR_REDDIT_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_providers_links_limits_links_limit_for_twitter: i64;
        match std::env::var(LINKS_LIMIT_FOR_TWITTER_ENV_NAME) {
            Ok(handle) => match handle.parse::<i64>() {
                Ok(handle) => {
                    handle_config_providers_links_limits_links_limit_for_twitter = handle;
                }
                Err(e) => {
                    return Err(ConfigError::LinksLimitForTwitterError {
                        was_dotenv_enable,
                        env_name: LINKS_LIMIT_FOR_TWITTER_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::LinksLimitForTwitterError { was_dotenv_enable, env_name: LINKS_LIMIT_FOR_TWITTER_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_arxiv_link_parts_for_mongo: bool;
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_arxiv_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableRandomizeOrderForArxivLinkPartsForMongoError {
                        was_dotenv_enable,
                        env_name: ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableRandomizeOrderForArxivLinkPartsForMongoError { was_dotenv_enable, env_name: ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_biorxiv_link_parts_for_mongo: bool;
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_biorxiv_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableRandomizeOrderForBiorxivLinkPartsForMongoError {
                        was_dotenv_enable,
                        env_name: ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableRandomizeOrderForBiorxivLinkPartsForMongoError { was_dotenv_enable, env_name: ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_github_link_parts_for_mongo: bool;
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_github_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableRandomizeOrderForGithubLinkPartsForMongoError {
                        was_dotenv_enable,
                        env_name: ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableRandomizeOrderForGithubLinkPartsForMongoError { was_dotenv_enable, env_name: ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_habr_link_parts_for_mongo: bool;
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_habr_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableRandomizeOrderForHabrLinkPartsForMongoError {
                        was_dotenv_enable,
                        env_name: ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableRandomizeOrderForHabrLinkPartsForMongoError { was_dotenv_enable, env_name: ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_medrxiv_link_parts_for_mongo: bool;
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_medrxiv_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableRandomizeOrderForMedrxivLinkPartsForMongoError {
                        was_dotenv_enable,
                        env_name: ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableRandomizeOrderForMedrxivLinkPartsForMongoError { was_dotenv_enable, env_name: ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_reddit_link_parts_for_mongo: bool;
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_reddit_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableRandomizeOrderForRedditLinkPartsForMongoError {
                        was_dotenv_enable,
                        env_name: ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableRandomizeOrderForRedditLinkPartsForMongoError { was_dotenv_enable, env_name: ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_twitter_link_parts_for_mongo: bool;
        match std::env::var(ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME) {
            Ok(handle) => match handle.parse::<bool>() {
                Ok(handle) => {
                    handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_twitter_link_parts_for_mongo = handle;
                }
                Err(e) => {
                    return Err(ConfigError::EnableRandomizeOrderForTwitterLinkPartsForMongoError {
                        was_dotenv_enable,
                        env_name: ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME,
                        env_error: VarOrBoolParseError::Bool(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::EnableRandomizeOrderForTwitterLinkPartsForMongoError { was_dotenv_enable, env_name: ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME, env_error: VarOrBoolParseError::Var(e), });
            }
        }
        let handle_config_print_colors_error_red: u8;
        match std::env::var(ERROR_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_error_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::ErrorRedError {
                        was_dotenv_enable,
                        env_name: ERROR_RED_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::ErrorRedError { was_dotenv_enable, env_name: ERROR_RED_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_error_green: u8;
        match std::env::var(ERROR_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_error_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::ErrorGreenError {
                        was_dotenv_enable,
                        env_name: ERROR_GREEN_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::ErrorGreenError { was_dotenv_enable, env_name: ERROR_GREEN_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_error_blue: u8;
        match std::env::var(ERROR_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_error_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::ErrorBlueError {
                        was_dotenv_enable,
                        env_name: ERROR_BLUE_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::ErrorBlueError { was_dotenv_enable, env_name: ERROR_BLUE_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_warning_high_red: u8;
        match std::env::var(WARNING_HIGH_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_warning_high_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::WarningHighRedError {
                        was_dotenv_enable,
                        env_name: WARNING_HIGH_RED_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::WarningHighRedError { was_dotenv_enable, env_name: WARNING_HIGH_RED_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_warning_high_green: u8;
        match std::env::var(WARNING_HIGH_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_warning_high_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::WarningHighGreenError {
                        was_dotenv_enable,
                        env_name: WARNING_HIGH_GREEN_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::WarningHighGreenError { was_dotenv_enable, env_name: WARNING_HIGH_GREEN_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_warning_high_blue: u8;
        match std::env::var(WARNING_HIGH_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_warning_high_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::WarningHighBlueError {
                        was_dotenv_enable,
                        env_name: WARNING_HIGH_BLUE_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::WarningHighBlueError { was_dotenv_enable, env_name: WARNING_HIGH_BLUE_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_warning_low_red: u8;
        match std::env::var(WARNING_LOW_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_warning_low_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::WarningLowRedError {
                        was_dotenv_enable,
                        env_name: WARNING_LOW_RED_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::WarningLowRedError { was_dotenv_enable, env_name: WARNING_LOW_RED_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_warning_low_green: u8;
        match std::env::var(WARNING_LOW_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_warning_low_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::WarningLowGreenError {
                        was_dotenv_enable,
                        env_name: WARNING_LOW_GREEN_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::WarningLowGreenError { was_dotenv_enable, env_name: WARNING_LOW_GREEN_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_warning_low_blue: u8;
        match std::env::var(WARNING_LOW_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_warning_low_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::WarningLowBlueError {
                        was_dotenv_enable,
                        env_name: WARNING_LOW_BLUE_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::WarningLowBlueError { was_dotenv_enable, env_name: WARNING_LOW_BLUE_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_success_red: u8;
        match std::env::var(SUCCESS_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_success_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::SuccessRedError {
                        was_dotenv_enable,
                        env_name: SUCCESS_RED_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::SuccessRedError { was_dotenv_enable, env_name: SUCCESS_RED_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_success_green: u8;
        match std::env::var(SUCCESS_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_success_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::SuccessGreenError {
                        was_dotenv_enable,
                        env_name: SUCCESS_GREEN_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::SuccessGreenError { was_dotenv_enable, env_name: SUCCESS_GREEN_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_success_blue: u8;
        match std::env::var(SUCCESS_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_success_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::SuccessBlueError {
                        was_dotenv_enable,
                        env_name: SUCCESS_BLUE_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::SuccessBlueError { was_dotenv_enable, env_name: SUCCESS_BLUE_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_partial_success_red: u8;
        match std::env::var(PARTIAL_SUCCESS_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_partial_success_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::PartialSuccessRedError {
                        was_dotenv_enable,
                        env_name: PARTIAL_SUCCESS_RED_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::PartialSuccessRedError { was_dotenv_enable, env_name: PARTIAL_SUCCESS_RED_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_partial_success_green: u8;
        match std::env::var(PARTIAL_SUCCESS_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_partial_success_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::PartialSuccessGreenError {
                        was_dotenv_enable,
                        env_name: PARTIAL_SUCCESS_GREEN_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::PartialSuccessGreenError { was_dotenv_enable, env_name: PARTIAL_SUCCESS_GREEN_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_partial_success_blue: u8;
        match std::env::var(PARTIAL_SUCCESS_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_partial_success_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::PartialSuccessBlueError {
                        was_dotenv_enable,
                        env_name: PARTIAL_SUCCESS_BLUE_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::PartialSuccessBlueError { was_dotenv_enable, env_name: PARTIAL_SUCCESS_BLUE_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_cleaning_red: u8;
        match std::env::var(CLEANING_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_cleaning_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::CleaningRedError {
                        was_dotenv_enable,
                        env_name: CLEANING_RED_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::CleaningRedError { was_dotenv_enable, env_name: CLEANING_RED_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_cleaning_green: u8;
        match std::env::var(CLEANING_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_cleaning_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::CleaningGreenError {
                        was_dotenv_enable,
                        env_name: CLEANING_GREEN_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::CleaningGreenError { was_dotenv_enable, env_name: CLEANING_GREEN_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_cleaning_blue: u8;
        match std::env::var(CLEANING_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_cleaning_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::CleaningBlueError {
                        was_dotenv_enable,
                        env_name: CLEANING_BLUE_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::CleaningBlueError { was_dotenv_enable, env_name: CLEANING_BLUE_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_time_measurement_red: u8;
        match std::env::var(TIME_MEASUREMENT_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_time_measurement_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::TimeMeasurementRedError {
                        was_dotenv_enable,
                        env_name: TIME_MEASUREMENT_RED_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::TimeMeasurementRedError { was_dotenv_enable, env_name: TIME_MEASUREMENT_RED_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_time_measurement_green: u8;
        match std::env::var(TIME_MEASUREMENT_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_time_measurement_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::TimeMeasurementGreenError {
                        was_dotenv_enable,
                        env_name: TIME_MEASUREMENT_GREEN_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::TimeMeasurementGreenError { was_dotenv_enable, env_name: TIME_MEASUREMENT_GREEN_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_time_measurement_blue: u8;
        match std::env::var(TIME_MEASUREMENT_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_time_measurement_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::TimeMeasurementBlueError {
                        was_dotenv_enable,
                        env_name: TIME_MEASUREMENT_BLUE_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::TimeMeasurementBlueError { was_dotenv_enable, env_name: TIME_MEASUREMENT_BLUE_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_info_red: u8;
        match std::env::var(INFO_RED_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_info_red = handle;
                }
                Err(e) => {
                    return Err(ConfigError::InfoRedError {
                        was_dotenv_enable,
                        env_name: INFO_RED_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::InfoRedError { was_dotenv_enable, env_name: INFO_RED_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_info_green: u8;
        match std::env::var(INFO_GREEN_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_info_green = handle;
                }
                Err(e) => {
                    return Err(ConfigError::InfoGreenError {
                        was_dotenv_enable,
                        env_name: INFO_GREEN_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::InfoGreenError { was_dotenv_enable, env_name: INFO_GREEN_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        let handle_config_print_colors_info_blue: u8;
        match std::env::var(INFO_BLUE_ENV_NAME) {
            Ok(handle) => match handle.parse::<u8>() {
                Ok(handle) => {
                    handle_config_print_colors_info_blue = handle;
                }
                Err(e) => {
                    return Err(ConfigError::InfoBlueError {
                        was_dotenv_enable,
                        env_name: INFO_BLUE_ENV_NAME,
                        env_error: VarOrIntParseError::Int(e),
                    });
                }
            },
            Err(e) => {
                return Err(ConfigError::InfoBlueError { was_dotenv_enable, env_name: INFO_BLUE_ENV_NAME, env_error: VarOrIntParseError::Var(e) });
            }
        }
        //todo: rewrite it with type system enum ProviderKind
        let mut vec_of_provider_names_handle = Vec::<String>::with_capacity(ProviderKind::get_length());
        if handle_config_enable_providers_enable_arxiv {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Arxiv).to_owned());
        }
        if handle_config_enable_providers_enable_biorxiv {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Biorxiv).to_owned())
        }
        if handle_config_enable_providers_enable_github {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Github).to_owned());
        }
        if handle_config_enable_providers_enable_habr {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Habr).to_owned())
        }
        if handle_config_enable_providers_enable_medrxiv {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Medrxiv).to_owned())
        }
        if handle_config_enable_providers_enable_reddit {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Reddit).to_owned())
        }
        if handle_config_enable_providers_enable_twitter {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Twitter).to_owned())
        } 
        let handle_config: ConfigStruct = ConfigStruct {
            github_authorization: GithubAuthorization {
                github_name: handle_config_github_authorization_github_name,
                github_token: handle_config_github_authorization_github_token,
            },
            reddit_authorization: RedditAuthorization {
                reddit_user_agent: handle_config_reddit_authorization_reddit_user_agent,
                reddit_client_id: handle_config_reddit_authorization_reddit_client_id,
                reddit_client_secret: handle_config_reddit_authorization_reddit_client_secret,
                reddit_username: handle_config_reddit_authorization_reddit_username,
                reddit_password: handle_config_reddit_authorization_reddit_password,
            },
            params: Params {
                //todo
                vec_of_provider_names: vec_of_provider_names_handle,
                starting_check_link: handle_config_params_starting_check_link,
                user_credentials_dummy_handle: handle_config_params_user_credentials_dummy_handle,
                warning_logs_directory_name: handle_config_params_warning_logs_directory_name,
                unhandled_success_handled_success_are_there_items_initialized_posts_dir: handle_config_params_unhandled_success_handled_success_are_there_items_initialized_posts_dir,
                enable_providers: handle_config_params_enable_providers,
                enable_cleaning_warning_logs_directory: handle_config_params_enable_cleaning_warning_logs_directory,
                enable_cleaning_warning_logs_db_in_mongo: handle_config_params_enable_cleaning_warning_logs_db_in_mongo,
                enable_cleaning_warning_logs_db_collections_in_mongo: handle_config_params_enable_cleaning_warning_logs_db_collections_in_mongo,
                enable_time_measurement: handle_config_params_enable_time_measurement,
                enable_provider_links_limit: handle_config_params_enable_provider_links_limit,
                enable_common_providers_links_limit: handle_config_params_enable_common_providers_links_limit,
                common_providers_links_limit: handle_config_params_common_providers_links_limit,
                enable_randomize_order_for_providers_link_parts_for_mongo: handle_config_params_enable_randomize_order_for_providers_link_parts_for_mongo,
                enable_prints: handle_config_params_enable_prints,
                enable_error_prints: handle_config_params_enable_error_prints,
                enable_warning_high_prints: handle_config_params_enable_warning_high_prints,
                enable_warning_low_prints: handle_config_params_enable_warning_low_prints,
                enable_success_prints: handle_config_params_enable_success_prints,
                enable_partial_success_prints: handle_config_params_enable_partial_success_prints,
                enable_time_measurement_prints: handle_config_params_enable_time_measurement_prints,
                enable_cleaning_warning_logs_directory_prints: handle_config_params_enable_cleaning_warning_logs_directory_prints,
                enable_info_prints: handle_config_params_enable_info_prints,
                enable_all_providers_prints: handle_config_params_enable_all_providers_prints,
                enable_error_prints_for_all_providers: handle_config_params_enable_error_prints_for_all_providers,
                enable_warning_high_prints_for_all_providers: handle_config_params_enable_warning_high_prints_for_all_providers,
                enable_warning_low_prints_for_all_providers: handle_config_params_enable_warning_low_prints_for_all_providers,
                enable_success_prints_for_all_providers: handle_config_params_enable_success_prints_for_all_providers,
                enable_partial_success_prints_for_all_providers: handle_config_params_enable_partial_success_prints_for_all_providers,
                enable_time_measurement_prints_for_all_providers: handle_config_params_enable_time_measurement_prints_for_all_providers,
                enable_cleaning_warning_logs_directory_prints_for_all_providers: handle_config_params_enable_cleaning_warning_logs_directory_prints_for_all_providers,
                enable_info_prints_for_all_providers: handle_config_params_enable_info_prints_for_all_providers,
                enable_write_error_logs_in_local_folder: handle_config_params_enable_write_error_logs_in_local_folder,
                enable_write_error_logs_in_mongo: handle_config_params_enable_write_error_logs_in_mongo,
                enable_initialize_mongo_with_providers_link_parts: handle_config_params_enable_initialize_mongo_with_providers_link_parts,
            },
            mongo_params: MongoParams {
                providers_db_name_handle: handle_config_mongo_params_providers_db_name_handle,
                providers_db_collection_handle_second_part: handle_config_mongo_params_providers_db_collection_handle_second_part,
                providers_db_collection_document_field_name_handle: handle_config_mongo_params_providers_db_collection_document_field_name_handle,
                db_providers_logs_name_handle: handle_config_mongo_params_db_providers_logs_name_handle,
                db_providers_logs_collection_handle_second_part: handle_config_mongo_params_db_providers_logs_collection_handle_second_part,
                db_providers_logs_collection_document_field_name_handle: handle_config_mongo_params_db_providers_logs_collection_document_field_name_handle,
                path_to_provider_link_parts_folder: handle_config_mongo_params_path_to_provider_link_parts_folder,
                log_file_extension: handle_config_mongo_params_log_file_extension,
                enable_initialize_mongo_with_providers_link_parts:
                    EnableInitializeMongoWithProvidersLinkParts {
                        enable_initialize_mongo_with_arxiv_link_parts: handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_arxiv_link_parts,
                        enable_initialize_mongo_with_biorxiv_link_parts: handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_biorxiv_link_parts,
                        enable_initialize_mongo_with_github_link_parts: handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_github_link_parts,
                        enable_initialize_mongo_with_habr_link_parts: handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_habr_link_parts,
                        enable_initialize_mongo_with_medrxiv_link_parts: handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_medrxiv_link_parts,
                        enable_initialize_mongo_with_reddit_link_parts: handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_reddit_link_parts,
                        enable_initialize_mongo_with_twitter_link_parts: handle_config_mongo_params_enable_initialize_mongo_with_providers_link_parts_enable_initialize_mongo_with_twitter_link_parts,
                    },
                mongo_url_parts: MongoUrlParts {
                    mongo_first_handle_url_part: handle_config_mongo_params_mongo_url_parts_mongo_first_handle_url_part,
                    mongo_second_handle_url_part: handle_config_mongo_params_mongo_url_parts_mongo_second_handle_url_part,
                    mongo_third_handle_url_part: handle_config_mongo_params_mongo_url_parts_mongo_third_handle_url_part,
                    mongo_fourth_handle_url_part: handle_config_mongo_params_mongo_url_parts_mongo_fourth_handle_url_part,
                    mongo_fifth_handle_url_part: handle_config_mongo_params_mongo_url_parts_mongo_fifth_handle_url_part,
                },
                mongo_authorization: MongoAuthorization {
                    mongo_login: handle_config_mongo_authorization_mongo_login,
                    mongo_password: handle_config_mongo_authorization_mongo_password,
                    mongo_ip: handle_config_mongo_authorization_mongo_ip,
                    mongo_port: handle_config_mongo_authorization_mongo_port,
                    mongo_params: handle_config_mongo_authorization_mongo_params,
                },
            },
            postgres_params: PostgresParams {
                postgres_url_parts: PostgresUrlParts {
                    postgres_first_handle_url_part: handle_config_postgres_params_postgres_url_parts_postgres_first_handle_url_part,
                postgres_second_handle_url_part: handle_config_postgres_params_postgres_url_parts_postgres_second_handle_url_part,
                postgres_third_handle_url_part: handle_config_postgres_params_postgres_url_parts_postgres_third_handle_url_part,
                postgres_fourth_handle_url_part: handle_config_postgres_params_postgres_url_parts_postgres_fourth_handle_url_part,
                postgres_fifth_handle_url_part: handle_config_postgres_params_postgres_url_parts_postgres_fifth_handle_url_part,
                },
                postgres_authorization: PostgresAuthorization {
                    postgres_login: handle_config_postgres_authorization_postgres_login,
                    postgres_password:
                        handle_config_postgres_authorization_postgres_password,
                    postgres_ip: handle_config_postgres_authorization_postgres_ip,
                    postgres_port: handle_config_postgres_authorization_postgres_port,
                    postgres_db: handle_config_postgres_authorization_postgres_db,
                },
            },
            enable_providers: EnableProviders {
                enable_arxiv: handle_config_enable_providers_enable_arxiv,
                enable_biorxiv: handle_config_enable_providers_enable_biorxiv,
                enable_github: handle_config_enable_providers_enable_github,
                enable_habr: handle_config_enable_providers_enable_habr,
                enable_medrxiv: handle_config_enable_providers_enable_medrxiv,
                enable_reddit: handle_config_enable_providers_enable_reddit,
                enable_twitter: handle_config_enable_providers_enable_twitter,
            },
            providers_check_links: ProvidersCheckLinks {
                arxiv_link: handle_config_providers_check_links_arxiv_link,
                biorxiv_link: handle_config_providers_check_links_biorxiv_link,
                github_link: handle_config_providers_check_links_github_link,
                habr_link: handle_config_providers_check_links_habr_link,
                medrxiv_link: handle_config_providers_check_links_medrxiv_link,
                reddit_link: handle_config_providers_check_links_reddit_link,
                twitter_link: handle_config_providers_check_links_twitter_link,
            },
            enable_providers_prints: EnableProvidersPrints {
                enable_prints_arxiv: handle_config_enable_providers_prints_enable_prints_arxiv,
                enable_prints_biorxiv: handle_config_enable_providers_prints_enable_prints_biorxiv,
                enable_prints_github: handle_config_enable_providers_prints_enable_prints_github,
                enable_prints_habr: handle_config_enable_providers_prints_enable_prints_habr,
                enable_prints_medrxiv: handle_config_enable_providers_prints_enable_prints_medrxiv,
                enable_prints_reddit: handle_config_enable_providers_prints_enable_prints_reddit,
                enable_prints_twitter: handle_config_enable_providers_prints_enable_prints_twitter,
            },
            enable_warning_high_providers_prints: EnableWarningHighProvidersPrints {
                enable_warning_high_prints_for_arxiv: handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_arxiv,
                enable_warning_high_prints_for_biorxiv: handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_biorxiv,
                enable_warning_high_prints_for_github: handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_github,
                enable_warning_high_prints_for_habr: handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_habr,
                enable_warning_high_prints_for_medrxiv: handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_medrxiv,
                enable_warning_high_prints_for_reddit: handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_reddit,
                enable_warning_high_prints_for_twitter: handle_config_enable_warning_high_providers_prints_enable_warning_high_prints_for_twitter,
            },
            enable_warning_low_providers_prints: EnableWarningLowProvidersPrints {
                enable_warning_low_prints_for_arxiv: handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_arxiv,
                enable_warning_low_prints_for_biorxiv: handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_biorxiv,
                enable_warning_low_prints_for_github: handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_github,
                enable_warning_low_prints_for_habr: handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_habr,
                enable_warning_low_prints_for_medrxiv: handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_medrxiv,
                enable_warning_low_prints_for_reddit: handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_reddit,
                enable_warning_low_prints_for_twitter: handle_config_enable_warning_low_providers_prints_enable_warning_low_prints_for_twitter,
            },
            enable_success_providers_prints: EnableSuccessProvidersPrints {
                enable_success_prints_for_arxiv: handle_config_enable_success_providers_prints_enable_success_prints_for_arxiv,
                enable_success_prints_for_biorxiv: handle_config_enable_success_providers_prints_enable_success_prints_for_biorxiv,
                enable_success_prints_for_github: handle_config_enable_success_providers_prints_enable_success_prints_for_github,
                enable_success_prints_for_habr: handle_config_enable_success_providers_prints_enable_success_prints_for_habr,
                enable_success_prints_for_medrxiv: handle_config_enable_success_providers_prints_enable_success_prints_for_medrxiv,
                enable_success_prints_for_reddit: handle_config_enable_success_providers_prints_enable_success_prints_for_reddit,
                enable_success_prints_for_twitter: handle_config_enable_success_providers_prints_enable_success_prints_for_twitter,
            },
            enable_partial_success_providers_prints: EnablePartialSuccessProvidersPrints {
                enable_partial_success_prints_for_arxiv: handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_arxiv,
                enable_partial_success_prints_for_biorxiv: handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_biorxiv,
                enable_partial_success_prints_for_github: handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_github,
                enable_partial_success_prints_for_habr: handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_habr,
                enable_partial_success_prints_for_medrxiv: handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_medrxiv,
                enable_partial_success_prints_for_reddit: handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_reddit,
                enable_partial_success_prints_for_twitter: handle_config_enable_partial_success_providers_prints_enable_partial_success_prints_for_twitter,
            },
            enable_error_providers_prints: EnableErrorProvidersPrints {
                enable_error_prints_for_arxiv: handle_config_enable_error_providers_prints_enable_error_prints_for_arxiv,
                enable_error_prints_for_biorxiv: handle_config_enable_error_providers_prints_enable_error_prints_for_biorxiv,
                enable_error_prints_for_github: handle_config_enable_error_providers_prints_enable_error_prints_for_github,
                enable_error_prints_for_habr: handle_config_enable_error_providers_prints_enable_error_prints_for_habr,
                enable_error_prints_for_medrxiv: handle_config_enable_error_providers_prints_enable_error_prints_for_medrxiv,
                enable_error_prints_for_reddit: handle_config_enable_error_providers_prints_enable_error_prints_for_reddit,
                enable_error_prints_for_twitter: handle_config_enable_error_providers_prints_enable_error_prints_for_twitter,
            },
            enable_providers_cleaning_warning_logs_directory:
                EnableProvidersCleaningWarningLogsDirectory {
                    enable_cleaning_warning_logs_directory_for_arxiv: handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_arxiv,
                    enable_cleaning_warning_logs_directory_for_biorxiv: handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_biorxiv,
                    enable_cleaning_warning_logs_directory_for_github: handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_github,
                    enable_cleaning_warning_logs_directory_for_habr: handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_habr,
                    enable_cleaning_warning_logs_directory_for_medrxiv: handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_medrxiv,
                    enable_cleaning_warning_logs_directory_for_reddit: handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_reddit,
                    enable_cleaning_warning_logs_directory_for_twitter: handle_config_enable_providers_cleaning_warning_logs_directory_enable_cleaning_warning_logs_directory_for_twitter,
                },
            enable_providers_cleaning_warning_logs_db_in_mongo:
                EnableProvidersCleaningWarningLogsDbInMongo {
                    enable_cleaning_warning_logs_db_in_mongo_for_arxiv: handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_arxiv,
                    enable_cleaning_warning_logs_db_in_mongo_for_biorxiv: handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_biorxiv,
                    enable_cleaning_warning_logs_db_in_mongo_for_github: handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_github,
                    enable_cleaning_warning_logs_db_in_mongo_for_habr: handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_habr,
                    enable_cleaning_warning_logs_db_in_mongo_for_medrxiv: handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_medrxiv,
                    enable_cleaning_warning_logs_db_in_mongo_for_reddit: handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_reddit,
                    enable_cleaning_warning_logs_db_in_mongo_for_twitter: handle_config_enable_providers_cleaning_warning_logs_db_in_mongo_enable_cleaning_warning_logs_db_in_mongo_for_twitter,
                },
            enable_providers_cleaning_warning_logs_db_collections_in_mongo:
                EnableProvidersCleaningWarningLogsDbCollectionsInMongo {
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv: handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv,
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv: handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv,
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_github: handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_github,
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_habr: handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_habr,
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv: handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv,
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit: handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit,
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter: handle_config_enable_providers_cleaning_warning_logs_db_collections_in_mongo_enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter,
                },
            enable_providers_time_measurement: EnableProvidersTimeMeasurement {
                enable_time_measurement_for_arxiv: handle_config_enable_providers_time_measurement_enable_time_measurement_for_arxiv,
                enable_time_measurement_for_biorxiv: handle_config_enable_providers_time_measurement_enable_time_measurement_for_biorxiv,
                enable_time_measurement_for_github: handle_config_enable_providers_time_measurement_enable_time_measurement_for_github,
                enable_time_measurement_for_habr: handle_config_enable_providers_time_measurement_enable_time_measurement_for_habr,
                enable_time_measurement_for_medrxiv: handle_config_enable_providers_time_measurement_enable_time_measurement_for_medrxiv,
                enable_time_measurement_for_reddit: handle_config_enable_providers_time_measurement_enable_time_measurement_for_reddit,
                enable_time_measurement_for_twitter: handle_config_enable_providers_time_measurement_enable_time_measurement_for_twitter,
            },
            enable_providers_info: EnableProvidersInfo {
                enable_info_for_arxiv: handle_config_enable_providers_info_enable_info_for_arxiv,
                enable_info_for_biorxiv: handle_config_enable_providers_info_enable_info_for_biorxiv,
                enable_info_for_github: handle_config_enable_providers_info_enable_info_for_github,
                enable_info_for_habr: handle_config_enable_providers_info_enable_info_for_habr,
                enable_info_for_medrxiv: handle_config_enable_providers_info_enable_info_for_medrxiv,
                enable_info_for_reddit: handle_config_enable_providers_info_enable_info_for_reddit,
                enable_info_for_twitter: handle_config_enable_providers_info_enable_info_for_twitter,
            },
            enable_providers_links_limits: EnableProvidersLinksLimit {
                enable_links_limit_for_arxiv: handle_config_enable_providers_links_limits_enable_links_limit_for_arxiv,
                enable_links_limit_for_biorxiv: handle_config_enable_providers_links_limits_enable_links_limit_for_biorxiv,
                enable_links_limit_for_github: handle_config_enable_providers_links_limits_enable_links_limit_for_github,
                enable_links_limit_for_habr: handle_config_enable_providers_links_limits_enable_links_limit_for_habr,
                enable_links_limit_for_medrxiv: handle_config_enable_providers_links_limits_enable_links_limit_for_medrxiv,
                enable_links_limit_for_reddit: handle_config_enable_providers_links_limits_enable_links_limit_for_reddit,
                enable_links_limit_for_twitter: handle_config_enable_providers_links_limits_enable_links_limit_for_twitter,
            },
            providers_links_limits: ProvidersLinksLimits {
                links_limit_for_arxiv: handle_config_providers_links_limits_links_limit_for_arxiv,
                links_limit_for_biorxiv: handle_config_providers_links_limits_links_limit_for_biorxiv,
                links_limit_for_github: handle_config_providers_links_limits_links_limit_for_github,
                links_limit_for_habr: handle_config_providers_links_limits_links_limit_for_habr,
                links_limit_for_medrxiv: handle_config_providers_links_limits_links_limit_for_medrxiv,
                links_limit_for_reddit: handle_config_providers_links_limits_links_limit_for_reddit,
                links_limit_for_twitter: handle_config_providers_links_limits_links_limit_for_twitter,
            },
            enable_randomize_order_for_providers_link_parts_for_mongo:
                EnableRandomizeOrderForProvidersLinkPartsForMongo {
                    enable_randomize_order_for_arxiv_link_parts_for_mongo: handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_arxiv_link_parts_for_mongo,
                    enable_randomize_order_for_biorxiv_link_parts_for_mongo: handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_biorxiv_link_parts_for_mongo,
                    enable_randomize_order_for_github_link_parts_for_mongo: handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_github_link_parts_for_mongo,
                    enable_randomize_order_for_habr_link_parts_for_mongo: handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_habr_link_parts_for_mongo,
                    enable_randomize_order_for_medrxiv_link_parts_for_mongo: handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_medrxiv_link_parts_for_mongo,
                    enable_randomize_order_for_reddit_link_parts_for_mongo: handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_reddit_link_parts_for_mongo,
                    enable_randomize_order_for_twitter_link_parts_for_mongo: handle_config_enable_randomize_order_for_providers_link_parts_for_mongo_enable_randomize_order_for_twitter_link_parts_for_mongo,
                },
            print_colors: PrintColors {
                error_red: handle_config_print_colors_error_red,
                error_green: handle_config_print_colors_error_green,
                error_blue: handle_config_print_colors_error_blue,
                warning_high_red: handle_config_print_colors_warning_high_red,
                warning_high_green: handle_config_print_colors_warning_high_green,
                warning_high_blue: handle_config_print_colors_warning_high_blue,
                warning_low_red: handle_config_print_colors_warning_low_red,
                warning_low_green: handle_config_print_colors_warning_low_green,
                warning_low_blue: handle_config_print_colors_warning_low_blue,
                success_red: handle_config_print_colors_success_red,
                success_green: handle_config_print_colors_success_green,
                success_blue: handle_config_print_colors_success_blue,
                partial_success_red: handle_config_print_colors_partial_success_red,
                partial_success_green: handle_config_print_colors_partial_success_green,
                partial_success_blue: handle_config_print_colors_partial_success_blue,
                cleaning_red: handle_config_print_colors_cleaning_red,
                cleaning_green: handle_config_print_colors_cleaning_green,
                cleaning_blue: handle_config_print_colors_cleaning_blue,
                time_measurement_red: handle_config_print_colors_time_measurement_red,
                time_measurement_green: handle_config_print_colors_time_measurement_green,
                time_measurement_blue: handle_config_print_colors_time_measurement_blue,
                info_red: handle_config_print_colors_info_red,
                info_green: handle_config_print_colors_info_green,
                info_blue: handle_config_print_colors_info_blue,
            },
        };
        return ConfigStruct::wrap_config_checks(handle_config);
    }
    fn wrap_config_checks(config_handle: ConfigStruct) -> Result<Self, ConfigError<'static>> {
        if !config_handle.github_authorization.github_name.is_empty() {
            let error: Result<ConfigStruct, ConfigError> =
                Err(ConfigError::Message("github_name is not valid".to_string()));
            drop(error);
        }
        if !config_handle.github_authorization.github_token.is_empty() {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "github_token is not valid".to_string(),
            ));
            drop(error);
        }
        if !config_handle
            .reddit_authorization
            .reddit_user_agent
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "reddit_user_agent is not valid".to_string(),
            ));
            drop(error);
        }
        if !config_handle
            .reddit_authorization
            .reddit_client_id
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "reddit_client_id is not valid".to_string(),
            ));
            drop(error);
        }
        if !config_handle
            .reddit_authorization
            .reddit_client_secret
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "reddit_client_secret is not valid".to_string(),
            ));
            drop(error);
        }
        if !config_handle
            .reddit_authorization
            .reddit_username
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "reddit_username is not valid".to_string(),
            ));
            drop(error);
        }
        if !config_handle
            .reddit_authorization
            .reddit_password
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "reddit_password is not valid".to_string(),
            ));
            drop(error);
        }
        if !config_handle
            .mongo_params.mongo_authorization
            .mongo_login
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_login is not valid".to_string(),
            ));
            drop(error);
        }
        if !config_handle
        .mongo_params
            .mongo_authorization
            .mongo_password
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_password is not valid".to_string(),
            ));
            drop(error);
        }
        if !config_handle
        .mongo_params
            .mongo_authorization
            .mongo_ip
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_ip is not valid".to_string(),
            ));
            drop(error);
        }
        if !config_handle
        .mongo_params
            .mongo_authorization
            .mongo_port
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_port is not valid".to_string(),
            ));
            drop(error);
        }
        if !config_handle
        .mongo_params
            .mongo_authorization
            .mongo_params
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "mongo_params is not valid".to_string(),
            ));
            drop(error);
        }
        //
        if config_handle.mongo_params.log_file_extension.is_empty() {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "log_file_extension is not empty".to_string(),
            ));
            drop(error);
        }
        if config_handle
            .mongo_params
            .path_to_provider_link_parts_folder
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "path_to_provider_link_parts_folder is empty"
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
                "db_collection_document_field_name_handle is empty"
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
                "db_collection_handle_second_part is empty"
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
                "db_name_handle is empty".to_string(),
            ));
            drop(error);
        }
        if config_handle
            .params
            .unhandled_success_handled_success_are_there_items_initialized_posts_dir
            .is_empty()
        {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                    "unhandled_success_handled_success_are_there_items_initialized_posts_dir is empty".to_string(),
                ));
            drop(error);
        }
        if config_handle.params.warning_logs_directory_name.is_empty() {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "warning_logs_directory_name is empty".to_string(),
            ));
            drop(error);
        }
        if config_handle.params.common_providers_links_limit > 0 {
            let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
                "common_providers_links_limit <= 0".to_string(),
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
        if config_handle.providers_links_limits.links_limit_for_arxiv <= 0 {
            return false
        }
        if config_handle.providers_links_limits.links_limit_for_biorxiv <= 0 {
            return false
        }
        if config_handle.providers_links_limits.links_limit_for_github <= 0 {
            return false
        }
        if config_handle.providers_links_limits.links_limit_for_habr <= 0 {
            return false
        }
        if config_handle.providers_links_limits.links_limit_for_medrxiv <= 0 {
            return false
        }
        if config_handle.providers_links_limits.links_limit_for_reddit <= 0 {
            return false
        }
        if config_handle.providers_links_limits.links_limit_for_twitter <= 0 {
            return false
        }
        true
    }
}
