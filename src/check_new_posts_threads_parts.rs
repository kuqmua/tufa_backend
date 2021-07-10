use std::thread;

use providers_info_lib::get_project_information::get_names::get_arxiv_names::get_arxiv_names;
use providers_info_lib::get_project_information::get_names::get_biorxiv_names::get_biorxiv_names;
use providers_info_lib::get_project_information::get_names::get_github_names::get_github_names;
use providers_info_lib::get_project_information::get_names::get_habr_names::get_habr_names;
use providers_info_lib::get_project_information::get_names::get_medrxiv_names::get_medrxiv_names;
use providers_info_lib::get_project_information::get_names::get_reddit_names::get_reddit_names;
use providers_info_lib::get_project_information::get_names::get_twitter_names::get_twitter_names;

use crate::fetch::rss_part::rss_part;

use crate::fetch::rss_provider_kind_enum::ProviderKind;

use prints_lib::print_colorful_message;
use prints_lib::PrintType;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use std::sync::{Arc, Mutex};

use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

use providers_info_lib::get_project_information::get_twitter_providers_names::get_twitter_providers_names;

use config_lib::get_project_information::get_config::get_config_information::CONFIG;

pub async fn check_new_posts_threads_parts() -> (
    Vec<CommonRssPostStruct>,
    Vec<(
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    )>,
) {
    let mut threads_vec = Vec::with_capacity(6);
    let posts = Arc::new(Mutex::new(Vec::<CommonRssPostStruct>::new()));
    let error_posts = Arc::new(Mutex::new(Vec::<(
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    )>::new()));
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_arxiv {
        let arxiv_links = get_arxiv_names();
        if arxiv_links.is_empty() {
            print_colorful_message(
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                "arxiv_links.is_empty".to_string(),
            );
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Arxiv;
            if CONFIG.params.enable_all_providers_prints && CONFIG.enable_prints.enable_prints_arxiv
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    arxiv_links.len(),
                    PROVIDER_KIND
                );
            };
            let posts_handle = Arc::clone(&posts);
            let error_posts_handle = Arc::clone(&error_posts);
            threads_vec.push(thread::spawn(move || {
                let enum_success_unsuccess_option_posts = rss_part(
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_arxiv,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG.enable_warning_prints.enable_warning_prints_for_arxiv,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_arxiv,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG.enable_time_measurement.enable_arxiv_time_measurement,
                    &CONFIG.links.arxiv_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_handle,
                );
                if let Some(success_posts) = enum_success_unsuccess_option_posts.0 {
                    let mut posts_handle_locked = posts_handle.lock().unwrap();
                    for value in success_posts {
                        posts_handle_locked.push(value);
                    }
                }
                if let Some(unsuccess_posts) = enum_success_unsuccess_option_posts.1 {
                    let mut error_posts_handle_locked = error_posts_handle.lock().unwrap();
                    for unsuccess_post in unsuccess_posts {
                        error_posts_handle_locked.push(unsuccess_post);
                    }
                }
            }));
        }
    }
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_biorxiv {
        let biorxiv_links = get_biorxiv_names();
        if biorxiv_links.is_empty() {
            print_colorful_message(
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                "biorxiv_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Biorxiv;
            if CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_biorxiv
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    biorxiv_links.len(),
                    PROVIDER_KIND
                );
            };
            let posts_handle = Arc::clone(&posts);
            let error_posts_handle = Arc::clone(&error_posts);
            threads_vec.push(thread::spawn(move || {
                let enum_success_unsuccess_option_posts = rss_part(
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_biorxiv,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG
                            .enable_warning_prints
                            .enable_warning_prints_for_biorxiv,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_biorxiv,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_time_measurement
                            .enable_biorxiv_time_measurement,
                    &CONFIG.links.biorxiv_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_handle,
                );
                if let Some(success_posts) = enum_success_unsuccess_option_posts.0 {
                    let mut posts_handle_locked = posts_handle.lock().unwrap();
                    for value in success_posts {
                        posts_handle_locked.push(value);
                    }
                }
                if let Some(unsuccess_posts) = enum_success_unsuccess_option_posts.1 {
                    let mut error_posts_handle_locked = error_posts_handle.lock().unwrap();
                    for unsuccess_post in unsuccess_posts {
                        error_posts_handle_locked.push(unsuccess_post);
                    }
                }
            }));
        }
    }
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_github {
        let github_links = get_github_names();
        if github_links.is_empty() {
            print_colorful_message(
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                "github_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Github;
            if CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_github
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    github_links.len(),
                    PROVIDER_KIND
                );
            };
            let posts_handle = Arc::clone(&posts);
            let error_posts_handle = Arc::clone(&error_posts);
            threads_vec.push(thread::spawn(move || {
                let enum_success_unsuccess_option_posts = rss_part(
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_github,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG
                            .enable_warning_prints
                            .enable_warning_prints_for_github,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_github,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_time_measurement
                            .enable_github_time_measurement,
                    &CONFIG.links.github_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_handle,
                );
                if let Some(success_posts) = enum_success_unsuccess_option_posts.0 {
                    let mut posts_handle_locked = posts_handle.lock().unwrap();
                    for value in success_posts {
                        posts_handle_locked.push(value);
                    }
                }
                if let Some(unsuccess_posts) = enum_success_unsuccess_option_posts.1 {
                    let mut error_posts_handle_locked = error_posts_handle.lock().unwrap();
                    for unsuccess_post in unsuccess_posts {
                        error_posts_handle_locked.push(unsuccess_post);
                    }
                }
            }));
        }
    }
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_habr {
        let habr_links = get_habr_names();
        if habr_links.is_empty() {
            print_colorful_message(
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                "habr_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Habr;
            if CONFIG.params.enable_all_providers_prints && CONFIG.enable_prints.enable_prints_habr
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    habr_links.len(),
                    &PROVIDER_KIND
                );
            };
            let posts_handle = Arc::clone(&posts);
            let error_posts_handle = Arc::clone(&error_posts);
            threads_vec.push(thread::spawn(move || {
                let enum_success_unsuccess_option_posts = rss_part(
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_habr,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG.enable_warning_prints.enable_warning_prints_for_habr,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_habr,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG.enable_time_measurement.enable_habr_time_measurement,
                    &CONFIG.links.habr_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_handle,
                );
                if let Some(success_posts) = enum_success_unsuccess_option_posts.0 {
                    let mut posts_handle_locked = posts_handle.lock().unwrap();
                    for value in success_posts {
                        posts_handle_locked.push(value);
                    }
                }
                if let Some(unsuccess_posts) = enum_success_unsuccess_option_posts.1 {
                    let mut error_posts_handle_locked = error_posts_handle.lock().unwrap();
                    for unsuccess_post in unsuccess_posts {
                        error_posts_handle_locked.push(unsuccess_post);
                    }
                }
            }));
        }
    }
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_medrxiv {
        let medrxiv_links = get_medrxiv_names();
        if medrxiv_links.is_empty() {
            print_colorful_message(
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                "medrxiv_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Medrxiv;
            if CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_medrxiv
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    medrxiv_links.len(),
                    PROVIDER_KIND
                );
            };
            let posts_handle = Arc::clone(&posts);
            let error_posts_handle = Arc::clone(&error_posts);
            threads_vec.push(thread::spawn(move || {
                let enum_success_unsuccess_option_posts = rss_part(
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_medrxiv,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG
                            .enable_warning_prints
                            .enable_warning_prints_for_medrxiv,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_medrxiv,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_time_measurement
                            .enable_medrxiv_time_measurement,
                    &CONFIG.links.medrxiv_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_handle,
                );
                if let Some(success_posts) = enum_success_unsuccess_option_posts.0 {
                    let mut posts_handle_locked = posts_handle.lock().unwrap();
                    for value in success_posts {
                        posts_handle_locked.push(value);
                    }
                }
                if let Some(unsuccess_posts) = enum_success_unsuccess_option_posts.1 {
                    let mut error_posts_handle_locked = error_posts_handle.lock().unwrap();
                    for unsuccess_post in unsuccess_posts {
                        error_posts_handle_locked.push(unsuccess_post);
                    }
                }
            }));
        }
    }
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_reddit {
        let reddit_links = get_reddit_names();
        if reddit_links.is_empty() {
            print_colorful_message(
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                "reddit_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Reddit;
            if CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_reddit
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    reddit_links.len(),
                    PROVIDER_KIND
                );
            };
            let posts_handle = Arc::clone(&posts);
            let error_posts_handle = Arc::clone(&error_posts);
            threads_vec.push(thread::spawn(move || {
                let enum_success_unsuccess_option_posts = rss_part(
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_reddit,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG
                            .enable_warning_prints
                            .enable_warning_prints_for_reddit,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_reddit,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_time_measurement
                            .enable_reddit_time_measurement,
                    &CONFIG.links.reddit_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_reddit,
                );
                if let Some(success_posts) = enum_success_unsuccess_option_posts.0 {
                    let mut posts_handle_locked = posts_handle.lock().unwrap();
                    for value in success_posts {
                        posts_handle_locked.push(value);
                    }
                }
                if let Some(unsuccess_posts) = enum_success_unsuccess_option_posts.1 {
                    let mut error_posts_handle_locked = error_posts_handle.lock().unwrap();
                    for unsuccess_post in unsuccess_posts {
                        error_posts_handle_locked.push(unsuccess_post);
                    }
                }
            }))
        };
    }
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_twitter {
        let twitter_links = get_twitter_names();
        let twitter_providers = get_twitter_providers_names();
        if twitter_links.is_empty() || twitter_providers.is_empty() {
            print_colorful_message(
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                "twiter_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Twitter;
            if CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_twitter
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    twitter_links.len(),
                    PROVIDER_KIND
                );
            };
            let posts_handle = Arc::clone(&posts);
            let error_posts_handle = Arc::clone(&error_posts);
            threads_vec.push(thread::spawn(move || {
                let enum_success_unsuccess_option_posts = rss_part(
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_twitter,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG
                            .enable_warning_prints
                            .enable_warning_prints_for_twitter,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_twitter,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_time_measurement
                            .enable_twitter_time_measurement,
                    &CONFIG.links.twitter_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_handle,
                );
                if let Some(success_posts) = enum_success_unsuccess_option_posts.0 {
                    let mut posts_handle_locked = posts_handle.lock().unwrap();
                    for value in success_posts {
                        posts_handle_locked.push(value);
                    }
                }
                if let Some(unsuccess_posts) = enum_success_unsuccess_option_posts.1 {
                    let mut error_posts_handle_locked = error_posts_handle.lock().unwrap();
                    for unsuccess_post in unsuccess_posts {
                        error_posts_handle_locked.push(unsuccess_post);
                    }
                }
            }));
        }
    }
    for i in threads_vec {
        i.join().unwrap();
    }
    let posts_done = posts.lock().unwrap().to_vec();
    let error_posts_done = error_posts.lock().unwrap().to_vec();
    (posts_done, error_posts_done)
}
