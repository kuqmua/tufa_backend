use crate::authorization::reddit::authorization_info;
use crate::authorization::reddit::reddit_authorization;
use crate::check_net::check_link::check_link;
use crate::fetch::reddit_fetch_wrapper::get_reddit_posts::get_reddit_posts;
use crate::fetch::rss_check_available_providers::rss_check_available_providers;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::get_group_names::get_subreddits::get_subreddits;
use crate::get_group_names::get_twitter_providers_names::get_twitter_providers_names;

pub fn reddit_part(
    enable_cleaning_logs_directory: bool,
    enable_prints: bool,
    enable_warning_prints: bool,
    enable_error_prints: bool,
    enable_time_measurement: bool,
    provider_link: &str,
    provider_kind: &'static ProviderKind,
) {
    let mut availability_checker_flag: bool = false;
    match provider_kind {
        ProviderKind::Arxiv => {
            if check_link(provider_link).0 {
                availability_checker_flag = true;
            }
        }
        ProviderKind::Biorxiv => {
            if check_link(provider_link).0 {
                availability_checker_flag = true;
            }
        }
        ProviderKind::Medrxiv => {
            if check_link(provider_link).0 {
                availability_checker_flag = true;
            }
        }
        ProviderKind::Twitter => {
            let twitter_providers_names: Vec<&str> = get_twitter_providers_names();
            let twitter_available_providers_links: Vec<&str> = rss_check_available_providers(
                enable_prints,
                enable_error_prints,
                enable_time_measurement,
                twitter_providers_names,
            );
            if !twitter_available_providers_links.is_empty() {
                availability_checker_flag = true;
            }
        }
        ProviderKind::Reddit => {
            if check_link(provider_link).0 {
                availability_checker_flag = false; //todo
            }
        }
    }
    if availability_checker_flag {
        let is_reddit_authorized = reddit_authorization::reddit_authorization(
            authorization_info::REDDIT_USER_AGENT,
            authorization_info::REDDIT_CLIENT_ID,
            authorization_info::REDDIT_CLIENT_SECRET,
            authorization_info::REDDIT_USERNAME,
            authorization_info::REDDIT_PASSWORD,
        );
        if is_reddit_authorized {
            println!("success reddit authorization");
            let subreddits_names = get_subreddits();
            get_reddit_posts(subreddits_names); //возможно неподходящее название////тут есть возвращаемое значение let vec_of_vec_of_strings =
        }
    }
}
