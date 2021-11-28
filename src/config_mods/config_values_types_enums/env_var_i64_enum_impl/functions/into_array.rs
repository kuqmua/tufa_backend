use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;

impl EnvI64Var {
    pub fn into_array() -> &'static [EnvI64Var] {
        EnvI64Var::all_variants()
    }
}
