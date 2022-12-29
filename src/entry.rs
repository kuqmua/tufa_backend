use std::collections::HashMap;

use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
// use crate::global_variables::hardcode::PROJECT_NAME;
use crate::global_variables::runtime::config::CONFIG;
// use crate::preparation::prepare_server::prepare_server;
// use crate::prints::print_colorful_message::print_colorful_message;
// use crate::server_wrapper::server_wrapper;
// use crate::telemetry::get_subscriber::get_subscriber;
// use crate::telemetry::init_subscriber::init_subscriber;
// use tufa_common::config_mods::print_type::PrintType;
// use tufa_common::traits::fields::GetLogType;
// use tufa_common::traits::get_git_source_file_link::GetGitSourceFileLink;
// use valuable::Valuable;

// #[derive(Clone, Debug, Valuable)]
// struct User {
//     name: String,
//     age: u32,
//     something: Vec<bool>,
//     address: Address,
// }

// #[derive(Clone, Debug, Valuable)]
// struct Address {
//     country: String,
//     city: String,
//     street: String,
// }

pub fn entry() {
    // match tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(num_cpus::get())
    //     .enable_all()
    //     .build()
    // {
    //     Err(e) => {
    //         print_colorful_message(
    //             None,
    //             tufa_common::config_mods::print_type::PrintType::WarningHigh,
    //             vec![format!("{}:{}:{}", file!(), line!(), column!())],
    //             vec![GIT_INFO_WITHOUT_LIFETIMES.get_git_source_file_link(file!(), line!())],
    //             format!("Cannot build tokio runtime {e:#?}"),
    //         );
    //     }
    //     Ok(runtime) => {
    one(true);
    //         if let tufa_common::config_mods::log_type::LogType::Tracing = CONFIG.log_type {
    //             if let Err(e) = init_subscriber(get_subscriber(
    //                 PROJECT_NAME.into(),
    //                 CONFIG.tracing_type.to_lower_snake_case(),
    //                 std::io::stdout,
    //             )) {
    //                 print_colorful_message(
    //                     None,
    //                     tufa_common::config_mods::print_type::PrintType::WarningHigh,
    //                     vec![format!("{}:{}:{}", file!(), line!(), column!())],
    //                     vec![GIT_INFO_WITHOUT_LIFETIMES.get_git_source_file_link(file!(), line!())],
    //                     format!("tracing init_subscriber error: {:#?}", e),
    //                 );
    //                 return;
    //             };
    //         }
    //         // let user = User {
    //         //     name: "Arwen Undomiel".to_string(),
    //         //     age: 3000,
    //         //     something: vec![true, false],
    //         //     address: Address {
    //         //         country: "Middle Earth".to_string(),
    //         //         city: "Rivendell".to_string(),
    //         //         street: "leafy lane".to_string(),
    //         //     },
    //         // };
    //         // tracing::error!(valuable = false, user = ?user);
    //         if let true = CONFIG.is_preparation_enabled {
    //             if runtime.block_on(prepare_server(true)).is_err() {
    //                 return;
    //             }
    //         }
    //         if let Err(e) = server_wrapper() {
    //             print_colorful_message(
    //                 None,
    //                 tufa_common::config_mods::print_type::PrintType::WarningHigh,
    //                 vec![format!("{}:{}:{}", file!(), line!(), column!())],
    //                 vec![GIT_INFO_WITHOUT_LIFETIMES.get_git_source_file_link(file!(), line!())],
    //                 format!("Cannot run actix-web HttpServer, error: {:#?}", e),
    //             );
    //         }
    //     }
    // }
}

use impl_get_source::ImplGetSourceFromTufaCommon;
use tufa_common::dev::ThreeWrapperError;
use tufa_common::traits::my_custom_display::DisplayError;
use tufa_common::traits::new_error_with_one_addition::NewErrorWithOneAddition;

use tufa_common::traits::code_path::CodePath;
use tufa_common::traits::console::Console;
use tufa_common::traits::fields::GetLogType;
use tufa_common::traits::fields::GetSourcePlaceType;
use tufa_common::traits::get_color::ErrorColorBold;
use tufa_common::traits::separator_symbol::SeparatorSymbol;
// use itertools::Itertools;

