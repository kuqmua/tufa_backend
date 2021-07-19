use std::thread;
use std::thread::JoinHandle;

use crate::fetch::rss_part::rss_part;

use prints_lib::print_colorful_message;
use prints_lib::PrintType;

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use std::sync::{Arc, Mutex};

use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_metainfo_fetch_structures::HandledFetchStatusInfo;
use crate::fetch::rss_metainfo_fetch_structures::UnhandledFetchStatusInfo;

use providers_info_lib::get_project_information::get_twitter_providers_names::get_twitter_providers_names;

use config_lib::get_project_information::get_config::get_config_information::CONFIG;
use config_lib::get_project_information::project_constants::get_config_provider_string_to_enum_struct;

use config_lib::get_project_information::provider_kind_enum::ProviderKind;

use providers_info_lib::get_providers_link_parts_wrapper::get_providers_link_parts_wrapper;

use providers_info_lib::get_project_information::generate_hashmap_links::generate_arxiv_hashmap_links::generate_arxiv_hashmap_links;
use providers_info_lib::get_project_information::generate_hashmap_links::generate_biorxiv_hashmap_links::generate_biorxiv_hashmap_links;
use providers_info_lib::get_project_information::generate_hashmap_links::generate_github_hashmap_links::generate_github_hashmap_links;
use providers_info_lib::get_project_information::generate_hashmap_links::generate_habr_hashmap_links::generate_habr_hashmap_links;
use providers_info_lib::get_project_information::generate_hashmap_links::generate_medrxiv_hashmap_links::generate_medrxiv_hashmap_links;
use providers_info_lib::get_project_information::generate_hashmap_links::generate_reddit_hashmap_links::generate_reddit_hashmap_links;
use providers_info_lib::get_project_information::generate_hashmap_links::generate_twitter_hashmap_links::generate_twitter_hashmap_links;

