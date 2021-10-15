
use serde::de;
use serde::ser;
use std::error::Error;
use std::fmt;
use std::env::VarError;

impl <'a> fmt::Display for ConfigError <'a>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::Message(ref s) => write!(f, "{}", s),
            _ => write!(f, "dfhdfhdfh{}", self.to_string()),
        }
    }
}

impl <'a> Error for ConfigError <'a>{
    // is it deprecated?
    // fn description(&self) -> &str {
    //     match *self {
    //         ConfigError::Frozen => "configuration is frozen",
    //         _ => "configuration error",
    //     }
    // }
    //dont know about it yet
    // fn cause(&self) -> Option<&Error> {
    //     match *self {
    //         ConfigError::Foreign(ref cause) | ConfigError::FileParse { ref cause, .. } => {
    //             Some(cause.as_ref())
    //         }
    //         ConfigError::Frozen => "configuration is frozen",
    //         _ => None,
    //     }
    // }
}

impl<'a> de::Error for ConfigError <'a> {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        ConfigError::Message(msg.to_string())
    }
}

impl <'a> ser::Error for ConfigError <'a> {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        ConfigError::Message(msg.to_string())
    }
}

#[derive(Debug)] 
pub enum ConfigError <'a> {
    Frozen,
    Message(String),
    GithubNameError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    GithubTokenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    RedditUserAgentError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    RedditClientIdError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    RedditClientSecretError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    RedditUsernameError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    RedditPasswordError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    StartingCheckLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    UserCredentialsDummyHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    WarningLogsDirectoryNameError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDirError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDirectoryError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbInMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableTimeMeasurementError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableProviderLinksLimitError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCommonProvidersLinksLimitError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    CommonProvidersLinksLimitError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableRandomizeOrderForProvidersLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableErrorPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningHighPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningLowPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableSuccessPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePartialSuccessPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableTimeMeasurementPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDirectoryPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInfoPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableAllProvidersPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableErrorPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningHighPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningLowPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableSuccessPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePartialSuccessPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableTimeMeasurementPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDirectoryPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInfoPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWriteErrorLogsInLocalFolderError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWriteErrorLogsInMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInitializeMongoWithProvidersLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ProvidersDbNameHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ProvidersDbCollectionHandleSecondPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ProvidersDbCollectionDocumentFieldNameHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    DbProvidersLogsNameHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    DbProvidersLogsCollectionHandleSecondPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    DbProvidersLogsCollectionDocumentFieldNameHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PathToProviderLinkPartsFolderError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    LogFileExtensionError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithArxivLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithBiorxivLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithGithubLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithHabrLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithMedrxivLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithRedditLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithTwitterLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    MongoFirstHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    MongoSecondHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    MongoThirdHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    MongoFourthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    MongoFifthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    MongoLoginError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    MongoPasswordError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    MongoIpError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    MongoPortError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    MongoParamsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PostgresFirstHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PostgresSecondHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PostgresThirdHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PostgresFourthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PostgresFifthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PostgresLoginError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PostgresPasswordError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PostgresIpError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PostgresPortError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PostgresDbError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ArxivLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    BiorxivLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    GithubLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    HabrLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    MedrxivLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    RedditLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    TwitterLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePrintsArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePrintsBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePrintsGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePrintsHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePrintsMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePrintsRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePrintsTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningHighPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningHighPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningHighPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningHighPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningHighPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningHighPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningHighPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningLowPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningLowPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningLowPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningLowPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningLowPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningLowPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableWarningLowPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableErrorPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableErrorPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableErrorPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableErrorPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableErrorPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableErrorPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableErrorPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableSuccessPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableSuccessPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableSuccessPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableSuccessPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableSuccessPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableSuccessPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableSuccessPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePartialSuccessPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePartialSuccessPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePartialSuccessPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePartialSuccessPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePartialSuccessPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePartialSuccessPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnablePartialSuccessPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDirectoryForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDirectoryForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDirectoryForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDirectoryForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDirectoryForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDirectoryForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDirectoryForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbInMongoForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbInMongoForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbInMongoForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbInMongoForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbInMongoForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbInMongoForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbInMongoForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableCleaningWarningLogsDbCollectionsInMongoForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableTimeMeasurementForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableTimeMeasurementForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableTimeMeasurementForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableTimeMeasurementForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableTimeMeasurementForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableTimeMeasurementForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableTimeMeasurementForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInfoForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInfoForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInfoForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInfoForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInfoForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInfoForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableInfoForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableLinksLimitForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableLinksLimitForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableLinksLimitForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableLinksLimitForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableLinksLimitForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableLinksLimitForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableLinksLimitForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    LinksLimitForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    LinksLimitForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    LinksLimitForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    LinksLimitForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    LinksLimitForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    LinksLimitForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    LinksLimitForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableRandomizeOrderForArxivLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableRandomizeOrderForBiorxivLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableRandomizeOrderForGithubLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableRandomizeOrderForHabrLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableRandomizeOrderForMedrxivLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableRandomizeOrderForRedditLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    EnableRandomizeOrderForTwitterLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ErrorRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ErrorGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ErrorBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    WarningHighRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    WarningHighGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    WarningHighBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    WarningLowRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    WarningLowGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    WarningLowBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    SuccessRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    SuccessGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    SuccessBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PartialSuccessRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PartialSuccessGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    PartialSuccessBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    CleaningRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    CleaningGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    CleaningBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    TimeMeasurementRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    TimeMeasurementGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    TimeMeasurementBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    InfoRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    InfoGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    InfoBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
}
    