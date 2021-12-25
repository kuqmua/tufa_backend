use crate::tests::tests_constants::USER_CREDENTIALS_DUMMY_HANDLE;

use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;
use crate::config_mods::config_values_types_enums::env_var_i64_enum::EnvI64Var;
use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;
use crate::config_mods::config_values_types_enums::env_var_u8_enum::EnvU8Var;
use crate::traits::env_var_typed_trait::EnvVarTypedTrait;

use dotenv::dotenv;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_env_vars() {
    let mut was_dotenv_enable = false;
    if dotenv().is_ok() {
        was_dotenv_enable = true;
    }
    match EnvStringVar::get_env_values_hashmap::<String>(was_dotenv_enable) {
        Err(e) => panic!("cannot get string env values hashmap, error: {:#?}", e),
        Ok(hashmap) => {
            for (key, value) in hashmap {
                if value != USER_CREDENTIALS_DUMMY_HANDLE {
                    panic!("{:?} is not {}", key, USER_CREDENTIALS_DUMMY_HANDLE);
                }
            }
        }
    }
    match EnvBoolVar::get_env_values_hashmap::<bool>(was_dotenv_enable) {
        Err(e) => panic!("cannot get bool env values hashmap, error: {:#?}", e),
        Ok(hashmap) => {
            for (key, value) in hashmap {
                if !value {
                    panic!("{:?} is not {}", key, true);
                }
            }
        }
    }
    match EnvU8Var::get_env_values_hashmap::<u8>(was_dotenv_enable) {
        Err(e) => panic!("cannot get u8 env values hashmap, error: {:#?}", e),
        Ok(hashmap) => {
            for (key, value) in hashmap {
                if value != 0 {
                    panic!("{:?} is not {}", key, 0);
                }
            }
        }
    }
    match EnvI64Var::get_env_values_hashmap::<i64>(was_dotenv_enable) {
        Err(e) => panic!("cannot get i64 env values hashmap, error: {:#?}", e),
        Ok(hashmap) => {
            for (key, value) in hashmap {
                if value != 0 {
                    panic!("{:?} is not {}", key, 0);
                }
            }
        }
    }
}
