// pub enum ALL ENV VARS NAMES
// HABR_NAME_TO_CHECKand Hashtable

pub enum EnvVarsEnum {
    GithubNameEnvName (String),
    GithubTokenEnvName (String),

    RedditUserAgentEnvName (String),
    RedditClientIdEnvName (String),
    RedditClientSecretEnvName (String),
    RedditUsernameEnvName (String),
    RedditPasswordEnvName (String),

    StartingCheckLinkEnvName (String),
    UserCredentialsDummyHandleEnvName (String),
    WarningLogsDirectoryNameEnvName (String),
    UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDirEnvName (String),
    EnableProvidersEnvName (bool),
    EnableCleaningWarningLogsDirectoryEnvName (bool),
    EnableCleaningWarningLogsDbInMongoEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoEnvName (bool),
    EnableTimeMeasurementEnvName (bool),
    EnableProviderLinksLimitEnvName (bool),
    EnableCommonProvidersLinksLimitEnvName (bool),
    CommonProvidersLinksLimitEnvName (i64),
    EnableRandomizeOrderForProvidersLinkPartsForMongoEnvName (bool),
    EnablePrintsEnvName (bool),
    EnableErrorPrintsEnvName (bool),
    EnableWarningHighPrintsEnvName (bool),
    EnableWarningLowPrintsEnvName (bool),
    EnableSuccessPrintsEnvName (bool),
    EnablePartialSuccessPrintsEnvName (bool),
    EnableTimeMeasurementPrintsEnvName (bool),
    EnableCleaningWarningLogsDirectoryPrintsEnvName (bool),
    EnableInfoPrintsEnvName (bool),
    EnableAllProvidersPrintsEnvName (bool),
    EnableErrorPrintsForAllProvidersEnvName (bool),
    EnableWarningHighPrintsForAllProvidersEnvName (bool),
    EnableWarningLowPrintsForAllProvidersEnvName (bool),
    EnableSuccessPrintsForAllProvidersEnvName (bool),
    EnablePartialSuccessPrintsForAllProvidersEnvName (bool),
    EnableTimeMeasurementPrintsForAllProvidersEnvName (bool),
    EnableCleaningWarningLogsDirectoryPrintsForAllProvidersEnvName (bool),
    EnableInfoPrintsForAllProvidersEnvName (bool),
    EnableWriteErrorLogsInLocalFolderEnvName (bool),
    EnableWriteErrorLogsInMongoEnvName (bool),
    EnableInitializeMongoWithProvidersLinkPartsEnvName (bool),

    ProvidersDbNameHandleEnvName (String),
    ProvidersDbCollectionHandleSecondPartEnvName (String),
    ProvidersDbCollectionDocumentFieldNameHandleEnvName (String),
    PathToProviderLinkPartsFolderEnvName (String),
    LogFileExtensionEnvName (String),
    DbProvidersLogsNameHandleEnvName (String),
    DbProvidersLogsCollectionHandleSecondPartEnvName (String),
    DbProvidersLogsCollectionDocumentFieldNameHandleEnvName (String),

    EnableInitializeMongoWithArxivLinkPartsEnvName (bool),
    EnableInitializeMongoWithBiorxivLinkPartsEnvName (bool),
    EnableInitializeMongoWithGithubLinkPartsEnvName (bool),
    EnableInitializeMongoWithHabrLinkPartsEnvName (bool),
    EnableInitializeMongoWithMedrxivLinkPartsEnvName (bool),
    EnableInitializeMongoWithRedditLinkPartsEnvName (bool),
    EnableInitializeMongoWithTwitterLinkPartsEnvName (bool),

    MongoFirstHandleUrlPartEnvName (String),
    MongoSecondHandleUrlPartEnvName (String),
    MongoThirdHandleUrlPartEnvName (String),
    MongoFourthHandleUrlPartEnvName (String),
    MongoFifthHandleUrlPartEnvName (String),

    MongoLoginEnvName (String),
    MongoPasswordEnvName (String),
    MongoIpEnvName (String),
    MongoPortEnvName (String),
    MongoParamsEnvName (String),

    PostgresFirstHandleUrlPartEnvName (String),
    PostgresSecondHandleUrlPartEnvName (String),
    PostgresThirdHandleUrlPartEnvName (String),
    PostgresFourthHandleUrlPartEnvName (String),
    PostgresFifthHandleUrlPartEnvName (String),

