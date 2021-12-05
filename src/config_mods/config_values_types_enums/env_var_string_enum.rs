use procedural_macros_lib::AllVariants;
use procedural_macros_lib::EnumVariantCount;

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
)]
pub enum EnvStringVar {
    GithubName,
    GithubToken,
    RedditUserAgent,
    RedditClientId,
    RedditClientSecret,
    RedditUsername,
    RedditPassword,

    WarningLogsDirectoryName,
    UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir,
    ProvidersDbNameHandle,
    ProvidersDbCollectionHandleSecondPart,
    ProvidersDbCollectionDocumentFieldNameHandle,
    PathToProviderLinkPartsFolder,
    LogFileExtension,
    DbProvidersLogsNameHandle,
    DbProvidersLogsCollectionHandleSecondPart,
    DbProvidersLogsCollectionDocumentFieldNameHandle,

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
