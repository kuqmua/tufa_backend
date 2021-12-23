use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;
use crate::traits::enum_extention::EnumExtenstion;
use crate::traits::env_var_typed_trait::EnvVarTypedTrait;
use dotenv::dotenv;
use strum::IntoEnumIterator;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
pub fn ci_check_valid_u8_env_vars() {
    if let Err(e) = dotenv() {
        panic!("dotenv() failed, error: {:?}", e);
    }
    for env_var_name_kind in EnvU8Var::iter() {
        match std::env::var(&env_var_name_kind.to_upper_snake_case()) {
            Err(e) => panic!(
                "no such env name {} error: {:#?}",
                &env_var_name_kind.to_upper_snake_case(),
                e
            ),
            Ok(handle) => {
                if let Err(e) = EnvU8Var::parse_string::<bool>(handle) {
                    panic!(
                        "parse env var {} error: {:#?}",
                        &env_var_name_kind.to_upper_snake_case(),
                        e
                    )
                }
            }
        }
    }
}
