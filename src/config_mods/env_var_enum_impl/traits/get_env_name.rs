use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::get_env_name_trait::GetEnvName;

impl GetEnvName for EnvVar {
    fn get_env_name(self: &EnvVar) -> &'static str {
        match self {
            EnvVar::GithubName => "GITHUB_NAME",
            EnvVar::GithubToken => "GITHUB_TOKEN",

            EnvVar::RedditUserAgent => "REDDIT_USER_AGENT",
            EnvVar::RedditClientId => "REDDIT_CLIENT_ID",
            EnvVar::RedditClientSecret => "REDDIT_CLIENT_SECRET",
            EnvVar::RedditUsername => "REDDIT_USERNAME",
            EnvVar::RedditPassword => "REDDIT_PASSWORD",
            EnvVar::StartingCheckLink => "STARTING_CHECK_LINK",
            EnvVar::WarningLogsDirectoryName => "WARNING_LOGS_DIRECTORY_NAME",
            EnvVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir => {
                "UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR"
            }
            EnvVar::EnableProviders => "ENABLE_PROVIDERS",
            EnvVar::EnableCleaningWarningLogsDirectory => "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY",
            EnvVar::EnableCleaningWarningLogsDbInMongo => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO"
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongo => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO"
            }
            EnvVar::EnableTimeMeasurement => "ENABLE_TIME_MEASUREMENT",
            EnvVar::EnableProviderLinksLimit => "ENABLE_PROVIDER_LINKS_LIMIT",
            EnvVar::EnableCommonProvidersLinksLimit => "ENABLE_COMMON_PROVIDERS_LINKS_LIMIT",
            EnvVar::CommonProvidersLinksLimit => "COMMON_PROVIDERS_LINKS_LIMIT",
            EnvVar::EnableRandomizeOrderForProvidersLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO"
            }
            EnvVar::EnablePrints => "ENABLE_PRINTS",
            EnvVar::EnableErrorPrints => "ENABLE_ERROR_PRINTS",
            EnvVar::EnableWarningHighPrints => "ENABLE_WARNING_HIGH_PRINTS",
            EnvVar::EnableWarningLowPrints => "ENABLE_WARNING_LOW_PRINTS",
            EnvVar::EnableSuccessPrints => "ENABLE_SUCCESS_PRINTS",
            EnvVar::EnablePartialSuccessPrints => "ENABLE_PARTIAL_SUCCESS_PRINTS",
            EnvVar::EnableTimeMeasurementPrints => "ENABLE_TIME_MEASUREMENT_PRINTS",
            EnvVar::EnableCleaningWarningLogsDirectoryPrints => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS"
            }
            EnvVar::EnableInfoPrints => "ENABLE_INFO_PRINTS",
            EnvVar::EnableAllProvidersPrints => "ENABLE_ALL_PROVIDERS_PRINTS",
            EnvVar::EnableErrorPrintsForAllProviders => "ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS",
            EnvVar::EnableWarningHighPrintsForAllProviders => {
                "ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS"
            }
            EnvVar::EnableWarningLowPrintsForAllProviders => {
                "ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS"
            }
            EnvVar::EnableSuccessPrintsForAllProviders => "ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS",
            EnvVar::EnablePartialSuccessPrintsForAllProviders => {
                "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS"
            }
            EnvVar::EnableTimeMeasurementPrintsForAllProviders => {
                "ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS"
            }
            EnvVar::EnableCleaningWarningLogsDirectoryPrintsForAllProviders => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS"
            }
            EnvVar::EnableInfoPrintsForAllProviders => "ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS",
            EnvVar::EnableWriteErrorLogsInLocalFolder => "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER",
            EnvVar::EnableWriteErrorLogsInMongo => "ENABLE_WRITE_ERROR_LOGS_IN_MONGO",
            EnvVar::EnableInitializeMongoWithProvidersLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS"
            }

            EnvVar::ProvidersDbNameHandle => "PROVIDERS_DB_NAME_HANDLE",
            EnvVar::ProvidersDbCollectionHandleSecondPart => {
                "PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART"
            }
            EnvVar::ProvidersDbCollectionDocumentFieldNameHandle => {
                "PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE"
            }
            EnvVar::PathToProviderLinkPartsFolder => "PATH_TO_PROVIDER_LINK_PARTS_FOLDER",
            EnvVar::LogFileExtension => "LOG_FILE_EXTENSION",
            EnvVar::DbProvidersLogsNameHandle => "DB_PROVIDERS_LOGS_NAME_HANDLE",
            EnvVar::DbProvidersLogsCollectionHandleSecondPart => {
                "DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART"
            }
            EnvVar::DbProvidersLogsCollectionDocumentFieldNameHandle => {
                "DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE"
            }

            EnvVar::EnableInitializeMongoWithArxivLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS"
            }
            EnvVar::EnableInitializeMongoWithBiorxivLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS"
            }
            EnvVar::EnableInitializeMongoWithGithubLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS"
            }
            EnvVar::EnableInitializeMongoWithHabrLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS"
            }
            EnvVar::EnableInitializeMongoWithMedrxivLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS"
            }
            EnvVar::EnableInitializeMongoWithRedditLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS"
            }
            EnvVar::EnableInitializeMongoWithTwitterLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS"
            }

            EnvVar::MongoFirstHandleUrlPart => "MONGO_FIRST_HANDLE_URL_PART",
            EnvVar::MongoSecondHandleUrlPart => "MONGO_SECOND_HANDLE_URL_PART",
            EnvVar::MongoThirdHandleUrlPart => "MONGO_THIRD_HANDLE_URL_PART",
            EnvVar::MongoFourthHandleUrlPart => "MONGO_FOURTH_HANDLE_URL_PART",
            EnvVar::MongoFifthHandleUrlPart => "MONGO_FIFTH_HANDLE_URL_PART",

            EnvVar::MongoLogin => "MONGO_LOGIN",
            EnvVar::MongoPassword => "MONGO_PASSWORD",
            EnvVar::MongoIp => "MONGO_IP",
            EnvVar::MongoPort => "MONGO_PORT",
            EnvVar::MongoParams => "MONGO_PARAMS",

            EnvVar::PostgresFirstHandleUrlPart => "POSTGRES_FIRST_HANDLE_URL_PART",
            EnvVar::PostgresSecondHandleUrlPart => "POSTGRES_SECOND_HANDLE_URL_PART",
            EnvVar::PostgresThirdHandleUrlPart => "POSTGRES_THIRD_HANDLE_URL_PART",
            EnvVar::PostgresFourthHandleUrlPart => "POSTGRES_FOURTH_HANDLE_URL_PART",
            EnvVar::PostgresFifthHandleUrlPart => "POSTGRES_FIFTH_HANDLE_URL_PART",

            EnvVar::PostgresLogin => "POSTGRES_LOGIN",
            EnvVar::PostgresPassword => "POSTGRES_PASSWORD",
            EnvVar::PostgresIp => "POSTGRES_IP",
            EnvVar::PostgresPort => "POSTGRES_PORT",
            EnvVar::PostgresDb => "POSTGRES_DB",

            EnvVar::EnableArxiv => "ENABLE_ARXIV",
            EnvVar::EnableBiorxiv => "ENABLE_BIORXIV",
            EnvVar::EnableGithub => "ENABLE_GITHUB",
            EnvVar::EnableHabr => "ENABLE_HABR",
            EnvVar::EnableMedrxiv => "ENABLE_MEDRXIV",
            EnvVar::EnableReddit => "ENABLE_REDDIT",
            EnvVar::EnableTwitter => "ENABLE_TWITTER",

            EnvVar::ArxivLink => "ARXIV_LINK",
            EnvVar::BiorxivLink => "BIORXIV_LINK",
            EnvVar::GithubLink => "GITHUB_LINK",
            EnvVar::HabrLink => "HABR_LINK",
            EnvVar::MedrxivLink => "MEDRXIV_LINK",
            EnvVar::RedditLink => "REDDIT_LINK",
            EnvVar::TwitterLink => "TWITTER_LINK",

            EnvVar::EnablePrintsArxiv => "ENABLE_PRINTS_ARXIV",
            EnvVar::EnablePrintsBiorxiv => "ENABLE_PRINTS_BIORXIV",
            EnvVar::EnablePrintsGithub => "ENABLE_PRINTS_GITHUB",
            EnvVar::EnablePrintsHabr => "ENABLE_PRINTS_HABR",
            EnvVar::EnablePrintsMedrxiv => "ENABLE_PRINTS_MEDRXIV",
            EnvVar::EnablePrintsReddit => "ENABLE_PRINTS_REDDIT",
            EnvVar::EnablePrintsTwitter => "ENABLE_PRINTS_TWITTER",

            EnvVar::EnableWarningHighPrintsForArxiv => "ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV",
            EnvVar::EnableWarningHighPrintsForBiorxiv => "ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV",
            EnvVar::EnableWarningHighPrintsForGithub => "ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB",
            EnvVar::EnableWarningHighPrintsForHabr => "ENABLE_WARNING_HIGH_PRINTS_FOR_HABR",
            EnvVar::EnableWarningHighPrintsForMedrxiv => "ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV",
            EnvVar::EnableWarningHighPrintsForReddit => "ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT",
            EnvVar::EnableWarningHighPrintsForTwitter => "ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER",

            EnvVar::EnableWarningLowPrintsForArxiv => "ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV",
            EnvVar::EnableWarningLowPrintsForBiorxiv => "ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV",
            EnvVar::EnableWarningLowPrintsForGithub => "ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB",
            EnvVar::EnableWarningLowPrintsForHabr => "ENABLE_WARNING_LOW_PRINTS_FOR_HABR",
            EnvVar::EnableWarningLowPrintsForMedrxiv => "ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV",
            EnvVar::EnableWarningLowPrintsForReddit => "ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT",
            EnvVar::EnableWarningLowPrintsForTwitter => "ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER",

            EnvVar::EnableErrorPrintsForArxiv => "ENABLE_ERROR_PRINTS_FOR_ARXIV",
            EnvVar::EnableErrorPrintsForBiorxiv => "ENABLE_ERROR_PRINTS_FOR_BIORXIV",
            EnvVar::EnableErrorPrintsForGithub => "ENABLE_ERROR_PRINTS_FOR_GITHUB",
            EnvVar::EnableErrorPrintsForHabr => "ENABLE_ERROR_PRINTS_FOR_HABR",
            EnvVar::EnableErrorPrintsForMedrxiv => "ENABLE_ERROR_PRINTS_FOR_MEDRXIV",
            EnvVar::EnableErrorPrintsForReddit => "ENABLE_ERROR_PRINTS_FOR_REDDIT",
            EnvVar::EnableErrorPrintsForTwitter => "ENABLE_ERROR_PRINTS_FOR_TWITTER",

            EnvVar::EnableSuccessPrintsForArxiv => "ENABLE_SUCCESS_PRINTS_FOR_ARXIV",
            EnvVar::EnableSuccessPrintsForBiorxiv => "ENABLE_SUCCESS_PRINTS_FOR_BIORXIV",
            EnvVar::EnableSuccessPrintsForGithub => "ENABLE_SUCCESS_PRINTS_FOR_GITHUB",
            EnvVar::EnableSuccessPrintsForHabr => "ENABLE_SUCCESS_PRINTS_FOR_HABR",
            EnvVar::EnableSuccessPrintsForMedrxiv => "ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV",
            EnvVar::EnableSuccessPrintsForReddit => "ENABLE_SUCCESS_PRINTS_FOR_REDDIT",
            EnvVar::EnableSuccessPrintsForTwitter => "ENABLE_SUCCESS_PRINTS_FOR_TWITTER",

            EnvVar::EnablePartialSuccessPrintsForArxiv => "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV",
            EnvVar::EnablePartialSuccessPrintsForBiorxiv => {
                "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV"
            }
            EnvVar::EnablePartialSuccessPrintsForGithub => {
                "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB"
            }
            EnvVar::EnablePartialSuccessPrintsForHabr => "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR",
            EnvVar::EnablePartialSuccessPrintsForMedrxiv => {
                "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV"
            }
            EnvVar::EnablePartialSuccessPrintsForReddit => {
                "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT"
            }
            EnvVar::EnablePartialSuccessPrintsForTwitter => {
                "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER"
            }

            EnvVar::EnableCleaningWarningLogsDirectoryForArxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV"
            }
            EnvVar::EnableCleaningWarningLogsDirectoryForBiorxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV"
            }
            EnvVar::EnableCleaningWarningLogsDirectoryForGithub => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB"
            }
            EnvVar::EnableCleaningWarningLogsDirectoryForHabr => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR"
            }
            EnvVar::EnableCleaningWarningLogsDirectoryForMedrxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV"
            }
            EnvVar::EnableCleaningWarningLogsDirectoryForReddit => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT"
            }
            EnvVar::EnableCleaningWarningLogsDirectoryForTwitter => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER"
            }

            EnvVar::EnableCleaningWarningLogsDbInMongoForArxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV"
            }
            EnvVar::EnableCleaningWarningLogsDbInMongoForBiorxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV"
            }
            EnvVar::EnableCleaningWarningLogsDbInMongoForGithub => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB"
            }
            EnvVar::EnableCleaningWarningLogsDbInMongoForHabr => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR"
            }
            EnvVar::EnableCleaningWarningLogsDbInMongoForMedrxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV"
            }
            EnvVar::EnableCleaningWarningLogsDbInMongoForReddit => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT"
            }
            EnvVar::EnableCleaningWarningLogsDbInMongoForTwitter => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER"
            }

            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV"
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV"
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForGithub => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB"
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForHabr => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR"
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV"
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForReddit => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT"
            }
            EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER"
            }

            EnvVar::EnableTimeMeasurementForArxiv => "ENABLE_TIME_MEASUREMENT_FOR_ARXIV",
            EnvVar::EnableTimeMeasurementForBiorxiv => "ENABLE_TIME_MEASUREMENT_FOR_BIORXIV",
            EnvVar::EnableTimeMeasurementForGithub => "ENABLE_TIME_MEASUREMENT_FOR_GITHUB",
            EnvVar::EnableTimeMeasurementForHabr => "ENABLE_TIME_MEASUREMENT_FOR_HABR",
            EnvVar::EnableTimeMeasurementForMedrxiv => "ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV",
            EnvVar::EnableTimeMeasurementForReddit => "ENABLE_TIME_MEASUREMENT_FOR_REDDIT",
            EnvVar::EnableTimeMeasurementForTwitter => "ENABLE_TIME_MEASUREMENT_FOR_TWITTER",

            EnvVar::EnableInfoForArxiv => "ENABLE_INFO_FOR_ARXIV",
            EnvVar::EnableInfoForBiorxiv => "ENABLE_INFO_FOR_BIORXIV",
            EnvVar::EnableInfoForGithub => "ENABLE_INFO_FOR_GITHUB",
            EnvVar::EnableInfoForHabr => "ENABLE_INFO_FOR_HABR",
            EnvVar::EnableInfoForMedrxiv => "ENABLE_INFO_FOR_MEDRXIV",
            EnvVar::EnableInfoForReddit => "ENABLE_INFO_FOR_REDDIT",
            EnvVar::EnableInfoForTwitter => "ENABLE_INFO_FOR_TWITTER",

            EnvVar::EnableLinksLimitForArxiv => "ENABLE_LINKS_LIMIT_FOR_ARXIV",
            EnvVar::EnableLinksLimitForBiorxiv => "ENABLE_LINKS_LIMIT_FOR_BIORXIV",
            EnvVar::EnableLinksLimitForGithub => "ENABLE_LINKS_LIMIT_FOR_GITHUB",
            EnvVar::EnableLinksLimitForHabr => "ENABLE_LINKS_LIMIT_FOR_HABR",
            EnvVar::EnableLinksLimitForMedrxiv => "ENABLE_LINKS_LIMIT_FOR_MEDRXIV",
            EnvVar::EnableLinksLimitForReddit => "ENABLE_LINKS_LIMIT_FOR_REDDIT",
            EnvVar::EnableLinksLimitForTwitter => "ENABLE_LINKS_LIMIT_FOR_TWITTER",

            EnvVar::EnableRandomizeOrderForArxivLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO"
            }
            EnvVar::EnableRandomizeOrderForBiorxivLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO"
            }
            EnvVar::EnableRandomizeOrderForGithubLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO"
            }
            EnvVar::EnableRandomizeOrderForHabrLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO"
            }
            EnvVar::EnableRandomizeOrderForMedrxivLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO"
            }
            EnvVar::EnableRandomizeOrderForRedditLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO"
            }
            EnvVar::EnableRandomizeOrderForTwitterLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO"
            }

            EnvVar::LinksLimitForArxiv => "LINKS_LIMIT_FOR_ARXIV",
            EnvVar::LinksLimitForBiorxiv => "LINKS_LIMIT_FOR_BIORXIV",
            EnvVar::LinksLimitForGithub => "LINKS_LIMIT_FOR_GITHUB",
            EnvVar::LinksLimitForHabr => "LINKS_LIMIT_FOR_HABR",
            EnvVar::LinksLimitForMedrxiv => "LINKS_LIMIT_FOR_MEDRXIV",
            EnvVar::LinksLimitForReddit => "LINKS_LIMIT_FOR_REDDIT",
            EnvVar::LinksLimitForTwitter => "LINKS_LIMIT_FOR_TWITTER",

            EnvVar::ErrorRed => "ERROR_RED",
            EnvVar::ErrorGreen => "ERROR_GREEN",
            EnvVar::ErrorBlue => "ERROR_BLUE",
            EnvVar::WarningHighRed => "WARNING_HIGH_RED",
            EnvVar::WarningHighGreen => "WARNING_HIGH_GREEN",
            EnvVar::WarningHighBlue => "WARNING_HIGH_BLUE",
            EnvVar::WarningLowRed => "WARNING_LOW_RED",
            EnvVar::WarningLowGreen => "WARNING_LOW_GREEN",
            EnvVar::WarningLowBlue => "WARNING_LOW_BLUE",
            EnvVar::SuccessRed => "SUCCESS_RED",
            EnvVar::SuccessGreen => "SUCCESS_GREEN",
            EnvVar::SuccessBlue => "SUCCESS_BLUE",
            EnvVar::PartialSuccessRed => "PARTIAL_SUCCESS_RED",
            EnvVar::PartialSuccessGreen => "PARTIAL_SUCCESS_GREEN",
            EnvVar::PartialSuccessBlue => "PARTIAL_SUCCESS_BLUE",
            EnvVar::CleaningRed => "CLEANING_RED",
            EnvVar::CleaningGreen => "CLEANING_GREEN",
            EnvVar::CleaningBlue => "CLEANING_BLUE",
            EnvVar::TimeMeasurementRed => "TIME_MEASUREMENT_RED",
            EnvVar::TimeMeasurementGreen => "TIME_MEASUREMENT_GREEN",
            EnvVar::TimeMeasurementBlue => "TIME_MEASUREMENT_BLUE",
            EnvVar::InfoRed => "INFO_RED",
            EnvVar::InfoGreen => "INFO_GREEN",
            EnvVar::InfoBlue => "INFO_BLUE",
        }
    }
}
