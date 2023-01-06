use std::collections::HashMap;
use std::fmt::format;

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

use itertools::Itertools;
use tufa_common::traits::code_path::CodePath;
use tufa_common::traits::console::Console;
use tufa_common::traits::fields::GetLogType;
use tufa_common::traits::fields::GetSourcePlaceType;
use tufa_common::traits::get_color::ErrorColorBold;
use tufa_common::traits::separator_symbol::SeparatorSymbol;

#[derive(Debug)]
pub struct PrepareForLog {
    pub error_as_string: Option<String>,
    pub code_occurences_as_string: String,
}

#[derive(Debug)]
pub struct ContentPrep {
    pub key_as_string: Option<String>,
    pub inner: String,
}
#[derive(Debug)]
pub struct OneWrapperError {
    source: OneWrapperErrorEnum,
    code_occurence: tufa_common::common::code_occurence::CodeOccurenceOldWay,
}

impl OneWrapperError {
    pub fn get_source_as_string(
        &self,
        config: &tufa_common::config_mods::config_struct::ConfigStruct,
    ) -> String {
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
        let mut sources_for_tracing: Vec<Vec<(String, Vec<String>)>> = vec![];
        let mut vec = self.get_inner_source_and_code_occurence_as_string(config);
        vec.iter_mut().for_each(|n| {
            n.increment += 1;
            n.source.iter().for_each(|f| {
                sources_for_tracing.push(f.clone());
            });
        });
        vec.push(
            tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: sources_for_tracing.clone(),
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        vec
    }
    pub fn log(&self, config: &tufa_common::config_mods::config_struct::ConfigStruct) {
        let log_type = config.get_log_type();
        let symbol = log_type.symbol();
        let mut is_keys_exists = false;
        let mut code_occurence_as_string_vec = self
            .source
            .get_inner_source_and_code_occurence_as_string(config);
        code_occurence_as_string_vec = code_occurence_as_string_vec.into_iter().unique().collect(); //todo - optimize it?
        let mut sources_all = vec![];
        let mut keys_all = vec![];
        let mut originals = vec![];
        let mut additions = vec![];
        code_occurence_as_string_vec.iter().for_each(|c| {
            match c.source.len() == 1 {
                true => match c.source[0].len() == 1 {
                    true => match c.source[0][0].1.is_empty() {
                        true => {
                            originals.push(c.clone());
                        }
                        false => {
                            additions.push(c.clone());
                        }
                    },
                    false => {
                        additions.push(c.clone());
                    }
                },
                false => {
                    additions.push(c.clone());
                }
            }
            match c.increment == 0 {
                true => {
                    c.source.iter().for_each(|v| {
                        v.iter().for_each(|(source, keys)| {
                            sources_all.push(source.clone());
                            keys.iter().for_each(|k| {
                                keys_all.push(k.clone());
                            });
                        });
                    });
                }
                false => (),
            }
        });
        sources_all = sources_all.into_iter().unique().collect(); //todo - optimize it?
        sources_all.sort();
        keys_all = keys_all.into_iter().unique().collect(); //todo - optimize it?
        keys_all.sort();
        let mut additions_partial = vec![];
        let mut additions_all = vec![];
        additions.iter().for_each(|c| {
            let mut local_sources = vec![];
            let mut local_keys = vec![];
            c.source.iter().for_each(|v| {
                v.iter().for_each(|(source, keys)| {
                    local_sources.push(source.clone());
                    keys.iter().for_each(|k| {
                        local_keys.push(k.clone());
                    });
                });
            });
            local_sources = local_sources.into_iter().unique().collect(); //todo - optimize it?
            local_sources.sort();
            local_keys = local_keys.into_iter().unique().collect(); //todo - optimize it?
            local_keys.sort();
            match (
                sources_all.len() == local_sources.len(),
                keys_all.len() == local_keys.len(),
            ) {
                (true, true) => {
                    let mut equal = true;
                    for i in 0..local_sources.len() {
                        match local_sources[i] == sources_all[i] {
                            true => (),
                            false => {
                                equal = false;
                                break;
                            }
                        }
                    }
                    match equal {
                        true => {
                            additions_all.push(c.clone());
                        }
                        false => {
                            additions_partial.push(c.clone());
                        }
                    }
                }
                (true, false) => {
                    additions_partial.push(c.clone());
                }
                (false, true) => {
                    additions_partial.push(c.clone());
                }
                (false, false) => {
                    additions_partial.push(c.clone());
                }
            }
        });
        let mut additions_partial_with_origins = vec![];
        additions_partial.iter().for_each(|o| {
            let mut local_sources = vec![];
            o.source.iter().for_each(|v| {
                v.iter().for_each(|(source, vec)| {
                    local_sources.push(source);
                });
            });
            local_sources = local_sources.into_iter().unique().collect();
            let mut vec_of_origins = vec![];
            local_sources.into_iter().for_each(|source| {
                originals.iter().for_each(|a| {
                    let mut contains = false;
                    for v in &a.source {
                        let mut inner_contains = false;
                        for (s, vec) in v {
                            match source == s {
                                true => {
                                    inner_contains = true;
                                    break;
                                }
                                false => (),
                            }
                        }
                        match inner_contains {
                            true => {
                                contains = true;
                                break;
                            }
                            false => (),
                        }
                    }
                    match contains {
                        true => {
                            vec_of_origins.push(a.clone());
                        }
                        false => (),
                    }
                });
            });
            additions_partial_with_origins.push((o.clone(), vec_of_origins));
        });
        additions_all.sort_by(|a, b| a.increment.cmp(&b.increment));
        additions_all.reverse();
        let mut almost_all = vec![];
        additions_partial_with_origins
            .iter()
            .for_each(|(part, origins)| {
                let mut origins_stack = vec![];
                part.source.iter().for_each(|v| {
                    let mut new_vec = vec![];
                    v.iter().for_each(|(source, vec)| {
                        let mut equals = None;
                        for vv in &additions_all[0].source {
                            let mut contains = None;
                            for (source_in_all, vec_in_all) in vv {
                                match source == source_in_all {
                                    true => {
                                        contains = Some(vec_in_all.clone());
                                        break;
                                    }
                                    false => (),
                                }
                            }
                            match contains {
                                Some(vf) => {
                                    equals = Some(vf.clone());
                                    break;
                                }
                                None => (),
                            }
                        }
                        match equals {
                            Some(vvv) => {
                                let mut difference = vec.clone();
                                //not sure about ordering
                                vvv.iter().for_each(|vvve| match difference.contains(vvve) {
                                    true => (),
                                    false => {
                                        difference.push(vvve.clone());
                                    }
                                });
                                new_vec.push((source.clone(), difference.clone()));
                            }
                            None => {
                                new_vec.push((source.clone(), vec.clone()));
                            }
                        }
                    });
                    origins_stack.push(new_vec.clone());
                });
                let mut cloned_part = part.clone();
                cloned_part.source = origins_stack;
                almost_all.push((cloned_part.clone(), origins));
            });
        let mut additions_partial_with_origins_as_string = vec![];
        almost_all.iter().for_each(|(source, origins_vec)| {
            let mut local_sources = vec![];
            let mut local_keys = vec![];
            source.source.iter().for_each(|v| {
                v.iter().for_each(|(source, vecc)| {
                    local_sources.push(source);
                    vecc.iter().for_each(|ve| {
                        local_keys.push(ve.clone());
                    });
                });
            });
            local_sources = local_sources.into_iter().unique().collect();
            local_keys = local_keys.into_iter().unique().collect();
            match local_keys.is_empty() {
                true => {
                    let mut fold = origins_vec.iter().fold(String::from(""), |mut acc, o| {
                        let source = o.source[0][0].0.clone(); //todo
                        acc.push_str(&format!(
                            "{}{}{}{}",
                            source, symbol, o.code_occurence, symbol
                        ));
                        acc
                    });
                    log_type.pop_last(&mut fold);
                    let mut fold = fold.lines().collect::<Vec<&str>>().iter().fold(
                        String::from(""),
                        |mut acc, element| {
                            acc.push_str(&format!(" {}{}", element, symbol));
                            acc
                        },
                    );
                    log_type.pop_last(&mut fold);
                    let fold = format!(
                        "[{}{}{}]{}{}",
                        symbol, fold, symbol, symbol, source.code_occurence
                    );
                    additions_partial_with_origins_as_string.push((source.clone(), fold.clone()));
                }
                false => {
                    let mut fold = origins_vec.iter().fold(String::from(""), |mut acc, o| {
                        let source = o.source[0][0].0.clone(); //todo
                        acc.push_str(&format!(
                            "{}{}{}{}",
                            source, symbol, o.code_occurence, symbol
                        ));
                        acc
                    });

                    log_type.pop_last(&mut fold);
                    let mut fold = fold.lines().collect::<Vec<&str>>().iter().fold(
                        String::from(""),
                        |mut acc, element| {
                            acc.push_str(&format!(" {}{}", element, symbol));
                            acc
                        },
                    );
                    println!("fold\n\n{}\n", fold);
                    log_type.pop_last(&mut fold);
                    let mut first = true;
                    let mut fold_handle =
                        local_keys
                            .iter()
                            .fold(String::from(""), |mut acc, local_key| {
                                println!("acc\n\n{}\n\n", acc);
                                match first {
                                    true => {
                                        acc.push_str(&format!(
                                            "{} [{}{}{}]",
                                            local_key, symbol, fold, symbol
                                        ));
                                        acc.push_str(&format!(
                                            "{}{}",
                                            symbol, source.code_occurence
                                        ));
                                        println!("acc after first\n\n{}\n\n", acc);
                                        first = false;
                                    }
                                    false => {
                                        let mut lined = acc
                                            .lines()
                                            .collect::<Vec<&str>>()
                                            .iter()
                                            .fold(String::from(""), |mut acc, element| {
                                                acc.push_str(&format!(" {}{}", element, symbol));
                                                acc
                                            });
                                        log_type.pop_last(&mut lined);
                                        println!("lined \n\n{}\n\nlined", lined);
                                        acc = format!(
                                            "{} [{}{}{}]",
                                            local_key,
                                            symbol,
                                            lined,
                                            symbol,
                                            // symbol,
                                            // source.code_occurence
                                        );
                                        // acc.push_str(&format!(
                                        //     "{}{}{}",
                                        //     symbol, source.code_occurence, symbol
                                        // ));
                                        println!("acc after others\n\n{}\n\n", acc);
                                    }
                                }
                                acc
                            });

                    additions_partial_with_origins_as_string
                        .push((source.clone(), fold_handle.clone()))
                }
            }
        });
        additions_partial_with_origins_as_string
            .iter()
            .for_each(|i| {
                println!("{}", i.1);
            });
        println!(
            "additions_partial_with_origins_as_string\n{:#?}\nadditions_partial_with_origins_as_string",
            additions_partial_with_origins_as_string
        );
        let mut f = additions_partial_with_origins_as_string.iter().fold(
            String::from(""),
            |mut acc, (one, two)| {
                acc.push_str(&format!("{}{}", two, symbol));
                acc
            },
        );
        let mut lined =
            f.lines()
                .collect::<Vec<&str>>()
                .iter()
                .fold(String::from(""), |mut acc, element| {
                    acc.push_str(&format!(" {}{}", element, symbol));
                    acc
                });
        log_type.pop_last(&mut lined);
        f = format!("[{}{}{}]{}", symbol, lined, symbol, symbol);
        match !additions_all.is_empty() {
            true => {
                additions_all.iter().for_each(|value| {
                    f.push_str(&format!("{}{}", value.code_occurence, symbol));
                });
            }
            false => (),
        }
        println!("{}", f);
        let mut prepared_log = f.clone();
        prepared_log.push_str(&format!("{}", &self.get_code_occurence_as_string(config)));
        log_type.console(&config.get_error_color_bold(), prepared_log)
    }
}

#[derive(Debug)]
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
}

