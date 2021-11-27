extern crate toml;

use crate::config_mods::config_structs::github_authorization_struct::GithubAuthorization;
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
use crate::config_mods::config_structs::enable_randomize_order_for_providers_link_parts_for_mongo_struct::EnableRandomizeOrderForProvidersLinkPartsForMongo;
use crate::config_mods::config_structs::enable_success_providers_prints_struct::EnableSuccessProvidersPrints;
use crate::config_mods::config_structs::enable_warning_high_providers_prints_struct::EnableWarningHighProvidersPrints;
use crate::config_mods::config_structs::enable_warning_low_providers_prints_struct::EnableWarningLowProvidersPrints;
use crate::config_mods::config_structs::params_struct::Params;
use crate::config_mods::config_structs::print_colors_struct::PrintColors;
use crate::config_mods::config_structs::providers_links_limits_struct::ProvidersLinksLimits;
use crate::config_mods::config_structs::enable_providers_cleaning_warning_logs_db_in_mongo_struct::EnableProvidersCleaningWarningLogsDbInMongo;
use crate::config_mods::config_structs::enable_providers_cleaning_warning_logs_db_collections_in_mongo_struct::EnableProvidersCleaningWarningLogsDbCollectionsInMongo;
use crate::config_mods::config_structs::reddit_authorization_struct::RedditAuthorization;

#[derive(Debug, Clone, PartialEq)] //Default,//serde_derive::Serialize, serde_derive::Deserialize
pub struct ConfigStruct {
    pub github_authorization: GithubAuthorization,
    pub reddit_authorization: RedditAuthorization,
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
