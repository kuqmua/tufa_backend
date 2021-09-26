use crate::get_project_information::get_config::config_struct::ConfigStruct;
use crate::get_project_information::project_constants::LOAD_CONFIG_FILE_ERROR_MESSAGE;
use crate::get_project_information::project_constants::PATH_TO_CONFIG;

lazy_static! {
    pub static ref CONFIG: ConfigStruct =
        ConfigStruct::new(None, PATH_TO_CONFIG).expect(LOAD_CONFIG_FILE_ERROR_MESSAGE);
}
