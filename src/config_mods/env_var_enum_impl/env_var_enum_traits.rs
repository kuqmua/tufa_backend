use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::env_var_trait::EnvVarTrait;

impl EnvVarTrait for EnvVar {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_env_name(&self) -> &'static str {
        match self {
            Self::GithubName => "GITHUB_NAME",
            Self::GithubToken => "GITHUB_TOKEN",

            Self::RedditUserAgent => "REDDIT_USER_AGENT",
            Self::RedditClientId => "REDDIT_CLIENT_ID",
            Self::RedditClientSecret => "REDDIT_CLIENT_SECRET",
            Self::RedditUsername => "REDDIT_USERNAME",
            Self::RedditPassword => "REDDIT_PASSWORD",
            Self::StartingCheckLink => "STARTING_CHECK_LINK",
            Self::WarningLogsDirectoryName => "WARNING_LOGS_DIRECTORY_NAME",
            Self::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir => {
                "UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR"
            }
            Self::EnableProviders => "ENABLE_PROVIDERS",
            Self::EnableCleaningWarningLogsDirectory => "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY",
            Self::EnableCleaningWarningLogsDbInMongo => "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO",
            Self::EnableCleaningWarningLogsDbCollectionsInMongo => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO"
            }
            Self::EnableTimeMeasurement => "ENABLE_TIME_MEASUREMENT",
            Self::EnableProviderLinksLimit => "ENABLE_PROVIDER_LINKS_LIMIT",
            Self::EnableCommonProvidersLinksLimit => "ENABLE_COMMON_PROVIDERS_LINKS_LIMIT",
            Self::CommonProvidersLinksLimit => "COMMON_PROVIDERS_LINKS_LIMIT",
            Self::EnableRandomizeOrderForProvidersLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO"
            }
            Self::EnablePrints => "ENABLE_PRINTS",
            Self::EnableErrorPrints => "ENABLE_ERROR_PRINTS",
            Self::EnableWarningHighPrints => "ENABLE_WARNING_HIGH_PRINTS",
            Self::EnableWarningLowPrints => "ENABLE_WARNING_LOW_PRINTS",
            Self::EnableSuccessPrints => "ENABLE_SUCCESS_PRINTS",
            Self::EnablePartialSuccessPrints => "ENABLE_PARTIAL_SUCCESS_PRINTS",
            Self::EnableTimeMeasurementPrints => "ENABLE_TIME_MEASUREMENT_PRINTS",
            Self::EnableCleaningWarningLogsDirectoryPrints => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS"
            }
            Self::EnableInfoPrints => "ENABLE_INFO_PRINTS",
            Self::EnableAllProvidersPrints => "ENABLE_ALL_PROVIDERS_PRINTS",
            Self::EnableErrorPrintsForAllProviders => "ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS",
            Self::EnableWarningHighPrintsForAllProviders => {
                "ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS"
            }
            Self::EnableWarningLowPrintsForAllProviders => {
                "ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS"
            }
            Self::EnableSuccessPrintsForAllProviders => "ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS",
            Self::EnablePartialSuccessPrintsForAllProviders => {
                "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS"
            }
            Self::EnableTimeMeasurementPrintsForAllProviders => {
                "ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS"
            }
            Self::EnableCleaningWarningLogsDirectoryPrintsForAllProviders => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS"
            }
            Self::EnableInfoPrintsForAllProviders => "ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS",
            Self::EnableWriteErrorLogsInLocalFolder => "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER",
            Self::EnableWriteErrorLogsInMongo => "ENABLE_WRITE_ERROR_LOGS_IN_MONGO",
            Self::EnableInitializeMongoWithProvidersLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS"
            }

            Self::ProvidersDbNameHandle => "PROVIDERS_DB_NAME_HANDLE",
            Self::ProvidersDbCollectionHandleSecondPart => {
                "PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART"
            }
            Self::ProvidersDbCollectionDocumentFieldNameHandle => {
                "PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE"
            }
            Self::PathToProviderLinkPartsFolder => "PATH_TO_PROVIDER_LINK_PARTS_FOLDER",
            Self::LogFileExtension => "LOG_FILE_EXTENSION",
            Self::DbProvidersLogsNameHandle => "DB_PROVIDERS_LOGS_NAME_HANDLE",
            Self::DbProvidersLogsCollectionHandleSecondPart => {
                "DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART"
            }
            Self::DbProvidersLogsCollectionDocumentFieldNameHandle => {
                "DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE"
            }

            Self::EnableInitializeMongoWithArxivLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS"
            }
            Self::EnableInitializeMongoWithBiorxivLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS"
            }
            Self::EnableInitializeMongoWithGithubLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS"
            }
            Self::EnableInitializeMongoWithHabrLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS"
            }
            Self::EnableInitializeMongoWithMedrxivLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS"
            }
            Self::EnableInitializeMongoWithRedditLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS"
            }
            Self::EnableInitializeMongoWithTwitterLinkParts => {
                "ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS"
            }

            Self::MongoFirstHandleUrlPart => "MONGO_FIRST_HANDLE_URL_PART",
            Self::MongoSecondHandleUrlPart => "MONGO_SECOND_HANDLE_URL_PART",
            Self::MongoThirdHandleUrlPart => "MONGO_THIRD_HANDLE_URL_PART",
            Self::MongoFourthHandleUrlPart => "MONGO_FOURTH_HANDLE_URL_PART",
            Self::MongoFifthHandleUrlPart => "MONGO_FIFTH_HANDLE_URL_PART",

            Self::MongoLogin => "MONGO_LOGIN",
            Self::MongoPassword => "MONGO_PASSWORD",
            Self::MongoIp => "MONGO_IP",
            Self::MongoPort => "MONGO_PORT",
            Self::MongoParams => "MONGO_PARAMS",

            Self::PostgresFirstHandleUrlPart => "POSTGRES_FIRST_HANDLE_URL_PART",
            Self::PostgresSecondHandleUrlPart => "POSTGRES_SECOND_HANDLE_URL_PART",
            Self::PostgresThirdHandleUrlPart => "POSTGRES_THIRD_HANDLE_URL_PART",
            Self::PostgresFourthHandleUrlPart => "POSTGRES_FOURTH_HANDLE_URL_PART",
            Self::PostgresFifthHandleUrlPart => "POSTGRES_FIFTH_HANDLE_URL_PART",

            Self::PostgresLogin => "POSTGRES_LOGIN",
            Self::PostgresPassword => "POSTGRES_PASSWORD",
            Self::PostgresIp => "POSTGRES_IP",
            Self::PostgresPort => "POSTGRES_PORT",
            Self::PostgresDb => "POSTGRES_DB",

            Self::EnableArxiv => "ENABLE_ARXIV",
            Self::EnableBiorxiv => "ENABLE_BIORXIV",
            Self::EnableGithub => "ENABLE_GITHUB",
            Self::EnableHabr => "ENABLE_HABR",
            Self::EnableMedrxiv => "ENABLE_MEDRXIV",
            Self::EnableReddit => "ENABLE_REDDIT",
            Self::EnableTwitter => "ENABLE_TWITTER",

            Self::ArxivLink => "ARXIV_LINK",
            Self::BiorxivLink => "BIORXIV_LINK",
            Self::GithubLink => "GITHUB_LINK",
            Self::HabrLink => "HABR_LINK",
            Self::MedrxivLink => "MEDRXIV_LINK",
            Self::RedditLink => "REDDIT_LINK",
            Self::TwitterLink => "TWITTER_LINK",

            Self::EnablePrintsArxiv => "ENABLE_PRINTS_ARXIV",
            Self::EnablePrintsBiorxiv => "ENABLE_PRINTS_BIORXIV",
            Self::EnablePrintsGithub => "ENABLE_PRINTS_GITHUB",
            Self::EnablePrintsHabr => "ENABLE_PRINTS_HABR",
            Self::EnablePrintsMedrxiv => "ENABLE_PRINTS_MEDRXIV",
            Self::EnablePrintsReddit => "ENABLE_PRINTS_REDDIT",
            Self::EnablePrintsTwitter => "ENABLE_PRINTS_TWITTER",

            Self::EnableWarningHighPrintsForArxiv => "ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV",
            Self::EnableWarningHighPrintsForBiorxiv => "ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV",
            Self::EnableWarningHighPrintsForGithub => "ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB",
            Self::EnableWarningHighPrintsForHabr => "ENABLE_WARNING_HIGH_PRINTS_FOR_HABR",
            Self::EnableWarningHighPrintsForMedrxiv => "ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV",
            Self::EnableWarningHighPrintsForReddit => "ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT",
            Self::EnableWarningHighPrintsForTwitter => "ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER",

            Self::EnableWarningLowPrintsForArxiv => "ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV",
            Self::EnableWarningLowPrintsForBiorxiv => "ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV",
            Self::EnableWarningLowPrintsForGithub => "ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB",
            Self::EnableWarningLowPrintsForHabr => "ENABLE_WARNING_LOW_PRINTS_FOR_HABR",
            Self::EnableWarningLowPrintsForMedrxiv => "ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV",
            Self::EnableWarningLowPrintsForReddit => "ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT",
            Self::EnableWarningLowPrintsForTwitter => "ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER",

            Self::EnableErrorPrintsForArxiv => "ENABLE_ERROR_PRINTS_FOR_ARXIV",
            Self::EnableErrorPrintsForBiorxiv => "ENABLE_ERROR_PRINTS_FOR_BIORXIV",
            Self::EnableErrorPrintsForGithub => "ENABLE_ERROR_PRINTS_FOR_GITHUB",
            Self::EnableErrorPrintsForHabr => "ENABLE_ERROR_PRINTS_FOR_HABR",
            Self::EnableErrorPrintsForMedrxiv => "ENABLE_ERROR_PRINTS_FOR_MEDRXIV",
            Self::EnableErrorPrintsForReddit => "ENABLE_ERROR_PRINTS_FOR_REDDIT",
            Self::EnableErrorPrintsForTwitter => "ENABLE_ERROR_PRINTS_FOR_TWITTER",

            Self::EnableSuccessPrintsForArxiv => "ENABLE_SUCCESS_PRINTS_FOR_ARXIV",
            Self::EnableSuccessPrintsForBiorxiv => "ENABLE_SUCCESS_PRINTS_FOR_BIORXIV",
            Self::EnableSuccessPrintsForGithub => "ENABLE_SUCCESS_PRINTS_FOR_GITHUB",
            Self::EnableSuccessPrintsForHabr => "ENABLE_SUCCESS_PRINTS_FOR_HABR",
            Self::EnableSuccessPrintsForMedrxiv => "ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV",
            Self::EnableSuccessPrintsForReddit => "ENABLE_SUCCESS_PRINTS_FOR_REDDIT",
            Self::EnableSuccessPrintsForTwitter => "ENABLE_SUCCESS_PRINTS_FOR_TWITTER",

            Self::EnablePartialSuccessPrintsForArxiv => "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV",
            Self::EnablePartialSuccessPrintsForBiorxiv => {
                "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV"
            }
            Self::EnablePartialSuccessPrintsForGithub => "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB",
            Self::EnablePartialSuccessPrintsForHabr => "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR",
            Self::EnablePartialSuccessPrintsForMedrxiv => {
                "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV"
            }
            Self::EnablePartialSuccessPrintsForReddit => "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT",
            Self::EnablePartialSuccessPrintsForTwitter => {
                "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER"
            }

            Self::EnableCleaningWarningLogsDirectoryForArxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV"
            }
            Self::EnableCleaningWarningLogsDirectoryForBiorxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV"
            }
            Self::EnableCleaningWarningLogsDirectoryForGithub => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB"
            }
            Self::EnableCleaningWarningLogsDirectoryForHabr => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR"
            }
            Self::EnableCleaningWarningLogsDirectoryForMedrxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV"
            }
            Self::EnableCleaningWarningLogsDirectoryForReddit => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT"
            }
            Self::EnableCleaningWarningLogsDirectoryForTwitter => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER"
            }

            Self::EnableCleaningWarningLogsDbInMongoForArxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV"
            }
            Self::EnableCleaningWarningLogsDbInMongoForBiorxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV"
            }
            Self::EnableCleaningWarningLogsDbInMongoForGithub => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB"
            }
            Self::EnableCleaningWarningLogsDbInMongoForHabr => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR"
            }
            Self::EnableCleaningWarningLogsDbInMongoForMedrxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV"
            }
            Self::EnableCleaningWarningLogsDbInMongoForReddit => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT"
            }
            Self::EnableCleaningWarningLogsDbInMongoForTwitter => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER"
            }

            Self::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV"
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV"
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongoForGithub => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB"
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongoForHabr => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR"
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV"
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongoForReddit => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT"
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter => {
                "ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER"
            }

            Self::EnableTimeMeasurementForArxiv => "ENABLE_TIME_MEASUREMENT_FOR_ARXIV",
            Self::EnableTimeMeasurementForBiorxiv => "ENABLE_TIME_MEASUREMENT_FOR_BIORXIV",
            Self::EnableTimeMeasurementForGithub => "ENABLE_TIME_MEASUREMENT_FOR_GITHUB",
            Self::EnableTimeMeasurementForHabr => "ENABLE_TIME_MEASUREMENT_FOR_HABR",
            Self::EnableTimeMeasurementForMedrxiv => "ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV",
            Self::EnableTimeMeasurementForReddit => "ENABLE_TIME_MEASUREMENT_FOR_REDDIT",
            Self::EnableTimeMeasurementForTwitter => "ENABLE_TIME_MEASUREMENT_FOR_TWITTER",

            Self::EnableInfoForArxiv => "ENABLE_INFO_FOR_ARXIV",
            Self::EnableInfoForBiorxiv => "ENABLE_INFO_FOR_BIORXIV",
            Self::EnableInfoForGithub => "ENABLE_INFO_FOR_GITHUB",
            Self::EnableInfoForHabr => "ENABLE_INFO_FOR_HABR",
            Self::EnableInfoForMedrxiv => "ENABLE_INFO_FOR_MEDRXIV",
            Self::EnableInfoForReddit => "ENABLE_INFO_FOR_REDDIT",
            Self::EnableInfoForTwitter => "ENABLE_INFO_FOR_TWITTER",

            Self::EnableLinksLimitForArxiv => "ENABLE_LINKS_LIMIT_FOR_ARXIV",
            Self::EnableLinksLimitForBiorxiv => "ENABLE_LINKS_LIMIT_FOR_BIORXIV",
            Self::EnableLinksLimitForGithub => "ENABLE_LINKS_LIMIT_FOR_GITHUB",
            Self::EnableLinksLimitForHabr => "ENABLE_LINKS_LIMIT_FOR_HABR",
            Self::EnableLinksLimitForMedrxiv => "ENABLE_LINKS_LIMIT_FOR_MEDRXIV",
            Self::EnableLinksLimitForReddit => "ENABLE_LINKS_LIMIT_FOR_REDDIT",
            Self::EnableLinksLimitForTwitter => "ENABLE_LINKS_LIMIT_FOR_TWITTER",

            Self::EnableRandomizeOrderForArxivLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO"
            }
            Self::EnableRandomizeOrderForBiorxivLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO"
            }
            Self::EnableRandomizeOrderForGithubLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO"
            }
            Self::EnableRandomizeOrderForHabrLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO"
            }
            Self::EnableRandomizeOrderForMedrxivLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO"
            }
            Self::EnableRandomizeOrderForRedditLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO"
            }
            Self::EnableRandomizeOrderForTwitterLinkPartsForMongo => {
                "ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO"
            }

            Self::LinksLimitForArxiv => "LINKS_LIMIT_FOR_ARXIV",
            Self::LinksLimitForBiorxiv => "LINKS_LIMIT_FOR_BIORXIV",
            Self::LinksLimitForGithub => "LINKS_LIMIT_FOR_GITHUB",
            Self::LinksLimitForHabr => "LINKS_LIMIT_FOR_HABR",
            Self::LinksLimitForMedrxiv => "LINKS_LIMIT_FOR_MEDRXIV",
            Self::LinksLimitForReddit => "LINKS_LIMIT_FOR_REDDIT",
            Self::LinksLimitForTwitter => "LINKS_LIMIT_FOR_TWITTER",

            Self::ErrorRed => "ERROR_RED",
            Self::ErrorGreen => "ERROR_GREEN",
            Self::ErrorBlue => "ERROR_BLUE",
            Self::WarningHighRed => "WARNING_HIGH_RED",
            Self::WarningHighGreen => "WARNING_HIGH_GREEN",
            Self::WarningHighBlue => "WARNING_HIGH_BLUE",
            Self::WarningLowRed => "WARNING_LOW_RED",
            Self::WarningLowGreen => "WARNING_LOW_GREEN",
            Self::WarningLowBlue => "WARNING_LOW_BLUE",
            Self::SuccessRed => "SUCCESS_RED",
            Self::SuccessGreen => "SUCCESS_GREEN",
            Self::SuccessBlue => "SUCCESS_BLUE",
            Self::PartialSuccessRed => "PARTIAL_SUCCESS_RED",
            Self::PartialSuccessGreen => "PARTIAL_SUCCESS_GREEN",
            Self::PartialSuccessBlue => "PARTIAL_SUCCESS_BLUE",
            Self::CleaningRed => "CLEANING_RED",
            Self::CleaningGreen => "CLEANING_GREEN",
            Self::CleaningBlue => "CLEANING_BLUE",
            Self::TimeMeasurementRed => "TIME_MEASUREMENT_RED",
            Self::TimeMeasurementGreen => "TIME_MEASUREMENT_GREEN",
            Self::TimeMeasurementBlue => "TIME_MEASUREMENT_BLUE",
            Self::InfoRed => "INFO_RED",
            Self::InfoGreen => "INFO_GREEN",
            Self::InfoBlue => "INFO_BLUE",
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_array() -> &'static [Self] {
        Self::all_variants()
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, Self> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<&'static str, Self> =
            HashMap::with_capacity(Self::get_length());
        for env_var_name_kind_kind in Self::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(
                env_var_name_kind_kind.get_env_name(),
                env_var_name_kind_kind,
            );
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, Self)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(Self::get_length());
        for env_var_name_kind in Self::iter() {
            env_var_name_kind_vec.push((env_var_name_kind.get_env_name(), env_var_name_kind));
        }
        env_var_name_kind_vec
    }
}
