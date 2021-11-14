use reqwest::StatusCode;

use crate::authorization::reddit::reddit_authorization;

use crate::check_net::fetch_link::fetch_link;

use crate::fetch::rss_fetch_and_parse_provider_data::rss_fetch_and_parse_provider_data;
use crate::fetch::rss_handle_unfiltered_posts::rss_handle_unfiltered_posts;
use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

use crate::config_mods::config::CONFIG;

//todo: think about naming
type SuccessErrorTuple = (
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
);

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn rss_part(
    provider_kind: ProviderKind,
    vec_of_provider_links: Vec<String>,
) -> SuccessErrorTuple {
    let mut availability_checker_flag: bool = false;
    let result = fetch_link(ProviderKind::get_check_link(provider_kind));
    match result {
        Ok(status_code) => {
            if StatusCode::is_success(&status_code) {
                availability_checker_flag = true;
            }
        }
        Err(e) => {
            print_colorful_message(
                Some(&provider_kind),
                PrintType::Error,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "i cannot reach {} for {:#?}, error: {:#?}",
                    ProviderKind::get_check_link(provider_kind),
                    provider_kind,
                    e
                ),
            );
            //todo: early return?
        }
    }
    if !availability_checker_flag {
        return (None, None);
    }
    let links_temp_naming: Vec<String> = vec_of_provider_links;
    if !links_temp_naming.is_empty() {
        // let links_len = links_temp_naming.len();
        let unfiltered_posts_vec_after_fetch_and_parse: Vec<(
            CommonRssPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
        )>;
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
                ////this logic was removed for refactoring reasons. maybe rewrite it but not here
                // let twitter_available_providers_links: Vec<String>;
                // match option_twitter_providers_names {
                //     Some(twitter_providers_names) => {
                //         twitter_available_providers_links =
                //             rss_check_available_providers(twitter_providers_names);
                //         let vec_of_hashmap_parts = rss_divide_to_equal_for_each_provider(
                //             twitter_available_providers_links,
                //             links_temp_naming,
                //             links_len,
                //         );
                //         let not_ready_processed_posts =
                //             Arc::new(Mutex::new(Vec::with_capacity(links_len)));
                //         let mut threads_vector = Vec::with_capacity(vec_of_hashmap_parts.len());
                //         let mut threads_vec_checker =
                //             Vec::<bool>::with_capacity(vec_of_hashmap_parts.len());
                //         for element in &mut vec_of_hashmap_parts.into_iter() {
                //             let not_ready_processed_posts_handle =
                //                 Arc::clone(&not_ready_processed_posts);
                //             let provider_kind_clone = provider_kind;
                //             let thread = thread::spawn(move || {
                //                 let unfiltered_posts_vec_after_fetch_and_parse =
                //                     rss_fetch_and_parse_provider_data(
                //                         element.clone(),
                //                         provider_kind_clone,
                //                     );
                //                 match not_ready_processed_posts_handle.lock() {
                //                     Ok(mut locked_not_ready_processed_posts) => {
                //                         for unfiltered_post in
                //                             unfiltered_posts_vec_after_fetch_and_parse
                //                         {
                //                             locked_not_ready_processed_posts.push(unfiltered_post);
                //                         }
                //                     }
                //                     Err(e) => {
                //                         print_colorful_message(
                //                                 None,
                //                                 PrintType::Error,
                //                                 file!().to_string(),
                //                                 line!().to_string(),
                //                                 format!("not_ready_processed_posts_handle.lock() error: {:#?}", e),
                //                             );
                //                     }
                //                 }
                //             });
                //             threads_vector.push(thread);
                //         }
                //         for thread in threads_vector {
                //             match thread.join() {
                //                 Ok(_) => threads_vec_checker.push(true),
                //                 Err(e) => {
                //                     threads_vec_checker.push(false);
                //                     print_colorful_message(
                //                         None,
                //                         PrintType::Error,
                //                         file!().to_string(),
                //                         line!().to_string(),
                //                         format!("thread.join()  error: {:#?}", e),
                //                     );
                //                 }
                //             }
                //         }
                //         let is_all_elelements_false =
                //             &threads_vec_checker.iter().all(|&item| !item);
                //         if *is_all_elelements_false {
                //             print_colorful_message(
                //                             None,
                //                             PrintType::Error,
                //                             file!().to_string(),
                //                             line!().to_string(),
                //                             "is_all_elelements_false for threads_vec_checker in twitter_available_providers_links".to_string(),
                //                         );
                //             return (None, None);
                //         } else {
                //             match not_ready_processed_posts.lock() {
                //                 Ok(not_ready_processed_posts_locked) => {
                //                     unfiltered_posts_vec_after_fetch_and_parse =
                //                         not_ready_processed_posts_locked.to_vec();
                //                 }
                //                 Err(e) => {
                //                     print_colorful_message(
                //                         None,
                //                         PrintType::Error,
                //                         file!().to_string(),
                //                         line!().to_string(),
                //                         format!(
                //                             "not_ready_processed_posts.lock()  error: {:#?}",
                //                             e
                //                         ),
                //                     );
                //                     return (None, None);
                //                 }
                //             }
                //         }
                //     }
                //     None => {
                //         unfiltered_posts_vec_after_fetch_and_parse = Vec::new();
                //         print_colorful_message(
                //             Some(&provider_kind),
                //             PrintType::WarningHigh,
                //             file!().to_string(),
                //             line!().to_string(),
                //             "option_twitter_providers_names is None for Twitter".to_string(),
                //         );
                //     }
                // }
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
            rss_handle_unfiltered_posts(unfiltered_posts_vec_after_fetch_and_parse, provider_kind)
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
}
