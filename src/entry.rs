use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::hardcode::PROJECT_NAME;
use crate::global_variables::runtime::config::CONFIG;
use crate::preparation::prepare_server::prepare_server;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::server_wrapper::server_wrapper;
use crate::telemetry::get_subscriber::get_subscriber;
use crate::telemetry::init_subscriber::init_subscriber;
use tufa_common::config_mods::print_type::PrintType;
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
use tufa_common::common::where_was::GitInfoForWhereWas;
use tufa_common::config_mods::log_type::LogType;
use tufa_common::config_mods::source_place_type::SourcePlaceType;
use tufa_common::config_mods::tracing_type::TracingType;
use tufa_common::traits::code_occurence::CodeOccurenceTrait;

use tufa_common::common::code_occurence::ThreeError;

pub struct OneError {
    source: OneErrorEnum,
    code_occurence: HashMap<GitInfoForWhereWas, Vec<TimeFileLineColumnIncrement>>,
}

pub enum OneErrorEnum {
    Three(ThreeError),
}

// pub trait GetCodeOccurence {
//     fn get_code_occurence(&self) -> HashMap<GitInfoForWhereWas, Vec<TimeFileLineColumnIncrement>>;
// }

// impl GetCodeOccurence for OneError {
//     fn get_code_occurence(&self) -> HashMap<GitInfoForWhereWas, Vec<TimeFileLineColumnIncrement>> {
//         //todo match enum
//         self.code_occurence
//     }
// }

pub fn one() -> Result<(), Box<OneError>> {
    if let Err(e) = tufa_common::common::code_occurence::three() {
        let mut code_oc = HashMap::from([(
            GitInfoForWhereWas {
                commit_id: String::from(GIT_INFO.commit_id),
                repo_link: String::from(GIT_INFO.repo_link),
                author: String::from(GIT_INFO.author),
                author_email: String::from(GIT_INFO.author_email),
                commit_unix_time: String::from(GIT_INFO.commit_unix_time),
                timezone: String::from(GIT_INFO.timezone),
                message: String::from(GIT_INFO.message),
            },
            vec![TimeFileLineColumnIncrement {
                increment: 0,
                value: TimeFileLineColumn {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file_line_column: FileLineColumn {
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                    },
                },
            }],
        )]);
        code_oc.add(e.code_occurence.clone());
        let f = Box::new(OneError {
            source: OneErrorEnum::Three(*e),
            code_occurence: code_oc,
        });
        f.code_occurence.log(
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
    code_occurence: HashMap<GitInfoForWhereWas, Vec<TimeFileLineColumnIncrement>>,
}
