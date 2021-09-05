use crate::get_project_information::get_config::structures_definitions::config_struct_def::ConfigStruct;
use crate::get_project_information::project_constants::PROJECT_MODE;

use config::{Config, ConfigError, File};

impl ConfigStruct {
    pub fn new(mode_handler: Option<&str>, path_to_config: &str) -> Result<Self, ConfigError> {
        let mode_string: String;
        if let Some(mode) = mode_handler {
            mode_string = mode.to_string();
        } else {
            mode_string = std::env::var("RUN_ENV").unwrap_or_else(|_| PROJECT_MODE.into());
        }
        let mut config = Config::new();
        match config.set("env", mode_string.clone()) {
            Ok(config_set_env_ok) => {
                match config_set_env_ok.merge(File::with_name(&format!(
                    "{}{}",
                    path_to_config, mode_string
                ))) {
                    Ok(_) => {
                        match config.try_into() {
                            Ok(config_handle) => ConfigStruct::wrap_config_checks(config_handle),
                            Err(e) => {
                                //cannot use print_colorful_message coz circular dependency
                                println!(
                                    "{}{}\nconfig.try_into error: {:#?}",
                                    file!().to_string(),
                                    line!().to_string(),
                                    e
                                );
                                Err(e)
                            }
                        }
                    }
                    Err(e) => {
                        //cannot use print_colorful_message coz circular dependency
                        println!(
                            "{}{}\nconfig.merge(File::with_name({}{})) error: {:#?}",
                            file!().to_string(),
                            line!().to_string(),
                            path_to_config,
                            mode_string,
                            e
                        );
                        Err(e)
                    }
                }
            }
            Err(e) => {
                //cannot use print_colorful_message coz circular dependency
                println!(
                    "{}{}\nconfig.set(\"env\", {}) error: {:#?}",
                    file!().to_string(),
                    line!().to_string(),
                    mode_string,
                    e
                );
                Err(e)
            }
        }
    }
}
