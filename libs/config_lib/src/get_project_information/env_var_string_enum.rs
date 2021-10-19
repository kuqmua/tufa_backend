use std::collections::HashMap;

use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;

use strum_macros::EnumIter;

use dotenv::dotenv;

use crate::get_project_information::env_var_string_names_constants::STARTING_CHECK_LINK_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::WARNING_LOGS_DIRECTORY_NAME_ENV_NAME;

// [mongo_params]
use crate::get_project_information::env_var_string_names_constants::DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::LOG_FILE_EXTENSION_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::PROVIDERS_DB_NAME_HANDLE_ENV_NAME;

// [mongo_params.mongo_url_parts]
use crate::get_project_information::env_var_string_names_constants::MONGO_FIRST_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::MONGO_FOURTH_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::MONGO_SECOND_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::MONGO_THIRD_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::MONGO_FIFTH_HANDLE_URL_PART_ENV_NAME;

// [postgres_params]
use crate::get_project_information::env_var_string_names_constants::POSTGRES_FIRST_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::POSTGRES_FOURTH_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::POSTGRES_SECOND_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::POSTGRES_THIRD_HANDLE_URL_PART_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::POSTGRES_FIFTH_HANDLE_URL_PART_ENV_NAME;

// [enable_providers]
use crate::get_project_information::env_var_string_names_constants::ARXIV_LINK_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::BIORXIV_LINK_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::GITHUB_LINK_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::HABR_LINK_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::MEDRXIV_LINK_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::REDDIT_LINK_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::TWITTER_LINK_ENV_NAME;

use crate::get_project_information::env_var_string_names_constants::GITHUB_NAME_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::GITHUB_TOKEN_ENV_NAME;

use crate::get_project_information::env_var_string_names_constants::MONGO_IP_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::MONGO_LOGIN_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::MONGO_PASSWORD_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::MONGO_PORT_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::MONGO_PARAMS_ENV_NAME;

use crate::get_project_information::env_var_string_names_constants::POSTGRES_LOGIN_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::POSTGRES_PASSWORD_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::POSTGRES_IP_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::POSTGRES_PORT_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::POSTGRES_DB_ENV_NAME;

use crate::get_project_information::env_var_string_names_constants::REDDIT_CLIENT_ID_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::REDDIT_CLIENT_SECRET_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::REDDIT_PASSWORD_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::REDDIT_USERNAME_ENV_NAME;
use crate::get_project_information::env_var_string_names_constants::REDDIT_USER_AGENT_ENV_NAME;

use crate::get_project_information::config_error_inner_type_enum::ConfigErrorInnerType;

use crate::get_project_information::env_var_types_enum::EnvVarTypes;

use crate::get_project_information::project_constants::ENV_FILE_NAME;

#[derive(
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
    StartingCheckLink,
    UserCredentialsDummyHandle,
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
    ArxivLink,
    BiorxivLink,
    GithubLink,
    HabrLink,
    MedrxivLink,
    RedditLink,
    TwitterLink,
}
#[derive(Debug)] 
pub struct ConfigTestError<'a> {
    env_var_name_kind: EnvVarTypes,
    was_dotenv_enable: bool,
    env_name: &'a str, 
    env_error: ConfigErrorInnerType
} 

