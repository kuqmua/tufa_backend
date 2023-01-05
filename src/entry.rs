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
// #[derive(ImplGetSourceFromTufaCommon)]
#[derive(Debug)]
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
        let mut sources_for_tracing = vec![];
        let mut keys_for_tracing = vec![];
        let mut vec = self.get_inner_source_and_code_occurence_as_string(config);
        vec.iter_mut().for_each(|n| {
            n.increment += 1;
            match &n.source {
                tufa_common::common::source_and_code_occurence::SourceEnum::SourceWithKeys(
                    source_with_keys,
                ) => {
                    source_with_keys.keys.iter().for_each(|k| {
                        keys_for_tracing.push(k.clone());
                    });
                }
                tufa_common::common::source_and_code_occurence::SourceEnum::Source(source) => {
                    sources_for_tracing.push(source.clone());
                }
                tufa_common::common::source_and_code_occurence::SourceEnum::SourcesForTracing(sources) => {
                    sources.iter().for_each(|s| {
                        sources_for_tracing.push(s.clone());
                    });
                }
                tufa_common::common::source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing) => {
                    sources_and_keys_for_tracing.sources.iter().for_each(|s| {
                        sources_for_tracing.push(s.clone());
                    });
                    sources_and_keys_for_tracing.keys.iter().for_each(|k| {
                        keys_for_tracing.push(k.clone());
                    });
                }
            }
        });
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        keys_for_tracing = keys_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        let source_handle = match keys_for_tracing.is_empty() {
            true => tufa_common::common::source_and_code_occurence::SourceEnum::SourcesForTracing(
                sources_for_tracing,
            ),
            false => {
                tufa_common::common::source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(
                    tufa_common::common::source_and_code_occurence::SourcesAndKeysForTracing {
                        sources: sources_for_tracing,
                        keys: keys_for_tracing,
                    },
                )
            }
        };
        vec.push(
            tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: source_handle,
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
        let mut sources_for_tracing = vec![];
        let mut keys_for_tracing = vec![];
        code_occurence_as_string_vec.iter().for_each(|code_occurence_as_string|{
            match &code_occurence_as_string.source {
                tufa_common::common::source_and_code_occurence::SourceEnum::SourceWithKeys(source_with_keys) => {
                    sources_for_tracing.push(source_with_keys.source.clone());
                    source_with_keys.keys.iter().for_each(|k|{
                        keys_for_tracing.push(k.clone());
                    });
                },
                tufa_common::common::source_and_code_occurence::SourceEnum::Source(source) => {
                    sources_for_tracing.push(source.clone());
                },
                tufa_common::common::source_and_code_occurence::SourceEnum::SourcesForTracing(sources) => {
                    sources.iter().for_each(|s|{
                        sources_for_tracing.push(s.clone());
                    });
                },
                tufa_common::common::source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing) => {
                    sources_and_keys_for_tracing.sources.iter().for_each(|s|{
                        sources_for_tracing.push(s.clone());
                    });
                    sources_and_keys_for_tracing.keys.iter().for_each(|k|{
                        keys_for_tracing.push(k.clone());
                    });
                },
            }
        });
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        keys_for_tracing = keys_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        // println!("{:#?}", sources_for_tracing);
        // println!("{:#?}", keys_for_tracing);
        let mut source_with_code_occurence_handle_vec: Vec<
            tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceHandle,
        > = Vec::new(); //todo - optimize
        let mut source_with_code_occurence_finder_vec_partial: Vec<
            tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder,
        > = Vec::new(); //todo - optimize
        let mut source_with_code_occurence_finder_vec_all: Vec<
            tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder,
        > = Vec::new(); //todo - optimize
        code_occurence_as_string_vec.iter().for_each(|code_occurence_as_string|{
            //todo - remove option here
                match &code_occurence_as_string.source {
                    tufa_common::common::source_and_code_occurence::SourceEnum::SourceWithKeys(source_with_keys) => {
                        source_with_code_occurence_handle_vec.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceHandle{
                            source: tufa_common::common::source_and_code_occurence::SourceHandleEnum::SourceWithKeys(source_with_keys.clone()),
                            code_occurence: code_occurence_as_string.code_occurence.clone(),
                            increment: code_occurence_as_string.increment.clone(),
                        });
                    },
                    tufa_common::common::source_and_code_occurence::SourceEnum::Source(source) => {
                        // sources_for_tracing.push(source.clone());
                        source_with_code_occurence_handle_vec.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceHandle{
                            source: tufa_common::common::source_and_code_occurence::SourceHandleEnum::Source(source.clone()),
                            code_occurence: code_occurence_as_string.code_occurence.clone(),
                            increment: code_occurence_as_string.increment.clone(),
                        });
                    },
                    tufa_common::common::source_and_code_occurence::SourceEnum::SourcesForTracing(sources) => {
                        match keys_for_tracing.is_empty() {
                            true => {
                                let mut sources_handle = sources.clone();
                                sources_handle.sort();
                                let mut sources_for_tracing_handle = sources_for_tracing.clone();
                                sources_for_tracing_handle.sort();
                                let mut equal = false;
                                if sources_handle.len() == sources_for_tracing_handle.len() {
                                    equal = true;
                                    for index in 0..sources_handle.len() {
                                        //todo - match indexes
                                        if sources_handle[index] != sources_for_tracing_handle[index] {
                                            equal = false;
                                            break;
                                        }
                                    }
                                }
                                match equal {
                                    true => {
                                        source_with_code_occurence_finder_vec_all.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                            source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesForTracing(sources.clone()),
                                            code_occurence: code_occurence_as_string.code_occurence.clone(),
                                            increment: code_occurence_as_string.increment.clone(),
                                        });
                                    },
                                    false => {
                                        source_with_code_occurence_finder_vec_partial.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                            source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesForTracing(sources.clone()),
                                            code_occurence: code_occurence_as_string.code_occurence.clone(),
                                            increment: code_occurence_as_string.increment.clone(),
                                        });
                                    },
                                }
                            },
                            false => {
                                source_with_code_occurence_finder_vec_partial.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                    source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesForTracing(sources.clone()),
                                    code_occurence: code_occurence_as_string.code_occurence.clone(),
                                    increment: code_occurence_as_string.increment.clone(),
                                });
                            },
                        }
                    },
                    tufa_common::common::source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing) => {
                        match (sources_and_keys_for_tracing.sources.len() == sources_for_tracing.len(), sources_and_keys_for_tracing.keys.len() == keys_for_tracing.len()) {
                            (true, true) => {
                                let mut sources_handle = sources_and_keys_for_tracing.sources.clone();
                                sources_handle.sort();
                                let mut sources_for_tracing_handle = sources_for_tracing.clone();
                                sources_for_tracing_handle.sort();
                                let mut sources_equal = true;
                                for index in 0..sources_handle.len() {
                                    //todo - match indexes
                                    if sources_handle[index] != sources_for_tracing_handle[index] {
                                        sources_equal = false;
                                        break;
                                    }
                                }
                                let mut keys_handle = sources_and_keys_for_tracing.keys.clone();
                                keys_handle.sort();
                                let mut keys_for_tracing_handle = keys_for_tracing.clone();
                                keys_for_tracing_handle.sort();
                                let mut keys_equal = true;
                                for index in 0..keys_handle.len() {
                                    //todo - match indexes
                                    if keys_handle[index] != keys_for_tracing_handle[index] {
                                        keys_equal = false;
                                        break;
                                    }
                                }
                                match (sources_equal, keys_equal) {
                                    (true, true) => {
                                        source_with_code_occurence_finder_vec_all.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                            source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing.clone()),
                                            code_occurence: code_occurence_as_string.code_occurence.clone(),
                                            increment: code_occurence_as_string.increment.clone(),
                                        });
                                    },
                                    (true, false) => {
                                        source_with_code_occurence_finder_vec_partial.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                            source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing.clone()),
                                            code_occurence: code_occurence_as_string.code_occurence.clone(),
                                            increment: code_occurence_as_string.increment.clone(),
                                        });
                                    },
                                    (false, true) => {
                                        source_with_code_occurence_finder_vec_partial.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                            source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing.clone()),
                                            code_occurence: code_occurence_as_string.code_occurence.clone(),
                                            increment: code_occurence_as_string.increment.clone(),
                                        });
                                    },
                                    (false, false) => {
                                        source_with_code_occurence_finder_vec_partial.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                            source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing.clone()),
                                            code_occurence: code_occurence_as_string.code_occurence.clone(),
                                            increment: code_occurence_as_string.increment.clone(),
                                        });
                                    },
                                }
                            },
                            (true, false) => {
                                source_with_code_occurence_finder_vec_partial.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                    source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing.clone()),
                                    code_occurence: code_occurence_as_string.code_occurence.clone(),
                                    increment: code_occurence_as_string.increment.clone(),
                                });
                            },
                            (false, true) => {
                                source_with_code_occurence_finder_vec_partial.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                    source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing.clone()),
                                    code_occurence: code_occurence_as_string.code_occurence.clone(),
                                    increment: code_occurence_as_string.increment.clone(),
                                });
                            },
                            (false, false) => {
                                source_with_code_occurence_finder_vec_partial.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                    source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing.clone()),
                                    code_occurence: code_occurence_as_string.code_occurence.clone(),
                                    increment: code_occurence_as_string.increment.clone(),
                                });
                            },
                        }
                    },
                }
        });
        println!("111{:#?}111", source_with_code_occurence_handle_vec);
        println!("222{:#?}222", source_with_code_occurence_finder_vec_partial);
        println!("333{:#?}333", source_with_code_occurence_finder_vec_all);
        let mut stage_one_prep_hashmap: HashMap<
            tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder,
            Vec<String>,
            //or maybe Vec<tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceHandle>
        > = HashMap::new();
        //todo somewhere here lossing information about where vec of keys happens(code occurence)
        //or maybe its just not all hashmap keys we implemented
        source_with_code_occurence_finder_vec_partial.iter().for_each(|p|{
            source_with_code_occurence_handle_vec.iter().for_each(|origin|{
                let s_handle = match &origin.source {
                    tufa_common::common::source_and_code_occurence::SourceHandleEnum::SourceWithKeys(source_with_keys) => {
                        source_with_keys.source.clone()
                    },
                    tufa_common::common::source_and_code_occurence::SourceHandleEnum::Source(source) => {
                        source.clone()
                    },
                };
                let stage_one_keys = stage_one_prep_hashmap.iter().map(|(stage_key, _)|{
                    stage_key.source.clone()
                }).collect::<Vec<tufa_common::common::source_and_code_occurence::SourceFinderEnum>>();
                let mut handle_to_insert_vec: Vec<String> = Vec::new(); 
                match &p.source {
                    tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesForTracing(sources_for_tracing) => {
                        match sources_for_tracing.contains(&s_handle) {
                            true => {
                                handle_to_insert_vec.push(format!("{}{}{}{}", s_handle.clone(), symbol, origin.code_occurence, symbol));
                            },
                            false => (),
                        }
                    },
                    tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing) => {
                        match sources_and_keys_for_tracing.sources.contains(&s_handle) {
                            true => {
                                 handle_to_insert_vec.push(format!("{}{}{}{}", s_handle.clone(), symbol, origin.code_occurence, symbol));
                            },
                            false => (),
                        }
                    },
                }
                match stage_one_keys.contains(&p.source) {
                    true => {
                        let mut inner = match stage_one_prep_hashmap.get_mut(&p) {
                            Some(v) => v.clone(),
                            None => vec![],
                        };
                        handle_to_insert_vec.iter().for_each(|h|{
                            inner.push(h.clone());
                        });
                        *stage_one_prep_hashmap.entry(p.clone()).or_insert(vec![]) = inner.clone();
                    },
                    false => {
                        stage_one_prep_hashmap.insert(p.clone(), handle_to_insert_vec.clone());
                    },
                }
            });
        });
        // println!("444{:#?}444", stage_one_prep_hashmap);
        
        let mut stage_two_prep_hashmap: HashMap<
            tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder,
            String,
        > = HashMap::new();
        stage_one_prep_hashmap.iter().for_each(|(key, value)|{
            match &key.source {
                tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesForTracing(_) => {
                    let mut fold = value.iter().fold(String::from(""), |mut acc, v| {
                        let mut handle_v = v.lines().collect::<Vec<&str>>().iter().fold(
                            String::from(""),
                            |mut acc, element| {
                                acc.push_str(&format!(" {}{}", element, symbol));
                                acc
                            },
                        );
                        log_type.pop_last(&mut handle_v);
                        acc.push_str(&format!("{}{}", handle_v, symbol));
                        acc
                    });
                    log_type.pop_last(&mut fold);
                    stage_two_prep_hashmap.insert(key.clone(), format!("[{}{}{}]{}{}", symbol, fold, symbol, symbol, key.code_occurence));
                },
                tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing) => {
                    //todo - manage keys addition ordering with increments - maybe should add increment for each key and inside five() function add additional hashmap with errors?
                    //maybe add all vec to partial ?
                    let mut fold = value.iter().fold(String::from(""), |mut acc, v| {
                        acc.push_str(&format!("{}{}", v, symbol));
                        acc
                    });
                    log_type.pop_last(&mut fold);
                    let mut first = true;
                    let mut fold_with_keys = sources_and_keys_for_tracing.keys.iter().fold(String::from(""), |mut acc, k| {
                        match first {
                            true => {
                                let mut handle_fold = fold.lines().collect::<Vec<&str>>().iter().fold(
                                    String::from(""),
                                    |mut acc, element| {
                                        acc.push_str(&format!(" {}{}", element, symbol));
                                        acc
                                    },
                                );
                                log_type.pop_last(&mut handle_fold);
                                acc = format!("(key: {}) [{}{}{}]", k, symbol, handle_fold, symbol);
                                first = false;
                            },
                            false => {
                                let mut handle_acc = acc.lines().collect::<Vec<&str>>().iter().fold(
                                    String::from(""),
                                    |mut acc, element| {
                                        acc.push_str(&format!(" {}{}", element, symbol));
                                        acc
                                    },
                                );
                                log_type.pop_last(&mut handle_acc);
                                //maybe not correct logic for code_occurence
                                acc = format!("(key: {}) [{}{}{} {}{}]{}", k, symbol, handle_acc, symbol, key.code_occurence, symbol, symbol);
                            },
                        }
                        acc
                    });
                    log_type.pop_last(&mut fold_with_keys);
                    // let prep = format!("{}{}{}", fold_with_keys, symbol, key.code_occurence);
                    stage_two_prep_hashmap.insert(key.clone(), fold_with_keys);
                },
            }
        });
        stage_two_prep_hashmap.iter().for_each(|(k, v)|{
            // println!("{}", v)
        });
        // println!("555{:#?}555", stage_two_prep_hashmap);
        let prep_value = stage_two_prep_hashmap.iter().fold(
            String::from(""),
            |mut acc, (_, v)| {
                acc.push_str(&format!("{}{}", v, symbol));
                acc
            },
        );
        println!("prep_value\n{}\nprep_value", prep_value);
        source_with_code_occurence_finder_vec_all.sort_by(|a, b| a.increment.cmp(&b.increment));
        source_with_code_occurence_finder_vec_all.reverse();
        let mut first = true;
        let mut prepared_log = source_with_code_occurence_finder_vec_all.iter().fold(
            String::from(""),
            |mut acc, element| {
                match first {
                    true => {
                        let mut keys_from_partial = stage_two_prep_hashmap.iter().fold(vec![], |mut acc, (key,v)|{
                            match &key.source {
                                tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesForTracing(_) => (),
                                tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesAndKeysForTracing(sakft) => {
                                    sakft.keys.iter().for_each(|k|{
                                        acc.push(k.clone());
                                    });
                                },
                            }
                            acc
                        });
                        keys_from_partial = keys_from_partial.into_iter().unique().collect();
                        println!("{:#?}", keys_from_partial);
                        match &element.source {
                            tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesForTracing(_) => {
                                acc.push_str(&format!("{}{}", element.code_occurence, symbol));
                            },
                            tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesAndKeysForTracing(sources_and_keys_with_tracing) => {
                                let mut keys_not_in_the_partial = vec![];
                                sources_and_keys_with_tracing.keys.iter().for_each(|k|{
                                    match keys_from_partial.contains(k) {
                                        true => (),
                                        false => {
                                            keys_not_in_the_partial.push(k);
                                        },
                                    }
                                });
                                //
                                let mut handle_acc = prep_value.lines().collect::<Vec<&str>>().iter().fold(
                                    String::from(""),
                                    |mut accc, element| {
                                        accc.push_str(&format!("{}{}", element, symbol));
                                        accc
                                    },
                                );
                                log_type.pop_last(&mut handle_acc);
                                println!("handle_acc\n{}\n handle_acc", handle_acc);
                                match keys_not_in_the_partial.is_empty() {
                                    true => (),
                                    false => {
                                        keys_not_in_the_partial.iter().for_each(|kk|{
                                            handle_acc = format!("(key: {}) [{}{}{} {}{}]{}", kk, symbol, handle_acc, symbol, element.code_occurence, symbol, symbol);
                                        });
                                        // log_type.pop_last(&mut handle_acc);
                                    },
                                }
                                acc = handle_acc;
                                //maybe not correct logic for code_occurence
                            },
                        }
                        first = false;
                        acc
                    },
                    false => {
                        acc.push_str(&format!("{}{}", element.code_occurence, symbol));
                        acc
                    },
                }
            },
        );
        prepared_log.push_str(&format!(
            "{}",
            &self.get_code_occurence_as_string(config)
        ));
        // println!("@@{}@@", prepared_log);
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
        // println!("one f {}", std::mem::size_of_val(&f));
        // println!("one source {}", std::mem::size_of_val(&f.source));
        // println!("one source {}", std::mem::size_of_val(&f.code_occurence));
        // println!("one-----");
        // println!("{:#?}", f);
        f.log(once_cell::sync::Lazy::force(
            &crate::global_variables::runtime::config::CONFIG,
        ));
        // println!("one-----");
        return Err(Box::new(f));
    }
    Ok(())
}

