use reqwest::StatusCode;

use crate::check_net::check_link_status_code::check_link_status_code;
use crate::check_net::check_link_status_code::CheckLinkStatusCodeError;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_filter_fetched_and_parsed_posts::rss_filter_fetched_and_parsed_posts;
use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;
use crate::providers::provider_kind_impl::functions::fetch_and_parse_provider_data::fetch_and_parse_provider_data;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::helpers::lazy_static_git_info::GIT_INFO;
use crate::traits::git_info_trait::GitInfo;
use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;

//todo: think about naming
type SuccessErrorTuple = (Vec<CommonRssPostStruct>, Vec<PostErrorVariant>);

#[derive(Debug, GitInfoDerive)]
pub enum RssPartErrorEnum {
    CheckLinkStatusCodeError {
        source: CheckLinkStatusCodeError,
        line: String,
    },
    StatusCode {
        source: StatusCode,
        line: String,
    },
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn rss_part(
    pk: ProviderKind,
    vec_of_provider_links: Vec<String>,
) -> Result<SuccessErrorTuple, Box<RssPartErrorEnum>> {
    match check_link_status_code(pk.check_link()).await {
        Err(e) => Err(Box::new(RssPartErrorEnum::CheckLinkStatusCodeError {
            source: e,
            line: format!("{}:{}:{}", file!(), line!(), column!()),
        })),
        Ok(status_code) => {
            if !StatusCode::is_success(&status_code) {
                return Err(Box::new(RssPartErrorEnum::StatusCode {
                    source: status_code,
                    line: format!("{}:{}:{}", file!(), line!(), column!()),
                }));
            }
            Ok(rss_filter_fetched_and_parsed_posts(
                fetch_and_parse_provider_data(vec_of_provider_links, pk).await,
                pk,
            ))
        }
    }
}
