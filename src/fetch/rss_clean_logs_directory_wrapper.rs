use crate::fetch::rss_clean_logs_directory::rss_clean_logs_directory;
use crate::config_mods::config::CONFIG;
use crate::providers_info::provider_kind_enum::ProviderKind;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn rss_clean_logs_directory_wrapper() {
    if CONFIG
        .enable_providers_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_arxiv
    {
        rss_clean_logs_directory(ProviderKind::Arxiv);
    }

    if CONFIG
        .enable_providers_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_biorxiv
    {
        rss_clean_logs_directory(ProviderKind::Biorxiv);
    }

    if CONFIG
        .enable_providers_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_habr
    {
        rss_clean_logs_directory(ProviderKind::Habr);
    }

    if CONFIG
        .enable_providers_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_medrxiv
    {
        rss_clean_logs_directory(ProviderKind::Medrxiv);
    }
    if CONFIG
        .enable_providers_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_reddit
    {
        rss_clean_logs_directory(ProviderKind::Reddit);
    }
    if CONFIG
        .enable_providers_cleaning_warning_logs_directory
        .enable_cleaning_warning_logs_directory_for_twitter
    {
        rss_clean_logs_directory(ProviderKind::Twitter);
    }
}
