use mongodb::bson::{Document, doc};

use crate::{providers::provider_kind_enum::ProviderKind, config_mods::lazy_static_config::CONFIG};

use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn get_mongo_provider_link_parts_aggregation(&self) -> Option<Document> {
        //its common case - rename later
        if CONFIG.is_links_limit_providers_enabled && self.is_mongo_link_parts_randomize_order_enabled() {
            Some(doc! { "$sample" : {"size": CONFIG.links_limit_providers }});
        }
        else if CONFIG.is_links_limit_providers_enabled {
            Some(doc! { "$limit" :  CONFIG.links_limit_providers });
        }
        else if self.is_links_limit_enabled() && self.is_mongo_link_parts_randomize_order_enabled() {
            Some(doc! { "$sample" : {"size": self.links_limit() }});
        }
        else if self.is_links_limit_enabled() {
            Some(doc! { "$limit" : self.links_limit() });
        }
        else if self.is_mongo_link_parts_randomize_order_enabled() {
            println!("todo: mongo sample(randomized aggregation) only works if size is valid number. No aggregation applied");
            return None;
        }
        None
    }
}