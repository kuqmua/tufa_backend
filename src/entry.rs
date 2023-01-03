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
            if let Some(source_enum) = &n.source {
                match &source_enum {
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
                    tufa_common::common::source_and_code_occurence::SourceEnum::KeysForTracing(keys) => {
                        keys.iter().for_each(|k| {
                            keys_for_tracing.push(k.clone());
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
            }
        });
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        keys_for_tracing = keys_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        let source_handle = match (sources_for_tracing.is_empty(), keys_for_tracing.is_empty()) {
            (true, true) => None,
            (true, false) => Some(
                tufa_common::common::source_and_code_occurence::SourceEnum::KeysForTracing(
                    keys_for_tracing,
                ),
            ),
            (false, true) => Some(
                tufa_common::common::source_and_code_occurence::SourceEnum::SourcesForTracing(
                    sources_for_tracing,
                ),
            ),
            (false, false) => Some(
                tufa_common::common::source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(
                    tufa_common::common::source_and_code_occurence::SourcesAndKeysForTracing {
                        sources: sources_for_tracing,
                        keys: keys_for_tracing,
                    },
                ),
            ),
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
            if let Some(source_handle) = &code_occurence_as_string.source {
                match source_handle {
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
                    tufa_common::common::source_and_code_occurence::SourceEnum::KeysForTracing(keys) => {
                        keys.iter().for_each(|k|{
                            keys_for_tracing.push(k.clone());
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
            }
        });
        sources_for_tracing = sources_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        keys_for_tracing = keys_for_tracing.into_iter().unique().collect(); //todo - optimize it?
        println!("{:#?}", sources_for_tracing);
        println!("{:#?}", keys_for_tracing);
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
            if let Some(source_handle) = &code_occurence_as_string.source {
                match source_handle {
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
                    tufa_common::common::source_and_code_occurence::SourceEnum::KeysForTracing(keys) => {
                        match sources_for_tracing.is_empty() {
                            true => {
                                let mut keys_handle = keys.clone();
                                keys_handle.sort();
                                let mut keys_for_tracing_handle = keys_for_tracing.clone();
                                keys_for_tracing_handle.sort();
                                let mut equal = false;
                                if keys_handle.len() == keys_for_tracing_handle.len() {
                                    equal = true;
                                    for index in 0..keys_handle.len() {
                                        //todo - match indexes
                                        if keys_handle[index] != keys_for_tracing_handle[index] {
                                            equal = false;
                                            break;
                                        }
                                    }
                                }
                                match equal {
                                    true => {
                                        source_with_code_occurence_finder_vec_all.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                            source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::KeysForTracing(keys.clone()),
                                            code_occurence: code_occurence_as_string.code_occurence.clone(),
                                            increment: code_occurence_as_string.increment.clone(),
                                        });
                                    },
                                    false => {
                                        source_with_code_occurence_finder_vec_partial.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                            source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::KeysForTracing(keys.clone()),
                                            code_occurence: code_occurence_as_string.code_occurence.clone(),
                                            increment: code_occurence_as_string.increment.clone(),
                                        });
                                    },
                                }
                            },
                            false => {
                                source_with_code_occurence_finder_vec_partial.push(tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder{
                                    source: tufa_common::common::source_and_code_occurence::SourceFinderEnum::KeysForTracing(keys.clone()),
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
                                let mut sources_equal = false;
                                if sources_handle.len() == sources_for_tracing_handle.len() {
                                    sources_equal = true;
                                    for index in 0..sources_handle.len() {
                                        //todo - match indexes
                                        if sources_handle[index] != sources_for_tracing_handle[index] {
                                            sources_equal = false;
                                            break;
                                        }
                                    }
                                }
                                //
                                let mut keys_handle = sources_and_keys_for_tracing.keys.clone();
                                sources_handle.sort();
                                let mut keys_for_tracing_handle = keys_for_tracing.clone();
                                keys_for_tracing_handle.sort();
                                let mut keys_equal = false;
                                if keys_handle.len() == keys_for_tracing_handle.len() {
                                    keys_equal = true;
                                    for index in 0..keys_handle.len() {
                                        //todo - match indexes
                                        if keys_handle[index] != keys_for_tracing_handle[index] {
                                            keys_equal = false;
                                            break;
                                        }
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
            }
        });
        println!("111{:#?}111", source_with_code_occurence_handle_vec);
        println!("222{:#?}222", source_with_code_occurence_finder_vec_partial);
        println!("333{:#?}333", source_with_code_occurence_finder_vec_all);
        let mut source_with_code_occurence_handle_with_finders_vec: Vec<(
            tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceHandle,
            Vec<tufa_common::common::source_and_code_occurence::SourceWithCodeOccurenceFinder>,
        )> = Vec::new();

        // source_with_code_occurence_handle_vec
        //     .iter()
        //     .for_each(|e| {
        //         source_with_code_occurence_finder_vec.iter().for_each(|f|{
        //             match f.source {
        //                 tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesForTracing(sources) => {
        //                     match sources.
        //                 },
        //                 tufa_common::common::source_and_code_occurence::SourceFinderEnum::KeysForTracing(keys) => {

        //                 },
        //                 tufa_common::common::source_and_code_occurence::SourceFinderEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing) => {

        //                 },
        //             }
        //         });
        //     });

        // let mut source_and_code_occurence_as_string_version_one = Vec::new();
        // let mut source_and_code_occurence_as_string_version_two = Vec::new();
        // let len = code_occurence_as_string_vec.len();
        // for c in &code_occurence_as_string_vec {
        //     // match &c.source {
        //     //     Some(_) => {

        //     //     },
        //     //     None => {
        //     //         source_and_code_occurence_as_string_version_one.push(tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsStringVersionOne{

        //     //         })
        //     //     },
        //     // }
        //     if let Some(source_enum) = &c.source {
        //         if let tufa_common::common::source_and_code_occurence::SourceEnum::SourceWithKeys(
        //             _,
        //         ) = source_enum
        //         {
        //             is_keys_exists = true;
        //             break;
        //         }
        //     }
        // }
        // let mut prepared_log = match is_keys_exists {
        //     true => {
        //         // let mut addition_to_increment_spaces = String::from("");
        //         let mut prepared_by_increments_hashmap: HashMap::<u64, Vec<tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString>> = HashMap::new();
        //         println!("{:#?}", code_occurence_as_string_vec);
        //         code_occurence_as_string_vec.iter().for_each(|element| {
        //             // println!("{:#?}", element);
        //             let mut should_insert_full_new = true;
        //             let mut fff = prepared_by_increments_hashmap.clone();
        //             for (k, v) in &mut fff {
        //                 if element.increment == *k {
        //                     //wrong logic!!!
        //                     should_insert_full_new = false;
        //                     if !v.contains(&element.clone()) {
        //                         let mut v_cloned = v.clone();
        //                         v_cloned.push(element.clone());
        //                         prepared_by_increments_hashmap.insert(element.increment, v_cloned);
        //                         break;
        //                     }

        //                     // break;
        //                 }
        //             }
        //             if should_insert_full_new {
        //                 prepared_by_increments_hashmap
        //                     .insert(element.increment, vec![element.clone()]);
        //             }
        //         });
        //         let mut prepared_by_increments_vec = Vec::new();
        //         prepared_by_increments_hashmap.iter().for_each(|(k, v)| {
        //             prepared_by_increments_vec.push((k, v));
        //         });
        //         prepared_by_increments_vec.sort_by(|(k1, v1), (k2, v2)| k1.cmp(k2));
        //         prepared_by_increments_vec.reverse();
        //         // println!("{:#?}", prepared_by_increments_vec);
        //         let mut content = ContentPrep {
        //             key_as_string: None,
        //             inner: String::from(""),
        //         };
        //         // println!("{:#?}", prepared_by_increments_vec);
        //         prepared_by_increments_vec.iter().for_each(|(_, v)| {
        //             let mut folded = v.iter().map(|element| {
        //                 // let mut increment_spaces = String::from("");
        //                 // for x in (0..element.increment) {
        //                 //     //0 or 1 ?
        //                 //     increment_spaces.push(' ');
        //                 // }
        //                 let prepare_for_log = match &element.source {
        //                     Some(source_enum) => match source_enum {
        //                         tufa_common::common::source_and_code_occurence::SourceEnum::SourceWithKeys(source_with_keys) => {
        //                             // let increment_spaces_prepared = increment_spaces.clone();
        //                             let mut prepared_keys = format!("{}[key: ", symbol);
        //                             // //todo maybe for each key add symbol and additional spaces for log structs where key is
        //                             source_with_keys.keys.iter().for_each(|e|{
        //                                 prepared_keys.push_str(e);
        //                                 prepared_keys.push_str(", ");
        //                             });
        //                             prepared_keys.pop();
        //                             prepared_keys.pop();
        //                             PrepareForLog {
        //                                 error_as_string: Some(format!("{}] {}", prepared_keys, source_with_keys.source)),
        //                                 code_occurences_as_string: element.code_occurence.clone(),
        //                             }
        //                             // format!("{}] {} {}{}{}", prepared_keys, source_with_keys.source, symbol, element.code_occurence, symbol)
        //                         },
        //                         tufa_common::common::source_and_code_occurence::SourceEnum::Source(source) => {
        //                             PrepareForLog {
        //                                 error_as_string: Some(source.clone()),
        //                                 code_occurences_as_string: element.code_occurence.clone(),
        //                             }
        //                             // format!("{}{}{}{}{}", symbol, source, symbol, element.code_occurence, symbol)
        //                         },
        //                         tufa_common::common::source_and_code_occurence::SourceEnum::SourcesForTracing(sources) => {
        //                             println!("sources todo");
        //                             PrepareForLog {
        //                                 error_as_string: None,
        //                                 code_occurences_as_string: String::from(""),
        //                             }
        //                         },
        //                         tufa_common::common::source_and_code_occurence::SourceEnum::KeysForTracing(keys) => {
        //                             println!("keys todo");
        //                             PrepareForLog {
        //                                 error_as_string: None,
        //                                 code_occurences_as_string: String::from(""),
        //                             }
        //                         },
        //                         tufa_common::common::source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing) => {
        //                             println!("sources_and_keys_for_tracing");
        //                             PrepareForLog {
        //                                 error_as_string: None,
        //                                 code_occurences_as_string: String::from(""),
        //                             }
        //                         }
        //                     },
        //                     None => {
        //                         PrepareForLog {
        //                             error_as_string: None,
        //                             code_occurences_as_string: element.code_occurence.clone(),
        //                         }
        //                         // format!("{}{}{}", symbol, element.code_occurence, symbol)
        //                     },
        //                 };
        //                 // log_type.pop_last(&mut formatted_handle);
        //                 // acc.push_str(&formatted_handle);
        //                 // acc.push_str(&format!("{}{}", formatted_handle, symbol));
        //                 // log_type.pop_last(&mut acc);
        //                 // println!("--{}--", acc);
        //                 // acc
        //                 prepare_for_log
        //             }).collect::<Vec<PrepareForLog>>();
        //             // println!("folded {:#?}", folded);
        //             // let content_part = format!("[{}{}]", folded, symbol);
        //             // println!("{}", content_part);
        //             // content.push_str(&folded);
        //             // content = content_part
        //             // println!("LEN{}LEN", folded.len());
        //             folded.sort_by(|a, b| a.error_as_string.cmp(&b.error_as_string));
        //             folded.reverse();
        //             match folded.len() == 1 {
        //                 true => {
        //                     match content.inner.is_empty() {
        //                         true => {
        //                             // println!("CONTENT INNER IS EMPTY AND LEN IS 1");
        //                             match &folded[0].error_as_string {
        //                                 Some(k) => {
        //                                     content = ContentPrep {
        //                                         key_as_string: Some(k.clone()),
        //                                         inner: format!("[{}{}{}]", symbol, folded[0].code_occurences_as_string.clone(), symbol),
        //                                     }
        //                                 },
        //                                 None => {
        //                                     content = ContentPrep {
        //                                         key_as_string: None,
        //                                         inner: folded[0].code_occurences_as_string.clone(),
        //                                     }
        //                                 },
        //                             }
        //                         },
        //                         false => {
        //                             // println!("CONTENT INNER IS NOT EMPTY AND LEN IS 1");
        //                             match &folded[0].error_as_string {
        //                                 Some(eas) => {
        //                                     match &content.key_as_string {
        //                                         Some(ckey) => {
        //                                             content = ContentPrep {
        //                                                 key_as_string: Some(eas.clone()),
        //                                                 inner: format!("{}[{}{}{}]{}", ckey, symbol, folded[0].code_occurences_as_string.clone(), symbol, content.inner),
        //                                             }
        //                                         },
        //                                         None => {
        //                                             content = ContentPrep {
        //                                                 key_as_string: Some(eas.clone()),
        //                                                 inner: format!("{}[{}{}]{}{}", symbol, folded[0].code_occurences_as_string.clone(), symbol, symbol, content.inner),
        //                                             }
        //                                         },
        //                                     }
        //                                 },
        //                                 None => {
        //                                     match &content.key_as_string {
        //                                         Some(k) => {
        //                                             content = ContentPrep {
        //                                                 key_as_string: None,
        //                                                 inner: format!("{}{}{}{}{}{}", symbol, k, symbol, content.inner, symbol, folded[0].code_occurences_as_string.clone()),
        //                                             }
        //                                         },
        //                                         None => {
        //                                             content = ContentPrep {
        //                                                 key_as_string: None,
        //                                                 inner: format!("{}{}{}{}", symbol, content.inner, symbol, folded[0].code_occurences_as_string.clone()),
        //                                             }
        //                                         },
        //                                     }
        //                                 },
        //                             }
        //                         },
        //                     }
        //                 },
        //                 false => {
        //                     let fold = folded.iter()
        //                     .fold(String::from(""), |mut acc, element| {
        //                         match &element.error_as_string {
        //                             Some(ke) => {
        //                                 acc.push_str(&format!("{}{}{}{}", symbol, ke, symbol, element.code_occurences_as_string));
        //                             },
        //                             None => {
        //                                 acc.push_str(&format!("{}{}", symbol, element.code_occurences_as_string));
        //                             },
        //                         }
        //                         acc
        //                     });
        //                     match &content.inner.is_empty() {
        //                         true => {
        //                             //then content.inner.is_empty - content.key_as_string must be None
        //                             match &content.key_as_string {
        //                                 Some(k) => {
        //                                     content = ContentPrep {
        //                                         key_as_string: None,
        //                                         inner: format!("{}{}{}", symbol, k, fold),
        //                                     }
        //                                 },
        //                                 None => {
        //                                     content = ContentPrep {
        //                                         key_as_string: None,
        //                                         inner: fold,
        //                                     }
        //                                 },
        //                             }
        //                         },
        //                         false => {
        //                             match &content.key_as_string {
        //                                 Some(ke) => {
        //                                     content = ContentPrep {
        //                                         key_as_string: None,
        //                                         inner: format!("{}[{}{}{}{}]", fold, symbol, ke, content.inner, symbol),
        //                                     }
        //                                 },
        //                                 None => {
        //                                     content = ContentPrep {
        //                                         key_as_string: None,
        //                                         inner: format!("{}{}{}{}", fold, symbol, content.inner, symbol),
        //                                     }
        //                                 },
        //                             }
        //                         },
        //                     }
        //                 },
        //             }
        //             // println!("{:#?}", content);
        //         });

        //         // println!("LAST{:#?}LAST", content);
        //         // content.key_as_string = Some(String::from("kekw for test"));
        //         let prepared_content = match content.key_as_string {
        //             Some(key) => {
        //                 let prepared_inner =
        //                     content.inner.lines().collect::<Vec<&str>>().iter().fold(
        //                         String::from(""),
        //                         |mut acc, element| {
        //                             acc.push_str(&format!(" {}{}", element, element));
        //                             acc
        //                         },
        //                     );
        //                 format!("{} [{}{}{}]", key, symbol, prepared_inner, symbol)
        //             }
        //             None => content.inner,
        //         };
        //         // println!("{}", prepared_content);
        //         prepared_content
        //     }
        //     false => {
        //         code_occurence_as_string_vec
        //             .iter()
        //             .fold(String::from(""), |mut acc, element| {
        //                 // println!("{:#?}", element);
        //                 let mut increment_spaces = String::from("");
        //                 for x in (0..element.increment) {
        //                     //0 or 1 ?
        //                     increment_spaces.push(' ');
        //                 }
        //                 let formatted_handle = match &element.source {
        //                     Some(source_enum) => {
        //                         match source_enum {
        //                             tufa_common::common::source_and_code_occurence::SourceEnum::SourceWithKeys(_) => String::from(""),//todo rewrite it
        //                             tufa_common::common::source_and_code_occurence::SourceEnum::Source(source) => format!("{}{}{}{}", increment_spaces, element.code_occurence, symbol, element.code_occurence),
        //                             tufa_common::common::source_and_code_occurence::SourceEnum::SourcesForTracing(sources) => String::from(""),//todo rewrite it
        //                             tufa_common::common::source_and_code_occurence::SourceEnum::KeysForTracing(keys) => String::from(""),//todo rewrite it
        //                             tufa_common::common::source_and_code_occurence::SourceEnum::SourcesAndKeysForTracing(sources_and_keys_for_tracing) => String::from(""),//todo rewrite it
        //                         }
        //                     },
        //                     None => format!("{}{}", increment_spaces, element.code_occurence),
        //                 };
        //                 acc.push_str(&format!("{}{}", formatted_handle, symbol));
        //                 acc
        //             })
        //     }
        // };
        let mut prepared_log = String::from("");
        prepared_log.push_str(&format!(
            "{}{}",
            symbol,
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
