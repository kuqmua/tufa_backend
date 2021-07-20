use std::sync::{Arc, Mutex};
use std::thread;

use crate::authorization::reddit::reddit_authorization;

use crate::check_net::check_link::check_link;

use crate::fetch::rss_check_available_providers::rss_check_available_providers;
use crate::fetch::rss_divide_to_equal_for_each_provider::rss_divide_to_equal_for_each_provider;
use crate::fetch::rss_fetch_and_parse_provider_data::rss_fetch_and_parse_provider_data;
use crate::fetch::rss_handle_unfiltered_posts::rss_handle_unfiltered_posts;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use prints_lib::print_colorful_message::print_colorful_message;
use prints_lib::print_type_enum::PrintType;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

use config_lib::get_project_information::get_user_credentials::get_user_credentials_information::USER_CREDENTIALS;

#[allow(clippy::clippy::too_many_arguments)]
pub fn rss_part(
    provider_link: &str,
    provider_kind: ProviderKind,
    enable_error_prints_handle: bool,
    vec_of_provider_links: Vec<String>,
    option_twitter_providers_names: Option<Vec<String>>,
) -> (
    Option<Vec<CommonRssPostStruct>>,
    Option<
        Vec<(
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
            ProviderKind,
        )>,
    >,
) {
    let mut availability_checker_flag: bool = false;
    match provider_kind {
        ProviderKind::Arxiv => {
            if check_link(provider_link, enable_error_prints_handle).0 {
                availability_checker_flag = true;
            }
        }
        ProviderKind::Biorxiv => {
            if check_link(provider_link, enable_error_prints_handle).0 {
                availability_checker_flag = true;
            }
        }
        ProviderKind::Github => {
            if check_link(provider_link, enable_error_prints_handle).0 {
                availability_checker_flag = true;
            }
        }
        ProviderKind::Medrxiv => {
            if check_link(provider_link, enable_error_prints_handle).0 {
                availability_checker_flag = true;
            }
        }
        ProviderKind::Twitter => match option_twitter_providers_names.clone() {
            Some(twitter_providers_names) => {
                let twitter_available_providers_links: Vec<String> =
                    rss_check_available_providers(twitter_providers_names);
                if !twitter_available_providers_links.is_empty() {
                    availability_checker_flag = true;
                }
            }
            None => {
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::WarningHigh,
                    file!().to_string(),
                    line!().to_string(),
                    "option_twitter_providers_names is None for Twitter".to_string(),
                );
            }
        },
        ProviderKind::Reddit => {
            if check_link(provider_link, enable_error_prints_handle).0 {
                availability_checker_flag = true; //todo
            }
        }
        ProviderKind::Habr => {
            if check_link(provider_link, enable_error_prints_handle).0 {
                availability_checker_flag = true;
            }
        }
    }
    if availability_checker_flag {
        print_colorful_message(
            Some(&provider_kind),
            PrintType::Success,
            file!().to_string(),
            line!().to_string(),
            format!("i can reach {}", provider_link),
        );
        let links_temp_naming: Vec<String> = vec_of_provider_links;
        let provider_kind_handle = provider_kind.clone();
        if !links_temp_naming.is_empty() {
            let links_len = links_temp_naming.len();
            let unfiltered_posts_hashmap_after_fetch_and_parse: Vec<(
                CommonRssPostStruct,
                String,
                UnhandledFetchStatusInfo,
                HandledFetchStatusInfo,
                AreThereItems,
            )>;
            let provider_kind_clone_for_prints = provider_kind.clone();
            match provider_kind {
                ProviderKind::Arxiv => {
                    unfiltered_posts_hashmap_after_fetch_and_parse =
                        rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
                }
                ProviderKind::Biorxiv => {
                    unfiltered_posts_hashmap_after_fetch_and_parse =
                        rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
                }
                ProviderKind::Github => {
                    unfiltered_posts_hashmap_after_fetch_and_parse =
                        rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
                }
                ProviderKind::Medrxiv => {
                    unfiltered_posts_hashmap_after_fetch_and_parse =
                        rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
                }
                ProviderKind::Twitter => {
                    let twitter_available_providers_links: Vec<String>;
                    match option_twitter_providers_names {
                        Some(twitter_providers_names) => {
                            twitter_available_providers_links =
                                rss_check_available_providers(twitter_providers_names);
                            let vec_of_hashmap_parts = rss_divide_to_equal_for_each_provider(
                                twitter_available_providers_links,
                                links_temp_naming,
                                links_len,
                            );
                            let not_ready_processed_posts =
                                Arc::new(Mutex::new(Vec::with_capacity(links_len)));
                            let mut threads_vector = Vec::with_capacity(vec_of_hashmap_parts.len());
                            for element in &mut vec_of_hashmap_parts.into_iter() {
                                let not_ready_processed_posts_handle =
                                    Arc::clone(&not_ready_processed_posts);
                                let provider_kind_clone = provider_kind.clone();
                                let thread = thread::spawn(move || {
                                    let unfiltered_posts_hashmap_after_fetch_and_parse =
                                        rss_fetch_and_parse_provider_data(
                                            element.clone(),
                                            provider_kind_clone,
                                        );
                                    let mut locked_not_ready_processed_posts =
                                        not_ready_processed_posts_handle.lock().unwrap();
                                    for unfiltered_post in
                                        unfiltered_posts_hashmap_after_fetch_and_parse
                                    {
                                        locked_not_ready_processed_posts.push(unfiltered_post);
                                    }
                                });
                                threads_vector.push(thread);
                            }
                            for thread in threads_vector {
                                thread.join().unwrap();
                            }
                            let f = &*not_ready_processed_posts.lock().unwrap().to_vec();
                            unfiltered_posts_hashmap_after_fetch_and_parse = f.to_vec();
                        }
                        None => {
                            unfiltered_posts_hashmap_after_fetch_and_parse = Vec::new();
                            print_colorful_message(
                                Some(&provider_kind),
                                PrintType::WarningHigh,
                                file!().to_string(),
                                line!().to_string(),
                                "option_twitter_providers_names is None for Twitter".to_string(),
                            );
                        }
                    }
                }
                ProviderKind::Reddit => {
                    //what should i do with authorization?
                    let is_reddit_authorized = reddit_authorization::reddit_authorization(
                        &USER_CREDENTIALS.reddit_authorization.reddit_user_agent,
                        &USER_CREDENTIALS.reddit_authorization.reddit_client_id,
                        &USER_CREDENTIALS.reddit_authorization.reddit_client_secret,
                        &USER_CREDENTIALS.reddit_authorization.reddit_username,
                        &USER_CREDENTIALS.reddit_authorization.reddit_password,
                    );
                    if is_reddit_authorized {
                        print_colorful_message(
                            Some(&provider_kind),
                            PrintType::Success,
                            file!().to_string(),
                            line!().to_string(),
                            "success reddit authorization".to_string(),
                        );
                        unfiltered_posts_hashmap_after_fetch_and_parse =
                            rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
                    } else {
                        unfiltered_posts_hashmap_after_fetch_and_parse = Vec::new(); //rethink this
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
                    unfiltered_posts_hashmap_after_fetch_and_parse =
                        rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
                }
            }
            if !unfiltered_posts_hashmap_after_fetch_and_parse.is_empty() {
                rss_handle_unfiltered_posts(
                    unfiltered_posts_hashmap_after_fetch_and_parse,
                    provider_kind_handle,
                )
            } else {
                print_colorful_message(
                    Some(&provider_kind_clone_for_prints),
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!(
                        "unfiltered_posts_hashmap_after_fetch_and_parse is empty for{:#?}",
                        provider_kind_handle
                    ),
                );
                (None, None)
            }
        } else {
            print_colorful_message(
                Some(&provider_kind),
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!("links_temp_naming is empty for{:#?}", provider_kind),
            );
            (None, None)
        }
    } else {
        match provider_kind {
            ProviderKind::Arxiv => {
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("i cannot reach {} for {:#?}", provider_link, provider_kind),
                );
            }
            ProviderKind::Biorxiv => {
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("i cannot reach {} for {:#?}", provider_link, provider_kind),
                );
            }
            ProviderKind::Github => {
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("i cannot reach {} for {:#?}", provider_link, provider_kind),
                );
            }
            ProviderKind::Medrxiv => {
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("i cannot reach {} for {:#?}", provider_link, provider_kind),
                );
            }
            ProviderKind::Twitter => {
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!(
                        "i cannot reach any of provider links for {:#?}",
                        provider_kind
                    ),
                );
            }
            ProviderKind::Reddit => {
                //todo
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("i cannot reach {} for {:#?}", provider_link, provider_kind),
                );
            }
            ProviderKind::Habr => {
                //todo
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("i cannot reach {} for {:#?}", provider_link, provider_kind),
                );
            }
        }
        (None, None)
    }
}