    PostgresLoginEnvName (String),
    PostgresPasswordEnvName (String),
    PostgresIpEnvName (String),
    PostgresPortEnvName (String),
    PostgresDbEnvName (String),

    EnableArxivEnvName (bool),
    EnableBiorxivEnvName (bool),
    EnableGithubEnvName (bool),
    EnableHabrEnvName (bool),
    EnableMedrxivEnvName (bool),
    EnableRedditEnvName (bool),
    EnableTwitterEnvName (bool),

    ArxivLinkEnvName (String),
    BiorxivLinkEnvName (String),
    GithubLinkEnvName (String),
    HabrLinkEnvName (String),
    MedrxivLinkEnvName (String),
    RedditLinkEnvName (String),
    TwitterLinkEnvName (String),

    EnablePrintsArxivEnvName (bool),
    EnablePrintsBiorxivEnvName (bool),
    EnablePrintsGithubEnvName (bool),
    EnablePrintsHabrEnvName (bool),
    EnablePrintsMedrxivEnvName (bool),
    EnablePrintsRedditEnvName (bool),
    EnablePrintsTwitterEnvName (bool),

    EnableWarningHighPrintsForArxivEnvName (bool),
    EnableWarningHighPrintsForBiorxivEnvName (bool),
    EnableWarningHighPrintsForGithubEnvName (bool),
    EnableWarningHighPrintsForHabrEnvName (bool),
    EnableWarningHighPrintsForMedrxivEnvName (bool),
    EnableWarningHighPrintsForRedditEnvName (bool),
    EnableWarningHighPrintsForTwitterEnvName (bool),

    EnableWarningLowPrintsForArxivEnvName (bool),
    EnableWarningLowPrintsForBiorxivEnvName (bool),
    EnableWarningLowPrintsForGithubEnvName (bool),
    EnableWarningLowPrintsForHabrEnvName (bool),
    EnableWarningLowPrintsForMedrxivEnvName (bool),
    EnableWarningLowPrintsForRedditEnvName (bool),
    EnableWarningLowPrintsForTwitterEnvName (bool),

    EnableErrorPrintsForArxivEnvName (bool),
    EnableErrorPrintsForBiorxivEnvName (bool),
    EnableErrorPrintsForGithubEnvName (bool),
    EnableErrorPrintsForHabrEnvName (bool),
    EnableErrorPrintsForMedrxivEnvName (bool),
    EnableErrorPrintsForRedditEnvName (bool),
    EnableErrorPrintsForTwitterEnvName (bool),

    EnableSuccessPrintsForArxivEnvName (bool),
    EnableSuccessPrintsForBiorxivEnvName (bool),
    EnableSuccessPrintsForGithubEnvName (bool),
    EnableSuccessPrintsForHabrEnvName (bool),
    EnableSuccessPrintsForMedrxivEnvName (bool),
    EnableSuccessPrintsForRedditEnvName (bool),
    EnableSuccessPrintsForTwitterEnvName (bool),

    EnablePartialSuccessPrintsForArxivEnvName (bool),
    EnablePartialSuccessPrintsForBiorxivEnvName (bool),
    EnablePartialSuccessPrintsForGithubEnvName (bool),
    EnablePartialSuccessPrintsForHabrEnvName (bool),
    EnablePartialSuccessPrintsForMedrxivEnvName (bool),
    EnablePartialSuccessPrintsForRedditEnvName (bool),
    EnablePartialSuccessPrintsForTwitterEnvName (bool),

    EnableCleaningWarningLogsDirectoryForArxivEnvName (bool),
    EnableCleaningWarningLogsDirectoryForBiorxivEnvName (bool),
    EnableCleaningWarningLogsDirectoryForGithubEnvName (bool),
    EnableCleaningWarningLogsDirectoryForHabrEnvName (bool),
    EnableCleaningWarningLogsDirectoryForMedrxivEnvName (bool),
    EnableCleaningWarningLogsDirectoryForRedditEnvName (bool),
    EnableCleaningWarningLogsDirectoryForTwitterEnvName (bool),

