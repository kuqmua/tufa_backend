use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::get_env_name_trait::GetEnvName;

impl GetEnvName for EnvI64Var {
    fn get_env_name(self: &EnvI64Var) -> &'static str {
        match self {
            EnvI64Var::CommonProvidersLinksLimit => {
                EnvVar::CommonProvidersLinksLimit.get_env_name()
            }
            EnvI64Var::LinksLimitForArxiv => EnvVar::LinksLimitForArxiv.get_env_name(),
            EnvI64Var::LinksLimitForBiorxiv => EnvVar::LinksLimitForBiorxiv.get_env_name(),
            EnvI64Var::LinksLimitForGithub => EnvVar::LinksLimitForGithub.get_env_name(),
            EnvI64Var::LinksLimitForHabr => EnvVar::LinksLimitForHabr.get_env_name(),
            EnvI64Var::LinksLimitForMedrxiv => EnvVar::LinksLimitForMedrxiv.get_env_name(),
            EnvI64Var::LinksLimitForReddit => EnvVar::LinksLimitForReddit.get_env_name(),
            EnvI64Var::LinksLimitForTwitter => EnvVar::LinksLimitForTwitter.get_env_name(),
        }
    }
    fn into_array() -> &'static [Self] {
        EnvI64Var::all_variants()
    }
}
