use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::env_var_trait::EnvVarTrait;

impl EnvVarTrait for EnvVar {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_env_name(&self) -> &'static str {
        match &self {
            Self::GithubName => "GITHUB_NAME",
            Self::GithubToken => "GITHUB_TOKEN",

            Self::RedditUserAgent => "REDDIT_USER_AGENT",
            Self::RedditClientId => "REDDIT_CLIENT_ID",
            Self::RedditClientSecret => "REDDIT_CLIENT_SECRET",
            Self::RedditUsername => "REDDIT_USERNAME",
            Self::RedditPassword => "REDDIT_PASSWORD",

            Self::DbsEnableInitialization => "DBS_ENABLE_INITIALIZATION",

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

            Self::MongoProvidersLogsDbName => "MONGO_PROVIDERS_LOGS_DB_NAME",
            Self::MongoProvidersLogsDbCollectionHandleSecondPart => {
                "MONGO_PROVIDERS_LOGS_DB_COLLECTION_HANDLE_SECOND_PART"
            }
            Self::MongoProvidersLogsDbCollectionDocumentFieldNameHandle => {
                "MONGO_PROVIDERS_LOGS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE"
            }

            Self::MongoEnableInitialization => "MONGO_ENABLE_INITIALIZATION",
            Self::MongoEnableInitializationForProviders => {
                "MONGO_ENABLE_INITIALIZATION_FOR_PROVIDERS"
            }
            Self::MongoEnableInitializationForArxiv => "MONGO_ENABLE_INITIALIZATION_FOR_ARXIV",
            Self::MongoEnableInitializationForBiorxiv => "MONGO_ENABLE_INITIALIZATION_FOR_BIORXIV",
            Self::MongoEnableInitializationForGithub => "MONGO_ENABLE_INITIALIZATION_FOR_GITHUB",
            Self::MongoEnableInitializationForHabr => "MONGO_ENABLE_INITIALIZATION_FOR_HABR",
            Self::MongoEnableInitializationForMedrxiv => "MONGO_ENABLE_INITIALIZATION_FOR_MEDRXIV",
            Self::MongoEnableInitializationForReddit => "MONGO_ENABLE_INITIALIZATION_FOR_REDDIT",
            Self::MongoEnableInitializationForTwitter => "MONGO_ENABLE_INITIALIZATION_FOR_TWITTER",

            Self::MongoEnableWriteErrorLogs => "MONGO_ENABLE_WRITE_ERROR_LOGS",
            Self::MongoEnableWriteErrorLogsForProviders => {
                "MONGO_ENABLE_WRITE_ERROR_LOGS_FOR_PROVIDERS"
            }
            Self::MongoEnableWriteErrorLogsForArxiv => "MONGO_ENABLE_WRITE_ERROR_LOGS_FOR_ARXIV",
            Self::MongoEnableWriteErrorLogsForBiorxiv => {
                "MONGO_ENABLE_WRITE_ERROR_LOGS_FOR_BIORXIV"
            }
            Self::MongoEnableWriteErrorLogsForGithub => "MONGO_ENABLE_WRITE_ERROR_LOGS_FOR_GITHUB",
            Self::MongoEnableWriteErrorLogsForHabr => "MONGO_ENABLE_WRITE_ERROR_LOGS_FOR_HABR",
            Self::MongoEnableWriteErrorLogsForMedrxiv => {
                "MONGO_ENABLE_WRITE_ERROR_LOGS_FOR_MEDRXIV"
            }
            Self::MongoEnableWriteErrorLogsForReddit => "MONGO_ENABLE_WRITE_ERROR_LOGS_FOR_REDDIT",
            Self::MongoEnableWriteErrorLogsForTwitter => {
                "MONGO_ENABLE_WRITE_ERROR_LOGS_FOR_TWITTER"
            }

            Self::MongoEnableCleaningWarningLogsDb => "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB",
            Self::MongoEnableCleaningWarningLogsDbForProviders => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_FOR_PROVIDERS"
            }
            Self::MongoEnableCleaningWarningLogsDbForArxiv => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_FOR_ARXIV"
            }
            Self::MongoEnableCleaningWarningLogsDbForBiorxiv => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_FOR_BIORXIV"
            }
            Self::MongoEnableCleaningWarningLogsDbForGithub => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_FOR_GITHUB"
            }
            Self::MongoEnableCleaningWarningLogsDbForHabr => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_FOR_HABR"
            }
            Self::MongoEnableCleaningWarningLogsDbForMedrxiv => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_FOR_MEDRXIV"
            }
            Self::MongoEnableCleaningWarningLogsDbForReddit => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_FOR_REDDIT"
            }
            Self::MongoEnableCleaningWarningLogsDbForTwitter => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_FOR_TWITTER"
            }

            Self::MongoEnableCleaningWarningLogsDbCollections => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS"
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForProviders => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_FOR_PROVIDERS"
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForArxiv => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_FOR_ARXIV"
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForBiorxiv => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_FOR_BIORXIV"
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForGithub => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_FOR_GITHUB"
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForHabr => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_FOR_HABR"
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForMedrxiv => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_FOR_MEDRXIV"
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForReddit => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_FOR_REDDIT"
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForTwitter => {
                "MONGO_ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_FOR_TWITTER"
            }

            Self::MongoEnableLinkPartsRandomizeOrder => "MONGO_ENABLE_LINK_PARTS_RANDOMIZE_ORDER",
            Self::MongoEnableLinkPartsRandomizeOrderForProviders => {
                "MONGO_ENABLE_LINK_PARTS_RANDOMIZE_ORDER_FOR_PROVIDERS"
            }
            Self::MongoEnableLinkPartsRandomizeOrderForArxiv => {
                "MONGO_ENABLE_LINK_PARTS_RANDOMIZE_ORDER_FOR_ARXIV"
            }
            Self::MongoEnableLinkPartsRandomizeOrderForBiorxiv => {
                "MONGO_ENABLE_LINK_PARTS_RANDOMIZE_ORDER_FOR_BIORXIV"
            }
            Self::MongoEnableLinkPartsRandomizeOrderForGithub => {
                "MONGO_ENABLE_LINK_PARTS_RANDOMIZE_ORDER_FOR_GITHUB"
            }
            Self::MongoEnableLinkPartsRandomizeOrderForHabr => {
                "MONGO_ENABLE_LINK_PARTS_RANDOMIZE_ORDER_FOR_HABR"
            }
            Self::MongoEnableLinkPartsRandomizeOrderForMedrxiv => {
                "MONGO_ENABLE_LINK_PARTS_RANDOMIZE_ORDER_FOR_MEDRXIV"
            }
            Self::MongoEnableLinkPartsRandomizeOrderForReddit => {
                "MONGO_ENABLE_LINK_PARTS_RANDOMIZE_ORDER_FOR_REDDIT"
            }
            Self::MongoEnableLinkPartsRandomizeOrderForTwitter => {
                "MONGO_ENABLE_LINK_PARTS_RANDOMIZE_ORDER_FOR_TWITTER"
            }

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

            Self::PostgresEnableInitialization => "POSTGRES_ENABLE_INITIALIZATION",
            Self::PostgresEnableInitializationForProviders => {
                "POSTGRES_ENABLE_INITIALIZATION_FOR_PROVIDERS"
            }
            Self::PostgresEnableInitializationForArxiv => {
                "POSTGRES_ENABLE_INITIALIZATION_FOR_ARXIV"
            }
            Self::PostgresEnableInitializationForBiorxiv => {
                "POSTGRES_ENABLE_INITIALIZATION_FOR_BIORXIV"
            }
            Self::PostgresEnableInitializationForGithub => {
                "POSTGRES_ENABLE_INITIALIZATION_FOR_GITHUB"
            }
            Self::PostgresEnableInitializationForHabr => "POSTGRES_ENABLE_INITIALIZATION_FOR_HABR",
            Self::PostgresEnableInitializationForMedrxiv => {
                "POSTGRES_ENABLE_INITIALIZATION_FOR_MEDRXIV"
            }
            Self::PostgresEnableInitializationForReddit => {
                "POSTGRES_ENABLE_INITIALIZATION_FOR_REDDIT"
            }
            Self::PostgresEnableInitializationForTwitter => {
                "POSTGRES_ENABLE_INITIALIZATION_FOR_TWITTER"
            }

            Self::WarningLogsDirectoryName => "WARNING_LOGS_DIRECTORY_NAME",
            Self::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir => {
                "UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR"
            }
            Self::PathToProviderLinkPartsFolder => "PATH_TO_PROVIDER_LINK_PARTS_FOLDER",
            Self::LogFileExtension => "LOG_FILE_EXTENSION",

            Self::EnableWriteErrorLogsInLocalFolder => "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER",
            Self::EnableWriteErrorLogsInLocalFolderForProvider => {
                "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_FOR_PROVIDERS"
            }
            Self::EnableWriteErrorLogsInLocalFolderForArxiv => {
                "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_FOR_ARXIV"
            }
            Self::EnableWriteErrorLogsInLocalFolderForBiorxiv => {
                "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_FOR_BIORXIV"
            }
            Self::EnableWriteErrorLogsInLocalFolderForGithub => {
                "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_FOR_GITHUB"
            }
            Self::EnableWriteErrorLogsInLocalFolderForHabr => {
                "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_FOR_HABR"
            }
            Self::EnableWriteErrorLogsInLocalFolderForMedrxiv => {
                "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_FOR_MEDRXIV"
            }
            Self::EnableWriteErrorLogsInLocalFolderForReddit => {
                "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_FOR_REDDIT"
            }
            Self::EnableWriteErrorLogsInLocalFolderForTwitter => {
                "ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER_FOR_TWITTER"
            }

            Self::EnableCleaningWarningLogsDirectory => "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY",
            Self::EnableCleaningWarningLogsDirectoryForProviders => {
                "ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_PROVIDERS"
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

            Self::StartingCheckLink => "STARTING_CHECK_LINK",
            Self::ArxivCheckLink => "ARXIV_CHECK_LINK",
            Self::BiorxivCheckLink => "BIORXIV_CHECK_LINK",
            Self::GithubCheckLink => "GITHUB_CHECK_LINK",
            Self::HabrCheckLink => "HABR_CHECK_LINK",
            Self::MedrxivCheckLink => "MEDRXIV_CHECK_LINK",
            Self::RedditCheckLink => "REDDIT_CHECK_LINK",
            Self::TwitterCheckLink => "TWITTER_CHECK_LINK",

            Self::EnableProviders => "ENABLE_PROVIDERS",
            Self::EnableArxiv => "ENABLE_ARXIV",
            Self::EnableBiorxiv => "ENABLE_BIORXIV",
            Self::EnableGithub => "ENABLE_GITHUB",
            Self::EnableHabr => "ENABLE_HABR",
            Self::EnableMedrxiv => "ENABLE_MEDRXIV",
            Self::EnableReddit => "ENABLE_REDDIT",
            Self::EnableTwitter => "ENABLE_TWITTER",

            Self::EnablePrints => "ENABLE_PRINTS",
            Self::EnablePrintsForProviders => "ENABLE_PRINTS_FOR_PROVIDERS",
            Self::EnablePrintsArxiv => "ENABLE_PRINTS_ARXIV",
            Self::EnablePrintsBiorxiv => "ENABLE_PRINTS_BIORXIV",
            Self::EnablePrintsGithub => "ENABLE_PRINTS_GITHUB",
            Self::EnablePrintsHabr => "ENABLE_PRINTS_HABR",
            Self::EnablePrintsMedrxiv => "ENABLE_PRINTS_MEDRXIV",
            Self::EnablePrintsReddit => "ENABLE_PRINTS_REDDIT",
            Self::EnablePrintsTwitter => "ENABLE_PRINTS_TWITTER",

            Self::EnableWarningHighPrints => "ENABLE_WARNING_HIGH_PRINTS",
            Self::EnableWarningHighPrintsForProviders => "ENABLE_WARNING_HIGH_PRINTS_FOR_PROVIDERS",
            Self::EnableWarningHighPrintsForArxiv => "ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV",
            Self::EnableWarningHighPrintsForBiorxiv => "ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV",
            Self::EnableWarningHighPrintsForGithub => "ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB",
            Self::EnableWarningHighPrintsForHabr => "ENABLE_WARNING_HIGH_PRINTS_FOR_HABR",
            Self::EnableWarningHighPrintsForMedrxiv => "ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV",
            Self::EnableWarningHighPrintsForReddit => "ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT",
            Self::EnableWarningHighPrintsForTwitter => "ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER",

            Self::EnableWarningLowPrints => "ENABLE_WARNING_LOW_PRINTS",
            Self::EnableWarningLowPrintsForProviders => "ENABLE_WARNING_LOW_PRINTS_FOR_PROVIDERS",
            Self::EnableWarningLowPrintsForArxiv => "ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV",
            Self::EnableWarningLowPrintsForBiorxiv => "ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV",
            Self::EnableWarningLowPrintsForGithub => "ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB",
            Self::EnableWarningLowPrintsForHabr => "ENABLE_WARNING_LOW_PRINTS_FOR_HABR",
            Self::EnableWarningLowPrintsForMedrxiv => "ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV",
            Self::EnableWarningLowPrintsForReddit => "ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT",
            Self::EnableWarningLowPrintsForTwitter => "ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER",

            Self::EnableSuccessPrints => "ENABLE_SUCCESS_PRINTS",
            Self::EnableSuccessPrintsForProviders => "ENABLE_SUCCESS_PRINTS_FOR_PROVIDERS",
            Self::EnableSuccessPrintsForArxiv => "ENABLE_SUCCESS_PRINTS_FOR_ARXIV",
            Self::EnableSuccessPrintsForBiorxiv => "ENABLE_SUCCESS_PRINTS_FOR_BIORXIV",
            Self::EnableSuccessPrintsForGithub => "ENABLE_SUCCESS_PRINTS_FOR_GITHUB",
            Self::EnableSuccessPrintsForHabr => "ENABLE_SUCCESS_PRINTS_FOR_HABR",
            Self::EnableSuccessPrintsForMedrxiv => "ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV",
            Self::EnableSuccessPrintsForReddit => "ENABLE_SUCCESS_PRINTS_FOR_REDDIT",
            Self::EnableSuccessPrintsForTwitter => "ENABLE_SUCCESS_PRINTS_FOR_TWITTER",

            Self::EnablePartialSuccessPrints => "ENABLE_PARTIAL_SUCCESS_PRINTS",
            Self::EnablePartialSuccessPrintsForProviders => {
                "ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_PROVIDERS"
            }
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

            Self::EnableErrorPrints => "ENABLE_ERROR_PRINTS",
            Self::EnableErrorPrintsForProviders => "ENABLE_ERROR_PRINTS_FOR_PROVIDERS",
            Self::EnableErrorPrintsForArxiv => "ENABLE_ERROR_PRINTS_FOR_ARXIV",
            Self::EnableErrorPrintsForBiorxiv => "ENABLE_ERROR_PRINTS_FOR_BIORXIV",
            Self::EnableErrorPrintsForGithub => "ENABLE_ERROR_PRINTS_FOR_GITHUB",
            Self::EnableErrorPrintsForHabr => "ENABLE_ERROR_PRINTS_FOR_HABR",
            Self::EnableErrorPrintsForMedrxiv => "ENABLE_ERROR_PRINTS_FOR_MEDRXIV",
            Self::EnableErrorPrintsForReddit => "ENABLE_ERROR_PRINTS_FOR_REDDIT",
            Self::EnableErrorPrintsForTwitter => "ENABLE_ERROR_PRINTS_FOR_TWITTER",

            Self::EnableTimeMeasurementPrints => "ENABLE_TIME_MEASUREMENT_PRINTS",
            Self::EnableTimeMeasurementPrintsForProviders => {
                "ENABLE_TIME_MEASUREMENT_PRINTS_FOR_PROVIDERS"
            }
            Self::EnableTimeMeasurementPrintsForArxiv => "ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ARXIV",
            Self::EnableTimeMeasurementPrintsForBiorxiv => {
                "ENABLE_TIME_MEASUREMENT_PRINTS_FOR_BIORXIV"
            }
            Self::EnableTimeMeasurementPrintsForGithub => {
                "ENABLE_TIME_MEASUREMENT_PRINTS_FOR_GITHUB"
            }
            Self::EnableTimeMeasurementPrintsForHabr => "ENABLE_TIME_MEASUREMENT_PRINTS_FOR_HABR",
            Self::EnableTimeMeasurementPrintsForMedrxiv => {
                "ENABLE_TIME_MEASUREMENT_PRINTS_FOR_MEDRXIV"
            }
            Self::EnableTimeMeasurementPrintsForReddit => {
                "ENABLE_TIME_MEASUREMENT_PRINTS_FOR_REDDIT"
            }
            Self::EnableTimeMeasurementPrintsForTwitter => {
                "ENABLE_TIME_MEASUREMENT_PRINTS_FOR_TWITTER"
            }

            Self::EnableInfoPrints => "ENABLE_INFO_PRINTS",
            Self::EnableInfoPrintsForProviders => "ENABLE_INFO_PRINTS_FOR_PROVIDERS",
            Self::EnableInfoPrintsForArxiv => "ENABLE_INFO_PRINTS_FOR_ARXIV",
            Self::EnableInfoPrintsForBiorxiv => "ENABLE_INFO_PRINTS_FOR_BIORXIV",
            Self::EnableInfoPrintsForGithub => "ENABLE_INFO_PRINTS_FOR_GITHUB",
            Self::EnableInfoPrintsForHabr => "ENABLE_INFO_PRINTS_FOR_HABR",
            Self::EnableInfoPrintsForMedrxiv => "ENABLE_INFO_PRINTS_FOR_MEDRXIV",
            Self::EnableInfoPrintsForReddit => "ENABLE_INFO_PRINTS_FOR_REDDIT",
            Self::EnableInfoPrintsForTwitter => "ENABLE_INFO_PRINTS_FOR_TWITTER",

            Self::EnableLinksLimit => "ENABLE_LINKS_LIMIT",
            Self::EnableLinksLimitForProviders => "ENABLE_LINKS_LIMIT_FOR_PROVIDERS",
            Self::EnableLinksLimitForArxiv => "ENABLE_LINKS_LIMIT_FOR_ARXIV",
            Self::EnableLinksLimitForBiorxiv => "ENABLE_LINKS_LIMIT_FOR_BIORXIV",
            Self::EnableLinksLimitForGithub => "ENABLE_LINKS_LIMIT_FOR_GITHUB",
            Self::EnableLinksLimitForHabr => "ENABLE_LINKS_LIMIT_FOR_HABR",
            Self::EnableLinksLimitForMedrxiv => "ENABLE_LINKS_LIMIT_FOR_MEDRXIV",
            Self::EnableLinksLimitForReddit => "ENABLE_LINKS_LIMIT_FOR_REDDIT",
            Self::EnableLinksLimitForTwitter => "ENABLE_LINKS_LIMIT_FOR_TWITTER",

            Self::EnableCommonProvidersLinksLimit => "ENABLE_COMMON_PROVIDERS_LINKS_LIMIT",
            Self::CommonProvidersLinksLimit => "COMMON_PROVIDERS_LINKS_LIMIT",
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
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_vec() -> Vec<Self> {
        let mut env_var_name_kind_vec = Vec::with_capacity(Self::get_length());
        for env_var_name_kind in Self::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
}