pub struct PrepareForLog {
    pub error_as_string: Option<String>,
    pub code_occurences_as_string: String,
}
pub struct ContentPrep {
    pub key_as_string: Option<String>,
    pub inner: String,
}
// #[derive(ImplGetSourceFromTufaCommon)]
pub struct OneWrapperError {
    source: OneWrapperErrorEnum,
    // code_occurence: tufa_common::common::code_occurence::CodeOccurence,
    code_occurence: tufa_common::common::code_occurence::CodeOccurenceOldWay,
}

impl OneWrapperError {
    pub fn get_source_as_string(
        &self,
        config: &tufa_common::config_mods::config_struct::ConfigStruct,
    ) -> String {
        //todo if origin - without config, if wrapper - with config
        // format!(
        //     "{}",
        //     self.source.get_source_and_code_occurence_as_string(config)
        // )
        format!("{}", self.source.get_source_as_string(config))
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &tufa_common::config_mods::config_struct::ConfigStruct,
    ) -> String {
        self.code_occurence.time_file_line_column.get_code_path(
            &self.code_occurence.git_info,
            config.get_source_place_type(),
        )
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &tufa_common::config_mods::config_struct::ConfigStruct, //todo maybe remove
    ) -> Vec<tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let mut vec = self.get_inner_source_and_code_occurence_as_string(config);
        vec.iter_mut().for_each(|n| n.increment += 1);
        vec.push(
            tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: None,
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        vec
    }
    // pub fn get_source_and_code_occurence_as_string(
    //     &self,
    //     config: &tufa_common::config_mods::config_struct::ConfigStruct,
    // ) -> String {
    //     format!(
    //         "{}{}{}",
    //         self.get_source_as_string(config),
    //         config.get_log_type().symbol(),
    //         self.get_code_occurence_as_string(config)
    //     )
    // }
    pub fn log(&self, config: &tufa_common::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        let symbol = log_type.symbol();
        // println!("{:#?}", self.source
        // .get_inner_source_and_code_occurence_as_string(config));
        // println!("_______________");
        // println!("{:#?}", &self.get_code_occurence_as_string(config));
        let mut is_keys_exists = false;
        let code_occurence_as_string_vec = self
            .source
            .get_inner_source_and_code_occurence_as_string(config);
        println!("{:#?}", code_occurence_as_string_vec);
        // let mut source_and_code_occurence_as_string_version_one = Vec::new();
        // let mut source_and_code_occurence_as_string_version_two = Vec::new();
        // let len = code_occurence_as_string_vec.len();
        for c in &code_occurence_as_string_vec {
            // match &c.source {
            //     Some(_) => {

            //     },
            //     None => {
            //         source_and_code_occurence_as_string_version_one.push(tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsStringVersionOne{

            //         })
            //     },
            // }
            if let Some(source_enum) = &c.source {
                if let tufa_common::common::source_and_code_occurence::SourceEnum::SourceWithKeys(
                    _,
                ) = source_enum
                {
                    is_keys_exists = true;
                    break;
                }
            }
        }
        let mut prepared_log = match is_keys_exists {
            true => {
                // let mut addition_to_increment_spaces = String::from("");
                let mut prepared_by_increments_hashmap: HashMap::<u64, Vec<tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString>> = HashMap::new();
                code_occurence_as_string_vec.iter().for_each(|element| {
                    // println!("{:#?}", element);
                    let mut should_insert_full_new = true;
                    let mut fff = prepared_by_increments_hashmap.clone();
                    for (k, v) in &mut fff {
                        if element.increment == *k {
                            //wrong logic!!!
                            should_insert_full_new = false;
                            if !v.contains(&element.clone()) {
                                let mut v_cloned = v.clone();
                                v_cloned.push(element.clone());
                                prepared_by_increments_hashmap.insert(element.increment, v_cloned);
                                break;
                            }

                            // break;
                        }
                    }
                    if should_insert_full_new {
                        prepared_by_increments_hashmap
                            .insert(element.increment, vec![element.clone()]);
                    }
                });
                let mut prepared_by_increments_vec = Vec::new();
                prepared_by_increments_hashmap.iter().for_each(|(k, v)| {
                    prepared_by_increments_vec.push((k, v));
                });
                prepared_by_increments_vec.sort_by(|(k1, v1), (k2, v2)| k1.cmp(k2));
                prepared_by_increments_vec.reverse();
                println!("{:#?}", prepared_by_increments_vec);
                let mut content = ContentPrep {
                    key_as_string: None,
                    inner: String::from(""),
                };
                prepared_by_increments_vec.iter().for_each(|(_, v)| {
                    let folded = v.iter().map(|element| {
                        // let mut increment_spaces = String::from("");
                        // for x in (0..element.increment) {
                        //     //0 or 1 ?
                        //     increment_spaces.push(' ');
                        // }
                        let prepare_for_log = match &element.source {
                            Some(source_enum) => match source_enum {
                                tufa_common::common::source_and_code_occurence::SourceEnum::SourceWithKeys(source_with_keys) => {
                                    // let increment_spaces_prepared = increment_spaces.clone();
                                    let mut prepared_keys = format!("{}[key: ", symbol);
                                    // //todo maybe for each key add symbol and additional spaces for log structs where key is
                                    source_with_keys.keys.iter().for_each(|e|{
                                        prepared_keys.push_str(e);
                                        prepared_keys.push_str(", ");
                                    });
                                    prepared_keys.pop();
                                    prepared_keys.pop();
                                    PrepareForLog {
                                        error_as_string: Some(format!("{}] {}", prepared_keys, source_with_keys.source)),
                                        code_occurences_as_string: element.code_occurence.clone(),
                                    }
                                    // format!("{}] {} {}{}{}", prepared_keys, source_with_keys.source, symbol, element.code_occurence, symbol)
                                },
                                tufa_common::common::source_and_code_occurence::SourceEnum::Source(source) => {
                                    PrepareForLog {
                                        error_as_string: Some(source.clone()),
                                        code_occurences_as_string: element.code_occurence.clone(),
                                    }
                                    // format!("{}{}{}{}{}", symbol, source, symbol, element.code_occurence, symbol)
                                },
                            },
                            None => {
                                PrepareForLog {
                                    error_as_string: None,
                                    code_occurences_as_string: element.code_occurence.clone(),
                                }
                                // format!("{}{}{}", symbol, element.code_occurence, symbol)
                            },
                        };
                        // log_type.pop_last(&mut formatted_handle);
                        // acc.push_str(&formatted_handle);
                        // acc.push_str(&format!("{}{}", formatted_handle, symbol));
                        // log_type.pop_last(&mut acc);
                        // println!("--{}--", acc);
                        // acc
                        prepare_for_log
                    }).collect::<Vec<PrepareForLog>>();
                    // let content_part = format!("[{}{}]", folded, symbol);
                    // println!("{}", content_part);
                    // content.push_str(&folded);
                    // content = content_part
                    match folded.len() == 1 {
                        true => {
                            match content.inner.is_empty() {
                                true => {
                                    content = ContentPrep {
                                        key_as_string: folded[0].error_as_string.clone(),
                                        inner: folded[0].code_occurences_as_string.clone(),
                                    }
                                },
                                false => {
                                    match &folded[0].error_as_string {
                                        Some(eas) => {
                                            content = ContentPrep {
                                                key_as_string: folded[0].error_as_string.clone(),
                                                inner: format!("[{}{}{}{}{}]", symbol, eas, symbol, folded[0].code_occurences_as_string.clone(), symbol),
                                            }
                                        },
                                        None => {
                                            content = ContentPrep {
                                                key_as_string: folded[0].error_as_string.clone(),
                                                inner: format!("[{}{}{}]", symbol, folded[0].code_occurences_as_string.clone(), symbol),
                                            }
                                        },
                                    }
                                },
                            }
                        },
                        false => {
                            todo!()
                        },
                    }
                });

                // println!("{}", content);
                //
                String::from("")
            }
            false => {
                code_occurence_as_string_vec
                    .iter()
                    .fold(String::from(""), |mut acc, element| {
                        // println!("{:#?}", element);
                        let mut increment_spaces = String::from("");
                        for x in (0..element.increment) {
                            //0 or 1 ?
                            increment_spaces.push(' ');
                        }
                        let formatted_handle = match &element.source {
                            Some(source_enum) => {
                                match source_enum {
                                    tufa_common::common::source_and_code_occurence::SourceEnum::SourceWithKeys(_) => String::from(""),//todo rewrite it
                                    tufa_common::common::source_and_code_occurence::SourceEnum::Source(source) => format!("{}{}{}{}", increment_spaces, element.code_occurence, symbol, element.code_occurence),
                                }
                            },
                            None => format!("{}{}", increment_spaces, element.code_occurence),
                        };
                        acc.push_str(&format!("{}{}", formatted_handle, symbol));
                        acc
                    })
            }
        };
        prepared_log.push_str(&self.get_code_occurence_as_string(config));
        log_type.console(&config.get_error_color_bold(), prepared_log)
    }
}