//todo - must ininitalize with keys not in the get_inner_source_and_code_occurence_as_string - its one step ahead of actual place

// [
//   (key: five_hashmap_key) [
//     error_five
//     tufa_common/src/dev.rs:693:17
//   ]
//   (key: six_hashmap_key) [
//     [
//       error_seven
//       tufa_common/src/dev.rs:1090:17
//       error_eight
//       tufa_common/src/dev.rs:1176:17
//     ]
//     tufa_common/src/dev.rs:939:25
//   ]
// ]
// tufa_common/src/dev.rs:562:25
// tufa_common/src/dev.rs:167:21
// tufa_server/src/entry.rs:583:21

// (key: five_hashmap_key) [
//   error_five
//   tufa_common/src/dev.rs:719:17
// ]
// (key: six_hashmap_key) [
//   [
//     error_seven
//     tufa_common/src/dev.rs:1128:17
//     error_eight
//     tufa_common/src/dev.rs:1214:17
//   ]
//   tufa_common/src/dev.rs:977:25
// ]
// tufa_common/src/dev.rs:588:25
// tufa_common/src/dev.rs:191:21

// // tufa_server/src/entry.rs:583:21

// [
//     SourceAndCodeOccurenceAsString {
//         source: Some(
//             SourceWithKeys(
//                 SourceWithKeys {
//                     keys: [
//                         "six_hashmap_key",
//                     ],
//                     source: "error_seven",
//                 },
//             ),
//         ),
//         code_occurence: "tufa_common/src/dev.rs:1128:17",
//         increment: 3,
//     },
//     SourceAndCodeOccurenceAsString {
//         source: Some(
//             SourceWithKeys(
//                 SourceWithKeys {
//                     keys: [
//                         "six_hashmap_key",
//                     ],
//                     source: "error_eight",
//                 },
//             ),
//         ),
//         code_occurence: "tufa_common/src/dev.rs:1214:17",
//         increment: 3,
//     },
//     SourceAndCodeOccurenceAsString {
//         source: None,//todo add here info
//         code_occurence: "tufa_common/src/dev.rs:977:25",
//         increment: 2,
//     },
//     SourceAndCodeOccurenceAsString {
//         source: Some(
//             SourceWithKeys(
//                 SourceWithKeys {
//                     keys: [
//                         "five_hashmap_key",
//                     ],
//                     source: "error_five",
//                 },
//             ),
//         ),
//         code_occurence: "tufa_common/src/dev.rs:719:17",
//         increment: 2,
//     },
//     SourceAndCodeOccurenceAsString {
//         source: None,//todo add here info
//         code_occurence: "tufa_common/src/dev.rs:588:25",
//         increment: 1,
//     },
//     SourceAndCodeOccurenceAsString {
//         source: Some(
//             Keys(
//                 [
//                     "six_hashmap_key",
//                     "five_hashmap_key",
//                 ],
//             ),
//         ),
//         code_occurence: "tufa_common/src/dev.rs:191:21",
//         increment: 0,
//     },
// ]
