extern crate toml;

use crate::config_mods::config_structs::enable_providers_struct::EnableProviders;
use crate::config_mods::config_structs::enable_providers_prints_struct::EnableProvidersPrints;
use crate::config_mods::config_structs::providers_check_links_struct::ProvidersCheckLinks;
use crate::config_mods::config_structs::mongo_params_struct::MongoParams;
use crate::config_mods::config_structs::postgres_params_struct::PostgresParams;
use crate::config_mods::config_structs::enable_error_providers_prints_struct::EnableErrorProvidersPrints;
use crate::config_mods::config_structs::enable_partial_success_providers_prints_struct::EnablePartialSuccessProvidersPrints;
use crate::config_mods::config_structs::enable_providers_cleaning_warning_logs_directory_struct::EnableProvidersCleaningWarningLogsDirectory;
use crate::config_mods::config_structs::enable_providers_links_limit_struct::EnableProvidersLinksLimit;
use crate::config_mods::config_structs::enable_providers_time_measurement_struct::EnableProvidersTimeMeasurement;
use crate::config_mods::config_structs::enable_providers_info_struct::EnableProvidersInfo;
use crate::config_mods::config_structs::enable_success_providers_prints_struct::EnableSuccessProvidersPrints;
use crate::config_mods::config_structs::enable_warning_high_providers_prints_struct::EnableWarningHighProvidersPrints;
use crate::config_mods::config_structs::enable_warning_low_providers_prints_struct::EnableWarningLowProvidersPrints;
use crate::config_mods::config_structs::providers_links_limits_struct::ProvidersLinksLimits;
use crate::config_mods::config_structs::enable_providers_cleaning_warning_logs_db_in_mongo_struct::EnableProvidersCleaningWarningLogsDbInMongo;
use crate::config_mods::config_structs::enable_providers_cleaning_warning_logs_db_collections_in_mongo_struct::EnableProvidersCleaningWarningLogsDbCollectionsInMongo;

#[derive(Debug, Clone, PartialEq)] //Default,//serde_derive::Serialize, serde_derive::Deserialize
pub struct ConfigStruct {
    pub github_name: String,
    pub github_token: String,

    pub reddit_user_agent: String,
    pub reddit_client_id: String,
    pub reddit_client_secret: String,
    pub reddit_username: String,
    pub reddit_password: String,

    pub starting_check_link: String,
    pub warning_logs_directory_name: String,
    pub unhandled_success_handled_success_are_there_items_initialized_posts_dir: String,
    pub enable_cleaning_warning_logs_db_in_mongo: bool,
    pub enable_cleaning_warning_logs_db_collections_in_mongo: bool,
    pub enable_time_measurement: bool,
    pub enable_provider_links_limit: bool,
    pub enable_common_providers_links_limit: bool,
    pub common_providers_links_limit: i64,
    pub enable_randomize_order_for_providers_link_parts_for_mongo: bool,
    //
    pub enable_prints: bool,
    pub enable_error_prints: bool,
    pub enable_warning_high_prints: bool,
    pub enable_warning_low_prints: bool,
    pub enable_success_prints: bool,
    pub enable_partial_success_prints: bool,
    pub enable_time_measurement_prints: bool,
    pub enable_cleaning_warning_logs_directory_prints: bool,
    pub enable_info_prints: bool,
    //
    pub enable_write_error_logs_in_local_folder: bool,
    pub enable_write_error_logs_in_mongo: bool,
    pub enable_initialize_mongo_with_providers_link_parts: bool,

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

    pub enable_randomize_order_for_arxiv_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_biorxiv_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_github_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_habr_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_medrxiv_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_reddit_link_parts_for_mongo: bool,
    pub enable_randomize_order_for_twitter_link_parts_for_mongo: bool,

    pub error_red: u8,
    pub error_green: u8,
    pub error_blue: u8,
    pub warning_high_red: u8,
    pub warning_high_green: u8,
    pub warning_high_blue: u8,
    pub warning_low_red: u8,
    pub warning_low_green: u8,
    pub warning_low_blue: u8,
    pub success_red: u8,
    pub success_green: u8,
    pub success_blue: u8,
    pub partial_success_red: u8,
    pub partial_success_green: u8,
    pub partial_success_blue: u8,
    pub cleaning_red: u8,
    pub cleaning_green: u8,
    pub cleaning_blue: u8,
    pub time_measurement_red: u8,
    pub time_measurement_green: u8,
    pub time_measurement_blue: u8,
    pub info_red: u8,
    pub info_green: u8,
    pub info_blue: u8,
}
