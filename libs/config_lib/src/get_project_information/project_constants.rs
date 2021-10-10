use crate::get_project_information::get_config::get_lazy_config_information::CONFIG;

pub const ENV_FILE_NAME: &str = ".env";

pub const LOAD_CONFIG_FILE_ERROR_MESSAGE: &str = "cannot create config";

pub const GITHUB_NAME_ENV_NAME: &str = "GITHUB_NAME";
pub const GITHUB_TOKEN_ENV_NAME: &str = "GITHUB_TOKEN";

pub const REDDIT_USER_AGENT_ENV_NAME: &str = "REDDIT_USER_AGENT";
pub const REDDIT_CLIENT_ID_ENV_NAME: &str = "REDDIT_CLIENT_ID";
pub const REDDIT_CLIENT_SECRET_ENV_NAME: &str = "REDDIT_CLIENT_SECRET";
pub const REDDIT_USERNAME_ENV_NAME: &str = "REDDIT_USERNAME";
pub const REDDIT_PASSWORD_ENV_NAME: &str = "REDDIT_PASSWORD";

pub const MONGO_LOGIN_ENV_NAME: &str = "MONGO_LOGIN";
pub const MONGO_PASSWORD_ENV_NAME: &str = "MONGO_PASSWORD";
pub const MONGO_IP_ENV_NAME: &str = "MONGO_IP";
pub const MONGO_PORT_ENV_NAME: &str = "MONGO_PORT";
pub const MONGO_PARAMS_ENV_NAME: &str = "MONGO_PARAMS";

pub const POSTGRES_LOGIN_ENV_NAME: &str = "POSTGRES_LOGIN";
pub const POSTGRES_PASSWORD_ENV_NAME: &str = "POSTGRES_PASSWORD";
pub const POSTGRES_IP_ENV_NAME: &str = "POSTGRES_IP";
pub const POSTGRES_PORT_ENV_NAME: &str = "POSTGRES_PORT";
pub const POSTGRES_DB_ENV_NAME: &str = "POSTGRES_DB";

//[params]
pub const STARTING_CHECK_LINK_ENV_NAME: &str = "STARTING_CHECK_LINK";
pub const USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME: &str = "USER_CREDENTIALS_DUMMY_HANDLE";
pub const WARNING_LOGS_DIRECTORY_NAME_ENV_NAME: &str = "WARNING_LOGS_DIRECTORY_NAME";
pub const UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME: &str =
    "UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR";
pub const ENABLE_PROVIDERS_ENV_NAME: &str = "ENABLE_PROVIDERS";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO";
pub const ENABLE_TIME_MEASUREMENT_ENV_NAME: &str = "ENABLE_TIME_MEASUREMENT";
pub const ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME: &str = "ENABLE_PROVIDER_LINKS_LIMIT";
pub const ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME: &str =
    "ENABLE_COMMON_PROVIDERS_LINKS_LIMIT";
pub const COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME: &str = "COMMON_PROVIDERS_LINKS_LIMIT";
pub const ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO";
pub const ENABLE_PRINTS_ENV_NAME: &str = "ENABLE_PRINTS";
pub const ENABLE_ERROR_PRINTS_ENV_NAME: &str = "ENABLE_ERROR_PRINTS";
pub const ENABLE_WARNING_HIGH_PRINTS_ENV_NAME: &str = "ENABLE_WARNING_HIGH_PRINTS";
pub const ENABLE_WARNING_LOW_PRINTS_ENV_NAME: &str = "ENABLE_WARNING_LOW_PRINTS";
pub const ENABLE_SUCCESS_PRINTS_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME: &str = "ENABLE_PARTIAL_SUCCESS_PRINTS";
pub const ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME: &str = "ENABLE_TIME_MEASUREMENT_PRINTS";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS";
pub const ENABLE_INFO_PRINTS_ENV_NAME: &str = "ENABLE_INFO_PRINTS";
pub const ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME: &str = "ENABLE_ALL_PROVIDERS_PRINTS";
pub const ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME: &str =
    "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER";
pub const ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME: &str = "ENABLE_WRITE_ERROR_LOGS_IN_MONGO";
pub const ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS";

//[mongo_params]
pub const MONGO_IS_CLOUD_ENV_NAME: &str = "MONGO_IS_CLOUD";
pub const PROVIDERS_DB_NAME_HANDLE_ENV_NAME: &str = "PROVIDERS_DB_NAME_HANDLE";
pub const PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME: &str =
    "PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART";
