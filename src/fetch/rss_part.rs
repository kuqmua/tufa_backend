use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::authorization::reddit::authorization_info;
use crate::authorization::reddit::reddit_authorization;

use crate::check_net::check_link::check_link;

use crate::fetch::rss_check_available_providers::rss_check_available_providers;
use crate::fetch::rss_divide_to_equal_for_each_provider::rss_divide_to_equal_for_each_provider;
use crate::fetch::rss_fetch_and_parse_provider_data::rss_fetch_and_parse_provider_data;
use crate::fetch::rss_handle_unfiltered_posts::handle_unfiltered_posts;
use crate::fetch::rss_provider_kind_enum::ProviderKind;

use crate::get_project_information::generate_hashmap_links::generate_arxiv_hashmap_links::generate_arxiv_hashmap_links;
use crate::get_project_information::generate_hashmap_links::generate_biorxiv_hashmap_links::generate_biorxiv_hashmap_links;
use crate::get_project_information::generate_hashmap_links::generate_habr_hashmap_links::generate_habr_hashmap_links;
use crate::get_project_information::generate_hashmap_links::generate_medrxiv_hashmap_links::generate_medrxiv_hashmap_links;
use crate::get_project_information::generate_hashmap_links::generate_reddit_hashmap_links::generate_reddit_hashmap_links;
use crate::get_project_information::generate_hashmap_links::generate_twitter_hashmap_links::generate_twitter_hashmap_links;

use crate::get_project_information::get_names::get_arxiv_names::get_arxiv_names;
use crate::get_project_information::get_names::get_biorxiv_names::get_biorxiv_names;
use crate::get_project_information::get_names::get_habr_names::get_habr_names;
use crate::get_project_information::get_names::get_medrxiv_names::get_medrxiv_names;
use crate::get_project_information::get_names::get_reddit_names::get_reddit_names;
use crate::get_project_information::get_names::get_twitter_names::get_twitter_names;

use crate::get_project_information::get_twitter_providers_names::get_twitter_providers_names;

use crate::overriding::prints::print_error_red;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

