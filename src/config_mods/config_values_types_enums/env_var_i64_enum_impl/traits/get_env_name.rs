use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::get_env_name_trait::GetEnvName;

impl GetEnvName for EnvI64Var {
    fn get_env_name(self: &EnvI64Var) -> &'static str {
        match self {
            EnvI64Var::CommonProvidersLinksLimit => {
                EnvVar::get_env_name(EnvVar::CommonProvidersLinksLimit)
            }
            EnvI64Var::LinksLimitForArxiv => EnvVar::get_env_name(EnvVar::LinksLimitForArxiv),
            EnvI64Var::LinksLimitForBiorxiv => EnvVar::get_env_name(EnvVar::LinksLimitForBiorxiv),
            EnvI64Var::LinksLimitForGithub => EnvVar::get_env_name(EnvVar::LinksLimitForGithub),
            EnvI64Var::LinksLimitForHabr => EnvVar::get_env_name(EnvVar::LinksLimitForHabr),
            EnvI64Var::LinksLimitForMedrxiv => EnvVar::get_env_name(EnvVar::LinksLimitForMedrxiv),
            EnvI64Var::LinksLimitForReddit => EnvVar::get_env_name(EnvVar::LinksLimitForReddit),
            EnvI64Var::LinksLimitForTwitter => EnvVar::get_env_name(EnvVar::LinksLimitForTwitter),
        }
    }
}
