use serde_json::Value;

use crate::fetch::rss_logs_create_dir_if_dont_exists::rss_logs_create_dir_if_dont_exists;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_warning_orange;
use std::error::Error;
use std::path;
use std::{fs::File, io::Write};

pub fn write_into_file(
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

pub fn rss_write_error_logs_into_file_for_provider(
    json_object: Value,
    provider_kind: &ProviderKind,
    dir: &str,
    enable_prints: bool,
    enable_error_prints: bool,
    warning_logs_directory_name: &str,
    link: &str,
) {
    rss_logs_create_dir_if_dont_exists(
        dir,
        &provider_kind,
        enable_prints,
        enable_error_prints,
        &warning_logs_directory_name,
    );
    let replaced_link = link.replace("/", "-").replace(":", "-").replace(".", "-");
    let file_name = format!(
        "logs/{}/{:?}/{}/{:?}-{}.json",
        &warning_logs_directory_name, provider_kind, dir, provider_kind, replaced_link
    ); //add save function what convert string into save path
    let result_of_opening_file = File::open(&file_name);
    match result_of_opening_file {
        Ok(_) => {
            let warning_message = format!("there is file with the same name: {}", &file_name);
            print_warning_orange(file!().to_string(), line!().to_string(), warning_message)
        }
        Err(ref err) => {
            if err.description() == "entity not found" {
                write_into_file(enable_prints, enable_error_prints, file_name, json_object);
            } else {
                let warning_message = format!(
                    "unexpected error while opening file, description: {:#?}",
                    &err.description()
                );
                print_warning_orange(file!().to_string(), line!().to_string(), warning_message);
                write_into_file(enable_prints, enable_error_prints, file_name, json_object);
            }
            // println!("description {:#?}", ffs.description());
            // println!("ERRROR      {:#?}", error);
        }
    }
}
