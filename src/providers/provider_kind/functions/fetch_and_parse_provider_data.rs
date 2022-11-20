use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
use crate::fetch::rss_parse_string_into_struct::rss_parse_string_into_struct;
use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use futures::future::join_all;
use git_info::GitInfoFromTufaCommon;
use std::time::Instant;
use tufa_common::common::where_was::WhereWas;
use tufa_common::server::http_request::http_request_error::HttpRequestOriginError;
use tufa_common::server::http_request::http_request_method::HttpRequestMethod;
use tufa_common::server::http_request::wrappers::text::async_http_request_text::async_http_request_text_wrapper;

#[derive(Debug, GitInfoFromTufaCommon)]
pub enum FetchAndParseProviderDataErrorEnum {
    AsyncFetchLinks {
        source: Vec<(String, Box<HttpRequestOriginError>)>, //link, error
        where_was: WhereWas,
    },
    NoItems {
        source: Vec<(String, NoItemsError)>, //link, error
        where_was: WhereWas,
    },
}

impl ProviderKind {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    pub async fn fetch_and_parse_provider_data(
        self,
        links: Vec<String>,
    ) -> Result<Vec<CommonRssPostStruct>, Box<FetchAndParseProviderDataErrorEnum>> {
        let time = Instant::now();
        let capacity = links.len();
        let vec_to_return = join_all(links.iter().map(|url| async move {
            let result = async_http_request_text_wrapper::<
                String,
                reqwest::cookie::Jar,
                core::time::Duration,
                u32,
                u32,
                u32,
                std::time::Duration,
                std::net::IpAddr,
                std::time::Duration,
                String, //todo - dyn std::any::Any
                String,
                String,
                String,
                String,
                String,
                String,
                String,
                String,
                String,
            >(
                url,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                HttpRequestMethod::Get,
                &CONFIG.source_place_type,
                false,
            )
            .await;
            print_colorful_message(
                None,
                PrintType::TimeMeasurement,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                format!(
                    "fetch_link {url} in {}.{}ms",
                    time.elapsed().as_secs(),
                    time.elapsed().as_millis() / 10,
                ),
            );
            (url, result)
        }))
        .await;
        let mut half_success_vec = Vec::with_capacity(capacity);
        let mut async_fetch_links_error_vec = Vec::new();
        for (link, result) in vec_to_return {
            match result {
                Err(e) => {
                    async_fetch_links_error_vec.push((link.to_string(), e));
                }
                Ok(str) => {
                    half_success_vec.push((link, str));
                }
            }
        }
        if !async_fetch_links_error_vec.is_empty() {
            //todo: maybe not all links must return Ok ?
            return Err(Box::new(
                FetchAndParseProviderDataErrorEnum::AsyncFetchLinks {
                    source: async_fetch_links_error_vec,
                    where_was: WhereWas {
                        time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("cannot convert time to unix_epoch"),
                        file: String::from(file!()),
                        line: line!(),
                        column: column!(),
                    },
                },
            ));
        }
        let mut success_vec = Vec::with_capacity(capacity);
        let mut no_items_error_vec = Vec::new();
        for (link, response_text) in half_success_vec {
            match rss_parse_string_into_struct(response_text, link, self) {
                Err(e) => no_items_error_vec.push((link.to_string(), e)),
                Ok(post_struct) => {
                    success_vec.push(post_struct); //todo maybe add link here?
                }
            }
        }
        if !no_items_error_vec.is_empty() {
            return Err(Box::new(FetchAndParseProviderDataErrorEnum::NoItems {
                source: no_items_error_vec,
                where_was: WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                },
            }));
        }
        Ok(success_vec)
    }
}
