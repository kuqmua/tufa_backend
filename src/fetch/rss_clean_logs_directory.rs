use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::get_project_information::get_config::get_config_information::CONFIG;
use crate::overriding::prints::print_error_red;
use std::fs;
use std::path::Path;

pub fn rss_clean_logs_directory() {
    if CONFIG
        .enable_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_arxiv
    {
        let path = format!(
            "logs/{}/{:?}",
            &CONFIG.params.warning_logs_directory_name,
            ProviderKind::Arxiv
        );
        if Path::new(&path).is_dir() {
            let result_of_recursively_removing_warning_logs_directory = fs::remove_dir_all(&path);
            match result_of_recursively_removing_warning_logs_directory {
                Ok(_) => {
                    if CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_arxiv
                    {
                        println!("folder {} has been deleted", &path);
                    }
                }
                Err(e) => {
                    if CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_arxiv
                    {
                        let error_message =
                            format!("delete folder problem{} {}", &path, e.to_string());
                        print_error_red(file!().to_string(), line!().to_string(), error_message)
                    }
                }
            }
        }
    }

    if CONFIG
        .enable_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_biorxiv
    {
        let path = format!(
            "logs/{}/{:?}",
            &CONFIG.params.warning_logs_directory_name,
            ProviderKind::Biorxiv
        );
        if Path::new(&path).is_dir() {
            let result_of_recursively_removing_warning_logs_directory = fs::remove_dir_all(&path);
            match result_of_recursively_removing_warning_logs_directory {
                Ok(_) => {
                    if CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_biorxiv
                    {
                        println!("folder {} has been deleted", &path);
                    }
                }
                Err(e) => {
                    if CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_biorxiv
                    {
                        let error_message =
                            format!("delete folder problem{} {}", &path, e.to_string());
                        print_error_red(file!().to_string(), line!().to_string(), error_message)
                    }
                }
            }
        }
    }

    if CONFIG
        .enable_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_habr
    {
        let path = format!(
            "logs/{}/{:?}",
            &CONFIG.params.warning_logs_directory_name,
            ProviderKind::Habr
        );
        if Path::new(&path).is_dir() {
            let result_of_recursively_removing_warning_logs_directory = fs::remove_dir_all(&path);
            match result_of_recursively_removing_warning_logs_directory {
                Ok(_) => {
                    if CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_habr
                    {
                        println!("folder {} has been deleted", &path);
                    }
                }
                Err(e) => {
                    if CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_habr
                    {
                        let error_message =
                            format!("delete folder problem{} {}", &path, e.to_string());
                        print_error_red(file!().to_string(), line!().to_string(), error_message)
                    }
                }
            }
        }
    }

    if CONFIG
        .enable_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_medrxiv
    {
        let path = format!(
            "logs/{}/{:?}",
            &CONFIG.params.warning_logs_directory_name,
            ProviderKind::Medrxiv
        );
        if Path::new(&path).is_dir() {
            let result_of_recursively_removing_warning_logs_directory = fs::remove_dir_all(&path);
            match result_of_recursively_removing_warning_logs_directory {
                Ok(_) => {
                    if CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_medrxiv
                    {
                        println!("folder {} has been deleted", &path);
                    }
                }
                Err(e) => {
                    if CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_medrxiv
                    {
                        let error_message =
                            format!("delete folder problem{} {}", &path, e.to_string());
                        print_error_red(file!().to_string(), line!().to_string(), error_message)
                    }
                }
            }
        }
    }
    if CONFIG
        .enable_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_reddit
    {
        let path = format!(
            "logs/{}/{:?}",
            &CONFIG.params.warning_logs_directory_name,
            ProviderKind::Reddit
        );
        if Path::new(&path).is_dir() {
            let result_of_recursively_removing_warning_logs_directory = fs::remove_dir_all(&path);
            match result_of_recursively_removing_warning_logs_directory {
                Ok(_) => {
                    if CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_reddit
                    {
                        println!("folder {} has been deleted", &path);
                    }
                }
                Err(e) => {
                    if CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_reddit
                    {
                        let error_message =
                            format!("delete folder problem{} {}", &path, e.to_string());
                        print_error_red(file!().to_string(), line!().to_string(), error_message)
                    }
                }
            }
        }
    }
    if CONFIG
        .enable_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_twitter
    {
        let path = format!(
            "logs/{}/{:?}",
            &CONFIG.params.warning_logs_directory_name,
            ProviderKind::Twitter
        );
        if Path::new(&path).is_dir() {
            let result_of_recursively_removing_warning_logs_directory = fs::remove_dir_all(&path);
            match result_of_recursively_removing_warning_logs_directory {
                Ok(_) => {
                    if CONFIG.params.enable_all_providers_prints
                        && CONFIG.enable_prints.enable_prints_twitter
                    {
                        println!("folder {} has been deleted", &path);
                    }
                }
                Err(e) => {
                    if CONFIG.params.enable_error_prints_for_all_providers
                        && CONFIG.enable_error_prints.enable_error_prints_for_twitter
                    {
                        let error_message =
                            format!("delete folder problem{} {}", &path, e.to_string());
                        print_error_red(file!().to_string(), line!().to_string(), error_message)
                    }
                }
            }
        }
    }
}
