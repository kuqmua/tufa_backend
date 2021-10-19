use std::collections::HashMap;

use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;

use strum_macros::EnumIter;

use dotenv::dotenv;

use crate::get_project_information::env_var_i64_names_constants::COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME;
use crate::get_project_information::env_var_i64_names_constants::LINKS_LIMIT_FOR_ARXIV_ENV_NAME;
use crate::get_project_information::env_var_i64_names_constants::LINKS_LIMIT_FOR_BIORXIV_ENV_NAME;
use crate::get_project_information::env_var_i64_names_constants::LINKS_LIMIT_FOR_GITHUB_ENV_NAME;
use crate::get_project_information::env_var_i64_names_constants::LINKS_LIMIT_FOR_HABR_ENV_NAME;
use crate::get_project_information::env_var_i64_names_constants::LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME;
use crate::get_project_information::env_var_i64_names_constants::LINKS_LIMIT_FOR_REDDIT_ENV_NAME;
use crate::get_project_information::env_var_i64_names_constants::LINKS_LIMIT_FOR_TWITTER_ENV_NAME;

use crate::get_project_information::var_or_int_parse_error_enum::VarOrIntParseError;
use crate::get_project_information::config_error_inner_type_enum::ConfigErrorInnerType;

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
pub enum EnvI64Var {
    CommonProvidersLinksLimit,
    LinksLimitForArxiv,
    LinksLimitForBiorxiv,
    LinksLimitForGithub,
    LinksLimitForHabr,
    LinksLimitForMedrxiv,
    LinksLimitForReddit,
    LinksLimitForTwitter,
}
#[derive(Debug)] 
pub struct ConfigTestError<'a> {
    env_var_name_kind: EnvI64Var,
    was_dotenv_enable: bool,
    env_name: &'a str, 
    env_error: ConfigErrorInnerType
} 

impl EnvI64Var {
    pub fn get_env_name(env_var_name_kind: EnvI64Var) -> &'static str {
        match env_var_name_kind {
            EnvI64Var::CommonProvidersLinksLimit => COMMON_PROVIDERS_LINKS_LIMIT_ENV_NAME,
            EnvI64Var::LinksLimitForArxiv => LINKS_LIMIT_FOR_ARXIV_ENV_NAME,
            EnvI64Var::LinksLimitForBiorxiv => LINKS_LIMIT_FOR_BIORXIV_ENV_NAME,
            EnvI64Var::LinksLimitForGithub => LINKS_LIMIT_FOR_GITHUB_ENV_NAME,
            EnvI64Var::LinksLimitForHabr => LINKS_LIMIT_FOR_HABR_ENV_NAME,
            EnvI64Var::LinksLimitForMedrxiv => LINKS_LIMIT_FOR_MEDRXIV_ENV_NAME,
            EnvI64Var::LinksLimitForReddit => LINKS_LIMIT_FOR_REDDIT_ENV_NAME,
            EnvI64Var::LinksLimitForTwitter => LINKS_LIMIT_FOR_TWITTER_ENV_NAME,
        }
    }
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    pub fn into_vec() -> Vec<EnvI64Var> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvI64Var::get_length());
        for env_var_name_kind in EnvI64Var::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvI64Var)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvI64Var::get_length());
        for env_var_name_kind in EnvI64Var::iter() {
            env_var_name_kind_vec.push((EnvI64Var::get_env_name(env_var_name_kind),   env_var_name_kind));
        }
        env_var_name_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvI64Var> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<&'static str, EnvI64Var> =
        HashMap::with_capacity(EnvI64Var::get_length());
        for env_var_name_kind_kind in EnvI64Var::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(EnvI64Var::get_env_name(env_var_name_kind_kind),   env_var_name_kind_kind);
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
    pub fn get_string_from_env_var(env_var_name_kind: EnvI64Var, was_dotenv_enable: bool) -> Result<String, ConfigTestError<'static>>{
        let string_name = EnvI64Var::get_env_name(env_var_name_kind);
        match std::env::var(string_name) {
            Ok(handle) => {
                Ok(handle)
            }
            Err(e) => {
                return Err(ConfigTestError {env_var_name_kind,  was_dotenv_enable, env_name: string_name, env_error: ConfigErrorInnerType::VarErrorHandle(e) })
            }   
        }
    }
    pub fn get_env_values_hashmap() -> Result<HashMap::<EnvI64Var, i64>, ConfigTestError<'static>> {
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
        let mut hmap: HashMap::<EnvI64Var, i64> = HashMap::new();
        let mut error_option: Option<ConfigTestError> = None;
        for env_var_name_kind in EnvI64Var::iter() {
            match EnvI64Var::get_string_from_env_var(env_var_name_kind, was_dotenv_enable) {
                Ok(env_var_string) => {
                    match env_var_string.parse::<i64>() {
                        Ok(handle) => {
                            hmap.insert(env_var_name_kind, handle);
                        },
                        Err(e) => {
                            error_option = Some(ConfigTestError {env_var_name_kind,  was_dotenv_enable, env_name: EnvI64Var::get_env_name(env_var_name_kind), env_error: ConfigErrorInnerType::VarOrIntParseErrorErrorHandle(VarOrIntParseError::Int(e)) });
                            break;
                        }
                    }
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