pub fn one(should_trace: bool) -> Result<(), Box<OneWrapperError>> {
    if let Err(e) = tufa_common::dev::three(false) {
        let f = OneWrapperError {
            source: OneWrapperErrorEnum::ThreeWrapper(*e),
            code_occurence: tufa_common::common::code_occurence::CodeOccurenceOldWay {
                git_info: once_cell::sync::Lazy::force(&crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES).clone(),
                time_file_line_column: tufa_common::common::time_file_line_column::TimeFileLineColumn::new_file_line_column(
                    String::from(file!()),
                    line!(),
                    column!(),
                ),
            }
        };
        f.log(once_cell::sync::Lazy::force(
            &crate::global_variables::runtime::config::CONFIG,
        ));
        return Err(Box::new(f));
    }
    Ok(())
}

//todo - must ininitalize with keys not in the get_inner_source_and_code_occurence_as_string - its one step ahead of actual place

// [
//  (key: five_hashmap_key)[
//   (key: five_one_hashmap key) [
//    five_one error
//    tufa_common/src/dev.rs:873:17
//   ]
//   tufa_common/src/dev.rs:795:17
//  ]
//  (key: six_hashmap_key)[
//   [
//    error_seven
//    tufa_common/src/dev.rs:1300:17
//    error_eight
//    tufa_common/src/dev.rs:1385:17
//   ]
//   tufa_common/src/dev.rs:1150:25
//  ]
// ]
// tufa_common/src/dev.rs:554:25
// tufa_common/src/dev.rs:211:21
// tufa_server/src/entry.rs:860:21
