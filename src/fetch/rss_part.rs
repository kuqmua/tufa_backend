use reqwest::StatusCode;

use crate::authorization::reddit::reddit_authorization;

use crate::check_net::check_link_status_code::check_link_status_code;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_fetch_and_parse_provider_data::rss_fetch_and_parse_provider_data;
use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;
use crate::fetch::rss_handle_unfiltered_posts::rss_handle_unfiltered_posts;
use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::config::CONFIG;

//todo: think about naming
type SuccessErrorTuple = (
    Option<Vec<CommonRssPostStruct>>,
    Option<Vec<PostErrorVariant>>,
);

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
    if StatusCode::is_success(&status_code) {
        return Err(RssPartError::StatusCode(status_code));
    }
    let links_temp_naming: Vec<String> = vec_of_provider_links;
    if !links_temp_naming.is_empty() {
        let unfiltered_posts_vec_after_fetch_and_parse: Vec<
            Result<
                Result<CommonRssPostStruct, (NoItemsError, String)>,
                (String, ProviderKind, String),
            >,
        >;
        match provider_kind {
            ProviderKind::Arxiv => {
                unfiltered_posts_vec_after_fetch_and_parse =
                    rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
            }
            ProviderKind::Biorxiv => {
                unfiltered_posts_vec_after_fetch_and_parse =
                    rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
            }
            ProviderKind::Github => {
                unfiltered_posts_vec_after_fetch_and_parse =
                    rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
            }
            ProviderKind::Medrxiv => {
                unfiltered_posts_vec_after_fetch_and_parse =
                    rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
            }
            ProviderKind::Twitter => {
                unfiltered_posts_vec_after_fetch_and_parse =
                    rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
            }
            ProviderKind::Reddit => {
                //what should i do with authorization?
                let is_reddit_authorized = reddit_authorization::reddit_authorization(
                    &CONFIG.reddit_authorization.reddit_user_agent,
                    &CONFIG.reddit_authorization.reddit_client_id,
                    &CONFIG.reddit_authorization.reddit_client_secret,
                    &CONFIG.reddit_authorization.reddit_username,
                    &CONFIG.reddit_authorization.reddit_password,
                );
                if is_reddit_authorized {
                    print_colorful_message(
                        Some(&provider_kind),
                        PrintType::Success,
                        file!().to_string(),
                        line!().to_string(),
                        "success reddit authorization".to_string(),
                    );
                    unfiltered_posts_vec_after_fetch_and_parse =
                        rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
                } else {
                    unfiltered_posts_vec_after_fetch_and_parse = Vec::new(); //rethink this
                    print_colorful_message(
                                Some(&provider_kind),
                                PrintType::Error,
                                file!().to_string(),
                                line!().to_string(),
                                "cannot authorize reddit(cannot put here authorization_info for future security reasons".to_string(),
                            );
                }
            }
            ProviderKind::Habr => {
                unfiltered_posts_vec_after_fetch_and_parse =
                    rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
            }
        }
        if !unfiltered_posts_vec_after_fetch_and_parse.is_empty() {
            Ok(rss_handle_unfiltered_posts(
                unfiltered_posts_vec_after_fetch_and_parse,
                provider_kind,
            ))
        } else {
            print_colorful_message(
                Some(&provider_kind),
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "unfiltered_posts_vec_after_fetch_and_parse is empty for{:#?}",
                    provider_kind
                ),
            );
            Ok((None, None))
        }
    } else {
        print_colorful_message(
            Some(&provider_kind),
            PrintType::Error,
            file!().to_string(),
            line!().to_string(),
            format!("links_temp_naming is empty for{:#?}", provider_kind),
        );
        Ok((None, None))
    }
}
