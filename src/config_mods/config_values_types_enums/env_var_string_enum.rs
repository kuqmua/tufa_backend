use procedural_macros_lib::AllVariants;
use procedural_macros_lib::EnumVariantCount;

use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(
    AllVariants,
    EnumVariantCount,
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

impl EnvStringVar {
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
}
