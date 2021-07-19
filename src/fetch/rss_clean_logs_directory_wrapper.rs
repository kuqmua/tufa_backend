use crate::fetch::rss_clean_logs_directory::rss_clean_logs_directory;
use config_lib::get_project_information::get_config::get_config_information::CONFIG;
use config_lib::get_project_information::provider_kind_enum::ProviderKind;

pub fn rss_clean_logs_directory_wrapper() {
    if CONFIG
        .enable_providers_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_arxiv
    {
        rss_clean_logs_directory(
            ProviderKind::Arxiv,
            CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_providers_prints.enable_prints_arxiv,
            CONFIG.params.enable_error_prints_for_all_providers
                && CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_arxiv,
        );
    }

    if CONFIG
        .enable_providers_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_biorxiv
    {
        rss_clean_logs_directory(
            ProviderKind::Biorxiv,
            CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_providers_prints.enable_prints_biorxiv,
            CONFIG.params.enable_error_prints_for_all_providers
                && CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_biorxiv,
        );
    }

    if CONFIG
        .enable_providers_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_habr
    {
        rss_clean_logs_directory(
            ProviderKind::Habr,
            CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_providers_prints.enable_prints_habr,
            CONFIG.params.enable_error_prints_for_all_providers
                && CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_habr,
        );
    }

    if CONFIG
        .enable_providers_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_medrxiv
    {
        rss_clean_logs_directory(
            ProviderKind::Medrxiv,
            CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_providers_prints.enable_prints_medrxiv,
            CONFIG.params.enable_error_prints_for_all_providers
                && CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_medrxiv,
        );
    }
    if CONFIG
        .enable_providers_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_reddit
    {
        rss_clean_logs_directory(
            ProviderKind::Reddit,
            CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_providers_prints.enable_prints_reddit,
            CONFIG.params.enable_error_prints_for_all_providers
                && CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_reddit,
        );
    }
    if CONFIG
        .enable_providers_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_twitter
    {
        rss_clean_logs_directory(
            ProviderKind::Twitter,
            CONFIG.params.enable_all_providers_prints
                && CONFIG.enable_providers_prints.enable_prints_twitter,
            CONFIG.params.enable_error_prints_for_all_providers
                && CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_twitter,
        );
    }
}
