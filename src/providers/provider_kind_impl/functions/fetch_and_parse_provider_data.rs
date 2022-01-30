use std::time::Instant;

use futures::future::join_all;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_handle_error_status_code::handle_error_status_code;
use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
use crate::fetch::rss_parse_string_into_struct::rss_parse_string_into_struct;

use crate::helpers::fetch::async_fetch_link::async_fetch_link;
use crate::helpers::fetch::fetch_link_error::FetchLinkError;
use crate::helpers::fetch::fetch_link_error::FetchLinkErrorEnum;
use crate::helpers::lazy_static_git_info::GIT_INFO;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::traits::git_info_trait::GitInfo;

#[derive(Debug, GitInfoDerive)]
pub enum FetchAndParseProviderDataErrorEnum {
    AsyncFetchLinks {
        source: Vec<(String, FetchLinkError)>, //link, error
        line: String,
    },
    NoItems {
        source: Vec<(String, NoItemsError)>, //link, error
        line: String,
    },
}

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub async fn fetch_and_parse_provider_data(
        self,
        links: Vec<String>,
    ) -> Result<Vec<CommonRssPostStruct>, Box<FetchAndParseProviderDataErrorEnum>> {
        let time = Instant::now();
        let capacity = links.len();
        let vec_to_return = join_all(links.iter().map(|link| async move {
            let result = async_fetch_link(&link).await;
            print_colorful_message(
                None,
                PrintType::TimeMeasurement,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "fetch_link {} in {}.{}ms",
                    link,
                    time.elapsed().as_secs(),
                    time.elapsed().as_millis() / 10,
                ),
            );
            (link, result)
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
            for (link, e) in &async_fetch_links_error_vec {
                if let FetchLinkErrorEnum::StatusCode(status_code) = *e.source {
                    handle_error_status_code(status_code, &link);
                }
            }
            return Err(Box::new(
                FetchAndParseProviderDataErrorEnum::AsyncFetchLinks {
                    source: async_fetch_links_error_vec,
                    line: format!("{}:{}:{}", file!(), line!(), column!()),
                },
            ));
        }
        let mut success_vec = Vec::with_capacity(capacity);
        let mut no_items_error_vec = Vec::new();
        for (link, response_text) in half_success_vec {
            match rss_parse_string_into_struct(response_text, &link, self) {
                Err(e) => no_items_error_vec.push((link.to_string(), e)),
                Ok(post_struct) => {
                    success_vec.push(post_struct); //todo maybe add link here?
                }
            }
        }
        if !no_items_error_vec.is_empty() {
            return Err(Box::new(FetchAndParseProviderDataErrorEnum::NoItems {
                source: no_items_error_vec,
                line: format!("{}:{}:{}", file!(), line!(), column!()),
            }));
        }
        Ok(success_vec)
    }
}
