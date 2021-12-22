use strum_macros::Display;
use strum_macros::EnumIter;

use crate::traits::enum_extention::EnumExtenstion;
use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

use convert_case::{Case, Casing};

use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config_mods;
use crate::config_mods::config_error_mods::config_env_var_error_type_enum::ConfigEnvVarErrorType;
use crate::config_mods::config_error_mods::config_error::ConfigError;

use dotenv::dotenv;

use crate::constants::project_constants::ENV_FILE_NAME;

#[derive(
    EnumExtenstion,
    EnvVarTypedTrait,
    EnumIter,
    Clone,
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
    Display,
)]
pub enum EnvBoolVar {
    DbsEnableInitialization,

    MongoEnableInitialization,
    MongoEnableInitializationForProviders,
    MongoEnableInitializationForArxiv,
    MongoEnableInitializationForBiorxiv,
    MongoEnableInitializationForGithub,
    MongoEnableInitializationForHabr,
    MongoEnableInitializationForMedrxiv,
    MongoEnableInitializationForReddit,
    MongoEnableInitializationForTwitter,

    MongoEnableWriteErrorLogs,
    MongoEnableWriteErrorLogsForProviders,
    MongoEnableWriteErrorLogsForArxiv,
    MongoEnableWriteErrorLogsForBiorxiv,
    MongoEnableWriteErrorLogsForGithub,
    MongoEnableWriteErrorLogsForHabr,
    MongoEnableWriteErrorLogsForMedrxiv,
    MongoEnableWriteErrorLogsForReddit,
    MongoEnableWriteErrorLogsForTwitter,

    MongoEnableCleaningWarningLogsDb,
    MongoEnableCleaningWarningLogsDbForProviders,
    MongoEnableCleaningWarningLogsDbForArxiv,
    MongoEnableCleaningWarningLogsDbForBiorxiv,
    MongoEnableCleaningWarningLogsDbForGithub,
    MongoEnableCleaningWarningLogsDbForHabr,
    MongoEnableCleaningWarningLogsDbForMedrxiv,
    MongoEnableCleaningWarningLogsDbForReddit,
    MongoEnableCleaningWarningLogsDbForTwitter,

    MongoEnableCleaningWarningLogsDbCollections,
    MongoEnableCleaningWarningLogsDbCollectionsForProviders,
    MongoEnableCleaningWarningLogsDbCollectionsForArxiv,
    MongoEnableCleaningWarningLogsDbCollectionsForBiorxiv,
    MongoEnableCleaningWarningLogsDbCollectionsForGithub,
    MongoEnableCleaningWarningLogsDbCollectionsForHabr,
    MongoEnableCleaningWarningLogsDbCollectionsForMedrxiv,
    MongoEnableCleaningWarningLogsDbCollectionsForReddit,
    MongoEnableCleaningWarningLogsDbCollectionsForTwitter,

    MongoEnableLinkPartsRandomizeOrder,
    MongoEnableLinkPartsRandomizeOrderForProviders,
    MongoEnableLinkPartsRandomizeOrderForArxiv,
    MongoEnableLinkPartsRandomizeOrderForBiorxiv,
    MongoEnableLinkPartsRandomizeOrderForGithub,
    MongoEnableLinkPartsRandomizeOrderForHabr,
    MongoEnableLinkPartsRandomizeOrderForMedrxiv,
    MongoEnableLinkPartsRandomizeOrderForReddit,
    MongoEnableLinkPartsRandomizeOrderForTwitter,

    PostgresEnableInitialization,
    PostgresEnableInitializationForProviders,
    PostgresEnableInitializationForArxiv,
    PostgresEnableInitializationForBiorxiv,
    PostgresEnableInitializationForGithub,
    PostgresEnableInitializationForHabr,
    PostgresEnableInitializationForMedrxiv,
    PostgresEnableInitializationForReddit,
    PostgresEnableInitializationForTwitter,

    EnableWriteErrorLogsInLocalFolder,
    EnableWriteErrorLogsInLocalFolderForProviders,
    EnableWriteErrorLogsInLocalFolderForArxiv,
    EnableWriteErrorLogsInLocalFolderForBiorxiv,
    EnableWriteErrorLogsInLocalFolderForGithub,
    EnableWriteErrorLogsInLocalFolderForHabr,
    EnableWriteErrorLogsInLocalFolderForMedrxiv,
    EnableWriteErrorLogsInLocalFolderForReddit,
    EnableWriteErrorLogsInLocalFolderForTwitter,

