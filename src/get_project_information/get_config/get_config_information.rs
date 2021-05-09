use crate::get_project_information::get_config::config_structures::Config;
use crate::overriding::prints::print_error_red;
use std::fs::File;
use std::io::prelude::*;

pub fn get_config_information() -> Option<Config> {
    let result_of_file_opening = File::open("././Cargo.toml");
    match result_of_file_opening {
        Ok(mut file) => {
            let mut string_file_content = String::new();
            let result_of_writing_to_string_from_file =
                file.read_to_string(&mut string_file_content);
            match result_of_writing_to_string_from_file {
                Ok(_) => {
                    let result_of_convertion_to_config_struct: std::result::Result<
                        Config,
                        toml::de::Error,
                    > = toml::from_str(&string_file_content);
                    match result_of_convertion_to_config_struct {
                        Ok(config) => Some(config),
                        Err(error) => {
                            let error_message =
                                format!("result_of_convertion_to_config_struct error {}", error);
                            print_error_red(
                                file!().to_string(),
                                line!().to_string(),
                                error_message,
                            );
                            None
                        }
                    }
                }
                Err(error) => {
                    let error_message =
                        format!("result_of_writing_to_string_from_file error {}", error);
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                    None
                }
            }
        }
        Err(error) => {
            let error_message = format!("result_of_file_opening error {}", error);
            print_error_red(file!().to_string(), line!().to_string(), error_message);
            None
        }
    }
}
