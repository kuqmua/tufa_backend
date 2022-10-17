use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSource;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitError,
    ImplErrorWithTracingForStructWithoutGetSource,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
)]
pub struct AsyncClientBuilderError {
    source: reqwest::Error,
    where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn async_client_builder(
    client_builder: reqwest::ClientBuilder,
    should_trace: bool,
) -> Result<reqwest::Client, Box<AsyncClientBuilderError>> {
    match client_builder.build() {
        Err(e) => Err(Box::new(
            AsyncClientBuilderError::init_error_with_possible_trace(
                e,
                WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ),
        )),
        Ok(request_builder) => Ok(request_builder),
    }
}

// <
//     UserAgentValue,
//     CookieProvider: CookieStore + 'static,
//     PoolIdleTimeoutDuration,
// >

// where
//     UserAgentValue: TryInto<HeaderValue>,
//     UserAgentValue::Error: Into<actix_web::http::Error>,
//     PoolIdleTimeoutDuration: Into<Option<Duration>>,

// link: &str,
// headers: Option<HeaderMap<HeaderValue>>,
// user_agent: Option<UserAgentValue>,
// default_headers: Option<HeaderMap>,
// cookie_store: Option<bool>,
// cookie_provider: Option<CookieProvider>,
// gzip: Option<bool>,
// brotli: Option<bool>,
// deflate: Option<bool>,
// no_gzip: Option<()>,
// no_brotli: Option<()>,
// no_deflate: Option<()>,
// redirect: Option<Policy>,
// referer: Option<bool>,
// proxy: Option<Proxy>,
// no_proxy: Option<()>,
// timeout: Option<Duration>,
// connect_timeout: Option<Duration>,
// connection_verbose: Option<bool>,
// pool_idle_timeout: Option<PoolIdleTimeoutDuration>,
// pool_max_idle_per_host: Option<usize>,
// http1_title_case_headers: Option<()>,
// http1_allow_obsolete_multiline_headers_in_responses: Option<bool>,
// http1_only: Option<()>,
// http09_responses: Option<()>,
// http2_prior_knowledge: Option<()>,
// http2_initial_stream_window_size: Option<impl Into<Option<u32>>>,
// http2_initial_connection_window_size: Option<impl Into<Option<u32>>>,
// http2_adaptive_window: Option<bool>,
// http2_max_frame_size: Option<impl Into<Option<u32>>>,
// http2_keep_alive_interval: impl Into<Option<Duration>>,
// http2_keep_alive_timeout: Duration,
// http2_keep_alive_while_idle:
// should_trace: bool,
