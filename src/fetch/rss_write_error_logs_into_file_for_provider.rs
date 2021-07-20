use serde_json::Value;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use std::{fs::File, io::Write};

pub fn rss_write_error_logs_into_file_for_provider(file_name: String, json_object: Value) {
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
                            print_colorful_message(
                                None,
                                PrintType::Success,
                                file!().to_string(),
                                line!().to_string(),
                                format!("logs were written in file {}", &file_name),
                            );
                        }
                        Err(e) => {
                            print_colorful_message(
                                None,
                                PrintType::Error,
                                file!().to_string(),
                                line!().to_string(),
                                format!("error writing in file {} {}", file_name, e.to_string()),
                            );
                        }
                    }
                }
                Err(e) => {
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        format!(
                            "error cast json into string {} {}",
                            &json_object,
                            e.to_string()
                        ),
                    );
                }
            }
        }
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!("error creating file {} {}", &file_name, e.to_string()),
            );
        }
    }
}