    EnableCleaningWarningLogsDbInMongoForArxivEnvName (bool),
    EnableCleaningWarningLogsDbInMongoForBiorxivEnvName (bool),
    EnableCleaningWarningLogsDbInMongoForGithubEnvName (bool),
    EnableCleaningWarningLogsDbInMongoForHabrEnvName (bool),
    EnableCleaningWarningLogsDbInMongoForMedrxivEnvName (bool),
    EnableCleaningWarningLogsDbInMongoForRedditEnvName (bool),
    EnableCleaningWarningLogsDbInMongoForTwitterEnvName (bool),

    EnableCleaningWarningLogsDbCollectionsInMongoForArxivEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoForBiorxivEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoForGithubEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoForHabrEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoForMedrxivEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoForRedditEnvName (bool),
    EnableCleaningWarningLogsDbCollectionsInMongoForTwitterEnvName (bool),

    EnableTimeMeasurementForArxivEnvName (bool),
    EnableTimeMeasurementForBiorxivEnvName (bool),
    EnableTimeMeasurementForGithubEnvName (bool),
    EnableTimeMeasurementForHabrEnvName (bool),
    EnableTimeMeasurementForMedrxivEnvName (bool),
    EnableTimeMeasurementForRedditEnvName (bool),
    EnableTimeMeasurementForTwitterEnvName (bool),

    EnableInfoForArxivEnvName (bool),
    EnableInfoForBiorxivEnvName (bool),
    EnableInfoForGithubEnvName (bool),
    EnableInfoForHabrEnvName (bool),
    EnableInfoForMedrxivEnvName (bool),
    EnableInfoForRedditEnvName (bool),
    EnableInfoForTwitterEnvName (bool),

    EnableLinksLimitForArxivEnvName (bool),
    EnableLinksLimitForBiorxivEnvName (bool),
    EnableLinksLimitForGithubEnvName (bool),
    EnableLinksLimitForHabrEnvName (bool),
    EnableLinksLimitForMedrxivEnvName (bool),
    EnableLinksLimitForRedditEnvName (bool),
    EnableLinksLimitForTwitterEnvName (bool),

    EnableRandomizeOrderForArxivLinkPartsForMongoEnvName (bool),
    EnableRandomizeOrderForBiorxivLinkPartsForMongoEnvName (bool),
    EnableRandomizeOrderForGithubLinkPartsForMongoEnvName (bool),
    EnableRandomizeOrderForHabrLinkPartsForMongoEnvName (bool),
    EnableRandomizeOrderForMedrxivLinkPartsForMongoEnvName (bool),
    EnableRandomizeOrderForRedditLinkPartsForMongoEnvName (bool),
    EnableRandomizeOrderForTwitterLinkPartsForMongoEnvName (bool),

    LinksLimitForArxivEnvName (i64),
    LinksLimitForBiorxivEnvName (i64),
    LinksLimitForGithubEnvName (i64),
    LinksLimitForHabrEnvName (i64),
    LinksLimitForMedrxivEnvName (i64),
    LinksLimitForRedditEnvName (i64),
    LinksLimitForTwitterEnvName (i64),

    ErrorRedEnvName (u8),
    ErrorGreenEnvName (u8),
    ErrorBlueEnvName (u8),
    WarningHighRedEnvName (u8),
    WarningHighGreenEnvName (u8),
    WarningHighBlueEnvName (u8),
    WarningLowRedEnvName (u8),
    WarningLowGreenEnvName (u8),
    WarningLowBlueEnvName (u8),
    SuccessRedEnvName (u8),
    SuccessGreenEnvName (u8),
    SuccessBlueEnvName (u8),
    PartialSuccessRedEnvName (u8),
    PartialSuccessGreenEnvName (u8),
    PartialSuccessBlueEnvName (u8),
    CleaningRedEnvName (u8),
    CleaningGreenEnvName (u8),
    CleaningBlueEnvName (u8),
    TimeMeasurementRedEnvName (u8),
    TimeMeasurementGreenEnvName (u8),
    TimeMeasurementBlueEnvName (u8),
    InfoRedEnvName (u8),
    InfoGreenEnvName (u8),
    InfoBlueEnvName (u8),
}
//     config_provider_string_to_enum_struct_hasmap

// pub struct ConfigProviderStringToEnumTypeStruct {
//     pub config_name_value: &'static str,
//     pub provider_kind_enum_type: ProviderKind,
// }

