use std::env::VarError;
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

#[derive(Debug)]
pub enum ConfigErrorInnerType {
    VarErrorHandle(VarError),
    VarOrBoolParseErrorHandle(VarOrBoolParseError),
    VarOrIntParseErrorErrorHandle(VarOrIntParseError)
}

#[must_use]
#[derive(Debug)] 
pub enum ConfigError <'a> {
    GithubNameError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    GithubTokenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    RedditUserAgentError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    RedditClientIdError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    RedditClientSecretError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    RedditUsernameError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    RedditPasswordError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    StartingCheckLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    UserCredentialsDummyHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    WarningLogsDirectoryNameError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDirError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDirectoryError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbInMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableTimeMeasurementError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableProviderLinksLimitError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCommonProvidersLinksLimitError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    CommonProvidersLinksLimitError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableRandomizeOrderForProvidersLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableErrorPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningHighPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningLowPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableSuccessPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePartialSuccessPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableTimeMeasurementPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDirectoryPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInfoPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableAllProvidersPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableErrorPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningHighPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningLowPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableSuccessPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePartialSuccessPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableTimeMeasurementPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDirectoryPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInfoPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWriteErrorLogsInLocalFolderError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWriteErrorLogsInMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInitializeMongoWithProvidersLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    ProvidersDbNameHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    ProvidersDbCollectionHandleSecondPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    ProvidersDbCollectionDocumentFieldNameHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    DbProvidersLogsNameHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    DbProvidersLogsCollectionHandleSecondPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    DbProvidersLogsCollectionDocumentFieldNameHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PathToProviderLinkPartsFolderError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    LogFileExtensionError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInitializeMongoWithArxivLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInitializeMongoWithBiorxivLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInitializeMongoWithGithubLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInitializeMongoWithHabrLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInitializeMongoWithMedrxivLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInitializeMongoWithRedditLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInitializeMongoWithTwitterLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    MongoFirstHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    MongoSecondHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    MongoThirdHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    MongoFourthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    MongoFifthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    MongoLoginError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    MongoPasswordError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    MongoIpError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    MongoPortError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    MongoParamsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PostgresFirstHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PostgresSecondHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PostgresThirdHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PostgresFourthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PostgresFifthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PostgresLoginError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PostgresPasswordError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PostgresIpError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PostgresPortError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PostgresDbError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    ArxivLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    BiorxivLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    GithubLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    HabrLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    MedrxivLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    RedditLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    TwitterLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePrintsArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePrintsBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePrintsGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePrintsHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePrintsMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePrintsRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePrintsTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningHighPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningHighPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningHighPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningHighPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningHighPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningHighPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningHighPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningLowPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningLowPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningLowPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningLowPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningLowPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningLowPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableWarningLowPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableErrorPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableErrorPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableErrorPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableErrorPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableErrorPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableErrorPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableErrorPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableSuccessPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableSuccessPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableSuccessPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableSuccessPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableSuccessPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableSuccessPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableSuccessPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePartialSuccessPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePartialSuccessPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePartialSuccessPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePartialSuccessPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePartialSuccessPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePartialSuccessPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnablePartialSuccessPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDirectoryForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDirectoryForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDirectoryForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDirectoryForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDirectoryForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDirectoryForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDirectoryForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbInMongoForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbInMongoForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbInMongoForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbInMongoForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbInMongoForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbInMongoForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbInMongoForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableTimeMeasurementForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableTimeMeasurementForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableTimeMeasurementForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableTimeMeasurementForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableTimeMeasurementForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableTimeMeasurementForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableTimeMeasurementForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInfoForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInfoForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInfoForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInfoForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInfoForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInfoForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableInfoForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableLinksLimitForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableLinksLimitForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableLinksLimitForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableLinksLimitForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableLinksLimitForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableLinksLimitForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableLinksLimitForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    LinksLimitForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    LinksLimitForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    LinksLimitForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    LinksLimitForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    LinksLimitForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    LinksLimitForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    LinksLimitForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableRandomizeOrderForArxivLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableRandomizeOrderForBiorxivLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableRandomizeOrderForGithubLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableRandomizeOrderForHabrLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableRandomizeOrderForMedrxivLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableRandomizeOrderForRedditLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    EnableRandomizeOrderForTwitterLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    ErrorRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    ErrorGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    ErrorBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    WarningHighRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    WarningHighGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    WarningHighBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    WarningLowRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    WarningLowGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    WarningLowBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    SuccessRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    SuccessGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    SuccessBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PartialSuccessRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PartialSuccessGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    PartialSuccessBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    CleaningRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    CleaningGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    CleaningBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    TimeMeasurementRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    TimeMeasurementGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    TimeMeasurementBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    InfoRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    InfoGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
    InfoBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: ConfigErrorInnerType,
    },
}
    