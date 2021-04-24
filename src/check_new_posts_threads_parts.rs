use std::thread;

use crate::config::ARXIV_LINK;
use crate::config::BIORXIV_LINK;
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

use crate::config::ENABLE_ARXIV_TIME_MEASUREMENT;
use crate::config::ENABLE_BIORXIV_TIME_MEASUREMENT;
use crate::config::ENABLE_MEDRXIV_TIME_MEASUREMENT;
// use crate::config::ENABLE_REDDIT_TIME_MEASUREMENT;
use crate::config::ENABLE_TWITTER_TIME_MEASUREMENT;

use crate::config::MEDRXIV_LINK;
use crate::config::TWITTER_LINK; //must be not only 1 str but many - twitter and many nitters
use crate::fetch::provider_kind_enum::ProviderKind;
// use crate::fetch::rxiv::twitter_part::twitter_part;
use crate::fetch::twitter::twitter_part::twitter_part;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use crate::get_group_names::get_biorxiv_links::get_biorxiv_links;
use crate::get_group_names::get_medrxiv_links::get_medrxiv_links;
// use crate::get_group_names::get_twitter_links::get_twitter_links;
use crate::overriding::prints::print_error_red;
// use crate::config::ENABLE_REDDIT;

pub async fn check_new_posts_threads_parts() {
    let mut threads_vec = Vec::with_capacity(4);
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
            if ENABLE_PRINTS_ARXIV {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    arxiv_links.len(),
                    ProviderKind::Arxiv
                );
            };
            threads_vec.push(thread::spawn(move || {
                twitter_part(
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ARXIV,
                    ENABLE_PRINTS_ARXIV,
                    ENABLE_WARNING_PRINTS_ARXIV,
                    ENABLE_ERROR_PRINTS_ARXIV,
                    ENABLE_ARXIV_TIME_MEASUREMENT,
                    ARXIV_LINK,
                    &ProviderKind::Arxiv,
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
            if ENABLE_PRINTS_BIORXIV {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    biorxiv_links.len(),
                    ProviderKind::Biorxiv
                );
            };
            threads_vec.push(thread::spawn(move || {
                twitter_part(
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_BIORXIV,
                    ENABLE_PRINTS_BIORXIV,
                    ENABLE_WARNING_PRINTS_BIORXIV,
                    ENABLE_ERROR_PRINTS_BIORXIV,
                    ENABLE_BIORXIV_TIME_MEASUREMENT,
                    BIORXIV_LINK,
                    &ProviderKind::Biorxiv,
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
                twitter_part(
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_MEDRXIV,
                    ENABLE_PRINTS_MEDRXIV,
                    ENABLE_WARNING_PRINTS_MEDRXIV,
                    ENABLE_ERROR_PRINTS_MEDRXIV,
                    ENABLE_MEDRXIV_TIME_MEASUREMENT,
                    MEDRXIV_LINK,
                    &ProviderKind::Medrxiv,
                );
            }));
        }
    }
    if ENABLE_TWITTER {
        threads_vec.push(thread::spawn(move || {
            twitter_part(
                ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_TWITTER,
                ENABLE_PRINTS_TWITTER,
                ENABLE_WARNING_PRINTS_TWITTER,
                ENABLE_ERROR_PRINTS_TWITTER,
                ENABLE_TWITTER_TIME_MEASUREMENT,
                TWITTER_LINK,
                &ProviderKind::Twitter,
            );
        }));
    }
    for i in threads_vec {
        i.join().unwrap();
    }
}