pub const PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME: &str =
    "PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE";
pub const PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME: &str = "PATH_TO_PROVIDER_LINK_PARTS_FOLDER";
pub const LOG_FILE_EXTENSION_ENV_NAME: &str = "LOG_FILE_EXTENSION";
pub const DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME: &str = "DB_PROVIDERS_LOGS_NAME_HANDLE";
pub const DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME: &str =
    "DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART";
pub const DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME: &str =
    "DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE";

//[mongo_params.mongo_url_parts]
pub const MONGO_FIRST_HANDLE_URL_PART_ENV_NAME: &str = "MONGO_FIRST_HANDLE_URL_PART";
pub const MONGO_SECOND_HANDLE_URL_PART_ENV_NAME: &str = "MONGO_SECOND_HANDLE_URL_PART";
pub const MONGO_THIRD_HANDLE_URL_PART_ENV_NAME: &str = "MONGO_THIRD_HANDLE_URL_PART";
pub const MONGO_FOURTH_HANDLE_URL_PART_ENV_NAME: &str = "MONGO_FOURTH_HANDLE_URL_PART";

//[mongo_params.enable_initialize_mongo_with_providers_link_parts]
pub const ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS";
pub const ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS";
pub const ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS";
pub const ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS";
pub const ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS";
pub const ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS";
pub const ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS";

//[postgres_params]
pub const POSTGRES_IS_CLOUD_ENV_NAME: &str = "POSTGRES_IS_CLOUD";
pub const POSTGRES_OWN_FIRST_HANDLE_URL_PART_ENV_NAME: &str = "POSTGRES_OWN_FIRST_HANDLE_URL_PART";
pub const POSTGRES_OWN_SECOND_HANDLE_URL_PART_ENV_NAME: &str =
    "POSTGRES_OWN_SECOND_HANDLE_URL_PART";
pub const POSTGRES_OWN_THIRD_HANDLE_URL_PART_ENV_NAME: &str = "POSTGRES_OWN_THIRD_HANDLE_URL_PART";
pub const POSTGRES_OWN_FOURTH_HANDLE_URL_PART_ENV_NAME: &str =
    "POSTGRES_OWN_FOURTH_HANDLE_URL_PART";

//[enable_providers]
pub const ENABLE_ARXIV_ENV_NAME: &str = "ENABLE_ARXIV";
pub const ENABLE_BIORXIV_ENV_NAME: &str = "ENABLE_BIORXIV";
pub const ENABLE_GITHUB_ENV_NAME: &str = "ENABLE_GITHUB";
pub const ENABLE_HABR_ENV_NAME: &str = "ENABLE_HABR";
pub const ENABLE_MEDRXIV_ENV_NAME: &str = "ENABLE_MEDRXIV";
pub const ENABLE_REDDIT_ENV_NAME: &str = "ENABLE_REDDIT";
pub const ENABLE_TWITTER_ENV_NAME: &str = "ENABLE_TWITTER";

//[providers_check_links]
pub const ARXIV_LINK_ENV_NAME: &str = "ARXIV_LINK";
pub const BIORXIV_LINK_ENV_NAME: &str = "BIORXIV_LINK";
pub const GITHUB_LINK_ENV_NAME: &str = "GITHUB_LINK";
pub const HABR_LINK_ENV_NAME: &str = "HABR_LINK";
pub const MEDRXIV_LINK_ENV_NAME: &str = "MEDRXIV_LINK";
pub const REDDIT_LINK_ENV_NAME: &str = "REDDIT_LINK";
pub const TWITTER_LINK_ENV_NAME: &str = "TWITTER_LINK";

//[enable_providers_prints]
pub const ENABLE_PRINTS_ARXIV_ENV_NAME: &str = "ENABLE_PRINTS_ARXIV";
pub const ENABLE_PRINTS_BIORXIV_ENV_NAME: &str = "ENABLE_PRINTS_BIORXIV";
pub const ENABLE_PRINTS_GITHUB_ENV_NAME: &str = "ENABLE_PRINTS_GITHUB";
pub const ENABLE_PRINTS_HABR_ENV_NAME: &str = "ENABLE_PRINTS_HABR";
pub const ENABLE_PRINTS_MEDRXIV_ENV_NAME: &str = "ENABLE_PRINTS_MEDRXIV";
pub const ENABLE_PRINTS_REDDIT_ENV_NAME: &str = "ENABLE_PRINTS_REDDIT";
pub const ENABLE_PRINTS_TWITTER_ENV_NAME: &str = "ENABLE_PRINTS_TWITTER";