// impl ConfigProviderStringToEnumTypeStruct {
//     pub const fn new(
//         config_name_value: &'static str,
//         provider_kind_enum_type: ProviderKind,
//     ) -> Self {
//         ConfigProviderStringToEnumTypeStruct {
//             config_name_value,
//             provider_kind_enum_type,
//         }
//     }
// }

pub const GITHUB_NAME_ENV_NAME: &str = "GITHUB_NAME";
pub const GITHUB_TOKEN_ENV_NAME: &str = "GITHUB_TOKEN";

pub const REDDIT_USER_AGENT_ENV_NAME: &str = "REDDIT_USER_AGENT";
pub const REDDIT_CLIENT_ID_ENV_NAME: &str = "REDDIT_CLIENT_ID";
pub const REDDIT_CLIENT_SECRET_ENV_NAME: &str = "REDDIT_CLIENT_SECRET";
pub const REDDIT_USERNAME_ENV_NAME: &str = "REDDIT_USERNAME";
pub const REDDIT_PASSWORD_ENV_NAME: &str = "REDDIT_PASSWORD";

pub const STARTING_CHECK_LINK_ENV_NAME: &str = "STARTING_CHECK_LINK";
pub const USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME: &str = "USER_CREDENTIALS_DUMMY_HANDLE";
pub const WARNING_LOGS_DIRECTORY_NAME_ENV_NAME: &str = "WARNING_LOGS_DIRECTORY_NAME";
pub const UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME: &str =
    "UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR";
pub const ENABLE_PROVIDERS_ENV_NAME: &str = "ENABLE_PROVIDERS";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO";
pub const ENABLE_TIME_MEASUREMENT_ENV_NAME: &str = "ENABLE_TIME_MEASUREMENT";
pub const ENABLE_PROVIDER_LINKS_LIMIT_ENV_NAME: &str = "ENABLE_PROVIDER_LINKS_LIMIT";
pub const ENABLE_COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME: &str =
    "ENABLE_COMMON_PROVIDERS_LINKS_LIMIT";
pub const COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME: &str = "COMMON_PROVIDERS_LINKS_LIMIT";
pub const ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO";
pub const ENABLE_PRINTS_ENV_NAME: &str = "ENABLE_PRINTS";
pub const ENABLE_ERROR_PRINTS_ENV_NAME: &str = "ENABLE_ERROR_PRINTS";
pub const ENABLE_WARNING_HIGH_PRINTS_ENV_NAME: &str = "ENABLE_WARNING_HIGH_PRINTS";
pub const ENABLE_WARNING_LOW_PRINTS_ENV_NAME: &str = "ENABLE_WARNING_LOW_PRINTS";
pub const ENABLE_SUCCESS_PRINTS_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_ENV_NAME: &str = "ENABLE_PARTIAL_SUCCESS_PRINTS";
pub const ENABLE_TIME_MEASUREMENT_PRINTS_ENV_NAME: &str = "ENABLE_TIME_MEASUREMENT_PRINTS";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS";
pub const ENABLE_INFO_PRINTS_ENV_NAME: &str = "ENABLE_INFO_PRINTS";
pub const ENABLE_ALL_PROVIDERS_PRINTS_ENV_NAME: &str = "ENABLE_ALL_PROVIDERS_PRINTS";
pub const ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS_ENV_NAME: &str =
    "ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS";
pub const ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_ENV_NAME: &str =
    "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER";
pub const ENABLE_WRITE_ERROR_LOGS_IN_MONGO_ENV_NAME: &str = "ENABLE_WRITE_ERROR_LOGS_IN_MONGO";
pub const ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS";

pub const PROVIDERS_DB_NAME_HANDLE_ENV_NAME: &str = "PROVIDERS_DB_NAME_HANDLE";
pub const PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME: &str =
    "PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART";
pub const PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME: &str =
    "PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE";
pub const PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME: &str = "PATH_TO_PROVIDER_LINK_PARTS_FOLDER";
pub const LOG_FILE_EXTENSION_ENV_NAME: &str = "LOG_FILE_EXTENSION";
pub const DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME: &str = "DB_PROVIDERS_LOGS_NAME_HANDLE";
pub const DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME: &str =
    "DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART";
pub const DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME: &str =
    "DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE";

pub const ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS";
pub const ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS";
pub const ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS";
pub const ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS";
pub const ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS";
pub const ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS";
pub const ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS_ENV_NAME: &str =
    "ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS";

