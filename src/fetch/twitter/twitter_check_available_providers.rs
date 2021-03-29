use crate::fetch::twitter::twitter_check_provider_status_aka_rxiv_fetch_link::twitter_check_provider_status_aka_rxiv_fetch_link;
use crate::overriding::prints::print_error_red;
use std::collections::HashMap;

pub fn twitter_check_available_providers(
    enable_prints: bool,
    enable_error_prints: bool,
    twitter_providers_names: Vec<&str>,
) -> Vec<&str> {
    // Vec<String>
    let mut twitter_providers_links: HashMap<&str, bool> =
        HashMap::with_capacity(twitter_providers_names.len());
    for provider_name in twitter_providers_names {
        twitter_providers_links.insert(provider_name, false);
    }
    let crossbeam_result = crossbeam::scope(|scope| {
        for (provider_name, provider_link_checker_flag) in twitter_providers_links.iter_mut() {
            scope.spawn(move |_| {
                let provider_link: String = format!("https://{}/TheCherno/rss", provider_name); //choose random account from following
                let check_status_result = twitter_check_provider_status_aka_rxiv_fetch_link(
                    &provider_link,
                    enable_error_prints,
                );
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
    let mut twitter_providers_links_available: Vec<&str> = Vec::new();
    for (key, value) in twitter_providers_links {
        if value {
            twitter_providers_links_available.push(key)
        }
    }
    twitter_providers_links_available
}
