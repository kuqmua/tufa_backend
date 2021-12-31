extern crate toml;

use crate::config_mods::config_error_mods::config_env_var_error_type_enum::ConfigEnvVarErrorType;
use crate::config_mods::config_error_mods::config_error::ConfigError;
use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;
use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;
use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

use crate::config_mods::config_struct::ConfigStruct;
use crate::traits::enum_extention::EnumExtenstion;
use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

use crate::helpers::resource::Resource;

use dotenv::dotenv;

impl ConfigStruct {
    pub fn new() -> Result<Self, ConfigError> {
        let mut was_dotenv_enable = false;
        if dotenv().is_ok() {
            was_dotenv_enable = true;
        }
        let string_vars = EnvStringVar::get_env_values_hashmap::<String>(was_dotenv_enable)?;
        let bool_vars = EnvBoolVar::get_env_values_hashmap::<bool>(was_dotenv_enable)?;
        let u8_vars = EnvU8Var::get_env_values_hashmap::<u8>(was_dotenv_enable)?;
        let i64_vars = EnvI64Var::get_env_values_hashmap::<i64>(was_dotenv_enable)?;
        let providers_link_parts_source_handle: Resource;
        if string_vars[&EnvStringVar::ProvidersLinkPartsSource] == "local" {
            providers_link_parts_source_handle = Resource::Local;
        } else if string_vars[&EnvStringVar::ProvidersLinkPartsSource] == "mongo" {
            providers_link_parts_source_handle = Resource::Mongodb;
        } else if string_vars[&EnvStringVar::ProvidersLinkPartsSource] == "postgres" {
            providers_link_parts_source_handle = Resource::PostgreSql;
        } else {
            return Err(ConfigError {
                env_var_name_kind: ConfigEnvVarErrorType::EnvStringVar(
                    EnvStringVar::ProvidersLinkPartsSource,
                ),
                was_dotenv_enable: false, //its incorrect hardcode, todo
                env_name: EnvStringVar::ProvidersLinkPartsSource.to_upper_snake_case(),
            });
        }
        let handle_config: ConfigStruct = ConfigStruct {
            github_name: string_vars[&EnvStringVar::GithubName].clone(),
            github_token: string_vars[&EnvStringVar::GithubToken].clone(),

            reddit_user_agent: string_vars[&EnvStringVar::RedditUserAgent].clone(),
            reddit_client_id: string_vars[&EnvStringVar::RedditClientId].clone(),
            reddit_client_secret: string_vars[&EnvStringVar::RedditClientSecret].clone(),
            reddit_username: string_vars[&EnvStringVar::RedditUsername].clone(),
            reddit_password: string_vars[&EnvStringVar::RedditPassword].clone(),

            dbs_enable_initialization: bool_vars[&EnvBoolVar::DbsEnableInitialization],
            providers_link_parts_source: providers_link_parts_source_handle,

            mongo_first_handle_url_part: string_vars[&EnvStringVar::MongoFirstHandleUrlPart]
                .clone(),
            mongo_second_handle_url_part: string_vars[&EnvStringVar::MongoSecondHandleUrlPart]
                .clone(),
            mongo_third_handle_url_part: string_vars[&EnvStringVar::MongoThirdHandleUrlPart]
                .clone(),
            mongo_fourth_handle_url_part: string_vars[&EnvStringVar::MongoFourthHandleUrlPart]
                .clone(),
            mongo_fifth_handle_url_part: string_vars[&EnvStringVar::MongoFifthHandleUrlPart]
                .clone(),

            mongo_login: string_vars[&EnvStringVar::MongoLogin].clone(),
            mongo_password: string_vars[&EnvStringVar::MongoPassword].clone(),
            mongo_ip: string_vars[&EnvStringVar::MongoIp].clone(),
            mongo_port: string_vars[&EnvStringVar::MongoPort].clone(),
            mongo_params: string_vars[&EnvStringVar::MongoParams].clone(),

            mongo_providers_logs_db_name: string_vars[&EnvStringVar::MongoProvidersLogsDbName]
                .clone(),
            mongo_providers_logs_db_collection_handle_second_part: string_vars
                [&EnvStringVar::MongoProvidersLogsDbCollectionHandleSecondPart]
                .clone(),
            mongo_providers_logs_db_collection_document_field_name_handle: string_vars
                [&EnvStringVar::MongoProvidersLogsDbCollectionDocumentFieldNameHandle]
                .clone(),

            is_mongo_initialization_enabled: bool_vars[&EnvBoolVar::DbsEnableInitialization]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabled],
            is_mongo_initialization_enabled_providers: bool_vars
                [&EnvBoolVar::IsMongoInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledProviders],

            is_mongo_initialization_enabled_arxiv: bool_vars
                [&EnvBoolVar::IsMongoInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledArxiv],
            is_mongo_initialization_enabled_biorxiv: bool_vars
                [&EnvBoolVar::IsMongoInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledBiorxiv],
            is_mongo_initialization_enabled_github: bool_vars
                [&EnvBoolVar::IsMongoInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledGithub],
            is_mongo_initialization_enabled_habr: bool_vars[&EnvBoolVar::IsMongoInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledHabr],
            is_mongo_initialization_enabled_medrxiv: bool_vars
                [&EnvBoolVar::IsMongoInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledMedrxiv],
            is_mongo_initialization_enabled_reddit: bool_vars
                [&EnvBoolVar::IsMongoInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledReddit],
            is_mongo_initialization_enabled_twitter: bool_vars
                [&EnvBoolVar::IsMongoInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoInitializationEnabledTwitter],

            is_mongo_write_error_logs_enabled: bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabled],
            is_mongo_write_error_logs_enabled_providers: bool_vars
                [&EnvBoolVar::IsMongoWriteErrorLogsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledProviders],
            is_mongo_write_error_logs_enabled_arxiv: bool_vars
                [&EnvBoolVar::IsMongoWriteErrorLogsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledArxiv],
            is_mongo_write_error_logs_enabled_biorxiv: bool_vars
                [&EnvBoolVar::IsMongoWriteErrorLogsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledBiorxiv],
            is_mongo_write_error_logs_enabled_github: bool_vars
                [&EnvBoolVar::IsMongoWriteErrorLogsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledGithub],
            is_mongo_write_error_logs_enabled_habr: bool_vars
                [&EnvBoolVar::IsMongoWriteErrorLogsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledHabr],
            is_mongo_write_error_logs_enabled_medrxiv: bool_vars
                [&EnvBoolVar::IsMongoWriteErrorLogsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledMedrxiv],
            is_mongo_write_error_logs_enabled_reddit: bool_vars
                [&EnvBoolVar::IsMongoWriteErrorLogsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledReddit],
            is_mongo_write_error_logs_enabled_twitter: bool_vars
                [&EnvBoolVar::IsMongoWriteErrorLogsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoWriteErrorLogsEnabledTwitter],

