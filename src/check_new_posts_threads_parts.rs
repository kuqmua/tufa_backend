use std::thread;

use crate::get_project_information::get_names::get_arxiv_names::get_arxiv_names;
use crate::get_project_information::get_names::get_biorxiv_names::get_biorxiv_names;
use crate::get_project_information::get_names::get_habr_names::get_habr_names;
use crate::get_project_information::get_names::get_medrxiv_names::get_medrxiv_names;
use crate::get_project_information::get_names::get_reddit_names::get_reddit_names;
use crate::get_project_information::get_names::get_twitter_names::get_twitter_names;

use crate::get_project_information::get_config::get_config_information::CONFIG;

use crate::fetch::rss_part::rss_part;

use crate::fetch::rss_provider_kind_enum::ProviderKind;

use crate::overriding::prints::print_error_red;

pub async fn check_new_posts_threads_parts() {
    let mut threads_vec = Vec::with_capacity(6);
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_arxiv {
        let arxiv_links = get_arxiv_names();
        if arxiv_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "arxiv_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Arxiv;
            if CONFIG.params.enable_all_providers_prints && CONFIG.enable_prints.enable_prints_arxiv
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    arxiv_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_cleaning_warning_logs_directory
                            .enable_cleaning_warning_logs_directory_for_arxiv,
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_arxiv,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG.enable_warning_prints.enable_warning_prints_for_arxiv,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_arxiv,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG.enable_time_measurement.enable_arxiv_time_measurement,
                    &CONFIG.links.arxiv_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_handle,
                    CONFIG.params.warning_logs_directory_name.clone(),
                );
            }));
        }
    }
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_biorxiv {
        let biorxiv_links = get_biorxiv_names();
        if biorxiv_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "biorxiv_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Biorxiv;
            if CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_biorxiv
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    biorxiv_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_cleaning_warning_logs_directory
                            .enable_cleaning_warning_logs_directory_for_biorxiv,
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_biorxiv,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG
                            .enable_warning_prints
                            .enable_warning_prints_for_biorxiv,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_biorxiv,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_time_measurement
                            .enable_biorxiv_time_measurement,
                    &CONFIG.links.biorxiv_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_handle,
                    CONFIG.params.warning_logs_directory_name.clone(),
                );
            }));
        }
    }
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_habr {
        let habr_links = get_habr_names();
        if habr_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "habr_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Habr;
            if CONFIG.params.enable_all_providers_prints && CONFIG.enable_prints.enable_prints_habr
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    habr_links.len(),
                    &PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_cleaning_warning_logs_directory
                            .enable_cleaning_warning_logs_directory_for_habr,
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_habr,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG.enable_warning_prints.enable_warning_prints_for_habr,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_habr,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG.enable_time_measurement.enable_habr_time_measurement,
                    &CONFIG.links.habr_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_handle,
                    CONFIG.params.warning_logs_directory_name.clone(),
                );
            }));
        }
    }
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_medrxiv {
        let medrxiv_links = get_medrxiv_names();
        if medrxiv_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "medrxiv_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Medrxiv;
            if CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_medrxiv
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    medrxiv_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_cleaning_warning_logs_directory
                            .enable_cleaning_warning_logs_directory_for_medrxiv,
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_medrxiv,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG
                            .enable_warning_prints
                            .enable_warning_prints_for_medrxiv,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_medrxiv,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_time_measurement
                            .enable_medrxiv_time_measurement,
                    &CONFIG.links.medrxiv_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_handle,
                    CONFIG.params.warning_logs_directory_name.clone(),
                );
            }));
        }
    }
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_reddit {
        let reddit_links = get_reddit_names();
        if reddit_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "arxiv_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Reddit;
            if CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_reddit
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    reddit_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_cleaning_warning_logs_directory
                            .enable_cleaning_warning_logs_directory_for_reddit,
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_reddit,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG
                            .enable_warning_prints
                            .enable_warning_prints_for_reddit,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_reddit,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_time_measurement
                            .enable_reddit_time_measurement,
                    &CONFIG.links.reddit_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_reddit,
                    CONFIG.params.warning_logs_directory_name.clone(),
                );
            }))
        };
    }
    if CONFIG.params.enable_all_providers && CONFIG.enable_providers.enable_twitter {
        let twitter_links = get_twitter_names();
        if twitter_links.is_empty() {
            print_error_red(
                file!().to_string(),
                line!().to_string(),
                "twitter_links.is_empty".to_string(),
            )
        } else {
            const PROVIDER_KIND: ProviderKind = ProviderKind::Twitter;
            if CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_twitter
            {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    twitter_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_cleaning_warning_logs_directory
                            .enable_cleaning_warning_logs_directory_for_twitter,
                    CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_twitter,
                    CONFIG.params.enable_warning_prints_for_all_providers
                        && CONFIG
                            .enable_warning_prints
                            .enable_warning_prints_for_twitter,
                    CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_twitter,
                    CONFIG.params.enable_all_time_measurement
                        && CONFIG
                            .enable_time_measurement
                            .enable_twitter_time_measurement,
                    &CONFIG.links.twitter_link,
                    &PROVIDER_KIND,
                    CONFIG.params.enable_error_prints_handle,
                    CONFIG.params.warning_logs_directory_name.clone(),
                );
            }));
        }
    }
    for i in threads_vec {
        i.join().unwrap();
    }
}
