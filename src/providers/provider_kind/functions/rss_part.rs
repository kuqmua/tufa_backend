use crate::config_mods::lazy_static_config::CONFIG;
use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::helpers::get_git_commit_string::get_git_commit_string;
use crate::helpers::where_was::WhereWas;
use crate::providers::provider_kind::functions::fetch_and_parse_provider_data::FetchAndParseProviderDataErrorEnum;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::git_info_trait::GitInfo;
use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;
use chrono::{DateTime, FixedOffset, Local, Utc};
use git_info::GitInfoDerive;
use reqwest::StatusCode;

#[derive(Debug, GitInfoDerive)]
pub enum RssPartErrorEnum {
    CheckLinkStatusCodeError {
        source: reqwest::Error,
        where_was: WhereWas,
    },
    StatusCode {
        source: StatusCode,
        where_was: WhereWas,
    },
    FetchAndParseProviderData {
        source: FetchAndParseProviderDataErrorEnum,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn rss_part(
    pk: ProviderKind,
    vec_of_provider_links: Vec<String>,
) -> Result<Vec<CommonRssPostStruct>, Box<RssPartErrorEnum>> {
    match reqwest::get(pk.check_link()).await {
        Err(e) => Err(Box::new(RssPartErrorEnum::CheckLinkStatusCodeError {
            source: e,
            where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
        })),
        Ok(res) => {
            let status_code = res.status();
            if !StatusCode::is_success(&status_code) {
                return Err(Box::new(RssPartErrorEnum::StatusCode {
                    source: status_code,
                    where_was: WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                }));
            }
            match ProviderKind::fetch_and_parse_provider_data(pk, vec_of_provider_links).await {
                Err(e) => Err(Box::new(RssPartErrorEnum::FetchAndParseProviderData {
                    source: *e,
                    where_was: WhereWas {
                        time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                            .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                })),
                Ok(vec) => Ok(vec),
            }
        }
    }
}
