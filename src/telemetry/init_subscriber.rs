pub fn init_subscriber<'a>(
    subscriber: impl tracing::Subscriber + Send + Sync,
) -> Result<(), tufa_common::repositories_types::tufa_server::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum<'a>>{
    if let Err(e) = tracing_log::LogTracer::init() {
        return Err(tufa_common::repositories_types::tufa_server::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum::SetLogger {
            error: e.to_string(),
            code_occurence: tufa_common::code_occurence!(),
        });
    }
    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        return Err(tufa_common::repositories_types::tufa_server::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum::SetGlobalDefault {
            error: e.to_string(),
            code_occurence: tufa_common::code_occurence!(),
        });
    }
    Ok(())
}
