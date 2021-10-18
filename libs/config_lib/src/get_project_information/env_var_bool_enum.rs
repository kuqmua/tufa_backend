use std::collections::HashMap;

use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;

use strum_macros::EnumIter;

use dotenv::dotenv;

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
use crate::get_project_information::env_var_names_constants::ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME;

// [mongo_params.enable_initialize_mongo_with_providers_link_parts]
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME;
use crate::get_project_information::env_var_names_constants::ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME;

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

use crate::get_project_information::project_constants::ENV_FILE_NAME;

use std::env::VarError;

#[derive(Debug)]
pub enum ConfigErrorInnerType {
    VarErrorHandle(VarError),
    VarOrBoolParseErrorHandle(VarOrBoolParseError),
    VarOrIntParseErrorErrorHandle(VarOrIntParseError)
}

use core::str::ParseBoolError;
use core::num::ParseIntError;

#[derive(Debug)] 
pub enum VarOrBoolParseError {
    Var(VarError),
    Bool(ParseBoolError)
}

#[derive(Debug)] 
pub enum VarOrIntParseError {
    Var(VarError),
    Int(ParseIntError)
}

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
pub enum EnvBoolVar {
    EnableProviders,
    EnableCleaningWarningLogsDirectory,
    EnableCleaningWarningLogsDbInMongo,
    EnableCleaningWarningLogsDbCollectionsInMongo,
    EnableTimeMeasurement,
    EnableProviderLinksLimit,
    EnableCommonProvidersLinksLimit,
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
    EnableInitializeMongoWithArxivLinkParts,
    EnableInitializeMongoWithBiorxivLinkParts,
    EnableInitializeMongoWithGithubLinkParts,
    EnableInitializeMongoWithHabrLinkParts,
    EnableInitializeMongoWithMedrxivLinkParts,
    EnableInitializeMongoWithRedditLinkParts,
    EnableInitializeMongoWithTwitterLinkParts,
    EnableArxiv,
    EnableBiorxiv,
    EnableGithub,
    EnableHabr,
    EnableMedrxiv,
    EnableReddit,
    EnableTwitter,
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
}
#[derive(Debug)] 
pub struct ConfigTestError<'a> {
    env_var_name_kind: EnvBoolVar,
    was_dotenv_enable: bool,
    env_name: &'a str, 
    env_error: ConfigErrorInnerType
} 

