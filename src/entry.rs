use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::hardcode::PROJECT_NAME;
use crate::global_variables::runtime::config::CONFIG;
use crate::preparation::prepare_server::prepare_server;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::server_wrapper::server_wrapper;
use crate::telemetry::get_subscriber::get_subscriber;
use crate::telemetry::init_subscriber::init_subscriber;
use tufa_common::config_mods::print_type::PrintType;
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
            one();
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
use tufa_common::traits::get_color::ErrorColorBold;
// use tufa_common::common::code_occurence::CodeOccurence;
use tufa_common::common::code_occurence::FileLineColumn;
use tufa_common::common::code_occurence::TimeFileLineColumn;
use tufa_common::common::code_occurence::TimeFileLineColumnIncrement;
use tufa_common::config_mods::log_type::LogType;
use tufa_common::config_mods::source_place_type::SourcePlaceType;
use tufa_common::config_mods::tracing_type::TracingType;
use tufa_common::traits::code_occurence::CodeOccurence;

use tufa_common::common::code_occurence::ThreeError;
use tufa_common::common::git::git_info::GitInformationWithoutLifetimes;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::log_code_occurence::LogCodeOccurence;
use tufa_common::traits::new_error::NewError;
use tufa_common::traits::with_tracing::WithTracing;

pub struct OneError {
    source: OneErrorEnum,
    code_occurence: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
}

impl LogCodeOccurence for OneError {
    fn log_code_occurence(
        &self,
        source_place_type: &SourcePlaceType,
        log_type: LogType,
        source: String,
        style: ansi_term::Style,
    ) {
        self.code_occurence
            .log_code_occurence(source_place_type, log_type, source, style);
    }
}

pub enum OneErrorEnum {
    Three(ThreeError),
}

pub trait WithTracingTest<T> {
    fn with_tracing_test(
        source: T,
        where_was: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
        source_place_type: &SourcePlaceType,
    ) -> Self;
}

pub trait NewErrorTest<T> {
    fn new_test(
        source: T,
        where_was: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
    ) -> Self;
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

impl<GenericErrorStruct, GenericErrorStructSource>
    InitErrorWithPossibleTraceTest<GenericErrorStruct, GenericErrorStructSource>
    for GenericErrorStruct
where
    GenericErrorStruct:
        WithTracingTest<GenericErrorStructSource> + NewErrorTest<GenericErrorStructSource>,
{
    fn init_error_with_possible_trace_test(
        source: GenericErrorStructSource,
        where_was: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
        source_place_type: &SourcePlaceType,
        should_trace: bool,
    ) -> Self {
        match should_trace {
            true => Self::with_tracing_test(source, where_was, source_place_type),
            false => Self::new_test(source, where_was),
        }
    }
}

// pub trait GetCodeOccurence {
//     fn get_code_occurence(&self) -> HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>;
// }

// impl GetCodeOccurence for OneError {
//     fn get_code_occurence(&self) -> HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>> {
//         //todo match enum
//         self.code_occurence
//     }
// }

pub fn one() -> Result<(), Box<OneError>> {
    if let Err(e) = tufa_common::common::code_occurence::three() {
        let mut code_oc = HashMap::from([(
            crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
            vec![TimeFileLineColumnIncrement::new(
                String::from(file!()),
                line!(),
                column!(),
            )],
        )]);
        code_oc.add(e.code_occurence.clone());
        let f = Box::new(OneError {
            source: OneErrorEnum::Three(*e),
            code_occurence: code_oc,
        });
        f.log_code_occurence(
            &SourcePlaceType::Github,
            LogType::Stack,
            String::from("kekw"),
            CONFIG.get_error_color_bold(),
        );
        return Err(f);
    }
    Ok(())
}

pub struct TwoError {
    source: bool,
    code_occurence: HashMap<GitInformationWithoutLifetimes, Vec<TimeFileLineColumnIncrement>>,
}