    EnableCleaningWarningLogsDirectory,
    EnableCleaningWarningLogsDirectoryForProviders,
    EnableCleaningWarningLogsDirectoryForArxiv,
    EnableCleaningWarningLogsDirectoryForBiorxiv,
    EnableCleaningWarningLogsDirectoryForGithub,
    EnableCleaningWarningLogsDirectoryForHabr,
    EnableCleaningWarningLogsDirectoryForMedrxiv,
    EnableCleaningWarningLogsDirectoryForReddit,
    EnableCleaningWarningLogsDirectoryForTwitter,

    EnableProviders,
    EnableArxiv,
    EnableBiorxiv,
    EnableGithub,
    EnableHabr,
    EnableMedrxiv,
    EnableReddit,
    EnableTwitter,

    EnablePrints,
    EnablePrintsForProviders,
    EnablePrintsArxiv,
    EnablePrintsBiorxiv,
    EnablePrintsGithub,
    EnablePrintsHabr,
    EnablePrintsMedrxiv,
    EnablePrintsReddit,
    EnablePrintsTwitter,

    EnableWarningHighPrints,
    EnableWarningHighPrintsForProviders,
    EnableWarningHighPrintsForArxiv,
    EnableWarningHighPrintsForBiorxiv,
    EnableWarningHighPrintsForGithub,
    EnableWarningHighPrintsForHabr,
    EnableWarningHighPrintsForMedrxiv,
    EnableWarningHighPrintsForReddit,
    EnableWarningHighPrintsForTwitter,

    EnableWarningLowPrints,
    EnableWarningLowPrintsForProviders,
    EnableWarningLowPrintsForArxiv,
    EnableWarningLowPrintsForBiorxiv,
    EnableWarningLowPrintsForGithub,
    EnableWarningLowPrintsForHabr,
    EnableWarningLowPrintsForMedrxiv,
    EnableWarningLowPrintsForReddit,
    EnableWarningLowPrintsForTwitter,

    EnableSuccessPrints,
    EnableSuccessPrintsForProviders,
    EnableSuccessPrintsForArxiv,
    EnableSuccessPrintsForBiorxiv,
    EnableSuccessPrintsForGithub,
    EnableSuccessPrintsForHabr,
    EnableSuccessPrintsForMedrxiv,
    EnableSuccessPrintsForReddit,
    EnableSuccessPrintsForTwitter,

    EnablePartialSuccessPrints,
    EnablePartialSuccessPrintsForProviders,
    EnablePartialSuccessPrintsForArxiv,
    EnablePartialSuccessPrintsForBiorxiv,
    EnablePartialSuccessPrintsForGithub,
    EnablePartialSuccessPrintsForHabr,
    EnablePartialSuccessPrintsForMedrxiv,
    EnablePartialSuccessPrintsForReddit,
    EnablePartialSuccessPrintsForTwitter,

    EnableErrorPrints,
    EnableErrorPrintsForProviders,
    EnableErrorPrintsForArxiv,
    EnableErrorPrintsForBiorxiv,
    EnableErrorPrintsForGithub,
    EnableErrorPrintsForHabr,
    EnableErrorPrintsForMedrxiv,
    EnableErrorPrintsForReddit,
    EnableErrorPrintsForTwitter,

    EnableTimeMeasurementPrints,
    EnableTimeMeasurementPrintsForProviders,
    EnableTimeMeasurementPrintsForArxiv,
    EnableTimeMeasurementPrintsForBiorxiv,
    EnableTimeMeasurementPrintsForGithub,
    EnableTimeMeasurementPrintsForHabr,
    EnableTimeMeasurementPrintsForMedrxiv,
    EnableTimeMeasurementPrintsForReddit,
    EnableTimeMeasurementPrintsForTwitter,

    EnableInfoPrints,
    EnableInfoPrintsForProviders,
    EnableInfoPrintsForArxiv,
    EnableInfoPrintsForBiorxiv,
    EnableInfoPrintsForGithub,
    EnableInfoPrintsForHabr,
    EnableInfoPrintsForMedrxiv,
    EnableInfoPrintsForReddit,
    EnableInfoPrintsForTwitter,

    EnableLinksLimit,
    EnableLinksLimitForProviders,
    EnableLinksLimitForArxiv,
    EnableLinksLimitForBiorxiv,
    EnableLinksLimitForGithub,
    EnableLinksLimitForHabr,
    EnableLinksLimitForMedrxiv,
    EnableLinksLimitForReddit,
    EnableLinksLimitForTwitter,

    EnableCommonProvidersLinksLimit,
}
