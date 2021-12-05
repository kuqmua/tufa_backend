use crate::providers::provider_kind_enum::ProviderKind;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::traits::env_var_bool_trait::EnvVarBoolTrait;

impl EnvVarBoolTrait for ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_mongo_initialization_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_arxiv_link_parts
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_biorxiv_link_parts
            }
            ProviderKind::Github => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_github_link_parts
            }
            ProviderKind::Habr => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_habr_link_parts
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_medrxiv_link_parts
            }
            ProviderKind::Reddit => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_reddit_link_parts
            }
            ProviderKind::Twitter => {
                CONFIG
                    .mongo_params
                    .enable_initialize_mongo_with_providers_link_parts
                    .enable_initialize_mongo_with_twitter_link_parts
            }
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_providers.enable_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_providers.enable_biorxiv,
            ProviderKind::Github => CONFIG.enable_providers.enable_github,
            ProviderKind::Habr => CONFIG.enable_providers.enable_habr,
            ProviderKind::Medrxiv => CONFIG.enable_providers.enable_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_providers.enable_reddit,
            ProviderKind::Twitter => CONFIG.enable_providers.enable_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_providers_prints.enable_prints_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_providers_prints.enable_prints_biorxiv,
            ProviderKind::Github => CONFIG.enable_providers_prints.enable_prints_github,
            ProviderKind::Habr => CONFIG.enable_providers_prints.enable_prints_habr,
            ProviderKind::Medrxiv => CONFIG.enable_providers_prints.enable_prints_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_providers_prints.enable_prints_reddit,
            ProviderKind::Twitter => CONFIG.enable_providers_prints.enable_prints_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_warning_high_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_warning_high_providers_prints
                    .enable_warning_high_prints_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_warning_high_providers_prints
                    .enable_warning_high_prints_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_warning_high_providers_prints
                    .enable_warning_high_prints_for_github
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_warning_high_providers_prints
                    .enable_warning_high_prints_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_warning_high_providers_prints
                    .enable_warning_high_prints_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_warning_high_providers_prints
                    .enable_warning_high_prints_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_warning_high_providers_prints
                    .enable_warning_high_prints_for_twitter
            }
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_warning_low_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_warning_low_providers_prints
                    .enable_warning_low_prints_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_warning_low_providers_prints
                    .enable_warning_low_prints_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_warning_low_providers_prints
                    .enable_warning_low_prints_for_github
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_warning_low_providers_prints
                    .enable_warning_low_prints_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_warning_low_providers_prints
                    .enable_warning_low_prints_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_warning_low_providers_prints
                    .enable_warning_low_prints_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_warning_low_providers_prints
                    .enable_warning_low_prints_for_twitter
            }
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_error_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_github
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_error_providers_prints
                    .enable_error_prints_for_twitter
            }
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_success_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_success_providers_prints
                    .enable_success_prints_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_success_providers_prints
                    .enable_success_prints_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_success_providers_prints
                    .enable_success_prints_for_github
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_success_providers_prints
                    .enable_success_prints_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_success_providers_prints
                    .enable_success_prints_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_success_providers_prints
                    .enable_success_prints_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_success_providers_prints
                    .enable_success_prints_for_twitter
            }
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_partial_success_prints_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_partial_success_providers_prints
                    .enable_partial_success_prints_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_partial_success_providers_prints
                    .enable_partial_success_prints_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_partial_success_providers_prints
                    .enable_partial_success_prints_for_github
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_partial_success_providers_prints
                    .enable_partial_success_prints_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_partial_success_providers_prints
                    .enable_partial_success_prints_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_partial_success_providers_prints
                    .enable_partial_success_prints_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_partial_success_providers_prints
                    .enable_partial_success_prints_for_twitter
            }
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_cleaning_warning_logs_directory_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_github
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_twitter
            }
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_cleaning_warning_logs_db_in_mongo_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => CONFIG.enable_cleaning_warning_logs_db_in_mongo_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.enable_cleaning_warning_logs_db_in_mongo_for_biorxiv,
            ProviderKind::Github => CONFIG.enable_cleaning_warning_logs_db_in_mongo_for_github,
            ProviderKind::Habr => CONFIG.enable_cleaning_warning_logs_db_in_mongo_for_habr,
            ProviderKind::Medrxiv => CONFIG.enable_cleaning_warning_logs_db_in_mongo_for_medrxiv,
            ProviderKind::Reddit => CONFIG.enable_cleaning_warning_logs_db_in_mongo_for_reddit,
            ProviderKind::Twitter => CONFIG.enable_cleaning_warning_logs_db_in_mongo_for_twitter,
        }
    }

    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_cleaning_warning_logs_db_collections_in_mongo_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG.enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG.enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG.enable_cleaning_warning_logs_db_collections_in_mongo_for_github
            }
            ProviderKind::Habr => {
                CONFIG.enable_cleaning_warning_logs_db_collections_in_mongo_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG.enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG.enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG.enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter
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
            ProviderKind::Arxiv => CONFIG.enable_randomize_order_for_arxiv_link_parts_for_mongo,
            ProviderKind::Biorxiv => CONFIG.enable_randomize_order_for_biorxiv_link_parts_for_mongo,
            ProviderKind::Github => CONFIG.enable_randomize_order_for_github_link_parts_for_mongo,
            ProviderKind::Habr => CONFIG.enable_randomize_order_for_habr_link_parts_for_mongo,
            ProviderKind::Medrxiv => CONFIG.enable_randomize_order_for_medrxiv_link_parts_for_mongo,
            ProviderKind::Reddit => CONFIG.enable_randomize_order_for_reddit_link_parts_for_mongo,
            ProviderKind::Twitter => CONFIG.enable_randomize_order_for_twitter_link_parts_for_mongo,
        }
    }

    //todo add errors warning low warning high info and others
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_cleaning_warning_logs_directory_enable(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_habr
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_providers_cleaning_warning_logs_directory
                    .enable_cleaning_warning_logs_directory_for_twitter
            }
        }
    }
}
