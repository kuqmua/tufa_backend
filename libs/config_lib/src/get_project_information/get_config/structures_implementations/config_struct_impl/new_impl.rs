use crate::get_project_information::get_config::structures_definitions::config_struct_def::ConfigStruct;
use crate::get_project_information::project_constants::PROJECT_MODE;

use config::{Config, ConfigError, File};

impl ConfigStruct {
    pub fn new(
        &self,
        mode_handler: Option<&str>,
        path_to_config: &str,
    ) -> Result<Self, ConfigError> {
        match mode_handler {
            Some(mode) => {
                //for tests - maybe remove and copy code for testing later but its more comfortable for now
                let mut config = Config::new();
                match config.set("env", mode) {
                    Ok(_) => {
                        match config.merge(File::with_name(&format!("{}{}", path_to_config, mode)))
                        {
                            Ok(_) => {
                                let config_result: Result<Self, ConfigError> = config.try_into();
                                match config_result {
                                    Ok(config_handle) => {
                                        ConfigStruct::wrap_config_checks(&self, config_handle)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            None => {
                // RUN_ENV=Testing cargo run
                let env = std::env::var("RUN_ENV").unwrap_or_else(|_| PROJECT_MODE.into());
                let mut config = Config::new();
                match config.set("env", env.clone()) {
                    Ok(_) => {
                        match config.merge(File::with_name(&format!("{}{}", path_to_config, env))) {
                            Ok(_) => {
                                let config_result: Result<Self, ConfigError> = config.try_into();
                                match config_result {
                                    Ok(config_handle) => {
                                        ConfigStruct::wrap_config_checks(&self, config_handle)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}
