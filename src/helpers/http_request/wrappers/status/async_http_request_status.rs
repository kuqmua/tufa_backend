use crate::helpers::http_request::async_http_request_client_request_builder_prep::async_http_request_client_request_builder_prep;
use crate::helpers::http_request::http_request_method::HttpRequestMethod;
use crate::helpers::http_request::request_builder_methods::status::async_http_request_status::async_http_request_status;
use crate::helpers::http_request::wrappers::status::http_request_status_error::HttpRequestWrapperStatusError;
use crate::helpers::http_request::wrappers::status::http_request_status_error::HttpRequestWrapperStatusErrorEnum;
use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[allow(clippy::too_many_arguments)]
pub async fn async_http_request_status_wrapper<
    //client generics
    UserAgentValueGeneric,
    CookieProviderGeneric: reqwest::cookie::CookieStore + 'static,
    PoolIdleTimeoutDurationGeneric,
    LocalAddressGeneric,
    TcpKeepaliveGeneric,
    //request builder generics
    HeaderKeyGeneric,
    HeaderValueGeneric,
    BasicAuthUsernameGeneric,
    BasicAuthPasswordGeneric,
    BearerAuthGeneric,
    BodyGeneric: Into<reqwest::Body>,
    QueryGeneric: serde::Serialize,
    FormGeneric: serde::Serialize,
    JsonGeneric: serde::Serialize,