pub const MONGO_FIRST_HANDLE_URL_PART_ENV_NAME: &str = "MONGO_FIRST_HANDLE_URL_PART";
pub const MONGO_SECOND_HANDLE_URL_PART_ENV_NAME: &str = "MONGO_SECOND_HANDLE_URL_PART";
pub const MONGO_THIRD_HANDLE_URL_PART_ENV_NAME: &str = "MONGO_THIRD_HANDLE_URL_PART";
pub const MONGO_FOURTH_HANDLE_URL_PART_ENV_NAME: &str = "MONGO_FOURTH_HANDLE_URL_PART";
pub const MONGO_FIFTH_HANDLE_URL_PART_ENV_NAME: &str = "MONGO_FIFTH_HANDLE_URL_PART";

pub const MONGO_LOGIN_ENV_NAME: &str = "MONGO_LOGIN";
pub const MONGO_PASSWORD_ENV_NAME: &str = "MONGO_PASSWORD";
pub const MONGO_IP_ENV_NAME: &str = "MONGO_IP";
pub const MONGO_PORT_ENV_NAME: &str = "MONGO_PORT";
pub const MONGO_PARAMS_ENV_NAME: &str = "MONGO_PARAMS";

pub const POSTGRES_FIRST_HANDLE_URL_PART_ENV_NAME: &str = "POSTGRES_FIRST_HANDLE_URL_PART";
pub const POSTGRES_SECOND_HANDLE_URL_PART_ENV_NAME: &str =
    "POSTGRES_SECOND_HANDLE_URL_PART";
pub const POSTGRES_THIRD_HANDLE_URL_PART_ENV_NAME: &str = "POSTGRES_THIRD_HANDLE_URL_PART";
pub const POSTGRES_FOURTH_HANDLE_URL_PART_ENV_NAME: &str =
    "POSTGRES_FOURTH_HANDLE_URL_PART";
pub const POSTGRES_FIFTH_HANDLE_URL_PART_ENV_NAME: &str = "POSTGRES_FIFTH_HANDLE_URL_PART";

pub const POSTGRES_LOGIN_ENV_NAME: &str = "POSTGRES_LOGIN";
pub const POSTGRES_PASSWORD_ENV_NAME: &str = "POSTGRES_PASSWORD";
pub const POSTGRES_IP_ENV_NAME: &str = "POSTGRES_IP";
pub const POSTGRES_PORT_ENV_NAME: &str = "POSTGRES_PORT";
pub const POSTGRES_DB_ENV_NAME: &str = "POSTGRES_DB";

pub const ENABLE_ARXIV_ENV_NAME: &str = "ENABLE_ARXIV";
pub const ENABLE_BIORXIV_ENV_NAME: &str = "ENABLE_BIORXIV";
pub const ENABLE_GITHUB_ENV_NAME: &str = "ENABLE_GITHUB";
pub const ENABLE_HABR_ENV_NAME: &str = "ENABLE_HABR";
pub const ENABLE_MEDRXIV_ENV_NAME: &str = "ENABLE_MEDRXIV";
pub const ENABLE_REDDIT_ENV_NAME: &str = "ENABLE_REDDIT";
pub const ENABLE_TWITTER_ENV_NAME: &str = "ENABLE_TWITTER";

pub const ARXIV_LINK_ENV_NAME: &str = "ARXIV_LINK";
pub const BIORXIV_LINK_ENV_NAME: &str = "BIORXIV_LINK";
pub const GITHUB_LINK_ENV_NAME: &str = "GITHUB_LINK";
pub const HABR_LINK_ENV_NAME: &str = "HABR_LINK";
pub const MEDRXIV_LINK_ENV_NAME: &str = "MEDRXIV_LINK";
pub const REDDIT_LINK_ENV_NAME: &str = "REDDIT_LINK";
pub const TWITTER_LINK_ENV_NAME: &str = "TWITTER_LINK";

