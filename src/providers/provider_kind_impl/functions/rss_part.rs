use reqwest::StatusCode;

use crate::check_net::check_link_status_code::check_link_status_code;
use crate::check_net::check_link_status_code::CheckLinkStatusCodeError;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind_impl::functions::fetch_and_parse_provider_data::FetchAndParseProviderDataErrorEnum;

use crate::helpers::get_git_commit_string::get_git_commit_string;

use crate::traits::git_info_trait::GitInfo;
use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;

#[derive(Debug, GitInfoDerive)]
pub enum RssPartErrorEnum {
    CheckLinkStatusCodeError {
        source: CheckLinkStatusCodeError,
        file: &'static str,
        line: u32,
        column: u32,
    },
    StatusCode {
        source: StatusCode,
        file: &'static str,
        line: u32,
        column: u32,
    },
    FetchAndParseProviderData {
        source: FetchAndParseProviderDataErrorEnum,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn rss_part(
    pk: ProviderKind,
    vec_of_provider_links: Vec<String>,
) -> Result<Vec<CommonRssPostStruct>, Box<RssPartErrorEnum>> {
    match check_link_status_code(pk.check_link()).await {
        Err(e) => Err(Box::new(RssPartErrorEnum::CheckLinkStatusCodeError {
            source: e,
            file: file!(),
            line: line!(),
            column: column!(),
        })),
        Ok(status_code) => {
            if !StatusCode::is_success(&status_code) {
                return Err(Box::new(RssPartErrorEnum::StatusCode {
                    source: status_code,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                }));
            }
            match ProviderKind::fetch_and_parse_provider_data(pk, vec_of_provider_links).await {
                Err(e) => Err(Box::new(RssPartErrorEnum::FetchAndParseProviderData {
                    source: *e,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                })),
                Ok(vec) => Ok(vec),
            }
        }
    }
}
