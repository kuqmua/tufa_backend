use crate::config_mods::config_error_mods::config_error::ConfigError;

pub trait EnvVarTypedTrait {
    fn get_string_from_env_var(
        &self,
        was_dotenv_enable: bool,
    ) -> Result<String, ConfigError<'static>>;
}
