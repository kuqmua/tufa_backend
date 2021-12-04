use std::collections::HashMap;

use crate::config_mods::config_error_mods::config_error::ConfigError;

pub trait EnvVarTrait {
    fn get_env_name(&self) -> &'static str;
    fn into_array() -> &'static [Self]
    where
        Self: std::marker::Sized;
    fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, Self>
    where
        Self: std::marker::Sized;
    fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, Self)>
    where
        Self: std::marker::Sized;
    fn into_vec() -> Vec<Self>
    where
        Self: std::marker::Sized;
    fn get_string_from_env_var(
        &self,
        was_dotenv_enable: bool,
    ) -> Result<String, ConfigError<'static>>;
}
