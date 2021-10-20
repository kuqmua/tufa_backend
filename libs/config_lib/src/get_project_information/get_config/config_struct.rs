extern crate toml;

use itertools::Itertools;

use crate::get_project_information::provider_kind_enum::ProviderKind;

use crate::get_project_information::get_config::github_authorization_struct::GithubAuthorization;
use crate::get_project_information::get_config::enable_providers_struct::EnableProviders;
use crate::get_project_information::get_config::enable_providers_prints_struct::EnableProvidersPrints;
use crate::get_project_information::get_config::providers_check_links_struct::ProvidersCheckLinks;
use crate::get_project_information::get_config::mongo_params_struct::MongoParams;
use crate::get_project_information::get_config::postgres_params_struct::PostgresParams;
use crate::get_project_information::get_config::postgres_url_parts_struct::PostgresUrlParts;
use crate::get_project_information::get_config::enable_error_providers_prints_struct::EnableErrorProvidersPrints;
use crate::get_project_information::get_config::enable_partial_success_providers_prints_struct::EnablePartialSuccessProvidersPrints;
use crate::get_project_information::get_config::enable_providers_cleaning_warning_logs_directory_struct::EnableProvidersCleaningWarningLogsDirectory;
use crate::get_project_information::get_config::enable_providers_links_limit_struct::EnableProvidersLinksLimit;
use crate::get_project_information::get_config::enable_providers_time_measurement_struct::EnableProvidersTimeMeasurement;
use crate::get_project_information::get_config::enable_providers_info_struct::EnableProvidersInfo;
use crate::get_project_information::get_config::enable_randomize_order_for_providers_link_parts_for_mongo_struct::EnableRandomizeOrderForProvidersLinkPartsForMongo;
use crate::get_project_information::get_config::enable_success_providers_prints_struct::EnableSuccessProvidersPrints;
use crate::get_project_information::get_config::enable_warning_high_providers_prints_struct::EnableWarningHighProvidersPrints;
use crate::get_project_information::get_config::enable_warning_low_providers_prints_struct::EnableWarningLowProvidersPrints;
use crate::get_project_information::get_config::params_struct::Params;
use crate::get_project_information::get_config::print_colors_struct::PrintColors;
use crate::get_project_information::get_config::providers_links_limits_struct::ProvidersLinksLimits;
use crate::get_project_information::get_config::enable_providers_cleaning_warning_logs_db_in_mongo_struct::EnableProvidersCleaningWarningLogsDbInMongo;
use crate::get_project_information::get_config::enable_providers_cleaning_warning_logs_db_collections_in_mongo_struct::EnableProvidersCleaningWarningLogsDbCollectionsInMongo;
use crate::get_project_information::get_config::enable_initialize_mongo_with_providers_link_parts_struct::EnableInitializeMongoWithProvidersLinkParts;
use crate::get_project_information::get_config::mongo_url_parts_struct::MongoUrlParts;
use crate::get_project_information::get_config::mongo_authorization_struct::MongoAuthorization;
use crate::get_project_information::get_config::postgres_authorization_struct::PostgresAuthorization;
use crate::get_project_information::get_config::reddit_authorization_struct::RedditAuthorization;

// use crate::get_project_information::get_config::config_error::ConfigError;
use crate::get_project_information::config_error::ConfigError;

// use crate::get_project_information::project_constants::ARXIV_NAME_TO_CHECK;
// use crate::get_project_information::project_constants::BIORXIV_NAME_TO_CHECK;
// use crate::get_project_information::project_constants::GITHUB_NAME_TO_CHECK;
// use crate::get_project_information::project_constants::HABR_NAME_TO_CHECK;
// use crate::get_project_information::project_constants::MEDRXIV_NAME_TO_CHECK;
// use crate::get_project_information::project_constants::REDDIT_NAME_TO_CHECK;
// use crate::get_project_information::project_constants::TWITTER_NAME_TO_CHECK;

use crate::get_project_information::env_var_string_enum::EnvStringVar;
use crate::get_project_information::env_var_bool_enum::EnvBoolVar;
use crate::get_project_information::env_var_u8_enum::EnvU8Var;
use crate::get_project_information::env_var_i64_enum::EnvI64Var;

