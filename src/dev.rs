// use crate::global_variables::runtime::config::CONFIG;
// use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
use itertools::Itertools;
use tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString;
use tufa_common::dev::ThreeWrapperError;
use tufa_common::traits::code_path::CodePath;
use tufa_common::traits::config_log::ConfigLog;
use tufa_common::traits::console::Console;
use tufa_common::traits::fields::GetLogType;
use tufa_common::traits::fields::GetSourcePlaceType;
use tufa_common::traits::get_color::ErrorColorBold;
use tufa_common::traits::separator_symbol::SeparatorSymbol;

pub fn dev() {
    let _f = one(true);
    if let Err(e) = _f {
        println!("{}", e);
    }
}

#[derive(Debug)]
pub struct OneWrapperError {
    source: OneWrapperErrorEnum,
    code_occurence: tufa_common::common::code_occurence::CodeOccurenceOldWay,
}

impl std::fmt::Display for OneWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let config =
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG);
        write!(
            f,
            "{}{}{} {}",
            self.source,
            config.symbol(),
            self.code_occurence
                .get_code_path(config.get_source_place_type()),
            chrono::DateTime::<chrono::Utc>::from(
                std::time::UNIX_EPOCH + self.code_occurence.time_file_line_column.time,
            )
            .with_timezone(&chrono::FixedOffset::east_opt(3 * 3600).unwrap())
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
        )
    }
}

