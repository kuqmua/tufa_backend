use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::hardcode::PROJECT_NAME;
use crate::global_variables::runtime::config::CONFIG;
use crate::preparation::prepare_server::prepare_server;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::server_wrapper::server_wrapper;
use crate::telemetry::get_subscriber::get_subscriber;
use crate::telemetry::init_subscriber::init_subscriber;
use tufa_common::config_mods::print_type::PrintType;
use tufa_common::config_mods::traits::fields::GetLogType;
use tufa_common::traits::get_git_source_file_link::GetGitSourceFileLink;
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
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Err(e) => {
            print_colorful_message(
                None,
                tufa_common::config_mods::print_type::PrintType::WarningHigh,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                format!("Cannot build tokio runtime {e:#?}"),
            );
        }
        Ok(runtime) => {
            one(true);
            if let tufa_common::config_mods::log_type::LogType::Tracing = CONFIG.log_type {
                if let Err(e) = init_subscriber(get_subscriber(
                    PROJECT_NAME.into(),
                    CONFIG.tracing_type.to_lower_snake_case(),
                    std::io::stdout,
                )) {
                    print_colorful_message(
                        None,
                        tufa_common::config_mods::print_type::PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                        format!("tracing init_subscriber error: {:#?}", e),
                    );
                    return;
                };
            }
            // let user = User {
            //     name: "Arwen Undomiel".to_string(),
            //     age: 3000,
            //     something: vec![true, false],
            //     address: Address {
            //         country: "Middle Earth".to_string(),
            //         city: "Rivendell".to_string(),
            //         street: "leafy lane".to_string(),
            //     },
            // };
            // tracing::error!(valuable = false, user = ?user);
            if let true = CONFIG.is_preparation_enabled {
                if runtime.block_on(prepare_server(true)).is_err() {
                    return;
                }
            }
            if let Err(e) = server_wrapper() {
                print_colorful_message(
                    None,
                    tufa_common::config_mods::print_type::PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                    format!("Cannot run actix-web HttpServer, error: {:#?}", e),
                );
            }
        }
    }
}

// use crate::global_variables::compile_time::git_info::GIT_INFO;
// use crate::global_variables::compile_time::git_info::GIT_INFO;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use tufa_common::common::code_occurence::FileLineColumn;
use tufa_common::common::code_occurence::TimeFileLineColumn;
use tufa_common::common::code_occurence::TimeFileLineColumnIncrement;
use tufa_common::config_mods::log_type::LogType;
use tufa_common::config_mods::source_place_type::SourcePlaceType;
use tufa_common::config_mods::tracing_type::TracingType;
use tufa_common::traits::code_occurence_methods::CodeOccurenceMethods;
use tufa_common::traits::get_color::ErrorColorBold;
use tufa_common::common::code_occurence::CodeOccurence;
use tufa_common::common::code_occurence::ThreeOriginError;
use tufa_common::common::git::git_info::GitInformationWithoutLifetimes;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::new_error::NewError;
use tufa_common::traits::with_tracing::WithTracing;
use impl_get_source::ImplGetSourceFromTufaCommon;
use tufa_common::traits::get_code_occurence::GetCodeOccurence;
use tufa_common::traits::log_error_code_occurence::LogErrorCodeOccurence;
use tufa_common::traits::get_source::GetSource;

#[derive(ImplGetSourceFromTufaCommon)]
pub struct OneWrapperError {
    source: OneWrapperErrorEnum,
    code_occurence: CodeOccurence,
}

impl GetCodeOccurence for OneWrapperError {
    fn get_code_occurence(&self) -> &CodeOccurence {
        &self.code_occurence
    }
}

// impl LogCodeOccurence for OneError {
//     fn log_code_occurence(
//         &self,
//         source_place_type: &SourcePlaceType,
//         log_type: LogType,
//         source: String,
//         style: ansi_term::Style,
//     ) {
//         self.code_occurence
//             .log_code_occurence(source_place_type, log_type, source, style);
//     }
// }

// #[derive(
//         // Debug,
//     // ImplDisplayForError,
//     ImplGetSourceFromTufaCommon,
//     // ImplGetWhereWasOriginOrWrapperFromTufaCommon,
// )]
pub enum OneWrapperErrorEnum {
    Three(ThreeOriginError),
}

impl GetSource for OneWrapperErrorEnum {
    fn get_source(&self) -> String {
        match self {
            OneWrapperErrorEnum::Three(t) => t.get_source(),
        }
    }
}

