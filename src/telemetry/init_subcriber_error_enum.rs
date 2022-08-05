use impl_display::ImplDisplayDerive;
use std::fmt;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::log::SetLoggerError;

#[derive(thiserror::Error, Debug, ImplDisplayDerive)]
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
