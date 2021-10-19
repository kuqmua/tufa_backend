use std::collections::HashMap;

use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;

use strum_macros::EnumIter;

use dotenv::dotenv;

use crate::get_project_information::env_var_u8_names_constants::ERROR_BLUE_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::ERROR_GREEN_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::ERROR_RED_ENV_NAME;

use crate::get_project_information::env_var_u8_names_constants::WARNING_HIGH_BLUE_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::WARNING_HIGH_GREEN_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::WARNING_HIGH_RED_ENV_NAME;

use crate::get_project_information::env_var_u8_names_constants::WARNING_LOW_BLUE_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::WARNING_LOW_GREEN_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::WARNING_LOW_RED_ENV_NAME;

use crate::get_project_information::env_var_u8_names_constants::SUCCESS_BLUE_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::SUCCESS_GREEN_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::SUCCESS_RED_ENV_NAME;

use crate::get_project_information::env_var_u8_names_constants::PARTIAL_SUCCESS_BLUE_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::PARTIAL_SUCCESS_GREEN_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::PARTIAL_SUCCESS_RED_ENV_NAME;

use crate::get_project_information::env_var_u8_names_constants::CLEANING_BLUE_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::CLEANING_GREEN_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::CLEANING_RED_ENV_NAME;

use crate::get_project_information::env_var_u8_names_constants::TIME_MEASUREMENT_BLUE_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::TIME_MEASUREMENT_GREEN_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::TIME_MEASUREMENT_RED_ENV_NAME;

use crate::get_project_information::env_var_u8_names_constants::INFO_BLUE_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::INFO_GREEN_ENV_NAME;
use crate::get_project_information::env_var_u8_names_constants::INFO_RED_ENV_NAME;

use crate::get_project_information::var_or_int_parse_error_enum::VarOrIntParseError;
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
pub enum EnvU8Var {
    ErrorRed,
    ErrorGreen,
    ErrorBlue,
    WarningHighRed,
    WarningHighGreen,
    WarningHighBlue,
    WarningLowRed,
    WarningLowGreen,
    WarningLowBlue,
    SuccessRed,
    SuccessGreen,
    SuccessBlue,
    PartialSuccessRed,
    PartialSuccessGreen,
    PartialSuccessBlue,
    CleaningRed,
    CleaningGreen,
    CleaningBlue,
    TimeMeasurementRed,
    TimeMeasurementGreen,
    TimeMeasurementBlue,
    InfoRed,
    InfoGreen,
    InfoBlue,
}
#[derive(Debug)] 
pub struct ConfigTestError<'a> {
    env_var_name_kind: EnvVarTypes,
    was_dotenv_enable: bool,
    env_name: &'a str, 
    env_error: ConfigErrorInnerType
} 

impl EnvU8Var {
    pub fn get_env_name(env_var_name_kind: EnvU8Var) -> &'static str {
        match env_var_name_kind {
            EnvU8Var::ErrorRed => ERROR_RED_ENV_NAME,
            EnvU8Var::ErrorGreen => ERROR_GREEN_ENV_NAME,
            EnvU8Var::ErrorBlue => ERROR_BLUE_ENV_NAME,
            EnvU8Var::WarningHighRed => WARNING_HIGH_RED_ENV_NAME,
            EnvU8Var::WarningHighGreen => WARNING_HIGH_GREEN_ENV_NAME,
            EnvU8Var::WarningHighBlue => WARNING_HIGH_BLUE_ENV_NAME,
            EnvU8Var::WarningLowRed => WARNING_LOW_RED_ENV_NAME,
            EnvU8Var::WarningLowGreen => WARNING_LOW_GREEN_ENV_NAME,
            EnvU8Var::WarningLowBlue => WARNING_LOW_BLUE_ENV_NAME,
            EnvU8Var::SuccessRed => SUCCESS_RED_ENV_NAME,
            EnvU8Var::SuccessGreen => SUCCESS_GREEN_ENV_NAME,
            EnvU8Var::SuccessBlue => SUCCESS_BLUE_ENV_NAME,
            EnvU8Var::PartialSuccessRed => PARTIAL_SUCCESS_RED_ENV_NAME,
            EnvU8Var::PartialSuccessGreen => PARTIAL_SUCCESS_GREEN_ENV_NAME,
            EnvU8Var::PartialSuccessBlue => PARTIAL_SUCCESS_BLUE_ENV_NAME,
            EnvU8Var::CleaningRed => CLEANING_RED_ENV_NAME,
            EnvU8Var::CleaningGreen => CLEANING_GREEN_ENV_NAME,
            EnvU8Var::CleaningBlue => CLEANING_BLUE_ENV_NAME,
            EnvU8Var::TimeMeasurementRed => TIME_MEASUREMENT_RED_ENV_NAME,
            EnvU8Var::TimeMeasurementGreen => TIME_MEASUREMENT_GREEN_ENV_NAME,
            EnvU8Var::TimeMeasurementBlue => TIME_MEASUREMENT_BLUE_ENV_NAME,
            EnvU8Var::InfoRed => INFO_RED_ENV_NAME,
            EnvU8Var::InfoGreen => INFO_GREEN_ENV_NAME,
            EnvU8Var::InfoBlue => INFO_BLUE_ENV_NAME, 
        }
    }
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    pub fn into_vec() -> Vec<EnvU8Var> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvU8Var::get_length());
        for env_var_name_kind in EnvU8Var::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvU8Var)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvU8Var::get_length());
        for env_var_name_kind in EnvU8Var::iter() {
            env_var_name_kind_vec.push((EnvU8Var::get_env_name(env_var_name_kind),   env_var_name_kind));
        }
        env_var_name_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvU8Var> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<&'static str, EnvU8Var> =
        HashMap::with_capacity(EnvU8Var::get_length());
        for env_var_name_kind_kind in EnvU8Var::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(EnvU8Var::get_env_name(env_var_name_kind_kind),   env_var_name_kind_kind);
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
    pub fn get_string_from_env_var(env_var_name_kind: EnvU8Var, was_dotenv_enable: bool) -> Result<String, ConfigTestError<'static>>{
        let string_name = EnvU8Var::get_env_name(env_var_name_kind);
        match std::env::var(string_name) {
            Ok(handle) => {
                Ok(handle)
            }
            Err(e) => {
                return Err(ConfigTestError {env_var_name_kind: EnvVarTypes::U8(env_var_name_kind),  was_dotenv_enable, env_name: string_name, env_error: ConfigErrorInnerType::VarErrorHandle(e) })
            }   
        }
    }
    pub fn get_env_values_hashmap() -> Result<HashMap::<EnvU8Var, u8>, ConfigTestError<'static>> {
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
        let mut hmap: HashMap::<EnvU8Var, u8> = HashMap::new();
        let mut error_option: Option<ConfigTestError> = None;
        for env_var_name_kind in EnvU8Var::iter() {
            match EnvU8Var::get_string_from_env_var(env_var_name_kind, was_dotenv_enable) {
                Ok(env_var_string) => {
                    match env_var_string.parse::<u8>() {
                        Ok(handle) => {
                            hmap.insert(env_var_name_kind, handle);
                        },
                        Err(e) => {
                            error_option = Some(ConfigTestError {env_var_name_kind: EnvVarTypes::U8(env_var_name_kind),  was_dotenv_enable, env_name: EnvU8Var::get_env_name(env_var_name_kind), env_error: ConfigErrorInnerType::VarOrIntParseErrorErrorHandle(VarOrIntParseError::Int(e)) });
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