>(
    url: &str,
    //client parameters
    user_agent_client_argument: Option<UserAgentValueGeneric>,
    default_headers_client_argument: Option<reqwest::header::HeaderMap>,
    cookie_store_client_argument: Option<bool>,
    cookie_provider_client_argument: Option<std::sync::Arc<CookieProviderGeneric>>,
    gzip_client_argument: Option<bool>,
    brotli_client_argument: Option<bool>,
    deflate_client_argument: Option<bool>,
    no_gzip_client_argument: Option<()>,
    no_brotli_client_argument: Option<()>,
    no_deflate_client_argument: Option<()>,
    redirect_client_argument: Option<reqwest::redirect::Policy>,
    referer_client_argument: Option<bool>,
    proxy_client_argument: Option<reqwest::Proxy>,
    no_proxy_client_argument: Option<()>,
    timeout_client_argument: Option<std::time::Duration>,
    connect_timeout_client_argument: Option<std::time::Duration>,
    connection_verbose_client_argument: Option<bool>,
    pool_idle_timeout_client_argument: Option<PoolIdleTimeoutDurationGeneric>,
    pool_max_idle_per_host_client_argument: Option<usize>,
    http1_title_case_headers_client_argument: Option<()>,
    http1_allow_obsolete_multiline_headers_in_responses_client_argument: Option<bool>,
    http1_only_client_argument: Option<()>,
    http09_responses_client_argument: Option<()>,
    http2_prior_knowledge_client_argument: Option<()>,
    http2_initial_stream_window_size_client_argument: Option<impl Into<Option<u32>>>,
    http2_initial_connection_window_size_client_argument: Option<impl Into<Option<u32>>>,
    http2_adaptive_window_client_argument: Option<bool>,
    http2_max_frame_size_client_argument: Option<impl Into<Option<u32>>>,
    http2_keep_alive_interval_client_argument: Option<impl Into<Option<std::time::Duration>>>,
    http2_keep_alive_timeout_client_argument: Option<std::time::Duration>,
    http2_keep_alive_while_idle_client_argument: Option<bool>,
    tcp_nodelay_client_argument: Option<bool>,
    local_address_client_argument: Option<LocalAddressGeneric>,
    tcp_keepalive_client_argument: Option<TcpKeepaliveGeneric>,
    add_root_certificate_client_argument: Option<reqwest::Certificate>,
    tls_built_in_root_certs_client_argument: Option<bool>,
    identity_client_argument: Option<reqwest::Identity>,
    danger_accept_invalid_hostnames_client_argument: Option<bool>,
    danger_accept_invalid_certs_client_argument: Option<bool>,
    min_tls_version_client_argument: Option<reqwest::tls::Version>,
    max_tls_version_client_argument: Option<reqwest::tls::Version>,
    use_native_tls_client_argument: Option<()>,
    use_rustls_tls_client_argument: Option<()>,
    use_preconfigured_tls_client_argument: Option<impl std::any::Any>,
    trust_dns_client_argument: Option<bool>,
    no_trust_dns_client_argument: Option<()>,
    https_only_client_argument: Option<bool>,
    resolve_client_argument: Option<(&str, std::net::SocketAddr)>,
    resolve_to_addrs_client_argument: Option<(&str, &[std::net::SocketAddr])>,
    //request builder parameters
    header_request_builder: Option<(HeaderKeyGeneric, HeaderValueGeneric)>,
    headers_request_builder: Option<reqwest::header::HeaderMap<reqwest::header::HeaderValue>>,
    basic_auth_request_builder: Option<(
        BasicAuthUsernameGeneric,
        Option<BasicAuthPasswordGeneric>,
    )>,
    bearer_auth_request_builder: Option<BearerAuthGeneric>,
    body_request_builder: Option<BodyGeneric>,
    timeout_request_builder: Option<std::time::Duration>,
    multipart_request_builder: Option<reqwest::multipart::Form>,
    query_request_builder: Option<QueryGeneric>,
    version_request_builder: Option<reqwest::Version>,
    form_request_builder: Option<FormGeneric>,
    json_request_builder: Option<JsonGeneric>,
    fetch_mode_no_cors_request_builder: Option<()>,
    //
    method: HttpRequestMethod,
    should_trace: bool,
) -> Result<reqwest::StatusCode, Box<HttpRequestWrapperStatusError>>
where
    UserAgentValueGeneric: TryInto<reqwest::header::HeaderValue>,
    UserAgentValueGeneric::Error: Into<http::Error>,
    PoolIdleTimeoutDurationGeneric: Into<Option<std::time::Duration>>,
    LocalAddressGeneric: Into<Option<std::net::IpAddr>>,
    TcpKeepaliveGeneric: Into<Option<std::time::Duration>>,

    reqwest::header::HeaderName: TryFrom<HeaderKeyGeneric>,
    <reqwest::header::HeaderName as TryFrom<HeaderKeyGeneric>>::Error: Into<http::Error>,
    reqwest::header::HeaderValue: TryFrom<HeaderValueGeneric>,
    <reqwest::header::HeaderValue as TryFrom<HeaderValueGeneric>>::Error: Into<http::Error>,
    BasicAuthUsernameGeneric: std::fmt::Display,
    BasicAuthPasswordGeneric: std::fmt::Display,
    BearerAuthGeneric: std::fmt::Display,
{
    match async_http_request_client_request_builder_prep(
        url,
        //client parameters
        user_agent_client_argument,
        default_headers_client_argument,
        cookie_store_client_argument,
        cookie_provider_client_argument,
        gzip_client_argument,
        brotli_client_argument,
        deflate_client_argument,
        no_gzip_client_argument,
        no_brotli_client_argument,
        no_deflate_client_argument,
        redirect_client_argument,
        referer_client_argument,
        proxy_client_argument,
        no_proxy_client_argument,
        timeout_client_argument,
        connect_timeout_client_argument,
        connection_verbose_client_argument,
        pool_idle_timeout_client_argument,
        pool_max_idle_per_host_client_argument,
        http1_title_case_headers_client_argument,
        http1_allow_obsolete_multiline_headers_in_responses_client_argument,
        http1_only_client_argument,
        http09_responses_client_argument,
        http2_prior_knowledge_client_argument,
        http2_initial_stream_window_size_client_argument,
        http2_initial_connection_window_size_client_argument,
        http2_adaptive_window_client_argument,
        http2_max_frame_size_client_argument,
        http2_keep_alive_interval_client_argument,
        http2_keep_alive_timeout_client_argument,
        http2_keep_alive_while_idle_client_argument,
        tcp_nodelay_client_argument,
        local_address_client_argument,
        tcp_keepalive_client_argument,
        add_root_certificate_client_argument,
        tls_built_in_root_certs_client_argument,
        identity_client_argument,
        danger_accept_invalid_hostnames_client_argument,
        danger_accept_invalid_certs_client_argument,
        min_tls_version_client_argument,
        max_tls_version_client_argument,
        use_native_tls_client_argument,
        use_rustls_tls_client_argument,
        use_preconfigured_tls_client_argument,
        trust_dns_client_argument,
        no_trust_dns_client_argument,
        https_only_client_argument,
        resolve_client_argument,
        resolve_to_addrs_client_argument,
        //request builder parameters
        header_request_builder,
        headers_request_builder,
        basic_auth_request_builder,
        bearer_auth_request_builder,
        body_request_builder,
        timeout_request_builder,
        multipart_request_builder,
        query_request_builder,
        version_request_builder,
        form_request_builder,
        json_request_builder,
        fetch_mode_no_cors_request_builder,
        //
        method,
        false,
    )
    .await
    {
        Err(e) => Err(Box::new(
            HttpRequestWrapperStatusError::init_error_with_possible_trace(
                HttpRequestWrapperStatusErrorEnum::Prep(*e),
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ),
        )),
        Ok(request_builder) => match async_http_request_status(request_builder, false).await {
            Err(e) => Err(Box::new(
                HttpRequestWrapperStatusError::init_error_with_possible_trace(
                    HttpRequestWrapperStatusErrorEnum::Status(*e),
                    WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        location: *core::panic::Location::caller(),
                    },
                    &CONFIG.source_place_type,
                    &GIT_INFO.data,
                    should_trace,
                ),
            )),
            Ok(status) => Ok(status),
        },
    }
}
