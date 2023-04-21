
pub fn entry() {
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Err(e) => {
            crate::prints::print_colorful_message::print_colorful_message(
                None,
                tufa_common::config_mods::print_type::PrintType::WarningHigh,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![{
                    use tufa_common::traits::get_git_source_file_link::GetGitSourceFileLink;
                    crate::global_variables::compile_time::git_info::GIT_INFO.get_git_source_file_link(file!(), line!())
                }],
                format!("Cannot build tokio runtime {e:#?}"),
            );
        }
        Ok(runtime) => {
            if let (
                tufa_common::config_mods::log_type::LogType::Tracing, 
                Err(e)
            ) = (
                crate::global_variables::runtime::config::CONFIG.log_type,
                crate::telemetry::init_subscriber::init_subscriber(
                    crate::telemetry::get_subscriber::get_subscriber(
                    crate::global_variables::hardcode::PROJECT_NAME.into(),
                    crate::global_variables::runtime::config::CONFIG.tracing_type.to_lower_snake_case(),
                    std::io::stdout,
                ))
            ) {
                crate::prints::print_colorful_message::print_colorful_message(
                    None,
                    tufa_common::config_mods::print_type::PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![{
                        use tufa_common::traits::get_git_source_file_link::GetGitSourceFileLink;
                        crate::global_variables::compile_time::git_info::GIT_INFO.get_git_source_file_link(file!(), line!())
                    }],
                    format!("tracing init_subscriber error: {:#?}", e),
                );
                return;
            }
            else {
                if let (
                    true, 
                    Err(e)
                ) = (
                    crate::global_variables::runtime::config::CONFIG.is_preparation_enabled,
                    runtime.block_on(crate::preparation::prepare_server::prepare_server(true))
                ) {
                    use tufa_common::traits::error_logs_logic::error_log::ErrorLog;
                    e.error_log(once_cell::sync::Lazy::force(
                        &crate::global_variables::runtime::config::CONFIG,
                    ));
                }
                // if let Err(e) = crate::server_wrapper::server_wrapper() {
                //     crate::prints::print_colorful_message::print_colorful_message(
                //         None,
                //         tufa_common::config_mods::print_type::PrintType::WarningHigh,
                //         vec![format!("{}:{}:{}", file!(), line!(), column!())],
                //         vec![{
                //             use tufa_common::traits::get_git_source_file_link::GetGitSourceFileLink;
                //             crate::global_variables::compile_time::git_info::GIT_INFO.get_git_source_file_link(file!(), line!())
                //         }],
                //         format!("Cannot run actix-web HttpServer, error: {:#?}", e),
                //     );
                // }
            }
        }
    }
}
