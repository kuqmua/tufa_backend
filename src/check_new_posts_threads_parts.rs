use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

use std::sync::{Arc, Mutex};

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;
use crate::fetch::rss_part::RssPartError;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::providers_new_posts_check::providers_new_posts_check;

use crate::helpers::resource::ResourceError;
use crate::traits::env_var_bool_trait::EnvVarBoolTrait;
use crate::traits::provider_kind_trait::ProviderKindTrait;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn check_new_posts_threads_parts(
    providers_link_parts: HashMap<ProviderKind, Vec<String>>,
) -> Result<
    Vec<(
        ProviderKind,
        Result<(Vec<CommonRssPostStruct>, Vec<PostErrorVariant>), RssPartError>,
    )>,
    ResourceError,
> {
    let mut threads_vec: Vec<JoinHandle<()>> =
        Vec::with_capacity(ProviderKind::get_enabled_string_name_vec().len());
    let mut threads_vec_checker =
        Vec::<bool>::with_capacity(ProviderKind::get_enabled_string_name_vec().len());
    let posts_and_errors: Vec<(
        ProviderKind,
        Result<(Vec<CommonRssPostStruct>, Vec<PostErrorVariant>), RssPartError>,
    )> = Vec::with_capacity(ProviderKind::get_enabled_providers_vec().len()); //todo: with_capacity
    let posts_and_errors_arc_mutex = Arc::new(Mutex::new(posts_and_errors));
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
                    if provider_kind.is_prints_enabled() {
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
                    let posts_and_errors_handle = Arc::clone(&posts_and_errors_arc_mutex);
                    let vec_of_provider_links =
                        provider_kind.get_provider_links(link_parts.to_vec());
                    threads_vec_checker.push(true);
                    if vec_of_provider_links.is_empty() {
                        print_colorful_message(
                            Some(&provider_kind),
                            PrintType::WarningLow,
                            file!().to_string(),
                            line!().to_string(),
                            format!("vec_of_provider_links.is_empty for {:?}", provider_kind),
                        );
                    } else {
                        threads_vec.push(thread::spawn(move || {
                            providers_new_posts_check(
                                provider_kind,
                                vec_of_provider_links,
                                posts_and_errors_handle,
                            );
                        }));
                    }
                }
            }
            None => {
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("no such provider_name for {:?}", provider_kind),
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
                            "threads_vec_checker.get_mut(index) is None".to_string(),
                        );
                    }
                }
            }
        }
    }
    let is_all_elelements_false = &threads_vec_checker.iter().all(|&item| !item);
    if *is_all_elelements_false {
        Ok(Vec::new())
    } else {
        let posts_and_errors_to_return: Vec<(
            ProviderKind,
            Result<(Vec<CommonRssPostStruct>, Vec<PostErrorVariant>), RssPartError>,
        )>;
        // let error_posts_done: Vec<PostErrorVariant>;
        match posts_and_errors_arc_mutex.lock() {
            Ok(mut ok_posts_and_errors) => {
                posts_and_errors_to_return = ok_posts_and_errors.drain(..).collect();
            }
            Err(e) => {
                posts_and_errors_to_return = Vec::new(); //todo - why we need this?
                print_colorful_message(
                    None,
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("posts.lock() error: {:#?}", e),
                );
            }
        }
        if posts_and_errors_to_return.is_empty() {
            Ok(Vec::new()) //todo - it must be not an option
        } else {
            Ok(posts_and_errors_to_return)
        }
    }
}
