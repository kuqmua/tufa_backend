use std::collections::HashMap;

use crate::config_mods::config_error_mods::config_error::ConfigError;

pub trait EnvVarTypedTrait {
    fn get_string_from_env_var(&self, was_dotenv_enable: bool) -> Result<String, ConfigError>;
    fn parse_string<T: std::str::FromStr>(value: String) -> Result<T, T::Err>;
    fn get_env_values_hashmap<T: std::str::FromStr>() -> Result<HashMap<Self, T>, ConfigError>
    where
        Self: std::marker::Sized;
    fn check_valid_typed_env_vars();//<T: std::str::FromStr>
}