// impl tufa_common::traits::get_source::GetSource for OneWrapperError {
//     fn get_source(&self) -> String {
//         self.source.get_source()
//     }
// }
// impl tufa_common::traits::get_source_value::GetSourceValue<OneWrapperErrorEnum>
//     for OneWrapperError
// {
//     fn get_source_value(&self) -> &OneWrapperErrorEnum {
//         &self.source
//     }
// }

// impl tufa_common::traits::get_code_occurence::GetCodeOccurence for OneWrapperError {
//     fn get_code_occurence(&self) -> &tufa_common::common::code_occurence::CodeOccurence {
//         &self.code_occurence
//     }
// }

// impl tufa_common::traits::init_error::InitError<OneWrapperErrorEnum> for OneWrapperError {
//     fn init_error(
//         source: OneWrapperErrorEnum,
//         code_occurence: tufa_common::common::code_occurence::CodeOccurence,
//     ) -> Self {
//         Self {
//             source,
//             code_occurence,
//         }
//     }
// }

// #[derive(ImplGetSourceFromTufaCommon)]
pub enum OneWrapperErrorEnum {
    ThreeWrapper(ThreeWrapperError),
}

impl OneWrapperErrorEnum {
    fn get_source_as_string(
        &self,
        config: &tufa_common::config_mods::config_struct::ConfigStruct,
    ) -> String {
        match self {
            //todo if origin - without config, if wrapper - with config
            OneWrapperErrorEnum::ThreeWrapper(i) => {
                // i.get_source_and_code_occurence_as_string(config)
                i.get_source_as_string(config)
            }
        }
    }
    fn get_code_occurence_as_string(
        &self,
        config: &tufa_common::config_mods::config_struct::ConfigStruct,
    ) -> String {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => i.get_code_occurence_as_string(config),
        }
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &tufa_common::config_mods::config_struct::ConfigStruct,
    ) -> Vec<tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(i) => {
                i.get_inner_source_and_code_occurence_as_string(config)
            }
        }
    }
    //does it need to be implemented here?
    // fn get_source_and_code_occurence_as_string(
    //     &self,
    //     config: &tufa_common::config_mods::config_struct::ConfigStruct,
    // ) -> String {
    //     match self {
    //         OneWrapperErrorEnum::ThreeWrapper(i) => {
    //             i.get_source_and_code_occurence_as_string(config)
    //         }
    //     }
    // }
}

