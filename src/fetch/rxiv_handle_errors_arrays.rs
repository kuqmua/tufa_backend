use crate::config::WARNING_LOGS_DIRECTORY_NAME;
use crate::fetch::handle_error_status_code::handle_error_status_code;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::fetch::rxiv_logs_create_dir_if_dont_exists::rxiv_logs_create_dir_if_dont_exists;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_warning_yellow;
use chrono::Local;
use reqwest::StatusCode;
use serde_json::json;
use std::collections::HashMap;
use std::{fs::File, io::Write};

#[allow(clippy::clippy::too_many_arguments)]
pub fn rxiv_handle_errors_arrays(
    rxiv_kind: RxivKind,
    enable_prints: bool,
    enable_warning_prints: bool,
    enable_error_prints: bool,
    unhandled_success_handled_success_are_there_items_initialized_posts: HashMap<String, String>,
    unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts: HashMap<
        String,
        (String, String),
    >,
    unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts: HashMap<
        String,
        (String, String, String),
    >,
    unhandled_success_handled_success_are_there_items_nope_no_tag_posts: HashMap<
        String,
        (String, String),
    >,
    unhandled_success_handled_initialized_posts: HashMap<String, String>,
    unhandled_success_handled_res_to_text_error_posts: HashMap<String, (String, String)>,
    unhandled_success_handled_res_status_error_posts: HashMap<String, (String, StatusCode)>,
    unhandled_initialized_posts: HashMap<String, String>,
    unhandled_failure_posts: HashMap<String, (String, String)>,
) {
    if !unhandled_success_handled_success_are_there_items_initialized_posts.is_empty()
        && enable_warning_prints
    {
        let underdirectory = "unhandled_success_handled_success_are_there_items_initialized_posts";
        if enable_warning_prints {
            let warning_message = format!(
                "{}.len() {}",
                underdirectory,
                unhandled_success_handled_success_are_there_items_initialized_posts.len()
            );
            print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
        }
        rxiv_logs_create_dir_if_dont_exists(
            underdirectory,
            &rxiv_kind,
            enable_prints,
            enable_error_prints,
        );
        for (key, value) in unhandled_success_handled_success_are_there_items_initialized_posts {
            //цикл должен быть асинхронным\паралельным
            let file_name = format!(
                "logs/{}/{}/{:?}/{:?}_{}.json",
                WARNING_LOGS_DIRECTORY_NAME,
                underdirectory,
                rxiv_kind,
                rxiv_kind,
                key.replace("/", "_")
            ); //подобрать функцию которая из стринги делает сейвовый путь
            let result_of_creating_file = File::create(file_name);
            match result_of_creating_file {
                Ok(mut created_for_logs_file) => {
                    let json_object = json!({
                        "topic": key,
                        "url": value,
                        "part_of": format!("{:?}", rxiv_kind),
                        "date": Local::now().to_string()
                    });
                    let result_of_stringify_json_object_pretty =
                        serde_json::to_string_pretty(&json_object);
                    match result_of_stringify_json_object_pretty {
                        Ok(string_json_prettyfied) => {
                            let result_of_writing =
                                created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
                            match result_of_writing {
                                Ok(_) => {
                                    if enable_prints {
                                        println!("логи записаны в файл для ключа {}", key);
                                    }
                                }
                                Err(e) => {
                                    if enable_error_prints {
                                        print_error_red(
                                            file!().to_string(),
                                            line!().to_string(),
                                            e.to_string(),
                                        )
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if enable_error_prints {
                                print_error_red(
                                    file!().to_string(),
                                    line!().to_string(),
                                    e.to_string(),
                                )
                            }
                        }
                    }
                }
                Err(e) => {
                    if enable_error_prints {
                        print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                    }
                }
            }
        }
    }
    if !unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts.is_empty()
        && enable_warning_prints
    {
        let underdirectory =
            "unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts";
        if enable_warning_prints {
            let warning_message = format!(
                "{}.len() {}",
                underdirectory,
                unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts.len()
            );
            print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
        }
        rxiv_logs_create_dir_if_dont_exists(
            underdirectory,
            &rxiv_kind,
            enable_prints,
            enable_error_prints,
        );
        for (key, value) in
            unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts
        {
            //цикл должен быть асинхронным\паралельным
            let file_name = format!(
                "logs/{}/{}/{:?}/{:?}_{}.json",
                WARNING_LOGS_DIRECTORY_NAME,
                underdirectory,
                rxiv_kind,
                rxiv_kind,
                key.replace("/", "_")
            ); //подобрать функцию которая из стринги делает сейвовый путь
            let result_of_creating_file = File::create(file_name);
            match result_of_creating_file {
                Ok(mut created_for_logs_file) => {
                    let json_object = json!({
                        "topic": key,
                        "url": value.0,
                        "fetch_result_string": value.1,
                        "part_of": format!("{:?}", rxiv_kind),
                        "date": Local::now().to_string()
                    });
                    let result_of_stringify_json_object_pretty =
                        serde_json::to_string_pretty(&json_object);
                    match result_of_stringify_json_object_pretty {
                        Ok(string_json_prettyfied) => {
                            let result_of_writing =
                                created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
                            match result_of_writing {
                                Ok(_) => {
                                    if enable_prints {
                                        println!("логи записаны в файл для ключа {}", key);
                                    }
                                }
                                Err(e) => {
                                    if enable_error_prints {
                                        print_error_red(
                                            file!().to_string(),
                                            line!().to_string(),
                                            e.to_string(),
                                        )
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if enable_error_prints {
                                print_error_red(
                                    file!().to_string(),
                                    line!().to_string(),
                                    e.to_string(),
                                )
                            }
                        }
                    }
                }
                Err(e) => {
                    if enable_error_prints {
                        print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                    }
                }
            }
        }
    }
    if !unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts.is_empty()
        && enable_warning_prints
    {
        let underdirectory =
            "unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts";
        if enable_warning_prints {
            let warning_message = format!(
                "{}.len() {}",
                underdirectory,
                unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts
                    .len()
            );
            print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
        }
        rxiv_logs_create_dir_if_dont_exists(
            underdirectory,
            &rxiv_kind,
            enable_prints,
            enable_error_prints,
        );
        for (key, value) in
            unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts
        {
            //цикл должен быть асинхронным\паралельным
            let file_name = format!(
                "logs/{}/{}/{:?}/{:?}_{}.json",
                WARNING_LOGS_DIRECTORY_NAME,
                underdirectory,
                rxiv_kind,
                rxiv_kind,
                key.replace("/", "_")
            ); //подобрать функцию которая из стринги делает сейвовый путь
            let result_of_creating_file = File::create(file_name);
            match result_of_creating_file {
                Ok(mut created_for_logs_file) => {
                    let json_object = json!({
                        "topic": key,
                        "url": value.0,
                        "fetch_result_string": value.1,
                        "error": value.2,
                        "part_of": format!("{:?}", rxiv_kind),
                        "date": Local::now().to_string()
                    });
                    let result_of_stringify_json_object_pretty =
                        serde_json::to_string_pretty(&json_object);
                    match result_of_stringify_json_object_pretty {
                        Ok(string_json_prettyfied) => {
                            let result_of_writing =
                                created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
                            match result_of_writing {
                                Ok(_) => {
                                    if enable_prints {
                                        println!("логи записаны в файл для ключа {}", key);
                                    }
                                }
                                Err(e) => {
                                    if enable_error_prints {
                                        print_error_red(
                                            file!().to_string(),
                                            line!().to_string(),
                                            e.to_string(),
                                        )
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if enable_error_prints {
                                print_error_red(
                                    file!().to_string(),
                                    line!().to_string(),
                                    e.to_string(),
                                )
                            }
                        }
                    }
                }
                Err(e) => {
                    if enable_error_prints {
                        print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                    }
                }
            }
        }
    }
    if !unhandled_success_handled_success_are_there_items_nope_no_tag_posts.is_empty()
        && enable_warning_prints
    {
        let underdirectory = "unhandled_success_handled_success_are_there_items_nope_no_tag_posts";
        if enable_warning_prints {
            let warning_message = format!(
                "{}.len() {}",
                underdirectory,
                unhandled_success_handled_success_are_there_items_nope_no_tag_posts.len()
            );
            print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
        }
        rxiv_logs_create_dir_if_dont_exists(
            underdirectory,
            &rxiv_kind,
            enable_prints,
            enable_error_prints,
        );
        for (key, value) in unhandled_success_handled_success_are_there_items_nope_no_tag_posts {
            //цикл должен быть асинхронным\паралельным
            let file_name = format!(
                "logs/{}/{}/{:?}/{:?}_{}.json",
                WARNING_LOGS_DIRECTORY_NAME,
                underdirectory,
                rxiv_kind,
                rxiv_kind,
                key.replace("/", "_")
            ); //подобрать функцию которая из стринги делает сейвовый путь
            let result_of_creating_file = File::create(file_name);
            match result_of_creating_file {
                Ok(mut created_for_logs_file) => {
                    let json_object = json!({
                        "topic": key,
                        "url": value.0,
                        "page_content": value.1,
                        "part_of": format!("{:?}", rxiv_kind),
                        "date": Local::now().to_string()
                    });
                    let result_of_stringify_json_object_pretty =
                        serde_json::to_string_pretty(&json_object);
                    match result_of_stringify_json_object_pretty {
                        Ok(string_json_prettyfied) => {
                            let result_of_writing =
                                created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
                            match result_of_writing {
                                Ok(_) => {
                                    if enable_prints {
                                        println!("логи записаны в файл для ключа {}", key);
                                    }
                                }
                                Err(e) => {
                                    println!("errr");
                                    if enable_error_prints {
                                        print_error_red(
                                            file!().to_string(),
                                            line!().to_string(),
                                            e.to_string(),
                                        )
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if enable_error_prints {
                                print_error_red(
                                    file!().to_string(),
                                    line!().to_string(),
                                    e.to_string(),
                                )
                            }
                        }
                    }
                }
                Err(e) => {
                    if enable_error_prints {
                        print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                    }
                }
            }
        }
    }
    if !unhandled_success_handled_initialized_posts.is_empty() && enable_warning_prints {
        let underdirectory = "unhandled_success_handled_initialized_posts";
        if enable_warning_prints {
            let warning_message = format!(
                "{}.len() {}",
                underdirectory,
                unhandled_success_handled_initialized_posts.len()
            );
            print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
        }
        rxiv_logs_create_dir_if_dont_exists(
            underdirectory,
            &rxiv_kind,
            enable_prints,
            enable_error_prints,
        );
        for (key, value) in unhandled_success_handled_initialized_posts {
            //цикл должен быть асинхронным\паралельным
            let file_name = format!(
                "logs/{}/{}/{:?}/{:?}_{}.json",
                WARNING_LOGS_DIRECTORY_NAME,
                underdirectory,
                rxiv_kind,
                rxiv_kind,
                key.replace("/", "_")
            ); //подобрать функцию которая из стринги делает сейвовый путь
            let result_of_creating_file = File::create(file_name);
            match result_of_creating_file {
                Ok(mut created_for_logs_file) => {
                    let json_object = json!({
                        "topic": key,
                        "url": value,
                        "part_of": format!("{:?}", rxiv_kind),
                        "date": Local::now().to_string()
                    });
                    let result_of_stringify_json_object_pretty =
                        serde_json::to_string_pretty(&json_object);
                    match result_of_stringify_json_object_pretty {
                        Ok(string_json_prettyfied) => {
                            let result_of_writing =
                                created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
                            match result_of_writing {
                                Ok(_) => {
                                    if enable_prints {
                                        println!("логи записаны в файл для ключа {}", key);
                                    }
                                }
                                Err(e) => {
                                    if enable_error_prints {
                                        print_error_red(
                                            file!().to_string(),
                                            line!().to_string(),
                                            e.to_string(),
                                        )
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if enable_error_prints {
                                print_error_red(
                                    file!().to_string(),
                                    line!().to_string(),
                                    e.to_string(),
                                )
                            }
                        }
                    }
                }
                Err(e) => {
                    if enable_error_prints {
                        print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                    }
                }
            }
        }
    }
    if !unhandled_success_handled_res_to_text_error_posts.is_empty() && enable_warning_prints {
        let underdirectory = "unhandled_success_handled_res_to_text_error_posts";
        if enable_warning_prints {
            let warning_message = format!(
                "{}.len() {}",
                underdirectory,
                unhandled_success_handled_res_to_text_error_posts.len()
            );
            print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
        }
        rxiv_logs_create_dir_if_dont_exists(
            underdirectory,
            &rxiv_kind,
            enable_prints,
            enable_error_prints,
        );
        for (key, value) in unhandled_success_handled_res_to_text_error_posts {
            //цикл должен быть асинхронным\паралельным
            let file_name = format!(
                "logs/{}/{}/{:?}/{:?}_{}.json",
                WARNING_LOGS_DIRECTORY_NAME,
                underdirectory,
                rxiv_kind,
                rxiv_kind,
                key.replace("/", "_")
            ); //подобрать функцию которая из стринги делает сейвовый путь
            let result_of_creating_file = File::create(file_name);
            match result_of_creating_file {
                Ok(mut created_for_logs_file) => {
                    let json_object = json!({
                        "topic": key,
                        "url": value.0,
                        "error": value.1,
                        "part_of": format!("{:?}", rxiv_kind),
                        "date": Local::now().to_string()
                    });
                    let result_of_stringify_json_object_pretty =
                        serde_json::to_string_pretty(&json_object);
                    match result_of_stringify_json_object_pretty {
                        Ok(string_json_prettyfied) => {
                            let result_of_writing =
                                created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
                            match result_of_writing {
                                Ok(_) => {
                                    if enable_prints {
                                        println!("логи записаны в файл для ключа {}", key);
                                    }
                                }
                                Err(e) => {
                                    if enable_error_prints {
                                        print_error_red(
                                            file!().to_string(),
                                            line!().to_string(),
                                            e.to_string(),
                                        )
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if enable_error_prints {
                                print_error_red(
                                    file!().to_string(),
                                    line!().to_string(),
                                    e.to_string(),
                                )
                            }
                        }
                    }
                }
                Err(e) => {
                    if enable_error_prints {
                        print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                    }
                }
            }
        }
    }
    if !unhandled_success_handled_res_status_error_posts.is_empty() && enable_warning_prints {
        let underdirectory = "unhandled_success_handled_res_status_error_posts";
        if enable_warning_prints {
            let warning_message = format!(
                "{}.len() {}",
                underdirectory,
                unhandled_success_handled_res_status_error_posts.len()
            );
            print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
        }
        rxiv_logs_create_dir_if_dont_exists(
            underdirectory,
            &rxiv_kind,
            enable_prints,
            enable_error_prints,
        );
        for (key, value) in unhandled_success_handled_res_status_error_posts {
            //цикл должен быть асинхронным\паралельным
            let file_name = format!(
                "logs/{}/{}/{:?}/{:?}_{}.json",
                WARNING_LOGS_DIRECTORY_NAME,
                underdirectory,
                rxiv_kind,
                rxiv_kind,
                key.replace("/", "_")
            ); //подобрать функцию которая из стринги делает сейвовый путь
            let result_of_creating_file = File::create(file_name);
            match result_of_creating_file {
                Ok(mut created_for_logs_file) => {
                    let json_object = json!({
                        "topic": key,
                        "url": value.0,
                        "status_code": value.1.to_string(),
                        "part_of": format!("{:?}", rxiv_kind),
                        "date": Local::now().to_string()
                    });
                    handle_error_status_code(value.1);
                    let result_of_stringify_json_object_pretty =
                        serde_json::to_string_pretty(&json_object);
                    match result_of_stringify_json_object_pretty {
                        Ok(string_json_prettyfied) => {
                            let result_of_writing =
                                created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
                            match result_of_writing {
                                Ok(_) => {
                                    if enable_prints {
                                        println!("логи записаны в файл для ключа {}", key);
                                    }
                                }
                                Err(e) => {
                                    if enable_error_prints {
                                        print_error_red(
                                            file!().to_string(),
                                            line!().to_string(),
                                            e.to_string(),
                                        )
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if enable_error_prints {
                                print_error_red(
                                    file!().to_string(),
                                    line!().to_string(),
                                    e.to_string(),
                                )
                            }
                        }
                    }
                }
                Err(e) => {
                    if enable_error_prints {
                        print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                    }
                }
            }
        }
    }
    if !unhandled_initialized_posts.is_empty() && enable_warning_prints {
        let underdirectory = "unhandled_initialized_posts";
        if enable_warning_prints {
            let warning_message = format!(
                "{}.len() {}",
                underdirectory,
                unhandled_initialized_posts.len()
            );
            print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
        }
        rxiv_logs_create_dir_if_dont_exists(
            underdirectory,
            &rxiv_kind,
            enable_prints,
            enable_error_prints,
        );
        for (key, value) in unhandled_initialized_posts {
            //цикл должен быть асинхронным\паралельным
            let file_name = format!(
                "logs/{}/{}/{:?}/{:?}_{}.json",
                WARNING_LOGS_DIRECTORY_NAME,
                underdirectory,
                rxiv_kind,
                rxiv_kind,
                key.replace("/", "_")
            ); //подобрать функцию которая из стринги делает сейвовый путь
            let result_of_creating_file = File::create(file_name);
            match result_of_creating_file {
                Ok(mut created_for_logs_file) => {
                    let json_object = json!({
                        "topic": key,
                        "url": value,
                        "part_of": format!("{:?}", rxiv_kind),
                        "date": Local::now().to_string()
                    });
                    let result_of_stringify_json_object_pretty =
                        serde_json::to_string_pretty(&json_object);
                    match result_of_stringify_json_object_pretty {
                        Ok(string_json_prettyfied) => {
                            let result_of_writing =
                                created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
                            match result_of_writing {
                                Ok(_) => {
                                    if enable_prints {
                                        println!("логи записаны в файл для ключа {}", key);
                                    }
                                }
                                Err(e) => {
                                    if enable_error_prints {
                                        print_error_red(
                                            file!().to_string(),
                                            line!().to_string(),
                                            e.to_string(),
                                        )
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if enable_error_prints {
                                print_error_red(
                                    file!().to_string(),
                                    line!().to_string(),
                                    e.to_string(),
                                )
                            }
                        }
                    }
                }
                Err(e) => {
                    if enable_error_prints {
                        print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                    }
                }
            }
        }
    }
    if !unhandled_failure_posts.is_empty() && enable_warning_prints {
        let underdirectory = "unhandled_failure_posts";
        if enable_warning_prints {
            let warning_message =
                format!("{}.len() {}", underdirectory, unhandled_failure_posts.len());
            print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
        }
        rxiv_logs_create_dir_if_dont_exists(
            underdirectory,
            &rxiv_kind,
            enable_prints,
            enable_error_prints,
        );
        for (key, value) in unhandled_failure_posts {
            //цикл должен быть асинхронным\паралельным
            let file_name = format!(
                "logs/{}/{}/{:?}/{:?}_{}.json",
                WARNING_LOGS_DIRECTORY_NAME,
                underdirectory,
                rxiv_kind,
                rxiv_kind,
                key.replace("/", "_")
            ); //подобрать функцию которая из стринги делает сейвовый путь
            let result_of_creating_file = File::create(file_name);
            match result_of_creating_file {
                Ok(mut created_for_logs_file) => {
                    let json_object = json!({
                        "topic": key,
                        "url": value.0,
                        "error": value.1,
                        "part_of": format!("{:?}", rxiv_kind),
                        "date": Local::now().to_string()
                    });
                    let result_of_stringify_json_object_pretty =
                        serde_json::to_string_pretty(&json_object);
                    match result_of_stringify_json_object_pretty {
                        Ok(string_json_prettyfied) => {
                            let result_of_writing =
                                created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
                            match result_of_writing {
                                Ok(_) => {
                                    if enable_prints {
                                        println!("логи записаны в файл для ключа {}", key);
                                    }
                                }
                                Err(e) => {
                                    if enable_error_prints {
                                        print_error_red(
                                            file!().to_string(),
                                            line!().to_string(),
                                            e.to_string(),
                                        )
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if enable_error_prints {
                                print_error_red(
                                    file!().to_string(),
                                    line!().to_string(),
                                    e.to_string(),
                                )
                            }
                        }
                    }
                }
                Err(e) => {
                    if enable_error_prints {
                        print_error_red(file!().to_string(), line!().to_string(), e.to_string())
                    }
                }
            }
        }
    }
}