impl EnvStringVar {
    pub fn get_env_name(env_var_name_kind: EnvStringVar) -> &'static str {
        match env_var_name_kind {
            EnvStringVar::GithubName => GITHUB_NAME_ENV_NAME,
            EnvStringVar::GithubToken => GITHUB_TOKEN_ENV_NAME,

            EnvStringVar::RedditUserAgent => REDDIT_USER_AGENT_ENV_NAME,
            EnvStringVar::RedditClientId => REDDIT_CLIENT_ID_ENV_NAME,
            EnvStringVar::RedditClientSecret => REDDIT_CLIENT_SECRET_ENV_NAME,
            EnvStringVar::RedditUsername => REDDIT_USERNAME_ENV_NAME,
            EnvStringVar::RedditPassword => REDDIT_PASSWORD_ENV_NAME,

            EnvStringVar::StartingCheckLink => STARTING_CHECK_LINK_ENV_NAME,
            EnvStringVar::UserCredentialsDummyHandle => USER_CREDENTIALS_DUMMY_HANDLE_ENV_NAME,
            EnvStringVar::WarningLogsDirectoryName => WARNING_LOGS_DIRECTORY_NAME_ENV_NAME,
            EnvStringVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir => UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR_ENV_NAME,
           
            EnvStringVar::ProvidersDbNameHandle => PROVIDERS_DB_NAME_HANDLE_ENV_NAME,
            EnvStringVar::ProvidersDbCollectionHandleSecondPart => PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART_ENV_NAME,
            EnvStringVar::ProvidersDbCollectionDocumentFieldNameHandle => PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME,
            EnvStringVar::PathToProviderLinkPartsFolder => PATH_TO_PROVIDER_LINK_PARTS_FOLDER_ENV_NAME,
            EnvStringVar::LogFileExtension => LOG_FILE_EXTENSION_ENV_NAME,
            EnvStringVar::DbProvidersLogsNameHandle => DB_PROVIDERS_LOGS_NAME_HANDLE_ENV_NAME,
            EnvStringVar::DbProvidersLogsCollectionHandleSecondPart => DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART_ENV_NAME,
            EnvStringVar::DbProvidersLogsCollectionDocumentFieldNameHandle => DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE_ENV_NAME,

            EnvStringVar::MongoFirstHandleUrlPart => MONGO_FIRST_HANDLE_URL_PART_ENV_NAME,
            EnvStringVar::MongoSecondHandleUrlPart => MONGO_SECOND_HANDLE_URL_PART_ENV_NAME,
            EnvStringVar::MongoThirdHandleUrlPart => MONGO_THIRD_HANDLE_URL_PART_ENV_NAME,
            EnvStringVar::MongoFourthHandleUrlPart => MONGO_FOURTH_HANDLE_URL_PART_ENV_NAME,
            EnvStringVar::MongoFifthHandleUrlPart => MONGO_FIFTH_HANDLE_URL_PART_ENV_NAME,

            EnvStringVar::MongoLogin => MONGO_LOGIN_ENV_NAME,
            EnvStringVar::MongoPassword => MONGO_PASSWORD_ENV_NAME,
            EnvStringVar::MongoIp => MONGO_IP_ENV_NAME,
            EnvStringVar::MongoPort => MONGO_PORT_ENV_NAME,
            EnvStringVar::MongoParams => MONGO_PARAMS_ENV_NAME,

            EnvStringVar::PostgresFirstHandleUrlPart => POSTGRES_FIRST_HANDLE_URL_PART_ENV_NAME,
            EnvStringVar::PostgresSecondHandleUrlPart => POSTGRES_SECOND_HANDLE_URL_PART_ENV_NAME,
            EnvStringVar::PostgresThirdHandleUrlPart => POSTGRES_THIRD_HANDLE_URL_PART_ENV_NAME,
            EnvStringVar::PostgresFourthHandleUrlPart => POSTGRES_FOURTH_HANDLE_URL_PART_ENV_NAME,
            EnvStringVar::PostgresFifthHandleUrlPart => POSTGRES_FIFTH_HANDLE_URL_PART_ENV_NAME,

            EnvStringVar::PostgresLogin => POSTGRES_LOGIN_ENV_NAME,
            EnvStringVar::PostgresPassword => POSTGRES_PASSWORD_ENV_NAME,
            EnvStringVar::PostgresIp => POSTGRES_IP_ENV_NAME,
            EnvStringVar::PostgresPort => POSTGRES_PORT_ENV_NAME,
            EnvStringVar::PostgresDb => POSTGRES_DB_ENV_NAME,

            EnvStringVar::ArxivLink => ARXIV_LINK_ENV_NAME,
            EnvStringVar::BiorxivLink => BIORXIV_LINK_ENV_NAME,
            EnvStringVar::GithubLink => GITHUB_LINK_ENV_NAME,
            EnvStringVar::HabrLink => HABR_LINK_ENV_NAME,
            EnvStringVar::MedrxivLink => MEDRXIV_LINK_ENV_NAME,
            EnvStringVar::RedditLink => REDDIT_LINK_ENV_NAME,
            EnvStringVar::TwitterLink => TWITTER_LINK_ENV_NAME,
        }
    }
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    pub fn into_vec() -> Vec<EnvStringVar> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvStringVar::get_length());
        for env_var_name_kind in EnvStringVar::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvStringVar)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvStringVar::get_length());
        for env_var_name_kind in EnvStringVar::iter() {
            env_var_name_kind_vec.push((EnvStringVar::get_env_name(env_var_name_kind),   env_var_name_kind));
        }
        env_var_name_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvStringVar> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<&'static str, EnvStringVar> =
        HashMap::with_capacity(EnvStringVar::get_length());
        for env_var_name_kind_kind in EnvStringVar::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(EnvStringVar::get_env_name(env_var_name_kind_kind),   env_var_name_kind_kind);
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
    pub fn get_string_from_env_var(env_var_name_kind: EnvStringVar, was_dotenv_enable: bool) -> Result<String, ConfigTestError<'static>>{
        let string_name = EnvStringVar::get_env_name(env_var_name_kind);
        match std::env::var(string_name) {
            Ok(handle) => {
                Ok(handle)
            }
            Err(e) => {
                return Err(ConfigTestError {env_var_name_kind: EnvVarTypes::String(env_var_name_kind),  was_dotenv_enable, env_name: string_name, env_error: ConfigErrorInnerType::VarErrorHandle(e) })
            }   
        }
    }
    pub fn get_env_values_hashmap() -> Result<HashMap::<EnvStringVar, String>, ConfigTestError<'static>> {
        let was_dotenv_enable: bool;
        match dotenv() {
            Ok(_) => {
                was_dotenv_enable = true;
            },
            Err(e) => {
                was_dotenv_enable = false;
                println!("dotenv() failed, trying without {} error: {:?}", ENV_FILE_NAME, e);
            }
        }
        let mut hmap: HashMap::<EnvStringVar, String> = HashMap::new();
        let mut error_option: Option<ConfigTestError> = None;
        for env_var_name_kind in EnvStringVar::iter() {
            match EnvStringVar::get_string_from_env_var(env_var_name_kind, was_dotenv_enable) {
                Ok(env_var_string) => {
                    hmap.insert(env_var_name_kind, env_var_string);
                }
                Err(e) => {
                    error_option = Some(e);
                    break;
                }
            }
            
        }
        if let Some(error) = error_option {
            return Err(error)
        }
        Ok(hmap)
    }
}

