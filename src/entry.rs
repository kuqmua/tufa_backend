use crate::helpers::git_info::GIT_INFO;
use crate::preparation::preparation;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;
use crate::project_constants::PROJECT_NAME;
use crate::server_wrapper::server_wrapper;
use crate::telemetry::get_subscriber;
use crate::telemetry::get_subscriber::get_subscriber;
use crate::telemetry::init_subscriber;
use crate::telemetry::init_subscriber::init_subscriber;
use std::time::Instant;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn entry() {
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::Error,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                format!("Cannot build tokio runtime {e:#?}"),
            );
        }
        Ok(runtime) => {
            //disable tracing to remove useless spam for now
            if let Err(e) = init_subscriber(get_subscriber(
                PROJECT_NAME.into(),
                "info".into(),
                std::io::stdout,
            )) {
                print_colorful_message(
                    None,
                    PrintType::Error,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                    format!("tracing init_subscriber error: {:#?}", e),
                );
                return;
            };
            if let Err(e) = runtime.block_on(preparation()) {
                println!("{e}");
                // print_colorful_message(
                //     None,
                //     PrintType::Error,
                //     vec![format!("{}:{}:{}", file!(), line!(), column!())],
                //     vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                //     format!("preparation failed, error:\n {}", *e),
                // );
                return;
            }
            if let Err(e) = server_wrapper() {
                print_colorful_message(
                    None,
                    PrintType::Error,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                    format!("Cannot run actix-web HttpServer, error: {:#?}", e),
                );
            }
        }
    }
}