impl OneWrapperError {
    pub fn get_source_as_string(
        &self,
        config: &tufa_common::config_mods::config_struct::ConfigStruct,
    ) -> String {
        self.source.get_source_as_string(config)
    }
    pub fn get_code_occurence_as_string(
        &self,
        config: &tufa_common::config_mods::config_struct::ConfigStruct,
    ) -> String {
        self.code_occurence
            .get_code_path(config.get_source_place_type())
    }
    pub fn get_inner_source_and_code_occurence_as_string(
        &self,
        config: &tufa_common::config_mods::config_struct::ConfigStruct, //todo maybe remove
    ) -> Vec<tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString> {
        let vec = self
            .source
            .get_inner_source_and_code_occurence_as_string(config);
        let mut sources_for_tracing: Vec<
            Vec<(
                tufa_common::common::source_and_code_occurence::Source,
                Vec<tufa_common::common::source_and_code_occurence::Key>,
            )>,
        > = Vec::with_capacity(
            vec.iter()
                .map(|e| e.source.len())
                .collect::<Vec<usize>>()
                .iter()
                .sum(),
        );
        let vec_capacity = vec.len() + 1;
        let mut new_vec =
            vec.into_iter()
                .fold(Vec::with_capacity(vec_capacity), |mut acc, mut s| {
                    s.source.iter().for_each(|v| {
                        sources_for_tracing.push(v.clone());
                    });
                    s.add_one();
                    acc.push(s);
                    acc
                });
        new_vec.push(
            tufa_common::common::source_and_code_occurence::SourceAndCodeOccurenceAsString {
                source: sources_for_tracing,
                code_occurence: self.get_code_occurence_as_string(config),
                increment: 0,
            },
        );
        new_vec
    }
    pub fn log(&self, config: &tufa_common::config_mods::config_struct::ConfigStruct) {
        let code_occurence_as_string_vec = self
            .source
            .get_inner_source_and_code_occurence_as_string(config);
        if let Some(last) = code_occurence_as_string_vec.last() {
            if let true = last.increment == 0 {
                let (mut sources_all, mut keys_all) =
                    last.source
                        .iter()
                        .fold((Vec::new(), Vec::new()), |mut acc, v| {
                            v.iter().for_each(|(source, keys)| {
                                acc.0.push(source.clone());
                                keys.iter().for_each(|k| {
                                    acc.1.push(k.clone());
                                });
                            });
                            acc
                        });
                sources_all = sources_all.into_iter().unique().collect(); //todo - optimize it?
                sources_all.sort_by(|a, b| b.uuid.cmp(&a.uuid));
                keys_all = keys_all.into_iter().unique().collect(); //todo - optimize it?
                keys_all.sort_by(|a, b| b.uuid.cmp(&a.uuid));
                let sources_all_len = sources_all.len();
                let keys_all_len = keys_all.len();
                let (originals, mut additions_partial, mut additions_all) =
                    code_occurence_as_string_vec.into_iter().fold(
                        (Vec::new(), Vec::new(), Vec::new()),
                        |mut accc, c| {
                            let is_original = match c.source.len() == 1 {
                                true => match c.source.get(0) {
                                    Some(first_element) => match first_element.len() == 1 {
                                        true => match first_element.get(0) {
                                            Some(first_element_of_the_first_element) => {
                                                match first_element_of_the_first_element
                                                    .1
                                                    .is_empty()
                                                {
                                                    true => true,
                                                    false => false,
                                                }
                                            }
                                            None => false,
                                        },
                                        false => false,
                                    },
                                    None => false,
                                },
                                false => false,
                            };
                            match is_original {
                                true => {
                                    accc.0.push(c);
                                }
                                false => {
                                    let (mut local_sources, mut local_keys) = c.source.iter().fold(
                                        (Vec::new(), Vec::new()),
                                        |mut acc, v| {
                                            v.iter().for_each(|(source, vecc)| {
                                                acc.0.push(source);
                                                vecc.iter().for_each(|ve| {
                                                    acc.1.push(ve);
                                                });
                                            });
                                            acc
                                        },
                                    );
                                    local_sources = local_sources.into_iter().unique().collect(); //todo - optimize it?
                                    local_sources.sort_by(|a, b| b.uuid.cmp(&a.uuid));
                                    local_keys = local_keys.into_iter().unique().collect(); //todo - optimize it?
                                    if let (true, true) = (
                                        sources_all_len == local_sources.len(),
                                        keys_all_len == local_keys.len(),
                                    ) {
                                        let mut sources_equal = true;
                                        //todo: maybe need to check keys too?
                                        for i in 0..local_sources.len() {
                                            match local_sources.get(i) {
                                                Some(local_sources_element) => {
                                                    match sources_all.get(i) {
                                                        Some(sources_all_element) => {
                                                            if let false = *local_sources_element
                                                                == sources_all_element
                                                            {
                                                                sources_equal = false;
                                                                break;
                                                            }
                                                        }
                                                        None => {
                                                            sources_equal = false;
                                                            break;
                                                        }
                                                    }
                                                }
                                                None => {
                                                    sources_equal = false;
                                                    break;
                                                }
                                            }
                                        }
                                        match sources_equal {
                                            true => {
                                                accc.2.push(c);
                                            }
                                            false => {
                                                accc.1.push(c);
                                            }
                                        }
                                    } else {
                                        accc.1.push(c);
                                    }
                                }
                            }
                            accc
                        },
                    );
                additions_all.sort_by(|a, b| b.increment.cmp(&a.increment));
                let cannot_get_source_handle =
                    tufa_common::common::source_and_code_occurence::Source {
                        source: String::from("cannot get source"),
                        uuid: uuid::Uuid::new_v4(),
                    };
                let symbol = config.symbol();
                let lined = additions_partial
                    .iter_mut()
                    .fold(String::from(""), |mut acccc, o| {
                        let vec_of_origins = o
                            .source
                            .iter()
                            .fold(
                                Vec::with_capacity(
                                    o.source
                                        .iter()
                                        .map(|v| v.len())
                                        .collect::<Vec<usize>>()
                                        .iter()
                                        .sum(),
                                ),
                                |mut acc, v| {
                                    v.iter().for_each(|(source, _vec)| {
                                        originals.iter().for_each(|a| {
                                            let mut contains = false;
                                            for v in &a.source {
                                                let mut inner_contains = false;
                                                for (s, _vec) in v {
                                                    if let true = source == s {
                                                        inner_contains = true;
                                                        break;
                                                    }
                                                }
                                                if let true = inner_contains {
                                                    contains = true;
                                                    break;
                                                }
                                            }
                                            if let true = contains {
                                                acc.push(a.clone());
                                            }
                                        });
                                    });
                                    acc
                                },
                            )
                            .into_iter()
                            .unique()
                            .collect::<Vec<SourceAndCodeOccurenceAsString>>();
                        o.source.iter_mut().for_each(|v| {
                            v.iter_mut().for_each(|(source, vec)| {
                                let mut equals = None;
                                if let Some(additions_all_first_element) = additions_all.get(0) {
                                    for vv in &additions_all_first_element.source {
                                        let mut contains = None;
                                        for (source_in_all, vec_in_all) in vv {
                                            if let true = source == source_in_all {
                                                contains = Some(vec_in_all);
                                                break;
                                            }
                                        }
                                        if let Some(vf) = contains {
                                            equals = Some(vf);
                                            break;
                                        }
                                    }
                                }
                                if let Some(vvv) = equals {
                                    //not sure about ordering
                                    vvv.iter().for_each(|vvve| {
                                        if let false = vec.contains(vvve) {
                                            vec.push(vvve.clone());
                                        }
                                    });
                                }
                            });
                        });
                        let mut local_keys = o.source.iter().fold(Vec::new(), |mut acc, v| {
                            v.iter().for_each(|(_source, vecc)| {
                                vecc.iter().for_each(|ve| {
                                    acc.push(ve.clone());
                                });
                            });
                            acc
                        });
                        local_keys = local_keys.into_iter().unique().collect(); //todo - optimize it
                        match local_keys.is_empty() {
                            true => {
                                let fold_original_source_lines =
                                    vec_of_origins.iter().fold(String::from(""), |mut acc, o| {
                                        let source = match o.source.first() {
                                            Some(first_element) => match first_element.first() {
                                                Some(first_inner_element) => &first_inner_element.0,
                                                None => &cannot_get_source_handle,
                                            },
                                            None => &cannot_get_source_handle,
                                        };
                                        acc.push_str(&format!(
                                            "  {}{}  {}{}",
                                            source.source, symbol, o.code_occurence, symbol
                                        ));
                                        acc
                                    });
                                acccc.push_str(&format!(
                                    " [{}{} ]{} {}{}",
                                    symbol,
                                    fold_original_source_lines,
                                    symbol,
                                    o.code_occurence,
                                    symbol
                                ))
                            }
                            false => {
                                let mut first = true;
                                let handle = local_keys.into_iter().fold(
                                    String::from(""),
                                    |mut acc, local_key| {
                                        match first {
                                            true => {
                                                let fold_lines = vec_of_origins.iter().fold(
                                                    String::from(""),
                                                    |mut acc, o| {
                                                        let source = match o.source.first() {
                                                            Some(first_element) => {
                                                                match first_element.first() {
                                                                    Some(first_inner_element) => {
                                                                        &first_inner_element.0
                                                                    }
                                                                    None => {
                                                                        &cannot_get_source_handle
                                                                    }
                                                                }
                                                            }
                                                            None => &cannot_get_source_handle,
                                                        };
                                                        acc.push_str(&format!(
                                                            "  {}{}  {}{}",
                                                            source.source,
                                                            symbol,
                                                            o.code_occurence,
                                                            symbol
                                                        ));
                                                        acc
                                                    },
                                                );
                                                acc.push_str(&format!(
                                                    " {} [{}{} ]{} {}",
                                                    local_key.key,
                                                    symbol,
                                                    fold_lines,
                                                    symbol,
                                                    o.code_occurence
                                                ));
                                                first = false;
                                            }
                                            false => {
                                                acc = format!(
                                                    " {} [{}{} ]",
                                                    local_key.key,
                                                    symbol,
                                                    acc.lines()
                                                        .collect::<Vec<&str>>()
                                                        .into_iter()
                                                        .fold(
                                                            String::from(""),
                                                            |mut acc_inner, element| {
                                                                acc_inner.push_str(&format!(
                                                                    " {}{}",
                                                                    element, symbol
                                                                ));
                                                                acc_inner
                                                            }
                                                        )
                                                );
                                            }
                                        }
                                        acc
                                    },
                                );
                                acccc.push_str(&format!("{}{}", handle, symbol));
                            }
                        }
                        acccc
                    });
                let mut prepared_log = format!("[{}{}]{}", symbol, lined, symbol);
                additions_all.into_iter().for_each(|value| {
                    prepared_log.push_str(&format!("{}{}", value.code_occurence, symbol));
                });
                prepared_log.push_str(&self.get_code_occurence_as_string(config));
                config.log(prepared_log);
            }
        }
    }
}

#[derive(Debug)]
pub enum OneWrapperErrorEnum {
    ThreeWrapper(ThreeWrapperError),
}

impl std::fmt::Display for OneWrapperErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OneWrapperErrorEnum::ThreeWrapper(e) => write!(f, "{}", e),
        }
    }
}

impl OneWrapperErrorEnum {
    fn get_source_as_string(
        &self,
        config: &tufa_common::config_mods::config_struct::ConfigStruct,
    ) -> String {
        match self {
            //todo if origin - without config, if wrapper - with config
            OneWrapperErrorEnum::ThreeWrapper(i) => i.get_source_as_string(config),
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
