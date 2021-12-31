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

    IsPartialSuccessPrintsEnabled,
    IsPartialSuccessPrintsEnabledProviders,
    IsPartialSuccessPrintsEnabledArxiv,
    IsPartialSuccessPrintsEnabledBiorxiv,
    IsPartialSuccessPrintsEnabledGithub,
    IsPartialSuccessPrintsEnabledHabr,
    IsPartialSuccessPrintsEnabledMedrxiv,
    IsPartialSuccessPrintsEnabledReddit,
    IsPartialSuccessPrintsEnabledTwitter,

    IsErrorPrintsEnabled,
    IsErrorPrintsEnabledProviders,
    IsErrorPrintsEnabledArxiv,
    IsErrorPrintsEnabledBiorxiv,
    IsErrorPrintsEnabledGithub,
    IsErrorPrintsEnabledHabr,
    IsErrorPrintsEnabledMedrxiv,
    IsErrorPrintsEnabledReddit,
    IsErrorPrintsEnabledTwitter,

    IsTimeMeasurementPrintsEnabled,
    IsTimeMeasurementPrintsEnabledProviders,
    IsTimeMeasurementPrintsEnabledArxiv,
    IsTimeMeasurementPrintsEnabledBiorxiv,
    IsTimeMeasurementPrintsEnabledGithub,
    IsTimeMeasurementPrintsEnabledHabr,
    IsTimeMeasurementPrintsEnabledMedrxiv,
    IsTimeMeasurementPrintsEnabledReddit,
    IsTimeMeasurementPrintsEnabledTwitter,

    IsInfoPrintsEnabled,
    IsInfoPrintsEnabledProviders,
    IsInfoPrintsEnabledArxiv,
    IsInfoPrintsEnabledBiorxiv,
    IsInfoPrintsEnabledGithub,
    IsInfoPrintsEnabledHabr,
    IsInfoPrintsEnabledMedrxiv,
    IsInfoPrintsEnabledReddit,
    IsInfoPrintsEnabledTwitter,

    IsLinksLimitEnabled,
    IsLinksLimitEnabledProviders,
    IsLinksLimitEnabledArxiv,
    IsLinksLimitEnabledBiorxiv,
    IsLinksLimitEnabledGithub,
    IsLinksLimitEnabledHabr,
    IsLinksLimitEnabledMedrxiv,
    IsLinksLimitEnabledReddit,
    IsLinksLimitEnabledTwitter,

    IsLinksLimitProvidersEnabled,
    CommonProvidersLinksLimit,
    LinksLimitArxiv,
    LinksLimitBiorxiv,
    LinksLimitGithub,
    LinksLimitHabr,
    LinksLimitMedrxiv,
    LinksLimitReddit,
    LinksLimitTwitter,

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
