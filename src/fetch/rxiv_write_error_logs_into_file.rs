use serde_json::Value;

use crate::config::WARNING_LOGS_DIRECTORY_NAME;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::fetch::rxiv_logs_create_dir_if_dont_exists::rxiv_logs_create_dir_if_dont_exists;
use crate::overriding::prints::print_error_red;
use std::{fs::File, io::Write};

pub fn rxiv_write_error_logs_into_file(
    json_object: Value,
    rxiv_kind: &RxivKind,
    dir: &str,
    enable_prints: bool,
    enable_error_prints: bool,
    key: String,
) {
    rxiv_logs_create_dir_if_dont_exists(dir, &rxiv_kind, enable_prints, enable_error_prints);
    let file_name = format!(
        "logs/{}/{:?}/{}/{:?}_{}.json",
        WARNING_LOGS_DIRECTORY_NAME,
        rxiv_kind,
        dir,
        rxiv_kind,
        key.replace("/", "_")
    ); //подобрать функцию которая из стринги делает сейвовый путь

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
                                println!("логи записаны в файл {}", &file_name);
                            }
                        }
                        Err(e) => {
                            let message =
                                format!("ошибка записи в файл {} {}", file_name, e.to_string());
                            if enable_error_prints {
                                print_error_red(file!().to_string(), line!().to_string(), message)
                            }
                        }
                    }
                }
                Err(e) => {
                    let message = format!(
                        "ошибка каста json в string {} {}",
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
            let message = format!("ошибка создания файла {} {}", &file_name, e.to_string());
            if enable_error_prints {
                print_error_red(file!().to_string(), line!().to_string(), message)
            }
        }
    }
}