//[enable_warning_high_providers_prints]
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_HABR";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER";

//[enable_warning_low_providers_prints]
pub const ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME: &str = "ENABLE_WARNING_LOW_PRINTS_FOR_HABR";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER";

//[enable_error_providers_prints]
pub const ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_ARXIV";
pub const ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_BIORXIV";
pub const ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_GITHUB";
pub const ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_HABR";
pub const ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_MEDRXIV";
pub const ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_REDDIT";
pub const ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_TWITTER";

//[enable_success_providers_prints]
pub const ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_ARXIV";
pub const ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_BIORXIV";
pub const ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_GITHUB";
pub const ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_HABR";
pub const ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV";
pub const ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_REDDIT";
pub const ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_TWITTER";

//[enable_partial_success_providers_prints]
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER";

//[enable_providers_cleaning_warning_logs_directory]
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER";

//[enable_providers_cleaning_warning_logs_db_in_mongo]
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER";

//[enable_providers_cleaning_warning_logs_db_collections_in_mongo]
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER";

//[enable_providers_time_measurement]
pub const ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME: &str = "ENABLE_TIME_MEASUREMENT_FOR_ARXIV";
pub const ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_TIME_MEASUREMENT_FOR_BIORXIV";
pub const ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME: &str = "ENABLE_TIME_MEASUREMENT_FOR_GITHUB";
pub const ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME: &str = "ENABLE_TIME_MEASUREMENT_FOR_HABR";
pub const ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV";
pub const ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME: &str = "ENABLE_TIME_MEASUREMENT_FOR_REDDIT";
pub const ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_TIME_MEASUREMENT_FOR_TWITTER";

//[enable_providers_info]
pub const ENABLE_INFO_FOR_ARXIV_ENV_NAME: &str = "ENABLE_INFO_FOR_ARXIV";
pub const ENABLE_INFO_FOR_BIORXIV_ENV_NAME: &str = "ENABLE_INFO_FOR_BIORXIV";
pub const ENABLE_INFO_FOR_GITHUB_ENV_NAME: &str = "ENABLE_INFO_FOR_GITHUB";
pub const ENABLE_INFO_FOR_HABR_ENV_NAME: &str = "ENABLE_INFO_FOR_HABR";
pub const ENABLE_INFO_FOR_MEDRXIV_ENV_NAME: &str = "ENABLE_INFO_FOR_MEDRXIV";
pub const ENABLE_INFO_FOR_REDDIT_ENV_NAME: &str = "ENABLE_INFO_FOR_REDDIT";
pub const ENABLE_INFO_FOR_TWITTER_ENV_NAME: &str = "ENABLE_INFO_FOR_TWITTER";

//[enable_providers_links_limits]
pub const ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_ARXIV";
pub const ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_BIORXIV";
pub const ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_GITHUB";
pub const ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_HABR";
pub const ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_MEDRXIV";
pub const ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_REDDIT";
pub const ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_TWITTER";

//[enable_randomize_order_for_providers_link_parts_for_mongo]
pub const ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO";
pub const ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO";
pub const ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO";
pub const ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO";
pub const ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO";
pub const ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO";
pub const ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO";

//[providers_links_limits]
pub const LINKS_LIMIT_FOR_ARXIV_ENV_NAME: &str = "LINKS_LIMIT_FOR_ARXIV";
pub const LINKS_LIMIT_FOR_BIORXIV_ENV_NAME: &str = "LINKS_LIMIT_FOR_BIORXIV";
pub const LINKS_LIMIT_FOR_GITHUB_ENV_NAME: &str = "LINKS_LIMIT_FOR_GITHUB";
pub const LINKS_LIMIT_FOR_HABR_ENV_NAME: &str = "LINKS_LIMIT_FOR_HABR";
pub const LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME: &str = "LINKS_LIMIT_FOR_MEDRXIV";
pub const LINKS_LIMIT_FOR_REDDIT_ENV_NAME: &str = "LINKS_LIMIT_FOR_REDDIT";
pub const LINKS_LIMIT_FOR_TWITTER_ENV_NAME: &str = "LINKS_LIMIT_FOR_TWITTER";

