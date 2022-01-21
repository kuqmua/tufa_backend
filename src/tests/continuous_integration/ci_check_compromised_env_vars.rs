use dotenv::dotenv;

use crate::traits::enum_extention::EnumExtenstion;

use crate::config_mods::common_env_var_enum::CommonEnvVar;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_env_vars() {
    let was_dotenv_enable = dotenv().is_ok();
    for env_var_kind in CommonEnvVar::into_array().iter() {
        env_var_kind.check_compromised_env_vars(was_dotenv_enable);
    }
}
