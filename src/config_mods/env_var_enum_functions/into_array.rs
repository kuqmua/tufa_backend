use crate::config_mods::env_var_enum::EnvVar;

impl EnvVar {
    pub fn into_array() -> &'static [EnvVar] {
        EnvVar::all_variants()
    }
}
