use impl_display::ImplDisplayDerive;
use std::fmt;
use tokio::task::JoinHandle;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::log::SetLoggerError;
use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_bunyan_formatter::JsonStorageLayer;
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // This "weird" syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink implements the `MakeWriter`
    // trait for all choices of the lifetime parameter `'a`
    // Check out https://doc.rust-lang.org/nomicon/hrtb.html
    // for more details.
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

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

pub fn init_subscriber(
    subscriber: impl Subscriber + Send + Sync,
) -> Result<(), InitSubcriberErrorEnum> {
    LogTracer::init()?;
    set_global_default(subscriber)?;
    Ok(())
}
