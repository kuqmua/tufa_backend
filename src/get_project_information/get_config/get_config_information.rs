use crate::get_project_information::get_config::config_structures::ConfigStruct;

lazy_static! {
    pub static ref CONFIG: ConfigStruct = ConfigStruct::new().expect("config can be loaded");
}