            is_mongo_cleaning_warning_logs_db_enabled: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabled],
            is_mongo_cleaning_warning_logs_db_enabled_providers: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledProviders],
            is_mongo_cleaning_warning_logs_db_enabled_arxiv: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledArxiv],
            is_mongo_cleaning_warning_logs_db_enabled_biorxiv: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledBiorxiv],
            is_mongo_cleaning_warning_logs_db_enabled_github: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledGithub],
            is_mongo_cleaning_warning_logs_db_enabled_habr: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledHabr],
            is_mongo_cleaning_warning_logs_db_enabled_medrxiv: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledMedrxiv],
            is_mongo_cleaning_warning_logs_db_enabled_reddit: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledReddit],
            is_mongo_cleaning_warning_logs_db_enabled_twitter: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbEnabledTwitter],

            is_mongo_cleaning_warning_logs_db_collections_enabled: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabled],
            is_mongo_cleaning_warning_logs_db_collections_enabled_providers: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledProviders],
            is_mongo_cleaning_warning_logs_db_collections_enabled_arxiv: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledArxiv],
            is_mongo_cleaning_warning_logs_db_collections_enabled_biorxiv: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledBiorxiv],
            is_mongo_cleaning_warning_logs_db_collections_enabled_github: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledGithub],
            is_mongo_cleaning_warning_logs_db_collections_enabled_habr: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledHabr],
            is_mongo_cleaning_warning_logs_db_collections_enabled_medrxiv: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledMedrxiv],
            is_mongo_cleaning_warning_logs_db_collections_enabled_reddit: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledReddit],
            is_mongo_cleaning_warning_logs_db_collections_enabled_twitter: bool_vars
                [&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabled]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoCleaningWarningLogsDbCollectionsEnabledTwitter],

            is_mongo_link_parts_randomize_order_enabled: bool_vars
                [&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabled],
            is_mongo_link_parts_randomize_order_enabled_providers: bool_vars
                [&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabled]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledProviders],
            is_mongo_link_parts_randomize_order_enabled_arxiv: bool_vars
                [&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabled]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledArxiv],
            is_mongo_link_parts_randomize_order_enabled_biorxiv: bool_vars
                [&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabled]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledBiorxiv],
            is_mongo_link_parts_randomize_order_enabled_github: bool_vars
                [&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabled]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledGithub],
            is_mongo_link_parts_randomize_order_enabled_habr: bool_vars
                [&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabled]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledHabr],
            is_mongo_link_parts_randomize_order_enabled_medrxiv: bool_vars
                [&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabled]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledMedrxiv],
            is_mongo_link_parts_randomize_order_enabled_reddit: bool_vars
                [&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabled]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledReddit],
            is_mongo_link_parts_randomize_order_enabled_twitter: bool_vars
                [&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabled]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsMongoLinkPartsRandomizeOrderEnabledTwitter],

            postgres_first_handle_url_part: string_vars[&EnvStringVar::PostgresFirstHandleUrlPart]
                .clone(),
            postgres_second_handle_url_part: string_vars
                [&EnvStringVar::PostgresSecondHandleUrlPart]
                .clone(),
            postgres_third_handle_url_part: string_vars[&EnvStringVar::PostgresThirdHandleUrlPart]
                .clone(),
            postgres_fourth_handle_url_part: string_vars
                [&EnvStringVar::PostgresFourthHandleUrlPart]
                .clone(),
            postgres_fifth_handle_url_part: string_vars[&EnvStringVar::PostgresFifthHandleUrlPart]
                .clone(),

            postgres_login: string_vars[&EnvStringVar::PostgresLogin].clone(),
            postgres_password: string_vars[&EnvStringVar::PostgresPassword].clone(),
            postgres_ip: string_vars[&EnvStringVar::PostgresIp].clone(),
            postgres_port: string_vars[&EnvStringVar::PostgresPort].clone(),
            postgres_db: string_vars[&EnvStringVar::PostgresDb].clone(),

            is_postgres_initialization_enabled: bool_vars[&EnvBoolVar::DbsEnableInitialization]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabled],
            is_postgres_initialization_enabled_providers: bool_vars
                [&EnvBoolVar::IsPostgresInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledProviders],
            is_postgres_initialization_enabled_arxiv: bool_vars
                [&EnvBoolVar::IsPostgresInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledArxiv],
            is_postgres_initialization_enabled_biorxiv: bool_vars
                [&EnvBoolVar::IsPostgresInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledBiorxiv],
            is_postgres_initialization_enabled_github: bool_vars
                [&EnvBoolVar::IsPostgresInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledGithub],
            is_postgres_initialization_enabled_habr: bool_vars
                [&EnvBoolVar::IsPostgresInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledHabr],
            is_postgres_initialization_enabled_medrxiv: bool_vars
                [&EnvBoolVar::IsPostgresInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledArxiv],
            is_postgres_initialization_enabled_reddit: bool_vars
                [&EnvBoolVar::IsPostgresInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledReddit],
            is_postgres_initialization_enabled_twitter: bool_vars
                [&EnvBoolVar::IsPostgresInitializationEnabled]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPostgresInitializationEnabledTwitter],

            warning_logs_directory_name: string_vars[&EnvStringVar::WarningLogsDirectoryName]
                .clone(),
            unhandled_success_handled_success_are_there_items_initialized_posts_dir: string_vars
                [&EnvStringVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir]
                .clone(),
            path_to_provider_link_parts_folder: string_vars
                [&EnvStringVar::PathToProviderLinkPartsFolder]
                .clone(),
            log_file_extension: string_vars[&EnvStringVar::LogFileExtension].clone(),

            is_write_error_logs_in_local_folder_enabled: bool_vars
                [&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabled],
            is_write_error_logs_in_local_folder_enabled_provider: bool_vars
                [&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabled]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledProviders],
            is_write_error_logs_in_local_folder_enabled_arxiv: bool_vars
                [&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabled]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledArxiv],
            is_write_error_logs_in_local_folder_enabled_biorxiv: bool_vars
                [&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabled]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledBiorxiv],
            is_write_error_logs_in_local_folder_enabled_github: bool_vars
                [&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabled]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledGithub],
            is_write_error_logs_in_local_folder_enabled_habr: bool_vars
                [&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabled]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledHabr],
            is_write_error_logs_in_local_folder_enabled_medrxiv: bool_vars
                [&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabled]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledMedrxiv],
            is_write_error_logs_in_local_folder_enabled_reddit: bool_vars
                [&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabled]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledReddit],
            is_write_error_logs_in_local_folder_enabled_twitter: bool_vars
                [&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabled]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledProviders]
                && bool_vars[&EnvBoolVar::IsWriteErrorLogsInLocalFolderEnabledTwitter],

            is_cleaning_warning_logs_directory_enabled: bool_vars
                [&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabled],
            is_cleaning_warning_logs_directory_enabled_providers: bool_vars
                [&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabled]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledProviders],
            is_cleaning_warning_logs_directory_enabled_arxiv: bool_vars
                [&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabled]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledProviders]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledArxiv],
            is_cleaning_warning_logs_directory_enabled_biorxiv: bool_vars
                [&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabled]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledProviders]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledBiorxiv],
            is_cleaning_warning_logs_directory_enabled_github: bool_vars
                [&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabled]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledProviders]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledGithub],
            is_cleaning_warning_logs_directory_enabled_habr: bool_vars
                [&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabled]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledProviders]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledHabr],
            is_cleaning_warning_logs_directory_enabled_medrxiv: bool_vars
                [&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabled]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledProviders]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledMedrxiv],
            is_cleaning_warning_logs_directory_enabled_reddit: bool_vars
                [&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabled]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledProviders]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledReddit],
            is_cleaning_warning_logs_directory_enabled_twitter: bool_vars
                [&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabled]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledProviders]
                && bool_vars[&EnvBoolVar::IsCleaningWarningLogsDirectoryEnabledTwitter],

            starting_check_link: string_vars[&EnvStringVar::StartingCheckLink].clone(),
            check_link_arxiv: string_vars[&EnvStringVar::CheckLinkArxiv].clone(),
            check_link_biorxiv: string_vars[&EnvStringVar::CheckLinkBiorxiv].clone(),
            check_link_github: string_vars[&EnvStringVar::CheckLinkGithub].clone(),
            check_link_habr: string_vars[&EnvStringVar::CheckLinkHabr].clone(),
            check_link_medrxiv: string_vars[&EnvStringVar::CheckLinkMedrxiv].clone(),
            check_link_reddit: string_vars[&EnvStringVar::CheckLinkReddit].clone(),
            check_link_twitter: string_vars[&EnvStringVar::CheckLinkTwitter].clone(),

            is_enabled_providers: bool_vars[&EnvBoolVar::IsEnabledProviders],
            is_enabled_arxiv: bool_vars[&EnvBoolVar::IsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsEnabledArxiv],
            is_enabled_biorxiv: bool_vars[&EnvBoolVar::IsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsEnabledBiorxiv],
            is_enabled_github: bool_vars[&EnvBoolVar::IsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsEnabledGithub],
            is_enabled_habr: bool_vars[&EnvBoolVar::IsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsEnabledHabr],
            is_enabled_medrxiv: bool_vars[&EnvBoolVar::IsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsEnabledMedrxiv],
            is_enabled_reddit: bool_vars[&EnvBoolVar::IsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsEnabledReddit],
            is_enabled_twitter: bool_vars[&EnvBoolVar::IsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsEnabledTwitter],

            enable_prints: bool_vars[&EnvBoolVar::IsPrintsEnabled],
            is_prints_enabled_for_providers: bool_vars[&EnvBoolVar::IsPrintsEnabled]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledProviders],
            is_prints_enabled_arxiv: bool_vars[&EnvBoolVar::IsPrintsEnabled]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledArxiv],
            is_prints_enabled_biorxiv: bool_vars[&EnvBoolVar::IsPrintsEnabled]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledBiorxiv],
            is_prints_enabled_github: bool_vars[&EnvBoolVar::IsPrintsEnabled]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledGithub],
            is_prints_enabled_habr: bool_vars[&EnvBoolVar::IsPrintsEnabled]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledHabr],
            is_prints_enabled_medrxiv: bool_vars[&EnvBoolVar::IsPrintsEnabled]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledMedrxiv],
            is_prints_enabled_reddit: bool_vars[&EnvBoolVar::IsPrintsEnabled]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledReddit],
            is_prints_enabled_twitter: bool_vars[&EnvBoolVar::IsPrintsEnabled]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledProviders]
                && bool_vars[&EnvBoolVar::IsPrintsEnabledTwitter],

            enable_warning_high_prints: bool_vars[&EnvBoolVar::EnableWarningHighPrints],
            enable_warning_high_prints_providers: bool_vars
                [&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsProviders],
            enable_warning_high_prints_arxiv: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsArxiv],
            enable_warning_high_prints_biorxiv: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsBiorxiv],
            enable_warning_high_prints_github: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsGithub],
            enable_warning_high_prints_habr: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsHabr],
            enable_warning_high_prints_medrxiv: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsMedrxiv],
            enable_warning_high_prints_reddit: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsReddit],
            enable_warning_high_prints_twitter: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsTwitter],

            enable_warning_low_prints: bool_vars[&EnvBoolVar::EnableWarningLowPrints],
            enable_warning_low_prints_for_providers: bool_vars[&EnvBoolVar::EnableWarningLowPrints]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForProviders],
            enable_warning_low_prints_for_arxiv: bool_vars[&EnvBoolVar::EnableWarningLowPrints]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForArxiv],
            enable_warning_low_prints_for_biorxiv: bool_vars[&EnvBoolVar::EnableWarningLowPrints]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForBiorxiv],
            enable_warning_low_prints_for_github: bool_vars[&EnvBoolVar::EnableWarningLowPrints]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForGithub],
            enable_warning_low_prints_for_habr: bool_vars[&EnvBoolVar::EnableWarningLowPrints]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForHabr],
            enable_warning_low_prints_for_medrxiv: bool_vars[&EnvBoolVar::EnableWarningLowPrints]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForMedrxiv],
            enable_warning_low_prints_for_reddit: bool_vars[&EnvBoolVar::EnableWarningLowPrints]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForReddit],
            enable_warning_low_prints_for_twitter: bool_vars[&EnvBoolVar::EnableWarningLowPrints]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningLowPrintsForTwitter],

            enable_success_prints: bool_vars[&EnvBoolVar::EnableSuccessPrints],
            enable_success_prints_for_providers: bool_vars[&EnvBoolVar::EnableSuccessPrints]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForProviders],
            enable_success_prints_for_arxiv: bool_vars[&EnvBoolVar::EnableSuccessPrints]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForArxiv],
            enable_success_prints_for_biorxiv: bool_vars[&EnvBoolVar::EnableSuccessPrints]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForBiorxiv],
            enable_success_prints_for_github: bool_vars[&EnvBoolVar::EnableSuccessPrints]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForGithub],
            enable_success_prints_for_habr: bool_vars[&EnvBoolVar::EnableSuccessPrints]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForHabr],
            enable_success_prints_for_medrxiv: bool_vars[&EnvBoolVar::EnableSuccessPrints]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForMedrxiv],
            enable_success_prints_for_reddit: bool_vars[&EnvBoolVar::EnableSuccessPrints]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForReddit],
            enable_success_prints_for_twitter: bool_vars[&EnvBoolVar::EnableSuccessPrints]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableSuccessPrintsForTwitter],

            enable_partial_success_prints: bool_vars[&EnvBoolVar::EnablePartialSuccessPrints],
            enable_partial_success_prints_for_providers: bool_vars
                [&EnvBoolVar::EnablePartialSuccessPrints]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForProviders],
            enable_partial_success_prints_for_arxiv: bool_vars
                [&EnvBoolVar::EnablePartialSuccessPrints]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForArxiv],
            enable_partial_success_prints_for_biorxiv: bool_vars
                [&EnvBoolVar::EnablePartialSuccessPrints]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForBiorxiv],
            enable_partial_success_prints_for_github: bool_vars
                [&EnvBoolVar::EnablePartialSuccessPrints]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForGithub],
            enable_partial_success_prints_for_habr: bool_vars
                [&EnvBoolVar::EnablePartialSuccessPrints]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForHabr],
            enable_partial_success_prints_for_medrxiv: bool_vars
                [&EnvBoolVar::EnablePartialSuccessPrints]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForMedrxiv],
            enable_partial_success_prints_for_reddit: bool_vars
                [&EnvBoolVar::EnablePartialSuccessPrints]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForReddit],
            enable_partial_success_prints_for_twitter: bool_vars
                [&EnvBoolVar::EnablePartialSuccessPrints]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForTwitter],

            enable_error_prints: bool_vars[&EnvBoolVar::EnableErrorPrints],
            enable_error_prints_for_providers: bool_vars[&EnvBoolVar::EnableErrorPrints]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForProviders],
            enable_error_prints_for_arxiv: bool_vars[&EnvBoolVar::EnableErrorPrints]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForArxiv],
            enable_error_prints_for_biorxiv: bool_vars[&EnvBoolVar::EnableErrorPrints]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForBiorxiv],
            enable_error_prints_for_github: bool_vars[&EnvBoolVar::EnableErrorPrints]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForGithub],
            enable_error_prints_for_habr: bool_vars[&EnvBoolVar::EnableErrorPrints]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForHabr],
            enable_error_prints_for_medrxiv: bool_vars[&EnvBoolVar::EnableErrorPrints]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForMedrxiv],
            enable_error_prints_for_reddit: bool_vars[&EnvBoolVar::EnableErrorPrints]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForReddit],
            enable_error_prints_for_twitter: bool_vars[&EnvBoolVar::EnableErrorPrints]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableErrorPrintsForTwitter],

            enable_time_measurement_prints: bool_vars[&EnvBoolVar::EnableTimeMeasurementPrints],
            enable_time_measurement_prints_for_providers: bool_vars
                [&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders],
            enable_time_measurement_prints_for_arxiv: bool_vars
                [&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForArxiv],
            enable_time_measurement_prints_for_biorxiv: bool_vars
                [&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForBiorxiv],
            enable_time_measurement_prints_for_github: bool_vars
                [&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForGithub],
            enable_time_measurement_prints_for_habr: bool_vars
                [&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForHabr],
            enable_time_measurement_prints_for_medrxiv: bool_vars
                [&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForMedrxiv],
            enable_time_measurement_prints_for_reddit: bool_vars
                [&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForReddit],
            enable_time_measurement_prints_for_twitter: bool_vars
                [&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForTwitter],

            enable_info_prints: bool_vars[&EnvBoolVar::EnableInfoPrints],
            enable_info_prints_for_providers: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders],
            enable_info_prints_for_arxiv: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForArxiv],
            enable_info_prints_for_biorxiv: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForBiorxiv],
            enable_info_prints_for_github: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForGithub],
            enable_info_prints_for_habr: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForHabr],
            enable_info_prints_for_medrxiv: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForReddit],
            enable_info_prints_for_reddit: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForReddit],
            enable_info_prints_for_twitter: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForTwitter],

            enable_links_limit: bool_vars[&EnvBoolVar::EnableLinksLimit],
            enable_links_limit_for_providers: bool_vars[&EnvBoolVar::EnableLinksLimit]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForProviders],
            enable_links_limit_for_arxiv: bool_vars[&EnvBoolVar::EnableLinksLimit]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForProviders]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForArxiv],
            enable_links_limit_for_biorxiv: bool_vars[&EnvBoolVar::EnableLinksLimit]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForProviders]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForBiorxiv],
            enable_links_limit_for_github: bool_vars[&EnvBoolVar::EnableLinksLimit]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForProviders]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForGithub],
            enable_links_limit_for_habr: bool_vars[&EnvBoolVar::EnableLinksLimit]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForProviders]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForHabr],
            enable_links_limit_for_medrxiv: bool_vars[&EnvBoolVar::EnableLinksLimit]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForProviders]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForMedrxiv],
            enable_links_limit_for_reddit: bool_vars[&EnvBoolVar::EnableLinksLimit]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForProviders]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForReddit],
            enable_links_limit_for_twitter: bool_vars[&EnvBoolVar::EnableLinksLimit]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForProviders]
                && bool_vars[&EnvBoolVar::EnableLinksLimitForTwitter],

            enable_common_providers_links_limit: bool_vars
                [&EnvBoolVar::EnableCommonProvidersLinksLimit],
            common_providers_links_limit: i64_vars[&EnvI64Var::CommonProvidersLinksLimit],
            links_limit_for_arxiv: i64_vars[&EnvI64Var::LinksLimitForArxiv],
            links_limit_for_biorxiv: i64_vars[&EnvI64Var::LinksLimitForBiorxiv],
            links_limit_for_github: i64_vars[&EnvI64Var::LinksLimitForGithub],
            links_limit_for_habr: i64_vars[&EnvI64Var::LinksLimitForHabr],
            links_limit_for_medrxiv: i64_vars[&EnvI64Var::LinksLimitForMedrxiv],
            links_limit_for_reddit: i64_vars[&EnvI64Var::LinksLimitForReddit],
            links_limit_for_twitter: i64_vars[&EnvI64Var::LinksLimitForTwitter],

            error_red: u8_vars[&EnvU8Var::ErrorRed],
            error_green: u8_vars[&EnvU8Var::ErrorGreen],
            error_blue: u8_vars[&EnvU8Var::ErrorBlue],
            warning_high_red: u8_vars[&EnvU8Var::WarningHighRed],
            warning_high_green: u8_vars[&EnvU8Var::WarningHighGreen],
            warning_high_blue: u8_vars[&EnvU8Var::WarningHighBlue],
            warning_low_red: u8_vars[&EnvU8Var::WarningLowRed],
            warning_low_green: u8_vars[&EnvU8Var::WarningLowGreen],
            warning_low_blue: u8_vars[&EnvU8Var::WarningLowBlue],
            success_red: u8_vars[&EnvU8Var::SuccessRed],
            success_green: u8_vars[&EnvU8Var::SuccessGreen],
            success_blue: u8_vars[&EnvU8Var::SuccessBlue],
            partial_success_red: u8_vars[&EnvU8Var::PartialSuccessRed],
            partial_success_green: u8_vars[&EnvU8Var::PartialSuccessGreen],
            partial_success_blue: u8_vars[&EnvU8Var::PartialSuccessBlue],
            cleaning_red: u8_vars[&EnvU8Var::CleaningRed],
            cleaning_green: u8_vars[&EnvU8Var::CleaningGreen],
            cleaning_blue: u8_vars[&EnvU8Var::CleaningBlue],
            time_measurement_red: u8_vars[&EnvU8Var::TimeMeasurementRed],
            time_measurement_green: u8_vars[&EnvU8Var::TimeMeasurementGreen],
            time_measurement_blue: u8_vars[&EnvU8Var::TimeMeasurementBlue],
            info_red: u8_vars[&EnvU8Var::InfoRed],
            info_green: u8_vars[&EnvU8Var::InfoGreen],
            info_blue: u8_vars[&EnvU8Var::InfoBlue],
        };
        ConfigStruct::wrap_config_checks(handle_config)
    }
}
