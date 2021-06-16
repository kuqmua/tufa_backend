use serde_json::Value;

use crate::overriding::prints::print_error_red;
use std::{fs::File, io::Write};

pub fn rss_write_error_logs_into_file_for_provider(
    enable_prints: bool,
    enable_error_prints: bool,
    file_name: String,
    json_object: Value,
) {
    let result_of_creating_file = File::create(&file_name);
    match result_of_creating_file {
        Ok(mut created_for_logs_file) => {
            let result_of_stringify_json_object_pretty = serde_json::to_string_pretty(&json_object);
            match result_of_stringify_json_object_pretty {
                Ok(string_json_prettyfied) => {
                    let result_of_writing =
                        created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
                    match result_of_writing {
                        Ok(_) => {
                            if enable_prints {
                                println!("logs were written in file {}", &file_name);
                            }
                        }
                        Err(e) => {
                            let message =
                                format!("error writing in file {} {}", file_name, e.to_string());
                            if enable_error_prints {
                                print_error_red(file!().to_string(), line!().to_string(), message)
                            }
                        }
                    }
                }
                Err(e) => {
                    let message = format!(
                        "error cast json into string {} {}",
                        &json_object,
                        e.to_string()
                    );
                    if enable_error_prints {
                        print_error_red(file!().to_string(), line!().to_string(), message)
                    }
                }
            }
        }
        Err(e) => {
            let message = format!("error creating file {} {}", &file_name, e.to_string());
            if enable_error_prints {
                print_error_red(file!().to_string(), line!().to_string(), message)
            }
        }
    }
}
