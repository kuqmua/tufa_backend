use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;

impl EnvU8Var {
    pub fn into_array() -> &'static [EnvU8Var] {
        EnvU8Var::all_variants()
    }
}