impl EnvBoolVar {
    pub fn get_env_name(env_var_name_kind: EnvBoolVar) -> &'static str {
        match env_var_name_kind {
            EnvBoolVar::EnableProviders => ENABLE_PROVIDERS_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDirectory => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbInMongo => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongo => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME,
            EnvBoolVar::EnableTimeMeasurement => ENABLE_TIME_MEASUREMENT_ENV_NAME,
            EnvBoolVar::EnableProviderLinksLimit => ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME,
            EnvBoolVar::EnableCommonProvidersLinksLimit => ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME,
            EnvBoolVar::EnableRandomizeOrderForProvidersLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvBoolVar::EnablePrints => ENABLE_PRINTS_ENV_NAME,
            EnvBoolVar::EnableErrorPrints => ENABLE_ERROR_PRINTS_ENV_NAME,
            EnvBoolVar::EnableWarningHighPrints => ENABLE_WARNING_HIGH_PRINTS_ENV_NAME,
            EnvBoolVar::EnableWarningLowPrints => ENABLE_WARNING_LOW_PRINTS_ENV_NAME,
            EnvBoolVar::EnableSuccessPrints => ENABLE_SUCCESS_PRINTS_ENV_NAME,
            EnvBoolVar::EnablePartialSuccessPrints => ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME,
            EnvBoolVar::EnableTimeMeasurementPrints => ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDirectoryPrints => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME,
            EnvBoolVar::EnableInfoPrints => ENABLE_INFO_PRINTS_ENV_NAME,
            EnvBoolVar::EnableAllProvidersPrints => ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME,
            EnvBoolVar::EnableErrorPrintsForAllProviders => ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvBoolVar::EnableWarningHighPrintsForAllProviders => ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvBoolVar::EnableWarningLowPrintsForAllProviders => ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvBoolVar::EnableSuccessPrintsForAllProviders => ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvBoolVar::EnablePartialSuccessPrintsForAllProviders => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvBoolVar::EnableTimeMeasurementPrintsForAllProviders => ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDirectoryPrintsForAllProviders => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvBoolVar::EnableInfoPrintsForAllProviders => ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME,
            EnvBoolVar::EnableWriteErrorLogsInLocalFolder => ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME,
            EnvBoolVar::EnableWriteErrorLogsInMongo => ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME,
            EnvBoolVar::EnableInitializeMongoWithProvidersLinkParts => ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME,

            EnvBoolVar::EnableInitializeMongoWithArxivLinkParts => ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME,
            EnvBoolVar::EnableInitializeMongoWithBiorxivLinkParts => ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME,
            EnvBoolVar::EnableInitializeMongoWithGithubLinkParts => ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME,
            EnvBoolVar::EnableInitializeMongoWithHabrLinkParts => ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME,
            EnvBoolVar::EnableInitializeMongoWithMedrxivLinkParts => ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME,
            EnvBoolVar::EnableInitializeMongoWithRedditLinkParts => ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME,
            EnvBoolVar::EnableInitializeMongoWithTwitterLinkParts => ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME,

            EnvBoolVar::EnableArxiv => ENABLE_ARXIV_ENV_NAME,
            EnvBoolVar::EnableBiorxiv => ENABLE_BIORXIV_ENV_NAME,
            EnvBoolVar::EnableGithub => ENABLE_GITHUB_ENV_NAME,
            EnvBoolVar::EnableHabr => ENABLE_HABR_ENV_NAME,
            EnvBoolVar::EnableMedrxiv => ENABLE_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnableReddit => ENABLE_REDDIT_ENV_NAME,
            EnvBoolVar::EnableTwitter => ENABLE_TWITTER_ENV_NAME,

            EnvBoolVar::EnablePrintsArxiv => ENABLE_PRINTS_ARXIV_ENV_NAME,
            EnvBoolVar::EnablePrintsBiorxiv => ENABLE_PRINTS_BIORXIV_ENV_NAME,
            EnvBoolVar::EnablePrintsGithub => ENABLE_PRINTS_GITHUB_ENV_NAME,
            EnvBoolVar::EnablePrintsHabr => ENABLE_PRINTS_HABR_ENV_NAME,
            EnvBoolVar::EnablePrintsMedrxiv => ENABLE_PRINTS_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnablePrintsReddit => ENABLE_PRINTS_REDDIT_ENV_NAME,
            EnvBoolVar::EnablePrintsTwitter => ENABLE_PRINTS_TWITTER_ENV_NAME,

            EnvBoolVar::EnableWarningHighPrintsForArxiv => ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvBoolVar::EnableWarningHighPrintsForBiorxiv => ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvBoolVar::EnableWarningHighPrintsForGithub => ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvBoolVar::EnableWarningHighPrintsForHabr => ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME,
            EnvBoolVar::EnableWarningHighPrintsForMedrxiv => ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnableWarningHighPrintsForReddit => ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvBoolVar::EnableWarningHighPrintsForTwitter => ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvBoolVar::EnableWarningLowPrintsForArxiv => ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvBoolVar::EnableWarningLowPrintsForBiorxiv => ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvBoolVar::EnableWarningLowPrintsForGithub => ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvBoolVar::EnableWarningLowPrintsForHabr => ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME,
            EnvBoolVar::EnableWarningLowPrintsForMedrxiv => ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnableWarningLowPrintsForReddit => ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvBoolVar::EnableWarningLowPrintsForTwitter => ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvBoolVar::EnableErrorPrintsForArxiv => ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvBoolVar::EnableErrorPrintsForBiorxiv => ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvBoolVar::EnableErrorPrintsForGithub => ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvBoolVar::EnableErrorPrintsForHabr => ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME,
            EnvBoolVar::EnableErrorPrintsForMedrxiv => ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnableErrorPrintsForReddit => ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvBoolVar::EnableErrorPrintsForTwitter => ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvBoolVar::EnableSuccessPrintsForArxiv => ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvBoolVar::EnableSuccessPrintsForBiorxiv => ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvBoolVar::EnableSuccessPrintsForGithub => ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvBoolVar::EnableSuccessPrintsForHabr => ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME,
            EnvBoolVar::EnableSuccessPrintsForMedrxiv => ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnableSuccessPrintsForReddit => ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvBoolVar::EnableSuccessPrintsForTwitter => ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvBoolVar::EnablePartialSuccessPrintsForArxiv => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME,
            EnvBoolVar::EnablePartialSuccessPrintsForBiorxiv => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME,
            EnvBoolVar::EnablePartialSuccessPrintsForGithub => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME,
            EnvBoolVar::EnablePartialSuccessPrintsForHabr => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME,
            EnvBoolVar::EnablePartialSuccessPrintsForMedrxiv => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnablePartialSuccessPrintsForReddit => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME,
            EnvBoolVar::EnablePartialSuccessPrintsForTwitter => ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME,

            EnvBoolVar::EnableCleaningWarningLogsDirectoryForArxiv => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForBiorxiv => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForGithub => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForHabr => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForMedrxiv => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForReddit => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForTwitter => ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME,

            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForArxiv => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForBiorxiv => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForGithub => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForHabr => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForMedrxiv => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForReddit => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForTwitter => ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME,

            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForGithub => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForHabr => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForReddit => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME,
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter => ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME,

            EnvBoolVar::EnableTimeMeasurementForArxiv => ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME,
            EnvBoolVar::EnableTimeMeasurementForBiorxiv => ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME,
            EnvBoolVar::EnableTimeMeasurementForGithub => ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME,
            EnvBoolVar::EnableTimeMeasurementForHabr => ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME,
            EnvBoolVar::EnableTimeMeasurementForMedrxiv => ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnableTimeMeasurementForReddit => ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME,
            EnvBoolVar::EnableTimeMeasurementForTwitter => ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME,

            EnvBoolVar::EnableInfoForArxiv => ENABLE_INFO_FOR_ARXIV_ENV_NAME,
            EnvBoolVar::EnableInfoForBiorxiv => ENABLE_INFO_FOR_BIORXIV_ENV_NAME,
            EnvBoolVar::EnableInfoForGithub => ENABLE_INFO_FOR_GITHUB_ENV_NAME,
            EnvBoolVar::EnableInfoForHabr => ENABLE_INFO_FOR_HABR_ENV_NAME,
            EnvBoolVar::EnableInfoForMedrxiv => ENABLE_INFO_FOR_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnableInfoForReddit => ENABLE_INFO_FOR_REDDIT_ENV_NAME,
            EnvBoolVar::EnableInfoForTwitter => ENABLE_INFO_FOR_TWITTER_ENV_NAME,

            EnvBoolVar::EnableLinksLimitForArxiv => ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME,
            EnvBoolVar::EnableLinksLimitForBiorxiv => ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME,
            EnvBoolVar::EnableLinksLimitForGithub => ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME,
            EnvBoolVar::EnableLinksLimitForHabr => ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME,
            EnvBoolVar::EnableLinksLimitForMedrxiv => ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME,
            EnvBoolVar::EnableLinksLimitForReddit => ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME,
            EnvBoolVar::EnableLinksLimitForTwitter => ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME,

            EnvBoolVar::EnableRandomizeOrderForArxivLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvBoolVar::EnableRandomizeOrderForBiorxivLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvBoolVar::EnableRandomizeOrderForGithubLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvBoolVar::EnableRandomizeOrderForHabrLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvBoolVar::EnableRandomizeOrderForMedrxivLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvBoolVar::EnableRandomizeOrderForRedditLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME,
            EnvBoolVar::EnableRandomizeOrderForTwitterLinkPartsForMongo => ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME,
        }
    }
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    pub fn into_vec() -> Vec<EnvBoolVar> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvBoolVar::get_length());
        for env_var_name_kind in EnvBoolVar::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvBoolVar)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvBoolVar::get_length());
        for env_var_name_kind in EnvBoolVar::iter() {
            env_var_name_kind_vec.push((EnvBoolVar::get_env_name(env_var_name_kind),   env_var_name_kind));
        }
        env_var_name_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvBoolVar> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<&'static str, EnvBoolVar> =
        HashMap::with_capacity(EnvBoolVar::get_length());
        for env_var_name_kind_kind in EnvBoolVar::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(EnvBoolVar::get_env_name(env_var_name_kind_kind),   env_var_name_kind_kind);
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
    pub fn get_string_from_env_var(env_var_name_kind: EnvBoolVar, was_dotenv_enable: bool) -> Result<String, ConfigTestError<'static>>{
        let string_name = EnvBoolVar::get_env_name(env_var_name_kind);
        match std::env::var(string_name) {
            Ok(handle) => {
                Ok(handle)
            }
            Err(e) => {
                return Err(ConfigTestError {env_var_name_kind,  was_dotenv_enable, env_name: string_name, env_error: ConfigErrorInnerType::VarErrorHandle(e) })
            }   
        }
    }
    pub fn get_env_values_hashmap() -> Result<HashMap::<EnvBoolVar, bool>, ConfigTestError<'static>> {
        let was_dotenv_enable: bool;
        match dotenv() {
            Ok(_) => {
                was_dotenv_enable = true;
            },
            Err(e) => {
                was_dotenv_enable = false;
                println!("dotenv() failed, trying without {} error: {:?}", ENV_FILE_NAME, e);
            }
        }
        let mut hmap: HashMap::<EnvBoolVar,bool> = HashMap::new();
        let mut error_option: Option<ConfigTestError> = None;
        for env_var_name_kind in EnvBoolVar::iter() {
            match EnvBoolVar::get_string_from_env_var(env_var_name_kind, was_dotenv_enable) {
                Ok(env_var_string) => {
                    match env_var_string.parse::<bool>() {
                        Ok(handle) => {
                            hmap.insert(env_var_name_kind, handle);
                        },
                        Err(e) => {
                            error_option = Some(ConfigTestError {env_var_name_kind,  was_dotenv_enable, env_name: EnvBoolVar::get_env_name(env_var_name_kind), env_error: ConfigErrorInnerType::VarOrBoolParseErrorHandle(VarOrBoolParseError::Bool(e)) });
                            break;
                        }
                    }
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
        Ok(hmap)
    }
}

