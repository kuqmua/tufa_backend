use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;
use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;
use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;
use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
pub fn ci_check_valid_env_vars_type() {
    EnvBoolVar::check_valid_typed_env_vars();
    EnvI64Var::check_valid_typed_env_vars();
    EnvStringVar::check_valid_typed_env_vars();
    EnvU8Var::check_valid_typed_env_vars();
}
