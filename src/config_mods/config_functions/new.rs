extern crate toml;

use std::collections::HashMap;

use crate::config_mods::config_error_mods::config_error::ConfigError;
use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;
use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;
use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

use crate::config_mods::config_struct::ConfigStruct;
use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

impl ConfigStruct {
    pub fn new() -> Result<Self, ConfigError<'static>> {
        let string_vars = EnvStringVar::get_env_values_hashmap::<String>()?;
        let bool_vars: HashMap<EnvBoolVar, bool> = EnvBoolVar::get_env_values_hashmap()?;
        let u8_vars = EnvU8Var::get_env_values_hashmap()?;
        let i64_vars = EnvI64Var::get_env_values_hashmap()?;
        let handle_config: ConfigStruct = ConfigStruct {
            github_name: string_vars[&EnvStringVar::GithubName].clone(),
            github_token: string_vars[&EnvStringVar::GithubToken].clone(),

            reddit_user_agent: string_vars[&EnvStringVar::RedditUserAgent].clone(),
            reddit_client_id: string_vars[&EnvStringVar::RedditClientId].clone(),
            reddit_client_secret: string_vars[&EnvStringVar::RedditClientSecret].clone(),
            reddit_username: string_vars[&EnvStringVar::RedditUsername].clone(),
            reddit_password: string_vars[&EnvStringVar::RedditPassword].clone(),

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

            mongo_enable_initialization: bool_vars[&EnvBoolVar::MongoEnableInitialization],
            mongo_enable_initialization_for_providers: bool_vars
                [&EnvBoolVar::MongoEnableInitialization]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForProviders],

