use std::thread;

use crate::config::ARXIV_URL;
use crate::config::BIORXIV_URL;
use crate::config::ENABLE_ARXIV;
use crate::config::ENABLE_BIORXIV;
use crate::config::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ARXIV;
use crate::config::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_BIORXIV;
use crate::config::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_MEDRXIV;
use crate::config::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_TWITTER;
use crate::config::ENABLE_ERROR_PRINTS_ARXIV;
use crate::config::ENABLE_ERROR_PRINTS_BIORXIV;
use crate::config::ENABLE_ERROR_PRINTS_MEDRXIV;
use crate::config::ENABLE_ERROR_PRINTS_TWITTER;
use crate::config::ENABLE_MEDRXIV;
use crate::config::ENABLE_PRINTS_ARXIV;
use crate::config::ENABLE_PRINTS_BIORXIV;
use crate::config::ENABLE_PRINTS_MEDRXIV;
use crate::config::ENABLE_PRINTS_TWITTER;
use crate::config::ENABLE_TWITTER;
use crate::config::ENABLE_WARNING_PRINTS_ARXIV;
use crate::config::ENABLE_WARNING_PRINTS_BIORXIV;
use crate::config::ENABLE_WARNING_PRINTS_MEDRXIV;
use crate::config::ENABLE_WARNING_PRINTS_TWITTER;
use crate::config::MEDRXIV_URL;
use crate::config::TWITTER_URL; //must be not only 1 str but many - twitter and many nitters
use crate::fetch::provider_kind_enum::ProviderKind;
use crate::fetch::rxiv::rxiv_part::rxiv_part;
use crate::fetch::twitter::twitter_part::twitter_part;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use crate::get_group_names::get_biorxiv_links::get_biorxiv_links;
use crate::get_group_names::get_medrxiv_links::get_medrxiv_links;
use crate::get_group_names::get_twitter_links::get_twitter_links;
use crate::overriding::prints::print_error_red;
// use crate::config::ENABLE_REDDIT;

pub async fn check_new_posts_threads_parts() {
    let mut threads_vec = vec![];
    // if ENABLE_REDDIT {
    //     threads_vec.push(thread::spawn(move || {
    //         fetch::reddit_fetch::reddit_fetch::reddit_part();
    //     }));
    // }
    if ENABLE_ARXIV {
        let arxiv_links = get_arxiv_links();
        if arxiv_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "arxiv_links.is_empty".to_string(),
            )
        } else {
            if ENABLE_PRINTS_MEDRXIV {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    arxiv_links.len(),
                    ProviderKind::Arxiv
                );
            };
            threads_vec.push(thread::spawn(move || {
                rxiv_part(
                    get_arxiv_links(),
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ARXIV,
                    ENABLE_PRINTS_ARXIV,
                    ENABLE_WARNING_PRINTS_ARXIV,
                    ENABLE_ERROR_PRINTS_ARXIV,
                    ARXIV_URL,
                    ProviderKind::Arxiv,
                );
            }));
        }
    }
    if ENABLE_BIORXIV {
        let biorxiv_links = get_biorxiv_links();
        if biorxiv_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "biorxiv_links.is_empty".to_string(),
            )
        } else {
            if ENABLE_PRINTS_MEDRXIV {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    biorxiv_links.len(),
                    ProviderKind::Biorxiv
                );
            };
            threads_vec.push(thread::spawn(move || {
                rxiv_part(
                    biorxiv_links,
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_BIORXIV,
                    ENABLE_PRINTS_BIORXIV,
                    ENABLE_WARNING_PRINTS_BIORXIV,
                    ENABLE_ERROR_PRINTS_BIORXIV,
                    BIORXIV_URL,
                    ProviderKind::Biorxiv,
                );
            }));
        }
    }
    if ENABLE_MEDRXIV {
        let medrxiv_links = get_medrxiv_links();
        if medrxiv_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "medrxiv_links.is_empty".to_string(),
            )
        } else {
            if ENABLE_PRINTS_MEDRXIV {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    medrxiv_links.len(),
                    ProviderKind::Medrxiv
                );
            };
            threads_vec.push(thread::spawn(move || {
                rxiv_part(
                    get_medrxiv_links(),
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_MEDRXIV,
                    ENABLE_PRINTS_MEDRXIV,
                    ENABLE_WARNING_PRINTS_MEDRXIV,
                    ENABLE_ERROR_PRINTS_MEDRXIV,
                    MEDRXIV_URL,
                    ProviderKind::Medrxiv,
                );
            }));
        }
    }
    if ENABLE_TWITTER {
        let twitter_links = get_medrxiv_links();
        if twitter_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "twitter_links.is_empty".to_string(),
            )
        } else {
            if ENABLE_PRINTS_TWITTER {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    twitter_links.len(),
                    ProviderKind::Twitter
                );
            };
            threads_vec.push(thread::spawn(move || {
                twitter_part(
                    get_twitter_links(),
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_TWITTER,
                    ENABLE_PRINTS_TWITTER,
                    ENABLE_WARNING_PRINTS_TWITTER,
                    ENABLE_ERROR_PRINTS_TWITTER,
                    TWITTER_URL,
                    ProviderKind::Twitter,
                );
            }));
        }
    }
    for i in threads_vec {
        i.join().unwrap();
    }
}
