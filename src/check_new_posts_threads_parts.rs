use std::thread;
use std::thread::JoinHandle;

use std::sync::{Arc, Mutex};

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;

use crate::providers::get_providers_link_parts_wrapper::get_providers_link_parts_wrapper;
use crate::providers::provider_kind_enum::ProviderKind;

use crate::providers_new_posts_check::providers_new_posts_check;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn check_new_posts_threads_parts() -> Option<(
    Vec<CommonRssPostStruct>,
    Vec<(String, NoItemsError, ProviderKind)>,
)> {
    if !ProviderKind::get_enabled_providers_vec().is_empty() {
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
                    let error_posts =
                        Arc::new(Mutex::new(
                            Vec::<(String, NoItemsError, ProviderKind)>::new(),
                        ));
                    //check if provider_names are unique
                    for provider_kind in ProviderKind::get_enabled_providers_vec() {
                        match providers_link_parts.get(&provider_kind) {
                            Some(link_parts) => {
                                if link_parts.is_empty() {
                                    print_colorful_message(
                                        Some(&provider_kind),
                                        PrintType::Error,
                                        file!().to_string(),
                                        line!().to_string(),
                                        "link_parts.is_empty".to_string(),
                                    );
                                } else {
                                    if ProviderKind::is_prints_enabled(provider_kind) {
                                        print_colorful_message(
                                            Some(&provider_kind),
                                            PrintType::Info,
                                            file!().to_string(),
                                            line!().to_string(),
                                            format!(
                                                "{:#?} elements in {:#?} HashMap",
                                                link_parts.len(),
                                                provider_kind
                                            ),
                                        );
                                    };
                                    let posts_handle = Arc::clone(&posts);
                                    let error_posts_handle = Arc::clone(&error_posts);
                                    let vec_of_provider_links = ProviderKind::get_provider_links(
                                        provider_kind,
                                        link_parts.to_vec(),
                                    );
                                    threads_vec_checker.push(true);
                                    threads_vec.push(thread::spawn(move || {
                                        providers_new_posts_check(
                                            provider_kind,
                                            vec_of_provider_links,
                                            posts_handle,
                                            error_posts_handle,
                                        );
                                    }));
                                }
                            }
                            None => {
                                print_colorful_message(
                                    Some(&provider_kind),
                                    PrintType::Error,
                                    file!().to_string(),
                                    line!().to_string(),
                                    format!(
                                        "no such provider_name - {} for {:#?}",
                                        ProviderKind::get_string_name(provider_kind),
                                        provider_kind
                                    ),
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
                        let error_posts_done: Vec<(String, NoItemsError, ProviderKind)>;
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
                    PrintType::WarningHigh,
                    file!().to_string(),
                    line!().to_string(),
                    "there arent provider link parts ".to_string(),
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
