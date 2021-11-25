use crate::constants::env_var_names_constants::COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_ERROR_PRINTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INFO_PRINTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PRINTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PROVIDERS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_SUCCESS_PRINTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_TIME_MEASUREMENT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME;
use crate::constants::env_var_names_constants::STARTING_CHECK_LINK_ENV_NAME;
use crate::constants::env_var_names_constants::UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME;
use crate::constants::env_var_names_constants::WARNING_LOGS_DIRECTORY_NAME_ENV_NAME;

// [mongo_params]
use crate::constants::env_var_names_constants::DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME;
use crate::constants::env_var_names_constants::DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME;
use crate::constants::env_var_names_constants::DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::constants::env_var_names_constants::LOG_FILE_EXTENSION_ENV_NAME;
use crate::constants::env_var_names_constants::PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME;
use crate::constants::env_var_names_constants::PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME;
use crate::constants::env_var_names_constants::PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME;
use crate::constants::env_var_names_constants::PROVIDERS_DB_NAME_HANDLE_ENV_NAME;

// [mongo_params.mongo_url_parts]
use crate::constants::env_var_names_constants::MONGO_FIFTH_HANDLE_URL_PART_ENV_NAME;
use crate::constants::env_var_names_constants::MONGO_FIRST_HANDLE_URL_PART_ENV_NAME;
use crate::constants::env_var_names_constants::MONGO_FOURTH_HANDLE_URL_PART_ENV_NAME;
use crate::constants::env_var_names_constants::MONGO_SECOND_HANDLE_URL_PART_ENV_NAME;
use crate::constants::env_var_names_constants::MONGO_THIRD_HANDLE_URL_PART_ENV_NAME;

// [mongo_params.enable_initialize_mongo_with_providers_link_parts]
use crate::constants::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME;

// [postgres_params]
use crate::constants::env_var_names_constants::POSTGRES_FIFTH_HANDLE_URL_PART_ENV_NAME;
use crate::constants::env_var_names_constants::POSTGRES_FIRST_HANDLE_URL_PART_ENV_NAME;
use crate::constants::env_var_names_constants::POSTGRES_FOURTH_HANDLE_URL_PART_ENV_NAME;
use crate::constants::env_var_names_constants::POSTGRES_SECOND_HANDLE_URL_PART_ENV_NAME;
use crate::constants::env_var_names_constants::POSTGRES_THIRD_HANDLE_URL_PART_ENV_NAME;

// [enable_providers]
use crate::constants::env_var_names_constants::ARXIV_LINK_ENV_NAME;
use crate::constants::env_var_names_constants::BIORXIV_LINK_ENV_NAME;
use crate::constants::env_var_names_constants::GITHUB_LINK_ENV_NAME;
use crate::constants::env_var_names_constants::HABR_LINK_ENV_NAME;
use crate::constants::env_var_names_constants::MEDRXIV_LINK_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_LINK_ENV_NAME;
use crate::constants::env_var_names_constants::TWITTER_LINK_ENV_NAME;

// [providers_check_links]
use crate::constants::env_var_names_constants::ENABLE_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_TWITTER_ENV_NAME;

// [enable_providers_prints]
use crate::constants::env_var_names_constants::ENABLE_PRINTS_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PRINTS_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PRINTS_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PRINTS_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PRINTS_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PRINTS_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PRINTS_TWITTER_ENV_NAME;

// [enable_warning_high_providers_prints]
use crate::constants::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_warning_low_providers_prints]
use crate::constants::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_error_providers_prints]
use crate::constants::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_success_providers_prints]
use crate::constants::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_partial_success_providers_prints]
use crate::constants::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME;

// [enable_providers_cleaning_warning_logs_directory]
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME;

// [enable_providers_cleaning_warning_logs_db_in_mongo]
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME;

// [enable_providers_cleaning_warning_logs_db_collections_in_mongo]
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME;

// [enable_providers_time_measurement]
use crate::constants::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME;

// [enable_providers_info]
use crate::constants::env_var_names_constants::ENABLE_INFO_FOR_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INFO_FOR_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INFO_FOR_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INFO_FOR_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INFO_FOR_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INFO_FOR_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_INFO_FOR_TWITTER_ENV_NAME;

