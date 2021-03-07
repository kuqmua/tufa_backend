use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_warning_yellow;
use chrono::Local;
use reqwest::StatusCode;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
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
        let warning_message =
            "unhandled_success_handled_success_are_there_items_initialized_posts.len() "
                .to_string()
                + &unhandled_success_handled_success_are_there_items_initialized_posts
                    .len()
                    .to_string();
        print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
    }
    if !unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts.is_empty()
        && enable_warning_prints
    {
        let warning_message =
            "unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts.len() "
                .to_string()
                + &unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts
                    .len()
                    .to_string();
        print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
    }
    if !unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts.is_empty()
        && enable_warning_prints
    {
        let warning_message = "unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts.len() ".to_string()
                    + &unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts.len().to_string();
        print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
    }
    if !unhandled_success_handled_success_are_there_items_nope_no_tag_posts.is_empty()
        && enable_warning_prints
    {
        let warning_message =
            "unhandled_success_handled_success_are_there_items_nope_no_tag_posts.len() "
                .to_string()
                + &unhandled_success_handled_success_are_there_items_nope_no_tag_posts
                    .len()
                    .to_string();
        print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
        let g = "logs/warning_logs/unhandled_success_handled_success_are_there_items_nope_no_tag_posts/".to_string();
        println!("ffff {}", g);
        if !Path::new(&g).exists() {
            fs::create_dir(g).unwrap();
        }
        let f = format!("logs/warning_logs/unhandled_success_handled_success_are_there_items_nope_no_tag_posts/{:?}", rxiv_kind);
        println!("ffff {}", f);
        if !Path::new(&f).exists() {
            fs::create_dir(f).unwrap();
        }
        println!("after create");
        for (key, value) in unhandled_success_handled_success_are_there_items_nope_no_tag_posts {
            //цикл должен быть асинхронным\паралельным
            let file_name = format!(
                "logs/warning_logs/unhandled_success_handled_success_are_there_items_nope_no_tag_posts/{:?}/{:?}_{}.json",
                rxiv_kind,
                rxiv_kind,
                key.replace("/", "_")
            ); //подобрать функцию которая из стринги делает сейвовый путь
            let mut fileonos = File::create(file_name).expect("could not create file");
            let json_object = json!({
                "topic": key,
                "url": value.0,
                "page_content": value.1,
                "part_of": format!("{:?}", rxiv_kind),
                "date": Local::now().to_string()
            });
            let result_of_stringify_json_object_pretty = serde_json::to_string_pretty(&json_object);
            match result_of_stringify_json_object_pretty {
                Ok(string_json_prettyfied) => {
                    let result_of_writing = fileonos.write(string_json_prettyfied.as_bytes()); //warning_message
                    match result_of_writing {
                        Ok(_) => {
                            if enable_prints {
                                println!("записано");
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
        let warning_message = "unhandled_success_handled_initialized_posts.len() ".to_string()
            + &unhandled_success_handled_initialized_posts
                .len()
                .to_string();

        print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
    }
    if !unhandled_success_handled_res_to_text_error_posts.is_empty() && enable_warning_prints {
        let warning_message = "unhandled_success_handled_res_to_text_error_posts.len() "
            .to_string()
            + &unhandled_success_handled_res_to_text_error_posts
                .len()
                .to_string();
        print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
    }
    if !unhandled_success_handled_res_status_error_posts.is_empty() && enable_warning_prints {
        let warning_message = "unhandled_success_handled_res_status_error_posts.len() ".to_string()
            + &unhandled_success_handled_res_status_error_posts
                .len()
                .to_string();
        print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
    }
    if !unhandled_initialized_posts.is_empty() && enable_warning_prints {
        let warning_message = "unhandled_initialized_posts.len() ".to_string()
            + &unhandled_initialized_posts.len().to_string();
        print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
    }
    if !unhandled_failure_posts.is_empty() && enable_warning_prints {
        let warning_message = "unhandled_failure_posts.len() ".to_string()
            + &unhandled_failure_posts.len().to_string();
        print_warning_yellow(file!().to_string(), line!().to_string(), warning_message)
    }
}