pub const ENABLE_PRINTS_ARXIV_ENV_NAME: &str = "ENABLE_PRINTS_ARXIV";
pub const ENABLE_PRINTS_BIORXIV_ENV_NAME: &str = "ENABLE_PRINTS_BIORXIV";
pub const ENABLE_PRINTS_GITHUB_ENV_NAME: &str = "ENABLE_PRINTS_GITHUB";
pub const ENABLE_PRINTS_HABR_ENV_NAME: &str = "ENABLE_PRINTS_HABR";
pub const ENABLE_PRINTS_MEDRXIV_ENV_NAME: &str = "ENABLE_PRINTS_MEDRXIV";
pub const ENABLE_PRINTS_REDDIT_ENV_NAME: &str = "ENABLE_PRINTS_REDDIT";
pub const ENABLE_PRINTS_TWITTER_ENV_NAME: &str = "ENABLE_PRINTS_TWITTER";

pub const ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_HABR_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_HABR";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT";
pub const ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER";

pub const ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_HABR_ENV_NAME: &str = "ENABLE_WARNING_LOW_PRINTS_FOR_HABR";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT";
pub const ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER";

pub const ENABLE_ERROR_PRINTS_FOR_ARXIV_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_ARXIV";
pub const ENABLE_ERROR_PRINTS_FOR_BIORXIV_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_BIORXIV";
pub const ENABLE_ERROR_PRINTS_FOR_GITHUB_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_GITHUB";
pub const ENABLE_ERROR_PRINTS_FOR_HABR_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_HABR";
pub const ENABLE_ERROR_PRINTS_FOR_MEDRXIV_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_MEDRXIV";
pub const ENABLE_ERROR_PRINTS_FOR_REDDIT_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_REDDIT";
pub const ENABLE_ERROR_PRINTS_FOR_TWITTER_ENV_NAME: &str = "ENABLE_ERROR_PRINTS_FOR_TWITTER";

pub const ENABLE_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_ARXIV";
pub const ENABLE_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_BIORXIV";
pub const ENABLE_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_GITHUB";
pub const ENABLE_SUCCESS_PRINTS_FOR_HABR_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_HABR";
pub const ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV";
pub const ENABLE_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_REDDIT";
pub const ENABLE_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME: &str = "ENABLE_SUCCESS_PRINTS_FOR_TWITTER";

pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT";
pub const ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER";

pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT";
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER";

pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER";

pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT";
pub const ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER";

pub const ENABLE_TIME_MEASUREMENT_FOR_ARXIV_ENV_NAME: &str = "ENABLE_TIME_MEASUREMENT_FOR_ARXIV";
pub const ENABLE_TIME_MEASUREMENT_FOR_BIORXIV_ENV_NAME: &str =
    "ENABLE_TIME_MEASUREMENT_FOR_BIORXIV";
pub const ENABLE_TIME_MEASUREMENT_FOR_GITHUB_ENV_NAME: &str = "ENABLE_TIME_MEASUREMENT_FOR_GITHUB";
pub const ENABLE_TIME_MEASUREMENT_FOR_HABR_ENV_NAME: &str = "ENABLE_TIME_MEASUREMENT_FOR_HABR";
pub const ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV_ENV_NAME: &str =
    "ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV";
pub const ENABLE_TIME_MEASUREMENT_FOR_REDDIT_ENV_NAME: &str = "ENABLE_TIME_MEASUREMENT_FOR_REDDIT";
pub const ENABLE_TIME_MEASUREMENT_FOR_TWITTER_ENV_NAME: &str =
    "ENABLE_TIME_MEASUREMENT_FOR_TWITTER";

pub const ENABLE_INFO_FOR_ARXIV_ENV_NAME: &str = "ENABLE_INFO_FOR_ARXIV";
pub const ENABLE_INFO_FOR_BIORXIV_ENV_NAME: &str = "ENABLE_INFO_FOR_BIORXIV";
pub const ENABLE_INFO_FOR_GITHUB_ENV_NAME: &str = "ENABLE_INFO_FOR_GITHUB";
pub const ENABLE_INFO_FOR_HABR_ENV_NAME: &str = "ENABLE_INFO_FOR_HABR";
pub const ENABLE_INFO_FOR_MEDRXIV_ENV_NAME: &str = "ENABLE_INFO_FOR_MEDRXIV";
pub const ENABLE_INFO_FOR_REDDIT_ENV_NAME: &str = "ENABLE_INFO_FOR_REDDIT";
pub const ENABLE_INFO_FOR_TWITTER_ENV_NAME: &str = "ENABLE_INFO_FOR_TWITTER";