//[providers_links_limits]
use crate::constants::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME;

// [enable_randomize_order_for_providers_link_parts_for_mongo]
use crate::constants::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME;
use crate::constants::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME;

// [providers_links_limits]
use crate::constants::env_var_names_constants::LINKS_LIMIT_FOR_ARXIV_ENV_NAME;
use crate::constants::env_var_names_constants::LINKS_LIMIT_FOR_BIORXIV_ENV_NAME;
use crate::constants::env_var_names_constants::LINKS_LIMIT_FOR_GITHUB_ENV_NAME;
use crate::constants::env_var_names_constants::LINKS_LIMIT_FOR_HABR_ENV_NAME;
use crate::constants::env_var_names_constants::LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME;
use crate::constants::env_var_names_constants::LINKS_LIMIT_FOR_REDDIT_ENV_NAME;
use crate::constants::env_var_names_constants::LINKS_LIMIT_FOR_TWITTER_ENV_NAME;

// [print_colors]
use crate::constants::env_var_names_constants::ERROR_BLUE_ENV_NAME;
use crate::constants::env_var_names_constants::ERROR_GREEN_ENV_NAME;
use crate::constants::env_var_names_constants::ERROR_RED_ENV_NAME;

use crate::constants::env_var_names_constants::WARNING_HIGH_BLUE_ENV_NAME;
use crate::constants::env_var_names_constants::WARNING_HIGH_GREEN_ENV_NAME;
use crate::constants::env_var_names_constants::WARNING_HIGH_RED_ENV_NAME;

use crate::constants::env_var_names_constants::WARNING_LOW_BLUE_ENV_NAME;
use crate::constants::env_var_names_constants::WARNING_LOW_GREEN_ENV_NAME;
use crate::constants::env_var_names_constants::WARNING_LOW_RED_ENV_NAME;

use crate::constants::env_var_names_constants::SUCCESS_BLUE_ENV_NAME;
use crate::constants::env_var_names_constants::SUCCESS_GREEN_ENV_NAME;
use crate::constants::env_var_names_constants::SUCCESS_RED_ENV_NAME;

use crate::constants::env_var_names_constants::PARTIAL_SUCCESS_BLUE_ENV_NAME;
use crate::constants::env_var_names_constants::PARTIAL_SUCCESS_GREEN_ENV_NAME;
use crate::constants::env_var_names_constants::PARTIAL_SUCCESS_RED_ENV_NAME;

use crate::constants::env_var_names_constants::CLEANING_BLUE_ENV_NAME;
use crate::constants::env_var_names_constants::CLEANING_GREEN_ENV_NAME;
use crate::constants::env_var_names_constants::CLEANING_RED_ENV_NAME;

use crate::constants::env_var_names_constants::TIME_MEASUREMENT_BLUE_ENV_NAME;
use crate::constants::env_var_names_constants::TIME_MEASUREMENT_GREEN_ENV_NAME;
use crate::constants::env_var_names_constants::TIME_MEASUREMENT_RED_ENV_NAME;

use crate::constants::env_var_names_constants::INFO_BLUE_ENV_NAME;
use crate::constants::env_var_names_constants::INFO_GREEN_ENV_NAME;
use crate::constants::env_var_names_constants::INFO_RED_ENV_NAME;

use crate::constants::env_var_names_constants::GITHUB_NAME_ENV_NAME;
use crate::constants::env_var_names_constants::GITHUB_TOKEN_ENV_NAME;

use crate::constants::env_var_names_constants::MONGO_IP_ENV_NAME;
use crate::constants::env_var_names_constants::MONGO_LOGIN_ENV_NAME;
use crate::constants::env_var_names_constants::MONGO_PARAMS_ENV_NAME;
use crate::constants::env_var_names_constants::MONGO_PASSWORD_ENV_NAME;
use crate::constants::env_var_names_constants::MONGO_PORT_ENV_NAME;

