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
    IsPostgresInitializationEnabledProviders,
    IsPostgresInitializationEnabledArxiv,
    IsPostgresInitializationEnabledBiorxiv,
    IsPostgresInitializationEnabledGithub,
    IsPostgresInitializationEnabledHabr,
    IsPostgresInitializationEnabledMedrxiv,
    IsPostgresInitializationEnabledReddit,
    IsPostgresInitializationEnabledTwitter,

    WarningLogsDirectoryName,
    UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir,
    PathToProviderLinkPartsFolder,
    LogFileExtension,

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

    StartingCheckLink,
    CheckLinkArxiv,
    CheckLinkBiorxiv,
    CheckLinkGithub,
    CheckLinkHabr,
    CheckLinkMedrxiv,
    CheckLinkReddit,
    CheckLinkTwitter,

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
