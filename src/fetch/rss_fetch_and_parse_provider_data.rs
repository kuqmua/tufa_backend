use std::time::Instant;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_handle_error_status_code::handle_error_status_code;
use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
use crate::fetch::rss_parse_string_into_struct::rss_parse_string_into_struct;
use crate::helpers::fetch::async_fetch_link::async_fetch_link;
use crate::helpers::fetch::fetch_link_error::{FetchLinkError, FetchLinkErrorEnum};

use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use futures::future::join_all;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn rss_fetch_and_parse_provider_data(
    links: Vec<String>,
    pk: ProviderKind,
) -> Vec<
    Result<
        Result<CommonRssPostStruct, (NoItemsError, String)>,
        (String, ProviderKind, FetchLinkError),
    >,
> {
    let time = Instant::now();
    let vec_to_return = join_all(links.into_iter().map(|link| async move {
        let fetch_result = async_fetch_link(&link).await;
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
        match fetch_result {
            Ok(response_text) => match rss_parse_string_into_struct(response_text, &link, pk) {
                Ok(post_struct) => {
                    return Ok(Ok(post_struct));
                }
                Err(e) => {
                    return Ok(Err((e, link)));
                }
            },
            Err(e) => {
                if let FetchLinkErrorEnum::StatusCode(status_code) = *e.source {
                    handle_error_status_code(status_code, &link);
                }
                print_colorful_message(
                    Some(&pk),
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("link: {} FetchLinkError {:#?}", link, e),
                );
                return Err((link, pk, e));
            }
        }
    }))
    .await;
    vec_to_return
}
