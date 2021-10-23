use std::collections::HashMap;

use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;

use strum_macros::EnumIter;

use dotenv::dotenv;

use crate::get_project_information::config_error_inner_type_enum::ConfigErrorInnerType;
use crate::get_project_information::config_error::ConfigError;

use crate::get_project_information::env_var_types_enum::EnvVarTypes;

use crate::constants::project_constants::ENV_FILE_NAME;

use crate::get_project_information::env_var_enum::EnvVar;

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

impl EnvStringVar {
    pub fn get_env_name(env_var_name_kind: EnvStringVar) -> &'static str {
        match env_var_name_kind {
            EnvStringVar::GithubName => EnvVar::get_env_name(EnvVar::GithubName),
            EnvStringVar::GithubToken => EnvVar::get_env_name(EnvVar::GithubToken),

            EnvStringVar::RedditUserAgent => EnvVar::get_env_name(EnvVar::RedditUserAgent),
            EnvStringVar::RedditClientId => EnvVar::get_env_name(EnvVar::RedditClientId),
            EnvStringVar::RedditClientSecret => EnvVar::get_env_name(EnvVar::RedditClientSecret),
            EnvStringVar::RedditUsername => EnvVar::get_env_name(EnvVar::RedditUsername),
            EnvStringVar::RedditPassword => EnvVar::get_env_name(EnvVar::RedditPassword),

            EnvStringVar::StartingCheckLink => EnvVar::get_env_name(EnvVar::StartingCheckLink),
            EnvStringVar::UserCredentialsDummyHandle => EnvVar::get_env_name(EnvVar::UserCredentialsDummyHandle),
            EnvStringVar::WarningLogsDirectoryName => EnvVar::get_env_name(EnvVar::WarningLogsDirectoryName),
            EnvStringVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir => EnvVar::get_env_name(EnvVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir),
           
            EnvStringVar::ProvidersDbNameHandle => EnvVar::get_env_name(EnvVar::ProvidersDbNameHandle),
            EnvStringVar::ProvidersDbCollectionHandleSecondPart => EnvVar::get_env_name(EnvVar::ProvidersDbCollectionHandleSecondPart),
            EnvStringVar::ProvidersDbCollectionDocumentFieldNameHandle => EnvVar::get_env_name(EnvVar::ProvidersDbCollectionDocumentFieldNameHandle),
            EnvStringVar::PathToProviderLinkPartsFolder => EnvVar::get_env_name(EnvVar::PathToProviderLinkPartsFolder),
            EnvStringVar::LogFileExtension => EnvVar::get_env_name(EnvVar::LogFileExtension),
            EnvStringVar::DbProvidersLogsNameHandle => EnvVar::get_env_name(EnvVar::DbProvidersLogsNameHandle),
            EnvStringVar::DbProvidersLogsCollectionHandleSecondPart => EnvVar::get_env_name(EnvVar::DbProvidersLogsCollectionHandleSecondPart),
            EnvStringVar::DbProvidersLogsCollectionDocumentFieldNameHandle => EnvVar::get_env_name(EnvVar::DbProvidersLogsCollectionDocumentFieldNameHandle),

            EnvStringVar::MongoFirstHandleUrlPart => EnvVar::get_env_name(EnvVar::MongoFirstHandleUrlPart),
            EnvStringVar::MongoSecondHandleUrlPart => EnvVar::get_env_name(EnvVar::MongoSecondHandleUrlPart),
            EnvStringVar::MongoThirdHandleUrlPart => EnvVar::get_env_name(EnvVar::MongoThirdHandleUrlPart),
            EnvStringVar::MongoFourthHandleUrlPart => EnvVar::get_env_name(EnvVar::MongoFourthHandleUrlPart),
            EnvStringVar::MongoFifthHandleUrlPart => EnvVar::get_env_name(EnvVar::MongoFifthHandleUrlPart),

            EnvStringVar::MongoLogin => EnvVar::get_env_name(EnvVar::MongoLogin),
            EnvStringVar::MongoPassword => EnvVar::get_env_name(EnvVar::MongoPassword),
            EnvStringVar::MongoIp => EnvVar::get_env_name(EnvVar::MongoIp),
            EnvStringVar::MongoPort => EnvVar::get_env_name(EnvVar::MongoPort),
            EnvStringVar::MongoParams => EnvVar::get_env_name(EnvVar::MongoParams),

            EnvStringVar::PostgresFirstHandleUrlPart => EnvVar::get_env_name(EnvVar::PostgresFirstHandleUrlPart),
            EnvStringVar::PostgresSecondHandleUrlPart => EnvVar::get_env_name(EnvVar::PostgresSecondHandleUrlPart),
            EnvStringVar::PostgresThirdHandleUrlPart => EnvVar::get_env_name(EnvVar::PostgresThirdHandleUrlPart),
            EnvStringVar::PostgresFourthHandleUrlPart => EnvVar::get_env_name(EnvVar::PostgresFourthHandleUrlPart),
            EnvStringVar::PostgresFifthHandleUrlPart => EnvVar::get_env_name(EnvVar::PostgresFifthHandleUrlPart),

            EnvStringVar::PostgresLogin => EnvVar::get_env_name(EnvVar::PostgresLogin),
            EnvStringVar::PostgresPassword => EnvVar::get_env_name(EnvVar::PostgresPassword),
            EnvStringVar::PostgresIp => EnvVar::get_env_name(EnvVar::PostgresIp),
            EnvStringVar::PostgresPort => EnvVar::get_env_name(EnvVar::PostgresPort),
            EnvStringVar::PostgresDb => EnvVar::get_env_name(EnvVar::PostgresDb),

            EnvStringVar::ArxivLink => EnvVar::get_env_name(EnvVar::ArxivLink),
            EnvStringVar::BiorxivLink => EnvVar::get_env_name(EnvVar::BiorxivLink),
            EnvStringVar::GithubLink => EnvVar::get_env_name(EnvVar::GithubLink),
            EnvStringVar::HabrLink => EnvVar::get_env_name(EnvVar::HabrLink),
            EnvStringVar::MedrxivLink => EnvVar::get_env_name(EnvVar::MedrxivLink),
            EnvStringVar::RedditLink => EnvVar::get_env_name(EnvVar::RedditLink),
            EnvStringVar::TwitterLink => EnvVar::get_env_name(EnvVar::TwitterLink),
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
    pub fn get_string_from_env_var(env_var_name_kind: EnvStringVar, was_dotenv_enable: bool) -> Result<String, ConfigError<'static>>{
        let string_name = EnvStringVar::get_env_name(env_var_name_kind);
        match std::env::var(string_name) {
            Ok(handle) => {
                Ok(handle)
            }
            Err(e) => {
                return Err(ConfigError {env_var_name_kind: EnvVarTypes::String(env_var_name_kind),  was_dotenv_enable, env_name: string_name, env_error: ConfigErrorInnerType::VarErrorHandle(e) })
            }   
        }
    }
    pub fn get_env_values_hashmap() -> Result<HashMap::<EnvStringVar, String>, ConfigError<'static>> {
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
        let mut error_option: Option<ConfigError> = None;
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

