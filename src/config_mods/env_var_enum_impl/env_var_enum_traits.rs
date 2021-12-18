use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::enum_extention::EnumExtenstion;
use crate::traits::env_var_trait::EnvVarTrait;

impl EnvVarTrait for EnvVar {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_env_name(&self) -> String {
        self.to_upper_snake_case()
    }
}
