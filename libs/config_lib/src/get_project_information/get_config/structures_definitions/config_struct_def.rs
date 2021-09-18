use crate::get_project_information::get_config::structures_definitions::enable_providers_def::EnableProviders;
use crate::get_project_information::get_config::structures_definitions::enable_providers_prints_def::EnableProvidersPrints;
use crate::get_project_information::get_config::structures_definitions::providers_check_links_def::ProvidersCheckLinks;
use crate::get_project_information::get_config::structures_definitions::mongo_params_def::MongoParams;
use crate::get_project_information::get_config::structures_definitions::postgres_params_def::PostgresParams;
use crate::get_project_information::get_config::structures_definitions::enable_error_providers_prints_def::EnableErrorProvidersPrints;
use crate::get_project_information::get_config::structures_definitions::enable_partial_success_providers_prints_def::EnablePartialSuccessProvidersPrints;
use crate::get_project_information::get_config::structures_definitions::enable_providers_cleaning_warning_logs_directory_def::EnableProvidersCleaningWarningLogsDirectory;
use crate::get_project_information::get_config::structures_definitions::enable_providers_links_limit_def::EnableProvidersLinksLimit;
use crate::get_project_information::get_config::structures_definitions::enable_providers_time_measurement_def::EnableProvidersTimeMeasurement;
use crate::get_project_information::get_config::structures_definitions::enable_randomize_order_for_providers_link_parts_for_mongo_def::EnableRandomizeOrderForProvidersLinkPartsForMongo;
use crate::get_project_information::get_config::structures_definitions::enable_success_providers_prints_def::EnableSuccessProvidersPrints;
use crate::get_project_information::get_config::structures_definitions::enable_warning_high_providers_prints_def::EnableWarningHighProvidersPrints;
use crate::get_project_information::get_config::structures_definitions::enable_warning_low_providers_prints_def::EnableWarningLowProvidersPrints;
use crate::get_project_information::get_config::structures_definitions::params_def::Params;
use crate::get_project_information::get_config::structures_definitions::print_colors_def::PrintColors;
use crate::get_project_information::get_config::structures_definitions::providers_links_limits_def::ProvidersLinksLimits;
use crate::get_project_information::get_config::structures_definitions::enable_providers_cleaning_warning_logs_db_in_mongo::EnableProvidersCleaningWarningLogsDbInMongo;
use crate::get_project_information::get_config::structures_definitions::enable_providers_cleaning_warning_logs_db_collections_in_mongo::EnableProvidersCleaningWarningLogsDbCollectionsInMongo;

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
    pub enable_providers_links_limits: EnableProvidersLinksLimit,
    pub providers_links_limits: ProvidersLinksLimits,
    pub enable_randomize_order_for_providers_link_parts_for_mongo:
        EnableRandomizeOrderForProvidersLinkPartsForMongo,
    pub print_colors: PrintColors,
}
