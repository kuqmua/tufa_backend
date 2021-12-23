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
pub enum EnvStringVar {
    GithubName,
    GithubToken,

    RedditUserAgent,
    RedditClientId,
    RedditClientSecret,
    RedditUsername,
    RedditPassword,

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

    WarningLogsDirectoryName,
    UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir,
    PathToProviderLinkPartsFolder,
    LogFileExtension,

    StartingCheckLink,
    ArxivCheckLink,
    BiorxivCheckLink,
    GithubCheckLink,
    HabrCheckLink,
    MedrxivCheckLink,
    RedditCheckLink,
    TwitterCheckLink,
}
