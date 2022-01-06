use reqwest::StatusCode;

use crate::check_net::check_link_status_code::check_link_status_code;
use crate::check_net::check_link_status_code::CheckLinkStatusCodeError;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_fetch_and_parse_provider_data::rss_fetch_and_parse_provider_data;
use crate::fetch::rss_filter_fetched_and_parsed_posts::rss_filter_fetched_and_parsed_posts;
use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;

//todo: think about naming
type SuccessErrorTuple = (Vec<CommonRssPostStruct>, Vec<PostErrorVariant>);

#[derive(displaydoc::Display, Debug, BoxErrFromErrDerive)]
pub struct RssPartError {
    pub source: Box<RssPartErrorEnum>,
}

#[derive(Debug, ImplFromForUpperStruct)]
pub enum RssPartErrorEnum {
    CheckLinkStatusCodeError(CheckLinkStatusCodeError),
    StatusCode(StatusCode),
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn rss_part(
    provider_kind: ProviderKind,
    vec_of_provider_links: Vec<String>,
) -> Result<SuccessErrorTuple, RssPartError> {
    let status_code = check_link_status_code(provider_kind.check_link()).await?;
    if !StatusCode::is_success(&status_code) {
        return Err(RssPartError {
            source: Box::new(RssPartErrorEnum::StatusCode(status_code)),
        });
    }
    Ok(rss_filter_fetched_and_parsed_posts(
        rss_fetch_and_parse_provider_data(vec_of_provider_links, provider_kind),
        provider_kind,
    ))
}
