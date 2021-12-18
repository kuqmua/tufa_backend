use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::env_var_trait::EnvVarTrait;

impl EnvVarTrait for EnvI64Var {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_env_name(self: &EnvI64Var) -> String {
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
}
