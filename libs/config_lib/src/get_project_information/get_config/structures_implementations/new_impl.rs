use crate::get_project_information::get_config::config_struct::ConfigStruct;
use crate::get_project_information::project_constants::PROJECT_MODES;
use crate::get_project_information::project_constants::PROJECT_RUN_MODE_ENV_NAME;

use config::{Config, ConfigError, File};

use dotenv::dotenv;

impl ConfigStruct {
    pub fn new(mode_handler: Option<&str>, path_to_config: &str) -> Result<Self, ConfigError> {
        let mode_string: String;
        if let Some(mode) = mode_handler {
            mode_string = mode.to_string();
        } else {
            let dotenv_result = dotenv();
            match dotenv_result {
                Ok(_) => {
                    //working from console like "ENV_NAME=value cargo run" and from .env file
                    match std::env::var(PROJECT_RUN_MODE_ENV_NAME) {
                        Ok(mode) => {
                            let mut check: bool = false;
                            for project_mode in PROJECT_MODES {
                                if project_mode == &mode {
                                    check = true;
                                    break;
                                }
                            }
                            if check {
                                mode_string = mode;
                            } else {
                                return Err(ConfigError::Message(format!(
                                    "no such project_mode: {}",
                                    mode
                                )));
                            }
                        }
                        Err(e) => {
                            return Err(ConfigError::Message(format!(
                            "std::env::var(\"{}\") failed for console and .env file, error: {:#?}",
                            PROJECT_RUN_MODE_ENV_NAME, e
                        )))
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "dotenv() failed, trying to get PROJECT_RUN_MODE_ENV_NAME from console, error: {:#?}",
                        e
                    );
                    //working from console like "ENV_NAME=value cargo run" and from .env file
                    match std::env::var(PROJECT_RUN_MODE_ENV_NAME) {
                        Ok(mode) => {
                            let mut check: bool = false;
                            for project_mode in PROJECT_MODES {
                                if project_mode == &mode {
                                    check = true;
                                    break;
                                }
                            }
                            if check {
                                mode_string = mode;
                            } else {
                                return Err(ConfigError::Message(format!(
                                    "no such project_mode: {}",
                                    mode
                                )));
                            }
                        }
                        Err(e) => {
                            return Err(ConfigError::Message(format!(
                            "std::env::var(\"{}\") failed for console and .env file, error: {:#?}",
                            PROJECT_RUN_MODE_ENV_NAME, e
                        )))
                        }
                    }
                }
            }
        }
        println!("mode: {}", mode_string);
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