pub const ENABLE_LINKS_LIMIT_FOR_ARXIV_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_ARXIV";
pub const ENABLE_LINKS_LIMIT_FOR_BIORXIV_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_BIORXIV";
pub const ENABLE_LINKS_LIMIT_FOR_GITHUB_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_GITHUB";
pub const ENABLE_LINKS_LIMIT_FOR_HABR_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_HABR";
pub const ENABLE_LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_MEDRXIV";
pub const ENABLE_LINKS_LIMIT_FOR_REDDIT_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_REDDIT";
pub const ENABLE_LINKS_LIMIT_FOR_TWITTER_ENV_NAME: &str = "ENABLE_LINKS_LIMIT_FOR_TWITTER";

pub const ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO";
pub const ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO";
pub const ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO";
pub const ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO";
pub const ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO";
pub const ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO";
pub const ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO_ENV_NAME: &str =
    "ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO";

pub const LINKS_LIMIT_FOR_ARXIV_ENV_NAME: &str = "LINKS_LIMIT_FOR_ARXIV";
pub const LINKS_LIMIT_FOR_BIORXIV_ENV_NAME: &str = "LINKS_LIMIT_FOR_BIORXIV";
pub const LINKS_LIMIT_FOR_GITHUB_ENV_NAME: &str = "LINKS_LIMIT_FOR_GITHUB";
pub const LINKS_LIMIT_FOR_HABR_ENV_NAME: &str = "LINKS_LIMIT_FOR_HABR";
pub const LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME: &str = "LINKS_LIMIT_FOR_MEDRXIV";
pub const LINKS_LIMIT_FOR_REDDIT_ENV_NAME: &str = "LINKS_LIMIT_FOR_REDDIT";
pub const LINKS_LIMIT_FOR_TWITTER_ENV_NAME: &str = "LINKS_LIMIT_FOR_TWITTER";

pub const ERROR_RED_ENV_NAME: &str = "ERROR_RED";
pub const ERROR_GREEN_ENV_NAME: &str = "ERROR_GREEN";
pub const ERROR_BLUE_ENV_NAME: &str = "ERROR_BLUE";
pub const WARNING_HIGH_RED_ENV_NAME: &str = "WARNING_HIGH_RED";
pub const WARNING_HIGH_GREEN_ENV_NAME: &str = "WARNING_HIGH_GREEN";
pub const WARNING_HIGH_BLUE_ENV_NAME: &str = "WARNING_HIGH_BLUE";
pub const WARNING_LOW_RED_ENV_NAME: &str = "WARNING_LOW_RED";
pub const WARNING_LOW_GREEN_ENV_NAME: &str = "WARNING_LOW_GREEN";
pub const WARNING_LOW_BLUE_ENV_NAME: &str = "WARNING_LOW_BLUE";
pub const SUCCESS_RED_ENV_NAME: &str = "SUCCESS_RED";
pub const SUCCESS_GREEN_ENV_NAME: &str = "SUCCESS_GREEN";
pub const SUCCESS_BLUE_ENV_NAME: &str = "SUCCESS_BLUE";
pub const PARTIAL_SUCCESS_RED_ENV_NAME: &str = "PARTIAL_SUCCESS_RED";
pub const PARTIAL_SUCCESS_GREEN_ENV_NAME: &str = "PARTIAL_SUCCESS_GREEN";
pub const PARTIAL_SUCCESS_BLUE_ENV_NAME: &str = "PARTIAL_SUCCESS_BLUE";
pub const CLEANING_RED_ENV_NAME: &str = "CLEANING_RED";
pub const CLEANING_GREEN_ENV_NAME: &str = "CLEANING_GREEN";
pub const CLEANING_BLUE_ENV_NAME: &str = "CLEANING_BLUE";
pub const TIME_MEASUREMENT_RED_ENV_NAME: &str = "TIME_MEASUREMENT_RED";
pub const TIME_MEASUREMENT_GREEN_ENV_NAME: &str = "TIME_MEASUREMENT_GREEN";
pub const TIME_MEASUREMENT_BLUE_ENV_NAME: &str = "TIME_MEASUREMENT_BLUE";
pub const INFO_RED_ENV_NAME: &str = "INFO_RED";
pub const INFO_GREEN_ENV_NAME: &str = "INFO_GREEN";
pub const INFO_BLUE_ENV_NAME: &str = "INFO_BLUE";