pub async fn check_new_posts_threads_parts() -> Option<(
    Vec<CommonRssPostStruct>,
    Vec<(
        String,
        UnhandledFetchStatusInfo,
        HandledFetchStatusInfo,
        AreThereItems,
        ProviderKind,
    )>,
)> {
    if CONFIG.params.enable_providers {
        if !CONFIG.params.vec_of_provider_names.is_empty() {
            let option_providers_link_parts = get_providers_link_parts_wrapper();
            match option_providers_link_parts {
                Some(providers_link_parts) => {
                    if !providers_link_parts.is_empty() {
                        // This
                        let mut threads_vec: Vec<JoinHandle<()>> =
                            Vec::with_capacity(CONFIG.params.vec_of_provider_names.len());
                        let posts = Arc::new(Mutex::new(Vec::<CommonRssPostStruct>::new()));
                        let error_posts = Arc::new(Mutex::new(Vec::<(
                            String,
                            UnhandledFetchStatusInfo,
                            HandledFetchStatusInfo,
                            AreThereItems,
                            ProviderKind,
                        )>::new()));

                        let config_provider_string_to_enum_struct_hashmap =
                            get_config_provider_string_to_enum_struct();
                        //check if provider_names are unique
                        for provider_name in &CONFIG.params.vec_of_provider_names {
                            match config_provider_string_to_enum_struct_hashmap.get(provider_name) {
                                Some(provider_kind_handle) => match provider_kind_handle {
                                    ProviderKind::Arxiv => {
                                        if CONFIG.enable_providers.enable_arxiv {
                                            let arxiv_link_parts =
                                                providers_link_parts[provider_name].clone(); //redo this later - .clone() its just to compile the code
                                            if arxiv_link_parts.is_empty() {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "arxiv_link_parts.is_empty".to_string(),
                                                );
                                            } else {
                                                if CONFIG.params.enable_all_providers_prints
                                                    && CONFIG.enable_prints.enable_prints_arxiv
                                                {
                                                    println!(
                                                        "{:#?} elements in {:#?} HashMap",
                                                        arxiv_link_parts.len(),
                                                        provider_kind_handle
                                                    );
                                                };
                                                let posts_handle = Arc::clone(&posts);
                                                let error_posts_handle = Arc::clone(&error_posts);
                                                let provider_kind_handle_clone =
                                                    provider_kind_handle.clone();
                                                let vec_of_provider_links =
                                                    generate_arxiv_hashmap_links(arxiv_link_parts);
                                                threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG.enable_prints.enable_prints_arxiv,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_high_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_high_prints
                                                            .enable_warning_high_prints_for_arxiv,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_arxiv,
                                                    CONFIG.params.enable_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_arxiv_time_measurement,
                                                    &CONFIG.links.arxiv_link,
                                                    provider_kind_handle_clone,
                                                    CONFIG.params.enable_error_prints,
                                                    vec_of_provider_links,
                                                    None
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }
                                        ));
                                            }
                                        }
                                    }
                                    ProviderKind::Biorxiv => {
                                        if CONFIG.enable_providers.enable_biorxiv {
                                            let biorxiv_link_parts =
                                                providers_link_parts[provider_name].clone(); //redo this later - .clone() its just to compile the code
                                            if biorxiv_link_parts.is_empty() {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "biorxiv_link_parts.is_empty".to_string(),
                                                );
                                            } else {
                                                if CONFIG.params.enable_all_providers_prints
                                                    && CONFIG.enable_prints.enable_prints_biorxiv
                                                {
                                                    println!(
                                                        "{:#?} elements in {:#?} HashMap",
                                                        biorxiv_link_parts.len(),
                                                        provider_kind_handle
                                                    );
                                                };
                                                let posts_handle = Arc::clone(&posts);
                                                let error_posts_handle = Arc::clone(&error_posts);
                                                let provider_kind_handle_clone =
                                                    provider_kind_handle.clone();
                                                let vec_of_provider_links =
                                                    generate_biorxiv_hashmap_links(
                                                        biorxiv_link_parts,
                                                    );
                                                threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG.enable_prints.enable_prints_biorxiv,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_high_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_high_prints
                                                            .enable_warning_high_prints_for_biorxiv,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_biorxiv,
                                                    CONFIG.params.enable_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_biorxiv_time_measurement,
                                                    &CONFIG.links.biorxiv_link,
                                                    provider_kind_handle_clone,
                                                    CONFIG.params.enable_error_prints,
                                                    vec_of_provider_links,
                                                    None
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }
                                        ));
                                            }
                                        }
                                    }
                                    ProviderKind::Github => {
                                        if CONFIG.enable_providers.enable_github {
                                            let github_link_parts =
                                                providers_link_parts[provider_name].clone(); //redo this later - .clone() its just to compile the code
                                            if github_link_parts.is_empty() {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "github_links.is_empty".to_string(),
                                                );
                                            } else {
                                                if CONFIG.params.enable_all_providers_prints
                                                    && CONFIG.enable_prints.enable_prints_github
                                                {
                                                    println!(
                                                        "{:#?} elements in {:#?} HashMap",
                                                        github_link_parts.len(),
                                                        provider_kind_handle
                                                    );
                                                };

                                                let posts_handle = Arc::clone(&posts);
                                                let error_posts_handle = Arc::clone(&error_posts);
                                                let provider_kind_handle_clone =
                                                    provider_kind_handle.clone();
                                                let vec_of_provider_links =
                                                    generate_github_hashmap_links(
                                                        github_link_parts,
                                                    );
                                                println!(
                                                    "github_link_parts vec_of_provider_links {:#?}",
                                                    vec_of_provider_links
                                                );

                                                threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG.enable_prints.enable_prints_github,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_high_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_high_prints
                                                            .enable_warning_high_prints_for_github,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_github,
                                                    CONFIG.params.enable_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_github_time_measurement,
                                                    &CONFIG.links.github_link,
                                                    provider_kind_handle_clone,
                                                    CONFIG.params.enable_error_prints,
                                                    vec_of_provider_links,
                                                    None
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }
                                        ));
                                            }
                                        }
                                    }
                                    ProviderKind::Habr => {
                                        if CONFIG.enable_providers.enable_habr {
                                            let habr_link_parts =
                                                providers_link_parts[provider_name].clone(); //redo this later - .clone() its just to compile the code
                                            if habr_link_parts.is_empty() {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "habr_link_parts.is_empty".to_string(),
                                                );
                                            } else {
                                                if CONFIG.params.enable_all_providers_prints
                                                    && CONFIG.enable_prints.enable_prints_habr
                                                {
                                                    println!(
                                                        "{:#?} elements in {:#?} HashMap",
                                                        habr_link_parts.len(),
                                                        provider_kind_handle
                                                    );
                                                };
                                                let posts_handle = Arc::clone(&posts);
                                                let error_posts_handle = Arc::clone(&error_posts);
                                                let provider_kind_handle_clone =
                                                    provider_kind_handle.clone();
                                                let vec_of_provider_links =
                                                    generate_habr_hashmap_links(habr_link_parts);
                                                threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG.enable_prints.enable_prints_habr,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_high_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_high_prints
                                                            .enable_warning_high_prints_for_habr,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_habr,
                                                    CONFIG.params.enable_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_habr_time_measurement,
                                                    &CONFIG.links.habr_link,
                                                    provider_kind_handle_clone,
                                                    CONFIG.params.enable_error_prints,
                                                    vec_of_provider_links,
                                                    None
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }
                                        ));
                                            }
                                        }
                                    }
                                    ProviderKind::Medrxiv => {
                                        if CONFIG.enable_providers.enable_medrxiv {
                                            let medrxiv_link_parts =
                                                providers_link_parts[provider_name].clone(); //redo this later - .clone() its just to compile the code
                                            if medrxiv_link_parts.is_empty() {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "medrxiv_link_parts.is_empty".to_string(),
                                                );
                                            } else {
                                                if CONFIG.params.enable_all_providers_prints
                                                    && CONFIG.enable_prints.enable_prints_medrxiv
                                                {
                                                    println!(
                                                        "{:#?} elements in {:#?} HashMap",
                                                        medrxiv_link_parts.len(),
                                                        provider_kind_handle
                                                    );
                                                };
                                                let posts_handle = Arc::clone(&posts);
                                                let error_posts_handle = Arc::clone(&error_posts);
                                                let provider_kind_handle_clone =
                                                    provider_kind_handle.clone();
                                                let vec_of_provider_links =
                                                    generate_medrxiv_hashmap_links(
                                                        medrxiv_link_parts,
                                                    );
                                                threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG.enable_prints.enable_prints_medrxiv,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_high_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_high_prints
                                                            .enable_warning_high_prints_for_medrxiv,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_medrxiv,
                                                    CONFIG.params.enable_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_medrxiv_time_measurement,
                                                    &CONFIG.links.medrxiv_link,
                                                    provider_kind_handle_clone,
                                                    CONFIG.params.enable_error_prints,
                                                    vec_of_provider_links,
                                                    None
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }
                                        ));
                                            }
                                        }
                                    }
                                    ProviderKind::Reddit => {
                                        if CONFIG.enable_providers.enable_reddit {
                                            let reddit_link_parts =
                                                providers_link_parts[provider_name].clone(); //redo this later - .clone() its just to compile the code
                                            if reddit_link_parts.is_empty() {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "reddit_link_parts.is_empty".to_string(),
                                                );
                                            } else {
                                                if CONFIG.params.enable_all_providers_prints
                                                    && CONFIG.enable_prints.enable_prints_reddit
                                                {
                                                    println!(
                                                        "{:#?} elements in {:#?} HashMap",
                                                        reddit_link_parts.len(),
                                                        provider_kind_handle
                                                    );
                                                };
                                                let posts_handle = Arc::clone(&posts);
                                                let error_posts_handle = Arc::clone(&error_posts);
                                                let provider_kind_handle_clone =
                                                    provider_kind_handle.clone();
                                                let vec_of_provider_links =
                                                    generate_reddit_hashmap_links(
                                                        reddit_link_parts,
                                                    );
                                                threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG.enable_prints.enable_prints_reddit,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_high_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_high_prints
                                                            .enable_warning_high_prints_for_reddit,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_reddit,
                                                    CONFIG.params.enable_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_reddit_time_measurement,
                                                    &CONFIG.links.reddit_link,
                                                    provider_kind_handle_clone,
                                                    CONFIG.params.enable_error_prints,
                                                    vec_of_provider_links,
                                                    None
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }
                                        ));
                                            }
                                        }
                                    }
                                    ProviderKind::Twitter => {
                                        if CONFIG.enable_providers.enable_twitter {
                                            let twitter_link_parts =
                                                providers_link_parts[provider_name].clone(); //redo this later - .clone() its just to compile the code
                                            let twitter_providers = get_twitter_providers_names();
                                            if twitter_link_parts.is_empty() {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "twitter_link_parts.is_empty".to_string(),
                                                );
                                            } else if twitter_providers.is_empty() {
                                                print_colorful_message(
                                                    Some(provider_kind_handle),
                                                    PrintType::Error,
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "twitter_providers.is_empty()".to_string(),
                                                );
                                            } else {
                                                if CONFIG.params.enable_all_providers_prints
                                                    && CONFIG.enable_prints.enable_prints_twitter
                                                {
                                                    println!(
                                                        "{:#?} elements in {:#?} twitter_link_parts HashMap",
                                                        twitter_link_parts.len(),
                                                        provider_kind_handle
                                                    );
                                                    println!(
                                                        "{:#?} elements in {:#?} twitter_providers HashMap",
                                                        twitter_providers.len(),
                                                        provider_kind_handle
                                                    );
                                                };
                                                let posts_handle = Arc::clone(&posts);
                                                let error_posts_handle = Arc::clone(&error_posts);
                                                let provider_kind_handle_clone =
                                                    provider_kind_handle.clone();

                                                let vec_of_provider_links =
                                                    generate_twitter_hashmap_links(
                                                        twitter_providers.clone(),
                                                        twitter_link_parts,
                                                    );
                                                println!(
                                                    "vec_of_provider_links {:#?}",
                                                    vec_of_provider_links
                                                );
                                                threads_vec.push(thread::spawn(move || {
                                                let enum_success_unsuccess_option_posts = rss_part(
                                                    CONFIG.params.enable_all_providers_prints
                                                        && CONFIG.enable_prints.enable_prints_twitter,
                                                    CONFIG
                                                        .params
                                                        .enable_warning_high_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_warning_high_prints
                                                            .enable_warning_high_prints_for_twitter,
                                                    CONFIG
                                                        .params
                                                        .enable_error_prints_for_all_providers
                                                        && CONFIG
                                                            .enable_error_prints
                                                            .enable_error_prints_for_twitter,
                                                    CONFIG.params.enable_time_measurement
                                                        && CONFIG
                                                            .enable_time_measurement
                                                            .enable_twitter_time_measurement,
                                                    &CONFIG.links.twitter_link,
                                                    provider_kind_handle_clone,
                                                    CONFIG.params.enable_error_prints,
                                                    vec_of_provider_links,
                                                    Some(twitter_providers)
                                                );
                                                if let Some(success_posts) =
                                                    enum_success_unsuccess_option_posts.0
                                                {
                                                    let mut posts_handle_locked =
                                                        posts_handle.lock().unwrap();
                                                    for value in success_posts {
                                                        posts_handle_locked.push(value);
                                                    }
                                                }
                                                if let Some(unsuccess_posts) =
                                                    enum_success_unsuccess_option_posts.1
                                                {
                                                    let mut error_posts_handle_locked =
                                                        error_posts_handle.lock().unwrap();
                                                    for unsuccess_post in unsuccess_posts {
                                                        error_posts_handle_locked
                                                            .push(unsuccess_post);
                                                    }
                                                }
                                            }
                                        ));
                                            }
                                        }
                                    }
                                },
                                None => {
                                    print_colorful_message(
                                        None,
                                        PrintType::WarningLow,
                                        file!().to_string(),
                                        line!().to_string(),
                                        "config_provider_string_to_enum_struct_hashmap.get(provider_name) is None".to_string(),
                                    );
                                }
                            }
                        }
                        for i in threads_vec {
                            i.join().unwrap();
                        }
                        let posts_done = posts.lock().unwrap().to_vec();
                        let error_posts_done = error_posts.lock().unwrap().to_vec();
                        Some((posts_done, error_posts_done))
                    } else {
                        print_colorful_message(
                            None,
                            PrintType::Error,
                            file!().to_string(),
                            line!().to_string(),
                            "providers_link_parts is empty".to_string(),
                        );
                        None
                    }
                }
                None => {
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        file!().to_string(),
                        line!().to_string(),
                        format!(
                            "option_providers_link_parts {:#?}",
                            option_providers_link_parts
                        ),
                    );
                    None
                }
            }
        } else {
            print_colorful_message(
                None,
                PrintType::WarningLow,
                file!().to_string(),
                line!().to_string(),
                "CONFIG.params.vec_of_provider_names is empty".to_string(),
            );
            None
        }
    } else {
        print_colorful_message(
            None,
            PrintType::WarningLow,
            file!().to_string(),
            line!().to_string(),
            "CONFIG.params.enable_all_providers is false".to_string(),
        );
        None
    }
}
