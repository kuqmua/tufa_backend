use std::thread;

use crate::config::ARXIV_LINK;
use crate::config::BIORXIV_LINK;
use crate::config::HABR_LINK;
use crate::config::MEDRXIV_LINK;
use crate::config::REDDIT_LINK;
use crate::config::TWITTER_LINK; //must be not only 1 str but many - twitter and many nitters

use crate::config::ENABLE_ARXIV;
use crate::config::ENABLE_BIORXIV;
use crate::config::ENABLE_HABR;
use crate::config::ENABLE_MEDRXIV;
use crate::config::ENABLE_REDDIT;
use crate::config::ENABLE_TWITTER;

use crate::config::ENABLE_ARXIV_TIME_MEASUREMENT;
use crate::config::ENABLE_BIORXIV_TIME_MEASUREMENT;
use crate::config::ENABLE_HABR_TIME_MEASUREMENT;
use crate::config::ENABLE_MEDRXIV_TIME_MEASUREMENT;
use crate::config::ENABLE_REDDIT_TIME_MEASUREMENT;
use crate::config::ENABLE_TWITTER_TIME_MEASUREMENT;

use crate::config::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV;
use crate::config::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV;
use crate::config::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR;
use crate::config::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV;
use crate::config::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT;
use crate::config::ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER;

use crate::config::ENABLE_ERROR_PRINTS_FOR_ARXIV;
use crate::config::ENABLE_ERROR_PRINTS_FOR_BIORXIV;
use crate::config::ENABLE_ERROR_PRINTS_FOR_HABR;
use crate::config::ENABLE_ERROR_PRINTS_FOR_MEDRXIV;
use crate::config::ENABLE_ERROR_PRINTS_FOR_REDDIT;
use crate::config::ENABLE_ERROR_PRINTS_FOR_TWITTER;

use crate::config::ENABLE_PRINTS_ARXIV;
use crate::config::ENABLE_PRINTS_BIORXIV;
use crate::config::ENABLE_PRINTS_HABR;
use crate::config::ENABLE_PRINTS_MEDRXIV;
use crate::config::ENABLE_PRINTS_REDDIT;
use crate::config::ENABLE_PRINTS_TWITTER;

use crate::config::ENABLE_WARNING_PRINTS_ARXIV;
use crate::config::ENABLE_WARNING_PRINTS_BIORXIV;
use crate::config::ENABLE_WARNING_PRINTS_HABR;
use crate::config::ENABLE_WARNING_PRINTS_MEDRXIV;
use crate::config::ENABLE_WARNING_PRINTS_REDDIT;
use crate::config::ENABLE_WARNING_PRINTS_TWITTER;

use crate::get_information::get_names::get_arxiv_names::get_arxiv_names;
use crate::get_information::get_names::get_biorxiv_names::get_biorxiv_names;
use crate::get_information::get_names::get_habr_names::get_habr_names;
use crate::get_information::get_names::get_medrxiv_names::get_medrxiv_names;
use crate::get_information::get_names::get_reddit_names::get_reddit_names;
use crate::get_information::get_names::get_twitter_names::get_twitter_names;

use crate::fetch::rss_part::rss_part;

use crate::fetch::rss_provider_kind_enum::ProviderKind;

use crate::overriding::prints::print_error_red;

pub async fn check_new_posts_threads_parts() {
    let mut threads_vec = Vec::with_capacity(4);
    if ENABLE_REDDIT {
        let reddit_links = get_reddit_names();
        if reddit_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "arxiv_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Reddit;
            if ENABLE_PRINTS_REDDIT {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    reddit_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT,
                    ENABLE_PRINTS_REDDIT,
                    ENABLE_WARNING_PRINTS_REDDIT,
                    ENABLE_ERROR_PRINTS_FOR_REDDIT,
                    ENABLE_REDDIT_TIME_MEASUREMENT,
                    REDDIT_LINK,
                    &PROVIDER_KIND,
                );
            }))
        };
    }
    if ENABLE_ARXIV {
        let arxiv_links = get_arxiv_names();
        if arxiv_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "arxiv_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Arxiv;
            if ENABLE_PRINTS_ARXIV {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    arxiv_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV,
                    ENABLE_PRINTS_ARXIV,
                    ENABLE_WARNING_PRINTS_ARXIV,
                    ENABLE_ERROR_PRINTS_FOR_ARXIV,
                    ENABLE_ARXIV_TIME_MEASUREMENT,
                    ARXIV_LINK,
                    &PROVIDER_KIND,
                );
            }));
        }
    }
    if ENABLE_BIORXIV {
        let biorxiv_links = get_biorxiv_names();
        if biorxiv_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "biorxiv_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Biorxiv;
            if ENABLE_PRINTS_BIORXIV {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    biorxiv_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV,
                    ENABLE_PRINTS_BIORXIV,
                    ENABLE_WARNING_PRINTS_BIORXIV,
                    ENABLE_ERROR_PRINTS_FOR_BIORXIV,
                    ENABLE_BIORXIV_TIME_MEASUREMENT,
                    BIORXIV_LINK,
                    &PROVIDER_KIND,
                );
            }));
        }
    }
    if ENABLE_MEDRXIV {
        let medrxiv_links = get_medrxiv_names();
        if medrxiv_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "medrxiv_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Medrxiv;
            if ENABLE_PRINTS_MEDRXIV {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    medrxiv_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV,
                    ENABLE_PRINTS_MEDRXIV,
                    ENABLE_WARNING_PRINTS_MEDRXIV,
                    ENABLE_ERROR_PRINTS_FOR_MEDRXIV,
                    ENABLE_MEDRXIV_TIME_MEASUREMENT,
                    MEDRXIV_LINK,
                    &PROVIDER_KIND,
                );
            }));
        }
    }
    if ENABLE_TWITTER {
        let twitter_links = get_twitter_names();
        if twitter_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "twitter_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Twitter;
            if ENABLE_PRINTS_TWITTER {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    twitter_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER,
                    ENABLE_PRINTS_TWITTER,
                    ENABLE_WARNING_PRINTS_TWITTER,
                    ENABLE_ERROR_PRINTS_FOR_TWITTER,
                    ENABLE_TWITTER_TIME_MEASUREMENT,
                    TWITTER_LINK,
                    &PROVIDER_KIND,
                );
            }));
        }
    }
    if ENABLE_HABR {
        let habr_links = get_habr_names();
        if habr_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "habr_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Habr;
            if ENABLE_PRINTS_HABR {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    habr_links.len(),
                    &PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR,
                    ENABLE_PRINTS_HABR,
                    ENABLE_WARNING_PRINTS_HABR,
                    ENABLE_ERROR_PRINTS_FOR_HABR,
                    ENABLE_HABR_TIME_MEASUREMENT,
                    HABR_LINK,
                    &PROVIDER_KIND,
                );
            }));
        }
    }
    for i in threads_vec {
        i.join().unwrap();
    }
}
