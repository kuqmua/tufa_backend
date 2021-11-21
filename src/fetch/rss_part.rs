use reqwest::StatusCode;

use crate::check_net::check_link_status_code::check_link_status_code;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_fetch_and_parse_provider_data::rss_fetch_and_parse_provider_data;
use crate::fetch::rss_filter_fetched_and_parsed_posts::rss_filter_fetched_and_parsed_posts;
use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;

use crate::providers::provider_kind_enum::ProviderKind;

//todo: think about naming
type SuccessErrorTuple = (Vec<CommonRssPostStruct>, Vec<PostErrorVariant>);

#[derive(Debug)]
pub enum RssPartError {
    ReqwestError(reqwest::Error),
    StatusCode(StatusCode),
}
impl From<reqwest::Error> for RssPartError {
    fn from(e: reqwest::Error) -> Self {
        RssPartError::ReqwestError(e)
    }
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn rss_part(
    provider_kind: ProviderKind,
    vec_of_provider_links: Vec<String>,
) -> Result<SuccessErrorTuple, RssPartError> {
    let status_code = check_link_status_code(ProviderKind::get_check_link(provider_kind))?;
    if !StatusCode::is_success(&status_code) {
        return Err(RssPartError::StatusCode(status_code));
    }
    Ok(rss_filter_fetched_and_parsed_posts(
        rss_fetch_and_parse_provider_data(vec_of_provider_links, provider_kind),
        provider_kind,
    ))
}