use crate::constants::env_var_names_constants::POSTGRES_DB_ENV_NAME;
use crate::constants::env_var_names_constants::POSTGRES_IP_ENV_NAME;
use crate::constants::env_var_names_constants::POSTGRES_LOGIN_ENV_NAME;
use crate::constants::env_var_names_constants::POSTGRES_PASSWORD_ENV_NAME;
use crate::constants::env_var_names_constants::POSTGRES_PORT_ENV_NAME;

use crate::constants::env_var_names_constants::REDDIT_CLIENT_ID_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_CLIENT_SECRET_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_PASSWORD_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_USERNAME_ENV_NAME;
use crate::constants::env_var_names_constants::REDDIT_USER_AGENT_ENV_NAME;

use crate::config_mods::env_var_enum::EnvVar;

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
            EnvVar::WarningLogsDirectoryName => WARNING_LOGS_DIRECTORY_NAME_ENV_NAME,
            EnvVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir => {
                UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME
            }
            EnvVar::EnableProviders => ENABLE_PROVIDERS_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDirectory => {
                ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbInMongo => {
                ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongo => {
                ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME
            }
            EnvVar::EnableTimeMeasurement => ENABLE_TIME_MEASUREMENT_ENV_NAME,
            EnvVar::EnableProviderLinksLimit => ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME,
            EnvVar::EnableCommonProvidersLinksLimit => ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME,
            EnvVar::CommonProvidersLinksLimit => COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME,
            EnvVar::EnableRandomizeOrderForProvidersLinkPartsForMongo => {
                ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME
            }
            EnvVar::EnablePrints => ENABLE_PRINTS_ENV_NAME,
            EnvVar::EnableErrorPrints => ENABLE_ERROR_PRINTS_ENV_NAME,
            EnvVar::EnableWarningHighPrints => ENABLE_WARNING_HIGH_PRINTS_ENV_NAME,
            EnvVar::EnableWarningLowPrints => ENABLE_WARNING_LOW_PRINTS_ENV_NAME,
            EnvVar::EnableSuccessPrints => ENABLE_SUCCESS_PRINTS_ENV_NAME,
            EnvVar::EnablePartialSuccessPrints => ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME,
            EnvVar::EnableTimeMeasurementPrints => ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME,
            EnvVar::EnableCleaningWarningLogsDirectoryPrints => {
                ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME
            }
            EnvVar::EnableInfoPrints => ENABLE_INFO_PRINTS_ENV_NAME,
            EnvVar::EnableAllProvidersPrints => ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME,
            EnvVar::EnableErrorPrintsForAllProviders => {
                ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME
            }
            EnvVar::EnableWarningHighPrintsForAllProviders => {
                ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME
            }
            EnvVar::EnableWarningLowPrintsForAllProviders => {
                ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME
            }
            EnvVar::EnableSuccessPrintsForAllProviders => {
                ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME
            }
            EnvVar::EnablePartialSuccessPrintsForAllProviders => {
                ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME
            }
            EnvVar::EnableTimeMeasurementPrintsForAllProviders => {
                ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDirectoryPrintsForAllProviders => {
                ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME
            }
            EnvVar::EnableInfoPrintsForAllProviders => {
                ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME
            }
            EnvVar::EnableWriteErrorLogsInLocalFolder => {
                ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME
            }
            EnvVar::EnableWriteErrorLogsInMongo => ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME,
            EnvVar::EnableInitializeMongoWithProvidersLinkParts => {
                ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME
            }

            EnvVar::ProvidersDbNameHandle => PROVIDERS_DB_NAME_HANDLE_ENV_NAME,
            EnvVar::ProvidersDbCollectionHandleSecondPart => {
                PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME
            }
            EnvVar::ProvidersDbCollectionDocumentFieldNameHandle => {
                PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME
            }
            EnvVar::PathToProviderLinkPartsFolder => PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME,
            EnvVar::LogFileExtension => LOG_FILE_EXTENSION_ENV_NAME,
            EnvVar::DbProvidersLogsNameHandle => DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME,
            EnvVar::DbProvidersLogsCollectionHandleSecondPart => {
                DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME
            }
            EnvVar::DbProvidersLogsCollectionDocumentFieldNameHandle => {
                DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME
            }

            EnvVar::EnableInitializeMongoWithArxivLinkParts => {
                ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME
            }
            EnvVar::EnableInitializeMongoWithBiorxivLinkParts => {
                ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME
            }
            EnvVar::EnableInitializeMongoWithGithubLinkParts => {
                ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME
            }
            EnvVar::EnableInitializeMongoWithHabrLinkParts => {
                ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME
            }
            EnvVar::EnableInitializeMongoWithMedrxivLinkParts => {
                ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME
            }
            EnvVar::EnableInitializeMongoWithRedditLinkParts => {
                ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME
            }
            EnvVar::EnableInitializeMongoWithTwitterLinkParts => {
                ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME
            }

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

            EnvVar::EnableWarningHighPrintsForArxiv => {
                ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME
            }
            EnvVar::EnableWarningHighPrintsForBiorxiv => {
                ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME
            }
            EnvVar::EnableWarningHighPrintsForGithub => {
                ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME
            }
            EnvVar::EnableWarningHighPrintsForHabr => ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME,
            EnvVar::EnableWarningHighPrintsForMedrxiv => {
                ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME
            }
            EnvVar::EnableWarningHighPrintsForReddit => {
                ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME
            }
            EnvVar::EnableWarningHighPrintsForTwitter => {
                ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME
            }

            EnvVar::EnableWarningLowPrintsForArxiv => ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvVar::EnableWarningLowPrintsForBiorxiv => {
                ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME
            }
            EnvVar::EnableWarningLowPrintsForGithub => {
                ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME
            }
            EnvVar::EnableWarningLowPrintsForHabr => ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME,
            EnvVar::EnableWarningLowPrintsForMedrxiv => {
                ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME
            }
            EnvVar::EnableWarningLowPrintsForReddit => {
                ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME
            }
            EnvVar::EnableWarningLowPrintsForTwitter => {
                ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME
            }

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

            EnvVar::EnablePartialSuccessPrintsForArxiv => {
                ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME
            }
            EnvVar::EnablePartialSuccessPrintsForBiorxiv => {
                ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME
            }
            EnvVar::EnablePartialSuccessPrintsForGithub => {
                ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME
            }
            EnvVar::EnablePartialSuccessPrintsForHabr => {
                ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME
            }
            EnvVar::EnablePartialSuccessPrintsForMedrxiv => {
                ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME
            }
            EnvVar::EnablePartialSuccessPrintsForReddit => {
                ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME
            }
            EnvVar::EnablePartialSuccessPrintsForTwitter => {
                ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME
            }

            EnvVar::EnableCleaningWarningLogsDirectoryForArxiv => {
                ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDirectoryForBiorxiv => {
                ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDirectoryForGithub => {
                ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDirectoryForHabr => {
                ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDirectoryForMedrxiv => {
                ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDirectoryForReddit => {
                ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDirectoryForTwitter => {
                ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME
            }

            EnvVar::EnableCleaningWarningLogsDbInMongoForArxiv => {
                ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbInMongoForBiorxiv => {
                ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbInMongoForGithub => {
                ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbInMongoForHabr => {
                ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbInMongoForMedrxiv => {
                ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbInMongoForReddit => {
                ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbInMongoForTwitter => {
                ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME
            }

            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv => {
                ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv => {
                ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForGithub => {
                ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForHabr => {
                ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv => {
                ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForReddit => {
                ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter => {
                ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME
            }

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

            EnvVar::EnableRandomizeOrderForArxivLinkPartsForMongo => {
                ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME
            }
            EnvVar::EnableRandomizeOrderForBiorxivLinkPartsForMongo => {
                ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME
            }
            EnvVar::EnableRandomizeOrderForGithubLinkPartsForMongo => {
                ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME
            }
            EnvVar::EnableRandomizeOrderForHabrLinkPartsForMongo => {
                ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME
            }
            EnvVar::EnableRandomizeOrderForMedrxivLinkPartsForMongo => {
                ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME
            }
            EnvVar::EnableRandomizeOrderForRedditLinkPartsForMongo => {
                ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME
            }
            EnvVar::EnableRandomizeOrderForTwitterLinkPartsForMongo => {
                ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME
            }

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
}
