use core::num::ParseIntError;
use core::str::ParseBoolError;
use std::env::VarError;

#[derive(Debug)]
pub enum VarOrBoolParseError {
    Var(VarError),
    Bool(ParseBoolError),
}

#[derive(Debug)]
pub enum VarOrIntParseError {
    Var(VarError),
    Int(ParseIntError),
}

#[must_use]
#[derive(Debug)]
pub enum ConfigErrorEnum<'a> {
    //todo some fields has been renamed
    Message(String),
    GithubNameError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    GithubTokenError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    RedditUserAgentError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    RedditClientIdError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    RedditClientSecretError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    RedditUsernameError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    RedditPasswordError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    StartingCheckLinkError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    UserCredentialsDummyHandleError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    WarningLogsDirectoryNameError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDirError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    EnableProvidersError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsCleaningWarningLogsDirectoryEnabledError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbInMongoError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableTimeMeasurementError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableProviderLinksLimitError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCommonProvidersLinksLimitError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    CommonProvidersLinksLimitError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    EnableRandomizeOrderForProvidersLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsPrintsEnabledError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableErrorPrintsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningHighPrintsEnabledError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningLowPrintsEnabledError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableSuccessPrintsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnablePartialSuccessPrintsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableTimeMeasurementPrintsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsCleaningWarningLogsDirectoryEnabledPrintsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInfoPrintsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableAllProvidersPrintsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableErrorPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningHighPrintsEnabledAllProvidersError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningLowPrintsEnabledAllProvidersError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableSuccessPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnablePartialSuccessPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableTimeMeasurementPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsCleaningWarningLogsDirectoryEnabledPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInfoPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWriteErrorLogsInLocalFolderEnabledError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableWriteErrorLogsInMongoError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInitializeMongoWithProvidersLinkPartsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    ProvidersDbNameHandleError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    ProvidersDbCollectionHandleSecondPartError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    ProvidersDbCollectionDocumentFieldNameHandleError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    DbProvidersLogsNameHandleError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    DbProvidersLogsCollectionHandleSecondPartError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    DbProvidersLogsCollectionDocumentFieldNameHandleError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    PathToProviderLinkPartsFolderError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    LogFileExtensionError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    EnableInitializeMongoWithArxivLinkPartsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInitializeMongoWithBiorxivLinkPartsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInitializeMongoWithGithubLinkPartsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInitializeMongoWithHabrLinkPartsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInitializeMongoWithMedrxivLinkPartsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInitializeMongoWithRedditLinkPartsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInitializeMongoWithTwitterLinkPartsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    MongoFirstHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    MongoSecondHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    MongoThirdHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    MongoFourthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    MongoFifthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    MongoLoginError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    MongoPasswordError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    MongoIpError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    MongoPortError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    MongoParamsError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    PostgresFirstHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    PostgresSecondHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    PostgresThirdHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    PostgresFourthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    PostgresFifthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    PostgresLoginError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    PostgresPasswordError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    PostgresIpError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    PostgresPortError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    PostgresDbError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    EnableArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    ArxivLinkError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    BiorxivLinkError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    GithubLinkError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    HabrLinkError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    MedrxivLinkError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    RedditLinkError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    TwitterLinkError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarError,
    },
    IsPrintsEnabledArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsPrintsEnabledBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsPrintsEnabledGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsPrintsEnabledHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsPrintsEnabledMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsPrintsEnabledRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsPrintsEnabledTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningHighPrintsEnabledArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningHighPrintsEnabledBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningHighPrintsEnabledGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningHighPrintsEnabledHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningHighPrintsEnabledMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningHighPrintsEnabledRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningHighPrintsEnabledTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningLowPrintsEnabledArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningLowPrintsEnabledBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningLowPrintsEnabledGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningLowPrintsEnabledHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningLowPrintsEnabledMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningLowPrintsEnabledRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsWarningLowPrintsEnabledTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableErrorPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableErrorPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableErrorPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableErrorPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableErrorPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableErrorPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableErrorPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableSuccessPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableSuccessPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableSuccessPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableSuccessPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableSuccessPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableSuccessPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableSuccessPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnablePartialSuccessPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnablePartialSuccessPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnablePartialSuccessPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnablePartialSuccessPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnablePartialSuccessPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnablePartialSuccessPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnablePartialSuccessPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsCleaningWarningLogsDirectoryEnabledArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsCleaningWarningLogsDirectoryEnabledBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsCleaningWarningLogsDirectoryEnabledGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsCleaningWarningLogsDirectoryEnabledHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsCleaningWarningLogsDirectoryEnabledMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsCleaningWarningLogsDirectoryEnabledRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    IsCleaningWarningLogsDirectoryEnabledTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbInMongoForArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbInMongoForBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbInMongoForGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbInMongoForHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbInMongoForMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbInMongoForRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbInMongoForTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableTimeMeasurementForArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableTimeMeasurementForBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableTimeMeasurementForGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableTimeMeasurementForHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableTimeMeasurementForMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableTimeMeasurementForRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableTimeMeasurementForTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInfoForArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInfoForBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInfoForGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInfoForHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInfoForMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInfoForRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableInfoForTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableLinksLimitForArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableLinksLimitForBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableLinksLimitForGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableLinksLimitForHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableLinksLimitForMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableLinksLimitForRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableLinksLimitForTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    LinksLimitForArxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    LinksLimitForBiorxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    LinksLimitForGithubError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    LinksLimitForHabrError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    LinksLimitForMedrxivError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    LinksLimitForRedditError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    LinksLimitForTwitterError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    EnableRandomizeOrderForArxivLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableRandomizeOrderForBiorxivLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableRandomizeOrderForGithubLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableRandomizeOrderForHabrLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableRandomizeOrderForMedrxivLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableRandomizeOrderForRedditLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    EnableRandomizeOrderForTwitterLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrBoolParseError,
    },
    ErrorRedError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    ErrorGreenError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    ErrorBlueError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    WarningHighRedError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    WarningHighGreenError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    WarningHighBlueError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    WarningLowRedError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    WarningLowGreenError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    WarningLowBlueError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    SuccessRedError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    SuccessGreenError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    SuccessBlueError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    PartialSuccessRedError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    PartialSuccessGreenError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    PartialSuccessBlueError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    CleaningRedError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    CleaningGreenError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    CleaningBlueError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    TimeMeasurementRedError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    TimeMeasurementGreenError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    TimeMeasurementBlueError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    InfoRedError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    InfoGreenError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
    InfoBlueError {
        was_dotenv_enable: bool,
        env_name: &'a str,
        env_error: VarOrIntParseError,
    },
}