// impl tufa_common::traits::get_source::GetSource for OneWrapperErrorEnum {
//     fn get_source(&self) -> String {
//         match self {
//             OneWrapperErrorEnum::ThreeWrapper(e) => e.get_source(),
//         }
//         // self.source.get_source()
//     }
// }

// impl tufa_common::traits::get_code_occurence::GetCodeOccurence for OneWrapperErrorEnum {
//     fn get_code_occurence(&self) -> &tufa_common::common::code_occurence::CodeOccurence {
//         match self {
//             OneWrapperErrorEnum::ThreeWrapper(e) => e.get_code_occurence(),
//         }
//     }
// }

pub fn one(should_trace: bool) -> Result<(), Box<OneWrapperError>> {
    if let Err(e) = tufa_common::dev::three(false) {
        // let eee = ;
        // println!("{}", eee.display_error(once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG)));
        // return Err(Box::new(OneWrapperError::new_error_with_one_addition(
        //     OneWrapperErrorEnum::ThreeWrapper(*e),
        //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG),
        //     once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES),
        //     String::from(file!()),
        //     line!(),
        //     column!(),
        //     should_trace
        // )));
        let f = OneWrapperError {
            source: OneWrapperErrorEnum::ThreeWrapper(*e),
            // code_occurence: tufa_common::common::code_occurence::CodeOccurence {
            //     occurences: HashMap::new(),
            // },
            code_occurence: tufa_common::common::code_occurence::CodeOccurenceOldWay {
                git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                time_file_line_column: tufa_common::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                    String::from(file!()),
                    line!(),
                    column!(),
                ),
            }
        };
        println!("one f {}", std::mem::size_of_val(&f));
        println!("one source {}", std::mem::size_of_val(&f.source));
        println!("one source {}", std::mem::size_of_val(&f.code_occurence));
        println!("one-----");
        f.log(once_cell::sync::Lazy::force(
            &crate::global_variables::runtime::config::CONFIG,
        ));
        println!("one-----");
        return Err(Box::new(f));
    }
    Ok(())
}
