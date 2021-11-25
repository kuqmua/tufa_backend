extern crate toml;

// use itertools::Itertools;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::config_mods::config_structs::github_authorization_struct::GithubAuthorization;
use crate::config_mods::config_structs::enable_providers_struct::EnableProviders;
use crate::config_mods::config_structs::enable_providers_prints_struct::EnableProvidersPrints;
use crate::config_mods::config_structs::providers_check_links_struct::ProvidersCheckLinks;
use crate::config_mods::config_structs::mongo_params_struct::MongoParams;
use crate::config_mods::config_structs::postgres_params_struct::PostgresParams;
use crate::config_mods::config_structs::postgres_url_parts_struct::PostgresUrlParts;
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
use crate::config_mods::config_structs::enable_initialize_mongo_with_providers_link_parts_struct::EnableInitializeMongoWithProvidersLinkParts;
use crate::config_mods::config_structs::mongo_url_parts_struct::MongoUrlParts;
use crate::config_mods::config_structs::mongo_authorization_struct::MongoAuthorization;
use crate::config_mods::config_structs::postgres_authorization_struct::PostgresAuthorization;
use crate::config_mods::config_structs::reddit_authorization_struct::RedditAuthorization;

// use crate::config_mods::config_structs::config_error::ConfigError;
use crate::config_mods::config_error_mods::config_error::ConfigError;

// use crate::constants::project_constants::ARXIV_NAME_TO_CHECK;
// use crate::constants::project_constants::BIORXIV_NAME_TO_CHECK;
// use crate::constants::project_constants::GITHUB_NAME_TO_CHECK;
// use crate::constants::project_constants::HABR_NAME_TO_CHECK;
// use crate::constants::project_constants::MEDRXIV_NAME_TO_CHECK;
// use crate::constants::project_constants::REDDIT_NAME_TO_CHECK;
// use crate::constants::project_constants::TWITTER_NAME_TO_CHECK;

use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;
use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;
use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

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

impl ConfigStruct {
    // fn wrap_config_checks(config_handle: ConfigStruct) -> Result<Self, ConfigError<'static>> {
    //     if !config_handle.github_authorization.github_name.is_empty() {
    //         let error: Result<ConfigStruct, ConfigError> =
    //             Err(ConfigError::Message("github_name is not valid".to_string()));
    //         drop(error);
    //     }
    //     if !config_handle.github_authorization.github_token.is_empty() {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "github_token is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //         .reddit_authorization
    //         .reddit_user_agent
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "reddit_user_agent is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //         .reddit_authorization
    //         .reddit_client_id
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "reddit_client_id is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //         .reddit_authorization
    //         .reddit_client_secret
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "reddit_client_secret is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //         .reddit_authorization
    //         .reddit_username
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "reddit_username is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //         .reddit_authorization
    //         .reddit_password
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "reddit_password is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //         .mongo_params.mongo_authorization
    //         .mongo_login
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "mongo_login is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //     .mongo_params
    //         .mongo_authorization
    //         .mongo_password
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "mongo_password is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //     .mongo_params
    //         .mongo_authorization
    //         .mongo_ip
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "mongo_ip is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //     .mongo_params
    //         .mongo_authorization
    //         .mongo_port
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "mongo_port is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //     .mongo_params
    //         .mongo_authorization
    //         .mongo_params
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "mongo_params is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     //
    //     if config_handle.mongo_params.log_file_extension.is_empty() {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "log_file_extension is not empty".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if config_handle
    //         .mongo_params
    //         .path_to_provider_link_parts_folder
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "path_to_provider_link_parts_folder is empty"
    //                 .to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if config_handle
    //         .mongo_params
    //         .providers_db_collection_document_field_name_handle
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "db_collection_document_field_name_handle is empty"
    //                 .to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if config_handle
    //         .mongo_params
    //         .providers_db_collection_handle_second_part
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "db_collection_handle_second_part is empty"
    //                 .to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if config_handle
    //         .mongo_params
    //         .providers_db_name_handle
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "db_name_handle is empty".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if config_handle
    //         .params
    //         .unhandled_success_handled_success_are_there_items_initialized_posts_dir
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //                 "unhandled_success_handled_success_are_there_items_initialized_posts_dir is empty".to_string(),
    //             ));
    //         drop(error);
    //     }
    //     if config_handle.params.warning_logs_directory_name.is_empty() {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "warning_logs_directory_name is empty".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if config_handle.params.common_providers_links_limit > 0 {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "common_providers_links_limit <= 0".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !ConfigStruct::check_valid_i64_providers_links_limits_for_mongo(&config_handle) {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "providers_links_limits are not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !ConfigStruct::check_valid_vec_of_provider_names(&config_handle) {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "vec_of_provider_names is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     Ok(config_handle)
    // }
}
