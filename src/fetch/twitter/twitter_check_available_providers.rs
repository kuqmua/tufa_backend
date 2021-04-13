use crate::fetch::twitter::twitter_check_provider_status_aka_rxiv_fetch_link::twitter_check_provider_status_aka_rxiv_fetch_link;
use crate::overriding::prints::print_error_red;

use std::sync::{Arc, Mutex};
use std::thread;

pub fn twitter_check_available_providers(
    enable_prints: bool,
    enable_error_prints: bool,
    twitter_providers_names: std::vec::Vec<&'static str>,
) -> Vec<&str> {
    let mut threads_vector = Vec::with_capacity(twitter_providers_names.len());
    let twitter_providers_links_available = Arc::new(Mutex::new(Vec::new()));
    for provider_name in &mut twitter_providers_names.into_iter() {
        let twitter_providers_links_available_handle =
            Arc::clone(&twitter_providers_links_available);
        let handle = thread::spawn(move || {
            let provider_link: String = format!("https://{}/TheCherno/rss", provider_name); //choose random account from following
            let check_status_result = twitter_check_provider_status_aka_rxiv_fetch_link(
                &provider_link,
                enable_error_prints,
            );
            match check_status_result {
                Ok(fetch_tuple_result) => {
                    if fetch_tuple_result.0 {
                        let mut twitter_providers_links_available_handle_locked =
                            twitter_providers_links_available_handle.lock().unwrap();
                        twitter_providers_links_available_handle_locked.push(provider_name);
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
        threads_vector.push(handle);
    }
    for thread in threads_vector {
        thread.join().unwrap();
    }
    let twitter_providers_links_available_done =
        twitter_providers_links_available.lock().unwrap().to_vec();
    twitter_providers_links_available_done
}
