use crate::traits::enum_extention::EnumExtenstion;

use crate::config_mods::common_env_var_enum::CommonEnvVar;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
pub fn ci_check_valid_env_vars_type() {
    for env_var_kind in CommonEnvVar::into_array().iter() {
        env_var_kind.check_valid_env_vars_type();
    }
}
