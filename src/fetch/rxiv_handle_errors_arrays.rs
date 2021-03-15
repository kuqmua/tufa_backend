use crate::config::WARNING_LOGS_DIRECTORY_NAME;
use crate::fetch::handle_error_status_code::handle_error_status_code;
use crate::fetch::metainfo_fetch_structures::AreThereItems;
use crate::fetch::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::fetch::rxiv_logs_create_dir_if_dont_exists::rxiv_logs_create_dir_if_dont_exists;
use crate::fetch::rxiv_structures::RxivPostStruct;
use crate::overriding::prints::print_error_red;
// use crate::overriding::prints::print_warning_yellow;
use chrono::Local;
use serde_json::json;
use std::collections::HashMap;
use std::{fs::File, io::Write};

#[allow(clippy::clippy::too_many_arguments)]
pub fn rxiv_handle_errors_arrays(
    rxiv_kind: RxivKind,
    enable_prints: bool,
    enable_warning_prints: bool,
    enable_error_prints: bool,
    some_error_posts: HashMap<
        String,
        (
            RxivPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
            RxivKind,
        ),
    >,
) {
    let underdirectory = "unhandled_success_handled_success_are_there_items_initialized_posts";
    for (key, value) in some_error_posts {
        match value.2 {
            UnhandledFetchStatusInfo::Success => match value.3 {
                HandledFetchStatusInfo::Success => match value.4 {
                    AreThereItems::Yep => (),
                    AreThereItems::Initialized => {
                        // unhandled_success_handled_success_are_there_items_initialized_posts
                        //     .insert(key, value.1);

                        rxiv_logs_create_dir_if_dont_exists(
                            underdirectory,
                            &rxiv_kind,
                            enable_prints,
                            enable_error_prints,
                        );
                        let file_name = format!(
                            "logs/{}/{:?}/{}/{:?}_{}.json",
                            WARNING_LOGS_DIRECTORY_NAME,
                            rxiv_kind,
                            underdirectory,
                            rxiv_kind,
                            key.replace("/", "_")
                        ); //подобрать функцию которая из стринги делает сейвовый путь
                        let result_of_creating_file = File::create(&file_name);
                        match result_of_creating_file {
                            Ok(mut created_for_logs_file) => {
                                let json_object = json!({
                                    "topic": key,
                                    "url": value.1,
                                    "part_of": format!("{:?}", rxiv_kind),
                                    "date": Local::now().to_string()
                                });
                                let result_of_stringify_json_object_pretty =
                                    serde_json::to_string_pretty(&json_object);
                                match result_of_stringify_json_object_pretty {
                                    Ok(string_json_prettyfied) => {
                                        let result_of_writing = created_for_logs_file
                                            .write(string_json_prettyfied.as_bytes()); //warning_message
                                        match result_of_writing {
                                            Ok(_) => {
                                                if enable_prints {
                                                    println!("логи записаны в файл {}", &file_name);
                                                }
                                            }
                                            Err(e) => {
                                                let message = format!(
                                                    "ошибка записи в файл {} {}",
                                                    file_name,
                                                    e.to_string()
                                                );
                                                if enable_error_prints {
                                                    print_error_red(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        message,
                                                    )
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
                                            print_error_red(
                                                file!().to_string(),
                                                line!().to_string(),
                                                message,
                                            )
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                let message = format!(
                                    "ошибка создания файла {} {}",
                                    &file_name,
                                    e.to_string()
                                );
                                if enable_error_prints {
                                    print_error_red(
                                        file!().to_string(),
                                        line!().to_string(),
                                        message,
                                    )
                                }
                            }
                        }
                    }
                    AreThereItems::NopeButThereIsTag(fetch_result_string) => {
                        //"</item>" tag
                        // unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts
                        //         .insert(key, (value.1, fetch_result_string));
                    }
                    AreThereItems::ConversionFromStrError(fetch_result_string, error) => {
                        // unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts
                        //         .insert(key, (value.1, fetch_result_string, error));
                    }
                    AreThereItems::NopeNoTag(fetch_result_string) => {
                        // unhandled_success_handled_success_are_there_items_nope_no_tag_posts
                        //     .insert(key, (value.1, fetch_result_string));
                    }
                },
                HandledFetchStatusInfo::Initialized => {
                    // unhandled_success_handled_initialized_posts.insert(key, value.1);
                }
                HandledFetchStatusInfo::ResToTextError(error) => {
                    // unhandled_success_handled_res_to_text_error_posts.insert(key, (value.1, error));
                }
                HandledFetchStatusInfo::ResStatusError(status_code) => {
                    // let should_refetch_it = handle_error_status_code(status_code);
                    // unhandled_success_handled_res_status_error_posts
                    //     .insert(key, (value.1, status_code));
                }
            },
            UnhandledFetchStatusInfo::Initialized => {
                // unhandled_initialized_posts.insert(key, value.1);
            }
            UnhandledFetchStatusInfo::Failure(box_dyn_error) => {
                // unhandled_failure_posts.insert(key, (value.1, box_dyn_error));
            }
        }
    }
    ///////////////////////////
    // if !unhandled_success_handled_success_are_there_items_initialized_posts.is_empty()
    //     && enable_warning_prints
    // {
    //     let underdirectory = "unhandled_success_handled_success_are_there_items_initialized_posts";
    //     if enable_warning_prints {
    //         let warning_message = format!(
    //             "{}.len() {}",
    //             underdirectory,
    //             unhandled_success_handled_success_are_there_items_initialized_posts.len()
    //         );
    //         print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
    //     }
    //     rxiv_logs_create_dir_if_dont_exists(
    //         underdirectory,
    //         &rxiv_kind,
    //         enable_prints,
    //         enable_error_prints,
    //     );
    //     for (key, value) in unhandled_success_handled_success_are_there_items_initialized_posts {
    //         //цикл должен быть асинхронным\паралельным
    //         let file_name = format!(
    //             "logs/{}/{:?}/{}/{:?}_{}.json",
    //             WARNING_LOGS_DIRECTORY_NAME,
    //             rxiv_kind,
    //             underdirectory,
    //             rxiv_kind,
    //             key.replace("/", "_")
    //         ); //подобрать функцию которая из стринги делает сейвовый путь
    //         let result_of_creating_file = File::create(&file_name);
    //         match result_of_creating_file {
    //             Ok(mut created_for_logs_file) => {
    //                 let json_object = json!({
    //                     "topic": key,
    //                     "url": value,
    //                     "part_of": format!("{:?}", rxiv_kind),
    //                     "date": Local::now().to_string()
    //                 });
    //                 let result_of_stringify_json_object_pretty =
    //                     serde_json::to_string_pretty(&json_object);
    //                 match result_of_stringify_json_object_pretty {
    //                     Ok(string_json_prettyfied) => {
    //                         let result_of_writing =
    //                             created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
    //                         match result_of_writing {
    //                             Ok(_) => {
    //                                 if enable_prints {
    //                                     println!("логи записаны в файл {}", &file_name);
    //                                 }
    //                             }
    //                             Err(e) => {
    //                                 let message = format!(
    //                                     "ошибка записи в файл {} {}",
    //                                     file_name,
    //                                     e.to_string()
    //                                 );
    //                                 if enable_error_prints {
    //                                     print_error_red(
    //                                         file!().to_string(),
    //                                         line!().to_string(),
    //                                         message,
    //                                     )
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     Err(e) => {
    //                         let message = format!(
    //                             "ошибка каста json в string {} {}",
    //                             &json_object,
    //                             e.to_string()
    //                         );
    //                         if enable_error_prints {
    //                             print_error_red(file!().to_string(), line!().to_string(), message)
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 let message = format!("ошибка создания файла {} {}", &file_name, e.to_string());
    //                 if enable_error_prints {
    //                     print_error_red(file!().to_string(), line!().to_string(), message)
    //                 }
    //             }
    //         }
    //     }
    // }
    // if !unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts.is_empty()
    //     && enable_warning_prints
    // {
    //     let underdirectory =
    //         "unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts";
    //     if enable_warning_prints {
    //         let warning_message = format!(
    //             "{}.len() {}",
    //             underdirectory,
    //             unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts.len()
    //         );
    //         print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
    //     }
    //     rxiv_logs_create_dir_if_dont_exists(
    //         underdirectory,
    //         &rxiv_kind,
    //         enable_prints,
    //         enable_error_prints,
    //     );
    //     for (key, value) in
    //         unhandled_success_handled_success_are_there_items_no_but_there_is_a_tag_posts
    //     {
    //         //цикл должен быть асинхронным\паралельным
    //         let file_name = format!(
    //             "logs/{}/{:?}/{}/{:?}_{}.json",
    //             WARNING_LOGS_DIRECTORY_NAME,
    //             rxiv_kind,
    //             underdirectory,
    //             rxiv_kind,
    //             key.replace("/", "_")
    //         ); //подобрать функцию которая из стринги делает сейвовый путь
    //         let result_of_creating_file = File::create(&file_name);
    //         match result_of_creating_file {
    //             Ok(mut created_for_logs_file) => {
    //                 let json_object = json!({
    //                     "topic": key,
    //                     "url": value.0,
    //                     "fetch_result_string": value.1,
    //                     "part_of": format!("{:?}", rxiv_kind),
    //                     "date": Local::now().to_string()
    //                 });
    //                 let result_of_stringify_json_object_pretty =
    //                     serde_json::to_string_pretty(&json_object);
    //                 match result_of_stringify_json_object_pretty {
    //                     Ok(string_json_prettyfied) => {
    //                         let result_of_writing =
    //                             created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
    //                         match result_of_writing {
    //                             Ok(_) => {
    //                                 if enable_prints {
    //                                     println!("логи записаны в файл {}", &file_name);
    //                                 }
    //                             }
    //                             Err(e) => {
    //                                 let message = format!(
    //                                     "ошибка записи в файл {} {}",
    //                                     file_name,
    //                                     e.to_string()
    //                                 );
    //                                 if enable_error_prints {
    //                                     print_error_red(
    //                                         file!().to_string(),
    //                                         line!().to_string(),
    //                                         message,
    //                                     )
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     Err(e) => {
    //                         let message = format!(
    //                             "ошибка каста json в string {} {}",
    //                             &json_object,
    //                             e.to_string()
    //                         );
    //                         if enable_error_prints {
    //                             print_error_red(file!().to_string(), line!().to_string(), message)
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 let message = format!("ошибка создания файла {} {}", &file_name, e.to_string());
    //                 if enable_error_prints {
    //                     print_error_red(file!().to_string(), line!().to_string(), message)
    //                 }
    //             }
    //         }
    //     }
    // }
    // if !unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts.is_empty()
    //     && enable_warning_prints
    // {
    //     let underdirectory =
    //         "unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts";
    //     if enable_warning_prints {
    //         let warning_message = format!(
    //             "{}.len() {}",
    //             underdirectory,
    //             unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts
    //                 .len()
    //         );
    //         print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
    //     }
    //     rxiv_logs_create_dir_if_dont_exists(
    //         underdirectory,
    //         &rxiv_kind,
    //         enable_prints,
    //         enable_error_prints,
    //     );
    //     for (key, value) in
    //         unhandled_success_handled_success_are_there_items_conversion_from_str_error_posts
    //     {
    //         //цикл должен быть асинхронным\паралельным
    //         let file_name = format!(
    //             "logs/{}/{:?}/{}/{:?}_{}.json",
    //             WARNING_LOGS_DIRECTORY_NAME,
    //             rxiv_kind,
    //             underdirectory,
    //             rxiv_kind,
    //             key.replace("/", "_")
    //         ); //подобрать функцию которая из стринги делает сейвовый путь
    //         let result_of_creating_file = File::create(&file_name);
    //         match result_of_creating_file {
    //             Ok(mut created_for_logs_file) => {
    //                 let json_object = json!({
    //                     "topic": key,
    //                     "url": value.0,
    //                     "fetch_result_string": value.1,
    //                     "error": value.2,
    //                     "part_of": format!("{:?}", rxiv_kind),
    //                     "date": Local::now().to_string()
    //                 });
    //                 let result_of_stringify_json_object_pretty =
    //                     serde_json::to_string_pretty(&json_object);
    //                 match result_of_stringify_json_object_pretty {
    //                     Ok(string_json_prettyfied) => {
    //                         let result_of_writing =
    //                             created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
    //                         match result_of_writing {
    //                             Ok(_) => {
    //                                 if enable_prints {
    //                                     println!("логи записаны в файл {}", &file_name);
    //                                 }
    //                             }
    //                             Err(e) => {
    //                                 let message = format!(
    //                                     "ошибка записи в файл {} {}",
    //                                     file_name,
    //                                     e.to_string()
    //                                 );
    //                                 if enable_error_prints {
    //                                     print_error_red(
    //                                         file!().to_string(),
    //                                         line!().to_string(),
    //                                         message,
    //                                     )
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     Err(e) => {
    //                         let message = format!(
    //                             "ошибка каста json в string {} {}",
    //                             &json_object,
    //                             e.to_string()
    //                         );
    //                         if enable_error_prints {
    //                             print_error_red(file!().to_string(), line!().to_string(), message)
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 let message = format!("ошибка создания файла {} {}", &file_name, e.to_string());
    //                 if enable_error_prints {
    //                     print_error_red(file!().to_string(), line!().to_string(), message)
    //                 }
    //             }
    //         }
    //     }
    // }
    // if !unhandled_success_handled_success_are_there_items_nope_no_tag_posts.is_empty()
    //     && enable_warning_prints
    // {
    //     let underdirectory = "unhandled_success_handled_success_are_there_items_nope_no_tag_posts";
    //     if enable_warning_prints {
    //         let warning_message = format!(
    //             "{}.len() {}",
    //             underdirectory,
    //             unhandled_success_handled_success_are_there_items_nope_no_tag_posts.len()
    //         );
    //         print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
    //     }
    //     rxiv_logs_create_dir_if_dont_exists(
    //         underdirectory,
    //         &rxiv_kind,
    //         enable_prints,
    //         enable_error_prints,
    //     );
    //     for (key, value) in unhandled_success_handled_success_are_there_items_nope_no_tag_posts {
    //         //цикл должен быть асинхронным\паралельным
    //         let file_name = format!(
    //             "logs/{}/{:?}/{}/{:?}_{}.json",
    //             WARNING_LOGS_DIRECTORY_NAME,
    //             rxiv_kind,
    //             underdirectory,
    //             rxiv_kind,
    //             key.replace("/", "_")
    //         ); //подобрать функцию которая из стринги делает сейвовый путь
    //         let result_of_creating_file = File::create(&file_name);
    //         match result_of_creating_file {
    //             Ok(mut created_for_logs_file) => {
    //                 let json_object = json!({
    //                     "topic": key,
    //                     "url": value.0,
    //                     "page_content": value.1,
    //                     "part_of": format!("{:?}", rxiv_kind),
    //                     "date": Local::now().to_string()
    //                 });
    //                 let result_of_stringify_json_object_pretty =
    //                     serde_json::to_string_pretty(&json_object);
    //                 match result_of_stringify_json_object_pretty {
    //                     Ok(string_json_prettyfied) => {
    //                         let result_of_writing =
    //                             created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
    //                         match result_of_writing {
    //                             Ok(_) => {
    //                                 if enable_prints {
    //                                     println!("логи записаны в файл {}", &file_name);
    //                                 }
    //                             }
    //                             Err(e) => {
    //                                 let message = format!(
    //                                     "ошибка записи в файл {} {}",
    //                                     file_name,
    //                                     e.to_string()
    //                                 );
    //                                 if enable_error_prints {
    //                                     print_error_red(
    //                                         file!().to_string(),
    //                                         line!().to_string(),
    //                                         message,
    //                                     )
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     Err(e) => {
    //                         let message = format!(
    //                             "ошибка каста json в string {} {}",
    //                             &json_object,
    //                             e.to_string()
    //                         );
    //                         if enable_error_prints {
    //                             print_error_red(file!().to_string(), line!().to_string(), message)
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 let message = format!("ошибка создания файла {} {}", &file_name, e.to_string());
    //                 if enable_error_prints {
    //                     print_error_red(file!().to_string(), line!().to_string(), message)
    //                 }
    //             }
    //         }
    //     }
    // }
    // if !unhandled_success_handled_initialized_posts.is_empty() && enable_warning_prints {
    //     let underdirectory = "unhandled_success_handled_initialized_posts";
    //     if enable_warning_prints {
    //         let warning_message = format!(
    //             "{}.len() {}",
    //             underdirectory,
    //             unhandled_success_handled_initialized_posts.len()
    //         );
    //         print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
    //     }
    //     rxiv_logs_create_dir_if_dont_exists(
    //         underdirectory,
    //         &rxiv_kind,
    //         enable_prints,
    //         enable_error_prints,
    //     );
    //     for (key, value) in unhandled_success_handled_initialized_posts {
    //         //цикл должен быть асинхронным\паралельным
    //         let file_name = format!(
    //             "logs/{}/{:?}/{}/{:?}_{}.json",
    //             WARNING_LOGS_DIRECTORY_NAME,
    //             rxiv_kind,
    //             underdirectory,
    //             rxiv_kind,
    //             key.replace("/", "_")
    //         ); //подобрать функцию которая из стринги делает сейвовый путь
    //         let result_of_creating_file = File::create(&file_name);
    //         match result_of_creating_file {
    //             Ok(mut created_for_logs_file) => {
    //                 let json_object = json!({
    //                     "topic": key,
    //                     "url": value,
    //                     "part_of": format!("{:?}", rxiv_kind),
    //                     "date": Local::now().to_string()
    //                 });
    //                 let result_of_stringify_json_object_pretty =
    //                     serde_json::to_string_pretty(&json_object);
    //                 match result_of_stringify_json_object_pretty {
    //                     Ok(string_json_prettyfied) => {
    //                         let result_of_writing =
    //                             created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
    //                         match result_of_writing {
    //                             Ok(_) => {
    //                                 if enable_prints {
    //                                     println!("логи записаны в файл {}", &file_name);
    //                                 }
    //                             }
    //                             Err(e) => {
    //                                 let message = format!(
    //                                     "ошибка записи в файл {} {}",
    //                                     file_name,
    //                                     e.to_string()
    //                                 );
    //                                 if enable_error_prints {
    //                                     print_error_red(
    //                                         file!().to_string(),
    //                                         line!().to_string(),
    //                                         message,
    //                                     )
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     Err(e) => {
    //                         let message = format!(
    //                             "ошибка каста json в string {} {}",
    //                             &json_object,
    //                             e.to_string()
    //                         );
    //                         if enable_error_prints {
    //                             print_error_red(file!().to_string(), line!().to_string(), message)
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 let message = format!("ошибка создания файла {} {}", &file_name, e.to_string());
    //                 if enable_error_prints {
    //                     print_error_red(file!().to_string(), line!().to_string(), message)
    //                 }
    //             }
    //         }
    //     }
    // }
    // if !unhandled_success_handled_res_to_text_error_posts.is_empty() && enable_warning_prints {
    //     let underdirectory = "unhandled_success_handled_res_to_text_error_posts";
    //     if enable_warning_prints {
    //         let warning_message = format!(
    //             "{}.len() {}",
    //             underdirectory,
    //             unhandled_success_handled_res_to_text_error_posts.len()
    //         );
    //         print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
    //     }
    //     rxiv_logs_create_dir_if_dont_exists(
    //         underdirectory,
    //         &rxiv_kind,
    //         enable_prints,
    //         enable_error_prints,
    //     );
    //     for (key, value) in unhandled_success_handled_res_to_text_error_posts {
    //         //цикл должен быть асинхронным\паралельным
    //         let file_name = format!(
    //             "logs/{}/{:?}/{}/{:?}_{}.json",
    //             WARNING_LOGS_DIRECTORY_NAME,
    //             rxiv_kind,
    //             underdirectory,
    //             rxiv_kind,
    //             key.replace("/", "_")
    //         ); //подобрать функцию которая из стринги делает сейвовый путь
    //         let result_of_creating_file = File::create(&file_name);
    //         match result_of_creating_file {
    //             Ok(mut created_for_logs_file) => {
    //                 let json_object = json!({
    //                     "topic": key,
    //                     "url": value.0,
    //                     "error": value.1,
    //                     "part_of": format!("{:?}", rxiv_kind),
    //                     "date": Local::now().to_string()
    //                 });
    //                 let result_of_stringify_json_object_pretty =
    //                     serde_json::to_string_pretty(&json_object);
    //                 match result_of_stringify_json_object_pretty {
    //                     Ok(string_json_prettyfied) => {
    //                         let result_of_writing =
    //                             created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
    //                         match result_of_writing {
    //                             Ok(_) => {
    //                                 if enable_prints {
    //                                     println!("логи записаны в файл {}", &file_name);
    //                                 }
    //                             }
    //                             Err(e) => {
    //                                 let message = format!(
    //                                     "ошибка записи в файл {} {}",
    //                                     file_name,
    //                                     e.to_string()
    //                                 );
    //                                 if enable_error_prints {
    //                                     print_error_red(
    //                                         file!().to_string(),
    //                                         line!().to_string(),
    //                                         message,
    //                                     )
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     Err(e) => {
    //                         let message = format!(
    //                             "ошибка каста json в string {} {}",
    //                             &json_object,
    //                             e.to_string()
    //                         );
    //                         if enable_error_prints {
    //                             print_error_red(file!().to_string(), line!().to_string(), message)
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 let message = format!("ошибка создания файла {} {}", &file_name, e.to_string());
    //                 if enable_error_prints {
    //                     print_error_red(file!().to_string(), line!().to_string(), message)
    //                 }
    //             }
    //         }
    //     }
    // }
    // if !unhandled_success_handled_res_status_error_posts.is_empty() && enable_warning_prints {
    //     let underdirectory = "unhandled_success_handled_res_status_error_posts";
    //     if enable_warning_prints {
    //         let warning_message = format!(
    //             "{}.len() {}",
    //             underdirectory,
    //             unhandled_success_handled_res_status_error_posts.len()
    //         );
    //         print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
    //     }
    //     rxiv_logs_create_dir_if_dont_exists(
    //         underdirectory,
    //         &rxiv_kind,
    //         enable_prints,
    //         enable_error_prints,
    //     );
    //     for (key, value) in unhandled_success_handled_res_status_error_posts {
    //         //цикл должен быть асинхронным\паралельным
    //         let file_name = format!(
    //             "logs/{}/{:?}/{}/{:?}_{}.json",
    //             WARNING_LOGS_DIRECTORY_NAME,
    //             rxiv_kind,
    //             underdirectory,
    //             rxiv_kind,
    //             key.replace("/", "_")
    //         ); //подобрать функцию которая из стринги делает сейвовый путь
    //         let result_of_creating_file = File::create(&file_name);
    //         match result_of_creating_file {
    //             Ok(mut created_for_logs_file) => {
    //                 let json_object = json!({
    //                     "topic": key,
    //                     "url": value.0,
    //                     "status_code": value.1.to_string(),
    //                     "part_of": format!("{:?}", rxiv_kind),
    //                     "date": Local::now().to_string()
    //                 });
    //                 let should_try_to_refetch_it = handle_error_status_code(value.1); //все таки переписать над фильтрс клонированием
    //                 let result_of_stringify_json_object_pretty =
    //                     serde_json::to_string_pretty(&json_object);
    //                 match result_of_stringify_json_object_pretty {
    //                     Ok(string_json_prettyfied) => {
    //                         let result_of_writing =
    //                             created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
    //                         match result_of_writing {
    //                             Ok(_) => {
    //                                 if enable_prints {
    //                                     println!("логи записаны в файл {}", &file_name);
    //                                 }
    //                             }
    //                             Err(e) => {
    //                                 let message = format!(
    //                                     "ошибка записи в файл {} {}",
    //                                     file_name,
    //                                     e.to_string()
    //                                 );
    //                                 if enable_error_prints {
    //                                     print_error_red(
    //                                         file!().to_string(),
    //                                         line!().to_string(),
    //                                         message,
    //                                     )
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     Err(e) => {
    //                         let message = format!(
    //                             "ошибка каста json в string {} {}",
    //                             &json_object,
    //                             e.to_string()
    //                         );
    //                         if enable_error_prints {
    //                             print_error_red(file!().to_string(), line!().to_string(), message)
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 let message = format!("ошибка создания файла {} {}", &file_name, e.to_string());
    //                 if enable_error_prints {
    //                     print_error_red(file!().to_string(), line!().to_string(), message)
    //                 }
    //             }
    //         }
    //     }
    // }
    // if !unhandled_initialized_posts.is_empty() && enable_warning_prints {
    //     let underdirectory = "unhandled_initialized_posts";
    //     if enable_warning_prints {
    //         let warning_message = format!(
    //             "{}.len() {}",
    //             underdirectory,
    //             unhandled_initialized_posts.len()
    //         );
    //         print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
    //     }
    //     rxiv_logs_create_dir_if_dont_exists(
    //         underdirectory,
    //         &rxiv_kind,
    //         enable_prints,
    //         enable_error_prints,
    //     );
    //     for (key, value) in unhandled_initialized_posts {
    //         //цикл должен быть асинхронным\паралельным
    //         let file_name = format!(
    //             "logs/{}/{:?}/{}/{:?}_{}.json",
    //             WARNING_LOGS_DIRECTORY_NAME,
    //             rxiv_kind,
    //             underdirectory,
    //             rxiv_kind,
    //             key.replace("/", "_")
    //         ); //подобрать функцию которая из стринги делает сейвовый путь
    //         let result_of_creating_file = File::create(&file_name);
    //         match result_of_creating_file {
    //             Ok(mut created_for_logs_file) => {
    //                 let json_object = json!({
    //                     "topic": key,
    //                     "url": value,
    //                     "part_of": format!("{:?}", rxiv_kind),
    //                     "date": Local::now().to_string()
    //                 });
    //                 let result_of_stringify_json_object_pretty =
    //                     serde_json::to_string_pretty(&json_object);
    //                 match result_of_stringify_json_object_pretty {
    //                     Ok(string_json_prettyfied) => {
    //                         let result_of_writing =
    //                             created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
    //                         match result_of_writing {
    //                             Ok(_) => {
    //                                 if enable_prints {
    //                                     println!("логи записаны в файл {}", &file_name);
    //                                 }
    //                             }
    //                             Err(e) => {
    //                                 let message = format!(
    //                                     "ошибка записи в файл {} {}",
    //                                     file_name,
    //                                     e.to_string()
    //                                 );
    //                                 if enable_error_prints {
    //                                     print_error_red(
    //                                         file!().to_string(),
    //                                         line!().to_string(),
    //                                         message,
    //                                     )
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     Err(e) => {
    //                         let message = format!(
    //                             "ошибка каста json в string {} {}",
    //                             &json_object,
    //                             e.to_string()
    //                         );
    //                         if enable_error_prints {
    //                             print_error_red(file!().to_string(), line!().to_string(), message)
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 let message = format!("ошибка создания файла {} {}", &file_name, e.to_string());
    //                 if enable_error_prints {
    //                     print_error_red(file!().to_string(), line!().to_string(), message)
    //                 }
    //             }
    //         }
    //     }
    // }
    // if !unhandled_failure_posts.is_empty() && enable_warning_prints {
    //     let underdirectory = "unhandled_failure_posts";
    //     if enable_warning_prints {
    //         let warning_message =
    //             format!("{}.len() {}", underdirectory, unhandled_failure_posts.len());
    //         print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
    //     }
    //     rxiv_logs_create_dir_if_dont_exists(
    //         underdirectory,
    //         &rxiv_kind,
    //         enable_prints,
    //         enable_error_prints,
    //     );
    //     for (key, value) in unhandled_failure_posts {
    //         //цикл должен быть асинхронным\паралельным
    //         let file_name = format!(
    //             "logs/{}/{:?}/{}/{:?}_{}.json",
    //             WARNING_LOGS_DIRECTORY_NAME,
    //             rxiv_kind,
    //             underdirectory,
    //             rxiv_kind,
    //             key.replace("/", "_")
    //         ); //подобрать функцию которая из стринги делает сейвовый путь
    //         let result_of_creating_file = File::create(&file_name);
    //         match result_of_creating_file {
    //             Ok(mut created_for_logs_file) => {
    //                 let json_object = json!({
    //                     "topic": key,
    //                     "url": value.0,
    //                     "error": value.1,
    //                     "part_of": format!("{:?}", rxiv_kind),
    //                     "date": Local::now().to_string()
    //                 });
    //                 let result_of_stringify_json_object_pretty =
    //                     serde_json::to_string_pretty(&json_object);
    //                 match result_of_stringify_json_object_pretty {
    //                     Ok(string_json_prettyfied) => {
    //                         let result_of_writing =
    //                             created_for_logs_file.write(string_json_prettyfied.as_bytes()); //warning_message
    //                         match result_of_writing {
    //                             Ok(_) => {
    //                                 if enable_prints {
    //                                     println!("логи записаны в файл {}", &file_name);
    //                                 }
    //                             }
    //                             Err(e) => {
    //                                 let message = format!(
    //                                     "ошибка записи в файл {} {}",
    //                                     file_name,
    //                                     e.to_string()
    //                                 );
    //                                 if enable_error_prints {
    //                                     print_error_red(
    //                                         file!().to_string(),
    //                                         line!().to_string(),
    //                                         message,
    //                                     )
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     Err(e) => {
    //                         let message = format!(
    //                             "ошибка каста json в string {} {}",
    //                             &json_object,
    //                             e.to_string()
    //                         );
    //                         if enable_error_prints {
    //                             print_error_red(file!().to_string(), line!().to_string(), message)
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 let message = format!("ошибка создания файла {} {}", &file_name, e.to_string());
    //                 if enable_error_prints {
    //                     print_error_red(file!().to_string(), line!().to_string(), message)
    //                 }
    //             }
    //         }
    //     }
    // }
}
