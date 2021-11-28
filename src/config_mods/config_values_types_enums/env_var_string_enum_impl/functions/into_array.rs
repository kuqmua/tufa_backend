use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;

impl EnvStringVar {
    pub fn into_array() -> &'static [EnvStringVar] {
        EnvStringVar::all_variants()
    }
}