//[print_colors]
pub const ERROR_RED_ENV_NAME: &str = "ERROR_RED";
pub const ERROR_GREEN_ENV_NAME: &str = "ERROR_GREEN";
pub const ERROR_BLUE_ENV_NAME: &str = "ERROR_BLUE";
pub const WARNING_HIGH_RED_ENV_NAME: &str = "WARNING_HIGH_RED";
pub const WARNING_HIGH_GREEN_ENV_NAME: &str = "WARNING_HIGH_GREEN";
pub const WARNING_HIGH_BLUE_ENV_NAME: &str = "WARNING_HIGH_BLUE";
pub const WARNING_LOW_RED_ENV_NAME: &str = "WARNING_LOW_RED";
pub const WARNING_LOW_GREEN_ENV_NAME: &str = "WARNING_LOW_GREEN";
pub const WARNING_LOW_BLUE_ENV_NAME: &str = "WARNING_LOW_BLUE";
pub const SUCCESS_RED_ENV_NAME: &str = "SUCCESS_RED";
pub const SUCCESS_GREEN_ENV_NAME: &str = "SUCCESS_GREEN";
pub const SUCCESS_BLUE_ENV_NAME: &str = "SUCCESS_BLUE";
pub const PARTIAL_SUCCESS_RED_ENV_NAME: &str = "PARTIAL_SUCCESS_RED";
pub const PARTIAL_SUCCESS_GREEN_ENV_NAME: &str = "PARTIAL_SUCCESS_GREEN";
pub const PARTIAL_SUCCESS_BLUE_ENV_NAME: &str = "PARTIAL_SUCCESS_BLUE";
pub const CLEANING_RED_ENV_NAME: &str = "CLEANING_RED";
pub const CLEANING_GREEN_ENV_NAME: &str = "CLEANING_GREEN";
pub const CLEANING_BLUE_ENV_NAME: &str = "CLEANING_BLUE";
pub const TIME_MEASUREMENT_RED_ENV_NAME: &str = "TIME_MEASUREMENT_RED";
pub const TIME_MEASUREMENT_GREEN_ENV_NAME: &str = "TIME_MEASUREMENT_GREEN";
pub const TIME_MEASUREMENT_BLUE_ENV_NAME: &str = "TIME_MEASUREMENT_BLUE";
pub const INFO_RED_ENV_NAME: &str = "INFO_RED";
pub const INFO_GREEN_ENV_NAME: &str = "INFO_GREEN";
pub const INFO_BLUE_ENV_NAME: &str = "INFO_BLUE";

///////////////////////////////////

pub const LOGS_COMMON_FOLDER_NAME: &str = "common_folder";

pub const ARXIV_NAME_TO_CHECK: &str = "arxiv";
pub const BIORXIV_NAME_TO_CHECK: &str = "biorxiv";
pub const GITHUB_NAME_TO_CHECK: &str = "github";
pub const HABR_NAME_TO_CHECK: &str = "habr";
pub const MEDRXIV_NAME_TO_CHECK: &str = "medrxiv";
pub const REDDIT_NAME_TO_CHECK: &str = "reddit";
pub const TWITTER_NAME_TO_CHECK: &str = "twitter";

pub const COMMON_PROVIDER_ITEM_HANDLE: &str = "</item>";
pub const GITHUB_PROVIDER_ITEM_HANDLE: &str = "</entry>";

pub const TWITTER_FILTER_HANDLE_TO_REMOVE_1: &str = "<dc:creator>";
pub const TWITTER_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "bbb<creator>";
pub const TWITTER_FILTER_HANDLE_TO_REMOVE_2: &str = "</dc:creator>";
pub const TWITTER_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "bbb</creator>";
pub const TWITTER_FILTER_HANDLE_TO_REMOVE_3: &str = "<atom:link";
pub const TWITTER_FILTER_HANDLE_TO_REPLACE_REMOVED_3: &str = "<atomllink";

pub const MEDRXIV_FILTER_HANDLE_TO_REMOVE_1: &str = "<dc:title>";
pub const MEDRXIV_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "<dccfifle>";
pub const MEDRXIV_FILTER_HANDLE_TO_REMOVE_2: &str = "</dc:title>";
pub const MEDRXIV_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "</dccfifle>";

