use crate::providers::provider_kind_enum::ProviderKind;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::traits::env_var_bool_trait::EnvVarBoolTrait;

impl EnvVarBoolTrait for ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_initialization_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.mongo_enable_initialization_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.mongo_enable_initialization_for_biorxiv,
            ProviderKind::Github => CONFIG.mongo_enable_initialization_for_github,
            ProviderKind::Habr => CONFIG.mongo_enable_initialization_for_habr,
            ProviderKind::Medrxiv => CONFIG.mongo_enable_initialization_for_medrxiv,
            ProviderKind::Reddit => CONFIG.mongo_enable_initialization_for_reddit,
            ProviderKind::Twitter => CONFIG.mongo_enable_initialization_for_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_biorxiv,
            ProviderKind::Github => CONFIG.enable_github,
            ProviderKind::Habr => CONFIG.enable_habr,
            ProviderKind::Medrxiv => CONFIG.enable_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_reddit,
            ProviderKind::Twitter => CONFIG.enable_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_prints_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_prints_biorxiv,
            ProviderKind::Github => CONFIG.enable_prints_github,
            ProviderKind::Habr => CONFIG.enable_prints_habr,
            ProviderKind::Medrxiv => CONFIG.enable_prints_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_prints_reddit,
            ProviderKind::Twitter => CONFIG.enable_prints_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_warning_high_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_warning_high_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_warning_high_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_warning_high_prints_for_github,
            ProviderKind::Habr => CONFIG.enable_warning_high_prints_for_habr,
            ProviderKind::Medrxiv => CONFIG.enable_warning_high_prints_for_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_warning_high_prints_for_reddit,
            ProviderKind::Twitter => CONFIG.enable_warning_high_prints_for_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_warning_low_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_warning_low_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_warning_low_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_warning_low_prints_for_github,
            ProviderKind::Habr => CONFIG.enable_warning_low_prints_for_habr,
            ProviderKind::Medrxiv => CONFIG.enable_warning_low_prints_for_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_warning_low_prints_for_reddit,
            ProviderKind::Twitter => CONFIG.enable_warning_low_prints_for_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_error_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_error_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_error_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_error_prints_for_github,
            ProviderKind::Habr => CONFIG.enable_error_prints_for_habr,
            ProviderKind::Medrxiv => CONFIG.enable_error_prints_for_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_error_prints_for_reddit,
            ProviderKind::Twitter => CONFIG.enable_error_prints_for_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_success_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_success_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_success_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_success_prints_for_github,
            ProviderKind::Habr => CONFIG.enable_success_prints_for_habr,
            ProviderKind::Medrxiv => CONFIG.enable_success_prints_for_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_success_prints_for_reddit,
            ProviderKind::Twitter => CONFIG.enable_success_prints_for_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_partial_success_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_partial_success_prints_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_partial_success_prints_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_partial_success_prints_for_github,
            ProviderKind::Habr => CONFIG.enable_partial_success_prints_for_habr,
            ProviderKind::Medrxiv => CONFIG.enable_partial_success_prints_for_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_partial_success_prints_for_reddit,
            ProviderKind::Twitter => CONFIG.enable_partial_success_prints_for_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_cleaning_warning_logs_directory_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_cleaning_warning_logs_directory_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_cleaning_warning_logs_directory_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_cleaning_warning_logs_directory_for_github,
            ProviderKind::Habr => CONFIG.enable_cleaning_warning_logs_directory_for_habr,
            ProviderKind::Medrxiv => CONFIG.enable_cleaning_warning_logs_directory_for_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_cleaning_warning_logs_directory_for_reddit,
            ProviderKind::Twitter => CONFIG.enable_cleaning_warning_logs_directory_for_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_cleaning_warning_logs_db_in_mongo_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.mongo_enable_cleaning_warning_logs_db_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.mongo_enable_cleaning_warning_logs_db_for_biorxiv,
            ProviderKind::Github => CONFIG.mongo_enable_cleaning_warning_logs_db_for_github,
            ProviderKind::Habr => CONFIG.mongo_enable_cleaning_warning_logs_db_for_habr,
            ProviderKind::Medrxiv => CONFIG.mongo_enable_cleaning_warning_logs_db_for_medrxiv,
            ProviderKind::Reddit => CONFIG.mongo_enable_cleaning_warning_logs_db_for_reddit,
            ProviderKind::Twitter => CONFIG.mongo_enable_cleaning_warning_logs_db_for_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_cleaning_warning_logs_db_collections_in_mongo_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_github
            }
            ProviderKind::Habr => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG.mongo_enable_cleaning_warning_logs_db_collections_for_twitter
            }
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_time_measurement_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_time_measurement_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_time_measurement_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_time_measurement_for_github,
            ProviderKind::Habr => CONFIG.enable_time_measurement_for_habr,
            ProviderKind::Medrxiv => CONFIG.enable_time_measurement_for_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_time_measurement_for_reddit,
            ProviderKind::Twitter => CONFIG.enable_time_measurement_for_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_info_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_info_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_info_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_info_for_github,
            ProviderKind::Habr => CONFIG.enable_info_for_habr,
            ProviderKind::Medrxiv => CONFIG.enable_info_for_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_info_for_reddit,
            ProviderKind::Twitter => CONFIG.enable_info_for_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_link_limits_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_links_limit_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_links_limit_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_links_limit_for_github,
            ProviderKind::Habr => CONFIG.enable_links_limit_for_habr,
            ProviderKind::Medrxiv => CONFIG.enable_links_limit_for_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_links_limit_for_reddit,
            ProviderKind::Twitter => CONFIG.enable_links_limit_for_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_randomize_order_mongo_link_parts_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.mongo_enable_link_parts_randomize_order_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.mongo_enable_link_parts_randomize_order_for_biorxiv,
            ProviderKind::Github => CONFIG.mongo_enable_link_parts_randomize_order_for_github,
            ProviderKind::Habr => CONFIG.mongo_enable_link_parts_randomize_order_for_habr,
            ProviderKind::Medrxiv => CONFIG.mongo_enable_link_parts_randomize_order_for_medrxiv,
            ProviderKind::Reddit => CONFIG.mongo_enable_link_parts_randomize_order_for_reddit,
            ProviderKind::Twitter => CONFIG.mongo_enable_link_parts_randomize_order_for_twitter,
        }
    }

    //todo add errors warning low warning high info and others
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_cleaning_warning_logs_directory_enable(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_cleaning_warning_logs_directory_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_cleaning_warning_logs_directory_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_cleaning_warning_logs_directory_for_habr,
            ProviderKind::Habr => CONFIG.enable_cleaning_warning_logs_directory_for_habr,
            ProviderKind::Medrxiv => CONFIG.enable_cleaning_warning_logs_directory_for_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_cleaning_warning_logs_directory_for_reddit,
            ProviderKind::Twitter => CONFIG.enable_cleaning_warning_logs_directory_for_twitter,
        }
    }
}
