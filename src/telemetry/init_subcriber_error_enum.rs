use tracing::dispatcher::SetGlobalDefaultError;
use tracing::log::SetLoggerError;

#[derive(thiserror::Error, Debug)]
pub enum InitSubcriberErrorEnum {
    SetLogger {
        #[from]
        source: SetLoggerError,
    },
    SetGlobalDefault {
        #[from]
        source: SetGlobalDefaultError,
    },
}

impl std::fmt::Display for InitSubcriberErrorEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
