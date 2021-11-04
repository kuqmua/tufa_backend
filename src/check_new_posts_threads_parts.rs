use std::thread;
use std::thread::JoinHandle;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use std::sync::{Arc, Mutex};

use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

use crate::config_mods::config::CONFIG;

use crate::providers::get_providers_link_parts_wrapper::get_providers_link_parts_wrapper;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::get_twitter_providers_names::get_twitter_providers_names;

use crate::providers::providers_info::links::generate_arxiv_links::generate_arxiv_links;
use crate::providers::providers_info::links::generate_biorxiv_links::generate_biorxiv_links;
use crate::providers::providers_info::links::generate_github_links::generate_github_links;
use crate::providers::providers_info::links::generate_habr_links::generate_habr_links;
use crate::providers::providers_info::links::generate_medrxiv_links::generate_medrxiv_links;
use crate::providers::providers_info::links::generate_reddit_links::generate_reddit_links;
use crate::providers::providers_info::links::generate_twitter_links::generate_twitter_links;

use crate::providers_new_posts_check::providers_new_posts_check;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn check_new_posts_threads_parts() -> Option<(
    Vec<CommonRssPostStruct>,
    Vec<(
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    )>,
)> {
    if !ProviderKind::get_enabled_string_name_vec().is_empty() {
        let option_providers_link_parts = get_providers_link_parts_wrapper().await;
        match option_providers_link_parts {
            Some(providers_link_parts) => {
                if !providers_link_parts.is_empty() {
                    let mut threads_vec: Vec<JoinHandle<()>> =
                        Vec::with_capacity(ProviderKind::get_enabled_string_name_vec().len());
                    let mut threads_vec_checker = Vec::<bool>::with_capacity(
                        ProviderKind::get_enabled_string_name_vec().len(),
                    );
                    let posts = Arc::new(Mutex::new(Vec::<CommonRssPostStruct>::new()));
                    let error_posts = Arc::new(Mutex::new(Vec::<(
                        String,
                        UnhandledFetchStatusInfo,
                        HandledFetchStatusInfo,
                        AreThereItems,
                        ProviderKind,
                    )>::new()));
                    let config_provider_string_to_enum_struct_hashmap =
                        ProviderKind::into_string_name_and_kind_hashmap();
                    //check if provider_names are unique
                    for provider_name in &ProviderKind::get_enabled_string_name_vec() {
                        match config_provider_string_to_enum_struct_hashmap.get(provider_name) {
                            Some(provider_kind_handle) => match provider_kind_handle {
                                ProviderKind::Arxiv => {
                                    if ProviderKind::is_enabled(*provider_kind_handle) {
                                        match providers_link_parts.get(provider_name) {
                                            Some(arxiv_link_parts) => {
                                                if arxiv_link_parts.is_empty() {
                                                    print_colorful_message(
                                                        Some(provider_kind_handle),
                                                        PrintType::Error,
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "arxiv_link_parts.is_empty".to_string(),
                                                    );
                                                } else {
                                                    if CONFIG
                                                        .enable_providers_prints
                                                        .enable_prints_arxiv
                                                    {
                                                        println!(
                                                            "{:#?} elements in {:#?} HashMap",
                                                            arxiv_link_parts.len(),
                                                            provider_kind_handle
                                                        );
                                                    };
                                                    let posts_handle = Arc::clone(&posts);
                                                    let error_posts_handle =
                                                        Arc::clone(&error_posts);
                                                    let provider_kind_handle_clone =
                                                        *provider_kind_handle;
                                                    let vec_of_provider_links =
                                                        generate_arxiv_links(
                                                            arxiv_link_parts.to_vec(),
                                                        );
                                                    threads_vec_checker.push(true);
                                                    threads_vec.push(thread::spawn(move || {
                                                        providers_new_posts_check(
                                                            provider_kind_handle_clone,
                                                            vec_of_provider_links,
                                                            None,
                                                            posts_handle,
                                                            error_posts_handle,
                                                        );
                                                    }));
                                                }
                                            }
                                            None => {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    format!(
                                                        "no such provider_name - {} for {:#?}",
                                                        provider_name, provider_kind_handle
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                                ProviderKind::Biorxiv => {
                                    if ProviderKind::is_enabled(*provider_kind_handle) {
                                        match providers_link_parts.get(provider_name) {
                                            Some(biorxiv_link_parts) => {
                                                if biorxiv_link_parts.is_empty() {
                                                    print_colorful_message(
                                                        Some(provider_kind_handle),
                                                        PrintType::Error,
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "biorxiv_link_parts.is_empty".to_string(),
                                                    );
                                                } else {
                                                    if CONFIG
                                                        .enable_providers_prints
                                                        .enable_prints_biorxiv
                                                    {
                                                        println!(
                                                            "{:#?} elements in {:#?} HashMap",
                                                            biorxiv_link_parts.len(),
                                                            provider_kind_handle
                                                        );
                                                    };
                                                    let posts_handle = Arc::clone(&posts);
                                                    let error_posts_handle =
                                                        Arc::clone(&error_posts);
                                                    let provider_kind_handle_clone =
                                                        *provider_kind_handle;
                                                    let vec_of_provider_links =
                                                        generate_biorxiv_links(
                                                            biorxiv_link_parts.to_vec(),
                                                        );
                                                    threads_vec_checker.push(true);
                                                    threads_vec.push(thread::spawn(move || {
                                                        providers_new_posts_check(
                                                            provider_kind_handle_clone,
                                                            vec_of_provider_links,
                                                            None,
                                                            posts_handle,
                                                            error_posts_handle,
                                                        );
                                                    }));
                                                }
                                            }
                                            None => {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    format!(
                                                        "no such provider_name - {} for {:#?}",
                                                        provider_name, provider_kind_handle
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                                ProviderKind::Github => {
                                    if ProviderKind::is_enabled(*provider_kind_handle) {
                                        match providers_link_parts.get(provider_name) {
                                            Some(github_link_parts) => {
                                                if github_link_parts.is_empty() {
                                                    print_colorful_message(
                                                        Some(provider_kind_handle),
                                                        PrintType::Error,
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "github_link_parts.is_empty".to_string(),
                                                    );
                                                } else {
                                                    if CONFIG
                                                        .enable_providers_prints
                                                        .enable_prints_github
                                                    {
                                                        println!(
                                                            "{:#?} elements in {:#?} HashMap",
                                                            github_link_parts.len(),
                                                            provider_kind_handle
                                                        );
                                                    };
                                                    let posts_handle = Arc::clone(&posts);
                                                    let error_posts_handle =
                                                        Arc::clone(&error_posts);
                                                    let provider_kind_handle_clone =
                                                        *provider_kind_handle;
                                                    let vec_of_provider_links =
                                                        generate_github_links(
                                                            github_link_parts.to_vec(),
                                                        );
                                                    threads_vec_checker.push(true);
                                                    threads_vec.push(thread::spawn(move || {
                                                        providers_new_posts_check(
                                                            provider_kind_handle_clone,
                                                            vec_of_provider_links,
                                                            None,
                                                            posts_handle,
                                                            error_posts_handle,
                                                        );
                                                    }));
                                                }
                                            }
                                            None => {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    format!(
                                                        "no such provider_name - {} for {:#?}",
                                                        provider_name, provider_kind_handle
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                                ProviderKind::Habr => {
                                    if ProviderKind::is_enabled(*provider_kind_handle) {
                                        match providers_link_parts.get(provider_name) {
                                            Some(habr_link_parts) => {
                                                if habr_link_parts.is_empty() {
                                                    print_colorful_message(
                                                        Some(provider_kind_handle),
                                                        PrintType::Error,
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "habr_link_parts.is_empty".to_string(),
                                                    );
                                                } else {
                                                    if CONFIG
                                                        .enable_providers_prints
                                                        .enable_prints_habr
                                                    {
                                                        println!(
                                                            "{:#?} elements in {:#?} HashMap",
                                                            habr_link_parts.len(),
                                                            provider_kind_handle
                                                        );
                                                    };
                                                    let posts_handle = Arc::clone(&posts);
                                                    let error_posts_handle =
                                                        Arc::clone(&error_posts);
                                                    let provider_kind_handle_clone =
                                                        *provider_kind_handle;
                                                    let vec_of_provider_links = generate_habr_links(
                                                        habr_link_parts.to_vec(),
                                                    );
                                                    threads_vec_checker.push(true);
                                                    threads_vec.push(thread::spawn(move || {
                                                        providers_new_posts_check(
                                                            provider_kind_handle_clone,
                                                            vec_of_provider_links,
                                                            None,
                                                            posts_handle,
                                                            error_posts_handle,
                                                        );
                                                    }));
                                                }
                                            }
                                            None => {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    format!(
                                                        "no such provider_name - {} for {:#?}",
                                                        provider_name, provider_kind_handle
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                                ProviderKind::Medrxiv => {
                                    if ProviderKind::is_enabled(*provider_kind_handle) {
                                        match providers_link_parts.get(provider_name) {
                                            Some(medrxiv_link_parts) => {
                                                if medrxiv_link_parts.is_empty() {
                                                    print_colorful_message(
                                                        Some(provider_kind_handle),
                                                        PrintType::Error,
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "medrxiv_link_parts.is_empty".to_string(),
                                                    );
                                                } else {
                                                    if CONFIG
                                                        .enable_providers_prints
                                                        .enable_prints_medrxiv
                                                    {
                                                        println!(
                                                            "{:#?} elements in {:#?} HashMap",
                                                            medrxiv_link_parts.len(),
                                                            provider_kind_handle
                                                        );
                                                    };
                                                    let posts_handle = Arc::clone(&posts);
                                                    let error_posts_handle =
                                                        Arc::clone(&error_posts);
                                                    let provider_kind_handle_clone =
                                                        *provider_kind_handle;
                                                    let vec_of_provider_links =
                                                        generate_medrxiv_links(
                                                            medrxiv_link_parts.to_vec(),
                                                        );
                                                    threads_vec_checker.push(true);
                                                    threads_vec.push(thread::spawn(move || {
                                                        providers_new_posts_check(
                                                            provider_kind_handle_clone,
                                                            vec_of_provider_links,
                                                            None,
                                                            posts_handle,
                                                            error_posts_handle,
                                                        );
                                                    }));
                                                }
                                            }
                                            None => {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    format!(
                                                        "no such provider_name - {} for {:#?}",
                                                        provider_name, provider_kind_handle
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                                ProviderKind::Reddit => {
                                    if ProviderKind::is_enabled(*provider_kind_handle) {
                                        match providers_link_parts.get(provider_name) {
                                            Some(reddit_link_parts) => {
                                                if reddit_link_parts.is_empty() {
                                                    print_colorful_message(
                                                        Some(provider_kind_handle),
                                                        PrintType::Error,
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "reddit_link_parts.is_empty".to_string(),
                                                    );
                                                } else {
                                                    if CONFIG
                                                        .enable_providers_prints
                                                        .enable_prints_reddit
                                                    {
                                                        println!(
                                                            "{:#?} elements in {:#?} HashMap",
                                                            reddit_link_parts.len(),
                                                            provider_kind_handle
                                                        );
                                                    };
                                                    let posts_handle = Arc::clone(&posts);
                                                    let error_posts_handle =
                                                        Arc::clone(&error_posts);
                                                    let provider_kind_handle_clone =
                                                        *provider_kind_handle;
                                                    let vec_of_provider_links =
                                                        generate_reddit_links(
                                                            reddit_link_parts.to_vec(),
                                                        );
                                                    threads_vec_checker.push(true);
                                                    threads_vec.push(thread::spawn(move || {
                                                        providers_new_posts_check(
                                                            provider_kind_handle_clone,
                                                            vec_of_provider_links,
                                                            None,
                                                            posts_handle,
                                                            error_posts_handle,
                                                        );
                                                    }));
                                                }
                                            }
                                            None => {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    format!(
                                                        "no such provider_name - {} for {:#?}",
                                                        provider_name, provider_kind_handle
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                                ProviderKind::Twitter => {
                                    if ProviderKind::is_enabled(*provider_kind_handle) {
                                        match providers_link_parts.get(provider_name) {
                                            Some(twitter_link_parts) => {
                                                let twitter_providers =
                                                    get_twitter_providers_names();
                                                if twitter_link_parts.is_empty() {
                                                    print_colorful_message(
                                                        Some(provider_kind_handle),
                                                        PrintType::Error,
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "twitter_link_parts.is_empty".to_string(),
                                                    );
                                                } else if twitter_providers.is_empty() {
                                                    print_colorful_message(
                                                        Some(provider_kind_handle),
                                                        PrintType::Error,
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "twitter_providers.is_empty()".to_string(),
                                                    );
                                                } else {
                                                    let posts_handle = Arc::clone(&posts);
                                                    let error_posts_handle =
                                                        Arc::clone(&error_posts);
                                                    let provider_kind_handle_clone =
                                                        *provider_kind_handle;

                                                    let vec_of_provider_links =
                                                        generate_twitter_links(
                                                            twitter_providers.clone(),
                                                            twitter_link_parts.to_vec(),
                                                        );
                                                    threads_vec_checker.push(true);
                                                    threads_vec.push(thread::spawn(move || {
                                                        providers_new_posts_check(
                                                            provider_kind_handle_clone,
                                                            vec_of_provider_links,
                                                            Some(twitter_providers),
                                                            posts_handle,
                                                            error_posts_handle,
                                                        );
                                                    }));
                                                }
                                            }
                                            None => {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    format!(
                                                        "no such provider_name - {} for {:#?}",
                                                        provider_name, provider_kind_handle
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                            },
                            None => {
                                print_colorful_message(
                                        None,
                                        PrintType::WarningLow,
                                        file!().to_string(),
                                        line!().to_string(),
                                        "config_provider_string_to_enum_struct_hashmap.get(provider_name) is None".to_string(),
                                    );
                            }
                        }
                    }
                    for (index, thread_vec) in threads_vec.into_iter().enumerate() {
                        match thread_vec.join() {
                            Ok(_) => {}
                            Err(e) => {
                                print_colorful_message(
                                    None,
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("thread_vec.join() error: {:#?}", e),
                                );
                                let option_element = threads_vec_checker.get_mut(index);
                                match option_element {
                                    Some(element) => {
                                        *element = false;
                                    }
                                    None => {
                                        print_colorful_message(
                                            None,
                                            PrintType::Error,
                                            file!().to_string(),
                                            line!().to_string(),
                                            "threads_vec_checker.get_mut(index) is None"
                                                .to_string(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    let is_all_elelements_false = &threads_vec_checker.iter().all(|&item| !item);
                    if *is_all_elelements_false {
                        None
                    } else {
                        let posts_done: Vec<CommonRssPostStruct>;
                        let error_posts_done: Vec<(
                            String,
                            UnhandledFetchStatusInfo,
                            HandledFetchStatusInfo,
                            AreThereItems,
                            ProviderKind,
                        )>;
                        match posts.lock() {
                            Ok(ok_posts_lock) => {
                                posts_done = ok_posts_lock.to_vec();
                            }
                            Err(e) => {
                                posts_done = Vec::new();
                                print_colorful_message(
                                    None,
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("posts.lock() error: {:#?}", e),
                                );
                            }
                        }
                        match error_posts.lock() {
                            Ok(ok_error_posts_lock) => {
                                error_posts_done = ok_error_posts_lock.to_vec();
                            }
                            Err(e) => {
                                error_posts_done = Vec::new();
                                print_colorful_message(
                                    None,
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!("error_posts.lock() error: {:#?}", e),
                                );
                            }
                        }
                        if posts_done.is_empty() && error_posts_done.is_empty() {
                            None
                        } else {
                            Some((posts_done, error_posts_done))
                        }
                    }
                } else {
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        "providers_link_parts is empty".to_string(),
                    );
                    None
                }
            }
            None => {
                print_colorful_message(
                    None,
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!(
                        "option_providers_link_parts {:#?}",
                        option_providers_link_parts
                    ),
                );
                None
            }
        }
    } else {
        print_colorful_message(
            None,
            PrintType::WarningLow,
            file!().to_string(),
            line!().to_string(),
            "CONFIG.params.vec_of_provider_names is empty".to_string(),
        );
        None
    }
}
