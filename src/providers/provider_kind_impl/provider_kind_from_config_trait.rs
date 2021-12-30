use crate::config_mods::lazy_static_config::CONFIG;

use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;

use crate::providers::provider_kind_enum::ProviderKind;

impl ProviderKindFromConfigTrait for ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_initialization_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_mongo_initialization_enabled_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_mongo_initialization_enabled_for_biorxiv,
            ProviderKind::Github => CONFIG.is_mongo_initialization_enabled_for_github,
            ProviderKind::Medrxiv => CONFIG.is_mongo_initialization_enabled_for_medrxiv,
            ProviderKind::Twitter => CONFIG.is_mongo_initialization_enabled_for_twitter,
            ProviderKind::Reddit => CONFIG.is_mongo_initialization_enabled_for_reddit,
            ProviderKind::Habr => CONFIG.is_mongo_initialization_enabled_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_write_error_logs_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_mongo_write_error_logs_enabled_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_mongo_write_error_logs_enabled_for_biorxiv,
            ProviderKind::Github => CONFIG.is_mongo_write_error_logs_enabled_for_github,
            ProviderKind::Medrxiv => CONFIG.is_mongo_write_error_logs_enabled_for_medrxiv,
            ProviderKind::Twitter => CONFIG.is_mongo_write_error_logs_enabled_for_twitter,
            ProviderKind::Reddit => CONFIG.is_mongo_write_error_logs_enabled_for_reddit,
            ProviderKind::Habr => CONFIG.is_mongo_write_error_logs_enabled_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_cleaning_warning_logs_db_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_for_biorxiv,
            ProviderKind::Github => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_for_github,
            ProviderKind::Medrxiv => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_for_medrxiv,
            ProviderKind::Twitter => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_for_twitter,
            ProviderKind::Reddit => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_for_reddit,
            ProviderKind::Habr => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_cleaning_warning_logs_db_collections_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_for_github
            }
            ProviderKind::Medrxiv => {
                CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_for_medrxiv
            }
            ProviderKind::Twitter => {
                CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_for_twitter
            }
            ProviderKind::Reddit => {
                CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_for_reddit
            }
            ProviderKind::Habr => CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_link_parts_randomize_order_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_mongo_link_parts_randomize_order_enabled_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_mongo_link_parts_randomize_order_enabled_for_biorxiv,
            ProviderKind::Github => CONFIG.is_mongo_link_parts_randomize_order_enabled_for_github,
            ProviderKind::Medrxiv => CONFIG.is_mongo_link_parts_randomize_order_enabled_for_medrxiv,
            ProviderKind::Twitter => CONFIG.is_mongo_link_parts_randomize_order_enabled_for_twitter,
            ProviderKind::Reddit => CONFIG.is_mongo_link_parts_randomize_order_enabled_for_reddit,
            ProviderKind::Habr => CONFIG.is_mongo_link_parts_randomize_order_enabled_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_postgres_initialization_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_postgres_initialization_enabled_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_postgres_initialization_enabled_for_biorxiv,
            ProviderKind::Github => CONFIG.is_postgres_initialization_enabled_for_github,
            ProviderKind::Medrxiv => CONFIG.is_postgres_initialization_enabled_for_medrxiv,
            ProviderKind::Twitter => CONFIG.is_postgres_initialization_enabled_for_twitter,
            ProviderKind::Reddit => CONFIG.is_postgres_initialization_enabled_for_reddit,
            ProviderKind::Habr => CONFIG.is_postgres_initialization_enabled_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_write_error_logs_in_local_folder_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_write_error_logs_in_local_folder_enabled_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_write_error_logs_in_local_folder_enabled_for_biorxiv,
            ProviderKind::Github => CONFIG.is_write_error_logs_in_local_folder_enabled_for_github,
            ProviderKind::Medrxiv => CONFIG.is_write_error_logs_in_local_folder_enabled_for_medrxiv,
            ProviderKind::Twitter => CONFIG.is_write_error_logs_in_local_folder_enabled_for_twitter,
            ProviderKind::Reddit => CONFIG.is_write_error_logs_in_local_folder_enabled_for_reddit,
            ProviderKind::Habr => CONFIG.is_write_error_logs_in_local_folder_enabled_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_cleaning_warning_logs_directory_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_cleaning_warning_logs_directory_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_cleaning_warning_logs_directory_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_cleaning_warning_logs_directory_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_cleaning_warning_logs_directory_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_cleaning_warning_logs_directory_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_cleaning_warning_logs_directory_for_reddit,
            ProviderKind::Habr => CONFIG.enable_cleaning_warning_logs_directory_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_check_link(&self) -> &'static str {
        match self {
            ProviderKind::Arxiv => &CONFIG.arxiv_check_link,
            ProviderKind::Biorxiv => &CONFIG.biorxiv_check_link,
            ProviderKind::Github => &CONFIG.github_check_link,
            ProviderKind::Medrxiv => &CONFIG.medrxiv_check_link,
            ProviderKind::Twitter => &CONFIG.twitter_check_link,
            ProviderKind::Reddit => &CONFIG.reddit_check_link,
            ProviderKind::Habr => &CONFIG.habr_check_link,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_biorxiv,
            ProviderKind::Github => CONFIG.enable_github,
            ProviderKind::Medrxiv => CONFIG.enable_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_twitter,
            ProviderKind::Reddit => CONFIG.enable_reddit,
            ProviderKind::Habr => CONFIG.enable_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_prints_arxiv, //todo add for
            ProviderKind::Biorxiv => CONFIG.enable_prints_biorxiv, //todo add for
            ProviderKind::Github => CONFIG.enable_prints_github, //todo add for
            ProviderKind::Medrxiv => CONFIG.enable_prints_medrxiv, //todo add for
            ProviderKind::Twitter => CONFIG.enable_prints_twitter, //todo add for
            ProviderKind::Reddit => CONFIG.enable_prints_reddit, //todo add for
            ProviderKind::Habr => CONFIG.enable_prints_habr,   //todo add for
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_warning_high_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_warning_high_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_warning_high_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_warning_high_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_warning_high_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_warning_high_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_warning_high_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_warning_high_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_warning_low_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_warning_low_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_warning_low_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_warning_low_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_warning_low_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_warning_low_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_warning_low_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_warning_low_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_success_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_success_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_success_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_success_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_success_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_success_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_success_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_success_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_partial_success_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_partial_success_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_partial_success_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_partial_success_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_partial_success_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_partial_success_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_partial_success_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_partial_success_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_error_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_error_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_error_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_error_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_error_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_error_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_error_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_error_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_time_measurement_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_time_measurement_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_time_measurement_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_time_measurement_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_time_measurement_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_time_measurement_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_time_measurement_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_time_measurement_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_info_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_info_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_info_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_info_prints_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_info_prints_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_info_prints_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_info_prints_for_reddit,
            ProviderKind::Habr => CONFIG.enable_info_prints_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_links_limit_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_links_limit_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_links_limit_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_links_limit_for_github,
            ProviderKind::Medrxiv => CONFIG.enable_links_limit_for_medrxiv,
            ProviderKind::Twitter => CONFIG.enable_links_limit_for_twitter,
            ProviderKind::Reddit => CONFIG.enable_links_limit_for_reddit,
            ProviderKind::Habr => CONFIG.enable_links_limit_for_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_links_limit(&self) -> i64 {
        match self {
            ProviderKind::Arxiv => CONFIG.links_limit_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.links_limit_for_biorxiv,
            ProviderKind::Github => CONFIG.links_limit_for_github,
            ProviderKind::Habr => CONFIG.links_limit_for_habr,
            ProviderKind::Medrxiv => CONFIG.links_limit_for_medrxiv,
            ProviderKind::Reddit => CONFIG.links_limit_for_reddit,
            ProviderKind::Twitter => CONFIG.links_limit_for_twitter,
        }
    }
}
