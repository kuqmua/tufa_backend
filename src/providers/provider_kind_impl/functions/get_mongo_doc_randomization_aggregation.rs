use crate::providers::provider_kind_enum::ProviderKind;

use mongodb::bson::{doc, Document};

impl ProviderKind {
    pub fn get_mongo_doc_randomization_aggregation(
        provider_kind: ProviderKind,
    ) -> Option<Document> {
        if ProviderKind::is_link_limits_enabled(provider_kind) {
            if ProviderKind::enable_randomize_order_mongo_link_parts_for(provider_kind) {
                Some(
                    doc! { "$sample" : {"size": ProviderKind::get_links_limit_for_provider(provider_kind) }},
                )
            } else {
                Some(doc! { "$limit" : ProviderKind::get_links_limit_for_provider(provider_kind) })
            }
        } else {
            None
        }
    }
}
