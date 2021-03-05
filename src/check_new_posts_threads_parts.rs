use std::thread;

use crate::config::ARXIV_URL;
use crate::config::BIORXIV_URL;
use crate::config::ENABLE_ARXIV;
use crate::config::ENABLE_BIORXIV;
use crate::config::ENABLE_ERROR_PRINTS_ARXIV;
use crate::config::ENABLE_ERROR_PRINTS_BIORXIV;
use crate::config::ENABLE_ERROR_PRINTS_MEDRXIV;
use crate::config::ENABLE_MEDRXIV;
use crate::config::ENABLE_PRINTS_ARXIV;
use crate::config::ENABLE_PRINTS_BIORXIV;
use crate::config::ENABLE_PRINTS_MEDRXIV;
use crate::config::ENABLE_WARNING_PRINTS_ARXIV;
use crate::config::ENABLE_WARNING_PRINTS_BIORXIV;
use crate::config::ENABLE_WARNING_PRINTS_MEDRXIV;
use crate::config::MEDRXIV_URL;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::fetch::rxiv_part::rxiv_part;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use crate::get_group_names::get_biorxiv_links::get_biorxiv_links;
use crate::get_group_names::get_medrxiv_links::get_medrxiv_links;
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
                    RxivKind::Arxiv
                );
            };
            threads_vec.push(thread::spawn(move || {
                rxiv_part(
                    get_arxiv_links(),
                    ENABLE_PRINTS_ARXIV,
                    ENABLE_WARNING_PRINTS_ARXIV,
                    ENABLE_ERROR_PRINTS_ARXIV,
                    ARXIV_URL,
                    RxivKind::Arxiv,
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
                    RxivKind::Biorxiv
                );
            };
            threads_vec.push(thread::spawn(move || {
                rxiv_part(
                    biorxiv_links,
                    ENABLE_PRINTS_BIORXIV,
                    ENABLE_WARNING_PRINTS_BIORXIV,
                    ENABLE_ERROR_PRINTS_BIORXIV,
                    BIORXIV_URL,
                    RxivKind::Biorxiv,
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
                    RxivKind::Medrxiv
                );
            };
            threads_vec.push(thread::spawn(move || {
                rxiv_part(
                    get_medrxiv_links(),
                    ENABLE_PRINTS_MEDRXIV,
                    ENABLE_WARNING_PRINTS_MEDRXIV,
                    ENABLE_ERROR_PRINTS_MEDRXIV,
                    MEDRXIV_URL,
                    RxivKind::Medrxiv,
                );
            }));
        }
    }
    for i in threads_vec {
        i.join().unwrap();
    }
}