            mongo_enable_initialization_for_arxiv: bool_vars
                [&EnvBoolVar::MongoEnableInitialization]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForArxiv],
            mongo_enable_initialization_for_biorxiv: bool_vars
                [&EnvBoolVar::MongoEnableInitialization]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForBiorxiv],
            mongo_enable_initialization_for_github: bool_vars
                [&EnvBoolVar::MongoEnableInitialization]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForGithub],
            mongo_enable_initialization_for_habr: bool_vars[&EnvBoolVar::MongoEnableInitialization]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForHabr],
            mongo_enable_initialization_for_medrxiv: bool_vars
                [&EnvBoolVar::MongoEnableInitialization]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForMedrxiv],
            mongo_enable_initialization_for_reddit: bool_vars
                [&EnvBoolVar::MongoEnableInitialization]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForReddit],
            mongo_enable_initialization_for_twitter: bool_vars
                [&EnvBoolVar::MongoEnableInitialization]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableInitializationForTwitter],

            mongo_enable_write_error_logs: bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogs],
            mongo_enable_write_error_logs_for_providers: bool_vars
                [&EnvBoolVar::MongoEnableWriteErrorLogs]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForProviders],
            mongo_enable_write_error_logs_for_arxiv: bool_vars
                [&EnvBoolVar::MongoEnableWriteErrorLogs]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForArxiv],
            mongo_enable_write_error_logs_for_biorxiv: bool_vars
                [&EnvBoolVar::MongoEnableWriteErrorLogs]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForBiorxiv],
            mongo_enable_write_error_logs_for_github: bool_vars
                [&EnvBoolVar::MongoEnableWriteErrorLogs]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForGithub],
            mongo_enable_write_error_logs_for_habr: bool_vars
                [&EnvBoolVar::MongoEnableWriteErrorLogs]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForHabr],
            mongo_enable_write_error_logs_for_medrxiv: bool_vars
                [&EnvBoolVar::MongoEnableWriteErrorLogs]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForMedrxiv],
            mongo_enable_write_error_logs_for_reddit: bool_vars
                [&EnvBoolVar::MongoEnableWriteErrorLogs]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForReddit],
            mongo_enable_write_error_logs_for_twitter: bool_vars
                [&EnvBoolVar::MongoEnableWriteErrorLogs]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableWriteErrorLogsForTwitter],

            mongo_enable_cleaning_warning_logs_db: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDb],
            mongo_enable_cleaning_warning_logs_db_for_providers: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDb]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForProviders],
            mongo_enable_cleaning_warning_logs_db_for_arxiv: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDb]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForArxiv],
            mongo_enable_cleaning_warning_logs_db_for_biorxiv: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDb]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForBiorxiv],
            mongo_enable_cleaning_warning_logs_db_for_github: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDb]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForGithub],
            mongo_enable_cleaning_warning_logs_db_for_habr: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDb]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForHabr],
            mongo_enable_cleaning_warning_logs_db_for_medrxiv: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDb]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForMedrxiv],
            mongo_enable_cleaning_warning_logs_db_for_reddit: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDb]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForReddit],
            mongo_enable_cleaning_warning_logs_db_for_twitter: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDb]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbForTwitter],

            mongo_enable_cleaning_warning_logs_db_collections: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollections],
            mongo_enable_cleaning_warning_logs_db_collections_for_providers: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollections]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForProviders],
            mongo_enable_cleaning_warning_logs_db_collections_for_arxiv: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollections]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForArxiv],
            mongo_enable_cleaning_warning_logs_db_collections_for_biorxiv: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollections]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForBiorxiv],
            mongo_enable_cleaning_warning_logs_db_collections_for_github: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollections]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForGithub],
            mongo_enable_cleaning_warning_logs_db_collections_for_habr: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollections]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForHabr],
            mongo_enable_cleaning_warning_logs_db_collections_for_medrxiv: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollections]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForMedrxiv],
            mongo_enable_cleaning_warning_logs_db_collections_for_reddit: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollections]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForReddit],
            mongo_enable_cleaning_warning_logs_db_collections_for_twitter: bool_vars
                [&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollections]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableCleaningWarningLogsDbCollectionsForTwitter],

            mongo_enable_link_parts_randomize_order: bool_vars
                [&EnvBoolVar::MongoEnableLinkPartsRandomizeOrder],
            mongo_enable_link_parts_randomize_order_for_providers: bool_vars
                [&EnvBoolVar::MongoEnableLinkPartsRandomizeOrder]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForProviders],
            mongo_enable_link_parts_randomize_order_for_arxiv: bool_vars
                [&EnvBoolVar::MongoEnableLinkPartsRandomizeOrder]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForArxiv],
            mongo_enable_link_parts_randomize_order_for_biorxiv: bool_vars
                [&EnvBoolVar::MongoEnableLinkPartsRandomizeOrder]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForBiorxiv],
            mongo_enable_link_parts_randomize_order_for_github: bool_vars
                [&EnvBoolVar::MongoEnableLinkPartsRandomizeOrder]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForGithub],
            mongo_enable_link_parts_randomize_order_for_habr: bool_vars
                [&EnvBoolVar::MongoEnableLinkPartsRandomizeOrder]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForHabr],
            mongo_enable_link_parts_randomize_order_for_medrxiv: bool_vars
                [&EnvBoolVar::MongoEnableLinkPartsRandomizeOrder]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForMedrxiv],
            mongo_enable_link_parts_randomize_order_for_reddit: bool_vars
                [&EnvBoolVar::MongoEnableLinkPartsRandomizeOrder]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForReddit],
            mongo_enable_link_parts_randomize_order_for_twitter: bool_vars
                [&EnvBoolVar::MongoEnableLinkPartsRandomizeOrder]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForProviders]
                && bool_vars[&EnvBoolVar::MongoEnableLinkPartsRandomizeOrderForTwitter],

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

            warning_logs_directory_name: string_vars[&EnvStringVar::WarningLogsDirectoryName]
                .clone(),
            unhandled_success_handled_success_are_there_items_initialized_posts_dir: string_vars
                [&EnvStringVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir]
                .clone(),
            path_to_provider_link_parts_folder: string_vars
                [&EnvStringVar::PathToProviderLinkPartsFolder]
                .clone(),
            log_file_extension: string_vars[&EnvStringVar::LogFileExtension].clone(),

            enable_write_error_logs_in_local_folder: bool_vars
                [&EnvBoolVar::EnableWriteErrorLogsInLocalFolder],
            enable_write_error_logs_in_local_folder_for_provider: bool_vars
                [&EnvBoolVar::EnableWriteErrorLogsInLocalFolder]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForProvider],
            enable_write_error_logs_in_local_folder_for_arxiv: bool_vars
                [&EnvBoolVar::EnableWriteErrorLogsInLocalFolder]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForProvider]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForArxiv],
            enable_write_error_logs_in_local_folder_for_biorxiv: bool_vars
                [&EnvBoolVar::EnableWriteErrorLogsInLocalFolder]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForProvider]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForBiorxiv],
            enable_write_error_logs_in_local_folder_for_github: bool_vars
                [&EnvBoolVar::EnableWriteErrorLogsInLocalFolder]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForProvider]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForGithub],
            enable_write_error_logs_in_local_folder_for_habr: bool_vars
                [&EnvBoolVar::EnableWriteErrorLogsInLocalFolder]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForProvider]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForHabr],
            enable_write_error_logs_in_local_folder_for_medrxiv: bool_vars
                [&EnvBoolVar::EnableWriteErrorLogsInLocalFolder]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForProvider]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForMedrxiv],
            enable_write_error_logs_in_local_folder_for_reddit: bool_vars
                [&EnvBoolVar::EnableWriteErrorLogsInLocalFolder]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForProvider]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForReddit],
            enable_write_error_logs_in_local_folder_for_twitter: bool_vars
                [&EnvBoolVar::EnableWriteErrorLogsInLocalFolder]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForProvider]
                && bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolderForTwitter],

            enable_cleaning_warning_logs_directory: bool_vars
                [&EnvBoolVar::EnableCleaningWarningLogsDirectory],
            enable_cleaning_warning_logs_directory_for_providers: bool_vars
                [&EnvBoolVar::EnableCleaningWarningLogsDirectory]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForProviders],
            enable_cleaning_warning_logs_directory_for_arxiv: bool_vars
                [&EnvBoolVar::EnableCleaningWarningLogsDirectory]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForProviders]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForArxiv],
            enable_cleaning_warning_logs_directory_for_biorxiv: bool_vars
                [&EnvBoolVar::EnableCleaningWarningLogsDirectory]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForProviders]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForBiorxiv],
            enable_cleaning_warning_logs_directory_for_github: bool_vars
                [&EnvBoolVar::EnableCleaningWarningLogsDirectory]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForProviders]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForGithub],
            enable_cleaning_warning_logs_directory_for_habr: bool_vars
                [&EnvBoolVar::EnableCleaningWarningLogsDirectory]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForProviders]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForHabr],
            enable_cleaning_warning_logs_directory_for_medrxiv: bool_vars
                [&EnvBoolVar::EnableCleaningWarningLogsDirectory]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForProviders]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForMedrxiv],
            enable_cleaning_warning_logs_directory_for_reddit: bool_vars
                [&EnvBoolVar::EnableCleaningWarningLogsDirectory]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForProviders]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForReddit],
            enable_cleaning_warning_logs_directory_for_twitter: bool_vars
                [&EnvBoolVar::EnableCleaningWarningLogsDirectory]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForProviders]
                && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForTwitter],

            starting_check_link: string_vars[&EnvStringVar::StartingCheckLink].clone(),
            arxiv_check_link: string_vars[&EnvStringVar::ArxivCheckLink].clone(),
            biorxiv_check_link: string_vars[&EnvStringVar::BiorxivCheckLink].clone(),
            github_check_link: string_vars[&EnvStringVar::GithubCheckLink].clone(),
            habr_check_link: string_vars[&EnvStringVar::HabrCheckLink].clone(),
            medrxiv_check_link: string_vars[&EnvStringVar::MedrxivCheckLink].clone(),
            reddit_check_link: string_vars[&EnvStringVar::RedditCheckLink].clone(),
            twitter_check_link: string_vars[&EnvStringVar::TwitterCheckLink].clone(),

            enable_providers: bool_vars[&EnvBoolVar::EnableProviders],
            enable_arxiv: bool_vars[&EnvBoolVar::EnableProviders]
                && bool_vars[&EnvBoolVar::EnableArxiv],
            enable_biorxiv: bool_vars[&EnvBoolVar::EnableProviders]
                && bool_vars[&EnvBoolVar::EnableBiorxiv],
            enable_github: bool_vars[&EnvBoolVar::EnableProviders]
                && bool_vars[&EnvBoolVar::EnableGithub],
            enable_habr: bool_vars[&EnvBoolVar::EnableProviders]
                && bool_vars[&EnvBoolVar::EnableHabr],
            enable_medrxiv: bool_vars[&EnvBoolVar::EnableProviders]
                && bool_vars[&EnvBoolVar::EnableMedrxiv],
            enable_reddit: bool_vars[&EnvBoolVar::EnableProviders]
                && bool_vars[&EnvBoolVar::EnableReddit],
            enable_twitter: bool_vars[&EnvBoolVar::EnableProviders]
                && bool_vars[&EnvBoolVar::EnableTwitter],

            enable_prints: bool_vars[&EnvBoolVar::EnablePrints],
            enable_prints_for_providers: bool_vars[&EnvBoolVar::EnablePrints]
                && bool_vars[&EnvBoolVar::EnablePrintsForProviders],
            enable_prints_arxiv: bool_vars[&EnvBoolVar::EnablePrints]
                && bool_vars[&EnvBoolVar::EnablePrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePrintsArxiv],
            enable_prints_biorxiv: bool_vars[&EnvBoolVar::EnablePrints]
                && bool_vars[&EnvBoolVar::EnablePrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePrintsBiorxiv],
            enable_prints_github: bool_vars[&EnvBoolVar::EnablePrints]
                && bool_vars[&EnvBoolVar::EnablePrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePrintsGithub],
            enable_prints_habr: bool_vars[&EnvBoolVar::EnablePrints]
                && bool_vars[&EnvBoolVar::EnablePrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePrintsHabr],
            enable_prints_medrxiv: bool_vars[&EnvBoolVar::EnablePrints]
                && bool_vars[&EnvBoolVar::EnablePrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePrintsMedrxiv],
            enable_prints_reddit: bool_vars[&EnvBoolVar::EnablePrints]
                && bool_vars[&EnvBoolVar::EnablePrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePrintsReddit],
            enable_prints_twitter: bool_vars[&EnvBoolVar::EnablePrints]
                && bool_vars[&EnvBoolVar::EnablePrintsForProviders]
                && bool_vars[&EnvBoolVar::EnablePrintsTwitter],

            enable_warning_high_prints: bool_vars[&EnvBoolVar::EnableWarningHighPrints],
            enable_warning_high_prints_for_providers: bool_vars
                [&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForProviders],
            enable_warning_high_prints_for_arxiv: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForArxiv],
            enable_warning_high_prints_for_biorxiv: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForBiorxiv],
            enable_warning_high_prints_for_github: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForGithub],
            enable_warning_high_prints_for_habr: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForHabr],
            enable_warning_high_prints_for_medrxiv: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForMedrxiv],
            enable_warning_high_prints_for_reddit: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForReddit],
            enable_warning_high_prints_for_twitter: bool_vars[&EnvBoolVar::EnableWarningHighPrints]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableWarningHighPrintsForTwitter],

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
            enable_time_measurement_for_arxiv: bool_vars[&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementForArxiv],
            enable_time_measurement_for_biorxiv: bool_vars
                [&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementForBiorxiv],
            enable_time_measurement_for_github: bool_vars[&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementForGithub],
            enable_time_measurement_for_habr: bool_vars[&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementForHabr],
            enable_time_measurement_for_medrxiv: bool_vars
                [&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementForMedrxiv],
            enable_time_measurement_for_reddit: bool_vars[&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementForReddit],
            enable_time_measurement_for_twitter: bool_vars
                [&EnvBoolVar::EnableTimeMeasurementPrints]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableTimeMeasurementForTwitter],

            enable_info_prints: bool_vars[&EnvBoolVar::EnableInfoPrints],
            enable_info_prints_for_providers: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders],
            enable_info_for_arxiv: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoForArxiv],
            enable_info_for_biorxiv: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoForBiorxiv],
            enable_info_for_github: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoForGithub],
            enable_info_for_habr: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoForHabr],
            enable_info_for_medrxiv: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoForReddit],
            enable_info_for_reddit: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoForReddit],
            enable_info_for_twitter: bool_vars[&EnvBoolVar::EnableInfoPrints]
                && bool_vars[&EnvBoolVar::EnableInfoPrintsForProviders]
                && bool_vars[&EnvBoolVar::EnableInfoForTwitter],

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
