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
        log_type.console(
            &config.get_error_color_bold(),
            // self.get_source_and_code_occurence_as_string(config),
            format!(
                "{}{}{}",
                self.get_source_as_string(config),
                config.get_log_type().symbol(),
                self.get_code_occurence_as_string(config)
            ),
        )
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
