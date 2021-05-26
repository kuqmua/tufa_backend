use crate::get_project_information::get_config::config_structures::ConfigStruct;

lazy_static! {
    pub static ref CONFIG: ConfigStruct = ConfigStruct::new().expect("сan not load config file");
}
