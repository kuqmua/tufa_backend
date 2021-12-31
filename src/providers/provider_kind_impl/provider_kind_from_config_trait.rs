use crate::config_mods::lazy_static_config::CONFIG;

use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;

use crate::providers::provider_kind_enum::ProviderKind;

impl ProviderKindFromConfigTrait for ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_initialization_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_mongo_initialization_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_mongo_initialization_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_mongo_initialization_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_mongo_initialization_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_mongo_initialization_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_mongo_initialization_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_mongo_initialization_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_write_error_logs_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_mongo_write_error_logs_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_mongo_write_error_logs_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_mongo_write_error_logs_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_mongo_write_error_logs_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_mongo_write_error_logs_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_mongo_write_error_logs_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_mongo_write_error_logs_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_cleaning_warning_logs_db_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_mongo_cleaning_warning_logs_db_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_cleaning_warning_logs_db_collections_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_biorxiv
            }
            ProviderKind::Github => {
                CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_github
            }
            ProviderKind::Medrxiv => {
                CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_medrxiv
            }
            ProviderKind::Twitter => {
                CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_twitter
            }
            ProviderKind::Reddit => {
                CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_reddit
            }
            ProviderKind::Habr => CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_link_parts_randomize_order_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_mongo_link_parts_randomize_order_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_mongo_link_parts_randomize_order_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_mongo_link_parts_randomize_order_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_mongo_link_parts_randomize_order_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_mongo_link_parts_randomize_order_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_mongo_link_parts_randomize_order_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_mongo_link_parts_randomize_order_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_postgres_initialization_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_postgres_initialization_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_postgres_initialization_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_postgres_initialization_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_postgres_initialization_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_postgres_initialization_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_postgres_initialization_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_postgres_initialization_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_write_error_logs_in_local_folder_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_write_error_logs_in_local_folder_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_write_error_logs_in_local_folder_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_write_error_logs_in_local_folder_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_write_error_logs_in_local_folder_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_write_error_logs_in_local_folder_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_write_error_logs_in_local_folder_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_write_error_logs_in_local_folder_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_cleaning_warning_logs_directory_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_cleaning_warning_logs_directory_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_cleaning_warning_logs_directory_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_cleaning_warning_logs_directory_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_cleaning_warning_logs_directory_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_cleaning_warning_logs_directory_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_cleaning_warning_logs_directory_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_cleaning_warning_logs_directory_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn check_link(&self) -> &'static str {
        match self {
            ProviderKind::Arxiv => &CONFIG.check_link_arxiv,
            ProviderKind::Biorxiv => &CONFIG.check_link_biorxiv,
            ProviderKind::Github => &CONFIG.check_link_github,
            ProviderKind::Medrxiv => &CONFIG.check_link_medrxiv,
            ProviderKind::Twitter => &CONFIG.check_link_twitter,
            ProviderKind::Reddit => &CONFIG.check_link_reddit,
            ProviderKind::Habr => &CONFIG.check_link_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_prints_enabled_arxiv, //todo add for
            ProviderKind::Biorxiv => CONFIG.is_prints_enabled_biorxiv, //todo add for
            ProviderKind::Github => CONFIG.is_prints_enabled_github, //todo add for
            ProviderKind::Medrxiv => CONFIG.is_prints_enabled_medrxiv, //todo add for
            ProviderKind::Twitter => CONFIG.is_prints_enabled_twitter, //todo add for
            ProviderKind::Reddit => CONFIG.is_prints_enabled_reddit, //todo add for
            ProviderKind::Habr => CONFIG.is_prints_enabled_habr,   //todo add for
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_warning_high_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_warning_high_prints_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_warning_high_prints_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_warning_high_prints_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_warning_high_prints_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_warning_high_prints_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_warning_high_prints_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_warning_high_prints_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_warning_low_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_warning_low_prints_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_warning_low_prints_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_warning_low_prints_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_warning_low_prints_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_warning_low_prints_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_warning_low_prints_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_warning_low_prints_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_success_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_success_prints_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_success_prints_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_success_prints_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_success_prints_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_success_prints_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_success_prints_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_success_prints_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_partial_success_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_partial_success_prints_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_partial_success_prints_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_partial_success_prints_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_partial_success_prints_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_partial_success_prints_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_partial_success_prints_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_partial_success_prints_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_error_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_error_prints_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_error_prints_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_error_prints_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_error_prints_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_error_prints_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_error_prints_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_error_prints_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_time_measurement_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_time_measurement_prints_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_time_measurement_prints_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_time_measurement_prints_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_time_measurement_prints_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_time_measurement_prints_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_time_measurement_prints_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_time_measurement_prints_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_info_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_info_prints_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_info_prints_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_info_prints_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_info_prints_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_info_prints_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_info_prints_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_info_prints_enabled_habr,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_links_limit_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.is_links_limit_enabled_arxiv,
            ProviderKind::Biorxiv => CONFIG.is_links_limit_enabled_biorxiv,
            ProviderKind::Github => CONFIG.is_links_limit_enabled_github,
            ProviderKind::Medrxiv => CONFIG.is_links_limit_enabled_medrxiv,
            ProviderKind::Twitter => CONFIG.is_links_limit_enabled_twitter,
            ProviderKind::Reddit => CONFIG.is_links_limit_enabled_reddit,
            ProviderKind::Habr => CONFIG.is_links_limit_enabled_habr,
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
