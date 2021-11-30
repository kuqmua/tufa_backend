use mongodb::bson::{Document, doc};

use crate::providers::provider_kind_enum::ProviderKind;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::traits::provider_kind_trait::ProviderKindTrait;

impl ProviderKindTrait for ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_link_limits_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_arxiv
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_biorxiv
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_github
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_habr
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_medrxiv
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_reddit
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_providers_links_limits
                    .enable_links_limit_for_twitter
            }
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn is_randomize_order_mongo_link_parts_enabled(&self) -> bool {
        match self {
            ProviderKind::Arxiv => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_arxiv_link_parts_for_mongo
            }
            ProviderKind::Biorxiv => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_biorxiv_link_parts_for_mongo
            }
            ProviderKind::Github => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_github_link_parts_for_mongo
            }
            ProviderKind::Habr => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_habr_link_parts_for_mongo
            }
            ProviderKind::Medrxiv => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_medrxiv_link_parts_for_mongo
            }
            ProviderKind::Reddit => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_reddit_link_parts_for_mongo
            }
            ProviderKind::Twitter => {
                CONFIG
                    .enable_randomize_order_for_providers_link_parts_for_mongo
                    .enable_randomize_order_for_twitter_link_parts_for_mongo
            }
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_check_link(&self) -> &'static str {
        match self {
            ProviderKind::Arxiv => &CONFIG.providers_check_links.arxiv_link,
            ProviderKind::Biorxiv => &CONFIG.providers_check_links.biorxiv_link,
            ProviderKind::Github => &CONFIG.providers_check_links.github_link,
            ProviderKind::Medrxiv => &CONFIG.providers_check_links.medrxiv_link,
            ProviderKind::Twitter => &CONFIG.providers_check_links.twitter_link,
            ProviderKind::Reddit => &CONFIG.providers_check_links.reddit_link,
            ProviderKind::Habr => &CONFIG.providers_check_links.habr_link,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_init_local_data_file_path(&self) -> String {
        format!(
            "{}{}{}{}",
            CONFIG.mongo_params.path_to_provider_link_parts_folder,
            self,
            CONFIG
                .mongo_params
                .providers_db_collection_handle_second_part,
            CONFIG.mongo_params.log_file_extension
        )
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_item_handle(&self) -> Option<&'static str> {
        match self {
            ProviderKind::Arxiv => Some("</item>"),
            ProviderKind::Biorxiv => Some("</item>"),
            ProviderKind::Github => Some("</entry>"),
            ProviderKind::Habr => Some("</item>"),
            ProviderKind::Medrxiv => Some("</item>"),
            ProviderKind::Reddit => None,
            ProviderKind::Twitter => Some("</item>"),
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_links_limit_for_provider(&self) -> i64 {
        match self {
            ProviderKind::Arxiv => CONFIG.providers_links_limits.links_limit_for_arxiv,
            ProviderKind::Biorxiv => CONFIG.providers_links_limits.links_limit_for_biorxiv,
            ProviderKind::Github => CONFIG.providers_links_limits.links_limit_for_github,
            ProviderKind::Habr => CONFIG.providers_links_limits.links_limit_for_habr,
            ProviderKind::Medrxiv => CONFIG.providers_links_limits.links_limit_for_medrxiv,
            ProviderKind::Reddit => CONFIG.providers_links_limits.links_limit_for_reddit,
            ProviderKind::Twitter => CONFIG.providers_links_limits.links_limit_for_twitter,
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_mongo_doc_randomization_aggregation(&self) -> Option<Document> {
        if self.is_link_limits_enabled() {
            if self.is_randomize_order_mongo_link_parts_enabled() {
                Some(
                    doc! { "$sample" : {"size": self.get_links_limit_for_provider() }},
                )
            } else {
                Some(doc! { "$limit" : self.get_links_limit_for_provider() })
            }
        } else {
            None
        }
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_mongo_log_collection_name(&self) -> String {
        format!(
            "{}{}",
            self,
            CONFIG
                .mongo_params
                .providers_db_collection_handle_second_part //todo rename it into db log collection
        )
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_path_to_logs_directory(&self) -> String {
        format!(
            "logs/{}/{:?}",
            &CONFIG.params.warning_logs_directory_name, self
        )
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_path_to_provider_log_file(&self) -> String {
        format!(
            "logs/{}/{:?}/{}",
            &CONFIG.params.warning_logs_directory_name,
            self,
            &CONFIG
                .params
                .unhandled_success_handled_success_are_there_items_initialized_posts_dir
        )
    }
}