#[derive(Debug, Clone, PartialEq)] //Default,//serde_derive::Serialize, serde_derive::Deserialize
pub struct ConfigStruct {
    pub github_authorization: GithubAuthorization,
    pub reddit_authorization: RedditAuthorization,
    //
    pub params: Params,
    pub mongo_params: MongoParams,
    pub postgres_params: PostgresParams,
    pub enable_providers: EnableProviders,
    pub providers_check_links: ProvidersCheckLinks,
    pub enable_providers_prints: EnableProvidersPrints,
    pub enable_warning_high_providers_prints: EnableWarningHighProvidersPrints, //todo maybe rename it into  EnableWarningHighPrintsForProviders
    pub enable_warning_low_providers_prints: EnableWarningLowProvidersPrints,
    pub enable_success_providers_prints: EnableSuccessProvidersPrints,
    pub enable_partial_success_providers_prints: EnablePartialSuccessProvidersPrints,
    pub enable_error_providers_prints: EnableErrorProvidersPrints,
    pub enable_providers_cleaning_warning_logs_directory:
        EnableProvidersCleaningWarningLogsDirectory,
    pub enable_providers_cleaning_warning_logs_db_in_mongo:
        EnableProvidersCleaningWarningLogsDbInMongo,
    pub enable_providers_cleaning_warning_logs_db_collections_in_mongo:
        EnableProvidersCleaningWarningLogsDbCollectionsInMongo,
    pub enable_providers_time_measurement: EnableProvidersTimeMeasurement,
    pub enable_providers_info: EnableProvidersInfo,
    pub enable_providers_links_limits: EnableProvidersLinksLimit,
    pub providers_links_limits: ProvidersLinksLimits,
    pub enable_randomize_order_for_providers_link_parts_for_mongo:
        EnableRandomizeOrderForProvidersLinkPartsForMongo,
    pub print_colors: PrintColors,
}

