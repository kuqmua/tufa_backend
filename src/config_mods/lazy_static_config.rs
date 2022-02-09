use crate::traits::wrap_config_checks_trait::WrapConfigChecks;
use crate::config_mods::config_struct::ConfigStruct;
use crate::constants::project_constants::LOAD_CONFIG_FILE_ERROR_MESSAGE;

lazy_static! {
    pub static ref CONFIG: ConfigStruct =
        ConfigStruct::new().expect(LOAD_CONFIG_FILE_ERROR_MESSAGE).wrap_config_checks().expect("wrap_config_checks error");
}