impl GetCodeOccurence for OneWrapperErrorEnum {
    fn get_code_occurence(&self) -> &tufa_common::common::code_occurence::CodeOccurence{
        match self {
            OneWrapperErrorEnum::Three(e) => e.get_code_occurence(),
        }
    }
}


pub trait WithTracingTest<T> {
    fn with_tracing_test(
        source: T,
        where_was: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
        source_place_type: &SourcePlaceType,
    ) -> Self;
}
 

pub trait NewErrorTest<T> {
    fn new_error_test(
        source: T,
        code_occurence: CodeOccurence,
    ) -> Self;
}

impl NewErrorTest<OneWrapperErrorEnum> for OneWrapperError {
    fn new_error_test(
        source: OneWrapperErrorEnum,
        code_occurence: CodeOccurence,
    ) -> Self {
        Self {
            source,
            code_occurence
        }
    }
}

pub trait InitErrorWithPossibleTraceTest<GenericErrorStruct, GenericErrorStructSource>
where
    GenericErrorStruct:
        WithTracingTest<GenericErrorStructSource> + NewErrorTest<GenericErrorStructSource>,
{
    fn init_error_with_possible_trace_test(
        source: GenericErrorStructSource,
        where_was: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
        source_place_type: &SourcePlaceType,
        should_trace: bool,
    ) -> Self;
}

// impl<GenericErrorStruct, GenericErrorStructSource>
//     InitErrorWithPossibleTraceTest<GenericErrorStruct, GenericErrorStructSource>
//     for GenericErrorStruct
// where
//     GenericErrorStruct:
//         WithTracingTest<GenericErrorStructSource> + NewErrorTest<GenericErrorStructSource>,
// {
//     fn init_error_with_possible_trace_test(
//         source: GenericErrorStructSource,
//         where_was: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
//         source_place_type: &SourcePlaceType,
//         should_trace: bool,
//     ) -> Self {
//         match should_trace {
//             true => Self::with_tracing_test(source, where_was, source_place_type),
//             false => Self::new_error_test(source, where_was),
//         }
//     }
// }

pub trait NewErrorTestTest<SourceGeneric, ConfigGeneric, ErrorColorBoldGeneric, ReturnSelfGeneric> {//
    fn new_error_test_test(
        source: SourceGeneric,
        config: ConfigGeneric,
        git_info: tufa_common::common::git::git_info::GitInformationWithoutLifetimes,
        file: String,
        line: u32,
        column: u32,
        should_trace: bool,
    ) -> ReturnSelfGeneric;
}
 
impl<SourceGeneric, ConfigGeneric, ErrorColorBoldGeneric, ReturnSelfGeneric> NewErrorTestTest<SourceGeneric, ConfigGeneric, ErrorColorBoldGeneric, ReturnSelfGeneric> for ReturnSelfGeneric
where 
    SourceGeneric: GetSource + GetCodeOccurence,
    ReturnSelfGeneric: NewErrorTest<SourceGeneric> + LogErrorCodeOccurence,
    ConfigGeneric: tufa_common::config_mods::traits::fields::GetSourcePlaceType + 
            tufa_common::config_mods::traits::fields::GetLogType + 
            tufa_common::traits::get_color::ErrorColorBold<ErrorColorBoldGeneric>
{
    fn new_error_test_test(
        source: SourceGeneric,
        config: ConfigGeneric,
        git_info: tufa_common::common::git::git_info::GitInformationWithoutLifetimes,
        file: String,
        line: u32,
        column: u32,
        should_trace: bool,
    ) -> ReturnSelfGeneric {
        let code_occurence = CodeOccurence::new(
                git_info, 
                file, 
                line, 
                column,
            ).add(source.get_code_occurence());
        let error = ReturnSelfGeneric::new_error_test(
            source,
            code_occurence,
        );
        if let true = should_trace {
            error.log_error_code_occurence(
                config.get_source_place_type(),
                config.get_log_type(),
                config.get_error_color_bold(),
            );
        }
        error
    }
}

pub fn one(should_trace: bool) -> Result<(), Box<OneWrapperError>> {
    if let Err(e) = tufa_common::common::code_occurence::three() {
        return Err(Box::new(OneWrapperError::new_error_test_test(
            OneWrapperErrorEnum::Three(*e), 
            once_cell::sync::Lazy::force(&crate::global_variables::runtime::config::CONFIG), 
            crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(), 
            String::from(file!()), 
            line!(), 
            column!(), 
            should_trace
        )));
    }
    Ok(())
}

pub struct TwoError {
    source: bool,
    code_occurence: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
}
