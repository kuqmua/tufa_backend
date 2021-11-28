use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;

impl EnvBoolVar {
    pub fn into_array() -> &'static [EnvBoolVar] {
        EnvBoolVar::all_variants()
    }
}
