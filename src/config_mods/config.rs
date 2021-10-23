use crate::constants::project_constants::LOAD_CONFIG_FILE_ERROR_MESSAGE;
use crate::config_mods::get_config::config_struct::ConfigStruct;

lazy_static! {
    pub static ref CONFIG: ConfigStruct =
        ConfigStruct::new().expect(LOAD_CONFIG_FILE_ERROR_MESSAGE);
}