pub const BIORXIV_FILTER_HANDLE_TO_REMOVE_1: &str = "<dc:title>";
pub const BIORXIV_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "<dcstitle>";
pub const BIORXIV_FILTER_HANDLE_TO_REMOVE_2: &str = "</dc:title>";
pub const BIORXIV_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "</dcstitle>";

pub const HABR_FILTER_HANDLE_TO_REMOVE_1: &str = "<channel>";
pub const HABR_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "         ";
pub const HABR_FILTER_HANDLE_TO_REMOVE_2: &str = "</channel>";
pub const HABR_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "         ";

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn get_mongo_url() -> String {
    let mongo_url: String;
    if CONFIG.mongo_params.mongo_is_cloud {
        // let mongo_cloud_first_handle_url_part = &CONFIG
        //     .mongo_params
        //     .enable_mongo_cloud_url_parts
        //     .mongo_cloud_first_handle_url_part;
        // let mongo_cloud_login = &CONFIG.mongo_cloud_authorization.mongo_cloud_login;
        // let mongo_cloud_second_handle_url_part = &CONFIG
        //     .mongo_params
        //     .enable_mongo_cloud_url_parts
        //     .mongo_cloud_second_handle_url_part;
        // let mongo_cloud_password = &CONFIG.mongo_cloud_authorization.mongo_cloud_password;
        // let mongo_cloud_third_handle_url_part = &CONFIG
        //     .mongo_params
        //     .enable_mongo_cloud_url_parts
        //     .mongo_cloud_third_handle_url_part;
        // let mongo_cloud_cluster_name = &CONFIG.mongo_cloud_authorization.mongo_cloud_cluster_name;
        // let mongo_cloud_fourth_handle_url_part = &CONFIG
        //     .mongo_params
        //     .enable_mongo_cloud_url_parts
        //     .mongo_cloud_fourth_handle_url_part;
        // let mongo_cloud_cluster_params =
        //     &CONFIG.mongo_cloud_authorization.mongo_cloud_cluster_params;
        // mongo_url = format!(
        //     "{}{}{}{}{}{}{}{}",
        //     mongo_cloud_first_handle_url_part,
        //     mongo_cloud_login,
        //     mongo_cloud_second_handle_url_part,
        //     mongo_cloud_password,
        //     mongo_cloud_third_handle_url_part,
        //     mongo_cloud_cluster_name,
        //     mongo_cloud_fourth_handle_url_part,
        //     mongo_cloud_cluster_params
        // );
        let mongo_first_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_first_handle_url_part;
        let mongo_login = &CONFIG.mongo_authorization.mongo_login;
        let mongo_second_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_second_handle_url_part;
        let mongo_password = &CONFIG.mongo_authorization.mongo_password;
        let mongo_third_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_third_handle_url_part;
        let mongo_ip = &CONFIG.mongo_authorization.mongo_ip;
        let mongo_fourth_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_fourth_handle_url_part;
        let mongo_port = &CONFIG.mongo_authorization.mongo_port;
        mongo_url = format!(
            "{}{}{}{}{}{}{}{}",
            mongo_first_handle_url_part,
            mongo_login,
            mongo_second_handle_url_part,
            mongo_password,
            mongo_third_handle_url_part,
            mongo_ip,
            mongo_fourth_handle_url_part,
            mongo_port
        );
    } else {
        let mongo_first_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_first_handle_url_part;
        let mongo_login = &CONFIG.mongo_authorization.mongo_login;
        let mongo_second_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_second_handle_url_part;
        let mongo_password = &CONFIG.mongo_authorization.mongo_password;
        let mongo_third_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_third_handle_url_part;
        let mongo_ip = &CONFIG.mongo_authorization.mongo_ip;
        let mongo_fourth_handle_url_part = &CONFIG
            .mongo_params
            .mongo_url_parts
            .mongo_fourth_handle_url_part;
        let mongo_port = &CONFIG.mongo_authorization.mongo_port;
        mongo_url = format!(
            "{}{}{}{}{}{}{}{}",
            mongo_first_handle_url_part,
            mongo_login,
            mongo_second_handle_url_part,
            mongo_password,
            mongo_third_handle_url_part,
            mongo_ip,
            mongo_fourth_handle_url_part,
            mongo_port
        );
    }
    mongo_url
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
