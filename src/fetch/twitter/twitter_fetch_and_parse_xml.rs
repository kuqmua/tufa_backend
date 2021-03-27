use ansi_term::Colour::Red;
use std::collections::HashMap;
use std::time::Instant;

use crate::fetch::provider_kind_enum::ProviderKind;
use crate::fetch::rxiv::metainfo_fetch_structures::AreThereItems;
use crate::fetch::rxiv::metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rxiv::metainfo_fetch_structures::UnhandledFetchStatusInfo;
use crate::fetch::rxiv::rxiv_fetch_link::rxiv_fetch_link;
use crate::fetch::twitter::twitter_check_handled_fetch_status_info::twitter_check_handled_fetch_status_info;
use crate::fetch::twitter::twitter_structures::TwitterPostStruct;
use crate::overriding::prints::print_error_red;

pub fn twitter_fetch_and_parse_xml(
    enable_prints: bool,
    enable_error_prints: bool,
    twitter_links: HashMap<&str, String>,
    provider_kind: ProviderKind,
) -> HashMap<
    String,
    (
        TwitterPostStruct,
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    ),
> {
    let time = Instant::now();
    let mut hashmap_to_return: HashMap<
        String,
        (
            TwitterPostStruct,
            String,
            UnhandledFetchStatusInfo,
            HandledFetchStatusInfo,
            AreThereItems,
            ProviderKind,
        ),
    > = HashMap::with_capacity(twitter_links.len());
    for (key, value) in twitter_links {
        let tuple = (
            TwitterPostStruct::new(),
            value.to_string(),
            UnhandledFetchStatusInfo::Initialized,
            HandledFetchStatusInfo::Initialized,
            AreThereItems::Initialized,
            provider_kind.clone(),
        );
        hashmap_to_return.insert(key.to_string(), tuple);
    }
    if enable_prints {
        println!(
            "hashmap init in {}.{}ms",
            time.elapsed().as_secs(),
            time.elapsed().as_millis(),
        );
    };
    let crossbeam_result = crossbeam::scope(|scope| {
        for (key, value) in &mut hashmap_to_return {
            scope.spawn(move |_| {
                let fetch_result =
                    rxiv_fetch_link(&value.1, key, time, enable_prints, enable_error_prints);
                match fetch_result {
                    Ok(fetch_tuple_result) => {
                        value.2 = UnhandledFetchStatusInfo::Success;
                        let (
                            value3,
                            rxiv_post_struct_wrapper_handle,
                            are_there_items_wrapper_handle,
                        ) = twitter_check_handled_fetch_status_info(
                            fetch_tuple_result.1,
                            fetch_tuple_result.0,
                            time,
                            key,
                            &value.1,
                            enable_prints,
                            enable_error_prints,
                            value.5.clone(),
                        );
                        value.3 = value3;
                        value.0 = rxiv_post_struct_wrapper_handle;
                        value.4 = are_there_items_wrapper_handle;
                    }
                    Err(e) => {
                        value.2 = UnhandledFetchStatusInfo::Failure(e.to_string()); // add e
                        if enable_error_prints {
                            let concated_error =
                                "UnhandledFetchStatusInfo::Failure".to_string() + &e.to_string();
                            print_error_red(
                                file!().to_string(),
                                line!().to_string(),
                                concated_error,
                            )
                        }
                    }
                }
            });
        }
    });
    match crossbeam_result {
        Ok(_) => {
            if enable_prints {
                println!("twitter_fetch_and_parse_xml crossbeam_result is ok")
            }
        }
        Err(e) => {
            if enable_prints {
                let error_message = format!(
                    "twitter_fetch_and_parse_xml crossbeam_result is not ok: {:#?}",
                    e
                );
                print_error_red(file!().to_string(), line!().to_string(), error_message);
            }
        }
    }
    hashmap_to_return
}
