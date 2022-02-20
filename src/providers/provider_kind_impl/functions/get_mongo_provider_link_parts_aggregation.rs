use mongodb::bson::{doc, Document};

use crate::{config_mods::lazy_static_config::CONFIG, providers::provider_kind_enum::ProviderKind};

use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn get_mongo_provider_link_parts_aggregation(&self) -> Option<Document> {
        if CONFIG.is_links_limit_enabled_providers
            && self.is_mongo_link_parts_randomize_order_enabled()
        {
            Some(doc! { "$sample" : {"size": CONFIG.links_limit_providers }});
        } else if CONFIG.is_links_limit_enabled_providers {
            Some(doc! { "$limit" :  CONFIG.links_limit_providers });
        } else if self.is_links_limit_enabled()
            && self.is_mongo_link_parts_randomize_order_enabled()
        {
            Some(doc! { "$sample" : {"size": self.links_limit() }});
        } else if self.is_links_limit_enabled() {
            Some(doc! { "$limit" : self.links_limit() });
        } else if self.is_mongo_link_parts_randomize_order_enabled() {
            println!("todo: mongo sample(randomized aggregation) only works if size is valid number. No aggregation applied");
            return None;
        }
        None
    }
}
