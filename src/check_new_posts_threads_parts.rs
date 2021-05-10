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
    let warning_logs_directory_name = CONFIG.params.warning_logs_directory_name.clone();
    let reddit_user_agent = CONFIG.reddit_authorization.reddit_user_agent.clone();
    let reddit_client_id = CONFIG.reddit_authorization.reddit_client_id.clone();
    let reddit_client_secret = CONFIG.reddit_authorization.reddit_client_secret.clone();
    let reddit_username = CONFIG.reddit_authorization.reddit_username.clone();
    let reddit_password = CONFIG.reddit_authorization.reddit_password.clone();
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
            let enable_cleaning_warning_logs_directory_for_arxiv =
                CONFIG.params.enable_all_time_measurement
                    && CONFIG
                        .enable_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_arxiv;
            let enable_prints_arxiv = CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_arxiv;
            let enable_warning_prints_for_arxiv =
                CONFIG.params.enable_warning_prints_for_all_providers
                    && CONFIG.enable_warning_prints.enable_warning_prints_for_arxiv;
            let enable_error_prints_for_arxiv = CONFIG.params.enable_error_prints_for_all_providers
                && CONFIG.enable_error_prints.enable_error_prints_for_arxiv;
            let enable_arxiv_time_measurement = CONFIG.params.enable_all_time_measurement
                && CONFIG.enable_time_measurement.enable_arxiv_time_measurement;
            let enable_error_prints_handle = CONFIG.params.enable_error_prints_handle;
            let warning_logs_directory_name_clone = warning_logs_directory_name.clone();
            let reddit_user_agent_clone = reddit_user_agent.clone();
            let reddit_client_id_clone = reddit_client_id.clone();
            let reddit_client_secret_clone = reddit_client_secret.clone();
            let reddit_username_clone = reddit_username.clone();
            let reddit_password_clone = reddit_password.clone();
            if enable_prints_arxiv {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    arxiv_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    enable_cleaning_warning_logs_directory_for_arxiv,
                    enable_prints_arxiv,
                    enable_warning_prints_for_arxiv,
                    enable_error_prints_for_arxiv,
                    enable_arxiv_time_measurement,
                    &CONFIG.links.arxiv_link,
                    &PROVIDER_KIND,
                    enable_error_prints_handle,
                    warning_logs_directory_name_clone,
                    &reddit_user_agent_clone,
                    &reddit_client_id_clone,
                    &reddit_client_secret_clone,
                    &reddit_username_clone,
                    &reddit_password_clone,
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
            let enable_cleaning_warning_logs_directory_for_biorxiv =
                CONFIG.params.enable_all_time_measurement
                    && CONFIG
                        .enable_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_biorxiv;
            let enable_prints_biorxiv = CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_biorxiv;
            let enable_warning_prints_for_biorxiv =
                CONFIG.params.enable_warning_prints_for_all_providers
                    && CONFIG
                        .enable_warning_prints
                        .enable_warning_prints_for_biorxiv;
            let enable_error_prints_for_biorxiv =
                CONFIG.params.enable_error_prints_for_all_providers
                    && CONFIG.enable_error_prints.enable_error_prints_for_biorxiv;
            let enable_biorxiv_time_measurement = CONFIG.params.enable_all_time_measurement
                && CONFIG
                    .enable_time_measurement
                    .enable_biorxiv_time_measurement;
            let enable_error_prints_handle = CONFIG.params.enable_error_prints_handle;
            let warning_logs_directory_name_clone = warning_logs_directory_name.clone();
            let reddit_user_agent_clone = reddit_user_agent.clone();
            let reddit_client_id_clone = reddit_client_id.clone();
            let reddit_client_secret_clone = reddit_client_secret.clone();
            let reddit_username_clone = reddit_username.clone();
            let reddit_password_clone = reddit_password.clone();
            if enable_prints_biorxiv {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    biorxiv_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    enable_cleaning_warning_logs_directory_for_biorxiv,
                    enable_prints_biorxiv,
                    enable_warning_prints_for_biorxiv,
                    enable_error_prints_for_biorxiv,
                    enable_biorxiv_time_measurement,
                    &CONFIG.links.biorxiv_link,
                    &PROVIDER_KIND,
                    enable_error_prints_handle,
                    warning_logs_directory_name_clone,
                    &reddit_user_agent_clone,
                    &reddit_client_id_clone,
                    &reddit_client_secret_clone,
                    &reddit_username_clone,
                    &reddit_password_clone,
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
            let enable_cleaning_warning_logs_directory_for_habr =
                CONFIG.params.enable_all_time_measurement
                    && CONFIG
                        .enable_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_habr;
            let enable_prints_habr = CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_habr;
            let enable_warning_prints_for_habr =
                CONFIG.params.enable_warning_prints_for_all_providers
                    && CONFIG.enable_warning_prints.enable_warning_prints_for_habr;
            let enable_error_prints_for_habr = CONFIG.params.enable_error_prints_for_all_providers
                && CONFIG.enable_error_prints.enable_error_prints_for_habr;
            let enable_habr_time_measurement = CONFIG.params.enable_all_time_measurement
                && CONFIG.enable_time_measurement.enable_habr_time_measurement;
            let enable_error_prints_handle = CONFIG.params.enable_error_prints_handle;
            let warning_logs_directory_name_clone = warning_logs_directory_name.clone();
            let reddit_user_agent_clone = reddit_user_agent.clone();
            let reddit_client_id_clone = reddit_client_id.clone();
            let reddit_client_secret_clone = reddit_client_secret.clone();
            let reddit_username_clone = reddit_username.clone();
            let reddit_password_clone = reddit_password.clone();
            if enable_prints_habr {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    habr_links.len(),
                    &PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    enable_cleaning_warning_logs_directory_for_habr,
                    enable_prints_habr,
                    enable_warning_prints_for_habr,
                    enable_error_prints_for_habr,
                    enable_habr_time_measurement,
                    &CONFIG.links.habr_link,
                    &PROVIDER_KIND,
                    enable_error_prints_handle,
                    warning_logs_directory_name_clone,
                    &reddit_user_agent_clone,
                    &reddit_client_id_clone,
                    &reddit_client_secret_clone,
                    &reddit_username_clone,
                    &reddit_password_clone,
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
            let enable_cleaning_warning_logs_directory_for_medrxiv =
                CONFIG.params.enable_all_time_measurement
                    && CONFIG
                        .enable_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_medrxiv;
            let enable_prints_medrxiv = CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_medrxiv;
            let enable_warning_prints_for_medrxiv =
                CONFIG.params.enable_warning_prints_for_all_providers
                    && CONFIG
                        .enable_warning_prints
                        .enable_warning_prints_for_medrxiv;
            let enable_error_prints_for_medrxiv =
                CONFIG.params.enable_error_prints_for_all_providers
                    && CONFIG.enable_error_prints.enable_error_prints_for_medrxiv;
            let enable_medrxiv_time_measurement = CONFIG.params.enable_all_time_measurement
                && CONFIG
                    .enable_time_measurement
                    .enable_medrxiv_time_measurement;
            let enable_error_prints_handle = CONFIG.params.enable_error_prints_handle;
            let warning_logs_directory_name_clone = warning_logs_directory_name.clone();
            let reddit_user_agent_clone = reddit_user_agent.clone();
            let reddit_client_id_clone = reddit_client_id.clone();
            let reddit_client_secret_clone = reddit_client_secret.clone();
            let reddit_username_clone = reddit_username.clone();
            let reddit_password_clone = reddit_password.clone();
            if enable_prints_medrxiv {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    medrxiv_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    enable_cleaning_warning_logs_directory_for_medrxiv,
                    enable_prints_medrxiv,
                    enable_warning_prints_for_medrxiv,
                    enable_error_prints_for_medrxiv,
                    enable_medrxiv_time_measurement,
                    &CONFIG.links.medrxiv_link,
                    &PROVIDER_KIND,
                    enable_error_prints_handle,
                    warning_logs_directory_name_clone,
                    &reddit_user_agent_clone,
                    &reddit_client_id_clone,
                    &reddit_client_secret_clone,
                    &reddit_username_clone,
                    &reddit_password_clone,
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
            let enable_cleaning_warning_logs_directory_for_reddit =
                CONFIG.params.enable_all_time_measurement
                    && CONFIG
                        .enable_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_reddit;
            let enable_prints_reddit = CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_reddit;
            let enable_warning_prints_for_reddit =
                CONFIG.params.enable_warning_prints_for_all_providers
                    && CONFIG
                        .enable_warning_prints
                        .enable_warning_prints_for_reddit;
            let enable_error_prints_for_reddit =
                CONFIG.params.enable_error_prints_for_all_providers
                    && CONFIG.enable_error_prints.enable_error_prints_for_reddit;
            let enable_reddit_time_measurement = CONFIG.params.enable_all_time_measurement
                && CONFIG
                    .enable_time_measurement
                    .enable_reddit_time_measurement;
            let enable_error_prints_handle = CONFIG.params.enable_error_prints_handle;
            let warning_logs_directory_name_clone = warning_logs_directory_name.clone();
            let reddit_user_agent_clone = reddit_user_agent.clone();
            let reddit_client_id_clone = reddit_client_id.clone();
            let reddit_client_secret_clone = reddit_client_secret.clone();
            let reddit_username_clone = reddit_username.clone();
            let reddit_password_clone = reddit_password.clone();
            if enable_prints_reddit {
                // ENABLE_PRINTS_REDDIT
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    reddit_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    enable_cleaning_warning_logs_directory_for_reddit,
                    enable_prints_reddit,
                    enable_warning_prints_for_reddit,
                    enable_error_prints_for_reddit,
                    enable_reddit_time_measurement,
                    &CONFIG.links.reddit_link,
                    &PROVIDER_KIND,
                    enable_error_prints_handle,
                    warning_logs_directory_name_clone,
                    &reddit_user_agent_clone,
                    &reddit_client_id_clone,
                    &reddit_client_secret_clone,
                    &reddit_username_clone,
                    &reddit_password_clone,
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
            let enable_cleaning_warning_logs_directory_for_twitter =
                CONFIG.params.enable_all_time_measurement
                    && CONFIG
                        .enable_cleaning_warning_logs_directory
                        .enable_cleaning_warning_logs_directory_for_twitter;
            let enable_prints_twitter = CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_prints.enable_prints_twitter;
            let enable_warning_prints_for_twitter =
                CONFIG.params.enable_warning_prints_for_all_providers
                    && CONFIG
                        .enable_warning_prints
                        .enable_warning_prints_for_twitter;
            let enable_error_prints_for_twitter =
                CONFIG.params.enable_error_prints_for_all_providers
                    && CONFIG.enable_error_prints.enable_error_prints_for_twitter;
            let enable_twitter_time_measurement = CONFIG.params.enable_all_time_measurement
                && CONFIG
                    .enable_time_measurement
                    .enable_twitter_time_measurement;
            let enable_error_prints_handle = CONFIG.params.enable_error_prints_handle;
            let warning_logs_directory_name_clone = warning_logs_directory_name.clone();
            let reddit_user_agent_clone = reddit_user_agent.clone();
            let reddit_client_id_clone = reddit_client_id.clone();
            let reddit_client_secret_clone = reddit_client_secret.clone();
            let reddit_username_clone = reddit_username.clone();
            let reddit_password_clone = reddit_password.clone();
            if enable_prints_twitter {
                println!(
                    "{:#?} elements in {:#?} HashMap",
                    twitter_links.len(),
                    PROVIDER_KIND
                );
            };
            threads_vec.push(thread::spawn(move || {
                rss_part(
                    enable_cleaning_warning_logs_directory_for_twitter,
                    enable_prints_twitter,
                    enable_warning_prints_for_twitter,
                    enable_error_prints_for_twitter,
                    enable_twitter_time_measurement,
                    &CONFIG.links.twitter_link,
                    &PROVIDER_KIND,
                    enable_error_prints_handle,
                    warning_logs_directory_name_clone,
                    &reddit_user_agent_clone,
                    &reddit_client_id_clone,
                    &reddit_client_secret_clone,
                    &reddit_username_clone,
                    &reddit_password_clone,
                );
            }));
        }
    }
    for i in threads_vec {
        i.join().unwrap();
    }
}
