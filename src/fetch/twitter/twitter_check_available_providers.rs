// use crate::check_net::check_link::check_link;
use crate::overriding::prints::print_error_red;
use std::collections::HashMap;

pub fn twitter_check_available_providers(
    enable_prints: bool,
    enable_error_prints: bool,
    twitter_providers_names: Vec<&str>,
) -> Vec<String> {
    let mut twitter_providers_links: HashMap<String, bool> =
        HashMap::with_capacity(twitter_providers_names.len());

    for provider_name in twitter_providers_names {
        let provider_link: String = format!("https://{}/TheCherno/rss", provider_name);
        twitter_providers_links.insert(provider_link, false);
    }

    let crossbeam_result = crossbeam::scope(|scope| {
        for (link, provider_link_checker_flag) in twitter_providers_links.iter_mut() {
            scope.spawn(move |_| {
                let check_status_result = something_to_fetch(link, enable_error_prints);
                //dont know why but check_link fucntion not working and returning 404
                // *provider_link_checker_flag = check_link(&link).0;
                match check_status_result {
                    Ok(fetch_tuple_result) => {
                        if fetch_tuple_result.0 {
                            *provider_link_checker_flag = fetch_tuple_result.0;
                        }
                    }
                    Err(e) => {
                        if enable_error_prints {
                            let error_message =
                                "UnhandledFetchStatusInfo::Failure".to_string() + &e.to_string();
                            print_error_red(file!().to_string(), line!().to_string(), error_message)
                        }
                    }
                }
            });
        }
    });
    match crossbeam_result {
        Ok(_) => {
            if enable_prints {
                println!("twitter_check_available_providers crossbeam_result is ok")
            }
        }
        Err(error) => {
            if enable_error_prints {
                let error_message = format!(
                    "twitter_check_available_providers crossbeam_result is not ok: {:#?}",
                    error
                );
                print_error_red(file!().to_string(), line!().to_string(), error_message);
            }
        }
    }
    let mut twitter_providers_links_available: Vec<String> = Vec::new();
    for (key, value) in twitter_providers_links {
        if value {
            twitter_providers_links_available.push(key)
        }
    }
    //check length
    twitter_providers_links_available
}

use crate::fetch::rxiv::metainfo_fetch_structures::HandledFetchStatusInfo;

pub fn something_to_fetch(
    link: &str,
    enable_error_prints: bool,
) -> Result<(bool, HandledFetchStatusInfo), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(link)?;
    let mut result_tuple: (bool, HandledFetchStatusInfo) =
        (false, HandledFetchStatusInfo::Initialized);
    if res.status() == reqwest::StatusCode::OK {
        result_tuple.0 = true;
    } else {
        result_tuple.1 = HandledFetchStatusInfo::ResStatusError(res.status());
        if enable_error_prints {
            let error_message = format!("{} {}", link, res.status());
            print_error_red(file!().to_string(), line!().to_string(), error_message);
        }
    }
    Ok(result_tuple)
}
