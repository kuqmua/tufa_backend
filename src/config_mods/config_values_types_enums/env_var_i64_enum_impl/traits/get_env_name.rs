use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::env_var_trait::EnvVarTrait;

impl EnvVarTrait for EnvI64Var {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
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
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_array() -> &'static [Self] {
        EnvI64Var::all_variants()
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, Self> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<&'static str, Self> =
            HashMap::with_capacity(Self::get_length());
        for env_var_name_kind_kind in Self::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(
                env_var_name_kind_kind.get_env_name(),
                env_var_name_kind_kind,
            );
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, Self)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(Self::get_length());
        for env_var_name_kind in Self::iter() {
            env_var_name_kind_vec.push((env_var_name_kind.get_env_name(), env_var_name_kind));
        }
        env_var_name_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_vec() -> Vec<Self> {
        let mut env_var_name_kind_vec = Vec::with_capacity(Self::get_length());
        for env_var_name_kind in Self::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
}
