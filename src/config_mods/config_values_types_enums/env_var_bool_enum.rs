use std::collections::HashMap;

use strum_macros::Display;
use strum_macros::EnumIter;

use strum::IntoEnumIterator;

use convert_case::{Case, Casing};

use dotenv::dotenv;

use crate::traits::enum_extention::EnumExtenstion;
use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

use crate::config_mods::config_error_mods::config_env_var_error_type_enum::ConfigEnvVarErrorType;
use crate::config_mods::config_error_mods::config_error::ConfigError;

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

    IsMongoInitializationEnabled,
    IsMongoInitializationEnabledProviders,
    IsMongoInitializationEnabledArxiv,
    IsMongoInitializationEnabledBiorxiv,
    IsMongoInitializationEnabledGithub,
    IsMongoInitializationEnabledHabr,
    IsMongoInitializationEnabledMedrxiv,
    IsMongoInitializationEnabledReddit,
    IsMongoInitializationEnabledTwitter,
    
    IsMongoWriteErrorLogsEnabled,
    IsMongoWriteErrorLogsEnabledProviders,
    IsMongoWriteErrorLogsEnabledArxiv,
    IsMongoWriteErrorLogsEnabledBiorxiv,
    IsMongoWriteErrorLogsEnabledGithub,
    IsMongoWriteErrorLogsEnabledHabr,
    IsMongoWriteErrorLogsEnabledMedrxiv,
    IsMongoWriteErrorLogsEnabledReddit,
    IsMongoWriteErrorLogsEnabledTwitter,

    IsMongoCleaningWarningLogsDbEnabled,
    IsMongoCleaningWarningLogsDbEnabledProviders,
    IsMongoCleaningWarningLogsDbEnabledArxiv,
    IsMongoCleaningWarningLogsDbEnabledBiorxiv,
    IsMongoCleaningWarningLogsDbEnabledGithub,
    IsMongoCleaningWarningLogsDbEnabledHabr,
    IsMongoCleaningWarningLogsDbEnabledMedrxiv,
    IsMongoCleaningWarningLogsDbEnabledReddit,
    IsMongoCleaningWarningLogsDbEnabledTwitter,

    IsMongoCleaningWarningLogsDbCollectionsEnabled,
    IsMongoCleaningWarningLogsDbCollectionsEnabledProviders,
    IsMongoCleaningWarningLogsDbCollectionsEnabledArxiv,
    IsMongoCleaningWarningLogsDbCollectionsEnabledBiorxiv,
    IsMongoCleaningWarningLogsDbCollectionsEnabledGithub,
    IsMongoCleaningWarningLogsDbCollectionsEnabledHabr,
    IsMongoCleaningWarningLogsDbCollectionsEnabledMedrxiv,
    IsMongoCleaningWarningLogsDbCollectionsEnabledReddit,
    IsMongoCleaningWarningLogsDbCollectionsEnabledTwitter,

    IsMongoLinkPartsRandomizeOrderEnabled,
    IsMongoLinkPartsRandomizeOrderEnabledProviders,
    IsMongoLinkPartsRandomizeOrderEnabledArxiv,
    IsMongoLinkPartsRandomizeOrderEnabledBiorxiv,
    IsMongoLinkPartsRandomizeOrderEnabledGithub,
    IsMongoLinkPartsRandomizeOrderEnabledHabr,
    IsMongoLinkPartsRandomizeOrderEnabledMedrxiv,
    IsMongoLinkPartsRandomizeOrderEnabledReddit,
    IsMongoLinkPartsRandomizeOrderEnabledTwitter,

    IsPostgresInitializationEnabled,
    IsPostgresInitializationEnabledProviders,
    IsPostgresInitializationEnabledArxiv,
    IsPostgresInitializationEnabledBiorxiv,
    IsPostgresInitializationEnabledGithub,
    IsPostgresInitializationEnabledHabr,
    IsPostgresInitializationEnabledMedrxiv,
    IsPostgresInitializationEnabledReddit,
    IsPostgresInitializationEnabledTwitter,

    IsWriteErrorLogsInLocalFolderEnabled,
    IsWriteErrorLogsInLocalFolderEnabledProviders,
    IsWriteErrorLogsInLocalFolderEnabledArxiv,
    IsWriteErrorLogsInLocalFolderEnabledBiorxiv,
    IsWriteErrorLogsInLocalFolderEnabledGithub,
    IsWriteErrorLogsInLocalFolderEnabledHabr,
    IsWriteErrorLogsInLocalFolderEnabledMedrxiv,
    IsWriteErrorLogsInLocalFolderEnabledReddit,
    IsWriteErrorLogsInLocalFolderEnabledTwitter,

    IsCleaningWarningLogsDirectoryEnabled,
    IsCleaningWarningLogsDirectoryEnabledProviders,
    IsCleaningWarningLogsDirectoryEnabledArxiv,
    IsCleaningWarningLogsDirectoryEnabledBiorxiv,
    IsCleaningWarningLogsDirectoryEnabledGithub,
    IsCleaningWarningLogsDirectoryEnabledHabr,
    IsCleaningWarningLogsDirectoryEnabledMedrxiv,
    IsCleaningWarningLogsDirectoryEnabledReddit,
    IsCleaningWarningLogsDirectoryEnabledTwitter,

    IsEnabledProviders,
    IsEnabledArxiv,
    IsEnabledBiorxiv,
    IsEnabledGithub,
    IsEnabledHabr,
    IsEnabledMedrxiv,
    IsEnabledReddit,
    IsEnabledTwitter,

    IsPrintsEnabled,
    IsPrintsEnabledProviders,
    IsPrintsEnabledArxiv,
    IsPrintsEnabledBiorxiv,
    IsPrintsEnabledGithub,
    IsPrintsEnabledHabr,
    IsPrintsEnabledMedrxiv,
    IsPrintsEnabledReddit,
    IsPrintsEnabledTwitter,
    
    IsWarningHighPrintsEnabled,
    IsWarningHighPrintsEnabledProviders,
    IsWarningHighPrintsEnabledArxiv,
    IsWarningHighPrintsEnabledBiorxiv,
    IsWarningHighPrintsEnabledGithub,
    IsWarningHighPrintsEnabledHabr,
    IsWarningHighPrintsEnabledMedrxiv,
    IsWarningHighPrintsEnabledReddit,
    IsWarningHighPrintsEnabledTwitter,

    IsWarningLowPrintsEnabled,
    IsWarningLowPrintsEnabledProviders,
    IsWarningLowPrintsEnabledArxiv,
    IsWarningLowPrintsEnabledBiorxiv,
    IsWarningLowPrintsEnabledGithub,
    IsWarningLowPrintsEnabledHabr,
    IsWarningLowPrintsEnabledMedrxiv,
    IsWarningLowPrintsEnabledReddit,
    IsWarningLowPrintsEnabledTwitter,

    IsSuccessPrintsEnabled,
    IsSuccessPrintsEnabledProviders,
    IsSuccessPrintsEnabledArxiv,
    IsSuccessPrintsEnabledBiorxiv,
    IsSuccessPrintsEnabledGithub,
    IsSuccessPrintsEnabledHabr,
    IsSuccessPrintsEnabledMedrxiv,
    IsSuccessPrintsEnabledReddit,
    IsSuccessPrintsEnabledTwitter,

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
