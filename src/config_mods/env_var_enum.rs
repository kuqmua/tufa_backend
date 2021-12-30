use crate::traits::enum_extention::EnumExtenstion;

use strum_macros::Display;
use strum_macros::EnumIter;

use convert_case::{Case, Casing};

use std::collections::HashMap;

use strum::IntoEnumIterator;

#[derive(
    EnumExtenstion,
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
pub enum EnvVar {
    GithubName,
    GithubToken,

    RedditUserAgent,
    RedditClientId,
    RedditClientSecret,
    RedditUsername,
    RedditPassword,

    DbsEnableInitialization,
    ProvidersLinkPartsSource,

    MongoFirstHandleUrlPart,
    MongoSecondHandleUrlPart,
    MongoThirdHandleUrlPart,
    MongoFourthHandleUrlPart,
    MongoFifthHandleUrlPart,

    MongoLogin,
    MongoPassword,
    MongoIp,
    MongoPort,
    MongoParams,

    MongoProvidersLogsDbName,
    MongoProvidersLogsDbCollectionHandleSecondPart,
    MongoProvidersLogsDbCollectionDocumentFieldNameHandle,

    IsMongoInitializationEnabled,
    IsMongoInitializationEnabledForProviders,
    IsMongoInitializationEnabledForArxiv,
    IsMongoInitializationEnabledForBiorxiv,
    IsMongoInitializationEnabledForGithub,
    IsMongoInitializationEnabledForHabr,
    IsMongoInitializationEnabledForMedrxiv,
    IsMongoInitializationEnabledForReddit,
    IsMongoInitializationEnabledForTwitter,

    IsMongoWriteErrorLogsEnabled,
    IsMongoWriteErrorLogsEnabledForProviders,
    IsMongoWriteErrorLogsEnabledForArxiv,
    IsMongoWriteErrorLogsEnabledForBiorxiv,
    IsMongoWriteErrorLogsEnabledForGithub,
    IsMongoWriteErrorLogsEnabledForHabr,
    IsMongoWriteErrorLogsEnabledForMedrxiv,
    IsMongoWriteErrorLogsEnabledForReddit,
    IsMongoWriteErrorLogsEnabledForTwitter,

    IsMongoCleaningWarningLogsDbEnabled,
    IsMongoCleaningWarningLogsDbEnabledForProviders,
    IsMongoCleaningWarningLogsDbEnabledForArxiv,
    IsMongoCleaningWarningLogsDbEnabledForBiorxiv,
    IsMongoCleaningWarningLogsDbEnabledForGithub,
    IsMongoCleaningWarningLogsDbEnabledForHabr,
    IsMongoCleaningWarningLogsDbEnabledForMedrxiv,
    IsMongoCleaningWarningLogsDbEnabledForReddit,
    IsMongoCleaningWarningLogsDbEnabledForTwitter,

    IsMongoCleaningWarningLogsDbCollectionsEnabled,
    IsMongoCleaningWarningLogsDbCollectionsEnabledForProviders,
    IsMongoCleaningWarningLogsDbCollectionsEnabledForArxiv,
    IsMongoCleaningWarningLogsDbCollectionsEnabledForBiorxiv,
    IsMongoCleaningWarningLogsDbCollectionsEnabledForGithub,
    IsMongoCleaningWarningLogsDbCollectionsEnabledForHabr,
    IsMongoCleaningWarningLogsDbCollectionsEnabledForMedrxiv,
    IsMongoCleaningWarningLogsDbCollectionsEnabledForReddit,
    IsMongoCleaningWarningLogsDbCollectionsEnabledForTwitter,

    IsMongoLinkPartsRandomizeOrderEnabled,
    IsMongoLinkPartsRandomizeOrderEnabledForProviders,
    IsMongoLinkPartsRandomizeOrderEnabledForArxiv,
    IsMongoLinkPartsRandomizeOrderEnabledForBiorxiv,
    IsMongoLinkPartsRandomizeOrderEnabledForGithub,
    IsMongoLinkPartsRandomizeOrderEnabledForHabr,
    IsMongoLinkPartsRandomizeOrderEnabledForMedrxiv,
    IsMongoLinkPartsRandomizeOrderEnabledForReddit,
    IsMongoLinkPartsRandomizeOrderEnabledForTwitter,

    PostgresFirstHandleUrlPart,
    PostgresSecondHandleUrlPart,
    PostgresThirdHandleUrlPart,
    PostgresFourthHandleUrlPart,
    PostgresFifthHandleUrlPart,

    PostgresLogin,
    PostgresPassword,
    PostgresIp,
    PostgresPort,
    PostgresDb,

    IsPostgresInitializationEnabled,
    IsPostgresInitializationEnabledForProviders,
    IsPostgresInitializationEnabledForArxiv,
    IsPostgresInitializationEnabledForBiorxiv,
    IsPostgresInitializationEnabledForGithub,
    IsPostgresInitializationEnabledForHabr,
    IsPostgresInitializationEnabledForMedrxiv,
    IsPostgresInitializationEnabledForReddit,
    IsPostgresInitializationEnabledForTwitter,

    WarningLogsDirectoryName,
    UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir,
    PathToProviderLinkPartsFolder,
    LogFileExtension,

    IsWriteErrorLogsInLocalFolderEnabled,
    IsWriteErrorLogsInLocalFolderEnabledForProviders,
    IsWriteErrorLogsInLocalFolderEnabledForArxiv,
    IsWriteErrorLogsInLocalFolderEnabledForBiorxiv,
    IsWriteErrorLogsInLocalFolderEnabledForGithub,
    IsWriteErrorLogsInLocalFolderEnabledForHabr,
    IsWriteErrorLogsInLocalFolderEnabledForMedrxiv,
    IsWriteErrorLogsInLocalFolderEnabledForReddit,
    IsWriteErrorLogsInLocalFolderEnabledForTwitter,

    EnableCleaningWarningLogsDirectory,
    EnableCleaningWarningLogsDirectoryForProviders,
    EnableCleaningWarningLogsDirectoryForArxiv,
    EnableCleaningWarningLogsDirectoryForBiorxiv,
    EnableCleaningWarningLogsDirectoryForGithub,
    EnableCleaningWarningLogsDirectoryForHabr,
    EnableCleaningWarningLogsDirectoryForMedrxiv,
    EnableCleaningWarningLogsDirectoryForReddit,
    EnableCleaningWarningLogsDirectoryForTwitter,

    StartingCheckLink,
    ArxivCheckLink,
    BiorxivCheckLink,
    GithubCheckLink,
    HabrCheckLink,
    MedrxivCheckLink,
    RedditCheckLink,
    TwitterCheckLink,

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
    CommonProvidersLinksLimit,
    LinksLimitForArxiv,
    LinksLimitForBiorxiv,
    LinksLimitForGithub,
    LinksLimitForHabr,
    LinksLimitForMedrxiv,
    LinksLimitForReddit,
    LinksLimitForTwitter,

    ErrorRed,
    ErrorGreen,
    ErrorBlue,
    WarningHighRed,
    WarningHighGreen,
    WarningHighBlue,
    WarningLowRed,
    WarningLowGreen,
    WarningLowBlue,
    SuccessRed,
    SuccessGreen,
    SuccessBlue,
    PartialSuccessRed,
    PartialSuccessGreen,
    PartialSuccessBlue,
    CleaningRed,
    CleaningGreen,
    CleaningBlue,
    TimeMeasurementRed,
    TimeMeasurementGreen,
    TimeMeasurementBlue,
    InfoRed,
    InfoGreen,
    InfoBlue,
}
