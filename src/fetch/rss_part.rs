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

#[derive(Debug)]
pub enum RssPartError {
    CheckLinkStatusCode(CheckLinkStatusCodeError),
    StatusCode(StatusCode),
}
// impl From<reqwest::Error> for RssPartError {
//     fn from(e: reqwest::Error) -> Self {
//         RssPartError::ReqwestError(e)
//     }
// }
// impl Clone for RssPartError {
//     fn clone(&self) -> RssPartError {
//         match self {
//             Self::ReqwestError(e) => RssPartError::ReqwestError(e.clone()), //reqwest::Error::new(*e)//i dont think its correct implementation
//             Self::StatusCode(status_code) => RssPartError::StatusCode(*status_code),
//         }
//     }
// }

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn rss_part(
    provider_kind: ProviderKind,
    vec_of_provider_links: Vec<String>,
) -> (ProviderKind, Result<SuccessErrorTuple, RssPartError>) {
    let check_link_result = check_link_status_code(provider_kind.check_link());
    match check_link_result {
        Ok(status_code) => {
            if !StatusCode::is_success(&status_code) {
                return (provider_kind, Err(RssPartError::StatusCode(status_code)));
            }
            (
                provider_kind,
                Ok(rss_filter_fetched_and_parsed_posts(
                    rss_fetch_and_parse_provider_data(vec_of_provider_links, provider_kind),
                    provider_kind,
                )),
            )
        }
        Err(e) => (provider_kind, Err(RssPartError::CheckLinkStatusCode(e))),
    }
}