#[allow(clippy::clippy::too_many_arguments)]
pub fn rss_part(
    enable_cleaning_logs_directory: bool,
    enable_prints: bool,
    enable_warning_prints: bool,
    enable_error_prints: bool,
    enable_time_measurement: bool,
    provider_link: &str,
    provider_kind: &'static ProviderKind,
    enable_error_prints_handle: bool,
    warning_logs_directory_name: String,
) -> bool {
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
        ProviderKind::Medrxiv => {
            if check_link(provider_link, enable_error_prints_handle).0 {
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
        if enable_prints {
            println!("i can reach {}", provider_link)
        };
        let links_temp_naming: HashMap<&str, String>;
        let twitter_available_providers_links: Vec<&str>;
        match provider_kind {
            ProviderKind::Arxiv => {
                twitter_available_providers_links = Vec::new();
            }
            ProviderKind::Biorxiv => {
                twitter_available_providers_links = Vec::new();
            }
            ProviderKind::Medrxiv => {
                twitter_available_providers_links = Vec::new();
            }
            ProviderKind::Twitter => {
                let twitter_providers_names: Vec<&str> = get_twitter_providers_names();
                // let twitter_available_providers_links: Vec<String> =
                twitter_available_providers_links = rss_check_available_providers(
                    enable_prints,
                    enable_error_prints,
                    enable_time_measurement,
                    twitter_providers_names,
                );
            }
            ProviderKind::Reddit => {
                twitter_available_providers_links = Vec::new(); //todo
            }
            ProviderKind::Habr => {
                twitter_available_providers_links = Vec::new();
            }
        }
        match provider_kind {
            ProviderKind::Arxiv => {
                links_temp_naming = generate_arxiv_hashmap_links(get_arxiv_names());
            }
            ProviderKind::Biorxiv => {
                links_temp_naming = generate_biorxiv_hashmap_links(get_biorxiv_names());
            }
            ProviderKind::Medrxiv => {
                links_temp_naming = generate_medrxiv_hashmap_links(get_medrxiv_names());
            }
            ProviderKind::Twitter => {
                links_temp_naming = generate_twitter_hashmap_links(
                    twitter_available_providers_links.clone(),
                    get_twitter_names(),
                );
            }
            ProviderKind::Reddit => {
                links_temp_naming = generate_reddit_hashmap_links(get_reddit_names());
            }
            ProviderKind::Habr => {
                links_temp_naming = generate_habr_hashmap_links(get_habr_names());
            }
        }
        if !links_temp_naming.is_empty() {
            let links_len = links_temp_naming.len();
            let unfiltered_posts_hashmap_after_fetch_and_parse: Vec<(
                String,
                (
                    CommonRssPostStruct,
                    String,
                    UnhandledFetchStatusInfo,
                    HandledFetchStatusInfo,
                    AreThereItems,
                ),
            )>;
            match provider_kind {
                ProviderKind::Arxiv => {
                    unfiltered_posts_hashmap_after_fetch_and_parse =
                        rss_fetch_and_parse_provider_data(
                            enable_prints,
                            enable_error_prints,
                            enable_time_measurement,
                            links_temp_naming,
                            provider_kind,
                        );
                }
                ProviderKind::Biorxiv => {
                    unfiltered_posts_hashmap_after_fetch_and_parse =
                        rss_fetch_and_parse_provider_data(
                            enable_prints,
                            enable_error_prints,
                            enable_time_measurement,
                            links_temp_naming,
                            provider_kind,
                        );
                }
                ProviderKind::Medrxiv => {
                    unfiltered_posts_hashmap_after_fetch_and_parse =
                        rss_fetch_and_parse_provider_data(
                            enable_prints,
                            enable_error_prints,
                            enable_time_measurement,
                            links_temp_naming,
                            provider_kind,
                        );
                }
                ProviderKind::Twitter => {
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
                        let thread = thread::spawn(move || {
                            let unfiltered_posts_hashmap_after_fetch_and_parse =
                                rss_fetch_and_parse_provider_data(
                                    enable_prints,
                                    enable_error_prints,
                                    enable_time_measurement,
                                    element.clone(),
                                    &provider_kind,
                                );
                            let mut locked_not_ready_processed_posts =
                                not_ready_processed_posts_handle.lock().unwrap();
                            for (key, value) in unfiltered_posts_hashmap_after_fetch_and_parse {
                                locked_not_ready_processed_posts.push((key, value));
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
                ProviderKind::Reddit => {
                    //what should i do with authorization?
                    let is_reddit_authorized = reddit_authorization::reddit_authorization(
                        authorization_info::REDDIT_USER_AGENT,
                        authorization_info::REDDIT_CLIENT_ID,
                        authorization_info::REDDIT_CLIENT_SECRET,
                        authorization_info::REDDIT_USERNAME,
                        authorization_info::REDDIT_PASSWORD,
                    );
                    if is_reddit_authorized {
                        if enable_prints {
                            println!("success reddit authorization");
                        }
                        unfiltered_posts_hashmap_after_fetch_and_parse =
                            rss_fetch_and_parse_provider_data(
                                enable_prints,
                                enable_error_prints,
                                enable_time_measurement,
                                links_temp_naming,
                                provider_kind,
                            );
                    } else {
                        unfiltered_posts_hashmap_after_fetch_and_parse = Vec::new();
                        if enable_error_prints {
                            print_error_red(
                                file!().to_string(),
                                line!().to_string(),
                                "cannot authorize reddit(cannot put here authorization_info for future security reasons".to_string(),
                            )
                        }
                    }
                }
                ProviderKind::Habr => {
                    unfiltered_posts_hashmap_after_fetch_and_parse =
                        rss_fetch_and_parse_provider_data(
                            enable_prints,
                            enable_error_prints,
                            enable_time_measurement,
                            links_temp_naming,
                            provider_kind,
                        );
                }
            }
            if !unfiltered_posts_hashmap_after_fetch_and_parse.is_empty() {
                handle_unfiltered_posts(
                    unfiltered_posts_hashmap_after_fetch_and_parse,
                    provider_kind,
                    enable_prints,
                    enable_warning_prints,
                    enable_error_prints,
                    enable_cleaning_logs_directory,
                    enable_time_measurement,
                    warning_logs_directory_name,
                );
                true
            } else {
                if enable_error_prints {
                    let error_message = format!(
                        "unfiltered_posts_hashmap_after_fetch_and_parse is empty for{:#?}",
                        provider_kind
                    );
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                }
                false
            }
        } else {
            if enable_error_prints {
                let error_message = format!("links_temp_naming is empty for{:#?}", provider_kind);
                print_error_red(file!().to_string(), line!().to_string(), error_message)
            }
            false
        }
    } else {
        if enable_error_prints {
            match provider_kind {
                ProviderKind::Arxiv => {
                    let error_message =
                        format!("i cannot reach {} for {:#?}", provider_link, provider_kind);
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                }
                ProviderKind::Biorxiv => {
                    let error_message =
                        format!("i cannot reach {} for {:#?}", provider_link, provider_kind);
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                }
                ProviderKind::Medrxiv => {
                    let error_message =
                        format!("i cannot reach {} for {:#?}", provider_link, provider_kind);
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                }
                ProviderKind::Twitter => {
                    let error_message = format!(
                        "i cannot reach any of provider links for {:#?}",
                        provider_kind
                    );
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                }
                ProviderKind::Reddit => {
                    //todo
                    let error_message =
                        format!("i cannot reach {} for {:#?}", provider_link, provider_kind);
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                }
                ProviderKind::Habr => {
                    //todo
                    let error_message =
                        format!("i cannot reach {} for {:#?}", provider_link, provider_kind);
                    print_error_red(file!().to_string(), line!().to_string(), error_message);
                }
            }
        };
        false
    }
}
