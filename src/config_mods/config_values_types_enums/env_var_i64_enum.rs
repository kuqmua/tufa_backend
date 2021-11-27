use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;

use strum_macros::EnumIter;

use crate::config_mods::env_var_enum::EnvVar;

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

impl EnvI64Var {
    pub fn get_env_name(env_var_name_kind: EnvI64Var) -> &'static str {
        match env_var_name_kind {
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
}
