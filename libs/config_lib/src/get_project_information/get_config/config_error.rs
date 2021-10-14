
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
    ConfigGithubAuthorizationGithubNameError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigGithubAuthorizationGithubTokenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigRedditAuthorizationRedditUserAgentError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigRedditAuthorizationRedditClientIdError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    RedditAuthorizationRedditClientSecretError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    RedditAuthorizationRedditUsernameError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    RedditAuthorizationRedditPasswordError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ParamsStartingCheckLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsUserCredentialsDummyHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ParamsWarningLogsDirectoryNameError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ParamsUnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDirError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ParamsEnableProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ParamsEnableCleaningWarningLogsDirectoryError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableCleaningWarningLogsDbInMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableCleaningWarningLogsDbCollectionsInMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableTimeMeasurementError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableProviderLinksLimitError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableCommonProvidersLinksLimitError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsCommonProvidersLinksLimitError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableRandomizeOrderForProvidersLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnablePrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableErrorPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableWarningHighPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableWarningLowPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableSuccessPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnablePartialSuccessPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableTimeMeasurementPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableCleaningWarningLogsDirectoryPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableInfoPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableAllProvidersPrintsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableErrorPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableWarningHighPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableWarningLowPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableSuccessPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnablePartialSuccessPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableTimeMeasurementPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableCleaningWarningLogsDirectoryPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableInfoPrintsForAllProvidersError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableWriteErrorLogsInLocalFolderError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableWriteErrorLogsInMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigParamsEnableInitializeMongoWithProvidersLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsProvidersDbNameHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsProvidersDbCollectionHandleSecondPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsProvidersDbCollectionDocumentFieldNameHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsDbProvidersLogsNameHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsDbProvidersLogsCollectionHandleSecondPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsDbProvidersLogsCollectionDocumentFieldNameHandleError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsPathToProviderLinkPartsFolderError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsLogFileExtensionError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsEnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithArxivLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsEnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithBiorxivLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsEnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithGithubLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsEnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithHabrLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsEnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithMedrxivLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsEnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithRedditLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsEnableInitializeMongoWithProvidersLinkPartsEnableInitializeMongoWithTwitterLinkPartsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsMongoUrlPartsMongoFirstHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsMongoUrlPartsMongoSecondHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsMongoUrlPartsMongoThirdHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsMongoUrlPartsMongoFourthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoParamsMongoUrlPartsMongoFifthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoAuthorizationMongoLoginError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoAuthorizationMongoPasswordError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoAuthorizationMongoIpError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoAuthorizationMongoPortError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigMongoAuthorizationMongoParamsError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPostgresParamsPostgresUrlPartsPostgresFirstHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPostgresParamsPostgresUrlPartsPostgresSecondHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPostgresParamsPostgresUrlPartsPostgresThirdHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPostgresParamsPostgresUrlPartsPostgresFourthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPostgresParamsPostgresUrlPartsPostgresFifthHandleUrlPartError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPostgresAuthorizationPostgresLoginError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPostgresAuthorizationPostgresPasswordError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPostgresAuthorizationPostgresIpError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPostgresAuthorizationPostgresPortError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPostgresAuthorizationPostgresDbError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersEnableArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersEnableBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersEnableGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersEnableHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersEnableMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersEnableRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersEnableTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersCheckLinksArxivLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersCheckLinksBiorxivLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersCheckLinksGithubLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersCheckLinksHabrLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersCheckLinksMedrxivLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersCheckLinksRedditLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersCheckLinksTwitterLinkError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersPrintsEnablePrintsArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersPrintsEnablePrintsBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersPrintsEnablePrintsGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersPrintsEnablePrintsHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersPrintsEnablePrintsMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersPrintsEnablePrintsRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersPrintsEnablePrintsTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningHighProvidersPrintsEnableWarningHighPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningHighProvidersPrintsEnableWarningHighPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningHighProvidersPrintsEnableWarningHighPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningHighProvidersPrintsEnableWarningHighPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningHighProvidersPrintsEnableWarningHighPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningHighProvidersPrintsEnableWarningHighPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningHighProvidersPrintsEnableWarningHighPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningLowProvidersPrintsEnableWarningLowPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningLowProvidersPrintsEnableWarningLowPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningLowProvidersPrintsEnableWarningLowPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningLowProvidersPrintsEnableWarningLowPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningLowProvidersPrintsEnableWarningLowPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningLowProvidersPrintsEnableWarningLowPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableWarningLowProvidersPrintsEnableWarningLowPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableSuccessProvidersPrintsEnableSuccessPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableSuccessProvidersPrintsEnableSuccessPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableSuccessProvidersPrintsEnableSuccessPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableSuccessProvidersPrintsEnableSuccessPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableSuccessProvidersPrintsEnableSuccessPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableSuccessProvidersPrintsEnableSuccessPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableSuccessProvidersPrintsEnableSuccessPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnablePartialSuccessProvidersPrintsEnablePartialSuccessPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnablePartialSuccessProvidersPrintsEnablePartialSuccessPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnablePartialSuccessProvidersPrintsEnablePartialSuccessPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnablePartialSuccessProvidersPrintsEnablePartialSuccessPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnablePartialSuccessProvidersPrintsEnablePartialSuccessPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnablePartialSuccessProvidersPrintsEnablePartialSuccessPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnablePartialSuccessProvidersPrintsEnablePartialSuccessPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableErrorProvidersPrintsEnableErrorPrintsForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableErrorProvidersPrintsEnableErrorPrintsForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableErrorProvidersPrintsEnableErrorPrintsForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableErrorProvidersPrintsEnableErrorPrintsForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableErrorProvidersPrintsEnableErrorPrintsForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableErrorProvidersPrintsEnableErrorPrintsForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableErrorProvidersPrintsEnableErrorPrintsForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDirectoryEnableCleaningWarningLogsDirectoryForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDirectoryEnableCleaningWarningLogsDirectoryForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDirectoryEnableCleaningWarningLogsDirectoryForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDirectoryEnableCleaningWarningLogsDirectoryForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDirectoryEnableCleaningWarningLogsDirectoryForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDirectoryEnableCleaningWarningLogsDirectoryForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDirectoryEnableCleaningWarningLogsDirectoryForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbInMongoEnableCleaningWarningLogsDbInMongoForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbInMongoEnableCleaningWarningLogsDbInMongoForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbInMongoEnableCleaningWarningLogsDbInMongoForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbInMongoEnableCleaningWarningLogsDbInMongoForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbInMongoEnableCleaningWarningLogsDbInMongoForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbInMongoEnableCleaningWarningLogsDbInMongoForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbInMongoEnableCleaningWarningLogsDbInMongoForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbCollectionsInMongoEnableCleaningWarningLogsDbCollectionsInMongoForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbCollectionsInMongoEnableCleaningWarningLogsDbCollectionsInMongoForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbCollectionsInMongoEnableCleaningWarningLogsDbCollectionsInMongoForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbCollectionsInMongoEnableCleaningWarningLogsDbCollectionsInMongoForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbCollectionsInMongoEnableCleaningWarningLogsDbCollectionsInMongoForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbCollectionsInMongoEnableCleaningWarningLogsDbCollectionsInMongoForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersCleaningWarningLogsDbCollectionsInMongoEnableCleaningWarningLogsDbCollectionsInMongoForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersTimeMeasurementEnableTimeMeasurementForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersTimeMeasurementEnableTimeMeasurementForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersTimeMeasurementEnableTimeMeasurementForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersTimeMeasurementEnableTimeMeasurementForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersTimeMeasurementEnableTimeMeasurementForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersTimeMeasurementEnableTimeMeasurementForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersTimeMeasurementEnableTimeMeasurementForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersInfoEnableInfoForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersInfoEnableInfoForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersInfoEnableInfoForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersInfoEnableInfoForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersInfoEnableInfoForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersInfoEnableInfoForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersInfoEnableInfoForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersLinksLimitsEnableLinksLimitForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersLinksLimitsEnableLinksLimitForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersLinksLimitsEnableLinksLimitForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersLinksLimitsEnableLinksLimitForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersLinksLimitsEnableLinksLimitForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersLinksLimitsEnableLinksLimitForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableProvidersLinksLimitsEnableLinksLimitForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersLinksLimitsLinksLimitForArxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersLinksLimitsLinksLimitForBiorxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersLinksLimitsLinksLimitForGithubError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersLinksLimitsLinksLimitForHabrError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersLinksLimitsLinksLimitForMedrxivError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersLinksLimitsLinksLimitForRedditError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigProvidersLinksLimitsLinksLimitForTwitterError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableRandomizeOrderForProvidersLinkPartsForMongoEnableRandomizeOrderForArxivLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableRandomizeOrderForProvidersLinkPartsForMongoEnableRandomizeOrderForBiorxivLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableRandomizeOrderForProvidersLinkPartsForMongoEnableRandomizeOrderForGithubLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableRandomizeOrderForProvidersLinkPartsForMongoEnableRandomizeOrderForHabrLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableRandomizeOrderForProvidersLinkPartsForMongoEnableRandomizeOrderForMedrxivLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableRandomizeOrderForProvidersLinkPartsForMongoEnableRandomizeOrderForRedditLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigEnableRandomizeOrderForProvidersLinkPartsForMongoEnableRandomizeOrderForTwitterLinkPartsForMongoError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsErrorRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsErrorGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsErrorBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsWarningHighRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsWarningHighGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsWarningHighBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsWarningLowRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsWarningLowGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsWarningLowBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsSuccessRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsSuccessGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsSuccessBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsPartialSuccessRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsPartialSuccessGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsPartialSuccessBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsCleaningRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsCleaningGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsCleaningBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsTimeMeasurementRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsTimeMeasurementGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsTimeMeasurementBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsInfoRedError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsInfoGreenError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
    ConfigPrintColorsInfoBlueError {
        was_dotenv_enable: bool,
        env_name: & 'a str,
        env_error: VarError,
    },
}
    