//todo: create custom error type
impl ConfigStruct {
    pub fn new() -> Result<Self, ConfigError<'static>> {
        let string_vars = EnvStringVar::get_env_values_hashmap()?;
        let bool_vars = EnvBoolVar::get_env_values_hashmap()?;
        let u8_vars = EnvU8Var::get_env_values_hashmap()?;
        let i64_vars = EnvI64Var::get_env_values_hashmap()?;
        //todo: rewrite it with type system enum ProviderKind
        let mut vec_of_provider_names_handle = Vec::<String>::with_capacity(ProviderKind::get_length());
        if bool_vars[&EnvBoolVar::EnableArxiv] {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Arxiv).to_owned());
        }
        if bool_vars[&EnvBoolVar::EnableBiorxiv] {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Biorxiv).to_owned())
        }
        if bool_vars[&EnvBoolVar::EnableGithub] {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Github).to_owned());
        }
        if bool_vars[&EnvBoolVar::EnableHabr] {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Habr).to_owned())
        }
        if bool_vars[&EnvBoolVar::EnableMedrxiv] {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Medrxiv).to_owned())
        }
        if bool_vars[&EnvBoolVar::EnableReddit] {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Reddit).to_owned())
        }
        if bool_vars[&EnvBoolVar::EnableTwitter] {
            vec_of_provider_names_handle.push(ProviderKind::get_string_name(ProviderKind::Twitter).to_owned())
        } 
        
        let handle_config: ConfigStruct = ConfigStruct {
            github_authorization: GithubAuthorization {
                github_name: string_vars[&EnvStringVar::GithubName].clone(), 
                github_token: string_vars[&EnvStringVar::GithubToken].clone(),
            },
            reddit_authorization: RedditAuthorization {
                reddit_user_agent: string_vars[&EnvStringVar::RedditUserAgent].clone(),
                reddit_client_id: string_vars[&EnvStringVar::RedditClientId].clone(),
                reddit_client_secret: string_vars[&EnvStringVar::RedditClientSecret].clone(),
                reddit_username: string_vars[&EnvStringVar::RedditUsername].clone(),
                reddit_password: string_vars[&EnvStringVar::RedditPassword].clone(),
            },
            params: Params {
                //todo
                vec_of_provider_names: vec_of_provider_names_handle,
                starting_check_link: string_vars[&EnvStringVar::StartingCheckLink].clone(),
                user_credentials_dummy_handle: string_vars[&EnvStringVar::UserCredentialsDummyHandle].clone(),
                warning_logs_directory_name: string_vars[&EnvStringVar::WarningLogsDirectoryName].clone(),
                unhandled_success_handled_success_are_there_items_initialized_posts_dir: string_vars[&EnvStringVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir].clone(),
                enable_cleaning_warning_logs_db_in_mongo: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongo],//used in code not for providers logic
                enable_cleaning_warning_logs_db_collections_in_mongo: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongo],//used in code not for providers logic
                enable_time_measurement: bool_vars[&EnvBoolVar::EnableTimeMeasurement],//used in code not for providers logic
                enable_provider_links_limit: bool_vars[&EnvBoolVar::EnableProviderLinksLimit],//used in code not for providers logic
                enable_common_providers_links_limit: bool_vars[&EnvBoolVar::EnableCommonProvidersLinksLimit],
                common_providers_links_limit: i64_vars[&EnvI64Var::CommonProvidersLinksLimit],
                enable_randomize_order_for_providers_link_parts_for_mongo: bool_vars[&EnvBoolVar::EnableRandomizeOrderForProvidersLinkPartsForMongo],
                enable_prints: bool_vars[&EnvBoolVar::EnablePrints],
                enable_error_prints: bool_vars[&EnvBoolVar::EnableErrorPrints],
                enable_warning_high_prints: bool_vars[&EnvBoolVar::EnableWarningHighPrints],
                enable_warning_low_prints: bool_vars[&EnvBoolVar::EnableWarningLowPrints],
                enable_success_prints: bool_vars[&EnvBoolVar::EnableSuccessPrints],
                enable_partial_success_prints: bool_vars[&EnvBoolVar::EnablePartialSuccessPrints],
                enable_time_measurement_prints: bool_vars[&EnvBoolVar::EnableTimeMeasurementPrints],
                enable_cleaning_warning_logs_directory_prints: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongo],
                enable_info_prints: bool_vars[&EnvBoolVar::EnableInfoPrints],
                enable_all_providers_prints: bool_vars[&EnvBoolVar::EnableAllProvidersPrints],
                enable_error_prints_for_all_providers: bool_vars[&EnvBoolVar::EnableErrorPrintsForAllProviders],
                enable_warning_high_prints_for_all_providers: bool_vars[&EnvBoolVar::EnableWarningHighPrintsForAllProviders],
                enable_warning_low_prints_for_all_providers: bool_vars[&EnvBoolVar::EnableWarningLowPrintsForAllProviders],
                enable_success_prints_for_all_providers: bool_vars[&EnvBoolVar::EnableSuccessPrintsForAllProviders],
                enable_partial_success_prints_for_all_providers: bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForAllProviders],
                enable_time_measurement_prints_for_all_providers: bool_vars[&EnvBoolVar::EnableTimeMeasurementPrintsForAllProviders],
                enable_cleaning_warning_logs_directory_prints_for_all_providers: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryPrintsForAllProviders],
                enable_info_prints_for_all_providers: bool_vars[&EnvBoolVar::EnableInfoPrintsForAllProviders],
                enable_write_error_logs_in_local_folder: bool_vars[&EnvBoolVar::EnableWriteErrorLogsInLocalFolder],
                enable_write_error_logs_in_mongo: bool_vars[&EnvBoolVar::EnableWriteErrorLogsInMongo],
                enable_initialize_mongo_with_providers_link_parts: bool_vars[&EnvBoolVar::EnableInitializeMongoWithProvidersLinkParts],
            },
            mongo_params: MongoParams {
                providers_db_name_handle: string_vars[&EnvStringVar::ProvidersDbNameHandle].clone(),
                providers_db_collection_handle_second_part: string_vars[&EnvStringVar::ProvidersDbCollectionHandleSecondPart].clone(),
                providers_db_collection_document_field_name_handle: string_vars[&EnvStringVar::ProvidersDbCollectionDocumentFieldNameHandle].clone(),
                db_providers_logs_name_handle: string_vars[&EnvStringVar::DbProvidersLogsNameHandle].clone(),
                db_providers_logs_collection_handle_second_part: string_vars[&EnvStringVar::DbProvidersLogsCollectionHandleSecondPart].clone(),
                db_providers_logs_collection_document_field_name_handle: string_vars[&EnvStringVar::DbProvidersLogsCollectionDocumentFieldNameHandle].clone(),
                path_to_provider_link_parts_folder: string_vars[&EnvStringVar::PathToProviderLinkPartsFolder].clone(),
                log_file_extension: string_vars[&EnvStringVar::LogFileExtension].clone(),
                enable_initialize_mongo_with_providers_link_parts:
                    EnableInitializeMongoWithProvidersLinkParts {
                        enable_initialize_mongo_with_arxiv_link_parts: bool_vars[&EnvBoolVar::EnableInitializeMongoWithArxivLinkParts],
                        enable_initialize_mongo_with_biorxiv_link_parts: bool_vars[&EnvBoolVar::EnableInitializeMongoWithBiorxivLinkParts],
                        enable_initialize_mongo_with_github_link_parts: bool_vars[&EnvBoolVar::EnableInitializeMongoWithGithubLinkParts],
                        enable_initialize_mongo_with_habr_link_parts: bool_vars[&EnvBoolVar::EnableInitializeMongoWithHabrLinkParts],
                        enable_initialize_mongo_with_medrxiv_link_parts: bool_vars[&EnvBoolVar::EnableInitializeMongoWithMedrxivLinkParts],
                        enable_initialize_mongo_with_reddit_link_parts: bool_vars[&EnvBoolVar::EnableInitializeMongoWithRedditLinkParts],
                        enable_initialize_mongo_with_twitter_link_parts: bool_vars[&EnvBoolVar::EnableInitializeMongoWithTwitterLinkParts],
                    },
                mongo_url_parts: MongoUrlParts {
                    mongo_first_handle_url_part: string_vars[&EnvStringVar::MongoFirstHandleUrlPart].clone(),
                    mongo_second_handle_url_part: string_vars[&EnvStringVar::MongoSecondHandleUrlPart].clone(),
                    mongo_third_handle_url_part: string_vars[&EnvStringVar::MongoThirdHandleUrlPart].clone(),
                    mongo_fourth_handle_url_part: string_vars[&EnvStringVar::MongoFourthHandleUrlPart].clone(),
                    mongo_fifth_handle_url_part: string_vars[&EnvStringVar::MongoFifthHandleUrlPart].clone(),
                },
                mongo_authorization: MongoAuthorization {
                    mongo_login: string_vars[&EnvStringVar::MongoLogin].clone(),
                    mongo_password: string_vars[&EnvStringVar::MongoPassword].clone(),
                    mongo_ip: string_vars[&EnvStringVar::MongoIp].clone(),
                    mongo_port: string_vars[&EnvStringVar::MongoPort].clone(),
                    mongo_params: string_vars[&EnvStringVar::MongoParams].clone(),
                },
            },
            postgres_params: PostgresParams {
                postgres_url_parts: PostgresUrlParts {
                    postgres_first_handle_url_part: string_vars[&EnvStringVar::PostgresFirstHandleUrlPart].clone(),
                postgres_second_handle_url_part: string_vars[&EnvStringVar::PostgresSecondHandleUrlPart].clone(),
                postgres_third_handle_url_part: string_vars[&EnvStringVar::PostgresThirdHandleUrlPart].clone(),
                postgres_fourth_handle_url_part: string_vars[&EnvStringVar::PostgresFourthHandleUrlPart].clone(),
                postgres_fifth_handle_url_part: string_vars[&EnvStringVar::PostgresFifthHandleUrlPart].clone(),
                },
                postgres_authorization: PostgresAuthorization {
                    postgres_login: string_vars[&EnvStringVar::PostgresLogin].clone(),
                    postgres_password: string_vars[&EnvStringVar::PostgresPassword].clone(),
                    postgres_ip: string_vars[&EnvStringVar::PostgresIp].clone(),
                    postgres_port: string_vars[&EnvStringVar::PostgresPort].clone(),
                    postgres_db: string_vars[&EnvStringVar::PostgresDb].clone(),
                },
            },
            enable_providers: EnableProviders {
                enable_arxiv: bool_vars[&EnvBoolVar::EnableProviders] && bool_vars[&EnvBoolVar::EnableArxiv],
                enable_biorxiv: bool_vars[&EnvBoolVar::EnableProviders] && bool_vars[&EnvBoolVar::EnableBiorxiv],
                enable_github: bool_vars[&EnvBoolVar::EnableProviders] && bool_vars[&EnvBoolVar::EnableGithub],
                enable_habr: bool_vars[&EnvBoolVar::EnableProviders] && bool_vars[&EnvBoolVar::EnableHabr],
                enable_medrxiv: bool_vars[&EnvBoolVar::EnableProviders] && bool_vars[&EnvBoolVar::EnableMedrxiv],
                enable_reddit: bool_vars[&EnvBoolVar::EnableProviders] && bool_vars[&EnvBoolVar::EnableReddit], 
                enable_twitter: bool_vars[&EnvBoolVar::EnableProviders] && bool_vars[&EnvBoolVar::EnableTwitter],
            },
            providers_check_links: ProvidersCheckLinks {
                arxiv_link: string_vars[&EnvStringVar::ArxivLink].clone(),
                biorxiv_link: string_vars[&EnvStringVar::BiorxivLink].clone(),
                github_link: string_vars[&EnvStringVar::GithubLink].clone(),
                habr_link: string_vars[&EnvStringVar::HabrLink].clone(),
                medrxiv_link: string_vars[&EnvStringVar::MedrxivLink].clone(),
                reddit_link: string_vars[&EnvStringVar::RedditLink].clone(),
                twitter_link: string_vars[&EnvStringVar::TwitterLink].clone(),
            },
            enable_providers_prints: EnableProvidersPrints {
                enable_prints_arxiv: bool_vars[&EnvBoolVar::EnablePrintsArxiv],
                enable_prints_biorxiv: bool_vars[&EnvBoolVar::EnablePrintsBiorxiv],
                enable_prints_github: bool_vars[&EnvBoolVar::EnablePrintsGithub],
                enable_prints_habr: bool_vars[&EnvBoolVar::EnablePrintsHabr],
                enable_prints_medrxiv: bool_vars[&EnvBoolVar::EnablePrintsMedrxiv],
                enable_prints_reddit: bool_vars[&EnvBoolVar::EnablePrintsReddit],
                enable_prints_twitter: bool_vars[&EnvBoolVar::EnablePrintsTwitter],
            },
            enable_warning_high_providers_prints: EnableWarningHighProvidersPrints {
                enable_warning_high_prints_for_arxiv: bool_vars[&EnvBoolVar::EnableWarningHighPrintsForArxiv],
                enable_warning_high_prints_for_biorxiv: bool_vars[&EnvBoolVar::EnableWarningHighPrintsForBiorxiv],
                enable_warning_high_prints_for_github: bool_vars[&EnvBoolVar::EnableWarningHighPrintsForGithub],
                enable_warning_high_prints_for_habr: bool_vars[&EnvBoolVar::EnableWarningHighPrintsForHabr],
                enable_warning_high_prints_for_medrxiv: bool_vars[&EnvBoolVar::EnableWarningHighPrintsForMedrxiv],
                enable_warning_high_prints_for_reddit: bool_vars[&EnvBoolVar::EnableWarningHighPrintsForReddit],
                enable_warning_high_prints_for_twitter: bool_vars[&EnvBoolVar::EnableWarningHighPrintsForTwitter],
            },
            enable_warning_low_providers_prints: EnableWarningLowProvidersPrints {
                enable_warning_low_prints_for_arxiv: bool_vars[&EnvBoolVar::EnableWarningLowPrintsForArxiv],
                enable_warning_low_prints_for_biorxiv: bool_vars[&EnvBoolVar::EnableWarningLowPrintsForBiorxiv],
                enable_warning_low_prints_for_github: bool_vars[&EnvBoolVar::EnableWarningLowPrintsForGithub],
                enable_warning_low_prints_for_habr: bool_vars[&EnvBoolVar::EnableWarningLowPrintsForHabr],
                enable_warning_low_prints_for_medrxiv: bool_vars[&EnvBoolVar::EnableWarningLowPrintsForMedrxiv],
                enable_warning_low_prints_for_reddit: bool_vars[&EnvBoolVar::EnableWarningLowPrintsForReddit],
                enable_warning_low_prints_for_twitter: bool_vars[&EnvBoolVar::EnableWarningLowPrintsForTwitter],
            },
            enable_success_providers_prints: EnableSuccessProvidersPrints {
                enable_success_prints_for_arxiv: bool_vars[&EnvBoolVar::EnableSuccessPrintsForArxiv],
                enable_success_prints_for_biorxiv: bool_vars[&EnvBoolVar::EnableSuccessPrintsForBiorxiv],
                enable_success_prints_for_github: bool_vars[&EnvBoolVar::EnableSuccessPrintsForGithub],
                enable_success_prints_for_habr: bool_vars[&EnvBoolVar::EnableSuccessPrintsForHabr],
                enable_success_prints_for_medrxiv: bool_vars[&EnvBoolVar::EnableSuccessPrintsForMedrxiv],
                enable_success_prints_for_reddit: bool_vars[&EnvBoolVar::EnableSuccessPrintsForReddit],
                enable_success_prints_for_twitter: bool_vars[&EnvBoolVar::EnableSuccessPrintsForTwitter],
            },
            enable_partial_success_providers_prints: EnablePartialSuccessProvidersPrints {
                enable_partial_success_prints_for_arxiv: bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForArxiv],
                enable_partial_success_prints_for_biorxiv: bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForBiorxiv],
                enable_partial_success_prints_for_github: bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForGithub],
                enable_partial_success_prints_for_habr: bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForHabr],
                enable_partial_success_prints_for_medrxiv: bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForMedrxiv],
                enable_partial_success_prints_for_reddit: bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForReddit],
                enable_partial_success_prints_for_twitter: bool_vars[&EnvBoolVar::EnablePartialSuccessPrintsForTwitter],
            },
            enable_error_providers_prints: EnableErrorProvidersPrints {
                enable_error_prints_for_arxiv: bool_vars[&EnvBoolVar::EnableErrorPrintsForArxiv],
                enable_error_prints_for_biorxiv: bool_vars[&EnvBoolVar::EnableErrorPrintsForBiorxiv],
                enable_error_prints_for_github: bool_vars[&EnvBoolVar::EnableErrorPrintsForGithub],
                enable_error_prints_for_habr: bool_vars[&EnvBoolVar::EnableErrorPrintsForHabr],
                enable_error_prints_for_medrxiv: bool_vars[&EnvBoolVar::EnableErrorPrintsForMedrxiv],
                enable_error_prints_for_reddit: bool_vars[&EnvBoolVar::EnableErrorPrintsForReddit],
                enable_error_prints_for_twitter: bool_vars[&EnvBoolVar::EnableErrorPrintsForTwitter],
            },
            enable_providers_cleaning_warning_logs_directory:
                EnableProvidersCleaningWarningLogsDirectory {
                    enable_cleaning_warning_logs_directory_for_arxiv: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectory] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForArxiv],
                    enable_cleaning_warning_logs_directory_for_biorxiv: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectory] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForBiorxiv],
                    enable_cleaning_warning_logs_directory_for_github: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectory] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForGithub],
                    enable_cleaning_warning_logs_directory_for_habr: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectory] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForHabr],
                    enable_cleaning_warning_logs_directory_for_medrxiv: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectory] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForMedrxiv],
                    enable_cleaning_warning_logs_directory_for_reddit: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectory] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForReddit],
                    enable_cleaning_warning_logs_directory_for_twitter: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectory] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDirectoryForTwitter],
                },
            enable_providers_cleaning_warning_logs_db_in_mongo:
                EnableProvidersCleaningWarningLogsDbInMongo {
                    enable_cleaning_warning_logs_db_in_mongo_for_arxiv: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongoForArxiv],
                    enable_cleaning_warning_logs_db_in_mongo_for_biorxiv: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongoForBiorxiv],
                    enable_cleaning_warning_logs_db_in_mongo_for_github: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongoForGithub],
                    enable_cleaning_warning_logs_db_in_mongo_for_habr: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongoForHabr],
                    enable_cleaning_warning_logs_db_in_mongo_for_medrxiv: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongoForMedrxiv],
                    enable_cleaning_warning_logs_db_in_mongo_for_reddit: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongoForReddit],
                    enable_cleaning_warning_logs_db_in_mongo_for_twitter: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbInMongoForTwitter],
                },
            enable_providers_cleaning_warning_logs_db_collections_in_mongo:
                EnableProvidersCleaningWarningLogsDbCollectionsInMongo {
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv],
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv],
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_github: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForGithub],
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_habr:bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForHabr],
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv],
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForReddit],
                    enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter: bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongo] && bool_vars[&EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter],
                },
            enable_providers_time_measurement: EnableProvidersTimeMeasurement {
                enable_time_measurement_for_arxiv: bool_vars[&EnvBoolVar::EnableTimeMeasurement] && bool_vars[&EnvBoolVar::EnableTimeMeasurementForArxiv],
                enable_time_measurement_for_biorxiv: bool_vars[&EnvBoolVar::EnableTimeMeasurement] && bool_vars[&EnvBoolVar::EnableTimeMeasurementForBiorxiv],
                enable_time_measurement_for_github: bool_vars[&EnvBoolVar::EnableTimeMeasurement] && bool_vars[&EnvBoolVar::EnableTimeMeasurementForGithub],
                enable_time_measurement_for_habr: bool_vars[&EnvBoolVar::EnableTimeMeasurement] && bool_vars[&EnvBoolVar::EnableTimeMeasurementForHabr],
                enable_time_measurement_for_medrxiv: bool_vars[&EnvBoolVar::EnableTimeMeasurement] && bool_vars[&EnvBoolVar::EnableTimeMeasurementForMedrxiv],
                enable_time_measurement_for_reddit: bool_vars[&EnvBoolVar::EnableTimeMeasurement] && bool_vars[&EnvBoolVar::EnableTimeMeasurementForReddit],
                enable_time_measurement_for_twitter: bool_vars[&EnvBoolVar::EnableTimeMeasurement] && bool_vars[&EnvBoolVar::EnableTimeMeasurementForTwitter],
            },
            enable_providers_info: EnableProvidersInfo {
                enable_info_for_arxiv: bool_vars[&EnvBoolVar::EnableInfoForArxiv],
                enable_info_for_biorxiv: bool_vars[&EnvBoolVar::EnableInfoForBiorxiv],
                enable_info_for_github: bool_vars[&EnvBoolVar::EnableInfoForGithub],
                enable_info_for_habr: bool_vars[&EnvBoolVar::EnableInfoForHabr],
                enable_info_for_medrxiv: bool_vars[&EnvBoolVar::EnableInfoForReddit],
                enable_info_for_reddit: bool_vars[&EnvBoolVar::EnableInfoForReddit],
                enable_info_for_twitter: bool_vars[&EnvBoolVar::EnableInfoForTwitter],
            },
            enable_providers_links_limits: EnableProvidersLinksLimit {
                enable_links_limit_for_arxiv: bool_vars[&EnvBoolVar::EnableProviderLinksLimit] && bool_vars[&EnvBoolVar::EnableLinksLimitForArxiv],
                enable_links_limit_for_biorxiv: bool_vars[&EnvBoolVar::EnableProviderLinksLimit] && bool_vars[&EnvBoolVar::EnableLinksLimitForBiorxiv],
                enable_links_limit_for_github: bool_vars[&EnvBoolVar::EnableProviderLinksLimit] && bool_vars[&EnvBoolVar::EnableLinksLimitForGithub],
                enable_links_limit_for_habr: bool_vars[&EnvBoolVar::EnableProviderLinksLimit] && bool_vars[&EnvBoolVar::EnableLinksLimitForHabr],
                enable_links_limit_for_medrxiv: bool_vars[&EnvBoolVar::EnableProviderLinksLimit] && bool_vars[&EnvBoolVar::EnableLinksLimitForMedrxiv],
                enable_links_limit_for_reddit: bool_vars[&EnvBoolVar::EnableProviderLinksLimit] && bool_vars[&EnvBoolVar::EnableLinksLimitForReddit],
                enable_links_limit_for_twitter: bool_vars[&EnvBoolVar::EnableProviderLinksLimit] && bool_vars[&EnvBoolVar::EnableLinksLimitForTwitter],
            },
            providers_links_limits: ProvidersLinksLimits {
                links_limit_for_arxiv: i64_vars[&EnvI64Var::LinksLimitForArxiv],
                links_limit_for_biorxiv: i64_vars[&EnvI64Var::LinksLimitForBiorxiv],
                links_limit_for_github: i64_vars[&EnvI64Var::LinksLimitForGithub], 
                links_limit_for_habr: i64_vars[&EnvI64Var::LinksLimitForHabr],
                links_limit_for_medrxiv: i64_vars[&EnvI64Var::LinksLimitForMedrxiv],
                links_limit_for_reddit: i64_vars[&EnvI64Var::LinksLimitForReddit],
                links_limit_for_twitter: i64_vars[&EnvI64Var::LinksLimitForTwitter],
            },
            enable_randomize_order_for_providers_link_parts_for_mongo:
                EnableRandomizeOrderForProvidersLinkPartsForMongo {
                    enable_randomize_order_for_arxiv_link_parts_for_mongo: bool_vars[&EnvBoolVar::EnableRandomizeOrderForArxivLinkPartsForMongo],
                    enable_randomize_order_for_biorxiv_link_parts_for_mongo: bool_vars[&EnvBoolVar::EnableRandomizeOrderForBiorxivLinkPartsForMongo],
                    enable_randomize_order_for_github_link_parts_for_mongo: bool_vars[&EnvBoolVar::EnableRandomizeOrderForGithubLinkPartsForMongo],
                    enable_randomize_order_for_habr_link_parts_for_mongo: bool_vars[&EnvBoolVar::EnableRandomizeOrderForHabrLinkPartsForMongo],
                    enable_randomize_order_for_medrxiv_link_parts_for_mongo: bool_vars[&EnvBoolVar::EnableRandomizeOrderForMedrxivLinkPartsForMongo],
                    enable_randomize_order_for_reddit_link_parts_for_mongo: bool_vars[&EnvBoolVar::EnableRandomizeOrderForRedditLinkPartsForMongo],
                    enable_randomize_order_for_twitter_link_parts_for_mongo: bool_vars[&EnvBoolVar::EnableRandomizeOrderForTwitterLinkPartsForMongo],
                },
            print_colors: PrintColors {
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
            },
        };
        return Ok(handle_config);
        // return ConfigStruct::wrap_config_checks(handle_config);
    }
    // fn wrap_config_checks(config_handle: ConfigStruct) -> Result<Self, ConfigError<'static>> {
    //     if !config_handle.github_authorization.github_name.is_empty() {
    //         let error: Result<ConfigStruct, ConfigError> =
    //             Err(ConfigError::Message("github_name is not valid".to_string()));
    //         drop(error);
    //     }
    //     if !config_handle.github_authorization.github_token.is_empty() {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "github_token is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //         .reddit_authorization
    //         .reddit_user_agent
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "reddit_user_agent is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //         .reddit_authorization
    //         .reddit_client_id
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "reddit_client_id is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //         .reddit_authorization
    //         .reddit_client_secret
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "reddit_client_secret is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //         .reddit_authorization
    //         .reddit_username
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "reddit_username is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //         .reddit_authorization
    //         .reddit_password
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "reddit_password is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //         .mongo_params.mongo_authorization
    //         .mongo_login
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "mongo_login is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //     .mongo_params
    //         .mongo_authorization
    //         .mongo_password
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "mongo_password is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //     .mongo_params
    //         .mongo_authorization
    //         .mongo_ip
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "mongo_ip is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //     .mongo_params
    //         .mongo_authorization
    //         .mongo_port
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "mongo_port is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !config_handle
    //     .mongo_params
    //         .mongo_authorization
    //         .mongo_params
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "mongo_params is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     //
    //     if config_handle.mongo_params.log_file_extension.is_empty() {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "log_file_extension is not empty".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if config_handle
    //         .mongo_params
    //         .path_to_provider_link_parts_folder
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "path_to_provider_link_parts_folder is empty"
    //                 .to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if config_handle
    //         .mongo_params
    //         .providers_db_collection_document_field_name_handle
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "db_collection_document_field_name_handle is empty"
    //                 .to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if config_handle
    //         .mongo_params
    //         .providers_db_collection_handle_second_part
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "db_collection_handle_second_part is empty"
    //                 .to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if config_handle
    //         .mongo_params
    //         .providers_db_name_handle
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "db_name_handle is empty".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if config_handle
    //         .params
    //         .unhandled_success_handled_success_are_there_items_initialized_posts_dir
    //         .is_empty()
    //     {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //                 "unhandled_success_handled_success_are_there_items_initialized_posts_dir is empty".to_string(),
    //             ));
    //         drop(error);
    //     }
    //     if config_handle.params.warning_logs_directory_name.is_empty() {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "warning_logs_directory_name is empty".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if config_handle.params.common_providers_links_limit > 0 {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "common_providers_links_limit <= 0".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !ConfigStruct::check_valid_i64_providers_links_limits_for_mongo(&config_handle) {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "providers_links_limits are not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     if !ConfigStruct::check_valid_vec_of_provider_names(&config_handle) {
    //         let error: Result<ConfigStruct, ConfigError> = Err(ConfigError::Message(
    //             "vec_of_provider_names is not valid".to_string(),
    //         ));
    //         drop(error);
    //     }
    //     Ok(config_handle)
    // }

    // fn check_valid_vec_of_provider_names(config_handle: &ConfigStruct) -> bool {
    //     if config_handle.params.vec_of_provider_names.len() == 0 {
    //         return true;
    //     }
    //     for potential_provider_name in &config_handle.params.vec_of_provider_names {
    //         if !(potential_provider_name == ARXIV_NAME_TO_CHECK
    //             || potential_provider_name == BIORXIV_NAME_TO_CHECK
    //             || potential_provider_name == GITHUB_NAME_TO_CHECK
    //             || potential_provider_name == HABR_NAME_TO_CHECK
    //             || potential_provider_name == MEDRXIV_NAME_TO_CHECK
    //             || potential_provider_name == REDDIT_NAME_TO_CHECK
    //             || potential_provider_name == TWITTER_NAME_TO_CHECK)
    //         {
    //             return false;
    //         }
    //     }
    //     let unique_vec_of_provider_names: Vec<String> = config_handle
    //         .params
    //         .vec_of_provider_names
    //         .clone()
    //         .into_iter()
    //         .unique()
    //         .collect();
    //     if config_handle.params.vec_of_provider_names == unique_vec_of_provider_names {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }

    // fn check_valid_i64_providers_links_limits_for_mongo(config_handle: &ConfigStruct) -> bool {
    //     if config_handle.providers_links_limits.links_limit_for_arxiv <= 0 {
    //         return false
    //     }
    //     if config_handle.providers_links_limits.links_limit_for_biorxiv <= 0 {
    //         return false
    //     }
    //     if config_handle.providers_links_limits.links_limit_for_github <= 0 {
    //         return false
    //     }
    //     if config_handle.providers_links_limits.links_limit_for_habr <= 0 {
    //         return false
    //     }
    //     if config_handle.providers_links_limits.links_limit_for_medrxiv <= 0 {
    //         return false
    //     }
    //     if config_handle.providers_links_limits.links_limit_for_reddit <= 0 {
    //         return false
    //     }
    //     if config_handle.providers_links_limits.links_limit_for_twitter <= 0 {
    //         return false
    //     }
    //     true
    // }
}
